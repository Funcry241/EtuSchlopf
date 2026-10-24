#[cfg(test)]
use super::authority_primitives::yes_no;
use super::locator_resolution::{direct_child_i64, direct_child_text};
use super::detector::{classify_tail_beam_state, detect_repair_intent};
use super::{
    LocatedStructuralRepairIntent, LocatedStructuralView, StructuralBeamState,
    StructuralCarrierRole, StructuralEvent, StructuralLocator, StructuralSourceEventLocator,
    StructuralStaffRelation, StructuralView, StructuralVoiceRelation,
};
use roxmltree::{Document, Node};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
struct RawStructuralEvent {
    onset: i64,
    duration: i64,
    voice: String,
    staff: String,
    primary_beam: StructuralBeamState,
    secondary_beam: StructuralBeamState,
    visible: bool,
    rest_like: bool,
    chord_member: bool,
    ordinal: usize,
    original_cursor_before: i64,
    original_cursor_after: i64,
}

#[derive(Debug, Clone, Default)]
pub struct StructuralSnapshotReadOnlyAdapterAudit {
    #[cfg(test)]
    source_stage: String,
    parse_ok: bool,
    part_count: usize,
    measure_count: usize,
    voice_run_count: usize,
    two_beat_view_count: usize,
    structural_event_count: usize,
    hidden_carrier_count: usize,
    visible_rest_count: usize,
    cross_staff_view_count: usize,
    identity_free_structure_count: usize,
    identity_free_candidate_count: usize,
    #[cfg(test)]
    locator_identity_exposed_to_detector: bool,
    #[cfg(test)]
    writer_token_used: bool,
    #[cfg(test)]
    verdict: String,
}

impl StructuralSnapshotReadOnlyAdapterAudit {
    pub fn parse_ok(&self) -> bool {
        self.parse_ok
    }

    #[cfg(test)]
    pub fn pipeline_line(&self, project_label: &str) -> String {
        format!(
            "StructuralSnapshotReadOnlyAdapter v{}: Project {}; SourceStage {}; ParseOk {}; Parts {}; Measures {}; VoiceRuns {}; TwoBeatViews {}; StructuralEvents {}; HiddenCarriers {}; VisibleRests {}; CrossStaffViews {}; IdentityFreeStructures {}; IdentityFreeCandidates {}; DetectorIdentityFieldsExposed {}; WriterTokenUsed {}; RenderXmlWrite nein; MusicXmlWrite nein; PdfWrite nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            project_label,
            self.source_stage,
            yes_no(self.parse_ok),
            self.part_count,
            self.measure_count,
            self.voice_run_count,
            self.two_beat_view_count,
            self.structural_event_count,
            self.hidden_carrier_count,
            self.visible_rest_count,
            self.cross_staff_view_count,
            self.identity_free_structure_count,
            self.identity_free_candidate_count,
            yes_no(self.locator_identity_exposed_to_detector),
            yes_no(self.writer_token_used),
            self.verdict,
        )
    }
}

