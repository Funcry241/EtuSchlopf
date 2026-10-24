fn normalize_current_facing_report_terms(text: &mut String) {
    let replacements = [
        ("Peter-Scholze-/Grothendieck-Mathematik", "LocalSection-/CategoricalGluing-Mathematik"),
        ("Grothendieck-Kategorie", "CategoricalGluing-Kategorie"),
        ("GrothendieckCategoryRows", "CategoricalGluingRows"),
        ("GrothendieckCategoryLedger", "CategoricalGluingLedger"),
        ("Grothendieck liefert die Kategorie, Scholze liefert die lokalen Beobachtungstopologien", "CategoricalGluing beschreibt die Kategorie, LocalSectionProjection beschreibt die lokalen Beobachtungstopologien"),
        ("Grothendieck", "CategoricalGluing"),
        ("ScholzeProjectionRows", "LocalSectionProjectionRows"),
        ("ScholzeProjectionLedger", "LocalSectionProjectionLedger"),
        ("Scholze-Projektionen", "LocalSectionProjection-Projektionen"),
        ("Scholze", "LocalSectionProjection"),
        ("SCHLOPF-KERNEL", "DECISION-KERNEL"),
        ("SchlopfKernel", "DecisionKernel"),
        ("Schlopf-Kernel", "Decision-Kernel"),
        ("SISTER-/LOWER-STAFF-GATE", "EXTERNAL-LOWER-STAFF-GATE"),
        ("SISTER-FEEDBACK", "EXTERNAL-NOTATION-FEEDBACK"),
        ("SisterFeedback", "ExternalNotationFeedback"),
        ("SisterReadyScorePreview", "ExternalFeedbackReadyScorePreview"),
        ("sister_ready", "external_feedback_ready"),
        ("sister-feedback", "external-feedback"),
        ("sister feedback", "external feedback"),
    ];
    for (from, to) in replacements {
        if text.contains(from) {
            *text = text.replace(from, to);
        }
    }
}






fn extract_manifest_version_from_text(text: &str) -> Option<String> {
    text.lines()
        .map(str::trim)
        .find(|line| line.starts_with("version") && line.contains('='))
        .and_then(|line| line.split_once('=').map(|(_, rhs)| rhs.trim().trim_matches('"').to_string()))
        .filter(|value| !value.is_empty())
}

fn extract_fixed_launcher_version_from_text(text: &str) -> Option<String> {
    text.lines()
        .map(str::trim)
        .find(|line| line.to_ascii_lowercase().starts_with("set etuschlopf_version=")
            || line.to_ascii_lowercase().starts_with("set \"etuschlopf_version="))
        .and_then(|line| line.split_once('=').map(|(_, rhs)| rhs.trim().trim_matches('"').to_string()))
        .filter(|value| !value.is_empty() && value != "unknown")
}
