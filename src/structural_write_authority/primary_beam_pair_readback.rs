//! Read-only pair observations at voice selection and the actual Master FinalChord projection.
use super::actual_source_beam_binding::BoundOccurrence;
use super::ordered_received_beam_readback::{
    OwnerEdgeReadback, Projection, closed_group, owner_groups, read_projected_edge,
};
use super::original_beam_voice_alignment::actual_beam_levels;
use super::*;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

#[derive(Debug, PartialEq, Eq)]
enum OriginalPair {
    Yes,
    No(&'static str),
    Open(&'static str),
}

#[derive(Clone, Copy)]
struct PrimaryMembership {
    group: (usize, usize),
    position: usize,
    closed: bool,
}

fn original_pair(
    left: Option<&PrimaryMembership>,
    right: Option<&PrimaryMembership>,
) -> OriginalPair {
    match (left, right) {
        (Some(left), Some(right)) if left.closed && right.closed => {
            if left.group != right.group {
                OriginalPair::No("different_original_primary_groups")
            } else if left.position.checked_add(1) != Some(right.position) {
                OriginalPair::No("not_direct_primary_neighbors")
            } else {
                OriginalPair::Yes
            }
        }
        (Some(_), Some(_)) => OriginalPair::Open("original_group_not_closed"),
        _ => OriginalPair::Open("original_primary_membership_unbound"),
    }
}

type Location = (usize, usize);
type SourceOccurrence = (usize, usize, usize);

pub(super) struct MasterOccurrences {
    pub(super) measures: Vec<FinalChordBindingMeasure>,
    pub(super) by_source_occurrence: BTreeMap<SourceOccurrence, Location>,
}

impl MasterOccurrences {
    pub(super) fn bind(
        plan: &FinalChordEncodingPlan,
        before: &str,
        after: &str,
        receipt: Option<&ExactTransformationReceipt>,
    ) -> Result<Self, &'static str> {
        if before.len() != plan.source_len || fingerprint(before) != plan.source_fingerprint {
            return Err("foreign_master_input");
        }
        let inputs = plan
            .original_course_bindings
            .as_ref()
            .and_then(|rows| rows.as_ref().ok())
            .ok_or("master_input_occurrences_unbound")?;
        let measures =
            final_chord_parse_binding_measures(after).map_err(|_| "master_xml_unreadable")?;
        let capture = match receipt {
            Some(receipt) => {
                let capture = receipt
                    .final_chord_capture()
                    .and_then(|capture| capture.as_ref().ok())
                    .ok_or("master_course_capture_unavailable")?;
                if capture.validate_execution(receipt, before).ok() != Some(after) {
                    return Err("master_course_capture_foreign");
                }
                Some(capture)
            }
            None if before == after => None,
            None => return Err("changed_master_without_receipt"),
        };
        let rows: BTreeMap<_, _> = capture
            .into_iter()
            .flat_map(|capture| capture.rows())
            .map(|row| ((row.input().start, row.input().end), row))
            .collect();
        let locations: BTreeMap<_, _> = measures
            .iter()
            .enumerate()
            .flat_map(|(measure, data)| {
                data.notes.iter().enumerate().map(move |(index, note)| {
                    ((note.range.start, note.range.end), (measure, index))
                })
            })
            .collect();
        let by_source_occurrence = inputs
            .input_occurrences()
            .filter_map(|input| {
                let range = (input.source_note_start, input.source_note_end);
                let output = if capture.is_some() {
                    let row = rows.get(&range)?;
                    // Only a unique pitched ordinal is sufficient here. In
                    // particular, a source that emitted a tied chain stays open.
                    let [(range, false)] = row.outputs() else {
                        return None;
                    };
                    if row.source() != Some(input.source_event_id) || row.emission_ordinals() != [0]
                    {
                        return None;
                    }
                    (range.start, range.end)
                } else {
                    range
                };
                let &(measure, index) = locations.get(&output)?;
                let notes = &measures[measure].notes;
                let note = &notes[index];
                let head = if note.chord_member {
                    note.anchor_index?
                } else {
                    index
                };
                let carrier = notes.get(head)?;
                (!carrier.chord_member
                    && !carrier.rest_like
                    && carrier.pitch_midi.is_some()
                    && carrier.duration > 0
                    && carrier.onset == note.onset
                    && carrier.staff == note.staff
                    && carrier.voice == note.voice)
                    .then_some((
                        (input.source_event_id, input.attack_id, input.fragment_id),
                        (measure, head),
                    ))
            })
            .collect();
        Ok(Self {
            measures,
            by_source_occurrence,
        })
    }

    fn locate(&self, locator: &FinalChordCanonicalSourceLocator) -> Option<Location> {
        self.by_source_occurrence
            .get(&(
                locator.source_event_id,
                locator.attack_id,
                locator.fragment_id,
            ))
            .copied()
    }
}

impl MasterOccurrences {
    pub(super) fn read_edge(
        &self,
        xml: &str,
        edge: &super::ordered_received_beam_readback::OwnerEdge<'_>,
        locators: &BTreeMap<usize, &FinalChordCanonicalSourceLocator>,
    ) -> OwnerEdgeReadback {
        let locate = |endpoint: &BoundOccurrence| {
            let locator = locators.get(&endpoint.fragment)?;
            (locator.source_event_id == endpoint.source && locator.attack_id == endpoint.attack)
                .then_some(self.locate(locator)).flatten()
        };
        locate(edge.left).zip(locate(edge.right)).map_or(OwnerEdgeReadback::Unknown, |(left, right)| {
            read_pair(xml, &self.measures, left, right)
        })
    }
}

fn read_pair(
    xml: &str,
    measures: &[FinalChordBindingMeasure],
    left: Location,
    right: Location,
) -> OwnerEdgeReadback {
    if left.0 != right.0 || left == right {
        return OwnerEdgeReadback::Unknown;
    }
    let notes = &measures[left.0].notes;
    let (a, b) = (&notes[left.1], &notes[right.1]);
    if a.staff == b.staff
        && !a.voice.is_empty()
        && !b.voice.is_empty()
        && a.onset
            .checked_add(a.duration)
            .is_some_and(|end| end <= b.onset)
        && a.voice != b.voice
    {
        return OwnerEdgeReadback::SplitVoice;
    }
    read_projected_edge(notes, left.1, right.1, 1, &|note| {
        actual_beam_levels(xml, note).map(Cow::Owned)
    })
}

fn first_observed(
    pre: OwnerEdgeReadback,
    plan: Option<(&str, &str)>,
    master: OwnerEdgeReadback,
) -> &'static str {
    if pre == OwnerEdgeReadback::SplitVoice {
        return "preprojection_already_split";
    }
    match plan {
        Some((left, right)) if left != right => "split_in_selected_plan",
        Some(_) if pre == OwnerEdgeReadback::Present => match master {
            OwnerEdgeReadback::SplitVoice => "split_in_master_projection",
            OwnerEdgeReadback::Interrupted { .. }
            | OwnerEdgeReadback::MarkedRest { .. }
            | OwnerEdgeReadback::EndpointMarkersMissing => "edge_lost_by_master_projection",
            _ => "unassigned",
        },
        _ => "unassigned",
    }
}

