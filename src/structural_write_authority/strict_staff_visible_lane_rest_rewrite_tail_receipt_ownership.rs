// Canonical structural mutation ownership.
// Included in the pipeline lexical namespace to retain private selection facts.
// The complete preparation and mutation implementations live here; no forwarding facade.

fn apply_structural_duplicate_same_pitch_exact_render_write_with_receipts(
    xml: &str,
    max_actions: usize,
) -> (String, Vec<BehindBarsRuleCheckRestAndTieAction>, usize, crate::master_state_identity::RenderTransitionReceipts) {
    let plan = plan_strict_staff_duplicate_same_pitch_suppression(
        xml,
        max_actions,
        StrictStaffSamePitchGrouping::ExactVoice,
        "external_feedback_duplicate_same_pitch_exact_render_write",
    );
    let (out, receipt) = crate::structural_write_authority::apply_duplicate_same_pitch_suppression_authorized(
        xml,
        plan.replacements,
    );
    let out = out.unwrap_or_else(|| xml.to_string());
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);
    (out, plan.actions, plan.candidates_seen, receipts)
}

fn apply_upper_staff_duplicate_visible_rest_suppression_with_receipts(
    xml: &str,
    max_actions: usize,
) -> (
    (String, usize, usize, i32, String, String),
    crate::master_state_identity::RenderTransitionReceipts,
) {
    let candidates = upper_staff_duplicate_visible_rest_suppression_candidates(xml);
    let candidates_seen = candidates.len();
    let mut replacements = Vec::<(usize, usize, String)>::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    let mut per_scope_selected = BTreeMap::<usize, usize>::new();
    let mut scopes = BTreeSet::<usize>::new();
    let mut total_duration = 0_i32;
    let mut selected = 0_usize;
    let mut interval_proofs = BTreeSet::<String>::new();
    for (_score, start, end, replacement, scope, _evidence, duration, interval_proof) in candidates {
        if selected >= max_actions {
            break;
        }
        if *per_scope_selected.get(&scope).unwrap_or(&0) >= 4 {
            continue;
        }
        if occupied.iter().any(|(used_start, used_end)| start < *used_end && end > *used_start) {
            continue;
        }
        occupied.push((start, end));
        replacements.push((start, end, replacement));
        *per_scope_selected.entry(scope).or_default() += 1;
        scopes.insert(scope);
        interval_proofs.insert(interval_proof);
        total_duration = total_duration.saturating_add(duration.max(0));
        selected += 1;
    }
    replacements.sort_by_key(|a| std::cmp::Reverse(a.0));
    let (out, receipt) = crate::structural_write_authority::apply_upper_duplicate_rest_suppression_authorized(
        xml,
        replacements,
    );
    let out = out.unwrap_or_else(|| xml.to_string());
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);
    let proof = if interval_proofs.is_empty() {
        "-".to_string()
    } else {
        interval_proofs.iter().cloned().collect::<Vec<_>>().join("; ")
    };
    ((
        out,
        candidates_seen,
        selected,
        total_duration,
        fmt_upper_staff_rest_scope_set(scopes),
        proof,
    ), receipts)
}

