use super::rust_source_scan_primitives::{
    collect_rust_files, line_number_at, line_starts, mask_non_code, normalize_path,
    token_boundary_after, token_boundary_before,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IdentityDebtFinding {
    pub kind: String,
    pub path: String,
    pub function: String,
    pub line: usize,
    pub payload: String,
    pub evidence: String,
}

impl IdentityDebtFinding {
    pub fn key(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.kind, self.path, self.function, self.payload
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct IdentityDebtScanResult {
    pub source_files: usize,
    pub findings: Vec<IdentityDebtFinding>,
    pub allowed_localizations: usize,
}

#[derive(Debug, Clone)]
struct FunctionSpan {
    name: String,
    start: usize,
    end: usize,
    start_line: usize,
    test_only: bool,
}

pub fn scan_source_roots(
    project_root: &Path,
    source_roots: &[&Path],
) -> io::Result<IdentityDebtScanResult> {
    let mut files = Vec::new();
    for source_root in source_roots {
        collect_rust_files(source_root, &mut files)?;
    }
    files.sort();
    files.dedup();

    let mut result = IdentityDebtScanResult {
        source_files: files.len(),
        ..IdentityDebtScanResult::default()
    };

    for path in files {
        let text = fs::read_to_string(&path)?;
        let relative = normalize_path(path.strip_prefix(project_root).unwrap_or(&path));
        let mut file_result = scan_source_text(&relative, &text);
        result.allowed_localizations += file_result.allowed_localizations;
        result.findings.append(&mut file_result.findings);
    }

    result.findings.sort();
    result.findings.dedup_by(|a, b| a.key() == b.key());
    Ok(result)
}

pub fn scan_source_text(path: &str, source: &str) -> IdentityDebtScanResult {
    let masked = mask_non_code(source);
    let line_starts = line_starts(source);
    let test_spans = test_only_spans(&masked);
    let functions = function_spans(&masked, &line_starts, &test_spans);
    let project_identity_constants = collect_project_identity_constants(source);
    let mut findings = Vec::new();
    let mut allowed_localizations = 0usize;

    for function in &functions {
        let body = &source[function.start..function.end];
        let body_masked = &masked[function.start..function.end];
        let is_writer = is_productive_mutation_function(&function.name);
        let is_selector = is_project_selector_function(&function.name);
        let identity_symbol = identity_symbol_payload(&function.name);

        if function.test_only
            || is_allowed_localization_function(&function.name)
                && !has_direct_mutation_surface(body)
                && !has_mutation_call_surface(body)
        {
            allowed_localizations += count_localization_markers(body);
            continue;
        }

        let measure_hits = collect_absolute_measure_hits(
            body,
            body_masked,
            function.start_line,
        );
        if !measure_hits.is_empty() {
            let measures = measure_hits
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(",");
            let (line, evidence) = first_hit_evidence(&measure_hits);
            findings.push(IdentityDebtFinding {
                kind: if is_writer {
                    "ABSOLUTE_MEASURE_SELECTOR".to_string()
                } else {
                    "ABSOLUTE_MEASURE_DEPENDENCY".to_string()
                },
                path: path.to_string(),
                function: function.name.clone(),
                line,
                payload: format!("measures={measures}"),
                evidence,
            });
        }

        let mut direct_project_hits = collect_project_identity_hits(
            body,
            function.start_line,
        );
        direct_project_hits.extend(collect_project_identity_constant_hits(
            body,
            function.start_line,
            &project_identity_constants,
        ));
        if !direct_project_hits.is_empty() {
            let projects = direct_project_hits
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(",");
            let (line, evidence) = first_hit_evidence(&direct_project_hits);
            findings.push(IdentityDebtFinding {
                kind: "PROJECT_IDENTITY_SELECTOR".to_string(),
                path: path.to_string(),
                function: function.name.clone(),
                line,
                payload: format!("projects={projects}"),
                evidence,
            });
        }

        if is_writer {
            if let Some(payload) = identity_symbol {
                findings.push(IdentityDebtFinding {
                    kind: "IDENTITY_NAMED_WRITER".to_string(),
                    path: path.to_string(),
                    function: function.name.clone(),
                    line: function.start_line,
                    payload,
                    evidence: declaration_evidence(body),
                });
            }

            if let Some(payload) = feedback_origin_symbol_payload(&function.name) {
                findings.push(IdentityDebtFinding {
                    kind: "FEEDBACK_ORIGIN_NAMED_WRITER".to_string(),
                    path: path.to_string(),
                    function: function.name.clone(),
                    line: function.start_line,
                    payload,
                    evidence: declaration_evidence(body),
                });
            }

            let feedback_channel_hits = collect_feedback_origin_channel_hits(
                body,
                function.start_line,
            );
            if !feedback_channel_hits.is_empty() {
                let channels = feedback_channel_hits
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(",");
                let (line, evidence) = first_hit_evidence(&feedback_channel_hits);
                findings.push(IdentityDebtFinding {
                    kind: "PRODUCTIVE_FEEDBACK_ORIGIN_CHANNEL".to_string(),
                    path: path.to_string(),
                    function: function.name.clone(),
                    line,
                    payload: format!("channels={channels}"),
                    evidence,
                });
            }

            let identity_predicate_hits = collect_identity_channel_predicate_hits(
                body,
                function.start_line,
            );
            if !identity_predicate_hits.is_empty() {
                let channels = identity_predicate_hits
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(",");
                let (line, evidence) = first_hit_evidence(&identity_predicate_hits);
                findings.push(IdentityDebtFinding {
                    kind: "PRODUCTIVE_IDENTITY_CHANNEL_PREDICATE".to_string(),
                    path: path.to_string(),
                    function: function.name.clone(),
                    line,
                    payload: format!("channels={channels}"),
                    evidence,
                });
            }

            let selector_dependency_hits = collect_identity_selector_call_hits(
                body,
                function.start_line,
            );
            if !selector_dependency_hits.is_empty() {
                let selectors = selector_dependency_hits
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(",");
                let (line, evidence) = first_hit_evidence(&selector_dependency_hits);
                findings.push(IdentityDebtFinding {
                    kind: "PROJECT_IDENTITY_DEPENDENCY".to_string(),
                    path: path.to_string(),
                    function: function.name.clone(),
                    line,
                    payload: format!("selectors={selectors}"),
                    evidence,
                });
            }

            let pitch_hits = collect_concrete_pitch_hits(
                body,
                function.start_line
            );
            if !pitch_hits.is_empty() {
                let pitches = pitch_hits
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(",");
                let (line, evidence) = first_hit_evidence(&pitch_hits);
                findings.push(IdentityDebtFinding {
                    kind: "CONCRETE_PITCH_ANCHOR".to_string(),
                    path: path.to_string(),
                    function: function.name.clone(),
                    line,
                    payload: format!("pitches={pitches}"),
                    evidence,
                });
            }
        }

        if is_selector
            && direct_project_hits.is_empty()
            && let Some(project) = identity_project_from_symbol(&function.name)
        {
            findings.push(IdentityDebtFinding {
                kind: "PROJECT_IDENTITY_SELECTOR".to_string(),
                path: path.to_string(),
                function: function.name.clone(),
                line: function.start_line,
                payload: format!("projects={project}"),
                evidence: declaration_evidence(body),
            });
        }
    }

    findings.sort();
    findings.dedup_by(|a, b| a.key() == b.key());
    IdentityDebtScanResult {
        source_files: 1,
        findings,
        allowed_localizations,
    }
}

fn declaration_evidence(body: &str) -> String {
    body.lines()
        .find(|line| line.contains("fn "))
        .unwrap_or("")
        .trim()
        .chars()
        .take(180)
        .collect()
}

fn first_hit_evidence(hits: &BTreeMap<String, (usize, String)>) -> (usize, String) {
    hits.values()
        .next()
        .cloned()
        .unwrap_or((0, String::new()))
}

fn count_localization_markers(body: &str) -> usize {
    let lower = body.to_lowercase();
    lower.matches("etüde nr.").count()
        + lower.matches("etude nr.").count()
        + lower.matches("target_measure").count()
        + lower.matches("measure_no").count()
}


fn has_direct_mutation_surface(body: &str) -> bool {
    body.contains("std::fs::write(")
        || body.contains("fs::write(")
        || body.contains(".replace_range(")
        || body.contains(".insert_str(")
        || body.contains(".remove_range(")
}

fn has_mutation_call_surface(body: &str) -> bool {
    if has_direct_mutation_surface(body) {
        return true;
    }
    called_identifiers(body).into_iter().any(|identifier| {
        let lower = identifier.to_ascii_lowercase();
        lower.contains("_authorized")
            || lower.starts_with("apply_")
            || lower.starts_with("patch_")
            || lower.starts_with("close_")
            || lower.starts_with("insert_")
            || lower.starts_with("rewrite_")
            || lower.starts_with("suppress_")
            || lower.starts_with("remove_")
            || lower.starts_with("restify_")
            || lower.starts_with("materialize_")
            || lower.starts_with("relocate_")
    })
}

fn feedback_origin_symbol_payload(name: &str) -> Option<String> {
    let lower = name.to_ascii_lowercase();
    for marker in ["feedback", "sister", "user_report", "reported_case", "requested_case"] {
        if lower.contains(marker) {
            return Some(format!("symbol={marker}"));
        }
    }
    None
}

fn collect_feedback_origin_channel_hits(
    body: &str,
    start_line: usize,
) -> BTreeMap<String, (usize, String)> {
    const CHANNELS: [&str; 12] = [
        "feedback_origin",
        "feedback_project",
        "feedback_measure",
        "user_feedback",
        "reported_project",
        "reported_measure",
        "requested_project",
        "requested_measure",
        "origin_project",
        "origin_measure",
        "feedback_location",
        "feedback_locator",
    ];
    let lower = body.to_ascii_lowercase();
    let mut hits = BTreeMap::new();
    for channel in CHANNELS {
        if let Some(offset) = lower.find(channel) {
            let relative_line = lower[..offset].bytes().filter(|byte| *byte == b'\n').count();
            let evidence = body
                .lines()
                .nth(relative_line)
                .unwrap_or("")
                .trim()
                .chars()
                .take(180)
                .collect();
            hits.insert(channel.to_string(), (start_line + relative_line, evidence));
        }
    }
    for identifier in called_identifiers(body) {
        if let Some(payload) = feedback_origin_symbol_payload(&identifier) {
            let line_index = body
                .lines()
                .position(|line| line.contains(&identifier))
                .unwrap_or(0);
            let evidence = body
                .lines()
                .nth(line_index)
                .unwrap_or("")
                .trim()
                .chars()
                .take(180)
                .collect();
            hits.entry(payload)
                .or_insert((start_line + line_index, evidence));
        }
    }
    hits
}

fn collect_identity_channel_predicate_hits(
    body: &str,
    start_line: usize,
) -> BTreeMap<String, (usize, String)> {
    const CHANNELS: [&str; 5] = [
        "project_label",
        "project_name",
        "source_label",
        "target_project",
        "reference_project",
    ];
    let mut hits = BTreeMap::new();
    for (line_index, raw) in body.lines().enumerate() {
        let lower = raw.to_ascii_lowercase();
        let predicate_shape = lower.contains("==")
            || lower.contains("!=")
            || lower.contains(".contains(")
            || lower.contains(".starts_with(")
            || lower.contains(".ends_with(")
            || lower.contains("matches!(")
            || lower.trim_start().starts_with("match ");
        if !predicate_shape {
            continue;
        }
        for channel in CHANNELS {
            if lower.contains(channel) {
                hits.entry(channel.to_string()).or_insert_with(|| {
                    (
                        start_line + line_index,
                        raw.trim().chars().take(180).collect(),
                    )
                });
            }
        }
    }
    hits
}

fn is_allowed_localization_function(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.contains("audit")
        || lower.contains("section")
        || lower.contains("summary")
        || lower.contains("pipeline_line")
        || lower.contains("no_mutation")
        || lower.contains("evidence")
        || lower.contains("readback")
        || lower.contains("verdict")
        || lower.starts_with("test_")
        || lower.starts_with("append_")
}

fn is_productive_mutation_function(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    if lower.contains("audit")
        || lower.contains("section")
        || lower.contains("summary")
        || lower.contains("pipeline_line")
        || lower.contains("no_mutation")
        || lower.contains("evidence")
        || lower.contains("readback")
        || lower.contains("verdict")
        || lower.starts_with("test_")
        || lower.starts_with("append_")
    {
        return false;
    }

    lower.starts_with("apply_")
        || lower.starts_with("patch_")
        || lower.starts_with("close_")
        || lower.starts_with("insert_")
        || lower.starts_with("rewrite_")
        || lower.starts_with("suppress_")
        || lower.starts_with("remove_")
        || lower.starts_with("restify_")
        || lower.starts_with("materialize_")
        || lower.starts_with("relocate_")
        || lower.starts_with("build_") && lower.contains("delta")
        || lower.contains("_insert_")
}

fn is_project_selector_function(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.contains("target_project")
        || lower.contains("reference_project")
        || lower.contains("scope") && identity_project_from_symbol(name).is_some()
}

fn identity_symbol_payload(name: &str) -> Option<String> {
    let lower = name.to_ascii_lowercase();
    for token in lower.split('_') {
        if let Some(number) = token.strip_prefix("etude")
            && is_positive_number(number)
        {
            return Some(format!("symbol=etude{number}"));
        }
        if let Some(number) = token.strip_prefix('m')
            && is_positive_number(number)
        {
            return Some(format!("symbol=m{number}"));
        }
        if let Some(number) = token.strip_prefix('t')
            && is_positive_number(number)
        {
            return Some(format!("symbol=t{number}"));
        }
    }
    None
}

fn identity_project_from_symbol(name: &str) -> Option<String> {
    let lower = name.to_ascii_lowercase();
    for token in lower.split('_') {
        if let Some(number) = token.strip_prefix("etude")
            && is_positive_number(number)
        {
            return Some(format!("etude{number}"));
        }
    }
    None
}

fn is_positive_number(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())
}

fn collect_absolute_measure_hits(
    body: &str,
    masked_body: &str,
    start_line: usize,
) -> BTreeMap<String, (usize, String)> {
    let mut hits = BTreeMap::new();
    let numeric_visible_mask = expose_numeric_string_literals(body, masked_body);

    for (offset, number) in literal_measure_locator_occurrences(&numeric_visible_mask) {
        let relative_line = masked_body[..offset]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count();
        let evidence = body
            .lines()
            .nth(relative_line)
            .unwrap_or("")
            .trim()
            .chars()
            .take(180)
            .collect();
        hits.entry(number.to_string())
            .or_insert((start_line + relative_line, evidence));
    }

    for (line_index, (raw, masked)) in body.lines().zip(numeric_visible_mask.lines()).enumerate() {
        let raw_trimmed = raw.trim();
        let masked_trimmed = masked.trim();
        if raw_trimmed.is_empty() {
            continue;
        }

        let line = start_line + line_index;
        let evidence: String = raw_trimmed.chars().take(180).collect();
        for number in identity_encoded_measure_values(masked_trimmed) {
            hits.entry(number.to_string())
                .or_insert_with(|| (line, evidence.clone()));
        }

        if !contains_identity_encoded_measure_name(masked_trimmed) {
            for number in explicit_measure_literal_values(masked_trimmed) {
                hits.entry(number.to_string())
                    .or_insert_with(|| (line, evidence.clone()));
            }
        }
    }

    hits
}

fn expose_numeric_string_literals(raw: &str, masked: &str) -> String {
    debug_assert_eq!(raw.len(), masked.len());
    let raw_bytes = raw.as_bytes();
    let mut out = masked.as_bytes().to_vec();
    let mut index = 0usize;
    while index < raw_bytes.len() {
        if raw_bytes[index] != b'"' {
            index += 1;
            continue;
        }
        let start = index + 1;
        let mut end = start;
        while end < raw_bytes.len() && raw_bytes[end] != b'"' && raw_bytes[end] != b'\n' {
            end += 1;
        }
        if end >= raw_bytes.len() || raw_bytes[end] != b'"' {
            index += 1;
            continue;
        }
        let content = &raw_bytes[start..end];
        let trimmed_start = content.iter().position(|byte| !byte.is_ascii_whitespace());
        let trimmed_end = content.iter().rposition(|byte| !byte.is_ascii_whitespace());
        if let (Some(first), Some(last)) = (trimmed_start, trimmed_end) {
            let token = &content[first..=last];
            if !token.is_empty() && token.iter().all(u8::is_ascii_digit) {
                let first_abs = start + first;
                let end_abs = start + last + 1;
                out[first_abs..end_abs].copy_from_slice(&raw_bytes[first_abs..end_abs]);
            }
        }
        index = end + 1;
    }
    String::from_utf8(out).expect("numeric string exposure preserves UTF-8")
}

fn explicit_measure_literal_values(line: &str) -> Vec<usize> {
    const MEASURE_IDENTIFIERS: [&str; 5] = [
        "target_measure",
        "return_measure",
        "measure_no",
        "measure_number",
        "note.measure",
    ];

    let lower = line.to_ascii_lowercase();
    let mut values = BTreeSet::new();

    for identifier in MEASURE_IDENTIFIERS {
        let mut scan = 0usize;
        while let Some(relative) = lower[scan..].find(identifier) {
            let start = scan + relative;
            let end = start + identifier.len();
            if identifier_boundary(&lower, start, end) {
                if let Some(value) = positive_literal_after_measure_operator(&lower[end..]) {
                    values.insert(value);
                }
                if let Some(value) = positive_literal_before_measure_comparison(&lower[..start]) {
                    values.insert(value);
                }
            }
            scan = end;
        }
    }

    values.extend(matches_measure_literal_values(&lower));
    values.into_iter().collect()
}

fn identifier_boundary(text: &str, start: usize, end: usize) -> bool {
    let bytes = text.as_bytes();
    let before_ok = start == 0
        || !(bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_');
    let after_ok = end >= bytes.len()
        || !(bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_');
    before_ok && after_ok
}

fn positive_literal_after_measure_operator(tail: &str) -> Option<usize> {
    let trimmed = tail.trim_start();
    let remainder = if let Some(rest) = trimmed.strip_prefix("==") {
        rest
    } else if let Some(rest) = trimmed.strip_prefix("!=") {
        rest
    } else if let Some(rest) = trimmed.strip_prefix('=') {
        rest
    } else {
        trimmed.strip_prefix(':')?
    };
    positive_integer_prefix(remainder)
}

fn positive_literal_before_measure_comparison(prefix: &str) -> Option<usize> {
    let trimmed = prefix.trim_end();
    let without_operator = trimmed
        .strip_suffix("==")
        .or_else(|| trimmed.strip_suffix("!="))?
        .trim_end();
    let digits_start = without_operator
        .rfind(|ch: char| !ch.is_ascii_digit())
        .map_or(0, |index| index + 1);
    let digits = &without_operator[digits_start..];
    if digits_start > 0 {
        let preceding = without_operator.as_bytes()[digits_start - 1];
        if preceding.is_ascii_alphanumeric() || preceding == b'_' {
            return None;
        }
    }
    positive_integer_argument(digits)
}

fn positive_integer_prefix(text: &str) -> Option<usize> {
    let trimmed = text.trim_start();
    let digits: String = trimmed
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    if digits.is_empty() {
        return None;
    }
    let next = trimmed.as_bytes().get(digits.len()).copied();
    if matches!(next, Some(byte) if byte.is_ascii_alphanumeric() || byte == b'_') {
        return None;
    }
    positive_integer_argument(&digits)
}

fn matches_measure_literal_values(line: &str) -> Vec<usize> {
    let Some(macro_start) = line.find("matches!(") else {
        return Vec::new();
    };
    let open = macro_start + "matches!".len();
    let Some(close) = matching_delimiter(line, open, b'(', b')') else {
        return Vec::new();
    };
    let arguments = split_top_level_arguments(&line[open + 1..close]);
    let Some(subject) = arguments.first() else {
        return Vec::new();
    };
    let subject_has_measure_identity = [
        "target_measure",
        "return_measure",
        "measure_no",
        "measure_number",
        "note.measure",
    ]
    .iter()
    .any(|identifier| subject.contains(identifier));
    if !subject_has_measure_identity {
        return Vec::new();
    }
    arguments
        .iter()
        .skip(1)
        .flat_map(|argument| numeric_literals(argument))
        .filter(|value| *value > 0)
        .collect()
}

fn contains_identity_encoded_measure_name(line: &str) -> bool {
    !identity_encoded_measure_values(line).is_empty()
}

fn identity_encoded_measure_values(line: &str) -> Vec<usize> {
    const NUMBER_WORDS: [(&str, usize); 20] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
        ("eleven", 11),
        ("twelve", 12),
        ("thirteen", 13),
        ("fourteen", 14),
        ("fifteen", 15),
        ("sixteen", 16),
        ("seventeen", 17),
        ("eighteen", 18),
        ("nineteen", 19),
        ("twenty", 20),
    ];
    let lower = line.to_ascii_lowercase();
    let mut values = BTreeSet::new();
    for (word, value) in NUMBER_WORDS {
        if lower.contains(&format!("measure_{word}_")) {
            values.insert(value);
        }
    }
    for token in lower.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_')) {
        let Some(tail) = token.strip_prefix("measure_") else {
            continue;
        };
        let Some(number) = tail.split('_').next() else {
            continue;
        };
        if is_positive_number(number)
            && let Ok(value) = number.parse::<usize>()
            && value > 0
        {
            values.insert(value);
        }
    }
    values.into_iter().collect()
}

fn literal_measure_locator_occurrences(masked: &str) -> Vec<(usize, usize)> {
    let bytes = masked.as_bytes();
    let mut values = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if !(bytes[index].is_ascii_alphabetic() || bytes[index] == b'_') {
            index += 1;
            continue;
        }
        let name_start = index;
        index += 1;
        while index < bytes.len()
            && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
        {
            index += 1;
        }
        let name = masked[name_start..index].to_ascii_lowercase();
        let mut cursor = index;
        while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if bytes.get(cursor) != Some(&b'(') || !is_measure_locator_function(&name) {
            continue;
        }
        let Some(close) = matching_delimiter(masked, cursor, b'(', b')') else {
            continue;
        };
        let arguments = split_top_level_arguments(&masked[cursor + 1..close]);
        let measure_argument = if arguments
            .first()
            .and_then(|argument| positive_integer_argument(argument))
            .is_some()
        {
            arguments.first()
        } else {
            arguments.get(1)
        };
        if let Some(value) = measure_argument.and_then(|argument| positive_integer_argument(argument)) {
            values.push((name_start, value));
        }
        index = close + 1;
    }
    values
}

fn is_measure_locator_function(name: &str) -> bool {
    name.contains("_for_measure")
        || name.contains("measure_by_number")
        || name.contains("measure_span_by_number")
        || name.contains("measure_number_bounds")
        || name.contains("measure_bounds_by_number")
        || name == "key_signature_label"
        || name == "key_signature_measure_fifths"
        || name == "carried_key_signature_fifths"
}

fn matching_delimiter(
    text: &str,
    open_index: usize,
    open: u8,
    close: u8,
) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut depth = 0usize;
    for (index, byte) in bytes.iter().enumerate().skip(open_index) {
        if *byte == open {
            depth += 1;
        } else if *byte == close {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(index);
            }
        }
    }
    None
}