fn primary_directions(level: Option<&str>) -> (bool, bool) {
    (
        matches!(level, Some("begin" | "continue")),
        matches!(level, Some("end" | "continue")),
    )
}

fn beam_head_label<'plan>(
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &'plan FrozenSerializedVoicePlan,
    locator: &FinalChordCanonicalSourceLocator,
) -> Option<&'plan str> {
    // A source label also identifies the carrier label for a head whose
    // structural representation is retained.
    if locator.chord_syntax
        || canonical
            .actions
            .iter()
            .any(|action| action.fragment_id == locator.fragment_id)
    {
        return None;
    }
    voice_plan.label_for_source(locator.source_event_id)
}

type PairLabels<'a> = (Option<&'a str>, Option<&'a str>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VoiceSelectionChange {
    SameVoiceRetained,
    SplitByReplacement,
    AlreadySplit,
    RejoinedInReplacement,
    Open,
}

fn voice_selection_change(before: PairLabels<'_>, chosen: PairLabels<'_>) -> VoiceSelectionChange {
    let (Some(a), Some(b)) = before else {
        return VoiceSelectionChange::Open;
    };
    let (Some(c), Some(d)) = chosen else {
        return VoiceSelectionChange::Open;
    };
    match (a == b, c == d) {
        (true, true) => VoiceSelectionChange::SameVoiceRetained,
        (true, false) => VoiceSelectionChange::SplitByReplacement,
        (false, false) => VoiceSelectionChange::AlreadySplit,
        (false, true) => VoiceSelectionChange::RejoinedInReplacement,
    }
}

