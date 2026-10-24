//! Audit-only renderer capability boundary for an authorized cross-staff beam.
//!
//! XML beam grammar and raw element contiguity are necessary but not sufficient
//! evidence that MuseScore will engrave one continuous cross-staff beam. This
//! module inspects only the already written candidate. It does not select a
//! musical repair, change XML, or infer success from beam tags alone.

use super::locator_resolution::{direct_child_i64, direct_child_text};
use roxmltree::{Document, Node};
use std::collections::{BTreeMap, BTreeSet};


#[derive(Debug, Clone, Default)]
pub struct RendererCrossStaffBeamCapabilityAudit {
    parse_ok: bool,
    primary_beam_groups: usize,
    cross_staff_beam_groups: usize,
    unique_cross_staff_target: bool,
    target_members: usize,
    target_xml_element_ordinals: Vec<usize>,
    target_xml_contiguous: bool,
    target_same_voice: bool,
    target_staff_count: usize,
    target_staff_transitions: usize,
    target_returns_to_origin_staff: bool,
    target_durations: Vec<i64>,
    target_types: Vec<String>,
    target_mixed_duration_beam: bool,
    target_primary_beam_grammar_green: bool,
    target_secondary_beam_grammar_green: bool,
    target_explicit_stems: usize,
    target_stem_directions: usize,
    target_consistent_explicit_stem_direction: bool,
    in_score_cross_staff_controls: usize,
    isolated_renderer_roundtrip_evidence_rows: usize,
    context_equivalent_renderer_evidence_rows: usize,
    renderer_primitive_capability_proven: bool,
    renderer_context_equivalence_proven: bool,
    xml_preconditions_green: bool,
    renderer_capability_proven: bool,
    isolated_renderer_probe_required: bool,
    release_hold_required: bool,
    #[cfg(test)]
    write_produced: bool,
    #[cfg(test)]
    verdict: String,
}

#[derive(Debug, Clone)]
struct BeamMember {
    element_ordinal: usize,
    voice: String,
    staff: String,
    duration: i64,
    note_type: String,
    stem: String,
    primary: String,
    secondary: Option<String>,
}

#[derive(Debug, Clone)]
struct BeamGroup {
    members: Vec<BeamMember>,
}

impl RendererCrossStaffBeamCapabilityAudit {
    pub fn green(&self) -> bool {
        self.renderer_capability_proven
    }

    #[cfg(test)]
    pub fn requires_release_hold(&self) -> bool {
        self.release_hold_required
    }

}