fn split_top_level_arguments(arguments: &str) -> Vec<&str> {
    let bytes = arguments.as_bytes();
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;
    for (index, byte) in bytes.iter().enumerate() {
        match *byte {
            b'(' => paren_depth += 1,
            b')' => paren_depth = paren_depth.saturating_sub(1),
            b'[' => bracket_depth += 1,
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'{' => brace_depth += 1,
            b'}' => brace_depth = brace_depth.saturating_sub(1),
            b',' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                parts.push(arguments[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }
    parts.push(arguments[start..].trim());
    parts
}

fn positive_integer_argument(argument: &str) -> Option<usize> {
    let trimmed = argument.trim();
    if !is_positive_number(trimmed) {
        return None;
    }
    trimmed.parse::<usize>().ok().filter(|value| *value > 0)
}

fn collect_project_identity_constants(source: &str) -> BTreeMap<String, String> {
    let mut constants = BTreeMap::new();
    for raw in source.lines() {
        let trimmed = raw.trim();
        let Some(const_tail) = trimmed
            .strip_prefix("const ")
            .or_else(|| trimmed.strip_prefix("pub(crate) const "))
            .or_else(|| trimmed.strip_prefix("pub const "))
        else {
            continue;
        };
        let Some((identifier, _)) = const_tail.split_once(':') else {
            continue;
        };
        let projects = project_literals(&raw.to_lowercase());
        if projects.len() == 1 {
            constants.insert(
                identifier.trim().to_string(),
                projects.into_iter().next().unwrap_or_default(),
            );
        }
    }
    constants
}

fn collect_project_identity_constant_hits(
    body: &str,
    start_line: usize,
    constants: &BTreeMap<String, String>,
) -> BTreeMap<String, (usize, String)> {
    let mut hits = BTreeMap::new();
    for (line_index, raw) in body.lines().enumerate() {
        let lower = raw.to_ascii_lowercase();
        let predicate_shape = lower.contains("==")
            || lower.contains("!=")
            || lower.contains("matches!(")
            || lower.contains(".contains(")
            || lower.contains(".starts_with(")
            || lower.contains(".ends_with(")
            || lower.trim_start().starts_with("match ");
        if !predicate_shape {
            continue;
        }
        for (identifier, project) in constants {
            if contains_identifier(raw, identifier) {
                hits.entry(project.clone()).or_insert_with(|| {
                    (
                        start_line + line_index,
                        raw.trim().chars().take(180).collect(),
                    )
                });
            }
        }
    }
    hits
}

fn contains_identifier(line: &str, identifier: &str) -> bool {
    let bytes = line.as_bytes();
    let needle = identifier.as_bytes();
    if needle.is_empty() || needle.len() > bytes.len() {
        return false;
    }
    for start in 0..=bytes.len() - needle.len() {
        if &bytes[start..start + needle.len()] != needle {
            continue;
        }
        let before_ok = start == 0
            || !(bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_');
        let end = start + needle.len();
        let after_ok = end == bytes.len()
            || !(bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_');
        if before_ok && after_ok {
            return true;
        }
    }
    false
}

fn collect_project_identity_hits(
    body: &str,
    start_line: usize,
) -> BTreeMap<String, (usize, String)> {
    let mut hits = BTreeMap::new();
    for (line_index, raw) in body.lines().enumerate() {
        let lower = raw.to_lowercase();
        let selector_shape = lower.contains("==")
            || lower.contains("!=")
            || lower.contains("contains(")
            || lower.contains("starts_with(")
            || lower.contains("target_project(")
            || lower.contains("reference_project(");
        if !selector_shape {
            continue;
        }

        for project in project_literals(&lower) {
            hits.entry(project).or_insert_with(|| {
                (
                    start_line + line_index,
                    raw.trim().chars().take(180).collect(),
                )
            });
        }
    }
    hits
}

fn project_literals(lower: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for marker in ["etuedenr", "etudenr", "etüde nr. ", "etude nr. "] {
        let mut scan = 0usize;
        while let Some(relative) = lower[scan..].find(marker) {
            let start = scan + relative + marker.len();
            let digits: String = lower[start..]
                .chars()
                .take_while(char::is_ascii_digit)
                .collect();
            if !digits.is_empty() {
                out.insert(format!("etude{digits}"));
            }
            scan = start.saturating_add(digits.len()).min(lower.len());
            if scan >= lower.len() {
                break;
            }
        }
    }
    out
}

fn collect_identity_selector_call_hits(
    body: &str,
    start_line: usize,
) -> BTreeMap<String, (usize, String)> {
    let mut hits = BTreeMap::new();
    for (line_index, raw) in body.lines().enumerate() {
        for identifier in called_identifiers(raw) {
            let lower = identifier.to_ascii_lowercase();
            let selector_shape = lower.contains("target_project")
                || lower.contains("reference_project")
                || lower.contains("scope") && identity_project_from_symbol(&identifier).is_some();
            if selector_shape {
                hits.entry(identifier).or_insert_with(|| {
                    (
                        start_line + line_index,
                        raw.trim().chars().take(180).collect(),
                    )
                });
            }
        }
    }
    hits
}

fn called_identifiers(raw: &str) -> Vec<String> {
    let bytes = raw.as_bytes();
    let mut out = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if !(bytes[index].is_ascii_alphabetic() || bytes[index] == b'_') {
            index += 1;
            continue;
        }
        let start = index;
        index += 1;
        while index < bytes.len()
            && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
        {
            index += 1;
        }
        let mut cursor = index;
        while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if bytes.get(cursor) == Some(&b'(') {
            let identifier = &raw[start..index];
            if identifier != "fn" {
                out.push(identifier.to_string());
            }
        }
    }
    out
}

fn collect_concrete_pitch_hits(
    body: &str,
    start_line: usize,
) -> BTreeMap<String, (usize, String)> {
    let mut hits = BTreeMap::new();
    for (line_index, raw) in body.lines().enumerate() {
        let lower = raw.to_ascii_lowercase();
        let pitch_shape = lower.contains("previous_anchor:")
            || lower.contains("new_anchor:")
            || lower.contains("apply_standard_rest_display_pitch(");
        if !pitch_shape {
            continue;
        }
        for pitch in pitch_literals(raw) {
            hits.entry(pitch).or_insert_with(|| {
                (
                    start_line + line_index,
                    raw.trim().chars().take(180).collect(),
                )
            });
        }
    }
    hits
}

fn pitch_literals(raw: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let bytes = raw.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] != b'"' {
            index += 1;
            continue;
        }
        let start = index + 1;
        let Some(relative_end) = raw[start..].find('"') else {
            break;
        };
        let end = start + relative_end;
        let value = &raw[start..end];
        if is_pitch_token(value) {
            out.insert(value.to_string());
        }
        index = end + 1;
    }
    out
}

fn is_pitch_token(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(step) = chars.next() else {
        return false;
    };
    if !matches!(step, 'A'..='G') {
        return false;
    }
    let rest: String = chars.collect();
    if rest.is_empty() {
        return true;
    }
    let rest = rest.strip_prefix('#').or_else(|| rest.strip_prefix('b')).unwrap_or(&rest);
    !rest.is_empty() && rest.chars().all(|ch| ch.is_ascii_digit())
}

fn numeric_literals(line: &str) -> Vec<usize> {
    let mut values = Vec::new();
    let bytes = line.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if !bytes[index].is_ascii_digit() {
            index += 1;
            continue;
        }
        if index > 0 && (bytes[index - 1].is_ascii_alphanumeric() || bytes[index - 1] == b'_') {
            index += 1;
            continue;
        }
        let start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        if index < bytes.len() && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_') {
            continue;
        }
        if let Ok(value) = line[start..index].parse::<usize>() {
            values.push(value);
        }
    }
    values
}