fn build_external_notation_feedback_safe_write_audit_prepared(
    xml: &str,
    source_label: &str,
    output_path: &Path,
) -> anyhow::Result<(String, TieMergeAccountingAndTrueTransactionalRestCoalesceAudit, Option<String>, crate::master_state_identity::RenderTransitionReceipts)> {
    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let mut audit = TieMergeAccountingAndTrueTransactionalRestCoalesceAudit {
        source_label: source_label.to_string(),
        visible_pitched_before: before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count(),
        hidden_pitched_before: before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count(),
        visible_rests_before: before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count(),
        sounding_signature_before: render_visible_sounding_signature(&before_events),
        dedup_sounding_signature_before: render_visible_sounding_signature_dedup(&before_events),
        ..TieMergeAccountingAndTrueTransactionalRestCoalesceAudit::default()
    };
    audit.attempted = true;
    let (candidate_xml_after_duplicate, mut actions, seen, mut receipts) = apply_structural_duplicate_same_pitch_exact_render_write_with_receipts(xml, 8);
    audit.duplicate_candidates_seen = seen;
    audit.duplicate_attempted = actions.len();
    let candidate_xml_after_tie = candidate_xml_after_duplicate.clone();
    let mut tie_actions = Vec::<BehindBarsRuleCheckRestAndTieAction>::new();
    let tie_seen = 0_usize;
    audit.tie_merge_candidates_seen = tie_seen;
    audit.tie_merge_attempted = tie_actions.len();
    audit.tie_merge_equivalent_contractions = tie_actions.len();
    audit.retention_neutralized_pitched_delta = tie_actions.len();
    let candidate_xml_after_shadow = candidate_xml_after_tie.clone();
    let mut shadow_actions = Vec::<BehindBarsRuleCheckRestAndTieAction>::new();
    let shadow_seen = 0_usize;
    audit.generic_shadow_rest_candidates = shadow_seen;
    audit.generic_shadow_rest_attempted = shadow_actions.len();

    // r735: the lower-staff staccato proxy pilot and its zero-only compatibility
    // accounting are fully retired. The producer has been absent since r472.
    let (candidate_xml, mut rest_actions, rest_seen, rest_receipts) = apply_structural_transactional_rest_coalesce_render_write_prepared(&candidate_xml_after_shadow, 4);
    receipts.append(rest_receipts);
    audit.transactional_rest_coalesce_candidates = rest_seen;
    audit.transactional_rest_coalesce_attempted = rest_actions.len();
    audit.transactional_rest_coalesce_adopted = rest_actions.iter().filter(|action| action.verdict == "adopted_transactionally").count();
    audit.transactional_rest_coalesce_rejected = rest_actions.len().saturating_sub(audit.transactional_rest_coalesce_adopted);

    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&candidate_xml);
    audit.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    audit.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    audit.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    audit.sounding_signature_after = render_visible_sounding_signature(&after_events);
    audit.dedup_sounding_signature_after = render_visible_sounding_signature_dedup(&after_events);
    audit.parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    audit.backup_forward_after_ok = after_backup_forward_ok;
    audit.raw_cursor_after_ok = after_backup_forward_ok;
    audit.duplicate_rows_after = build_behind_bars_rule_check_duplicate_stem_rows(&candidate_xml).into_iter().take(12).collect();
    let expected_visible_after = audit.visible_pitched_before
        .saturating_sub(audit.duplicate_attempted)
        .saturating_sub(audit.tie_merge_attempted);
    let expected_hidden_after = audit.hidden_pitched_before.saturating_add(audit.duplicate_attempted);
    let adopted_rest_delta: usize = rest_actions
        .iter()
        .filter(|action| action.verdict == "adopted_transactionally")
        .map(|action| action.old_rest_count.saturating_sub(action.new_rest_count))
        .sum();
    let shadow_rest_delta: usize = shadow_actions
        .iter()
        .map(|action| action.old_rest_count.saturating_sub(action.new_rest_count))
        .sum();
    let expected_visible_rests_after = audit.visible_rests_before
        .saturating_sub(adopted_rest_delta)
        .saturating_sub(shadow_rest_delta);
    let any_write_attempted = audit.duplicate_attempted > 0
        || audit.tie_merge_attempted > 0
        || audit.generic_shadow_rest_attempted > 0
        || audit.transactional_rest_coalesce_adopted > 0;
    let sounding_contract_ok = if audit.tie_merge_attempted > 0 {
        // Same-pitch tie contraction intentionally changes visible notation granularity
        // while preserving the musical target as a notational equivalence class.
        true
    } else {
        audit.dedup_sounding_signature_before == audit.dedup_sounding_signature_after
    };
    let invariant_ok = any_write_attempted
        && before_backup_forward_ok
        && audit.parse_after_ok
        && audit.backup_forward_after_ok
        && sounding_contract_ok
        && audit.visible_pitched_after == expected_visible_after
        && audit.hidden_pitched_after == expected_hidden_after
        && audit.visible_rests_after == expected_visible_rests_after;

    if invariant_ok {
        audit.written = true;
        audit.duplicate_adopted = audit.duplicate_attempted;
        audit.tie_merge_adopted = audit.tie_merge_attempted;
        audit.generic_shadow_rest_adopted = audit.generic_shadow_rest_attempted;
        for action in &mut actions { action.verdict = "adopted".to_string(); }
        for action in &mut tie_actions { action.verdict = "adopted".to_string(); }
        for action in &mut shadow_actions { action.verdict = "adopted".to_string(); }
        audit.verdict = "generic_shadow_rest_and_rest_coalesce_green".to_string();
    } else {
        audit.duplicate_rejected = audit.duplicate_attempted;
        audit.tie_merge_rejected = audit.tie_merge_attempted;
        audit.generic_shadow_rest_rejected = audit.generic_shadow_rest_attempted;
        for action in &mut actions { action.verdict = "rejected_by_external_feedback_duplicate_invariant_guard".to_string(); }
        for action in &mut tie_actions { action.verdict = "rejected_by_external_feedback_tie_merge_invariant_guard".to_string(); }
        for action in &mut shadow_actions { action.verdict = "rejected_by_generic_shadow_rest_invariant_guard".to_string(); }
        for action in &mut rest_actions { action.verdict = "rejected_by_global_external_feedback_safewrite_guard".to_string(); }
        audit.transactional_rest_coalesce_rejected = audit.transactional_rest_coalesce_attempted;
        audit.transactional_rest_coalesce_adopted = 0;
        audit.verdict = if !any_write_attempted {
            "external_feedback_tie_merge_accounting_no_write_candidate".to_string()
        } else {
            "external_feedback_tie_merge_or_transactional_rest_rejected_by_invariant_guard".to_string()
        };
    }
    audit.notation_rule_change = audit.written;
    audit.music_mutation = false;
    actions.extend(tie_actions);
    actions.extend(shadow_actions);
    actions.extend(rest_actions);
    audit.actions = actions;
    audit.done_fragment = format!(
        "GenericShadowRestAndTransactionalRest: DuplicateSamePitch exact seen/attempted/adopted {}/{}/{}; SamePitchTieMerge seen/attempted/adopted {}/{}/{}; TieMergeEquivalentContractions {}; UpperShadowRest generic seen/attempted/adopted {}/{}/{}; RestCoalesce transactional candidates/attempted/adopted/rejected {}/{}/{}/{}; sichtbare klingende Noten {}→{} retention-neutralisiert {}; hidden klingend {}→{}; MusicMutation nein",
        audit.duplicate_candidates_seen,
        audit.duplicate_attempted,
        audit.duplicate_adopted,
        audit.tie_merge_candidates_seen,
        audit.tie_merge_attempted,
        audit.tie_merge_adopted,
        audit.tie_merge_equivalent_contractions,
        audit.generic_shadow_rest_candidates,
        audit.generic_shadow_rest_attempted,
        audit.generic_shadow_rest_adopted,
        audit.transactional_rest_coalesce_candidates,
        audit.transactional_rest_coalesce_attempted,
        audit.transactional_rest_coalesce_adopted,
        audit.transactional_rest_coalesce_rejected,
        audit.visible_pitched_before,
        audit.visible_pitched_after,
        audit.retention_neutralized_pitched_delta,
        audit.hidden_pitched_before,
        audit.hidden_pitched_after
    );
    audit.pipeline_line = format!(
        "GenericShadowRestAndTransactionalRest v{}: Render-XML-only auf {}; DuplicateSamePitch exact seen/attempted/adopted/rejected {}/{}/{}/{}; SamePitchTieMerge seen/attempted/adopted/rejected {}/{}/{}/{}; TieMergeEquivalentContractions {}; UpperShadowRest generic seen/attempted/adopted/rejected {}/{}/{}/{}; RestCoalesce transactional candidates/attempted/adopted/rejected {}/{}/{}/{}; sichtbare klingende Noten {}→{}; hidden klingend {}→{}; sichtbare Pausen {}→{}; sounding_signature {}/{}; dedup_sounding_signature {}/{} stabil {}; parse/back-forward/raw {}/{}/{}; geschrieben {}; MusicMutation nein; MasterApply nein; P3/P4 nein; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        audit.source_label,
        audit.duplicate_candidates_seen,
        audit.duplicate_attempted,
        audit.duplicate_adopted,
        audit.duplicate_rejected,
        audit.tie_merge_candidates_seen,
        audit.tie_merge_attempted,
        audit.tie_merge_adopted,
        audit.tie_merge_rejected,
        audit.tie_merge_equivalent_contractions,
        audit.generic_shadow_rest_candidates,
        audit.generic_shadow_rest_attempted,
        audit.generic_shadow_rest_adopted,
        audit.generic_shadow_rest_rejected,
        audit.transactional_rest_coalesce_candidates,
        audit.transactional_rest_coalesce_attempted,
        audit.transactional_rest_coalesce_adopted,
        audit.transactional_rest_coalesce_rejected,
        audit.visible_pitched_before,
        audit.visible_pitched_after,
        audit.hidden_pitched_before,
        audit.hidden_pitched_after,
        audit.visible_rests_before,
        audit.visible_rests_after,
        audit.sounding_signature_before,
        audit.sounding_signature_after,
        audit.dedup_sounding_signature_before,
        audit.dedup_sounding_signature_after,
        yes_no(audit.tie_merge_attempted > 0 || audit.dedup_sounding_signature_before == audit.dedup_sounding_signature_after),
        if audit.parse_after_ok { "ok" } else { "fail" },
        if audit.backup_forward_after_ok { "ok" } else { "fail" },
        if audit.raw_cursor_after_ok { "ok" } else { "fail" },
        yes_no(audit.written),
        audit.verdict
    );
    let candidate = audit.written.then(|| candidate_xml.clone());
    let _ = output_path;
    Ok((if audit.written { candidate_xml } else { xml.to_string() }, audit, candidate, receipts))
}

fn preview_upper_rest_duration(fragment: &str, duration: i32) -> Option<String> {
    let (post, receipt) = set_rest_note_duration_type_with_receipt(fragment, duration);
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    if let Some(post) = post.as_deref() {
        receipts.push_generated_block(receipt, fragment, post, None);
    }
    receipts.discard_unadopted(
        crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe,
    );
    post
}
