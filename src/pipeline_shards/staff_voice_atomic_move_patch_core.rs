fn atomic_note_is_tie_bound(note: &AtomicNote) -> bool {
    note.tie_start || note.tie_stop
}

fn move_cluster_choice(notes: &[&AtomicNote], name: &str, target_staff: &str) -> Vec<MoveChoice> {
    let mut targets = Vec::new();
    for note in notes.iter().copied() {
        if !note.visible || note.pitch.is_none() || note.staff == target_staff {
            continue;
        }
        if let Some(target) = move_target_for_note(note, target_staff) {
            targets.push(target);
        }
    }
    targets.sort();
    targets.dedup();
    if targets.is_empty() {
        return Vec::new();
    }
    let pitch_labels = notes
        .iter()
        .filter_map(|note| note.pitch.as_ref().map(|p| sanitize_patch_label(&p.label)))
        .take(4)
        .collect::<Vec<_>>()
        .join("_");
    vec![MoveChoice {
        name: if pitch_labels.is_empty() { name.to_string() } else { format!("{name}_{pitch_labels}") },
        targets,
    }]
}

fn move_target_for_note(note: &AtomicNote, target_staff: &str) -> Option<MoveTarget> {
    if note.staff == target_staff {
        return None;
    }
    Some(MoveTarget {
        note_start: note.note_start,
        note_end: note.note_end,
        staff_value_range: note.staff_value_range,
        voice_value_range: note.voice_value_range,
        target_staff: target_staff.to_string(),
        target_voice: note.voice.clone(),
    })
}

fn dedupe_move_choices(choices: Vec<MoveChoice>, limit: usize) -> Vec<MoveChoice> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for mut choice in choices {
        choice.targets.sort();
        choice.targets.dedup();
        if choice.targets.is_empty() {
            continue;
        }
        let key = choice.targets.clone();
        if !seen.insert(key) {
            continue;
        }
        out.push(choice);
        if out.len() >= limit {
            break;
        }
    }
    out
}

fn push_compound_move_variants(xml: &str, variants: &mut Vec<PatchVariant>, groups: &[Vec<MoveChoice>], limit: usize) {
    if groups.is_empty() { return; }
    let mut combos = vec![MoveChoice { name: "trajectory".to_string(), targets: Vec::new() }];
    for group in groups.iter().take(5) {
        let mut next = Vec::new();
        for combo in &combos {
            for choice in group.iter().take(4) {
                let mut merged = combo.clone();
                if merged.name == "trajectory" {
                    merged.name = choice.name.clone();
                } else {
                    merged.name.push('_');
                    merged.name.push_str(&choice.name);
                }
                merged.targets.extend(choice.targets.iter().cloned());
                merged.targets.sort();
                merged.targets.dedup();
                next.push(merged);
                if next.len() >= 192 { break; }
            }
            if next.len() >= 192 { break; }
        }
        combos = dedupe_move_choices(next, 192);
    }

    for (idx, combo) in combos.into_iter().enumerate() {
        if variants.len() >= limit { break; }
        if combo.targets.is_empty() { continue; }
        let patched = apply_multi_staff_voice_moves(xml, &combo.targets);
        let short_name = combo.name.chars().take(96).collect::<String>();
        push_final_variant(variants, format!("global_hand_trajectory_{:03}_{}", idx + 1, short_name), patched, limit);
    }
}

fn push_mixed_move_restify_variants(
    xml: &str,
    variants: &mut Vec<PatchVariant>,
    move_groups: &[Vec<MoveChoice>],
    restify_groups: &[Vec<RestifyChoice>],
    limit: usize,
) {
    if move_groups.is_empty() || restify_groups.is_empty() { return; }
    for (m_idx, move_group) in move_groups.iter().take(4).enumerate() {
        for move_choice in move_group.iter().take(3) {
            for (r_idx, rest_group) in restify_groups.iter().take(4).enumerate() {
                for rest_choice in rest_group.iter().take(3) {
                    if variants.len() >= limit { return; }
                    // Every variant executes its own #40 move, so each variant
                    // carries the receipt of its own owner event (no sharing).
                    let moved = apply_multi_staff_voice_moves(xml, &move_choice.targets);
                    let restified = apply_multi_restify_patch(&moved.xml, &rest_choice.ranges);
                    let patched = moved.followed_by(restified);
                    let name = format!(
                        "global_hand_mixed_m{}_r{}_{}_{}",
                        m_idx + 1,
                        r_idx + 1,
                        move_choice.name.chars().take(48).collect::<String>(),
                        rest_choice.name.chars().take(48).collect::<String>()
                    );
                    push_final_variant(variants, name, patched, limit);
                }
            }
        }
    }
}

/// Patched document together with the owner receipts minted while it was
/// produced, in application order. Provenance only; it never decides adoption.
/// Move-only: its receipts belong to exactly this variant's owner events.
#[derive(Debug)]
struct PatchedXml {
    xml: String,
    lineage_receipts: master_state_identity::OwnerReceipts,
}

impl PatchedXml {
    fn with_exact(xml: String, receipt: Option<ExactTransformationReceipt>) -> Self {
        let mut lineage_receipts = master_state_identity::OwnerReceipts::default();
        lineage_receipts.push_exact(receipt);
        Self {
            xml,
            lineage_receipts,
        }
    }

    /// Applies a further owner transformation to this patched document; the
    /// receipts of both steps stay in application order.
    fn followed_by(self, next: PatchedXml) -> Self {
        let mut lineage_receipts = self.lineage_receipts;
        lineage_receipts.append(next.lineage_receipts);
        Self {
            xml: next.xml,
            lineage_receipts,
        }
    }
}

/// A named patch variant: the patched document and its owner receipts.
#[derive(Debug)]
struct PatchVariant {
    name: String,
    patched: PatchedXml,
}

fn apply_multi_staff_voice_moves(xml: &str, targets: &[MoveTarget]) -> PatchedXml {
    let mut replacements: Vec<(usize, usize, String)> = Vec::new();
    for target in targets {
        if let Some((start, end)) = target.voice_value_range {
            replacements.push((start, end, target.target_voice.clone()));
        }
        if let Some((start, end)) = target.staff_value_range {
            replacements.push((start, end, target.target_staff.clone()));
        }
    }
    replacements.sort_by_key(|a| std::cmp::Reverse(a.0));
    let (moved, receipt) =
        crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(
            xml,
            replacements,
        );
    PatchedXml::with_exact(moved.unwrap_or_else(|| xml.to_string()), receipt)
}