pub fn audit_renderer_cross_staff_beam_capability(
    post_writer_xml: &str,
    _source_stage: &str,
    isolated_renderer_roundtrip_evidence_rows: usize,
    context_equivalent_renderer_evidence_rows: usize,
) -> RendererCrossStaffBeamCapabilityAudit {
    let mut audit = RendererCrossStaffBeamCapabilityAudit {
        isolated_renderer_roundtrip_evidence_rows,
        context_equivalent_renderer_evidence_rows,
        #[cfg(test)]
        verdict: "renderer_cross_staff_beam_capability_not_audited".to_string(),
        ..RendererCrossStaffBeamCapabilityAudit::default()
    };

    let Ok(document) = Document::parse(post_writer_xml) else {
        audit.isolated_renderer_probe_required = true;
        audit.release_hold_required = true;
        #[cfg(test)]
        {
            audit.verdict = "renderer_cross_staff_beam_capability_xml_parse_failed".to_string();
        }
        return audit;
    };
    audit.parse_ok = true;

    let groups = collect_primary_beam_groups(&document);
    audit.primary_beam_groups = groups.len();
    let cross_staff = groups
        .into_iter()
        .filter(|group| distinct_staff_count(group) > 1)
        .collect::<Vec<_>>();
    audit.cross_staff_beam_groups = cross_staff.len();
    audit.unique_cross_staff_target = cross_staff.len() == 1;
    audit.in_score_cross_staff_controls = cross_staff.len().saturating_sub(1);

    let Some(target) = cross_staff.first() else {
        audit.isolated_renderer_probe_required = false;
        audit.release_hold_required = false;
        #[cfg(test)]
        {
            audit.verdict = "renderer_cross_staff_beam_capability_not_applicable_no_target".to_string();
        }
        return audit;
    };

    audit.target_members = target.members.len();
    audit.target_xml_element_ordinals = target
        .members
        .iter()
        .map(|member| member.element_ordinal)
        .collect();
    audit.target_xml_contiguous = target
        .members
        .windows(2)
        .all(|pair| pair[1].element_ordinal == pair[0].element_ordinal.saturating_add(1));
    audit.target_same_voice = target
        .members
        .first()
        .is_some_and(|first| {
                !first.voice.is_empty()
                    && target
                        .members
                        .iter()
                        .all(|member| member.voice == first.voice)
            },
        );
    audit.target_staff_count = distinct_staff_count(target);
    audit.target_staff_transitions = target
        .members
        .windows(2)
        .filter(|pair| pair[0].staff != pair[1].staff)
        .count();
    audit.target_returns_to_origin_staff = target.members.len() >= 3
        && target
            .members
            .first()
            .zip(target.members.last())
            .is_some_and(|(first, last)| {
                    first.staff == last.staff
                        && target
                            .members
                            .iter()
                            .skip(1)
                            .take(target.members.len().saturating_sub(2))
                            .any(|member| member.staff != first.staff)
                },
            );
    audit.target_durations = target
        .members
        .iter()
        .map(|member| member.duration)
        .collect();
    audit.target_types = target
        .members
        .iter()
        .map(|member| member.note_type.clone())
        .collect();
    audit.target_mixed_duration_beam = audit
        .target_durations
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .len()
        > 1;
    audit.target_primary_beam_grammar_green = primary_grammar_green(target);
    audit.target_secondary_beam_grammar_green = secondary_grammar_green(target);
    audit.target_explicit_stems = target
        .members
        .iter()
        .filter(|member| !member.stem.is_empty())
        .count();
    audit.target_stem_directions = target
        .members
        .iter()
        .filter_map(|member| (!member.stem.is_empty()).then_some(member.stem.as_str()))
        .collect::<BTreeSet<_>>()
        .len();
    audit.target_consistent_explicit_stem_direction =
        audit.target_explicit_stems == target.members.len() && audit.target_stem_directions == 1;

    audit.xml_preconditions_green = audit.unique_cross_staff_target
        && audit.target_members >= 2
        && audit.target_xml_contiguous
        && audit.target_same_voice
        && audit.target_staff_count == 2
        && audit.target_primary_beam_grammar_green
        && audit.target_secondary_beam_grammar_green;

    // v0.10.1165 proved the isolated A-B-A renderer primitive, but the same
    // structural class still fails in the productive full-score context.
    // Productive capability therefore requires both primitive evidence and a
    // context-equivalent positive row from the contextual probe matrix.
    audit.renderer_primitive_capability_proven =
        audit.xml_preconditions_green && audit.isolated_renderer_roundtrip_evidence_rows > 0;
    audit.renderer_context_equivalence_proven =
        audit.xml_preconditions_green && audit.context_equivalent_renderer_evidence_rows > 0;
    audit.renderer_capability_proven =
        audit.renderer_primitive_capability_proven && audit.renderer_context_equivalence_proven;
    audit.isolated_renderer_probe_required =
        audit.xml_preconditions_green && !audit.renderer_primitive_capability_proven;
    audit.release_hold_required =
        audit.unique_cross_staff_target && !audit.renderer_capability_proven;
    #[cfg(test)]
    {
        audit.verdict = if audit.renderer_capability_proven {
            "renderer_cross_staff_beam_capability_proven_by_context_equivalent_roundtrip".to_string()
        } else if audit.renderer_primitive_capability_proven {
            "renderer_cross_staff_beam_primitive_proven_but_context_equivalence_unproven".to_string()
        } else if audit.xml_preconditions_green {
            "renderer_cross_staff_beam_xml_green_but_renderer_primitive_unproven".to_string()
        } else {
            "renderer_cross_staff_beam_xml_preconditions_red".to_string()
        };
    }
    audit
}

fn collect_primary_beam_groups(document: &Document<'_>) -> Vec<BeamGroup> {
    let mut groups = Vec::new();
    for measure in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("measure"))
    {
        let mut active = BTreeMap::<String, Vec<BeamMember>>::new();
        for (element_ordinal, child) in measure.children().filter(Node::is_element).enumerate() {
            if !child.has_tag_name("note") {
                continue;
            }
            let voice = direct_child_text(child, "voice").unwrap_or_default();
            let primary = beam_state(child, "1");
            let Some(primary_state) = primary.as_deref() else {
                continue;
            };
            let member = BeamMember {
                element_ordinal,
                voice: voice.clone(),
                staff: direct_child_text(child, "staff").unwrap_or_default(),
                duration: direct_child_i64(child, "duration").unwrap_or(0),
                note_type: direct_child_text(child, "type").unwrap_or_default(),
                stem: direct_child_text(child, "stem").unwrap_or_default(),
                primary: primary_state.to_string(),
                secondary: beam_state(child, "2"),
            };
            match primary_state {
                "begin" => {
                    active.insert(voice, vec![member]);
                }
                "continue" => {
                    if let Some(group) = active.get_mut(&voice) {
                        group.push(member);
                    }
                }
                "end" => {
                    if let Some(mut group) = active.remove(&voice) {
                        group.push(member);
                        groups.push(BeamGroup { members: group });
                    }
                }
                _ => {}
            }
        }
    }
    groups
}

