use super::{
    adapt_structural_snapshot_read_only, audit_identity_free_semantic_event_equivalence,
    detect_structural_repair_intents_read_only, GenericityEvidence, LocatedStructuralView,
    StructuralCarrierRole,
};

#[derive(Debug, Clone)]
struct PositiveVariantRow {
    #[cfg(test)]
    transformation_applied: bool,
    candidate_count: usize,
    repair_intent_stable: bool,
    serialization_plan_type_stable: bool,
    semantic_event_delta_rows: usize,
    serialization_encoding_delta_rows: usize,
    sounding_identity_delta_rows: usize,
    required_chord_normalizations: usize,
    proven_chord_normalizations: usize,
    artificial_chord_additions: usize,
    detector_identity_fields_exposed: bool,
    passed: bool,
}

#[derive(Debug, Clone)]
struct NegativeVariantRow {
    #[cfg(test)]
    writer_token_used: bool,
    #[cfg(test)]
    write_produced: bool,
    rejected: bool,
}

#[derive(Debug, Clone, Default)]
pub struct IdentityFreeMetamorphicSafetyNetAudit {
    source_parse_ok: bool,
    baseline_candidate_count: usize,
    baseline_serialization_plan_feasible: bool,
    positive_variants: Vec<PositiveVariantRow>,
    negative_variants: Vec<NegativeVariantRow>,
    positive_passed: usize,
    negative_rejected: usize,
    candidate_count_stable: bool,
    repair_intent_stable: bool,
    serialization_plan_type_stable: bool,
    semantic_event_invariants_stable: bool,
    encoding_normalization_stable: bool,
    negatives_rejected: bool,
    independent_candidate_duplication_invariant: bool,
    detector_identity_fields_exposed: bool,
    genericity_evidence_complete: bool,
    authorization_attempted: bool,
    writer_token_used: bool,
    write_produced: bool,
    #[cfg(test)]
    verdict: String,
}

impl IdentityFreeMetamorphicSafetyNetAudit {
    pub fn authorization_evidence(&self) -> GenericityEvidence {
        GenericityEvidence {
            real_pipeline_snapshot_passed: self.source_parse_ok
                && self.baseline_candidate_count > 0
                && self.baseline_serialization_plan_feasible,
            metamorphic_positive_variants: self.positive_passed,
            metamorphic_negative_variants: self.negative_rejected,
            positive_delta_kind_stable: self.candidate_count_stable
                && self.repair_intent_stable
                && self.serialization_plan_type_stable
                && self.semantic_event_invariants_stable
                && self.encoding_normalization_stable,
            negatives_rejected: self.negatives_rejected,
            independent_candidate_duplication_invariant:
                self.independent_candidate_duplication_invariant,
            candidate_count: self.baseline_candidate_count,
        }
    }

    pub fn authorizes_token_origin(&self) -> bool {
        self.genericity_evidence_complete
            && self.authorization_evidence().authorizes_write()
            && !self.detector_identity_fields_exposed
            && !self.authorization_attempted
            && !self.writer_token_used
            && !self.write_produced
    }
}