fn restify_choices_for_issue(notes: &[AtomicNote], issue: &Issue) -> Vec<RestifyChoice> {
    let mut choices = Vec::new();
    match issue.issue_type.as_str() {
        "PIANO_FAST_LARGE_JUMP" => {
            for (from_note, to_note) in find_jump_issue_note_pairs(notes, issue).into_iter().take(3) {
                if let Some(choice) = restify_choice_from_notes("hide_from", &[from_note]) {
                    choices.push(choice);
                }
                if let Some(choice) = restify_choice_from_notes("hide_to", &[to_note]) {
                    choices.push(choice);
                }
                if let Some(choice) = restify_choice_from_notes("hide_pair", &[from_note, to_note]) {
                    choices.push(choice);
                }
            }
        }
        "PIANO_HAND_SPAN_LARGE" => {
            for (low_note, high_note) in find_span_issue_note_pairs(notes, issue).into_iter().take(3) {
                if let Some(choice) = restify_choice_from_notes("hide_low", &[low_note]) {
                    choices.push(choice);
                }
                if let Some(choice) = restify_choice_from_notes("hide_high", &[high_note]) {
                    choices.push(choice);
                }
                if let Some(choice) = restify_choice_from_notes("hide_span_pair", &[low_note, high_note]) {
                    choices.push(choice);
                }
            }
        }
        "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF" => {
            for patch in overload_atomic_patches(notes, issue).into_iter().take(4) {
                choices.push(RestifyChoice {
                    name: patch.name,
                    ranges: vec![(patch.note_start, patch.note_end)],
                });
            }
        }
        _ => {}
    }
    dedupe_restify_choices(choices, 8)
}

fn restify_choice_from_notes(prefix: &str, notes: &[&AtomicNote]) -> Option<RestifyChoice> {
    if notes.iter().copied().any(atomic_note_is_tie_bound) {
        return None;
    }
    let mut ranges: Vec<(usize, usize)> = notes
        .iter()
        .filter(|note| !note.is_chord && note.pitch_block_range.is_some())
        .map(|note| (note.note_start, note.note_end))
        .collect();
    ranges.sort_unstable();
    ranges.dedup();
    if ranges.is_empty() {
        return None;
    }
    let labels = notes
        .iter()
        .filter_map(|note| note.pitch.as_ref().map(|pitch| sanitize_patch_label(&pitch.label)))
        .collect::<Vec<_>>()
        .join("_");
    Some(RestifyChoice {
        name: if labels.is_empty() { prefix.to_string() } else { format!("{prefix}_{labels}") },
        ranges,
    })
}

fn dedupe_restify_choices(choices: Vec<RestifyChoice>, limit: usize) -> Vec<RestifyChoice> {
    let mut out: Vec<RestifyChoice> = Vec::new();
    for mut choice in choices {
        choice.ranges.sort_unstable();
        choice.ranges.dedup();
        if choice.ranges.is_empty() {
            continue;
        }
        if out.iter().any(|existing| existing.ranges == choice.ranges) {
            continue;
        }
        out.push(choice);
        if out.len() >= limit {
            break;
        }
    }
    out
}

fn push_compound_restify_variants(
    xml: &str,
    variants: &mut Vec<PatchVariant>,
    choice_groups: &[Vec<RestifyChoice>],
    limit: usize,
) {
    if choice_groups.is_empty() {
        return;
    }

    let mut combos: Vec<RestifyChoice> = vec![RestifyChoice { name: "combo".to_string(), ranges: Vec::new() }];
    for group in choice_groups.iter().take(5) {
        let mut next = Vec::new();
        for combo in &combos {
            for choice in group.iter().take(4) {
                let mut merged = combo.clone();
                if merged.name == "combo" {
                    merged.name = choice.name.clone();
                } else {
                    merged.name.push('_');
                    merged.name.push_str(&choice.name);
                }
                merged.ranges.extend(choice.ranges.iter().copied());
                merged.ranges.sort_unstable();
                merged.ranges.dedup();
                next.push(merged);
                if next.len() >= 192 {
                    break;
                }
            }
            if next.len() >= 192 {
                break;
            }
        }
        combos = dedupe_restify_choices(next, 192);
    }

    for (idx, combo) in combos.into_iter().enumerate() {
        if variants.len() >= limit {
            break;
        }
        if combo.ranges.is_empty() {
            continue;
        }
        let patched = apply_multi_restify_patch(xml, &combo.ranges);
        let short_name = combo.name.chars().take(96).collect::<String>();
        push_final_variant(
            variants,
            format!("final_playability_polish_{:03}_{}", idx + 1, short_name),
            patched,
            limit,
        );
    }
}

fn find_span_issue_note_pairs<'a>(notes: &'a [AtomicNote], issue: &Issue) -> Vec<(&'a AtomicNote, &'a AtomicNote)> {
    let Some(staff) = issue.staff.as_deref() else { return Vec::new(); };
    let actual = issue.actual_divisions.unwrap_or(0) as i32;
    let mut by_start: BTreeMap<i64, Vec<&AtomicNote>> = BTreeMap::new();
    for note in notes.iter().filter(|note| note.measure == issue.measure.as_str() && note.staff == staff && note.visible && note.pitch.is_some()) {
        by_start.entry(note.start).or_default().push(note);
    }

    let mut pairs = Vec::new();
    for (_start, group) in by_start {
        if group.len() < 2 {
            continue;
        }
        let Some(low) = group.iter().min_by_key(|note| note.pitch.as_ref().map_or(0, |p| p.midi)).copied() else { continue; };
        let Some(high) = group.iter().max_by_key(|note| note.pitch.as_ref().map_or(0, |p| p.midi)).copied() else { continue; };
        let span = high.pitch.as_ref().unwrap().midi - low.pitch.as_ref().unwrap().midi;
        if span == actual {
            pairs.push((low, high));
        }
    }
    pairs
}