struct PrimaryBeamVoiceSelectionRow<'a> {
    fragments: (usize, usize),
    endpoints: (
        Option<&'a FinalChordCanonicalSourceLocator>,
        Option<&'a FinalChordCanonicalSourceLocator>,
    ),
    baseline: PairLabels<'a>,
}

pub(crate) struct PrimaryBeamVoiceSelection<'a> {
    canonical: &'a CanonicalFinalChordStructuralPlan,
    rows: Result<Vec<PrimaryBeamVoiceSelectionRow<'a>>, &'static str>,
}

impl CanonicalFinalChordStructuralPlan {
    pub(crate) fn observe_primary_beam_voice_selection<'a>(
        &'a self,
        capture: &ActualSourceBeamBindingObservation,
        baseline: &'a FrozenSerializedVoicePlan,
    ) -> PrimaryBeamVoiceSelection<'a> {
        let rows = capture.graph_received_occurrences(self).map(|rows| {
            let locators: BTreeMap<_, _> = self
                .source_locators
                .iter()
                .map(|row| ((row.source_event_id, row.attack_id, row.fragment_id), row))
                .collect();
            let bind = |row: &BoundOccurrence| {
                locators
                    .get(&(row.source, row.attack, row.fragment))
                    .copied()
            };
            let label = |row| beam_head_label(self, baseline, row);
            let mut observations = vec![];
            for ((level, _), members) in owner_groups(rows) {
                if level != 1 || !closed_group(&members) {
                    continue;
                }
                for pair in members.windows(2) {
                    let (left, right) = (pair[0].0, pair[1].0);
                    let (a, b) = (bind(left), bind(right));
                    observations.push(PrimaryBeamVoiceSelectionRow {
                        fragments: (left.fragment, right.fragment),
                        endpoints: (a, b),
                        baseline: (a.and_then(label), b.and_then(label)),
                    });
                }
            }
            observations
        });
        PrimaryBeamVoiceSelection {
            canonical: self,
            rows,
        }
    }
}

impl PrimaryBeamVoiceSelection<'_> {
    pub(crate) fn append(self, out: &mut String, replacement: Option<&FrozenSerializedVoicePlan>) {
        let adopted = replacement.is_some();
        let rows = match self.rows {
            Ok(rows) => rows,
            Err(reason) => {
                let _ = writeln!(
                    out,
                    "PrimaryBeamVoiceSelection: replacement_adopted={adopted}; unavailable={reason}; authority=none"
                );
                return;
            }
        };
        let mut counts = [0; 5];
        let pairs = rows.len();
        for row in rows {
            let chosen = replacement.map_or(row.baseline, |plan| {
                let label = |locator| beam_head_label(self.canonical, plan, locator);
                (
                    row.endpoints.0.and_then(label),
                    row.endpoints.1.and_then(label),
                )
            });
            let change = voice_selection_change(row.baseline, chosen);
            counts[change as usize] += 1;
            let _ = writeln!(
                out,
                "PrimaryBeamVoiceSelectionPair: fragments={:?}; baseline_labels={:?}; chosen_labels={chosen:?}; result={change:?}; authority=none",
                row.fragments, row.baseline
            );
        }
        let [
            same_voice_retained,
            split_by_replacement,
            already_split,
            rejoined_in_replacement,
            open,
        ] = counts;
        let _ = writeln!(
            out,
            "PrimaryBeamVoiceSelection: replacement_adopted={adopted}; pairs={pairs}; same_voice_retained={same_voice_retained}; already_split={already_split}; split_by_replacement={split_by_replacement}; rejoined_in_replacement={rejoined_in_replacement}; open={open}; authority=none"
        );
    }
}