pub fn audit_identity_free_metamorphic_safety_net(
    xml: &str,
    fallback_divisions: i64,
    source_stage: &str,
) -> IdentityFreeMetamorphicSafetyNetAudit {
    let (baseline_views, baseline_adapter) =
        adapt_structural_snapshot_read_only(xml, fallback_divisions, source_stage);
    let baseline_intents = detect_structural_repair_intents_read_only(&baseline_views);
    let baseline_evidence_rows = baseline_intents
        .iter()
        .map(|intent| {
            audit_identity_free_semantic_event_equivalence(
                std::slice::from_ref(intent),
                1,
                source_stage,
            )
            .metamorphic_evidence_snapshot()
        })
        .collect::<Vec<_>>();
    let mut baseline_plan_fingerprints = baseline_evidence_rows
        .iter()
        .map(|row| row.abstract_plan_fingerprint.clone())
        .collect::<Vec<_>>();
    baseline_plan_fingerprints.sort();
    let baseline_serialization_plan_feasible = !baseline_evidence_rows.is_empty()
        && baseline_evidence_rows
            .iter()
            .all(|row| row.serialization_plan_feasible);

    let baseline_candidate_views = baseline_views
        .iter()
        .filter(|view| {
            detect_structural_repair_intents_read_only(std::slice::from_ref(*view)).len() == 1
        })
        .cloned()
        .collect::<Vec<_>>();

    let independent_candidate_duplication_invariant = !baseline_candidate_views.is_empty()
        && baseline_candidate_views.iter().all(|candidate| {
            let mut duplicate = candidate.clone();
            let ordinal_offset = baseline_views
                .iter()
                .flat_map(|view| view.locator.source_events.iter())
                .map(|event| event.event_ordinal)
                .max()
                .unwrap_or(0)
                .saturating_add(1);
            for source_event in &mut duplicate.locator.source_events {
                source_event.event_ordinal =
                    source_event.event_ordinal.saturating_add(ordinal_offset);
            }
            let duplicated_views = vec![candidate.clone(), duplicate];
            let duplicated_intents = detect_structural_repair_intents_read_only(&duplicated_views);
            duplicated_intents.len() == 2
                && duplicated_intents.iter().all(|intent| {
                    audit_identity_free_semantic_event_equivalence(
                        std::slice::from_ref(intent),
                        1,
                        "independent-candidate-duplication-probe",
                    )
                    .metamorphic_evidence_snapshot()
                    .serialization_plan_feasible
                })
        });

    let mut audit = IdentityFreeMetamorphicSafetyNetAudit {
        source_parse_ok: baseline_adapter.parse_ok(),
        baseline_candidate_count: baseline_intents.len(),
        baseline_serialization_plan_feasible,
        independent_candidate_duplication_invariant,
        detector_identity_fields_exposed: baseline_evidence_rows
            .iter()
            .any(|row| row.detector_identity_fields_exposed),
        authorization_attempted: false,
        writer_token_used: false,
        write_produced: false,
        #[cfg(test)]
        verdict: "identity_free_metamorphic_safety_net_incomplete".to_string(),
        ..IdentityFreeMetamorphicSafetyNetAudit::default()
    };

    let positive_xml_variants = positive_variants(xml);
    for (name, transformed_xml, transformation_applied) in positive_xml_variants {
        let (variant_views, _) = adapt_structural_snapshot_read_only(
            &transformed_xml,
            fallback_divisions,
            name,
        );
        let variant_intents = detect_structural_repair_intents_read_only(&variant_views);
        let variant_evidence_rows = variant_intents
            .iter()
            .map(|intent| {
                audit_identity_free_semantic_event_equivalence(
                    std::slice::from_ref(intent),
                    1,
                    name,
                )
                .metamorphic_evidence_snapshot()
            })
            .collect::<Vec<_>>();
        let mut variant_plan_fingerprints = variant_evidence_rows
            .iter()
            .map(|row| row.abstract_plan_fingerprint.clone())
            .collect::<Vec<_>>();
        variant_plan_fingerprints.sort();

        let repair_intent_stable =
            repair_intent_name(&variant_intents) == repair_intent_name(&baseline_intents);
        let serialization_plan_type_stable =
            variant_plan_fingerprints == baseline_plan_fingerprints;
        let candidate_count = variant_intents.len();
        let semantic_event_delta_rows = variant_evidence_rows
            .iter()
            .map(|row| row.semantic_complete_event_delta_rows)
            .sum::<usize>();
        let serialization_encoding_delta_rows = variant_evidence_rows
            .iter()
            .map(|row| row.serialization_encoding_delta_rows)
            .sum::<usize>();
        let sounding_identity_delta_rows = variant_evidence_rows
            .iter()
            .map(|row| row.sounding_identity_delta_rows)
            .sum::<usize>();
        let required_chord_normalizations = variant_evidence_rows
            .iter()
            .map(|row| row.required_chord_normalizations)
            .sum::<usize>();
        let proven_chord_normalizations = variant_evidence_rows
            .iter()
            .map(|row| row.proven_chord_normalizations)
            .sum::<usize>();
        let artificial_chord_additions = variant_evidence_rows
            .iter()
            .map(|row| row.artificial_chord_additions)
            .sum::<usize>();
        let detector_identity_fields_exposed = variant_evidence_rows
            .iter()
            .any(|row| row.detector_identity_fields_exposed);
        let passed = transformation_applied
            && candidate_count == baseline_intents.len()
            && candidate_count > 0
            && repair_intent_stable
            && serialization_plan_type_stable
            && variant_evidence_rows
                .iter()
                .all(|row| row.serialization_plan_feasible)
            && semantic_event_delta_rows == 0
            && serialization_encoding_delta_rows == candidate_count.saturating_mul(2)
            && sounding_identity_delta_rows == 0
            && required_chord_normalizations == candidate_count
            && proven_chord_normalizations == candidate_count
            && variant_evidence_rows
                .iter()
                .all(|row| row.unproven_chord_normalizations == 0)
            && artificial_chord_additions == 0
            && !detector_identity_fields_exposed
            && variant_evidence_rows.iter().all(|row| !row.writer_token_used)
            && variant_evidence_rows.iter().all(|row| !row.write_produced);
        audit.positive_variants.push(PositiveVariantRow {
            #[cfg(test)]
            transformation_applied,
            candidate_count,
            repair_intent_stable,
            serialization_plan_type_stable,
            semantic_event_delta_rows,
            serialization_encoding_delta_rows,
            sounding_identity_delta_rows,
            required_chord_normalizations,
            proven_chord_normalizations,
            artificial_chord_additions,
            detector_identity_fields_exposed,
            passed,
        });
    }

    for candidate in &baseline_candidate_views {
        for (name, views) in negative_variants(candidate) {
            let intents = detect_structural_repair_intents_read_only(&views);
            let evidence_rows = intents
                .iter()
                .map(|intent| {
                    audit_identity_free_semantic_event_equivalence(
                        std::slice::from_ref(intent),
                        views.len(),
                        name,
                    )
                    .metamorphic_evidence_snapshot()
                })
                .collect::<Vec<_>>();
            let local_conflict = intents.iter().enumerate().any(|(index, left)| {
                intents.iter().skip(index.saturating_add(1)).any(|right| {
                    let left_locator = &left.located_view.locator;
                    let right_locator = &right.located_view.locator;
                    left_locator.source_events.iter().any(|left_event| {
                        right_locator
                            .source_events
                            .iter()
                            .any(|right_event| {
                                left_event.event_ordinal == right_event.event_ordinal
                            })
                    })
                })
            });
            let rejected = (intents.is_empty()
                || local_conflict
                || evidence_rows
                    .iter()
                    .any(|row| !row.serialization_plan_feasible))
                && evidence_rows.iter().all(|row| !row.writer_token_used)
                && evidence_rows.iter().all(|row| !row.write_produced);
            audit.negative_variants.push(NegativeVariantRow {
                #[cfg(test)]
                writer_token_used: evidence_rows.iter().any(|row| row.writer_token_used),
                #[cfg(test)]
                write_produced: evidence_rows.iter().any(|row| row.write_produced),
                rejected,
            });
        }
    }

    audit.positive_passed = audit.positive_variants.iter().filter(|row| row.passed).count();
    audit.negative_rejected = audit.negative_variants.iter().filter(|row| row.rejected).count();
    audit.candidate_count_stable = !audit.positive_variants.is_empty()
        && audit
            .positive_variants
            .iter()
            .all(|row| row.candidate_count == audit.baseline_candidate_count);
    audit.repair_intent_stable = !audit.positive_variants.is_empty()
        && audit
            .positive_variants
            .iter()
            .all(|row| row.repair_intent_stable);
    audit.serialization_plan_type_stable = !audit.positive_variants.is_empty()
        && audit
            .positive_variants
            .iter()
            .all(|row| row.serialization_plan_type_stable);
    audit.semantic_event_invariants_stable = !audit.positive_variants.is_empty()
        && audit.positive_variants.iter().all(|row| {
            row.semantic_event_delta_rows == 0
                && row.serialization_encoding_delta_rows
                    == row.candidate_count.saturating_mul(2)
                && row.sounding_identity_delta_rows == 0
        });
    audit.encoding_normalization_stable = !audit.positive_variants.is_empty()
        && audit.positive_variants.iter().all(|row| {
            row.required_chord_normalizations == row.candidate_count
                && row.proven_chord_normalizations == row.candidate_count
                && row.artificial_chord_additions == 0
        });
    audit.negatives_rejected = !audit.negative_variants.is_empty()
        && audit
            .negative_variants
            .iter()
            .all(|row| row.rejected);
    audit.detector_identity_fields_exposed = audit.detector_identity_fields_exposed
        || audit
            .positive_variants
            .iter()
            .any(|row| row.detector_identity_fields_exposed);

    let genericity_evidence = audit.authorization_evidence();
    audit.genericity_evidence_complete = genericity_evidence.authorizes_write();

    #[cfg(test)]
    {
        let all_green = audit.source_parse_ok
            && audit.baseline_candidate_count > 0
            && audit.baseline_serialization_plan_feasible
            && audit.positive_passed == audit.positive_variants.len()
            && audit.positive_variants.len() == 5
            && audit.negative_rejected == audit.negative_variants.len()
            && audit.negative_variants.len()
                == baseline_candidate_views.len().saturating_mul(6)
            && audit.independent_candidate_duplication_invariant
            && audit.genericity_evidence_complete
            && !audit.detector_identity_fields_exposed
            && !audit.authorization_attempted
            && !audit.writer_token_used
            && !audit.write_produced;
        audit.verdict = if all_green {
            "identity_free_metamorphic_positive_negative_and_duplication_safety_net_green_audit_only"
                .to_string()
        } else {
            "identity_free_metamorphic_safety_net_incomplete".to_string()
        };
    }

    audit
}