fn final_critical_takt_variants(xml: &str, report: &CheckReport, limit: usize) -> Vec<PatchVariant> {
    let notes = parse_atomic_notes(xml);
    let mut variants: Vec<PatchVariant> = Vec::new();

    for (issue_index, issue) in report.issues.iter().filter(|issue| is_critical_piano_issue(issue)).enumerate() {
        let patch_prefix = format!("final_critical_{:03}", issue_index + 1);
        match issue.issue_type.as_str() {
            "PIANO_FAST_LARGE_JUMP" => {
                // Erst die bekannten sicheren Einzelvarianten erneut prüfen,
                // aber nach dem StrictWarningCleaner auf der finalen lokalen
                // Struktur. Danach folgen echte Taktgruppen-Varianten.
                for patch in jump_atomic_patches(&notes, issue) {
                    let (patched, patch_receipt) = apply_atomic_patch(xml, &patch);
                    let patched = PatchedXml::with_exact(patched, patch_receipt);
                    push_final_variant(&mut variants, patch.name, patched, limit);
                }

                if let Some((from_note, to_note)) = find_jump_pair_notes(&notes, issue) {
                    let from_name = from_note.pitch.as_ref().map_or_else(|| "from".to_string(), |p| sanitize_patch_label(&p.label));
                    let to_name = to_note.pitch.as_ref().map_or_else(|| "to".to_string(), |p| sanitize_patch_label(&p.label));

                    push_restify_variant(
                        xml,
                        &mut variants,
                        &format!("{patch_prefix}_hide_to_{to_name}"),
                        &[to_note],
                        limit,
                    );
                    push_restify_variant(
                        xml,
                        &mut variants,
                        &format!("{patch_prefix}_hide_from_{from_name}"),
                        &[from_note],
                        limit,
                    );
                    push_restify_variant(
                        xml,
                        &mut variants,
                        &format!("{patch_prefix}_hide_pair_{from_name}_{to_name}"),
                        &[from_note, to_note],
                        limit,
                    );

                    let high_staff_notes = collect_measure_staff_notes(&notes, &issue.measure, issue.staff.as_deref(), Some(60), None, 8);
                    if !high_staff_notes.is_empty() {
                        push_restify_variant(
                            xml,
                            &mut variants,
                            &format!("{patch_prefix}_hide_staff_high"),
                            &high_staff_notes,
                            limit,
                        );
                        let mut to_plus_high = high_staff_notes.clone();
                        to_plus_high.push(to_note);
                        push_restify_variant(
                            xml,
                            &mut variants,
                            &format!("{patch_prefix}_hide_to_plus_high"),
                            &to_plus_high,
                            limit,
                        );
                    }
                }
            }
            "PIANO_HAND_SPAN_LARGE" => {
                for patch in span_atomic_patches(&notes, issue) {
                    let (patched, patch_receipt) = apply_atomic_patch(xml, &patch);
                    let patched = PatchedXml::with_exact(patched, patch_receipt);
                    push_final_variant(&mut variants, patch.name, patched, limit);
                }
            }
            _ => {}
        }

        if variants.len() >= limit {
            break;
        }
    }

    variants.truncate(limit);
    variants
}

fn push_final_variant(variants: &mut Vec<PatchVariant>, name: String, patched: PatchedXml, limit: usize) {
    if variants.len() >= limit {
        return;
    }
    if variants
        .iter()
        .any(|existing| existing.name == name || existing.patched.xml == patched.xml)
    {
        return;
    }
    variants.push(PatchVariant { name, patched });
}

fn push_restify_variant(
    xml: &str,
    variants: &mut Vec<PatchVariant>,
    name: &str,
    notes: &[&AtomicNote],
    limit: usize,
) {
    if notes.iter().copied().any(atomic_note_is_tie_bound) {
        return;
    }
    let mut ranges: Vec<(usize, usize)> = notes
        .iter()
        .filter(|note| !note.is_chord && note.pitch_block_range.is_some())
        .map(|note| (note.note_start, note.note_end))
        .collect();
    ranges.sort_unstable();
    ranges.dedup();
    if ranges.is_empty() {
        return;
    }
    let patched = apply_multi_restify_patch(xml, &ranges);
    push_final_variant(variants, name.to_string(), patched, limit);
}
fn apply_multi_restify_patch(xml: &str, ranges: &[(usize, usize)]) -> PatchedXml {
    let mut sorted = ranges.to_vec();
    sorted.sort_by_key(|a| std::cmp::Reverse(a.0));
    let mut replacements = Vec::new();
    for (start, end) in sorted {
        if start >= end || end > xml.len() {
            continue;
        }
        let block = xml[start..end].to_string();
        let restified = restify_note_block(&block);
        replacements.push((start, end, restified));
    }
    let (restified_xml, receipt) =
        crate::structural_write_authority::apply_atomic_restification_batch_authorized(
            xml,
            replacements,
        );
    PatchedXml::with_exact(restified_xml.unwrap_or_else(|| xml.to_string()), receipt)
}

fn find_jump_pair_notes<'a>(
    notes: &'a [AtomicNote],
    issue: &Issue,
) -> Option<(&'a AtomicNote, &'a AtomicNote)> {
    find_jump_issue_note_pairs(notes, issue).into_iter().next()
}

fn collect_measure_staff_notes<'a>(
    notes: &'a [AtomicNote],
    measure: &str,
    staff: Option<&str>,
    min_midi: Option<i32>,
    max_midi: Option<i32>,
    limit: usize,
) -> Vec<&'a AtomicNote> {
    let mut picked = notes
        .iter()
        .filter(|note| {
            if note.measure != measure || note.is_chord {
                return false;
            }
            if let Some(staff) = staff
                && note.staff.as_str() != staff
            {
                return false;
            }
            let Some(pitch) = note.pitch.as_ref() else { return false; };
            if let Some(min_midi) = min_midi
                && pitch.midi < min_midi
            {
                return false;
            }
            if let Some(max_midi) = max_midi
                && pitch.midi > max_midi
            {
                return false;
            }
            true
        })
        .collect::<Vec<_>>();
    picked.sort_by_key(|note| {
        let midi = note.pitch.as_ref().map_or(0, |p| p.midi);
        (note.start, -midi)
    });
    picked.truncate(limit);
    picked
}

fn warning_repair_patches(xml: &str, report: &CheckReport, limit: usize) -> Vec<AtomicPatch> {
    let notes = parse_atomic_notes(xml);
    let mut patches = Vec::new();

    for issue in report.issues.iter().filter(|issue| issue.severity == Severity::Warning) {
        match issue.issue_type.as_str() {
            "PIANO_FAST_LARGE_JUMP" => {
                patches.extend(jump_atomic_patches(&notes, issue));
            }
            "PIANO_HAND_SPAN_LARGE" => {
                patches.extend(span_atomic_patches(&notes, issue));
            }
            "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF" => {
                patches.extend(overload_atomic_patches(&notes, issue));
            }
            _ => {}
        }
    }

    dedupe_atomic_patches(patches, limit)
}