fn function_spans(masked: &str, starts: &[usize], test_spans: &[(usize, usize)]) -> Vec<FunctionSpan> {
    let bytes = masked.as_bytes();
    let mut spans = Vec::new();
    let mut index = 0usize;
    while index + 2 <= bytes.len() {
        if &bytes[index..index + 2] != b"fn"
            || !token_boundary_before(bytes, index)
            || !token_boundary_after(bytes, index + 2)
        {
            index += 1;
            continue;
        }

        let mut cursor = index + 2;
        while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        let name_start = cursor;
        while cursor < bytes.len()
            && (bytes[cursor].is_ascii_alphanumeric() || bytes[cursor] == b'_')
        {
            cursor += 1;
        }
        if cursor == name_start {
            index += 2;
            continue;
        }
        let name = masked[name_start..cursor].to_string();
        let Some(open) = find_function_open_brace(masked, cursor) else {
            index = cursor;
            continue;
        };
        let Some(close) = matching_brace(masked, open) else {
            index = open + 1;
            continue;
        };
        let start = index;
        let end = close + 1;
        let test_only = test_spans
            .iter()
            .any(|(test_start, test_end)| start >= *test_start && end <= *test_end)
            || has_test_attribute_before(masked, start);
        spans.push(FunctionSpan {
            name,
            start,
            end,
            start_line: line_number_at(start, starts),
            test_only,
        });
        index = end;
    }
    spans
}