fn repair_intent_name(intents: &[super::LocatedStructuralRepairIntent]) -> &'static str {
    if intents.is_empty() {
        "none"
    } else {
        "complete_tail_beat_beam"
    }
}

fn positive_variants(xml: &str) -> Vec<(&'static str, String, bool)> {
    let (measure_renumbered, measure_changes) = renumber_measure_attributes(xml);
    let (pitch_bijection, pitch_changes) = map_exact_tag_text(xml, "step", |value| {
        match value {
            "C" => "D".to_string(),
            "D" => "E".to_string(),
            "E" => "F".to_string(),
            "F" => "G".to_string(),
            "G" => "A".to_string(),
            "A" => "B".to_string(),
            "B" => "C".to_string(),
            other => other.to_string(),
        }
    });
    let (voice_bijection, voice_changes) = map_exact_tag_text(xml, "voice", |value| {
        value
            .parse::<i64>()
            .map_or_else(|_| format!("voice_{value}"), |number| number.saturating_add(1000).to_string())
    });
    let (staff_bijection, staff_changes) = map_exact_tag_text(xml, "staff", |value| {
        value
            .parse::<i64>()
            .map_or_else(|_| format!("staff_{value}"), |number| number.saturating_add(2000).to_string())
    });
    let (beam_order_varied, beam_order_changes) = reverse_beam_element_order_in_notes(xml);

    vec![
        ("measure_number_bijection", measure_renumbered, measure_changes > 0),
        ("pitch_step_bijection", pitch_bijection, pitch_changes > 0),
        ("voice_id_bijection", voice_bijection, voice_changes > 0),
        ("staff_id_bijection", staff_bijection, staff_changes > 0),
        (
            "irrelevant_beam_xml_order_permutation",
            beam_order_varied,
            beam_order_changes > 0,
        ),
    ]
}