fn dedupe_atomic_patches(patches: Vec<AtomicPatch>, limit: usize) -> Vec<AtomicPatch> {
    let mut unique: Vec<AtomicPatch> = Vec::new();
    for patch in patches {
        if unique.iter().any(|p: &AtomicPatch| {
            p.note_start == patch.note_start
                && p.note_end == patch.note_end
                && p.target_staff == patch.target_staff
                && p.target_voice == patch.target_voice
                && p.action == patch.action
                && p.insert_after == patch.insert_after
        }) {
            continue;
        }
        unique.push(patch);
        if unique.len() >= limit {
            break;
        }
    }
    unique
}

fn atomic_repair_patches(xml: &str, report: &CheckReport, limit: usize) -> Vec<AtomicPatch> {
    let notes = parse_atomic_notes(xml);
    let mut patches = Vec::new();

    for issue in report.issues.iter().filter(|issue| is_critical_piano_issue(issue)) {
        match issue.issue_type.as_str() {
            "PIANO_FAST_LARGE_JUMP" => {
                patches.extend(jump_atomic_patches(&notes, issue));
            }
            "PIANO_HAND_SPAN_LARGE" => {
                patches.extend(span_atomic_patches(&notes, issue));
            }
            _ => {}
        }
    }

    dedupe_atomic_patches(patches, limit)
}

fn jump_atomic_patches(notes: &[AtomicNote], issue: &Issue) -> Vec<AtomicPatch> {
    let mut patches = Vec::new();
    for (a, b) in find_jump_issue_note_pairs(notes, issue).into_iter().take(4) {
        let (Some(pa), Some(pb)) = (a.pitch.as_ref(), b.pitch.as_ref()) else { continue; };
        add_safe_atomic_variants(&mut patches, notes, a, &format!("jump{}_from_{}", issue.measure, sanitize_patch_label(&pa.label)));
        add_safe_atomic_variants(&mut patches, notes, b, &format!("jump{}_to_{}", issue.measure, sanitize_patch_label(&pb.label)));
        if pa.midi > pb.midi {
            add_safe_atomic_variants(&mut patches, notes, a, &format!("jump{}_high_{}", issue.measure, sanitize_patch_label(&pa.label)));
            add_safe_atomic_variants(&mut patches, notes, b, &format!("jump{}_low_{}", issue.measure, sanitize_patch_label(&pb.label)));
        } else {
            add_safe_atomic_variants(&mut patches, notes, b, &format!("jump{}_high_{}", issue.measure, sanitize_patch_label(&pb.label)));
            add_safe_atomic_variants(&mut patches, notes, a, &format!("jump{}_low_{}", issue.measure, sanitize_patch_label(&pa.label)));
        }
    }
    dedupe_atomic_patches(patches, 64)
}

fn find_jump_issue_note_pairs<'a>(
    notes: &'a [AtomicNote],
    issue: &Issue,
) -> Vec<(&'a AtomicNote, &'a AtomicNote)> {
    let Some(identity) = issue.han03_jump_identity.as_ref() else {
        return Vec::new();
    };
    let Some(staff) = issue.staff.as_deref() else {
        return Vec::new();
    };
    let voice_scope = if identity.line_kind.starts_with("voice-") {
        let Some(voice) = issue.voice.as_deref() else {
            return Vec::new();
        };
        Some(voice)
    } else if matches!(identity.line_kind.as_str(), "staff-outer-high" | "staff-outer-low") {
        None
    } else {
        return Vec::new();
    };

    let resolve = |endpoint: &crate::model::Han03EndpointIdentity| {
        notes
            .iter()
            .filter(|note| {
                note.measure == issue.measure.as_str()
                    && note.staff == staff
                    && voice_scope.is_none_or(|voice| note.voice.as_str() == voice)
                    && note.visible
                    && note.start == endpoint.start
                    && note.duration == endpoint.duration
                    && note.is_chord == endpoint.is_chord
                    && note.tie_stop == endpoint.tie_stop
                    && note.pitch.as_ref().is_some_and(|pitch| {
                        pitch.midi == endpoint.midi && pitch.label == endpoint.pitch_label
                    })
            })
            .collect::<Vec<_>>()
    };

    let from_matches = resolve(&identity.from);
    let to_matches = resolve(&identity.to);
    if from_matches.len() != 1 || to_matches.len() != 1 {
        return Vec::new();
    }
    let from_note = from_matches[0];
    let to_note = to_matches[0];
    if from_note.note_start == to_note.note_start {
        return Vec::new();
    }
    vec![(from_note, to_note)]
}

fn span_atomic_patches(notes: &[AtomicNote], issue: &Issue) -> Vec<AtomicPatch> {
    let Some(staff) = issue.staff.as_deref() else { return Vec::new(); };
    let actual = issue.actual_divisions.unwrap_or(0) as i32;
    let mut by_start: BTreeMap<i64, Vec<&AtomicNote>> = BTreeMap::new();
    for note in notes.iter().filter(|note| note.measure == issue.measure.as_str() && note.staff == staff && note.visible && note.pitch.is_some()) {
        by_start.entry(note.start).or_default().push(note);
    }

    let mut patches = Vec::new();
    for (_start, group) in by_start {
        if group.len() < 2 {
            continue;
        }
        let Some(low) = group.iter().min_by_key(|note| note.pitch.as_ref().map_or(0, |p| p.midi)).copied() else { continue; };
        let Some(high) = group.iter().max_by_key(|note| note.pitch.as_ref().map_or(0, |p| p.midi)).copied() else { continue; };
        let span = high.pitch.as_ref().unwrap().midi - low.pitch.as_ref().unwrap().midi;
        if span != actual {
            continue;
        }
        add_safe_atomic_variants(&mut patches, notes, high, &format!("span{}_high_{}", issue.measure, sanitize_patch_label(&high.pitch.as_ref().unwrap().label)));
        add_safe_atomic_variants(&mut patches, notes, low, &format!("span{}_low_{}", issue.measure, sanitize_patch_label(&low.pitch.as_ref().unwrap().label)));
        break;
    }
    patches
}