pub fn adapt_structural_snapshot_read_only(
    xml: &str,
    fallback_divisions: i64,
    source_stage: &str,
) -> (
    Vec<LocatedStructuralView>,
    StructuralSnapshotReadOnlyAdapterAudit,
) {
    let mut audit = StructuralSnapshotReadOnlyAdapterAudit {
        #[cfg(test)]
        source_stage: source_stage.to_string(),
        #[cfg(test)]
        locator_identity_exposed_to_detector: false,
        #[cfg(test)]
        writer_token_used: false,
        #[cfg(test)]
        verdict: "read_only_structural_snapshot_adapter_pending".to_string(),
        ..StructuralSnapshotReadOnlyAdapterAudit::default()
    };
    let document = match Document::parse(xml) {
        Ok(document) => document,
        Err(_) => {
            #[cfg(test)]
            {
                audit.verdict = "read_only_structural_snapshot_adapter_parse_failed".to_string();
            }
            return (Vec::new(), audit);
        }
    };
    audit.parse_ok = true;

    let source_nonce = stable_nonce(source_stage);
    let source_fingerprint = stable_nonce(xml);
    let root = document.root_element();
    let mut located_views = Vec::new();
    let mut next_event_ordinal = 0_usize;

    if root.has_tag_name("measure") {
        audit.part_count = 1;
        let divisions = measure_divisions(root).unwrap_or(fallback_divisions).max(1);
        append_measure_views(
            root,
            StructuralSnapshotMeasureContext {
                part_ordinal: 0,
                measure_ordinal: 0,
                divisions,
                source_nonce,
                source_fingerprint,
            },
            &mut next_event_ordinal,
            &mut located_views,
            &mut audit,
        );
    } else {
        let parts = root
            .descendants()
            .filter(|node| node.is_element() && node.has_tag_name("part"))
            .collect::<Vec<_>>();
        if parts.is_empty() {
            audit.part_count = 1;
            let mut divisions = fallback_divisions.max(1);
            for (measure_ordinal, measure) in root
                .descendants()
                .filter(|node| node.is_element() && node.has_tag_name("measure"))
                .enumerate()
            {
                divisions = measure_divisions(measure).unwrap_or(divisions).max(1);
                append_measure_views(
                    measure,
                    StructuralSnapshotMeasureContext {
                        part_ordinal: 0,
                        measure_ordinal,
                        divisions,
                        source_nonce,
                        source_fingerprint,
                    },
                    &mut next_event_ordinal,
                    &mut located_views,
                    &mut audit,
                );
            }
        } else {
            audit.part_count = parts.len();
            for (part_ordinal, part) in parts.into_iter().enumerate() {
                let mut divisions = fallback_divisions.max(1);
                for (measure_ordinal, measure) in part
                    .children()
                    .filter(|node| node.is_element() && node.has_tag_name("measure"))
                    .enumerate()
                {
                    divisions = measure_divisions(measure).unwrap_or(divisions).max(1);
                    append_measure_views(
                        measure,
                        StructuralSnapshotMeasureContext {
                            part_ordinal,
                            measure_ordinal,
                            divisions,
                            source_nonce,
                            source_fingerprint,
                        },
                        &mut next_event_ordinal,
                        &mut located_views,
                        &mut audit,
                    );
                }
            }
        }
    }

    for located in &located_views {
        if classify_tail_beam_state(located.detector_view()).is_some() {
            audit.identity_free_structure_count += 1;
            if detect_repair_intent(located.detector_view()).is_some() {
                audit.identity_free_candidate_count += 1;
            }
        }
    }
    #[cfg(test)]
    {
        audit.verdict = "read_only_structural_snapshot_adapter_green".to_string();
    }

    (located_views, audit)
}

#[derive(Debug, Clone, Copy)]
struct StructuralSnapshotMeasureContext {
    part_ordinal: usize,
    measure_ordinal: usize,
    divisions: i64,
    source_nonce: u64,
    source_fingerprint: u64,
}