fn negative_variants(
    candidate: &LocatedStructuralView,
) -> Vec<(&'static str, Vec<LocatedStructuralView>)> {
    let quarter = candidate.view.metric_units_per_quarter();
    let sixteenth = (quarter / 4).max(1);
    let mut variants = Vec::new();

    let mut missing_carrier = candidate.clone();
    remove_first_role(&mut missing_carrier, StructuralCarrierRole::HiddenCarrier);
    variants.push(("missing_hidden_carrier", vec![missing_carrier]));

    let mut carrier_onset_drift = candidate.clone();
    if let Some(event) = carrier_onset_drift
        .view
        .events
        .iter_mut()
        .find(|event| event.carrier_role == StructuralCarrierRole::HiddenCarrier)
    {
        event.onset_in_window = event.onset_in_window.saturating_sub(sixteenth);
    }
    variants.push(("hidden_carrier_onset_drift", vec![carrier_onset_drift]));

    let mut carrier_duration_drift = candidate.clone();
    if let Some(event) = carrier_duration_drift
        .view
        .events
        .iter_mut()
        .find(|event| event.carrier_role == StructuralCarrierRole::HiddenCarrier)
    {
        event.duration = event.duration.saturating_add(sixteenth);
    }
    variants.push(("hidden_carrier_duration_drift", vec![carrier_duration_drift]));

    variants.push((
        "same_domain_competing_candidate",
        vec![candidate.clone(), candidate.clone()],
    ));

    let mut beat_topology_changed = candidate.clone();
    if let Some(event) = beat_topology_changed
        .view
        .events
        .iter_mut()
        .find(|event| {
            event.carrier_role == StructuralCarrierRole::Sounding
                && matches!(
                    event.staff_relation,
                    super::StructuralStaffRelation::DifferentFromAnchor
                )
        })
    {
        event.onset_in_window = event.onset_in_window.saturating_add(sixteenth);
    }
    variants.push(("musically_different_beat_topology", vec![beat_topology_changed]));

    let mut artificial_chord_pressure = candidate.clone();
    if let Some(event) = artificial_chord_pressure
        .view
        .events
        .iter_mut()
        .find(|event| event.carrier_role == StructuralCarrierRole::HiddenCarrier)
    {
        event.carrier_role = StructuralCarrierRole::Sounding;
        event.visible = true;
        event.rest_like = false;
        event.chord_member = false;
    }
    variants.push((
        "artificial_chord_addition_pressure",
        vec![artificial_chord_pressure],
    ));

    variants
}