fn overload_atomic_patches(notes: &[AtomicNote], issue: &Issue) -> Vec<AtomicPatch> {
    let Some(staff) = issue.staff.as_deref() else { return Vec::new(); };
    let actual = issue.actual_divisions.unwrap_or(0).max(0) as usize;
    let mut by_start: BTreeMap<i64, Vec<&AtomicNote>> = BTreeMap::new();
    for note in notes.iter().filter(|note| note.measure == issue.measure.as_str() && note.staff == staff && note.visible && note.pitch.is_some()) {
        by_start.entry(note.start).or_default().push(note);
    }

    let mut patches = Vec::new();
    for (_start, mut group) in by_start {
        if group.len() != actual || group.len() <= 5 {
            continue;
        }
        group.sort_by_key(|note| note.pitch.as_ref().map_or(0, |p| p.midi));
        for note in group.iter().skip(1).take(group.len().saturating_sub(2)) {
            if let Some(pitch) = note.pitch.as_ref() {
                add_safe_atomic_variants(
                    &mut patches,
                    notes,
                    note,
                    &format!("overload{}_inner_{}", issue.measure, sanitize_patch_label(&pitch.label)),
                );
            }
        }
        break;
    }
    patches
}

fn add_safe_atomic_variants(patches: &mut Vec<AtomicPatch>, notes: &[AtomicNote], note: &AtomicNote, reason: &str) {
    let Some(pitch) = note.pitch.as_ref() else { return; };
    let target_staff = preferred_target_staff(note);
    let target_voice = note.voice.clone();

    if target_staff != note.staff {
        patches.push(AtomicPatch {
            name: format!(
                "safe_atomic_{}_move_m{}_s{}_{}_to_s{}",
                reason,
                note.measure,
                note.staff,
                sanitize_patch_label(&pitch.label),
                target_staff
            ),
            note_start: note.note_start,
            note_end: note.note_end,
            target_staff: target_staff.clone(),
            target_voice: target_voice.clone(),
            staff_value_range: note.staff_value_range,
            voice_value_range: note.voice_value_range,
            action: AtomicPatchAction::MoveStaff,
            insert_after: None,
            anchor_duration: None,
            material_transfer_carrier: Some(
                CandidateMaterialRepresentationTransferCarrier {
                    before_note_start: note.note_start,
                    before_note_end: note.note_end,
                    after_measure: note.measure.clone(),
                    after_onset: note.start,
                    after_pitch: pitch.midi,
                    after_staff: target_staff.clone(),
                    after_voice: target_voice.clone(),
                    after_is_chord: note.is_chord,
                },
            ),
        });
    }

    // v0.1.53: notationssichere Minimalvariante. Die ursprüngliche Stimme bleibt
    // zeitlich vollständig, die problematische Note wird nur als versteckte Pause
    // geführt. Das ist musikalisch konservativ als letzte Rettung: lieber lokal
    // eine kritische Ziel-/Sprungnote opfern als eine formale Blockade erzeugen.
    if !note.is_chord && note.pitch_block_range.is_some() && !atomic_note_is_tie_bound(note) {
        patches.push(AtomicPatch {
            name: format!(
                "safe_atomic_{}_hide_m{}_s{}_{}",
                reason,
                note.measure,
                note.staff,
                sanitize_patch_label(&pitch.label)
            ),
            note_start: note.note_start,
            note_end: note.note_end,
            target_staff: note.staff.clone(),
            target_voice: note.voice.clone(),
            staff_value_range: note.staff_value_range,
            voice_value_range: note.voice_value_range,
            action: AtomicPatchAction::RestifyHidden,
            insert_after: None,
            anchor_duration: None,
            material_transfer_carrier: None,
        });
    }

    // v0.1.53: echte Safe-Umhängung. Die Quellstimme bekommt eine versteckte
    // Pause, die Zielhand erhält die Note als Akkordton an einer bereits
    // vorhandenen Zielstimmenposition. Dadurch bleiben staff/voice-Abdeckungen
    // intakt, und der globale Cursor wird nicht angefasst.
    if target_staff != note.staff
        && !note.is_chord
        && note.pitch_block_range.is_some()
        && !atomic_note_is_tie_bound(note)
        && let Some(anchor) = find_safe_transfer_anchor(notes, note, &target_staff) {
            patches.push(AtomicPatch {
                name: format!(
                    "safe_atomic_{}_xfer_m{}_s{}_{}_to_s{}",
                    reason,
                    note.measure,
                    note.staff,
                    sanitize_patch_label(&pitch.label),
                    target_staff
                ),
                note_start: note.note_start,
                note_end: note.note_end,
                target_staff: target_staff.clone(),
                target_voice: anchor.voice.clone(),
                staff_value_range: note.staff_value_range,
                voice_value_range: note.voice_value_range,
                action: AtomicPatchAction::SafeTransferChord,
                insert_after: Some(anchor.note_end),
                anchor_duration: Some(anchor.duration),
                material_transfer_carrier: Some(
                    CandidateMaterialRepresentationTransferCarrier {
                        before_note_start: note.note_start,
                        before_note_end: note.note_end,
                        after_measure: note.measure.clone(),
                        after_onset: note.start,
                        after_pitch: pitch.midi,
                        after_staff: target_staff.clone(),
                        after_voice: anchor.voice.clone(),
                        after_is_chord: true,
                    },
                ),
            });
        }
}

fn preferred_target_staff(note: &AtomicNote) -> String {
    let midi = note.pitch.as_ref().map_or(60, |p| p.midi);
    match note.staff.as_str() {
        "2" if midi >= 60 => "1".to_string(),
        "1" if midi <= 60 => "2".to_string(),
        "2" => "1".to_string(),
        "1" => "2".to_string(),
        _ if midi >= 60 => "1".to_string(),
        _ => "2".to_string(),
    }
}

fn find_safe_transfer_anchor<'a>(notes: &'a [AtomicNote], note: &AtomicNote, target_staff: &str) -> Option<&'a AtomicNote> {
    notes
        .iter()
        .filter(|candidate| {
            candidate.measure == note.measure
                && candidate.start == note.start
                && candidate.staff == target_staff
                && candidate.pitch.is_some()
                && !candidate.is_chord
        })
        .min_by_key(|candidate| {
            let duration_gap = (candidate.duration - note.duration).abs();
            let pitch_gap = match (candidate.pitch.as_ref(), note.pitch.as_ref()) {
                (Some(a), Some(b)) => (a.midi - b.midi).abs() as i64,
                _ => 99,
            };
            (duration_gap, pitch_gap)
        })
}

fn apply_atomic_patch(xml: &str, patch: &AtomicPatch) -> (String, Option<ExactTransformationReceipt>) {
    match patch.action {
        AtomicPatchAction::MoveStaff => apply_staff_voice_move_patch(xml, patch),
        AtomicPatchAction::RestifyHidden => apply_restify_patch(xml, patch),
        AtomicPatchAction::SafeTransferChord => apply_safe_transfer_patch(xml, patch),
    }
}

