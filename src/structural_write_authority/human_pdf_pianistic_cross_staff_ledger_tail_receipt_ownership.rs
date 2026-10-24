// Canonical structural mutation ownership.
// Included in the pipeline lexical namespace to retain private selection facts.
// The complete preparation and mutation implementations live here; no forwarding facade.

fn apply_pianistic_missing_rest_insert_with_receipt(
    xml: &str,
    max_actions: usize,
    blocked_examples: &mut Vec<String>,
) -> (
    String,
    Vec<RenderXmlPianisticMissingRestInsertAction>,
    usize,
    usize,
    Option<crate::master_state_identity::ExactTransformationReceipt>,
) {
    let (candidates, missing_seen) = build_missing_rest_insert_candidates(xml, blocked_examples);
    let mut selected: Vec<RenderXmlMissingRestInsertCandidate> = Vec::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    for candidate in &candidates {
        if selected.len() >= max_actions {
            break;
        }
        let overlaps = occupied.iter().any(|(start, end)| candidate.forward_start < *end && candidate.forward_end > *start);
        if overlaps {
            continue;
        }
        occupied.push((candidate.forward_start, candidate.forward_end));
        selected.push(candidate.clone());
    }
    let metric_by_scope = render_scope_metric_expected_and_beat_map(xml);
    let mut replacements = Vec::<(usize, usize, String)>::new();
    let mut actions = Vec::new();
    for candidate in &selected {
        let Some((expected, beat_unit)) = metric_by_scope.get(&candidate.scope).copied() else {
            continue;
        };
        let Some(rest_note) = missing_rest_visible_sequence_template(
            &candidate.staff,
            &candidate.voice,
            candidate.onset,
            candidate.duration,
            expected,
            beat_unit,
        ) else { continue; };
        replacements.push((candidate.forward_start, candidate.forward_end, rest_note));
        actions.push(RenderXmlPianisticMissingRestInsertAction {
            scope: candidate.scope,
            staff: candidate.staff.clone(),
            voice: candidate.voice.clone(),
        });
    }
    let (out, missing_rest_receipt) = crate::structural_write_authority::apply_bound_missing_rest_batch_authorized(
        xml,
        replacements,
    );
    let out = out.expect("missing-rest capability must match the selected forward ranges");
    (out, actions, candidates.len(), missing_seen, missing_rest_receipt)
}