fn remove_first_role(view: &mut LocatedStructuralView, role: StructuralCarrierRole) -> bool {
    let Some(index) = view
        .view
        .events
        .iter()
        .position(|event| event.carrier_role == role)
    else {
        return false;
    };
    view.view.events.remove(index);
    if index < view.locator.source_events.len() {
        view.locator.source_events.remove(index);
    }
    true
}

fn renumber_measure_attributes(xml: &str) -> (String, usize) {
    let mut out = String::with_capacity(xml.len());
    let mut cursor = 0_usize;
    let mut changes = 0_usize;
    while let Some(relative_start) = xml[cursor..].find("<measure") {
        let start = cursor + relative_start;
        out.push_str(&xml[cursor..start]);
        let Some(relative_end) = xml[start..].find('>') else {
            out.push_str(&xml[start..]);
            return (out, changes);
        };
        let end = start + relative_end + 1;
        let tag = &xml[start..end];
        if let Some(number_start) = tag.find("number=\"") {
            let value_start = number_start + "number=\"".len();
            if let Some(value_end_relative) = tag[value_start..].find('"') {
                let value_end = value_start + value_end_relative;
                out.push_str(&tag[..value_start]);
                out.push_str("metamorph_");
                out.push_str(&tag[value_start..value_end]);
                out.push_str(&tag[value_end..]);
                changes += 1;
            } else {
                out.push_str(tag);
            }
        } else {
            out.push_str(tag);
        }
        cursor = end;
    }
    out.push_str(&xml[cursor..]);
    (out, changes)
}

fn map_exact_tag_text(
    xml: &str,
    tag: &str,
    mut mapper: impl FnMut(&str) -> String,
) -> (String, usize) {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut out = String::with_capacity(xml.len());
    let mut cursor = 0_usize;
    let mut changes = 0_usize;
    while let Some(relative_start) = xml[cursor..].find(&open) {
        let start = cursor + relative_start;
        let text_start = start + open.len();
        let Some(relative_end) = xml[text_start..].find(&close) else {
            break;
        };
        let text_end = text_start + relative_end;
        let original = &xml[text_start..text_end];
        let trimmed = original.trim();
        let mapped = mapper(trimmed);
        out.push_str(&xml[cursor..text_start]);
        if let Some(trimmed_start) = original.find(trimmed) {
            out.push_str(&original[..trimmed_start]);
            out.push_str(&mapped);
            out.push_str(&original[trimmed_start + trimmed.len()..]);
        } else {
            out.push_str(&mapped);
        }
        if mapped != trimmed {
            changes += 1;
        }
        out.push_str(&close);
        cursor = text_end + close.len();
    }
    out.push_str(&xml[cursor..]);
    (out, changes)
}

