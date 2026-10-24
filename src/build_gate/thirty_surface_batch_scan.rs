use super::productive_write_path_scan::ProductiveWriteSurface;
use super::rust_source_scan_primitives::mask_non_code;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ThirtySurfaceBatchRow {
    pub path: String,
    pub function: String,
    pub family: String,
    pub wave: String,
    pub max_xml_buffer_mutations: usize,
    pub max_xml_file_writes: usize,
    pub direct_from_real_main_allowed: bool,
    pub symbol_occurrences: usize,
    pub risk: String,
    pub target_capability: String,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ThirtySurfaceBatchValidation {
    pub rows: usize,
    pub wave_a: usize,
    pub wave_b: usize,
    pub wave_c: usize,
    pub centralized_surfaces: usize,
    pub still_external_surfaces: Vec<String>,
    pub symbol_occurrence_escalations: Vec<String>,
    pub retired_consumer_occurrences: usize,
    pub missing_target_capabilities: Vec<String>,
    pub duplicate_keys: Vec<String>,
    pub invalid_rows: Vec<String>,
}

impl ThirtySurfaceBatchValidation {
    pub fn is_green(&self) -> bool {
        self.rows == 30
            && self.wave_a == 15
            && self.wave_b == 11
            && self.wave_c == 4
            && self.centralized_surfaces == 30
            && self.still_external_surfaces.is_empty()
            && self.symbol_occurrence_escalations.is_empty()
            && self.missing_target_capabilities.is_empty()
            && self.duplicate_keys.is_empty()
            && self.invalid_rows.is_empty()
    }
}

pub fn parse_thirty_surface_batch_manifest(
    text: &str,
) -> Result<Vec<ThirtySurfaceBatchRow>, String> {
    let mut rows = Vec::new();
    for (line_index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let columns: Vec<&str> = line.split('\t').collect();
        if columns.len() != 10 {
            return Err(format!(
                "thirty-surface manifest line {} expected 10 tab-separated columns, got {}",
                line_index + 1,
                columns.len()
            ));
        }
        rows.push(ThirtySurfaceBatchRow {
            path: columns[0].trim().to_string(),
            function: columns[1].trim().to_string(),
            family: columns[2].trim().to_string(),
            wave: columns[3].trim().to_string(),
            max_xml_buffer_mutations: parse_usize(columns[4], line_index + 1, "buffer")?,
            max_xml_file_writes: parse_usize(columns[5], line_index + 1, "file")?,
            direct_from_real_main_allowed: parse_yes_no(columns[6], line_index + 1)?,
            symbol_occurrences: parse_usize(columns[7], line_index + 1, "occurrences")?,
            risk: columns[8].trim().to_string(),
            target_capability: columns[9].trim().to_string(),
        });
    }
    Ok(rows)
}

pub fn validate_thirty_surface_productive_cutover(
    source_root: &Path,
    surfaces: &[ProductiveWriteSurface],
    rows: &[ThirtySurfaceBatchRow],
) -> Result<ThirtySurfaceBatchValidation, String> {
    let mut validation = ThirtySurfaceBatchValidation {
        rows: rows.len(),
        ..ThirtySurfaceBatchValidation::default()
    };
    let external_by_key: BTreeMap<String, &ProductiveWriteSurface> = surfaces
        .iter()
        .filter(|surface| !surface.authority_boundary)
        .map(|surface| (key(&surface.path, &surface.function), surface))
        .collect();
    let sources = read_rust_sources(source_root)?;
    let mut seen = BTreeSet::new();

    for row in rows {
        let row_key = key(&row.path, &row.function);
        if !seen.insert(row_key.clone()) {
            validation.duplicate_keys.push(row_key.clone());
        }
        match row.wave.as_str() {
            "A" => validation.wave_a += 1,
            "B" => validation.wave_b += 1,
            "C" => validation.wave_c += 1,
            _ => validation
                .invalid_rows
                .push(format!("{row_key} invalid wave {:?}", row.wave)),
        }
        if row.path.is_empty()
            || row.function.is_empty()
            || row.family.is_empty()
            || row.target_capability.is_empty()
            || !matches!(row.risk.as_str(), "low" | "medium" | "high")
            || row.function == "real_main"
            || (row.max_xml_buffer_mutations == 0 && row.max_xml_file_writes == 0)
        {
            validation.invalid_rows.push(row_key.clone());
        }

        if let Some(surface) = external_by_key.get(&row_key) {
            validation.still_external_surfaces.push(format!(
                "{} buffer {} file {} direct_main {}",
                row_key,
                surface.xml_buffer_mutations,
                surface.xml_file_writes,
                surface.direct_from_real_main
            ));
        } else {
            validation.centralized_surfaces += 1;
        }

        let occurrences = sources
            .values()
            .map(|source| count_identifier_occurrences(source, &row.function))
            .sum::<usize>();
        if occurrences > row.symbol_occurrences {
            validation.symbol_occurrence_escalations.push(format!(
                "{} occurrences {}/{}",
                row_key, occurrences, row.symbol_occurrences
            ));
        } else {
            // Zero is valid after productive cutover: an obsolete owner may be removed
            // completely once its final consumer has moved behind the capability.
            validation.retired_consumer_occurrences += row.symbol_occurrences - occurrences;
        }

        if target_capability_missing(&sources, occurrences, &row.target_capability) {
            validation.missing_target_capabilities.push(format!(
                "{} target {}",
                row_key, row.target_capability
            ));
        }
    }
    Ok(validation)
}

pub fn enforce_thirty_surface_productive_cutover(
    validation: &ThirtySurfaceBatchValidation,
) -> Result<(), String> {
    if validation.is_green() {
        return Ok(());
    }
    Err(format!(
        "THIRTY-SURFACE PRODUCTIVE CUTOVER DRIFT: rows={} waves={}/{}/{} centralized={} still_external={:?} occurrence_escalations={:?} missing_capabilities={:?} duplicates={:?} invalid={:?}",
        validation.rows,
        validation.wave_a,
        validation.wave_b,
        validation.wave_c,
        validation.centralized_surfaces,
        validation.still_external_surfaces,
        validation.symbol_occurrence_escalations,
        validation.missing_target_capabilities,
        validation.duplicate_keys,
        validation.invalid_rows
    ))
}

fn parse_usize(value: &str, line: usize, column: &str) -> Result<usize, String> {
    value.trim().parse::<usize>().map_err(|error| {
        format!(
            "thirty-surface manifest line {line} invalid {column} count {value:?}: {error}"
        )
    })
}

fn parse_yes_no(value: &str, line: usize) -> Result<bool, String> {
    match value.trim() {
        "ja" => Ok(true),
        "nein" => Ok(false),
        other => Err(format!(
            "thirty-surface manifest line {line} expected ja/nein, got {other:?}"
        )),
    }
}

fn key(path: &str, function: &str) -> String {
    format!("{path}::{function}")
}

fn read_rust_sources(source_root: &Path) -> Result<BTreeMap<PathBuf, String>, String> {
    let mut files = Vec::new();
    collect_rust_files(source_root, &mut files)?;
    files.sort();
    let mut sources = BTreeMap::new();
    for path in files {
        let source = fs::read_to_string(&path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        sources.insert(path, mask_non_code(&source));
    }
    Ok(sources)
}

fn collect_rust_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?
    {
        let entry = entry.map_err(|error| error.to_string())?;
        let child = entry.path();
        if child.is_dir() {
            collect_rust_files(&child, files)?;
        } else if child.extension().and_then(|value| value.to_str()) == Some("rs") {
            files.push(child);
        }
    }
    Ok(())
}


fn target_capability_missing(
    sources: &BTreeMap<PathBuf, String>,
    owner_occurrences: usize,
    target_capability: &str,
) -> bool {
    owner_occurrences > 0
        && !sources
            .values()
            .any(|source| contains_struct_declaration(source, target_capability))
}

fn contains_struct_declaration(source: &str, expected_name: &str) -> bool {
    let mut identifiers = source
        .split(|ch: char| !is_identifier_char(ch))
        .filter(|token| !token.is_empty());
    let mut previous = identifiers.next();
    for current in identifiers {
        if previous == Some("struct") && current == expected_name {
            return true;
        }
        previous = Some(current);
    }
    false
}

fn count_identifier_occurrences(source: &str, identifier: &str) -> usize {
    if identifier.is_empty() {
        return 0;
    }
    source
        .match_indices(identifier)
        .filter(|(start, _)| {
            let end = *start + identifier.len();
            let before_ok = source[..*start]
                .chars()
                .next_back()
                .is_none_or(|ch| !is_identifier_char(ch));
            let after_ok = source[end..]
                .chars()
                .next()
                .is_none_or(|ch| !is_identifier_char(ch));
            before_ok && after_ok
        })
        .count()
}

fn is_identifier_char(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::{
        contains_struct_declaration, count_identifier_occurrences, target_capability_missing,
    };
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    #[test]
    fn identifier_count_ignores_substrings() {
        assert_eq!(count_identifier_occurrences("foo foo_bar xfoo foo()", "foo"), 2);
    }

    #[test]
    fn capability_check_requires_a_struct_declaration() {
        assert!(contains_struct_declaration(
            "fn use_it(_: ExactCapability) {} struct ExactCapability {}",
            "ExactCapability"
        ));
        assert!(!contains_struct_declaration(
            "fn use_it(_: ExactCapability) {}",
            "ExactCapability"
        ));
    }

    #[test]
    fn active_legacy_owner_without_target_capability_is_rejected() {
        let sources = BTreeMap::from([(
            PathBuf::from("source.rs"),
            "fn legacy_owner() {}".to_string(),
        )]);
        assert!(target_capability_missing(
            &sources,
            1,
            "RetiredCapability"
        ));
    }

    #[test]
    fn fully_retired_owner_without_target_capability_is_accepted() {
        let sources = BTreeMap::from([(
            PathBuf::from("source.rs"),
            "fn unrelated_owner() {}".to_string(),
        )]);
        assert!(!target_capability_missing(
            &sources,
            0,
            "RetiredCapability"
        ));
    }
}