fn test_only_spans(masked: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut scan = 0usize;
    while let Some(relative) = masked[scan..].find("#[cfg(test)]") {
        let attr_start = scan + relative;
        let after = attr_start + "#[cfg(test)]".len();
        let Some(mod_relative) = masked[after..].find("mod") else {
            break;
        };
        let mod_start = after + mod_relative;
        let Some(open_relative) = masked[mod_start..].find('{') else {
            break;
        };
        let open = mod_start + open_relative;
        if let Some(close) = matching_brace(masked, open) {
            spans.push((attr_start, close + 1));
            scan = close + 1;
        } else {
            break;
        }
    }
    spans
}

fn has_test_attribute_before(masked: &str, function_start: usize) -> bool {
    let prefix_start = masked[..function_start]
        .rfind('\n')
        .map_or(0, |index| index.saturating_sub(256));
    let prefix = &masked[prefix_start..function_start];
    prefix.contains("#[test]")
}

fn find_function_open_brace(masked: &str, start: usize) -> Option<usize> {
    let bytes = masked.as_bytes();
    let mut index = start;
    while index < bytes.len() {
        match bytes[index] {
            b'{' => return Some(index),
            b';' => return None,
            _ => index += 1,
        }
    }
    None
}

fn matching_brace(masked: &str, open: usize) -> Option<usize> {
    let bytes = masked.as_bytes();
    if bytes.get(open) != Some(&b'{') {
        return None;
    }
    let mut depth = 0usize;
    for (index, byte) in bytes.iter().enumerate().skip(open) {
        match *byte {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod identity_debt_regression_tests {
    use super::*;

    #[test]
    fn literal_measure_locator_call_is_identity_debt() {
        let source = r"
            fn apply_unsafe(xml: &str) -> String {
                rewrite_key_signature_for_measure(xml, 1, 5)
            }
        ";
        let scan = scan_source_text("src/example.rs", source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "ABSOLUTE_MEASURE_SELECTOR"
                && finding.payload == "measures=1"
        }));
    }

    #[test]
    fn literal_read_locator_inside_writer_is_identity_debt() {
        let source = r"
            fn apply_unsafe(xml: &str) -> String {
                let observed = key_signature_label(xml, 1);
                observed
            }
        ";
        let scan = scan_source_text("src/example.rs", source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "ABSOLUTE_MEASURE_SELECTOR"
                && finding.payload == "measures=1"
        }));
    }

    #[test]
    fn multiline_literal_measure_locator_call_is_identity_debt() {
        let source = r"
            fn apply_unsafe(xml: &str) -> String {
                rewrite_key_signature_for_measure(
                    xml,
                    1,
                    5,
                )
            }
        ";
        let scan = scan_source_text("src/example.rs", source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "ABSOLUTE_MEASURE_SELECTOR"
                && finding.payload == "measures=1"
        }));
    }

    #[test]
    fn language_encoded_measure_name_is_not_whitelisted() {
        let source = r"
            fn apply_unsafe(xml: &str) -> String {
                let measure_six_key = xml;
                measure_six_key.to_string()
            }
        ";
        let scan = scan_source_text("src/example.rs", source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "ABSOLUTE_MEASURE_SELECTOR"
                && finding.payload.contains('6')
        }));
    }


    #[test]
    fn explicit_first_measure_assignment_is_identity_debt() {
        let source = format!(
            r"
            fn apply_unsafe(xml: &str) -> String {{
                let target_measure = {};
                rewrite_key_signature_for_measure(xml, target_measure, 5)
            }}
        ",
            1
        );
        let scan = scan_source_text("src/example.rs", &source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "ABSOLUTE_MEASURE_SELECTOR"
                && finding.payload == "measures=1"
        }));
    }

    #[test]
    fn unrelated_count_comparison_on_measure_named_line_is_not_identity_debt() {
        let source = r"
            fn true_pitch_only_mutation_proof(target_measure_signature_equal: bool, changed_pitch_blocks: usize) -> bool {
                target_measure_signature_equal && changed_pitch_blocks == 1
            }
        ";
        let scan = scan_source_text("src/example.rs", source);
        assert!(scan.findings.is_empty());
    }

    #[test]
    fn explicit_measure_comparison_is_identity_debt() {
        let source = format!(
            r"
            fn apply_unsafe(measure_number: usize) -> bool {{
                measure_number == {}
            }}
        ",
            6
        );
        let scan = scan_source_text("src/example.rs", &source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "ABSOLUTE_MEASURE_SELECTOR"
                && finding.payload == "measures=6"
        }));
    }

    #[test]
    fn reverse_explicit_measure_comparison_is_identity_debt() {
        let source = r"
            fn apply_unsafe(measure_number: usize) -> bool {
                6 == measure_number
            }
        ";
        let scan = scan_source_text("src/example.rs", source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "ABSOLUTE_MEASURE_SELECTOR"
                && finding.payload == "measures=6"
        }));
    }

    #[test]
    fn matches_measure_literals_are_identity_debt() {
        let source = r"
            fn apply_unsafe(measure_number: usize) -> bool {
                matches!(measure_number, 1 | 6)
            }
        ";
        let scan = scan_source_text("src/example.rs", source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "ABSOLUTE_MEASURE_SELECTOR"
                && finding.payload == "measures=1,6"
        }));
    }

    #[test]
    fn project_identity_hidden_behind_constant_is_detected() {
        let source = format!(
            r#"
            const TARGET_PROJECT: &str = "Etüde Nr. {}";
            fn is_target_project(project: &str) -> bool {{
                project == TARGET_PROJECT
            }}
        "#,
            6
        );
        let scan = scan_source_text("src/example.rs", &source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "PROJECT_IDENTITY_SELECTOR"
                && finding.payload == format!("projects=etude{}", 6)
        }));
    }

    #[test]
    fn dynamic_measure_locator_call_remains_allowed() {
        let source = r"
            fn apply_structural(xml: &str, measure: usize) -> String {
                rewrite_key_signature_for_measure(xml, measure, 5)
            }
        ";
        let scan = scan_source_text("src/example.rs", source);
        assert!(scan.findings.is_empty());
    }
}