fn append_measure_views(
    measure: Node<'_, '_>,
    context: StructuralSnapshotMeasureContext,
    next_event_ordinal: &mut usize,
    located_views: &mut Vec<LocatedStructuralView>,
    audit: &mut StructuralSnapshotReadOnlyAdapterAudit,
) {
    let StructuralSnapshotMeasureContext {
        part_ordinal,
        measure_ordinal,
        divisions,
        source_nonce,
        source_fingerprint,
    } = context;
    audit.measure_count += 1;
    let events = parse_measure_events(measure, next_event_ordinal);
    let measure_extent = events
        .iter()
        .map(|event| event.onset + event.duration)
        .max()
        .unwrap_or(0);
    if measure_extent < divisions * 2 {
        return;
    }

    let mut by_voice: BTreeMap<String, Vec<RawStructuralEvent>> = BTreeMap::new();
    for event in events {
        by_voice.entry(event.voice.clone()).or_default().push(event);
    }
    audit.voice_run_count += by_voice.len();

    for (voice, mut voice_events) in by_voice {
        voice_events.sort_by_key(|event| (event.onset, event.ordinal));
        let mut window_start = 0_i64;
        while window_start + divisions * 2 <= measure_extent {
            let mut selected = voice_events
                .iter()
                .filter(|event| {
                    event.onset >= window_start && event.onset < window_start + divisions * 2
                })
                .cloned()
                .collect::<Vec<_>>();
            selected.sort_by_key(|event| (event.onset, event.ordinal));
            if selected.len() >= 2
                && let Some(anchor_staff) = selected
                    .iter()
                    .find(|event| event.visible && !event.rest_like && !event.chord_member)
                    .map(|event| event.staff.clone())
                {
                    let structural_events = selected
                        .iter()
                        .map(|event| StructuralEvent {
                            onset_in_window: i32::try_from(event.onset - window_start)
                                .unwrap_or(i32::MAX),
                            duration: i32::try_from(event.duration).unwrap_or(i32::MAX),
                            primary_beam: event.primary_beam,
                            secondary_beam: event.secondary_beam,
                            staff_relation: if event.staff == anchor_staff {
                                StructuralStaffRelation::SameAsAnchor
                            } else {
                                StructuralStaffRelation::DifferentFromAnchor
                            },
                            voice_relation: StructuralVoiceRelation::SameRun,
                            carrier_role: if !event.visible {
                                StructuralCarrierRole::HiddenCarrier
                            } else if event.rest_like {
                                StructuralCarrierRole::Rest
                            } else {
                                StructuralCarrierRole::Sounding
                            },
                            visible: event.visible,
                            rest_like: event.rest_like,
                            chord_member: event.chord_member,
                        })
                        .collect::<Vec<_>>();
                    let source_events = selected
                        .iter()
                        .map(|event| StructuralSourceEventLocator {
                            event_ordinal: event.ordinal,
                            original_cursor_before: event.original_cursor_before,
                            original_cursor_after: event.original_cursor_after,
                        })
                        .collect();
                    let source_identity = format!(
                        "part={part_ordinal};measure={};voice={voice}",
                        measure.attribute("number").unwrap_or("")
                    );
                    let locator = StructuralLocator::from_snapshot(
                        source_nonce,
                        source_fingerprint,
                        part_ordinal,
                        measure_ordinal,
                        source_identity,
                        window_start,
                        source_events,
                    );
                    let view = StructuralView::new(
                        i32::try_from(divisions).unwrap_or(i32::MAX),
                        structural_events,
                    );
                    audit.two_beat_view_count += 1;
                    audit.structural_event_count += view.events().len();
                    audit.hidden_carrier_count += view
                        .events()
                        .iter()
                        .filter(|event| event.carrier_role == StructuralCarrierRole::HiddenCarrier)
                        .count();
                    audit.visible_rest_count += view
                        .events()
                        .iter()
                        .filter(|event| event.visible && event.rest_like)
                        .count();
                    if view.events().iter().any(|event| {
                        event.staff_relation == StructuralStaffRelation::DifferentFromAnchor
                    }) {
                        audit.cross_staff_view_count += 1;
                    }
                    located_views.push(LocatedStructuralView::new(view, locator));
                }
            window_start += divisions;
        }
    }
}

fn parse_measure_events(
    measure: Node<'_, '_>,
    next_event_ordinal: &mut usize,
) -> Vec<RawStructuralEvent> {
    let mut events = Vec::new();
    let mut cursor = 0_i64;
    let mut last_onset_by_voice: HashMap<String, i64> = HashMap::new();

    for child in measure.children().filter(Node::is_element) {
        if child.has_tag_name("backup") {
            cursor -= direct_child_i64(child, "duration").unwrap_or(0);
            continue;
        }
        if child.has_tag_name("forward") {
            cursor += direct_child_i64(child, "duration").unwrap_or(0);
            continue;
        }
        if !child.has_tag_name("note") {
            continue;
        }

        let original_cursor_before = cursor;
        let duration = direct_child_i64(child, "duration").unwrap_or(0).max(0);
        let voice = direct_child_text(child, "voice").unwrap_or_else(|| "1".to_string());
        let staff = direct_child_text(child, "staff").unwrap_or_else(|| "1".to_string());
        let chord_member = child
            .children()
            .any(|node| node.is_element() && node.has_tag_name("chord"));
        let onset = if chord_member {
            last_onset_by_voice.get(&voice).copied().unwrap_or(cursor)
        } else {
            let onset = cursor;
            last_onset_by_voice.insert(voice.clone(), onset);
            cursor += duration;
            onset
        };
        let original_cursor_after = cursor;
        let visible = child.attribute("print-object") != Some("no")
            && direct_child_text(child, "notehead").as_deref() != Some("none");
        let rest_like = child
            .children()
            .any(|node| node.is_element() && node.has_tag_name("rest"));
        let primary_beam = direct_beam_state(child, "1");
        let secondary_beam = direct_beam_state(child, "2");
        let ordinal = *next_event_ordinal;
        *next_event_ordinal += 1;
        events.push(RawStructuralEvent {
            onset,
            duration,
            voice,
            staff,
            primary_beam,
            secondary_beam,
            visible,
            rest_like,
            chord_member,
            ordinal,
            original_cursor_before,
            original_cursor_after,
        });
    }

    events
}