#[cfg(test)]
#[path = "../../tests/unit/physical_rhythmic_birth_handoff.rs"]
mod physical_rhythmic_birth_handoff;

fn apply_staff_voice_move_patch(xml: &str, patch: &AtomicPatch) -> (String, Option<ExactTransformationReceipt>) {
    let (moved, receipt) = crate::structural_write_authority::apply_bound_staff_voice_move_authorized(
        xml,
        patch.voice_value_range,
        patch.staff_value_range,
        patch.note_end,
        patch.target_voice.clone(),
        patch.target_staff.clone(),
    );
    (
        moved.expect("bound staff/voice move capability must match the selected note"),
        receipt,
    )
}

fn apply_restify_patch(xml: &str, patch: &AtomicPatch) -> (String, Option<ExactTransformationReceipt>) {
    let Some(block) = xml.get(patch.note_start..patch.note_end) else {
        return (xml.to_string(), None);
    };
    let restified = restify_note_block(block);
    let receipt_edits = vec![crate::master_state_identity::ExactEdit::bound(
        patch.note_start,
        patch.note_end,
        restified.clone(),
    )];
    // The patch-level owner #63 mints the receipt for this transition; the
    // inner batch receipt (#39) describes the same edit and is not attached twice.
    let (restified_xml, mut inner_batch_receipt) =
        crate::structural_write_authority::apply_atomic_restification_batch_authorized(
            xml,
            vec![(patch.note_start, patch.note_end, restified)],
        );
    let Some(out) = restified_xml else {
        return (xml.to_string(), None);
    };
    // EXACT receipt #63, minted by this owner from the AtomicPatch range bound
    // at the patch origin and the restified replacement it just applied.
    let mut receipt = ExactTransformationReceipt::mint_exact_receipt(
        RestifyPatchExactMint { _sealed: () },
        xml,
        &out,
        receipt_edits,
    );
    if let (Some(inner), Some(outer)) = (inner_batch_receipt.as_mut(), receipt.as_mut())
        && let Some(births) = inner.take_physical_births() {
        outer.attach_physical_births(births).expect("Patch must carry its actual inner owner births without reminting");
    }
    (out, receipt)
}

/// Sealed mint capability of `apply_restify_patch` (EXACT #63). Its field is
/// private to the `pipeline` module, which owns this transformation.
pub(crate) struct RestifyPatchExactMint {
    _sealed: (),
}

fn apply_safe_transfer_patch(xml: &str, patch: &AtomicPatch) -> (String, Option<ExactTransformationReceipt>) {
    let Some(insert_after_original) = patch.insert_after else { return apply_restify_patch(xml, patch); };
    let source_block = &xml[patch.note_start..patch.note_end];
    let restified = restify_note_block(source_block);
    let transferred = transfer_chord_note_block(
        source_block,
        &patch.target_staff,
        &patch.target_voice,
        patch.anchor_duration,
    );

    let (transferred_xml, receipt) = crate::structural_write_authority::apply_bound_safe_transfer_authorized(
        xml,
        patch.note_start,
        patch.note_end,
        insert_after_original,
        restified,
        transferred,
    );
    (
        transferred_xml.expect(
            "bound safe-transfer capability must match the selected note and insertion anchor",
        ),
        receipt,
    )
}

fn restify_note_block(block: &str) -> String {
    crate::structural_write_authority::restify_atomic_move_note_authorized(block)
}

fn transfer_chord_note_block(block: &str, target_staff: &str, target_voice: &str, anchor_duration: Option<i64>) -> String {
    let mut out = block.to_string();
    out = ensure_chord_tag(&out);
    out = ensure_child_text_value(&out, "staff", target_staff);
    out = ensure_child_text_value(&out, "voice", target_voice);
    if let Some(duration) = anchor_duration.filter(|d| *d > 0) {
        out = ensure_child_text_value(&out, "duration", &duration.to_string());
    }
    out = remove_all_child_blocks(&out, "beam");
    out = sync_note_type_and_dots(&out);
    out
}

fn sync_note_type_and_dots(block: &str) -> String {
    crate::structural_write_authority::sync_atomic_move_note_type_and_dots_authorized(block)
}

fn ensure_chord_tag(block: &str) -> String {
    if block.contains("<chord") {
        return block.to_string();
    }
    let Some(open_end) = block.find('>') else { return block.to_string(); };
    let mut out = String::with_capacity(block.len() + 18);
    out.push_str(&block[..=open_end]);
    out.push_str("\n        <chord/>");
    out.push_str(&block[open_end + 1..]);
    out
}

fn ensure_child_text_value(block: &str, tag: &str, value: &str) -> String {
    crate::structural_write_authority::ensure_atomic_move_child_text_value_authorized(block, tag, value)
}

fn remove_all_child_blocks(block: &str, tag: &str) -> String {
    let mut out = block.to_string();
    while let Some((start, end)) = child_block_range_relative(&out, tag) {
        out.replace_range(start..end, "");
    }
    out
}

fn child_block_range_relative(block: &str, tag: &str) -> Option<(usize, usize)> {
    let open = format!("<{tag}");
    let start = block.find(&open)?;
    let open_end = start + block[start..].find('>')?;
    if block.as_bytes().get(open_end.saturating_sub(1)) == Some(&b'/') {
        return Some((start, open_end + 1));
    }
    let close = format!("</{tag}>");
    let end = open_end + 1 + block[open_end + 1..].find(&close)? + close.len();
    Some((start, end))
}

fn parse_atomic_notes(xml: &str) -> Vec<AtomicNote> {
    let mut notes = Vec::new();
    let mut search_from = 0usize;
    while let Some(measure_start_rel) = xml[search_from..].find("<measure") {
        let measure_start = search_from + measure_start_rel;
        let Some(open_end_rel) = xml[measure_start..].find('>') else { break; };
        let open_end = measure_start + open_end_rel;
        let open_tag = &xml[measure_start..=open_end];
        let measure_number = parse_attr_value(open_tag, "number").unwrap_or_else(|| "?".to_string());
        let Some(close_rel) = xml[open_end..].find("</measure>") else { break; };
        let measure_end = open_end + close_rel + "</measure>".len();
        parse_atomic_measure_notes(xml, open_end + 1, measure_end - "</measure>".len(), &measure_number, &mut notes);
        search_from = measure_end;
    }
    notes
}