fn distinct_staff_count(group: &BeamGroup) -> usize {
    group
        .members
        .iter()
        .map(|member| member.staff.as_str())
        .filter(|staff| !staff.is_empty())
        .collect::<BTreeSet<_>>()
        .len()
}

fn primary_grammar_green(group: &BeamGroup) -> bool {
    if group.members.len() < 2 {
        return false;
    }
    group.members.first().map(|member| member.primary.as_str()) == Some("begin")
        && group.members.last().map(|member| member.primary.as_str()) == Some("end")
        && group
            .members
            .iter()
            .skip(1)
            .take(group.members.len().saturating_sub(2))
            .all(|member| member.primary == "continue")
}

fn secondary_grammar_green(group: &BeamGroup) -> bool {
    let secondary = group
        .members
        .iter()
        .filter_map(|member| member.secondary.as_deref())
        .collect::<Vec<_>>();
    if secondary.is_empty() {
        return true;
    }
    secondary.len() >= 2
        && secondary.first().copied() == Some("begin")
        && secondary.last().copied() == Some("end")
        && secondary
            .iter()
            .skip(1)
            .take(secondary.len().saturating_sub(2))
            .all(|state| *state == "continue")
}

fn beam_state(note: Node<'_, '_>, number: &str) -> Option<String> {
    note.children()
        .filter(Node::is_element)
        .find(|child| child.has_tag_name("beam") && child.attribute("number") == Some(number))
        .and_then(|beam| beam.text())
        .map(str::trim)
        .map(str::to_string)
}




#[cfg(test)]
mod tests {
    use super::*;

    const CROSS_STAFF: &str = r#"
<score-partwise version="4.0"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list><part id="P1"><measure number="x">
<note><pitch><step>D</step><octave>4</octave></pitch><duration>48</duration><voice>2</voice><type>eighth</type><stem>down</stem><staff>2</staff><beam number="1">begin</beam></note>
<note><pitch><step>B</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><stem>down</stem><staff>1</staff><beam number="1">continue</beam><beam number="2">begin</beam></note>
<note><pitch><step>B</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><stem>down</stem><staff>2</staff><beam number="1">end</beam><beam number="2">end</beam></note>
</measure></part></score-partwise>"#;

    #[test]
    fn xml_green_without_renderer_roundtrip_stays_blocked() {
        let audit = audit_renderer_cross_staff_beam_capability(CROSS_STAFF, "test", 0, 0);
        assert!(audit.xml_preconditions_green);
        assert!(!audit.green());
        assert!(audit.requires_release_hold());
        assert!(audit.target_returns_to_origin_staff);
        assert_eq!(audit.target_staff_transitions, 2);
        assert_eq!(audit.target_durations, vec![48, 24, 24]);
        assert_eq!(
            audit.verdict,
            "renderer_cross_staff_beam_xml_green_but_renderer_primitive_unproven"
        );
    }

    #[test]
    fn context_equivalent_roundtrip_evidence_can_close_capability_without_a_write() {
        let audit = audit_renderer_cross_staff_beam_capability(CROSS_STAFF, "test", 1, 1);
        assert!(audit.green());
        assert!(!audit.requires_release_hold());
        assert!(!audit.write_produced);
    }

    #[test]
    fn isolated_primitive_evidence_alone_stays_blocked_by_context() {
        let audit = audit_renderer_cross_staff_beam_capability(CROSS_STAFF, "test", 5, 0);
        assert!(audit.renderer_primitive_capability_proven);
        assert!(!audit.renderer_context_equivalence_proven);
        assert!(!audit.green());
        assert!(audit.requires_release_hold());
        assert_eq!(
            audit.verdict,
            "renderer_cross_staff_beam_primitive_proven_but_context_equivalence_unproven"
        );
    }

    #[test]
    fn same_staff_group_is_not_a_cross_staff_target() {
        let xml = CROSS_STAFF.replace("<staff>1</staff>", "<staff>2</staff>");
        let audit = audit_renderer_cross_staff_beam_capability(&xml, "test", 0, 0);
        assert_eq!(audit.cross_staff_beam_groups, 0);
        assert!(!audit.requires_release_hold());
        assert_eq!(
            audit.verdict,
            "renderer_cross_staff_beam_capability_not_applicable_no_target"
        );
    }
}
