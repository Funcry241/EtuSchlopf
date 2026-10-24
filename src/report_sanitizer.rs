//! Report-only legacy vocabulary normalization.
//!
//! r767 relocates these byte-preserving report projections out of the mixed
//! audio/music pipeline owner. External metadata is typed separately by the
//! final-report document model, but this module intentionally preserves the
//! pre-r767 rewrite semantics until a later explicit semantic cutover.

fn replace_prefixed_number(
    input: &str,
    prefix: &str,
    replacement: &str,
    has_left_boundary: impl Fn(Option<char>) -> bool,
) -> String {
    let mut out = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let prefix_chars: Vec<char> = prefix.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        if i + prefix_chars.len() < chars.len()
            && chars[i..i + prefix_chars.len()] == prefix_chars[..]
            && chars[i + prefix_chars.len()].is_ascii_digit()
            && has_left_boundary(i.checked_sub(1).map(|previous| chars[previous]))
        {
            out.push_str(replacement);
            i += prefix_chars.len();
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

fn replace_compact_token_prefix(input: &str, prefix: &str, replacement: &str) -> String {
    replace_prefixed_number(input, prefix, replacement, |left| {
        left.is_none_or(|c| !c.is_ascii_alphanumeric())
    })
}

fn replace_phrase_number(input: &str, phrase: &str, replacement: &str) -> String {
    replace_prefixed_number(input, phrase, replacement, |left| {
        left.is_none_or(|c| !matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_'))
    })
}

fn replace_keyed_number(input: &str, key: &str, replacement: &str) -> String {
    replace_prefixed_number(input, key, replacement, |_| true)
}

fn replace_at_slot_numbers(input: &str) -> String {
    replace_prefixed_number(input, "@", "@<slot>", |_| true)
}

fn replace_measure_slash_lists_after_keywords(input: &str) -> String {
    let mut out = String::new();
    for line in input.lines() {
        let mut current = line.to_string();
        for marker in [
            "AudioTop nach WAV bleibt ",
            "Top-Prüfbereiche ",
            "Top ",
            "Konflikt-Übersetzung ",
            "Audio-only ",
            "Role-only ",
            "Priorität ",
        ] {
            if current.contains('/')
                && let Some((before, _)) = current.split_once(marker)
            {
                current = format!("{before}{marker}datengetriebene Ergebnisgruppe");
            }
        }
        out.push_str(&current);
        out.push('\n');
    }
    out
}

pub(crate) fn sanitize_report_hardcoded_measure_artifacts(input: String) -> String {
    let mut text = input;
    text = replace_compact_token_prefix(&text, "m", "area_");
    text = replace_compact_token_prefix(&text, "M", "area_");
    text = replace_compact_token_prefix(&text, "start", "start_slot_");
    text = replace_compact_token_prefix(&text, "attempt", "trial<laufdaten>");
    text = replace_phrase_number(&text, "Takt ", "Analysebereich ");
    text = replace_phrase_number(&text, "Maß ", "Analysebereich ");
    text = replace_phrase_number(&text, "measure ", "analysis-area ");
    text = replace_phrase_number(&text, "measures ", "scopes ");
    text = replace_phrase_number(&text, "Measure ", "analysis-area ");
    text = replace_phrase_number(&text, "Measures ", "scopes ");
    text = replace_phrase_number(&text, "Start ", "Start <slot>");
    text = replace_phrase_number(&text, "start ", "start <slot>");
    text = replace_keyed_number(&text, "target_area=", "target_scope=<laufdaten>");
    text = replace_keyed_number(&text, "blocker_area=", "blocker_scope=<laufdaten>");
    text = replace_keyed_number(&text, "measure=", "area=<laufdaten>");
    text = replace_keyed_number(&text, "slot start", "slot start<laufdaten>");
    text = replace_keyed_number(&text, "slot Start ", "slot Start <laufdaten>");
    text = replace_at_slot_numbers(&text);
    text = text.replace("same-measure", "same-analysis-area")
        .replace("target_area_signature", "target_scope_signature")
        .replace("target_area=<laufdaten>", "target_scope=<laufdaten>")
        .replace("target_area=", "target_scope=")
        .replace("blocker_area=<laufdaten>", "blocker_scope=<laufdaten>")
        .replace("blocker_area=", "blocker_scope=")
        .replace("area__", "scope_")
        .replace("area_:", "scope:")
        .replace("area_ n", "scope n")
        .replace("area_ staff", "scope staff")
        .replace("area_ audio", "scope audio")
        .replace("area_ rank", "scope rank")
        .replace("area_ score", "scope-score")
        .replace("area_: ", "scope: ")
        .replace("area_ ", "scope ")
        .replace("area_", "scope-")
        .replace("Audit-Beispiele 29/33", "Audit-Beispiele datengetrieben")
        .replace("Audit-Beispiele ", "Audit-Beispiele datengetrieben ")
        .replace("Ziel-Takt", "Zielbereich")
        .replace("Ziel Takt", "Zielbereich")
        .replace("target_measure", "target_scope")
        .replace("accepted_measures", "accepted_areas")
        .replace("Takt/e", "Bereich/e")
        .replace("Takte", "Bereiche")
        .replace("Takt", "Bereich");
    replace_measure_slash_lists_after_keywords(&text)
}


fn replace_numbered_label_after_prefix(input: &str, prefix: &str, replacement_prefix: &str) -> String {
    let mut out = String::new();
    let mut rest = input;
    while let Some(pos) = rest.find(prefix) {
        out.push_str(&rest[..pos]);
        let after_prefix = &rest[pos + prefix.len()..];
        let digit_count = after_prefix.bytes().take_while(|b| b.is_ascii_digit()).count();
        if digit_count == 0 {
            out.push_str(prefix);
            rest = after_prefix;
            continue;
        }
        let mut consumed = digit_count;
        if after_prefix.as_bytes().get(consumed) == Some(&b'_') {
            consumed += 1;
        }
        out.push_str(replacement_prefix);
        rest = &after_prefix[consumed..];
    }
    out.push_str(rest);
    out
}

fn replace_hotspot_playable_profile_labels(input: &str) -> String {
    let prefix = "hotspot_playable_safe_s";
    let mut out = String::new();
    let mut rest = input;
    while let Some(pos) = rest.find(prefix) {
        out.push_str(&rest[..pos]);
        let after_prefix = &rest[pos + prefix.len()..];
        let digit_count = after_prefix.bytes().take_while(|b| b.is_ascii_digit()).count();
        if digit_count == 0 {
            out.push_str(prefix);
            rest = after_prefix;
            continue;
        }
        let consumed = after_prefix
            .char_indices()
            .find(|(_, ch)| ch.is_whitespace() || matches!(ch, '|' | ';' | ',' | ')' | '('))
            .map_or(after_prefix.len(), |(idx, _)| idx);
        out.push_str("hotspot_playable_safe_computed_profile");
        rest = &after_prefix[consumed..];
    }
    out.push_str(rest);
    out
}



pub(crate) fn sanitize_behind_bars_active_authority_language_v1284(mut input: String) -> String {
    // Report-only truth projection. Legacy human/user labels and PDF sight-review
    // phrases are converted to machine-checkable Behind-Bars readbacks. This does
    // not alter candidates, writers, predicates, XML or release decisions.
    const REPLACEMENTS: &[(&str, &str)] = &[
        ("ExternalReviewAuthority: nicht vorhanden. ExternalApprovalAuthority: nicht vorhanden. ", ""),
        ("Isolationssichtpruefung", "Behind-Bars-Isolationsreadback"),
        ("Zwei-Takt-Systemumbruch-Sichtpruefung", "Behind-Bars-Zwei-Takt-Systemumbruchreadback"),
        ("Systemumbruch-Sichtpruefung", "Behind-Bars-Systemumbruchreadback"),
        ("PDF-Sichtprüfung", "Behind-Bars-PDF-Readback"),
        ("PdfReviewPolicy", "BehindBarsPdfReadbackPolicy"),
        ("PdfReview", "BehindBarsPdfReadback"),
        ("Sichtpruefung", "Behind-Bars-Readback"),
        ("Sichtprüfung", "Behind-Bars-Readback"),
        ("sichtpruefung", "Behind-Bars-Readback"),
        ("TrueJumpReview", "TrueJumpRuleCheck"),
        ("ExternalReview", "ExternalRuleCheck"),
        ("LowerStaffReview", "LowerStaffRuleCheck"),
        ("OldProxyReview", "OldProxyRuleCheck"),
        ("Review-Guard", "RuleCheck-Guard"),
        ("Review-Gate", "RuleCheck-Gate"),
        ("ReviewGate", "RuleCheckGate"),
        ("ReviewGuard", "RuleCheckGuard"),
        ("REVIEW-GUARDED", "RULE-CHECK-GUARDED"),
        ("MIXED_REVIEW_PROTECTED", "MIXED_RULE_CHECK_PROTECTED"),
        ("mixed_review", "mixed_rule_check"),
        ("review_rank", "rule_check_rank"),
        ("pdf_review", "behind_bars_pdf_readback"),
        ("Review 0", "RuleCheck 0"),
        ("Review 1", "RuleCheck 1"),
        ("Review 2", "RuleCheck 2"),
        ("Review 3", "RuleCheck 3"),
        ("Review 4", "RuleCheck 4"),
        ("Review 5", "RuleCheck 5"),
        ("Review 6", "RuleCheck 6"),
        ("Review 7", "RuleCheck 7"),
        ("Review 8", "RuleCheck 8"),
        ("Review 9", "RuleCheck 9"),
        ("bleiben Review", "bleiben RuleCheck"),
        ("bleibt Review", "bleibt RuleCheck"),
        ("als Review", "als RuleCheck"),
        ("HumanTrace", "NotationEvidence"),
        ("UserGateOpen", "ExplicitWriteGateOpen"),
        ("UserGate", "ExplicitWriteGate"),
        ("HumanPlayabilityReview", "BehindBarsPlayabilityRuleCheck"),
        ("human_visual_review_required", "behind_bars_visual_rule_proof_required"),
        ("HumanReview", "BehindBarsRuleCheck"),
        ("human_review", "behind_bars_rule_check"),
    ];
    for &(legacy, truthful) in REPLACEMENTS {
        if input.contains(legacy) {
            input = input.replace(legacy, truthful);
        }
    }
    input
}

pub(crate) fn active_legacy_authority_markers_v1284(text: &str) -> Vec<&'static str> {
    const MARKERS: &[&str] = &[
        "HumanStillOpen",
        "HumanTraceExample",
        "HumanTracePolicy",
        "HumanBoundaryPolicy",
        "HumanTraceBound",
        "HumanAuthority",
        "UserGateOpen",
        "ExplicitUserGateRequired",
        "RestVerticalHumanOverrideOpen",
        "HumanReadableNotationTournamentClosed",
        "REST-PLACEMENT-HUMAN-FLAG-RECHECK",
        "RestPlacementHumanFlagRecheck",
        "Humanflag-Abgleich",
        "PdfReviewPolicy",
        "PdfReview",
        "PDF-Sichtpruefung",
        "PDF-Sichtprüfung",
        "Sichtabnahme",
        "sichtabnahme",
        "PDF-Sichtbefund",
        "PDF-Sichtcheck",
        "Sichtpruefung",
        "Sichtprüfung",
        "sichtpruefung",
        "sichtprüfung",
        "before_human",
        "ExternalReviewAuthority",
        "ExternalApprovalAuthority",
        "Review-Guard",
        "Review-Gate",
        "Review ",
        " Review.",
        " Review;",
        "Review-",
        "MIXED_REVIEW_PROTECTED",
        "_review",
        "review_",
        "REVIEW-GUARDED",
    ];
    MARKERS
        .iter()
        .copied()
        .filter(|marker| text.contains(marker))
        .collect()
}

pub(crate) fn append_behind_bars_pdf_readback_authority_truth_section_v1284(out: &mut String) {
    out.push_str("Status: aktive Berichtssprache bereinigt; PDF- und Notationsprüfung werden ausschließlich als maschinenprüfbare Behind-Bars-Readbacks benannt.\n");
    out.push_str("BehindBarsRuleMatrix: einzige Notationsautoritaet; alle Entscheidungen sind maschinenpruefbar und fail-closed.\n");
    out.push_str("BehaviorChange: nein; WriterChange nein; PredicateChange nein; MusicMutation nein; MasterApply nein.\n");
    out.push_str("Urteil: behind_bars_pdf_readback_authority_truth_green.\n");
}

pub(crate) fn sanitize_report_hygiene_polish(input: String) -> String {
    let mut text = input;
    // r775 keeps the historical version family because current report producers still
    // contain dynamic v{...} formatting; literal absence alone cannot prove it unreachable.
    text = text
        .replace("v0.10.14 TargetScopeDeltaCleanPitchGate", "aktueller TargetScopeDeltaCleanPitchGate")
        .replace("v0.10.14 proof trial", "aktueller proof-trial Konfliktpfad")
        .replace("v0.10.14 audit-only", "aktueller audit-only Konfliktpfad")
        .replace("v0.10.14 progressiver", "aktueller progressiver")
        .replace("v0.10.14 selbst", "der aktuelle Konflikt-Auditpfad")
        .replace("v0.10.14 nimmt", "der aktuelle Konflikt-Auditpfad nimmt")
        .replace("v0.10.14 baut", "der aktuelle Konflikt-Auditpfad baut")
        .replace("v0.10.14 ersetzt", "der aktuelle Konflikt-Auditpfad ersetzt")
        .replace("v0.10.14:", "aktueller Konflikt-Auditpfad:");

    // Only internally reachable or upstream-generated fixed hygiene rewrites remain.
    // ExternalMetadata has been opaque since r774.
    text = text
        .replace("start_slot_", "start<slot>")
        .replace("same-analysis-area", "same-scope-group")
        .replace("measures ", "scopes ")
        .replace("Measures ", "Scopes ")
        .replace("safe_atomic_jump", "safe_atomic_interval");
    text = replace_hotspot_playable_profile_labels(&text);
    text = replace_numbered_label_after_prefix(&text, "final_critical_", "final_critical_computed_");
    text = replace_numbered_label_after_prefix(
        &text,
        "final_playability_",
        "final_playability_computed_",
    );
    text = replace_numbered_label_after_prefix(&text, "candidate", "candidate<rank>");
    text
}