fn parse_atomic_measure_notes(xml: &str, start: usize, end: usize, measure_number: &str, notes: &mut Vec<AtomicNote>) {
    let mut pos = start;
    let mut cursor = 0_i64;
    let mut last_non_chord_start = 0_i64;
    let mut run = 0_usize;

    while pos < end {
        let next_note = find_next_tag(xml, pos, end, "<note");
        let next_backup = find_next_tag(xml, pos, end, "<backup");
        let next_forward = find_next_tag(xml, pos, end, "<forward");
        let Some((tag_start, tag_name)) = smallest_tag([next_note.map(|v| (v, "note")), next_backup.map(|v| (v, "backup")), next_forward.map(|v| (v, "forward"))]) else {
            break;
        };

        match tag_name {
            "note" => {
                let Some(note_close_rel) = xml[tag_start..end].find("</note>") else { break; };
                let note_end = tag_start + note_close_rel + "</note>".len();
                let block = &xml[tag_start..note_end];
                let duration = child_text_from_block(block, "duration")
                    .and_then(|s| s.trim().parse::<i64>().ok())
                    .unwrap_or(0)
                    .max(0);
                let is_chord = block.contains("<chord");
                let is_rest = block.contains("<rest");
                let has_staccato = block.contains("<staccato") || block.contains("<staccatissimo");
                let tie_start =
                    block.contains("<tie type=\"start\"") || block.contains("<tie type='start'");
                let tie_stop =
                    block.contains("<tie type=\"stop\"") || block.contains("<tie type='stop'");
                let open_tag_end = block.find('>').map_or(block.len(), |idx| idx + 1);
                let note_is_hidden = block
                    .get(..open_tag_end)
                    .is_some_and(|open_tag| open_tag.contains("print-object=\"no\"") || open_tag.contains("print-object='no'"));
                let visible = !note_is_hidden;
                let staff = child_text_from_block(block, "staff")
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "1".to_string());
                let voice = child_text_from_block(block, "voice")
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "1".to_string());
                let note_start_time = if is_chord { last_non_chord_start } else { cursor };
                let pitch = if is_rest { None } else { parse_pitch_from_note_block(block) };
                let staff_value_range = child_value_range(block, tag_start, "staff");
                let voice_value_range = child_value_range(block, tag_start, "voice");
                let pitch_block_range = child_block_range(block, tag_start, "pitch");

                notes.push(AtomicNote {
                    measure: measure_number.to_string(),
                    run,
                    staff,
                    voice,
                    start: note_start_time,
                    duration,
                    pitch,
                    visible,
                    note_start: tag_start,
                    note_end,
                    staff_value_range,
                    voice_value_range,
                    pitch_block_range,
                    is_chord,
                    has_staccato,
                    tie_start,
                    tie_stop,
                });

                if !is_chord {
                    last_non_chord_start = note_start_time;
                    cursor += duration;
                }
                pos = note_end;
            }
            "backup" => {
                let Some(block_end_rel) = xml[tag_start..end].find("</backup>") else { break; };
                let block_end = tag_start + block_end_rel + "</backup>".len();
                let block = &xml[tag_start..block_end];
                let duration = child_text_from_block(block, "duration")
                    .and_then(|s| s.trim().parse::<i64>().ok())
                    .unwrap_or(0)
                    .max(0);
                cursor -= duration;
                run += 1;
                pos = block_end;
            }
            "forward" => {
                let Some(block_end_rel) = xml[tag_start..end].find("</forward>") else { break; };
                let block_end = tag_start + block_end_rel + "</forward>".len();
                let block = &xml[tag_start..block_end];
                let duration = child_text_from_block(block, "duration")
                    .and_then(|s| s.trim().parse::<i64>().ok())
                    .unwrap_or(0)
                    .max(0);
                cursor += duration;
                pos = block_end;
            }
            _ => break,
        }
    }
}

fn find_next_tag(xml: &str, start: usize, end: usize, tag: &str) -> Option<usize> {
    xml[start..end].find(tag).map(|offset| start + offset)
}