pub fn detect_structural_repair_intents_read_only(
    located_views: &[LocatedStructuralView],
) -> Vec<LocatedStructuralRepairIntent> {
    located_views
        .iter()
        .filter_map(|located| {
            detect_repair_intent(located.detector_view()).map(|intent| {
                LocatedStructuralRepairIntent::new(intent, located.clone())
            })
        })
        .collect()
}

fn direct_beam_state(note: Node<'_, '_>, number: &str) -> StructuralBeamState {
    let value = note
        .children()
        .find(|child| {
            child.is_element()
                && child.has_tag_name("beam")
                && child.attribute("number").unwrap_or("1") == number
        })
        .and_then(|beam| beam.text())
        .map(str::trim);
    match value {
        Some("begin") => StructuralBeamState::Begin,
        Some("continue") => StructuralBeamState::Continue,
        Some("end") => StructuralBeamState::End,
        _ => StructuralBeamState::Absent,
    }
}

fn measure_divisions(measure: Node<'_, '_>) -> Option<i64> {
    measure
        .descendants()
        .find(|node| node.is_element() && node.has_tag_name("divisions"))
        .and_then(|node| node.text())
        .map(str::trim)
        .and_then(|text| text.parse().ok())
}

fn stable_nonce(text: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in text.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::structural_write_authority::detector::IdentityFreeTailBeamState;

    const SYNTHETIC_SNAPSHOT: &str = include_str!(
        "../../tests/fixtures/synthetic/cross_staff_tail_structural_invariant.xml"
    );

    fn matching_fingerprints(xml: &str) -> Vec<(i32, Vec<StructuralEvent>)> {
        let (views, audit) = adapt_structural_snapshot_read_only(xml, 96, "test-snapshot");
        assert!(audit.parse_ok);
        views
            .iter()
            .filter(|located| classify_tail_beam_state(located.detector_view()).is_some())
            .map(|located| located.detector_view().fingerprint())
            .collect()
    }

    #[test]
    fn synthetic_cross_staff_tail_projects_one_identity_free_candidate() {
        let (views, audit) =
            adapt_structural_snapshot_read_only(SYNTHETIC_SNAPSHOT, 96, "synthetic");
        assert!(audit.parse_ok);
        assert_eq!(audit.measure_count, 1);
        assert_eq!(audit.identity_free_structure_count, 1);
        assert_eq!(audit.identity_free_candidate_count, 1);
        let matches = views
            .iter()
            .filter_map(|located| classify_tail_beam_state(located.detector_view()))
            .collect::<Vec<_>>();
        assert_eq!(matches, vec![IdentityFreeTailBeamState::Beat3CorrectBeat4Open]);
    }

    #[test]
    fn projection_fingerprint_ignores_measure_pitch_voice_and_staff_identity() {
        let transformed = SYNTHETIC_SNAPSHOT
            .replace("number=\"synthetic\"", "number=\"relocated\"")
            .replace("<voice>2</voice>", "<voice>9</voice>")
            .replace("<staff>1</staff>", "<staff>4</staff>")
            .replace("<staff>2</staff>", "<staff>6</staff>")
            .replace("<step>B</step>", "<step>C</step>")
            .replace("<step>F</step>", "<step>D</step>")
            .replace("<step>A</step>", "<step>E</step>");
        assert_eq!(
            matching_fingerprints(SYNTHETIC_SNAPSHOT),
            matching_fingerprints(&transformed)
        );
    }

    #[test]
    fn missing_overlap_rest_is_rejected_by_identity_free_detector() {
        let broken = SYNTHETIC_SNAPSHOT.replacen(
            "<note>\n    <rest/>\n    <duration>48</duration>",
            "<note print-object=\"no\">\n    <rest/>\n    <duration>48</duration>",
            1,
        );
        let (_views, audit) = adapt_structural_snapshot_read_only(&broken, 96, "negative");
        assert!(audit.parse_ok);
        assert_eq!(audit.identity_free_structure_count, 0);
        assert_eq!(audit.identity_free_candidate_count, 0);
    }

    #[test]
    fn adapter_is_audit_only_and_does_not_issue_writer_token() {
        let (_views, audit) =
            adapt_structural_snapshot_read_only(SYNTHETIC_SNAPSHOT, 96, "audit-only");
        assert_eq!(audit.verdict, "read_only_structural_snapshot_adapter_green");
        assert!(!audit.locator_identity_exposed_to_detector);
        assert!(!audit.writer_token_used);
        assert!(audit
            .pipeline_line("Synthetic Project Nr. 6")
            .contains("RenderXmlWrite nein"));
    }
}