fn reverse_beam_element_order_in_notes(xml: &str) -> (String, usize) {
    let mut out = String::with_capacity(xml.len());
    let mut cursor = 0_usize;
    let mut changes = 0_usize;
    while let Some(start) = find_next_note_start(xml, cursor) {
        out.push_str(&xml[cursor..start]);
        let Some(relative_end) = xml[start..].find("</note>") else {
            out.push_str(&xml[start..]);
            return (out, changes);
        };
        let end = start + relative_end + "</note>".len();
        let note = &xml[start..end];
        let (reordered, note_changed) = reverse_beams_in_note(note);
        out.push_str(&reordered);
        if note_changed {
            changes += 1;
        }
        cursor = end;
    }
    out.push_str(&xml[cursor..]);
    (out, changes)
}


fn find_next_note_start(xml: &str, cursor: usize) -> Option<usize> {
    let plain = xml[cursor..].find("<note>").map(|offset| cursor + offset);
    let attributed = xml[cursor..].find("<note ").map(|offset| cursor + offset);
    match (plain, attributed) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

fn reverse_beams_in_note(note: &str) -> (String, bool) {
    let mut ranges = Vec::new();
    let mut cursor = 0_usize;
    while let Some(relative_start) = note[cursor..].find("<beam") {
        let start = cursor + relative_start;
        let Some(relative_end) = note[start..].find("</beam>") else {
            break;
        };
        let end = start + relative_end + "</beam>".len();
        ranges.push((start, end));
        cursor = end;
    }
    if ranges.len() < 2 {
        return (note.to_string(), false);
    }
    let beam_blocks = ranges
        .iter()
        .map(|(start, end)| note[*start..*end].to_string())
        .collect::<Vec<_>>();
    let mut out = String::with_capacity(note.len());
    let mut source_cursor = 0_usize;
    for (index, (start, end)) in ranges.iter().enumerate() {
        out.push_str(&note[source_cursor..*start]);
        out.push_str(&beam_blocks[beam_blocks.len() - 1 - index]);
        source_cursor = *end;
    }
    out.push_str(&note[source_cursor..]);
    (out, true)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SYNTHETIC_SNAPSHOT: &str = include_str!(
        "../../tests/fixtures/synthetic/cross_staff_tail_structural_invariant.xml"
    );

    fn synthetic_audit() -> IdentityFreeMetamorphicSafetyNetAudit {
        audit_identity_free_metamorphic_safety_net(
            SYNTHETIC_SNAPSHOT,
            96,
            "synthetic-snapshot",
        )
    }

    #[test]
    fn five_positive_identity_metamorphoses_preserve_candidate_intent_and_plan() {
        let audit = synthetic_audit();
        assert_eq!(audit.positive_variants.len(), 5);
        assert_eq!(audit.positive_passed, 5);
        assert!(audit.candidate_count_stable);
        assert!(audit.repair_intent_stable);
        assert!(audit.serialization_plan_type_stable);
        assert!(audit.semantic_event_invariants_stable);
        assert!(audit.encoding_normalization_stable);
        assert!(audit
            .positive_variants
            .iter()
            .all(|row| row.transformation_applied && row.passed));
    }

    #[test]
    fn six_negative_structural_variants_are_rejected_without_token_or_write() {
        let audit = synthetic_audit();
        assert_eq!(audit.negative_variants.len(), 6);
        assert_eq!(audit.negative_rejected, 6);
        assert!(audit.negatives_rejected);
        assert!(audit.negative_variants.iter().all(|row| {
            row.rejected && !row.writer_token_used && !row.write_produced
        }));
    }

    #[test]
    fn complete_genericity_evidence_is_audited_but_authorization_is_not_called() {
        let audit = synthetic_audit();
        assert!(audit.genericity_evidence_complete);
        assert!(audit.independent_candidate_duplication_invariant);
        assert!(!audit.authorization_attempted);
        assert!(!audit.writer_token_used);
        assert!(!audit.write_produced);
        assert_eq!(
            audit.verdict,
            "identity_free_metamorphic_positive_negative_and_duplication_safety_net_green_audit_only"
        );
    }

    #[test]
    fn irrelevant_beam_order_permutation_changes_xml_but_not_detection() {
        let (transformed, changes) =
            reverse_beam_element_order_in_notes(SYNTHETIC_SNAPSHOT);
        assert!(changes > 0);
        assert_ne!(transformed, SYNTHETIC_SNAPSHOT);
        let (views, _) = adapt_structural_snapshot_read_only(&transformed, 96, "beam-order");
        let intents = detect_structural_repair_intents_read_only(&views);
        assert_eq!(intents.len(), 1);
    }
}