fn smallest_tag(items: [Option<(usize, &'static str)>; 3]) -> Option<(usize, &'static str)> {
    items.into_iter().flatten().min_by_key(|(idx, _)| *idx)
}

fn parse_attr_value(tag: &str, attr: &str) -> Option<String> {
    let needle = format!("{attr}=\"");
    if let Some(start) = tag.find(&needle) {
        let value_start = start + needle.len();
        let value_end = value_start + tag[value_start..].find('"')?;
        return Some(tag[value_start..value_end].to_string());
    }
    let needle = format!("{attr}='");
    let start = tag.find(&needle)?;
    let value_start = start + needle.len();
    let value_end = value_start + tag[value_start..].find('\'')?;
    Some(tag[value_start..value_end].to_string())
}

fn child_text_from_block(block: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}");
    let start = block.find(&open)?;
    let value_start = start + block[start..].find('>')? + 1;
    let close = format!("</{tag}>");
    let value_end = value_start + block[value_start..].find(&close)?;
    Some(block[value_start..value_end].to_string())
}

fn child_value_range(block: &str, absolute_block_start: usize, tag: &str) -> Option<(usize, usize)> {
    let open = format!("<{tag}");
    let start = block.find(&open)?;
    let value_start = start + block[start..].find('>')? + 1;
    let close = format!("</{tag}>");
    let value_end = value_start + block[value_start..].find(&close)?;
    Some((absolute_block_start + value_start, absolute_block_start + value_end))
}

fn child_block_range(block: &str, absolute_block_start: usize, tag: &str) -> Option<(usize, usize)> {
    let open = format!("<{tag}");
    let start = block.find(&open)?;
    let open_end = start + block[start..].find('>')?;
    if block.as_bytes().get(open_end.saturating_sub(1)) == Some(&b'/') {
        return Some((absolute_block_start + start, absolute_block_start + open_end + 1));
    }
    let close = format!("</{tag}>");
    let end = open_end + 1 + block[open_end + 1..].find(&close)? + close.len();
    Some((absolute_block_start + start, absolute_block_start + end))
}

fn parse_pitch_from_note_block(block: &str) -> Option<Pitch> {
    let pitch_start = block.find("<pitch")?;
    let pitch_end = pitch_start + block[pitch_start..].find("</pitch>")? + "</pitch>".len();
    let pitch_block = &block[pitch_start..pitch_end];
    let step = child_text_from_block(pitch_block, "step")?.trim().to_string();
    let alter = child_text_from_block(pitch_block, "alter")
        .and_then(|s| s.trim().parse::<i32>().ok())
        .unwrap_or(0);
    let octave = child_text_from_block(pitch_block, "octave")?.trim().parse::<i32>().ok()?;
    Some(Pitch::new(step, alter, octave))
}

fn sanitize_patch_label(label: &str) -> String {
    let cleaned = label
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect::<String>()
        .trim_matches('_')
        .to_string();
    if cleaned.is_empty() { "note".to_string() } else { cleaned }
}

#[cfg(test)]
mod tie_safe_restify_tests {
    use super::*;

    fn tied_pair_xml() -> String {
        r#"<score-partwise><part id="P1"><measure number="25">
<note><pitch><step>G</step><octave>4</octave></pitch><duration>48</duration><tie type="start"/><voice>2</voice><staff>2</staff><notations><tied type="start"/></notations></note>
<note><pitch><step>G</step><octave>4</octave></pitch><duration>96</duration><tie type="stop"/><voice>2</voice><staff>2</staff><notations><tied type="stop"/></notations></note>
</measure></part></score-partwise>"#.to_string()
    }

    #[test]
    fn atomic_parser_carries_tie_role() {
        let notes = parse_atomic_notes(&tied_pair_xml());
        assert_eq!(notes.len(), 2);
        assert!(notes[0].tie_start);
        assert!(!notes[0].tie_stop);
        assert!(!notes[1].tie_start);
        assert!(notes[1].tie_stop);
    }

    #[test]
    fn generic_restify_choice_rejects_tie_bound_notes() {
        let notes = parse_atomic_notes(&tied_pair_xml());
        assert!(restify_choice_from_notes("hide_start", &[&notes[0]]).is_none());
        assert!(restify_choice_from_notes("hide_stop", &[&notes[1]]).is_none());
        assert!(restify_choice_from_notes("hide_pair", &[&notes[0], &notes[1]]).is_none());
    }

    #[test]
    fn atomic_fallback_keeps_only_non_restifying_actions_for_tie_bound_note() {
        let notes = parse_atomic_notes(&tied_pair_xml());
        let mut patches = Vec::new();
        add_safe_atomic_variants(&mut patches, &notes, &notes[0], "tie_guard");
        assert!(!patches.is_empty());
        assert!(patches
            .iter()
            .all(|patch| patch.action == AtomicPatchAction::MoveStaff));
        assert!(patches
            .iter()
            .all(|patch| patch.target_voice == notes[0].voice));
    }

    #[test]
    fn trajectory_restaff_preserves_existing_voice_lane() {
        let notes = parse_atomic_notes(&tied_pair_xml());
        let target = move_target_for_note(&notes[0], "1").expect("cross-staff target must exist");
        assert_eq!(target.target_staff, "1");
        assert_eq!(target.target_voice, notes[0].voice);
    }

    fn han03_issue(details: &str, identity: Option<crate::model::Han03JumpIdentity>) -> Issue {
        Issue {
            severity: Severity::Warning,
            issue_type: "PIANO_FAST_LARGE_JUMP".to_string(),
            rule_id: "HAN03".to_string(),
            rule_severity: "WARNUNG".to_string(),
            rule_weight: 1200,
            part_id: "P1".to_string(),
            part_name: "Piano".to_string(),
            measure: "25".to_string(),
            staff: Some("2".to_string()),
            voice: Some("2".to_string()),
            expected_divisions: None,
            actual_divisions: Some(24),
            actual_unit: Some("Halbtöne".to_string()),
            delta_divisions: None,
            han03_jump_identity: identity,
            details: details.to_string(),
            message_for_ai: String::new(),
        }
    }

    fn canonical_g_sharp_jump_identity() -> crate::model::Han03JumpIdentity {
        crate::model::Han03JumpIdentity {
            line_kind: "voice-high".to_string(),
            from: crate::model::Han03EndpointIdentity {
                start: 48,
                duration: 48,
                midi: 44,
                pitch_label: "G#2".to_string(),
                is_chord: false,
                tie_stop: false,
            },
            to: crate::model::Han03EndpointIdentity {
                start: 144,
                duration: 48,
                midi: 68,
                pitch_label: "G#4".to_string(),
                is_chord: false,
                tie_stop: false,
            },
            temporal_edge: crate::model::Han03TemporalEdge {
                from_end: 96,
                onset_delta: 96,
                release_to_attack_gap: 48,
                direct_attack_successor: false,
            },
        }
    }

    #[test]
    fn han03_locator_uses_canonical_identity_instead_of_issue_text() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
25\">\n<forward><duration>48</duration></forward>\n<note><pitch><step>G</step><alter>1</alter><octave>2</octave></pitch><duration>48</duration><voice>2</voice><staff>2</staff></note>\n<note><pitch><step>G</step><alter>1</alter><octave>2</octave></pitch><duration>48</duration><tie type=\"stop\"/><voice>2</voice><staff>2</staff></note>\n<note><pitch><step>G</step><alter>1</alter><octave>4</octave></pitch><duration>48</duration><voice>2</voice><staff>2</staff></note>\n</measure></part></score-partwise>";
        let notes = parse_atomic_notes(xml);
        let issue = han03_issue(
            "absichtlich nicht parsebarer Legacy-Text",
            Some(canonical_g_sharp_jump_identity()),
        );
        let pairs = find_jump_issue_note_pairs(&notes, &issue);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].0.start, 48);
        assert!(!pairs[0].0.tie_stop);
        assert_eq!(pairs[0].1.start, 144);
    }

    #[test]
    fn han03_locator_fails_closed_without_canonical_identity() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
25\">\n<forward><duration>48</duration></forward>\n<note><pitch><step>G</step><alter>1</alter><octave>2</octave></pitch><duration>48</duration><voice>2</voice><staff>2</staff></note>\n<note><pitch><step>G</step><alter>1</alter><octave>4</octave></pitch><duration>48</duration><voice>2</voice><staff>2</staff></note>\n</measure></part></score-partwise>";
        let notes = parse_atomic_notes(xml);
        let issue = han03_issue(
            "Schneller Sprung auf Staff 2 Voice 2 Linie voice-high: G#2 -> G#4. Sprungweite: 24 Halbtöne; Zeitabstand: 48 divisions.",
            None,
        );
        assert!(find_jump_issue_note_pairs(&notes, &issue).is_empty());
    }
}