impl FinalChordEncodingPlan {
    pub(crate) fn append_primary_beam_pair_readback(
        &self,
        out: &mut String,
        canonical: &CanonicalFinalChordStructuralPlan,
        capture: &ActualSourceBeamBindingObservation,
        voice_plan: &FrozenSerializedVoicePlan,
        master_execution: (&str, &str, Option<&ExactTransformationReceipt>),
    ) {
        self.append_primary_beam_tie_trials(out, canonical, capture, master_execution);
        let (before, after, receipt) = master_execution;
        let Some(written) = canonical.written_projection.as_ref() else {
            let _ = writeln!(
                out,
                "PrimaryBeamPairReadback: unavailable=preprojection_unbound; authority=none"
            );
            return;
        };
        let rows = capture.graph_received_occurrences(canonical);
        if let Err(reason) = rows {
            let _ = writeln!(
                out,
                "PrimaryBeamPairReadback: original_capture={reason}; authority=none"
            );
        }
        let rows = rows.unwrap_or_default();
        let original: BTreeMap<_, _> = rows.iter().map(|row| (row.fragment, row)).collect();
        let pre = Projection::bind(written);
        let master = if before == written.original_xml.as_ref() {
            MasterOccurrences::bind(self, before, after, receipt)
        } else {
            Err("master_and_preprojection_have_different_sources")
        };
        if let Err(reason) = &master {
            let _ = writeln!(
                out,
                "PrimaryBeamPairReadback: master_binding={reason}; authority=none"
            );
        }
        let master = master.ok();
        let locators: BTreeMap<_, _> = canonical
            .source_locators
            .iter()
            .map(|locator| (locator.fragment_id, locator))
            .collect();

        let mut source_order: Vec<_> = canonical.source_locators.iter().collect();
        source_order.sort_unstable_by_key(|locator| (locator.measure, locator.raw_note_index));
        let score_ordinals: BTreeMap<_, _> = source_order
            .into_iter()
            .enumerate()
            .map(|(ordinal, locator)| (locator.fragment_id, ordinal))
            .collect();
        let mut membership = BTreeMap::new();
        let mut pairs = BTreeSet::new();
        for ((level, group), members) in owner_groups(rows) {
            if level != 1 {
                continue;
            }
            let closed = closed_group(&members);
            for (position, (row, _)) in members.iter().enumerate() {
                membership.insert(
                    row.fragment,
                    PrimaryMembership {
                        group,
                        position,
                        closed,
                    },
                );
            }
            pairs.extend(
                members
                    .windows(2)
                    .map(|pair| (pair[0].0.fragment, pair[1].0.fragment)),
            );
        }
        // Cross-group candidates are included for diagnosis even if no owner
        // edge was admitted. Use direction and time, never the voice label.
        let mut marked_lanes = BTreeMap::<_, Vec<_>>::new();
        let mut bound_master_carriers = BTreeSet::new();
        for (&fragment, locator) in &locators {
            let original_level = original.get(&fragment).and_then(|row| {
                row.levels
                    .iter()
                    .find(|(level, _, _)| *level == 1)?
                    .2
                    .as_deref()
            });
            let master_levels = master.as_ref().and_then(|master| {
                let (measure, index) = master.locate(locator)?;
                bound_master_carriers.insert((measure, index));
                actual_beam_levels(after, &master.measures[measure].notes[index])
            });
            let (outgoing, incoming) = primary_directions(original_level);
            let (master_out, master_in) = primary_directions(
                master_levels
                    .as_ref()
                    .and_then(|levels| levels.get(&1))
                    .map(String::as_str),
            );
            if outgoing || incoming || master_out || master_in {
                marked_lanes
                    .entry((locator.measure, locator.staff.as_str()))
                    .or_default()
                    .push((locator, outgoing || master_out, incoming || master_in));
            }
        }
        for lane in marked_lanes.values() {
            for (left, _, _) in lane.iter().filter(|(_, outgoing, _)| *outgoing) {
                for (right, _, _) in lane.iter().filter(|(_, _, incoming)| *incoming) {
                    if left.fragment_id != right.fragment_id
                        && left
                            .onset
                            .checked_add(left.duration)
                            .is_some_and(|end| end <= right.onset)
                    {
                        pairs.insert((left.fragment_id, right.fragment_id));
                    }
                }
            }
        }
        if let Some(master) = &master {
            for (measure, data) in master.measures.iter().enumerate() {
                for (index, note) in data.notes.iter().enumerate() {
                    if !bound_master_carriers.contains(&(measure, index))
                        && !note.chord_member
                        && actual_beam_levels(after, note)
                            .is_some_and(|levels| levels.contains_key(&1))
                    {
                        let _ = writeln!(
                            out,
                            "PrimaryBeamUnboundMasterCarrier: measure={}; staff={}; onset={}; pitch={:?}; note_index={}; reason=canonical_output_unbound; authority=none",
                            data.measure,
                            note.staff,
                            note.onset,
                            note.pitch_midi,
                            note.raw_note_index
                        );
                    }
                }
            }
        }
        let _ = writeln!(
            out,
            "PrimaryBeamPairReadback: pairs={}; master_seam=before_countervoice_stems; authority=none",
            pairs.len()
        );
        for (left, right) in pairs {
            let (Some(a), Some(b)) = (locators.get(&left), locators.get(&right)) else {
                continue;
            };
            let same_group = original_pair(membership.get(&left), membership.get(&right));
            let (answer, reason) = match same_group {
                OriginalPair::Yes => ("yes", "direct_primary_neighbors"),
                OriginalPair::No(reason) => ("no", reason),
                OriginalPair::Open(reason) => ("open", reason),
            };
            let _ = writeln!(
                out,
                "PrimaryBeamPairOrigin: answer={answer}; reason={reason}; measure={}; staff={}; onsets={:?}; pitches={:?}; source_pitched_ordinals={:?}; source_attacks_fragments={:?}; authority=none",
                a.measure,
                a.staff,
                (a.onset, b.onset),
                (a.pitch_midi, b.pitch_midi),
                (score_ordinals[&left], score_ordinals[&right]),
                [
                    (a.source_event_id, a.attack_id, left),
                    (b.source_event_id, b.attack_id, right)
                ]
            );
            if same_group != OriginalPair::Yes {
                continue;
            }
            let pre_state = pre
                .as_ref()
                .and_then(|pre| {
                    let locate = |id| pre.locate(original.get(&id).copied()?);
                    Some(read_pair(
                        &written.xml,
                        &pre.measures,
                        locate(left)?,
                        locate(right)?,
                    ))
                })
                .unwrap_or(OwnerEdgeReadback::Unknown);
            let plan = beam_head_label(canonical, voice_plan, a)
                .zip(beam_head_label(canonical, voice_plan, b));
            let master_state = master
                .as_ref()
                .and_then(|master| {
                    Some(read_pair(
                        after,
                        &master.measures,
                        master.locate(a)?,
                        master.locate(b)?,
                    ))
                })
                .unwrap_or(OwnerEdgeReadback::Unknown);
            let observed = first_observed(pre_state, plan, master_state);
            let _ = writeln!(
                out,
                "PrimaryBeamPairProjection: fragments={:?}; pre={pre_state:?}; plan_labels={plan:?}; master={master_state:?}; first_observed={observed}; authority=none",
                (left, right)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voice_selection_preserves_the_pair_relation_under_label_renaming() {
        use VoiceSelectionChange::*;
        let labels = |left, right| (Some(left), Some(right));
        assert_eq!(
            voice_selection_change(labels("1", "1"), labels("3", "3")),
            SameVoiceRetained
        );
        assert_eq!(
            voice_selection_change(labels("1", "1"), labels("3", "4")),
            SplitByReplacement
        );
        assert_eq!(
            voice_selection_change(labels("1", "2"), labels("3", "4")),
            AlreadySplit
        );
        assert_eq!(
            voice_selection_change(labels("1", "2"), labels("3", "3")),
            RejoinedInReplacement
        );
        assert_eq!(
            voice_selection_change(labels("1", "2"), (Some("3"), None)),
            Open
        );
    }

    #[test]
    fn adoption_is_reported_even_when_a_pair_keeps_its_labels() {
        let xml = &format!(
            r#"<score-partwise><part id="synthetic"><measure number="{}">
            <attributes><divisions>2</divisions></attributes>
            <note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>eighth</type><staff>1</staff></note>
            <note><pitch><step>D</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>eighth</type><staff>1</staff></note>
            </measure></part></score-partwise>"#,
            61
        );
        let evidence = final_chord_declassified_evidence_from_xml(xml).unwrap();
        let (canonical, _) = plan_canonical_final_chord_structure_once(xml, &evidence).unwrap();
        let [a, b] = canonical.source_locators.as_ref() else {
            panic!("two bound pitches expected");
        };
        let plan = FrozenSerializedVoicePlan {
            source_to_label: [
                (a.source_event_id, "1".to_owned()),
                (b.source_event_id, "1".to_owned()),
            ]
            .into(),
            ..Default::default()
        };
        for replacement in [None, Some(&plan)] {
            let readback = PrimaryBeamVoiceSelection {
                canonical: &canonical,
                rows: Ok(vec![PrimaryBeamVoiceSelectionRow {
                    fragments: (a.fragment_id, b.fragment_id),
                    endpoints: (Some(a), Some(b)),
                    baseline: (Some("1"), Some("1")),
                }]),
            };
            let mut out = String::new();
            readback.append(&mut out, replacement);
            assert!(out.contains(&format!(
                "replacement_adopted={}; pairs=1; same_voice_retained=1",
                replacement.is_some()
            )));
            assert!(out.contains("result=SameVoiceRetained"));
        }
    }

    #[test]
    fn direct_membership_distinguishes_disproved_and_open_pairs() {
        let first = PrimaryMembership {
            group: (0, 0),
            position: 0,
            closed: true,
        };
        let next = PrimaryMembership {
            position: 1,
            ..first
        };
        let later = PrimaryMembership {
            position: 2,
            ..first
        };
        let separate = PrimaryMembership {
            group: (0, 1),
            ..next
        };
        let unclosed = PrimaryMembership {
            closed: false,
            ..next
        };
        assert_eq!(original_pair(Some(&first), Some(&next)), OriginalPair::Yes);
        assert_eq!(
            original_pair(Some(&first), Some(&later)),
            OriginalPair::No("not_direct_primary_neighbors")
        );
        assert_eq!(
            original_pair(Some(&first), Some(&separate)),
            OriginalPair::No("different_original_primary_groups")
        );
        assert!(matches!(
            original_pair(Some(&first), Some(&unclosed)),
            OriginalPair::Open(_)
        ));
        assert!(matches!(
            original_pair(Some(&first), None),
            OriginalPair::Open(_)
        ));
    }

    #[test]
    fn distinguishes_selected_voice_plan_from_master_projection() {
        for number in [17, 83] {
            let xml = format!(
                r#"<score-partwise><part id="P"><measure number="{number}">
            <note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>eighth</type><staff>1</staff><beam number="1">begin</beam></note>
            <note><pitch><step>D</step><octave>4</octave></pitch><duration>1</duration><voice>2</voice><type>eighth</type><staff>1</staff><beam number="1">end</beam></note>
            </measure></part></score-partwise>"#
            );
            let measures = final_chord_parse_binding_measures(&xml).unwrap();
            let split = read_pair(&xml, &measures, (0, 0), (0, 1));
            assert_eq!(split, OwnerEdgeReadback::SplitVoice);
            let connected = xml.replace("<voice>2</voice>", "<voice>1</voice>");
            let measures = final_chord_parse_binding_measures(&connected).unwrap();
            let pre = read_pair(&connected, &measures, (0, 0), (0, 1));
            assert_eq!(pre, OwnerEdgeReadback::Present);
            assert_eq!(
                first_observed(pre, Some(("1", "1")), split),
                "split_in_master_projection"
            );
            assert_eq!(
                first_observed(pre, Some(("1", "2")), split),
                "split_in_selected_plan"
            );
            assert_eq!(first_observed(pre, None, split), "unassigned");
            assert_eq!(
                first_observed(split, Some(("1", "2")), split),
                "preprojection_already_split"
            );
        }
    }

    #[test]
    fn actual_master_receipt_binds_pitched_outputs_and_rejects_foreign_bytes() {
        let number = 61;
        let xml = &format!(
            r#"<score-partwise><part id="synthetic"><measure number="{number}">
            <attributes><divisions>2</divisions><time><beats>2</beats><beat-type>4</beat-type></time></attributes>
            <note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><voice>7</voice><type>eighth</type><staff>1</staff><beam number="1">begin</beam></note>
            <note><pitch><step>D</step><octave>4</octave></pitch><duration>1</duration><voice>7</voice><type>eighth</type><staff>1</staff><beam number="1">end</beam></note>
            </measure></part></score-partwise>"#
        );
        let evidence = final_chord_declassified_evidence_from_xml(xml).unwrap();
        let (canonical, _) = plan_canonical_final_chord_structure_once(xml, &evidence).unwrap();
        let attacks = evidence
            .iter()
            .enumerate()
            .map(|(index, row)| FrozenVoiceAttackEvidenceRow {
                source_event_id: row.source_event_id,
                attack_id: row.attack_id,
                absolute_start: index as i64,
                absolute_end: index as i64 + 1,
                staff: row.staff,
                technical_lane: 1,
            })
            .collect::<Vec<_>>();
        let channels =
            build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
        let labels = bind_musicxml_voice_labels(&channels, &[]).unwrap();
        let plan = bind_canonical_final_chord_encoding_plan(xml, &canonical, &labels, false).unwrap();
        let (after, receipt) = project_final_chord_encoding_plan(xml, &plan);
        let after = after.unwrap();
        assert!(receipt.is_some());
        let binding = MasterOccurrences::bind(&plan, xml, &after, receipt.as_ref()).unwrap();
        let [left, right] = canonical.source_locators.as_ref() else {
            panic!("two bound pitches expected");
        };
        let left = binding.locate(left).unwrap();
        let right = binding.locate(right).unwrap();
        assert_eq!(
            read_pair(&after, &binding.measures, left, right),
            OwnerEdgeReadback::Present
        );
        assert!(MasterOccurrences::bind(&plan, xml, &after, None).is_err());
        assert!(MasterOccurrences::bind(&plan, xml, &(after + " "), receipt.as_ref()).is_err());
        let noop = MasterOccurrences::bind(&plan, xml, xml, None).unwrap();
        assert_eq!(
            read_pair(xml, &noop.measures, left, right),
            OwnerEdgeReadback::Present
        );
    }
}
