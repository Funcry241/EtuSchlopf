#[cfg(not(test))]
use super::rust_source_scan_primitives::{
    collect_rust_files, fnv1a64_update, line_number_at, line_starts, mask_non_code,
    normalize_path, token_boundary_after, token_boundary_before, FNV1A64_OFFSET_BASIS,
};
#[cfg(test)]
use crate::identity_debt_inventory::rust_source_scan_primitives::{
    collect_rust_files, fnv1a64_update, line_number_at, line_starts, mask_non_code,
    normalize_path, token_boundary_after, token_boundary_before, FNV1A64_OFFSET_BASIS,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ProductiveWriteSurface {
    pub path: String,
    pub function: String,
    pub line: usize,
    pub xml_buffer_mutations: usize,
    pub xml_file_writes: usize,
    pub direct_from_real_main: bool,
    pub authority_boundary: bool,
    pub class: String,
    pub evidence: String,
}

#[derive(Debug, Clone, Default)]
pub struct ProductiveWritePathScanResult {
    pub source_files: usize,
    pub functions_scanned: usize,
    pub surfaces: Vec<ProductiveWriteSurface>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProductiveWriteBypassBaselineRow {
    pub path: String,
    pub function: String,
    pub max_xml_buffer_mutations: usize,
    pub max_xml_file_writes: usize,
    pub direct_from_real_main_allowed: bool,
    pub owner: String,
    pub decision: String,
}

impl ProductiveWriteBypassBaselineRow {
    fn key(&self) -> String {
        format!("{}::{}", self.path, self.function)
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ProductiveWriteBypassRegisterValidation {
    pub baseline_external_surfaces: usize,
    pub current_external_surfaces: usize,
    pub retired_external_surfaces: usize,
    pub unknown_external_surfaces: Vec<String>,
    pub operation_escalations: Vec<String>,
    pub direct_main_escalations: Vec<String>,
    pub duplicate_current_external_surfaces: Vec<String>,
}

impl ProductiveWriteBypassRegisterValidation {
    pub fn is_green(&self) -> bool {
        self.unknown_external_surfaces.is_empty()
            && self.operation_escalations.is_empty()
            && self.direct_main_escalations.is_empty()
            && self.duplicate_current_external_surfaces.is_empty()
            && self.current_external_surfaces <= self.baseline_external_surfaces
    }
}

pub fn parse_productive_write_bypass_baseline(
    text: &str,
) -> Result<Vec<ProductiveWriteBypassBaselineRow>, String> {
    let mut rows = Vec::new();
    let mut keys = BTreeSet::new();
    for (line_index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let columns: Vec<&str> = line.split('\t').collect();
        if columns.len() != 7 {
            return Err(format!(
                "line {} expected 7 tab-separated columns",
                line_index + 1
            ));
        }
        let path = columns[0].trim().to_string();
        let function = columns[1].trim().to_string();
        let max_xml_buffer_mutations = columns[2].trim().parse::<usize>().map_err(|error| {
            format!("line {} invalid buffer budget: {error}", line_index + 1)
        })?;
        let max_xml_file_writes = columns[3].trim().parse::<usize>().map_err(|error| {
            format!("line {} invalid file-write budget: {error}", line_index + 1)
        })?;
        let direct_from_real_main_allowed = match columns[4].trim() {
            "ja" => true,
            "nein" => false,
            value => {
                return Err(format!(
                    "line {} invalid direct_from_real_main_allowed value {value:?}",
                    line_index + 1
                ));
            }
        };
        let owner = columns[5].trim().to_string();
        let decision = columns[6].trim().to_string();
        if path.is_empty() || function.is_empty() || owner.is_empty() || decision.is_empty() {
            return Err(format!("line {} contains an empty required field", line_index + 1));
        }
        if !path.starts_with("src/") {
            return Err(format!(
                "line {} path must be rooted below src/: {path}",
                line_index + 1
            ));
        }
        if max_xml_buffer_mutations == 0 && max_xml_file_writes == 0 {
            return Err(format!(
                "line {} has no registered write operation",
                line_index + 1
            ));
        }
        let row = ProductiveWriteBypassBaselineRow {
            path,
            function,
            max_xml_buffer_mutations,
            max_xml_file_writes,
            direct_from_real_main_allowed,
            owner,
            decision,
        };
        let key = row.key();
        if !keys.insert(key.clone()) {
            return Err(format!("line {} duplicate baseline key {key}", line_index + 1));
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return Err("productive write bypass baseline is empty".to_string());
    }
    Ok(rows)
}

pub fn productive_write_bypass_baseline_fingerprint(text: &str) -> u64 {
    let mut hash = FNV1A64_OFFSET_BASIS;
    for line in text.lines() {
        hash = fnv1a64_update(hash, line.trim_end().as_bytes());
        hash = fnv1a64_update(hash, b"\n");
    }
    hash
}

pub fn validate_productive_write_bypass_register(
    surfaces: &[ProductiveWriteSurface],
    baseline: &[ProductiveWriteBypassBaselineRow],
) -> ProductiveWriteBypassRegisterValidation {
    let baseline_by_key: BTreeMap<String, &ProductiveWriteBypassBaselineRow> = baseline
        .iter()
        .map(|row| (row.key(), row))
        .collect();
    let mut current_keys = BTreeSet::new();
    let mut validation = ProductiveWriteBypassRegisterValidation {
        baseline_external_surfaces: baseline.len(),
        ..ProductiveWriteBypassRegisterValidation::default()
    };

    for surface in surfaces.iter().filter(|surface| !surface.authority_boundary) {
        validation.current_external_surfaces += 1;
        let key = format!("{}::{}", surface.path, surface.function);
        if !current_keys.insert(key.clone()) {
            validation
                .duplicate_current_external_surfaces
                .push(key.clone());
            continue;
        }
        let Some(row) = baseline_by_key.get(&key) else {
            validation.unknown_external_surfaces.push(key);
            continue;
        };
        if surface.xml_buffer_mutations > row.max_xml_buffer_mutations
            || surface.xml_file_writes > row.max_xml_file_writes
        {
            validation.operation_escalations.push(format!(
                "{} buffer {}/{} file {}/{}",
                key,
                surface.xml_buffer_mutations,
                row.max_xml_buffer_mutations,
                surface.xml_file_writes,
                row.max_xml_file_writes
            ));
        }
        if surface.direct_from_real_main && !row.direct_from_real_main_allowed {
            validation.direct_main_escalations.push(key);
        }
    }

    validation.retired_external_surfaces = baseline_by_key
        .keys()
        .filter(|key| !current_keys.contains(*key))
        .count();
    validation
}

#[derive(Debug, Clone)]
struct FunctionSpan {
    path: String,
    name: String,
    start: usize,
    end: usize,
    start_line: usize,
    test_only: bool,
}

pub fn scan_productive_write_paths_roots(
    project_root: &Path,
    source_roots: &[&Path],
) -> io::Result<ProductiveWritePathScanResult> {
    let mut files = Vec::new();
    for source_root in source_roots {
        collect_rust_files(source_root, &mut files)?;
    }
    files.sort();
    files.dedup();

    let mut functions = Vec::new();
    let mut sources = BTreeMap::new();
    for path in &files {
        let source = fs::read_to_string(path)?;
        let relative = normalize_path(path.strip_prefix(project_root).unwrap_or(path));
        functions.extend(function_spans_for_source(&relative, &source));
        sources.insert(relative, source);
    }

    let direct_real_main_calls = functions
        .iter()
        .find(|function| {
            function.name == "real_main"
                && function
                    .path
                    .ends_with("pipeline_shards/pipeline_orchestrator_report_compaction.rs")
        })
        .and_then(|function| sources.get(&function.path).map(|source| (function, source)))
        .map(|(function, source)| {
            let body = &source[function.start..function.end];
            called_identifiers(&mask_non_code(body))
                .into_iter()
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();

    let mut surfaces = Vec::new();
    for function in &functions {
        if function.test_only {
            continue;
        }
        let source = sources.get(&function.path).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, format!("missing source {}", function.path))
        })?;
        let body = &source[function.start..function.end];
        let masked = mask_non_code(body);
        let xml_buffer_mutations = count_xml_buffer_mutations(&function.name, body, &masked);
        let xml_file_writes = count_xml_file_writes(&function.name, body, &masked);
        if xml_buffer_mutations == 0 && xml_file_writes == 0 {
            continue;
        }

        let authority_boundary = function.path.starts_with("src/structural_write_authority/");
        let class = if authority_boundary {
            if xml_file_writes > 0 {
                "CENTRAL_AUTHORITY_XML_FILE_SURFACE"
            } else {
                "CENTRAL_AUTHORITY_XML_BUFFER_SURFACE"
            }
        } else if xml_buffer_mutations > 0 && xml_file_writes > 0 {
            "LEGACY_XML_BUFFER_AND_FILE_SURFACE"
        } else if xml_file_writes > 0 {
            "LEGACY_XML_FILE_SINK"
        } else {
            "LEGACY_XML_BUFFER_MUTATOR"
        };

        surfaces.push(ProductiveWriteSurface {
            path: function.path.clone(),
            function: function.name.clone(),
            line: function.start_line,
            xml_buffer_mutations,
            xml_file_writes,
            direct_from_real_main: function.name == "real_main"
                || direct_real_main_calls.contains(&function.name),
            authority_boundary,
            class: class.to_string(),
            evidence: first_write_evidence(&function.name, body, &masked),
        });
    }

    surfaces.sort();
    surfaces.dedup_by(|left, right| {
        left.path == right.path && left.function == right.function && left.line == right.line
    });

    Ok(ProductiveWritePathScanResult {
        source_files: files.len(),
        functions_scanned: functions.len(),
        surfaces,
    })
}

pub fn scan_productive_write_paths_text(path: &str, source: &str) -> ProductiveWritePathScanResult {
    let functions = function_spans_for_source(path, source);
    let direct_real_main_calls = functions
        .iter()
        .find(|function| function.name == "real_main")
        .map(|function| {
            called_identifiers(&mask_non_code(&source[function.start..function.end]))
                .into_iter()
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let mut surfaces = Vec::new();
    for function in &functions {
        if function.test_only {
            continue;
        }
        let body = &source[function.start..function.end];
        let masked = mask_non_code(body);
        let xml_buffer_mutations = count_xml_buffer_mutations(&function.name, body, &masked);
        let xml_file_writes = count_xml_file_writes(&function.name, body, &masked);
        if xml_buffer_mutations == 0 && xml_file_writes == 0 {
            continue;
        }
        let authority_boundary = path.starts_with("src/structural_write_authority/");
        let class = if authority_boundary {
            if xml_file_writes > 0 {
                "CENTRAL_AUTHORITY_XML_FILE_SURFACE"
            } else {
                "CENTRAL_AUTHORITY_XML_BUFFER_SURFACE"
            }
        } else if xml_buffer_mutations > 0 && xml_file_writes > 0 {
            "LEGACY_XML_BUFFER_AND_FILE_SURFACE"
        } else if xml_file_writes > 0 {
            "LEGACY_XML_FILE_SINK"
        } else {
            "LEGACY_XML_BUFFER_MUTATOR"
        };
        surfaces.push(ProductiveWriteSurface {
            path: path.to_string(),
            function: function.name.clone(),
            line: function.start_line,
            xml_buffer_mutations,
            xml_file_writes,
            direct_from_real_main: function.name == "real_main"
                || direct_real_main_calls.contains(&function.name),
            authority_boundary,
            class: class.to_string(),
            evidence: first_write_evidence(&function.name, body, &masked),
        });
    }
    surfaces.sort();
    ProductiveWritePathScanResult {
        source_files: 1,
        functions_scanned: functions.len(),
        surfaces,
    }
}

fn count_xml_buffer_mutations(function_name: &str, body: &str, masked: &str) -> usize {
    if is_non_output_xml_helper(function_name) {
        return 0;
    }
    xml_buffer_mutation_positions(function_name, body, masked).len()
}

fn xml_buffer_mutation_positions(function_name: &str, body: &str, masked: &str) -> Vec<usize> {
    let reportish = is_report_only_name(function_name);
    let lower = body.to_ascii_lowercase();
    let mut positions = Vec::new();
    for operation in ["replace_range", "insert_str"] {
        let needle = format!(".{operation}(");
        let mut cursor = 0usize;
        while let Some(relative) = masked[cursor..].find(&needle) {
            let operation_start = cursor + relative;
            let receiver = receiver_before(masked, operation_start);
            let receiver_lower = receiver.to_ascii_lowercase();
            let explicit_xml_receiver = [
                "xml", "candidate", "render", "music", "note", "block", "fragment",
                "tag", "rewritten", "restified", "cleaned", "start_tag",
            ]
            .iter()
            .any(|token| receiver_lower.contains(token));
            let generic_output_receiver = matches!(receiver_lower.as_str(), "out" | "output")
                && contains_music_xml_domain(&lower)
                && (!reportish || function_signature_has_xml_input(body));
            if explicit_xml_receiver || generic_output_receiver {
                positions.push(operation_start);
            }
            cursor = operation_start + needle.len();
        }
    }
    positions.sort_unstable();
    positions
}

fn count_xml_file_writes(function_name: &str, body: &str, masked: &str) -> usize {
    xml_file_write_ranges(function_name, body, masked).len()
}

fn xml_file_write_ranges(
    _function_name: &str,
    body: &str,
    masked: &str,
) -> Vec<(usize, usize)> {
    let mut starts = BTreeSet::new();
    for needle in ["std::fs::write(", "fs::write("] {
        let mut cursor = 0usize;
        while let Some(relative) = masked[cursor..].find(needle) {
            let start = cursor + relative;
            if needle == "fs::write("
                && start >= "std::".len()
                && &masked[start - "std::".len()..start] == "std::"
            {
                cursor = start + needle.len();
                continue;
            }
            starts.insert(start);
            cursor = start + needle.len();
        }
    }

    let mut ranges = Vec::new();
    for start in starts {
        let Some(open_relative) = masked[start..].find('(') else {
            continue;
        };
        let open = start + open_relative;
        let Some(close) = matching_parenthesis(masked, open) else {
            continue;
        };
        let call = body[start..=close].to_ascii_lowercase();
        let explicit_xml_context = [
            "xml", "musicxml", "render_xml", "candidate_xml", "delta.xml", "musicxml_out",
        ]
        .iter()
        .any(|token| call.contains(token));
        if explicit_xml_context {
            ranges.push((start, close));
        }
    }
    ranges
}

fn matching_parenthesis(masked: &str, open: usize) -> Option<usize> {
    let bytes = masked.as_bytes();
    if bytes.get(open) != Some(&b'(') {
        return None;
    }
    let mut depth = 0usize;
    for (index, byte) in bytes.iter().enumerate().skip(open) {
        match *byte {
            b'(' => depth += 1,
            b')' => {
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

fn function_signature_has_xml_input(body: &str) -> bool {
    let signature = body.split_once('{').map_or(body, |(signature, _)| signature);
    let lower = signature.to_ascii_lowercase();
    lower.contains("xml:")
        || lower.contains("xml :")
        || lower.contains("musicxml")
        || lower.contains("render_xml")
}

fn contains_music_xml_domain(lower: &str) -> bool {
    [
        "xml", "musicxml", "render", "measure", "note", "rest", "beam", "clef",
        "staff", "voice", "octave", "accent", "tenuto", "staccato", "slur", "pitch",
        "key_signature", "barline", "direction", "articulation", "layout", "chord", "tie",
    ]
    .iter()
    .any(|token| lower.contains(token))
}


fn is_non_output_xml_helper(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.starts_with("parse_") || lower == "strip_doctype"
}

fn is_report_only_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    [
        "append_", "audit", "section", "summary", "report", "pipeline_line", "sanitize",
        "consolidate", "format_", "render_generated", "manifest", "context", "catalog",
        "console", "log",
    ]
    .iter()
    .any(|token| lower.contains(token))
}

fn receiver_before(masked: &str, dot_start: usize) -> String {
    let bytes = masked.as_bytes();
    let mut end = dot_start;
    while end > 0 && bytes[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    let mut start = end;
    while start > 0
        && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_')
    {
        start -= 1;
    }
    masked[start..end].to_string()
}

fn first_write_evidence(function_name: &str, body: &str, masked: &str) -> String {
    let buffer_positions = xml_buffer_mutation_positions(function_name, body, masked);
    let file_ranges = xml_file_write_ranges(function_name, body, masked);
    let first_buffer = buffer_positions.first().copied();
    let first_file = file_ranges.first().copied();
    match (first_buffer, first_file) {
        (Some(buffer), Some((file, close))) if file < buffer => {
            compact_evidence(&body[file..=close])
        }
        (Some(buffer), _) => line_evidence_at(body, buffer),
        (None, Some((file, close))) => compact_evidence(&body[file..=close]),
        (None, None) => String::new(),
    }
}

fn line_evidence_at(body: &str, offset: usize) -> String {
    let start = body[..offset].rfind('\n').map_or(0, |index| index + 1);
    let end = body[offset..]
        .find('\n')
        .map_or(body.len(), |relative| offset + relative);
    compact_evidence(&body[start..end])
}

fn compact_evidence(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(180)
        .collect()
}

fn function_spans_for_source(path: &str, source: &str) -> Vec<FunctionSpan> {
    let masked = mask_non_code(source);
    let starts = line_starts(source);
    let test_spans = test_only_spans(&masked);
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
        let Some(open) = find_function_open_brace(&masked, cursor) else {
            index = cursor;
            continue;
        };
        let Some(close) = matching_brace(&masked, open) else {
            index = open + 1;
            continue;
        };
        let start = index;
        let end = close + 1;
        let test_only = test_spans
            .iter()
            .any(|(test_start, test_end)| start >= *test_start && end <= *test_end)
            || has_test_attribute_before(&masked, start);
        spans.push(FunctionSpan {
            path: path.to_string(),
            name,
            start,
            end,
            start_line: line_number_at(start, &starts),
            test_only,
        });
        index = end;
    }
    spans
}

fn called_identifiers(masked: &str) -> Vec<String> {
    let bytes = masked.as_bytes();
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
            let identifier = &masked[start..index];
            if identifier != "fn" {
                out.push(identifier.to_string());
            }
        }
    }
    out
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
    let prefix_start = function_start.saturating_sub(320);
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
mod atomic_commit_tests {
    use super::*;
    #[test]
    fn atomic_fixture_retains_exactly_one_visible_commit() {
        let source = include_str!("ssc_render57_clef_batch_atomic_v2.txt");
        let scan = scan_productive_write_paths_text("src/structural_write_authority/fixture.rs", source);
        let writer = scan.surfaces.iter().find(|s| s.function == "authorized_lower_staff_clef_move_writer").unwrap();
        assert_eq!(writer.xml_buffer_mutations, 1);
        assert_eq!(writer.xml_file_writes, 0);
        assert!(writer.authority_boundary);
    }
    #[test]
    fn commit_visibility_is_required_by_the_v2_construction() {
        let source = include_str!("ssc_render57_clef_batch_atomic_v2.txt");
        let erased = source.replace("xml.replace_range(.., &candidate);", "*xml = candidate;");
        let scan = scan_productive_write_paths_text("src/structural_write_authority/fixture.rs", &erased);
        assert!(!scan.surfaces.iter().any(|s| s.function == "authorized_lower_staff_clef_move_writer"));
    }
}
