use super::corpus_run_manifest::json_escape as json_escape_local;
use super::*;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::Write as FmtWrite;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

pub(super) const INVENTORY_JSON_FILE: &str = "EtuSchlopf.inventory.json";
pub(super) const INVENTORY_XLSX_FILE: &str = "EtuSchlopf.inventory.xlsx";
const INVENTORY_JSON_PARTIAL: &str = "EtuSchlopf.inventory.json.partial";
const INVENTORY_XLSX_PARTIAL: &str = "EtuSchlopf.inventory.xlsx.partial";
const INVENTORY_JSON_PREVIOUS: &str = "EtuSchlopf.inventory.json.previous";
const INVENTORY_XLSX_PREVIOUS: &str = "EtuSchlopf.inventory.xlsx.previous";
const ESI_CONTRACT: &str = "ESI-3";
const ARCH_CONTRACT: &str = "ARCH-4";
const CMS_CONTRACT: &str = "CMS-1";
const XLSX_CONTRACT: &str = "XLSX-3";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum InventoryPlan {
    Disabled,
    Enabled,
}

impl InventoryPlan {
    pub(super) const fn requested(self) -> bool {
        matches!(self, Self::Enabled)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct InventoryArtifacts {
    pub(super) scope_label: String,
    pub(super) row_count: usize,
    pub(super) class_counts: ArchCounts,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct ArchCounts {
    pub(super) a: usize,
    pub(super) b: usize,
    pub(super) c: usize,
    pub(super) d: usize,
    pub(super) open: usize,
}

impl ArchCounts {
    fn total(self) -> usize {
        self.a + self.b + self.c + self.d + self.open
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NativeMetrics {
    blockers: u64,
    warnings: u64,
    hints: u64,
    canonical_events: u64,
    retained: u64,
    removed: u64,
    producer_binding: String,
    producer_version: String,
    producer_source_signature: String,
    producer_binary_fingerprint: String,
}

impl NativeMetrics {
    fn zero_with_binding(
        producer_binding: String,
        producer_version: String,
        producer_source_signature: String,
        producer_binary_fingerprint: String,
    ) -> Self {
        Self {
            blockers: 0,
            warnings: 0,
            hints: 0,
            canonical_events: 0,
            retained: 0,
            removed: 0,
            producer_binding,
            producer_version,
            producer_source_signature,
            producer_binary_fingerprint,
        }
    }

    fn checked_add(&mut self, other: &Self) -> AppResult<()> {
        self.blockers = self.blockers.checked_add(other.blockers).ok_or_else(|| anyhow!("Inventory CMS-1 aggregation overflow for blocker"))?;
        self.warnings = self.warnings.checked_add(other.warnings).ok_or_else(|| anyhow!("Inventory CMS-1 aggregation overflow for warning"))?;
        self.hints = self.hints.checked_add(other.hints).ok_or_else(|| anyhow!("Inventory CMS-1 aggregation overflow for hint"))?;
        self.canonical_events = self.canonical_events.checked_add(other.canonical_events).ok_or_else(|| anyhow!("Inventory CMS-1 aggregation overflow for canonical_events"))?;
        self.retained = self.retained.checked_add(other.retained).ok_or_else(|| anyhow!("Inventory CMS-1 aggregation overflow for retained"))?;
        self.removed = self.removed.checked_add(other.removed).ok_or_else(|| anyhow!("Inventory CMS-1 aggregation overflow for removed"))?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MeasurementProjection {
    state: &'static str,
    metrics: Option<NativeMetrics>,
    project_count: usize,
    corpus_manifest_run_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InventoryScope<'a> {
    Project(&'a str),
    Corpus,
}

impl InventoryScope<'_> {
    fn label(self) -> &'static str {
        match self {
            Self::Project(_) => "PROJECT",
            Self::Corpus => "CORPUS",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceFile {
    relative: String,
    source: String,
    context: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    text: String,
    line: usize,
    start: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SymbolRow {
    class: String,
    class_rule: String,
    evidence: String,
    qualified_symbol: String,
    simple_name: String,
    module_scope: String,
    declaration_kind: String,
    relative_source_path: String,
    start_line: usize,
    line_count: usize,
    source_visibility: String,
    context: String,
    productive_reachability: String,
    authority: String,
    lexical_hits: usize,
    resolved_references: usize,
    ambiguous_references: usize,
    incoming_calls: usize,
    value_bindings: usize,
    signature_fingerprint: String,
    signature_shape_fingerprint: String,
    body_fingerprint: String,
    a_candidate: bool,
    a_component: String,
    a_closure: String,
    a_blocker: String,
    b_candidate: bool,
    b_counterpart: String,
    b_evidence: String,
    b_blocker: String,
    merge_target: String,
    reason: String,
    body_start: Option<usize>,
    body_end: Option<usize>,
    declaration_name_token: usize,
    has_cfg_or_export_attr: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InventoryModel {
    version: String,
    revision: String,
    take_name: String,
    run_id: String,
    scope: String,
    scope_project: String,
    source_scope_sha256: String,
    source_file_count: usize,
    input_fingerprint: String,
    binary_fingerprint: String,
    measurement: MeasurementProjection,
    rows: Vec<SymbolRow>,
    counts: ArchCounts,
    diagnostics: InventoryDiagnostics,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct InventoryDiagnostics {
    duplicate_simple_name_groups: usize,
    duplicate_simple_name_symbols: usize,
    resolved_reference_sites: usize,
    ambiguous_reference_sites: usize,
    public_api_roots: usize,
    uncertain_public_roots: usize,
    entry_roots: usize,
    reachability_yes: usize,
    reachability_no: usize,
    reachability_unknown: usize,
    reachability_only_test: usize,
    reachability_only_tool: usize,
    open_yes: usize,
    open_no: usize,
    open_unknown: usize,
    open_only_test: usize,
    open_only_tool: usize,
    d_entrypoint: usize,
    d_writer_authority: usize,
    d_selection_authority: usize,
    d_canonical_authority: usize,
    d_governance_gate: usize,
    lexical_only_sites: usize,
    include_bound_reference_sites: usize,
    macro_bound_reference_sites: usize,
    self_method_resolved_sites: usize,
    a_candidate_symbols: usize,
    a_candidate_components: usize,
    a_closed_components: usize,
    a_blocked_ownerless_components: usize,
    a_blocked_productive_components: usize,
    a_blocked_test_components: usize,
    a_blocked_tool_components: usize,
    a_blocked_unknown_components: usize,
    a_blocked_other_no_components: usize,
    b_fingerprint_groups: usize,
    b_candidate_symbols: usize,
    b_cross_file_candidate_symbols: usize,
    b_proven_symbols: usize,
    b_independence_blocked_symbols: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct RemovalAnalysis {
    removable_component_size: BTreeMap<usize, usize>,
    candidate_symbols: usize,
    candidate_components: usize,
    closed_components: usize,
    blocked_ownerless_components: usize,
    blocked_productive_components: usize,
    blocked_test_components: usize,
    blocked_tool_components: usize,
    blocked_unknown_components: usize,
    blocked_other_no_components: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct BCandidateAnalysis {
    fingerprint_groups: usize,
    candidate_symbols: usize,
    cross_file_candidate_symbols: usize,
    proven_symbols: usize,
    independence_blocked_symbols: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct RepositoryRelationships {
    removable_component_size: BTreeMap<usize, usize>,
    duplicate_simple_name_groups: usize,
    duplicate_simple_name_symbols: usize,
    resolved_reference_sites: usize,
    ambiguous_reference_sites: usize,
    lexical_only_sites: usize,
    include_bound_reference_sites: usize,
    macro_bound_reference_sites: usize,
    self_method_resolved_sites: usize,
    public_api_roots: usize,
    uncertain_public_roots: usize,
    entry_roots: usize,
    a_candidate_symbols: usize,
    a_candidate_components: usize,
    a_closed_components: usize,
    a_blocked_ownerless_components: usize,
    a_blocked_productive_components: usize,
    a_blocked_test_components: usize,
    a_blocked_tool_components: usize,
    a_blocked_unknown_components: usize,
    a_blocked_other_no_components: usize,
    b_fingerprint_groups: usize,
    b_candidate_symbols: usize,
    b_cross_file_candidate_symbols: usize,
    b_proven_symbols: usize,
    b_independence_blocked_symbols: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct ItemAttributeFacts {
    test_only: bool,
    sensitive: bool,
}

pub(super) fn inventory_output_name(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "etuschlopf.inventory.json"
            | "etuschlopf.inventory.xlsx"
            | "etuschlopf.inventory.json.partial"
            | "etuschlopf.inventory.xlsx.partial"
            | "etuschlopf.inventory.json.previous"
            | "etuschlopf.inventory.xlsx.previous"
    )
}

pub(super) fn generate_project_inventory(
    project_root: &Path,
    version: &str,
    run_id: &str,
    project: &str,
    target_exe: &Path,
    final_exit: i32,
) -> AppResult<InventoryArtifacts> {
    generate_inventory(
        project_root,
        version,
        run_id,
        InventoryScope::Project(project),
        target_exe,
        final_exit,
    )
}

pub(super) fn generate_corpus_inventory(
    project_root: &Path,
    version: &str,
    run_id: &str,
    target_exe: &Path,
    final_exit: i32,
) -> AppResult<InventoryArtifacts> {
    generate_inventory(
        project_root,
        version,
        run_id,
        InventoryScope::Corpus,
        target_exe,
        final_exit,
    )
}

fn generate_inventory(
    project_root: &Path,
    version: &str,
    run_id: &str,
    scope: InventoryScope<'_>,
    target_exe: &Path,
    final_exit: i32,
) -> AppResult<InventoryArtifacts> {
    cleanup_inventory_transaction_files(project_root)?;
    let (revision, take_name) = take_identity(project_root)?;
    let source_files = collect_source_files(project_root)?;
    let source_scope_sha256 = source_scope_sha256(&source_files);
    let source_file_count = source_files.len();
    let mut rows = extract_symbols(&source_files)?;
    let relationships = derive_repository_relationships(&source_files, &mut rows)?;
    let counts = classify_architecture(&mut rows, &relationships);
    let diagnostics = inventory_diagnostics(&rows, &relationships);
    if counts.total() != rows.len() {
        return Err(anyhow!(
            "ARCH-4 balance failed: rows={} classified={}",
            rows.len(),
            counts.total()
        ));
    }
    validate_inventory_diagnostics(rows.len(), counts, diagnostics)?;

    let measurement = read_measurement_projection(project_root, scope, run_id, final_exit)?;
    let input_fingerprint = input_fingerprint(project_root, scope)?;
    let binary_fingerprint = file_fnv1a64(target_exe).unwrap_or_else(|_| "unavailable".to_string());
    let scope_project = match scope {
        InventoryScope::Project(project) => project.to_string(),
        InventoryScope::Corpus => "10/10".to_string(),
    };
    let model = InventoryModel {
        version: version.to_string(),
        revision,
        take_name,
        run_id: run_id.to_string(),
        scope: scope.label().to_string(),
        scope_project,
        source_scope_sha256,
        source_file_count,
        input_fingerprint,
        binary_fingerprint,
        measurement,
        rows,
        counts,
        diagnostics,
    };

    let json_bytes = render_inventory_json(&model).into_bytes();
    let json_partial = project_root.join(INVENTORY_JSON_PARTIAL);
    fs::write(&json_partial, &json_bytes)?;
    validate_inventory_json(&json_partial, &model)?;

    let xlsx_partial = project_root.join(INVENTORY_XLSX_PARTIAL);
    write_inventory_xlsx(&xlsx_partial, &model)?;
    validate_inventory_xlsx(&xlsx_partial, &model)?;

    publish_inventory_pair(project_root)?;

    Ok(InventoryArtifacts {
        scope_label: model.scope,
        row_count: model.rows.len(),
        class_counts: model.counts,
    })
}

fn cleanup_inventory_transaction_files(project_root: &Path) -> AppResult<()> {
    for name in [
        INVENTORY_JSON_PARTIAL,
        INVENTORY_XLSX_PARTIAL,
        INVENTORY_JSON_PREVIOUS,
        INVENTORY_XLSX_PREVIOUS,
    ] {
        let path = project_root.join(name);
        match fs::remove_file(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(anyhow!(
                    "Inventory stale transaction file could not be removed {}: {error}",
                    path.display()
                ));
            }
        }
    }
    Ok(())
}

fn publish_inventory_pair(project_root: &Path) -> AppResult<()> {
    let json_target = project_root.join(INVENTORY_JSON_FILE);
    let xlsx_target = project_root.join(INVENTORY_XLSX_FILE);
    let json_partial = project_root.join(INVENTORY_JSON_PARTIAL);
    let xlsx_partial = project_root.join(INVENTORY_XLSX_PARTIAL);
    let json_previous = project_root.join(INVENTORY_JSON_PREVIOUS);
    let xlsx_previous = project_root.join(INVENTORY_XLSX_PREVIOUS);

    let json_had_previous = move_if_exists(&json_target, &json_previous)?;
    let xlsx_had_previous = match move_if_exists(&xlsx_target, &xlsx_previous) {
        Ok(value) => value,
        Err(error) => {
            restore_if_needed(&json_previous, &json_target, json_had_previous);
            return Err(error);
        }
    };

    if let Err(error) = fs::rename(&json_partial, &json_target) {
        restore_if_needed(&json_previous, &json_target, json_had_previous);
        restore_if_needed(&xlsx_previous, &xlsx_target, xlsx_had_previous);
        return Err(anyhow!("Inventory JSON publish failed: {error}"));
    }
    if let Err(error) = fs::rename(&xlsx_partial, &xlsx_target) {
        let _ = fs::remove_file(&json_target);
        restore_if_needed(&json_previous, &json_target, json_had_previous);
        restore_if_needed(&xlsx_previous, &xlsx_target, xlsx_had_previous);
        return Err(anyhow!("Inventory XLSX publish failed: {error}"));
    }

    let _ = fs::remove_file(&json_previous);
    let _ = fs::remove_file(&xlsx_previous);
    Ok(())
}

fn move_if_exists(from: &Path, to: &Path) -> AppResult<bool> {
    if !from.exists() {
        return Ok(false);
    }
    let _ = fs::remove_file(to);
    fs::rename(from, to).map_err(|error| {
        anyhow!(
            "Inventory previous-file staging failed {} -> {}: {error}",
            from.display(),
            to.display()
        )
    })?;
    Ok(true)
}

fn restore_if_needed(previous: &Path, target: &Path, needed: bool) {
    if needed && previous.exists() {
        let _ = fs::remove_file(target);
        let _ = fs::rename(previous, target);
    }
}

fn take_identity(project_root: &Path) -> AppResult<(String, String)> {
    let text = fs::read_to_string(project_root.join("ZIP_KIND.txt"))?;
    let mut revision = None;
    let mut name = None;
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("revision=") {
            revision = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("name=") {
            name = Some(value.trim().to_string());
        }
    }
    Ok((
        revision.ok_or_else(|| anyhow!("Inventory ZIP_KIND has no revision"))?,
        name.ok_or_else(|| anyhow!("Inventory ZIP_KIND has no name"))?,
    ))
}

fn collect_source_files(project_root: &Path) -> AppResult<Vec<SourceFile>> {
    let mut paths = Vec::new();
    collect_rs_under(&project_root.join("src"), project_root, &mut paths)?;
    collect_rs_under(&project_root.join("tests"), project_root, &mut paths)?;
    for relative in [
        "tools/RUST_KODEX_GATE_CORE/src",
        "tools/etuschlopf_watch/src",
        "tools/wav_pitch_external_sensor_audit/src",
    ] {
        collect_rs_under(&project_root.join(relative), project_root, &mut paths)?;
    }
    paths.sort();
    paths.dedup();
    let mut files = Vec::with_capacity(paths.len());
    for path in paths {
        let relative = path
            .strip_prefix(project_root)
            .map_err(|error| anyhow!("Inventory source path outside root: {error}"))?
            .to_string_lossy()
            .replace('\\', "/");
        let source = fs::read_to_string(&path).map_err(|error| {
            anyhow!("Inventory source is not readable UTF-8 {}: {error}", path.display())
        })?;
        let context = source_context(&relative, &source);
        files.push(SourceFile {
            relative,
            source,
            context,
        });
    }
    if files.is_empty() {
        return Err(anyhow!("ESI-3 source scope is empty"));
    }
    Ok(files)
}

fn collect_rs_under(dir: &Path, project_root: &Path, out: &mut Vec<PathBuf>) -> AppResult<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.starts_with(project_root.join("target")) {
            continue;
        }
        if path.is_dir() {
            collect_rs_under(&path, project_root, out)?;
        } else if path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case("rs"))
        {
            out.push(path);
        }
    }
    Ok(())
}

fn source_context(relative: &str, source: &str) -> String {
    let lower = relative.to_ascii_lowercase();
    if relative.starts_with("tests/") || source.contains("#![cfg(test)]") {
        return "Test".to_string();
    }
    if relative.starts_with("tools/") {
        return "Tool".to_string();
    }
    if lower.starts_with("src/bin/") || lower.contains("launcher") {
        return "Launcher / Build".to_string();
    }
    if lower.contains("build_gate")
        || lower.contains("kodex")
        || lower.contains("take_change")
        || lower.contains("identity_")
        || lower.contains("trust_anchor")
        || lower.contains("feedback_structuralization_gate")
    {
        return "Governance / Gate".to_string();
    }
    "Produktiv".to_string()
}

fn source_scope_sha256(files: &[SourceFile]) -> String {
    let mut canonical = Vec::new();
    for file in files {
        canonical.extend_from_slice(file.relative.as_bytes());
        canonical.push(0);
        canonical.extend_from_slice(sha256::hex(file.source.as_bytes()).as_bytes());
        canonical.push(b'\n');
    }
    format!("sha256:{}", sha256::hex(&canonical))
}

fn tokenize(source: &str) -> Vec<Token> {
    let bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut index = 0usize;
    let mut line = 1usize;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'\n' {
            line += 1;
            index += 1;
            continue;
        }
        if byte.is_ascii_whitespace() {
            index += 1;
            continue;
        }
        if byte == b'/' && bytes.get(index + 1) == Some(&b'/') {
            index += 2;
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
            continue;
        }
        if byte == b'/' && bytes.get(index + 1) == Some(&b'*') {
            index += 2;
            let mut depth = 1usize;
            while index < bytes.len() && depth > 0 {
                if bytes[index] == b'\n' {
                    line += 1;
                    index += 1;
                } else if bytes[index] == b'/' && bytes.get(index + 1) == Some(&b'*') {
                    depth += 1;
                    index += 2;
                } else if bytes[index] == b'*' && bytes.get(index + 1) == Some(&b'/') {
                    depth -= 1;
                    index += 2;
                } else {
                    index += 1;
                }
            }
            continue;
        }
        if let Some(next) = skip_rust_string(bytes, index, &mut line) {
            out.push(Token {
                text: format!(
                    "<string:{}>",
                    sha256::hex(&bytes[index..next])
                ),
                line,
                start: index,
            });
            index = next;
            continue;
        }
        if let Some(next) = skip_rust_char_literal(bytes, index) {
            out.push(Token {
                text: format!(
                    "<char:{}>",
                    sha256::hex(&bytes[index..next])
                ),
                line,
                start: index,
            });
            index = next;
            continue;
        }
        if is_ident_start(byte) {
            let start = index;
            index += 1;
            while index < bytes.len() && is_ident_continue(bytes[index]) {
                index += 1;
            }
            out.push(Token {
                text: String::from_utf8_lossy(&bytes[start..index]).to_string(),
                line,
                start,
            });
            continue;
        }
        if byte == b':' && bytes.get(index + 1) == Some(&b':') {
            out.push(Token {
                text: "::".to_string(),
                line,
                start: index,
            });
            index += 2;
            continue;
        }
        out.push(Token {
            text: (byte as char).to_string(),
            line,
            start: index,
        });
        index += 1;
    }
    out
}

fn skip_rust_string(bytes: &[u8], index: usize, line: &mut usize) -> Option<usize> {
    let mut prefix = index;
    if bytes.get(prefix) == Some(&b'b')
        && (bytes.get(prefix + 1) == Some(&b'\"') || bytes.get(prefix + 1) == Some(&b'r'))
    {
        prefix += 1;
    }
    if bytes.get(prefix) == Some(&b'r') {
        let mut cursor = prefix + 1;
        let mut hashes = 0usize;
        while bytes.get(cursor) == Some(&b'#') {
            hashes += 1;
            cursor += 1;
        }
        if bytes.get(cursor) != Some(&b'"') {
            return None;
        }
        cursor += 1;
        while cursor < bytes.len() {
            if bytes[cursor] == b'\n' {
                *line += 1;
            }
            if bytes[cursor] == b'"' {
                let mut ok = true;
                for offset in 0..hashes {
                    if bytes.get(cursor + 1 + offset) != Some(&b'#') {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    return Some(cursor + 1 + hashes);
                }
            }
            cursor += 1;
        }
        return Some(cursor);
    }
    if bytes.get(prefix) == Some(&b'"') {
        let mut cursor = prefix + 1;
        while cursor < bytes.len() {
            if bytes[cursor] == b'\n' {
                *line += 1;
            }
            if bytes[cursor] == b'\\' {
                cursor = (cursor + 2).min(bytes.len());
                continue;
            }
            if bytes[cursor] == b'"' {
                return Some(cursor + 1);
            }
            cursor += 1;
        }
        return Some(cursor);
    }
    None
}

fn skip_rust_char_literal(bytes: &[u8], index: usize) -> Option<usize> {
    let mut quote = index;
    if bytes.get(quote) == Some(&b'b') && bytes.get(quote + 1) == Some(&b'\'') {
        quote += 1;
    }
    if bytes.get(quote) != Some(&b'\'') {
        return None;
    }
    let mut cursor = quote + 1;
    if cursor >= bytes.len() {
        return None;
    }
    if bytes[cursor] == b'\\' {
        cursor += 1;
        match bytes.get(cursor).copied()? {
            b'u' if bytes.get(cursor + 1) == Some(&b'{') => {
                cursor += 2;
                while cursor < bytes.len() && bytes[cursor] != b'}' {
                    cursor += 1;
                }
                if bytes.get(cursor) != Some(&b'}') {
                    return None;
                }
                cursor += 1;
            }
            b'x' => {
                cursor = cursor.checked_add(3)?;
            }
            _ => {
                cursor += 1;
            }
        }
    } else {
        let width = utf8_scalar_width(bytes[cursor]);
        if width == 0 || cursor.checked_add(width)? > bytes.len() {
            return None;
        }
        cursor += width;
    }
    if bytes.get(cursor) == Some(&b'\'') {
        Some(cursor + 1)
    } else {
        None
    }
}

fn utf8_scalar_width(first: u8) -> usize {
    match first {
        0x00..=0x7f => 1,
        0xc0..=0xdf => 2,
        0xe0..=0xef => 3,
        0xf0..=0xf7 => 4,
        _ => 0,
    }
}

fn is_ident_start(byte: u8) -> bool {
    byte == b'_' || byte.is_ascii_alphabetic() || byte >= 0x80
}

fn is_ident_continue(byte: u8) -> bool {
    is_ident_start(byte) || byte.is_ascii_digit()
}

fn matching_braces(tokens: &[Token]) -> AppResult<BTreeMap<usize, usize>> {
    let mut stack = Vec::new();
    let mut map = BTreeMap::new();
    for (index, token) in tokens.iter().enumerate() {
        if token.text == "{" {
            stack.push(index);
        } else if token.text == "}" {
            let open = stack.pop().ok_or_else(|| anyhow!("ESI-3 unmatched closing brace"))?;
            map.insert(open, index);
        }
    }
    if !stack.is_empty() {
        return Err(anyhow!("ESI-3 unmatched opening brace"));
    }
    Ok(map)
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ItemContext {
    Module(String),
    Trait(String),
    Impl(String),
    Function(String),
    TestScope,
}

fn extract_symbols(files: &[SourceFile]) -> AppResult<Vec<SymbolRow>> {
    let mut rows = Vec::new();
    for file in files {
        let tokens = tokenize(&file.source);
        let braces = matching_braces(&tokens)?;
        let module = module_path_from_file(&file.relative);
        let mut initial_contexts = vec![ItemContext::Module(module)];
        if file.context == "Test" {
            initial_contexts.push(ItemContext::TestScope);
        }
        parse_items(
            file,
            &tokens,
            &braces,
            0,
            tokens.len(),
            &initial_contexts,
            &mut rows,
        )?;
    }
    rows.sort_by(|left, right| {
        left.relative_source_path
            .cmp(&right.relative_source_path)
            .then_with(|| left.qualified_symbol.cmp(&right.qualified_symbol))
    });
    let mut identities = BTreeSet::new();
    for row in &rows {
        let identity = format!("{}\0{}", row.relative_source_path, row.qualified_symbol);
        if !identities.insert(identity) {
            return Err(anyhow!(
                "ESI-3 duplicate semantic identity: {} :: {}",
                row.relative_source_path,
                row.qualified_symbol
            ));
        }
    }
    if rows.is_empty() {
        return Err(anyhow!("ESI-3 extracted zero symbols"));
    }
    Ok(rows)
}

fn parse_items(
    file: &SourceFile,
    tokens: &[Token],
    braces: &BTreeMap<usize, usize>,
    start: usize,
    end: usize,
    contexts: &[ItemContext],
    rows: &mut Vec<SymbolRow>,
) -> AppResult<()> {
    let mut index = start;
    while index < end {
        let text = tokens[index].text.as_str();
        if text == "mod"
            && is_identifier_token(tokens.get(index + 1))
            && let Some(open) = find_item_open_brace(tokens, index + 2, end)
            && let Some(close) = braces.get(&open).copied()
        {
            let attributes = item_attribute_facts(tokens, index, start);
            let mut nested = contexts.to_vec();
            nested.push(ItemContext::Module(tokens[index + 1].text.clone()));
            if attributes.test_only {
                nested.push(ItemContext::TestScope);
            }
            parse_items(file, tokens, braces, open + 1, close, &nested, rows)?;
            index = close + 1;
            continue;
        }
        if text == "trait"
            && is_identifier_token(tokens.get(index + 1))
            && let Some(open) = find_item_open_brace(tokens, index + 2, end)
            && let Some(close) = braces.get(&open).copied()
        {
            let attributes = item_attribute_facts(tokens, index, start);
            let mut nested = contexts.to_vec();
            nested.push(ItemContext::Trait(tokens[index + 1].text.clone()));
            if attributes.test_only {
                nested.push(ItemContext::TestScope);
            }
            parse_items(file, tokens, braces, open + 1, close, &nested, rows)?;
            index = close + 1;
            continue;
        }
        if text == "impl"
            && let Some(open) = find_item_open_brace(tokens, index + 1, end)
            && let Some(close) = braces.get(&open).copied()
        {
            let attributes = item_attribute_facts(tokens, index, start);
            let label = normalize_impl_label(&tokens[index + 1..open]);
            let mut nested = contexts.to_vec();
            nested.push(ItemContext::Impl(label));
            if attributes.test_only {
                nested.push(ItemContext::TestScope);
            }
            parse_items(file, tokens, braces, open + 1, close, &nested, rows)?;
            index = close + 1;
            continue;
        }
        if text == "fn" && is_identifier_token(tokens.get(index + 1)) {
            let name_index = index + 1;
            let name = tokens[name_index].text.clone();
            let declaration_start = tokens[index].line;
            let (body_start, body_end, end_line, skip_to) =
                function_extent(tokens, braces, name_index + 1, end)?;
            let prefix_start = item_prefix_start(tokens, index, start);
            let attributes = item_attribute_facts(tokens, index, start);
            let declaration_kind = declaration_kind(contexts);
            let qualified_symbol = qualify_symbol(contexts, &name);
            let module_scope = module_scope(contexts);
            let visibility = item_visibility(tokens, index, start, contexts);
            let context = if attributes.test_only
                || contexts
                    .iter()
                    .any(|context| matches!(context, ItemContext::TestScope))
            {
                "Test".to_string()
            } else {
                file.context.clone()
            };
            let signature_end = body_start.unwrap_or_else(|| skip_to.saturating_sub(1));
            let signature_fingerprint = token_fingerprint(&tokens[prefix_start..signature_end]);
            let signature_shape_fingerprint = token_fingerprint_with_name_placeholder(
                &tokens[prefix_start..signature_end],
                name_index.saturating_sub(prefix_start),
            );
            let body_fingerprint = match (body_start, body_end) {
                (Some(open), Some(close)) => token_fingerprint(&tokens[open + 1..close]),
                _ => "none".to_string(),
            };
            rows.push(SymbolRow {
                class: "OFFEN".to_string(),
                class_rule: "OFFEN.UNCLASSIFIED".to_string(),
                evidence: "UNKNOWN".to_string(),
                qualified_symbol,
                simple_name: name.clone(),
                module_scope,
                declaration_kind,
                relative_source_path: file.relative.clone(),
                start_line: declaration_start,
                line_count: end_line.saturating_sub(declaration_start).saturating_add(1),
                source_visibility: visibility,
                context,
                productive_reachability: "UNKNOWN".to_string(),
                authority: "NONE".to_string(),
                lexical_hits: 0,
                resolved_references: 0,
                ambiguous_references: 0,
                incoming_calls: 0,
                value_bindings: 0,
                signature_fingerprint,
                signature_shape_fingerprint,
                body_fingerprint,
                a_candidate: false,
                a_component: String::new(),
                a_closure: "NOT_EVALUATED".to_string(),
                a_blocker: String::new(),
                b_candidate: false,
                b_counterpart: String::new(),
                b_evidence: String::new(),
                b_blocker: String::new(),
                merge_target: String::new(),
                reason: "ARCH-4 prerequisites not evaluated".to_string(),
                body_start,
                body_end,
                declaration_name_token: name_index,
                has_cfg_or_export_attr: attributes.sensitive,
            });
            if let (Some(open), Some(close)) = (body_start, body_end) {
                let mut nested = contexts.to_vec();
                nested.push(ItemContext::Function(name));
                if attributes.test_only {
                    nested.push(ItemContext::TestScope);
                }
                parse_items(file, tokens, braces, open + 1, close, &nested, rows)?;
            }
            index = skip_to;
            continue;
        }
        if text == "{" && let Some(close) = braces.get(&index).copied() {
            index = close + 1;
            continue;
        }
        index += 1;
    }
    Ok(())
}

fn is_identifier_token(token: Option<&Token>) -> bool {
    token.is_some_and(|token| {
        token
            .text
            .as_bytes()
            .first()
            .copied()
            .is_some_and(is_ident_start)
    })
}

fn find_item_open_brace(tokens: &[Token], start: usize, end: usize) -> Option<usize> {
    let mut paren = 0isize;
    let mut bracket = 0isize;
    for (index, token) in tokens.iter().enumerate().take(end).skip(start) {
        match token.text.as_str() {
            "(" => paren += 1,
            ")" => paren -= 1,
            "[" => bracket += 1,
            "]" => bracket -= 1,
            "{" if paren == 0 && bracket == 0 => return Some(index),
            ";" if paren == 0 && bracket == 0 => return None,
            _ => {}
        }
    }
    None
}

fn function_extent(
    tokens: &[Token],
    braces: &BTreeMap<usize, usize>,
    start: usize,
    end: usize,
) -> AppResult<(Option<usize>, Option<usize>, usize, usize)> {
    let mut paren = 0isize;
    let mut bracket = 0isize;
    for index in start..end {
        match tokens[index].text.as_str() {
            "(" => paren += 1,
            ")" => paren -= 1,
            "[" => bracket += 1,
            "]" => bracket -= 1,
            "{" if paren == 0 && bracket == 0 => {
                let close = braces
                    .get(&index)
                    .copied()
                    .ok_or_else(|| anyhow!("ESI-3 function body brace not matched"))?;
                return Ok((Some(index), Some(close), tokens[close].line, close + 1));
            }
            ";" if paren == 0 && bracket == 0 => {
                return Ok((None, None, tokens[index].line, index + 1));
            }
            _ => {}
        }
    }
    Err(anyhow!("ESI-3 function declaration has no body or semicolon"))
}

fn normalize_impl_label(tokens: &[Token]) -> String {
    let mut out = String::new();
    let mut angle = 0isize;
    for token in tokens {
        if token.text == "<" {
            angle += 1;
            if angle == 1 && out.is_empty() {
                continue;
            }
        }
        if token.text == ">" && angle > 0 {
            angle -= 1;
            if angle == 0 && out.is_empty() {
                continue;
            }
        }
        if angle > 0 && out.is_empty() {
            continue;
        }
        if !out.is_empty()
            && token.text != "::"
            && !matches!(out.chars().last(), Some(':') | Some('<') | Some('('))
            && token.text != ">"
            && token.text != ","
        {
            out.push(' ');
        }
        out.push_str(&token.text);
    }
    let compact = out.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.is_empty() {
        "impl".to_string()
    } else {
        compact
    }
}

fn module_path_from_file(relative: &str) -> String {
    let mut path = relative.trim_end_matches(".rs").replace('/', "::");
    if let Some(stripped) = path.strip_prefix("src::") {
        path = stripped.to_string();
    }
    if let Some(stripped) = path.strip_suffix("::mod") {
        path = stripped.to_string();
    }
    path
}

fn declaration_kind(contexts: &[ItemContext]) -> String {
    for context in contexts.iter().rev() {
        match context {
            ItemContext::Trait(_) => return "trait_method".to_string(),
            ItemContext::Impl(label) if label.contains(" for ") => {
                return "trait_impl_method".to_string();
            }
            ItemContext::Impl(_) => return "inherent_method".to_string(),
            ItemContext::Function(_) => return "nested_function".to_string(),
            ItemContext::Module(_) | ItemContext::TestScope => {}
        }
    }
    "function".to_string()
}

fn qualify_symbol(contexts: &[ItemContext], name: &str) -> String {
    let mut pieces = Vec::new();
    for context in contexts {
        match context {
            ItemContext::Module(value) if !value.is_empty() => pieces.push(value.clone()),
            ItemContext::Trait(value) => pieces.push(value.clone()),
            ItemContext::Impl(value) => pieces.push(value.clone()),
            ItemContext::Function(value) => pieces.push(value.clone()),
            ItemContext::Module(_) | ItemContext::TestScope => {}
        }
    }
    pieces.push(name.to_string());
    pieces.join("::")
}

fn module_scope(contexts: &[ItemContext]) -> String {
    contexts
        .iter()
        .filter_map(|context| match context {
            ItemContext::Module(value) if !value.is_empty() => Some(value.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("::")
}

fn item_prefix_start(tokens: &[Token], item_index: usize, lower_bound: usize) -> usize {
    let mut cursor = item_index;
    while cursor > lower_bound {
        match tokens[cursor - 1].text.as_str() {
            ";" | "{" | "}" => break,
            _ => cursor -= 1,
        }
    }
    cursor
}

fn item_visibility(
    tokens: &[Token],
    item_index: usize,
    lower_bound: usize,
    contexts: &[ItemContext],
) -> String {
    if contexts.iter().any(|context| matches!(context, ItemContext::Trait(_))) {
        return "trait_member".to_string();
    }
    let start = item_prefix_start(tokens, item_index, lower_bound);
    let Some(pub_index) = (start..item_index)
        .rev()
        .find(|&index| tokens[index].text == "pub")
    else {
        return "private".to_string();
    };
    if tokens.get(pub_index + 1).is_some_and(|token| token.text == "(") {
        match tokens.get(pub_index + 2).map(|token| token.text.as_str()) {
            Some("crate") => "pub(crate)".to_string(),
            Some("super") => "pub(super)".to_string(),
            Some("in") => "pub(in path)".to_string(),
            _ => "pub(restricted)".to_string(),
        }
    } else {
        "pub".to_string()
    }
}

fn item_attribute_facts(tokens: &[Token], item_index: usize, lower_bound: usize) -> ItemAttributeFacts {
    let start = item_prefix_start(tokens, item_index, lower_bound);
    let mut facts = ItemAttributeFacts::default();
    let mut index = start;
    while index + 2 < item_index {
        if tokens[index].text != "#" || tokens[index + 1].text != "[" {
            index += 1;
            continue;
        }
        let mut depth = 1usize;
        let mut cursor = index + 2;
        while cursor < item_index && depth > 0 {
            match tokens[cursor].text.as_str() {
                "[" => depth += 1,
                "]" => depth -= 1,
                _ => {}
            }
            cursor += 1;
        }
        if depth != 0 {
            break;
        }
        let attribute_end = cursor.saturating_sub(1);
        apply_attribute_facts(&tokens[index + 2..attribute_end], &mut facts);
        index = cursor;
    }
    facts
}

fn apply_attribute_facts(tokens: &[Token], facts: &mut ItemAttributeFacts) {
    let Some(name) = tokens.first().map(|token| token.text.as_str()) else {
        return;
    };
    if tokens.iter().any(|token| {
        matches!(
            token.text.as_str(),
            "no_mangle"
                | "export_name"
                | "link_section"
                | "used"
                | "proc_macro"
                | "proc_macro_attribute"
                | "proc_macro_derive"
                | "dead_code"
        )
    }) {
        facts.sensitive = true;
    }
    match name {
        "test" | "bench" => {
            facts.test_only = true;
            facts.sensitive = true;
        }
        "cfg" => {
            facts.sensitive = true;
            if tokens.len() >= 3
                && tokens[1].text == "("
                && tokens.last().is_some_and(|token| token.text == ")")
                && cfg_expr_requires_test(&tokens[2..tokens.len() - 1])
            {
                facts.test_only = true;
            }
        }
        "cfg_attr" => {
            facts.sensitive = true;
        }
        _ => {}
    }
}

fn cfg_expr_requires_test(tokens: &[Token]) -> bool {
    if tokens.len() == 1 {
        return tokens[0].text == "test";
    }
    if tokens.len() < 3 || tokens[1].text != "(" || tokens.last().is_none_or(|token| token.text != ")") {
        return false;
    }
    let operator = tokens[0].text.as_str();
    let args = split_cfg_args(&tokens[2..tokens.len() - 1]);
    match operator {
        "all" => args.iter().any(|arg| cfg_expr_requires_test(arg)),
        "any" => !args.is_empty() && args.iter().all(|arg| cfg_expr_requires_test(arg)),
        "not" => false,
        _ => false,
    }
}

fn split_cfg_args(tokens: &[Token]) -> Vec<&[Token]> {
    let mut args = Vec::new();
    let mut start = 0usize;
    let mut depth = 0isize;
    for (index, token) in tokens.iter().enumerate() {
        match token.text.as_str() {
            "(" => depth += 1,
            ")" => depth -= 1,
            "," if depth == 0 => {
                if start < index {
                    args.push(&tokens[start..index]);
                }
                start = index + 1;
            }
            _ => {}
        }
    }
    if start < tokens.len() {
        args.push(&tokens[start..]);
    }
    args
}

fn token_fingerprint(tokens: &[Token]) -> String {
    let mut canonical = Vec::new();
    for token in tokens {
        canonical.extend_from_slice(&(token.text.len() as u64).to_le_bytes());
        canonical.extend_from_slice(token.text.as_bytes());
    }
    format!("sha256:{}", sha256::hex(&canonical))
}

fn token_fingerprint_with_name_placeholder(tokens: &[Token], name_index: usize) -> String {
    let mut canonical = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        let text = if index == name_index { "<fn-name>" } else { token.text.as_str() };
        canonical.extend_from_slice(&(text.len() as u64).to_le_bytes());
        canonical.extend_from_slice(text.as_bytes());
    }
    format!("sha256:{}", sha256::hex(&canonical))
}

fn derive_repository_relationships(
    files: &[SourceFile],
    rows: &mut [SymbolRow],
) -> AppResult<RepositoryRelationships> {
    let mut file_tokens = BTreeMap::new();
    for file in files {
        file_tokens.insert(file.relative.clone(), tokenize(&file.source));
    }
    let mut names: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, row) in rows.iter().enumerate() {
        names.entry(row.simple_name.clone()).or_default().push(index);
    }
    let duplicate_simple_name_groups = names.values().filter(|values| values.len() > 1).count();
    let duplicate_simple_name_symbols = names
        .values()
        .filter(|values| values.len() > 1)
        .map(Vec::len)
        .sum();

    let token_owners = build_token_owner_map(&file_tokens, rows);
    let include_file_owners = static_include_file_owners(files, &file_tokens, &token_owners)?;
    let macro_body_owners = discover_macro_body_owners(&file_tokens, &token_owners)?;

    let mut reference_owners: Vec<BTreeSet<Option<usize>>> = vec![BTreeSet::new(); rows.len()];
    let mut edges: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); rows.len()];
    let mut resolved_reference_sites = 0usize;
    let mut ambiguous_reference_sites = 0usize;
    let mut lexical_only_sites = 0usize;
    let mut include_bound_reference_sites = 0usize;
    let mut macro_bound_reference_sites = 0usize;
    let mut self_method_resolved_sites = 0usize;

    for (file, tokens) in &file_tokens {
        let owners = token_owners
            .get(file)
            .ok_or_else(|| anyhow!("ESI-3 token-owner map missing for {file}"))?;
        let inherited_file_owners = include_file_owners
            .get(file)
            .ok_or_else(|| anyhow!("ESI-3 include-owner map missing for {file}"))?;
        let macro_owners = macro_body_owners
            .get(file)
            .ok_or_else(|| anyhow!("ESI-3 macro-owner map missing for {file}"))?;
        for (token_index, token) in tokens.iter().enumerate() {
            let Some(candidates) = names.get(&token.text) else {
                continue;
            };
            if candidates.iter().any(|&row_index| {
                rows[row_index].relative_source_path == *file
                    && rows[row_index].declaration_name_token == token_index
            }) {
                continue;
            }
            for &row_index in candidates {
                rows[row_index].lexical_hits = rows[row_index].lexical_hits.saturating_add(1);
            }

            let Some(site_kind) = reference_site_kind(tokens, token_index) else {
                lexical_only_sites = lexical_only_sites.saturating_add(1);
                continue;
            };

            let macro_site_owners = macro_owners[token_index].as_ref();
            if macro_site_owners.is_some_and(BTreeSet::is_empty) {
                continue;
            }
            let direct_owner = owners[token_index];
            let include_bound = direct_owner.is_none() && !inherited_file_owners.is_empty();
            let mut site_owners = BTreeSet::new();
            if let Some(macro_site_owners) = macro_site_owners {
                site_owners.extend(macro_site_owners.iter().copied());
            } else if let Some(owner) = direct_owner {
                site_owners.insert(owner);
            } else {
                site_owners.extend(inherited_file_owners.iter().copied());
            }
            let resolution_owner = (site_owners.len() == 1)
                .then(|| *site_owners.iter().next().expect("single owner"))
                .or(direct_owner);

            match resolve_reference_site(
                rows,
                candidates,
                resolution_owner,
                file,
                tokens,
                token_index,
            ) {
                ReferenceResolution::Resolved(target) => {
                    resolved_reference_sites = resolved_reference_sites.saturating_add(1);
                    rows[target].resolved_references =
                        rows[target].resolved_references.saturating_add(1);
                    if site_owners.is_empty() {
                        reference_owners[target].insert(None);
                    } else {
                        for &owner in &site_owners {
                            reference_owners[target].insert(Some(owner));
                        }
                    }
                    if include_bound {
                        include_bound_reference_sites =
                            include_bound_reference_sites.saturating_add(1);
                    }
                    if macro_site_owners.is_some() {
                        macro_bound_reference_sites =
                            macro_bound_reference_sites.saturating_add(1);
                    }
                    if site_kind == ReferenceSiteKind::SelfMethodCall {
                        self_method_resolved_sites =
                            self_method_resolved_sites.saturating_add(1);
                    }

                    for &caller in &site_owners {
                        if caller == target {
                            continue;
                        }
                        match site_kind {
                            ReferenceSiteKind::Call | ReferenceSiteKind::SelfMethodCall => {
                                rows[target].incoming_calls =
                                    rows[target].incoming_calls.saturating_add(1);
                                edges[caller].insert(target);
                            }
                            ReferenceSiteKind::ValueBinding => {
                                rows[target].value_bindings =
                                    rows[target].value_bindings.saturating_add(1);
                                edges[caller].insert(target);
                            }
                            ReferenceSiteKind::QualifiedPath | ReferenceSiteKind::Import => {}
                        }
                    }
                }
                ReferenceResolution::Ambiguous(indices) => {
                    ambiguous_reference_sites = ambiguous_reference_sites.saturating_add(1);
                    for target in indices {
                        rows[target].ambiguous_references =
                            rows[target].ambiguous_references.saturating_add(1);
                    }
                }
            }
        }
    }

    let public_module_names = direct_public_lib_module_names(files);
    let mut reachable = BTreeSet::new();
    let mut queue = VecDeque::new();
    let mut uncertain_reachable = BTreeSet::new();
    let mut uncertain_queue = VecDeque::new();
    let mut public_api_roots = 0usize;
    let mut uncertain_public_roots = 0usize;
    let mut entry_roots = 0usize;

    for (index, row) in rows.iter().enumerate() {
        if matches!(row.context.as_str(), "Test" | "Tool") {
            continue;
        }
        let entry_root = row.simple_name == "main"
            && matches!(
                row.context.as_str(),
                "Produktiv" | "Launcher / Build" | "Governance / Gate"
            );
        let public_api_root = row.source_visibility == "pub"
            && is_direct_public_api_scope(row, &public_module_names);
        if entry_root || public_api_root {
            if entry_root {
                entry_roots += 1;
            }
            if public_api_root {
                public_api_roots += 1;
            }
            if reachable.insert(index) {
                queue.push_back(index);
            }
        } else if row.source_visibility == "pub" {
            uncertain_public_roots += 1;
            if uncertain_reachable.insert(index) {
                uncertain_queue.push_back(index);
            }
        }
    }
    while let Some(index) = queue.pop_front() {
        for &target in &edges[index] {
            if reachable.insert(target) {
                queue.push_back(target);
            }
        }
    }
    while let Some(index) = uncertain_queue.pop_front() {
        for &target in &edges[index] {
            if !reachable.contains(&target) && uncertain_reachable.insert(target) {
                uncertain_queue.push_back(target);
            }
        }
    }

    for (index, row) in rows.iter_mut().enumerate() {
        row.productive_reachability = match row.context.as_str() {
            "Test" => "ONLY_TEST".to_string(),
            "Tool" => "ONLY_TOOL".to_string(),
            _ if reachable.contains(&index) => "YES".to_string(),
            _ if uncertain_reachable.contains(&index)
                || row.ambiguous_references > 0
                || row.declaration_kind.contains("trait") => "UNKNOWN".to_string(),
            _ => "NO".to_string(),
        };
    }

    let removal = analyze_removal_components(rows, &reference_owners);
    let b_analysis = analyze_b_candidates(rows);

    Ok(RepositoryRelationships {
        removable_component_size: removal.removable_component_size,
        duplicate_simple_name_groups,
        duplicate_simple_name_symbols,
        resolved_reference_sites,
        ambiguous_reference_sites,
        lexical_only_sites,
        include_bound_reference_sites,
        macro_bound_reference_sites,
        self_method_resolved_sites,
        public_api_roots,
        uncertain_public_roots,
        entry_roots,
        a_candidate_symbols: removal.candidate_symbols,
        a_candidate_components: removal.candidate_components,
        a_closed_components: removal.closed_components,
        a_blocked_ownerless_components: removal.blocked_ownerless_components,
        a_blocked_productive_components: removal.blocked_productive_components,
        a_blocked_test_components: removal.blocked_test_components,
        a_blocked_tool_components: removal.blocked_tool_components,
        a_blocked_unknown_components: removal.blocked_unknown_components,
        a_blocked_other_no_components: removal.blocked_other_no_components,
        b_fingerprint_groups: b_analysis.fingerprint_groups,
        b_candidate_symbols: b_analysis.candidate_symbols,
        b_cross_file_candidate_symbols: b_analysis.cross_file_candidate_symbols,
        b_proven_symbols: b_analysis.proven_symbols,
        b_independence_blocked_symbols: b_analysis.independence_blocked_symbols,
    })
}

fn build_token_owner_map(
    file_tokens: &BTreeMap<String, Vec<Token>>,
    rows: &[SymbolRow],
) -> BTreeMap<String, Vec<Option<usize>>> {
    let mut token_owners = BTreeMap::new();
    for (file, tokens) in file_tokens {
        let mut owners = vec![None; tokens.len()];
        let mut spans = rows
            .iter()
            .enumerate()
            .filter_map(|(row_index, row)| {
                if row.relative_source_path != *file {
                    return None;
                }
                let (Some(body_start), Some(body_end)) = (row.body_start, row.body_end) else {
                    return None;
                };
                Some((body_end.saturating_sub(body_start), row_index, body_start, body_end))
            })
            .collect::<Vec<_>>();
        spans.sort_by(|left, right| {
            right
                .0
                .cmp(&left.0)
                .then_with(|| left.2.cmp(&right.2))
                .then_with(|| left.1.cmp(&right.1))
        });
        for (_, row_index, body_start, body_end) in spans {
            let start = body_start.saturating_add(1);
            let end = body_end.min(owners.len());
            for owner in owners.iter_mut().take(end).skip(start) {
                *owner = Some(row_index);
            }
        }
        token_owners.insert(file.clone(), owners);
    }
    token_owners
}

fn static_include_file_owners(
    files: &[SourceFile],
    file_tokens: &BTreeMap<String, Vec<Token>>,
    token_owners: &BTreeMap<String, Vec<Option<usize>>>,
) -> AppResult<BTreeMap<String, BTreeSet<usize>>> {
    let source_by_path = files
        .iter()
        .map(|file| (file.relative.clone(), file.source.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut edges = Vec::new();
    for (file, tokens) in file_tokens {
        let Some(source) = source_by_path.get(file) else {
            continue;
        };
        let owners = token_owners
            .get(file)
            .ok_or_else(|| anyhow!("ESI-3 include source owner map missing for {file}"))?;
        for index in 0..tokens.len() {
            if tokens[index].text != "include"
                || tokens.get(index + 1).is_none_or(|token| token.text != "!")
            {
                continue;
            }
            let Some(target) = static_include_target(file, source, tokens, index) else {
                continue;
            };
            if file_tokens.contains_key(&target) {
                edges.push((file.clone(), owners[index], target));
            }
        }
    }

    let mut inherited = file_tokens
        .keys()
        .map(|file| (file.clone(), BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for _ in 0..=file_tokens.len() {
        let mut next = inherited.clone();
        for (source, direct_owner, target) in &edges {
            let source_owners = if let Some(owner) = direct_owner {
                BTreeSet::from([*owner])
            } else {
                inherited
                    .get(source)
                    .cloned()
                    .ok_or_else(|| anyhow!("ESI-3 include source inheritance missing for {source}"))?
            };
            next.get_mut(target)
                .ok_or_else(|| anyhow!("ESI-3 include target inheritance missing for {target}"))?
                .extend(source_owners);
        }
        if next == inherited {
            return Ok(inherited);
        }
        inherited = next;
    }
    Err(anyhow!("ESI-3 static include ownership did not converge"))
}

fn static_include_target(
    file: &str,
    source: &str,
    tokens: &[Token],
    include_index: usize,
) -> Option<String> {
    if tokens.get(include_index + 1)?.text != "!"
        || tokens.get(include_index + 2)?.text != "("
        || !tokens.get(include_index + 3)?.text.starts_with("<string:")
        || tokens.get(include_index + 4)?.text != ")"
    {
        return None;
    }
    let literal = plain_string_literal_value(source, tokens[include_index + 3].start)?;
    let parent = Path::new(file).parent().unwrap_or_else(|| Path::new(""));
    normalize_relative_path(&parent.join(literal))
}

fn plain_string_literal_value(source: &str, start: usize) -> Option<String> {
    let bytes = source.as_bytes();
    if bytes.get(start) != Some(&b'\"') {
        return None;
    }
    let mut out = String::new();
    let mut cursor = start + 1;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'\"' => return Some(out),
            b'\\' => {
                cursor += 1;
                let escaped = *bytes.get(cursor)?;
                match escaped {
                    b'\\' => out.push('\\'),
                    b'\"' => out.push('\"'),
                    b'n' => out.push('\n'),
                    b'r' => out.push('\r'),
                    b't' => out.push('\t'),
                    _ => return None,
                }
            }
            byte if byte.is_ascii() => out.push(byte as char),
            _ => return None,
        }
        cursor += 1;
    }
    None
}

fn normalize_relative_path(path: &Path) -> Option<String> {
    use std::path::Component;
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(value) => parts.push(value.to_string_lossy().to_string()),
            Component::ParentDir => {
                parts.pop()?;
            }
            Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    Some(parts.join("/"))
}

#[derive(Debug, Clone)]
struct MacroDefinition {
    file: String,
    name: String,
    open: usize,
    close: usize,
}

fn discover_macro_body_owners(
    file_tokens: &BTreeMap<String, Vec<Token>>,
    token_owners: &BTreeMap<String, Vec<Option<usize>>>,
) -> AppResult<BTreeMap<String, Vec<Option<BTreeSet<usize>>>>> {
    let mut definitions = Vec::new();
    for (file, tokens) in file_tokens {
        let braces = matching_braces(tokens)?;
        let mut index = 0usize;
        while index + 3 < tokens.len() {
            if tokens[index].text == "macro_rules"
                && tokens[index + 1].text == "!"
                && is_identifier_token(tokens.get(index + 2))
                && let Some(open) = find_item_open_brace(tokens, index + 3, tokens.len())
                && let Some(close) = braces.get(&open).copied()
            {
                definitions.push(MacroDefinition {
                    file: file.clone(),
                    name: tokens[index + 2].text.clone(),
                    open,
                    close,
                });
                index = close + 1;
                continue;
            }
            index += 1;
        }
    }

    let mut by_name: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, definition) in definitions.iter().enumerate() {
        by_name.entry(definition.name.clone()).or_default().push(index);
    }
    let mut invocation_owners = vec![BTreeSet::new(); definitions.len()];
    for (file, tokens) in file_tokens {
        let owners = token_owners
            .get(file)
            .ok_or_else(|| anyhow!("ESI-3 macro token-owner map missing for {file}"))?;
        for index in 0..tokens.len().saturating_sub(1) {
            if tokens[index + 1].text != "!" || !is_identifier_token(tokens.get(index)) {
                continue;
            }
            if index > 0 && tokens[index - 1].text == "macro_rules" {
                continue;
            }
            let Some(candidates) = by_name.get(&tokens[index].text) else {
                continue;
            };
            let same_file = candidates
                .iter()
                .copied()
                .filter(|&candidate| definitions[candidate].file == *file)
                .collect::<Vec<_>>();
            let definition_index = if same_file.len() == 1 {
                Some(same_file[0])
            } else if candidates.len() == 1 {
                Some(candidates[0])
            } else {
                None
            };
            let (Some(definition_index), Some(owner)) = (definition_index, owners[index]) else {
                continue;
            };
            invocation_owners[definition_index].insert(owner);
        }
    }

    let mut body_owners = file_tokens
        .iter()
        .map(|(file, tokens)| (file.clone(), vec![None; tokens.len()]))
        .collect::<BTreeMap<_, _>>();
    for (definition_index, definition) in definitions.iter().enumerate() {
        let slots = body_owners
            .get_mut(&definition.file)
            .ok_or_else(|| anyhow!("ESI-3 macro body map missing for {}", definition.file))?;
        let owner_set = invocation_owners[definition_index].clone();
        for slot in slots
            .iter_mut()
            .take(definition.close)
            .skip(definition.open.saturating_add(1))
        {
            *slot = Some(owner_set.clone());
        }
    }
    Ok(body_owners)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReferenceSiteKind {
    Call,
    SelfMethodCall,
    ValueBinding,
    QualifiedPath,
    Import,
}

fn reference_site_kind(tokens: &[Token], token_index: usize) -> Option<ReferenceSiteKind> {
    if is_use_path_reference(tokens, token_index) {
        return Some(ReferenceSiteKind::Import);
    }
    let previous_is_member = token_index > 0 && tokens[token_index - 1].text == ".";
    if previous_is_member && is_call_reference(tokens, token_index) {
        if token_index >= 2 && tokens[token_index - 2].text == "self" {
            return Some(ReferenceSiteKind::SelfMethodCall);
        }
        return Some(ReferenceSiteKind::Call);
    }
    if is_call_reference(tokens, token_index) {
        return Some(ReferenceSiteKind::Call);
    }
    if explicit_path_suffix(tokens, token_index).is_some() {
        return Some(if is_value_binding_reference(tokens, token_index) {
            ReferenceSiteKind::ValueBinding
        } else {
            ReferenceSiteKind::QualifiedPath
        });
    }
    is_value_binding_reference(tokens, token_index).then_some(ReferenceSiteKind::ValueBinding)
}

fn is_use_path_reference(tokens: &[Token], token_index: usize) -> bool {
    let mut cursor = token_index;
    while cursor > 0 {
        cursor -= 1;
        match tokens[cursor].text.as_str() {
            ";" | "{" | "}" => return false,
            "use" => return true,
            "fn" | "struct" | "enum" | "trait" | "impl" | "mod" => return false,
            _ => {}
        }
    }
    false
}

fn analyze_removal_components(
    rows: &mut [SymbolRow],
    reference_owners: &[BTreeSet<Option<usize>>],
) -> RemovalAnalysis {
    let candidates = rows
        .iter()
        .enumerate()
        .filter_map(|(index, row)| removal_component_candidate(row).then_some(index))
        .collect::<BTreeSet<_>>();
    let mut adjacency: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); rows.len()];
    for &target in &candidates {
        for owner in &reference_owners[target] {
            let Some(owner) = *owner else {
                continue;
            };
            if owner != target && candidates.contains(&owner) {
                adjacency[target].insert(owner);
                adjacency[owner].insert(target);
            }
        }
    }

    let mut analysis = RemovalAnalysis {
        candidate_symbols: candidates.len(),
        ..RemovalAnalysis::default()
    };
    let mut seen = BTreeSet::new();
    let mut component_ordinal = 0usize;
    for &start in &candidates {
        if !seen.insert(start) {
            continue;
        }
        component_ordinal += 1;
        analysis.candidate_components += 1;
        let mut component = Vec::new();
        let mut queue = VecDeque::from([start]);
        while let Some(index) = queue.pop_front() {
            component.push(index);
            for &next in &adjacency[index] {
                if seen.insert(next) {
                    queue.push_back(next);
                }
            }
        }
        let component_set = component.iter().copied().collect::<BTreeSet<_>>();
        let mut blockers = BTreeSet::new();
        for &target in &component {
            for owner in &reference_owners[target] {
                match owner {
                    None => {
                        blockers.insert("OWNERLESS_SOURCE".to_string());
                    }
                    Some(owner_index) if component_set.contains(owner_index) => {}
                    Some(owner_index) => {
                        let owner_row = &rows[*owner_index];
                        let blocker = match owner_row.productive_reachability.as_str() {
                            "YES" => "PRODUCTIVE_DEPENDENCY",
                            "ONLY_TEST" => "TEST_DEPENDENCY",
                            "ONLY_TOOL" => "TOOL_DEPENDENCY",
                            "UNKNOWN" => "UNKNOWN_OWNER",
                            "NO" => "OTHER_UNREACHABLE_COMPONENT",
                            _ => "UNKNOWN_OWNER",
                        };
                        blockers.insert(blocker.to_string());
                    }
                }
            }
        }
        let closed = blockers.is_empty();
        if closed {
            analysis.closed_components += 1;
            let size = component.len();
            for &index in &component {
                analysis.removable_component_size.insert(index, size);
            }
        } else {
            if blockers.contains("OWNERLESS_SOURCE") {
                analysis.blocked_ownerless_components += 1;
            }
            if blockers.contains("PRODUCTIVE_DEPENDENCY") {
                analysis.blocked_productive_components += 1;
            }
            if blockers.contains("TEST_DEPENDENCY") {
                analysis.blocked_test_components += 1;
            }
            if blockers.contains("TOOL_DEPENDENCY") {
                analysis.blocked_tool_components += 1;
            }
            if blockers.contains("UNKNOWN_OWNER") {
                analysis.blocked_unknown_components += 1;
            }
            if blockers.contains("OTHER_UNREACHABLE_COMPONENT") {
                analysis.blocked_other_no_components += 1;
            }
        }
        let component_id = format!("A-COMP-{component_ordinal:04}");
        let blocker_text = blockers.into_iter().collect::<Vec<_>>().join(";");
        for index in component {
            let row = &mut rows[index];
            row.a_candidate = true;
            row.a_component = component_id.clone();
            row.a_closure = if closed { "PROVEN" } else { "BLOCKED" }.to_string();
            row.a_blocker = blocker_text.clone();
        }
    }
    analysis
}

fn analyze_b_candidates(rows: &mut [SymbolRow]) -> BCandidateAnalysis {
    let mut groups: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
    for (index, row) in rows.iter().enumerate() {
        if row.body_fingerprint == "none" {
            continue;
        }
        groups
            .entry((
                row.signature_shape_fingerprint.clone(),
                row.body_fingerprint.clone(),
            ))
            .or_default()
            .push(index);
    }
    let mut analysis = BCandidateAnalysis::default();
    for indices in groups.values().filter(|indices| indices.len() > 1) {
        analysis.fingerprint_groups += 1;
        for &index in indices {
            if matches!(rows[index].context.as_str(), "Test" | "Tool") {
                continue;
            }
            analysis.candidate_symbols += 1;
            rows[index].b_candidate = true;
            let counterpart = indices
                .iter()
                .copied()
                .filter(|&candidate| {
                    candidate != index
                        && !matches!(rows[candidate].context.as_str(), "Test" | "Tool")
                })
                .max_by_key(|&candidate| {
                    (rows[candidate].productive_reachability == "YES") as usize * 2
                        + (rows[candidate].context == rows[index].context) as usize
                });
            let Some(counterpart) = counterpart else {
                continue;
            };
            rows[index].b_counterpart = rows[counterpart].qualified_symbol.clone();
            rows[index].b_evidence =
                "IDENTICAL_SIGNATURE_SHAPE_AND_BODY".to_string();
            if rows[index].relative_source_path != rows[counterpart].relative_source_path {
                analysis.cross_file_candidate_symbols += 1;
            }
            if independence_sensitive_pair(&rows[index], &rows[counterpart]) {
                rows[index].b_blocker = "INDEPENDENCE_NOT_PROVEN".to_string();
                analysis.independence_blocked_symbols += 1;
            } else {
                rows[index].b_blocker = "RESPONSIBILITY_EQUIVALENCE_NOT_PROVEN".to_string();
            }
        }
    }

    let mut by_name: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, row) in rows.iter().enumerate() {
        by_name.entry(row.simple_name.clone()).or_default().push(index);
    }
    for index in 0..rows.len() {
        let Some(base_name) = historical_counterpart_name(&rows[index].simple_name) else {
            continue;
        };
        if matches!(rows[index].context.as_str(), "Test" | "Tool") {
            continue;
        }
        let Some(candidates) = by_name.get(base_name) else {
            continue;
        };
        let matching = candidates
            .iter()
            .copied()
            .filter(|&candidate| {
                candidate != index
                    && rows[candidate].productive_reachability == "YES"
                    && rows[candidate].signature_shape_fingerprint
                        == rows[index].signature_shape_fingerprint
                    && rows[candidate].body_fingerprint == rows[index].body_fingerprint
            })
            .collect::<Vec<_>>();
        if matching.len() == 1 {
            let target = matching[0];
            if !rows[index].b_candidate {
                analysis.candidate_symbols += 1;
            }
            if rows[index].relative_source_path != rows[target].relative_source_path
                && rows[index].b_counterpart.is_empty()
            {
                analysis.cross_file_candidate_symbols += 1;
            }
            rows[index].b_candidate = true;
            rows[index].b_counterpart = rows[target].qualified_symbol.clone();
            if independence_sensitive_pair(&rows[index], &rows[target]) {
                rows[index].b_evidence =
                    "IDENTICAL_LEGACY_SIGNATURE_SHAPE_AND_BODY".to_string();
                rows[index].b_blocker = "INDEPENDENCE_NOT_PROVEN".to_string();
                analysis.independence_blocked_symbols += 1;
            } else {
                rows[index].b_evidence = "PROVEN_LEGACY_EXACT_REPLACEMENT".to_string();
                rows[index].b_blocker.clear();
                analysis.proven_symbols += 1;
            }
        }
    }
    analysis
}

fn independence_sensitive_pair(left: &SymbolRow, right: &SymbolRow) -> bool {
    left.context == "Governance / Gate"
        || right.context == "Governance / Gate"
        || left.relative_source_path.contains("/build_gate/")
        || right.relative_source_path.contains("/build_gate/")
        || left.relative_source_path.contains("/audit_only/")
        || right.relative_source_path.contains("/audit_only/")
}

fn historical_counterpart_name(name: &str) -> Option<&str> {
    name.strip_prefix("legacy_")
        .or_else(|| name.strip_suffix("_legacy"))
        .or_else(|| name.strip_prefix("obsolete_"))
        .or_else(|| name.strip_prefix("deprecated_"))
        .or_else(|| name.strip_prefix("shadow_"))
        .or_else(|| name.strip_suffix("_shadow"))
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum ReferenceResolution {
    Resolved(usize),
    Ambiguous(Vec<usize>),
}

fn resolve_reference_site(
    rows: &[SymbolRow],
    candidates: &[usize],
    owner: Option<usize>,
    file: &str,
    tokens: &[Token],
    token_index: usize,
) -> ReferenceResolution {
    let previous_is_member = token_index > 0 && tokens[token_index - 1].text == ".";
    if previous_is_member
        && token_index >= 2
        && tokens[token_index - 2].text == "self"
        && let Some(owner_index) = owner
        && let Some((owner_prefix, _)) = rows[owner_index].qualified_symbol.rsplit_once("::")
    {
        let matching = candidates
            .iter()
            .copied()
            .filter(|&index| {
                rows[index]
                    .qualified_symbol
                    .rsplit_once("::")
                    .is_some_and(|(prefix, _)| prefix == owner_prefix)
                    && matches!(
                        rows[index].declaration_kind.as_str(),
                        "inherent_method" | "trait_impl_method" | "trait_method"
                    )
            })
            .collect::<Vec<_>>();
        if matching.len() == 1 {
            return ReferenceResolution::Resolved(matching[0]);
        }
    }
    if candidates.len() == 1 && !previous_is_member {
        return ReferenceResolution::Resolved(candidates[0]);
    }

    if let Some(suffix) = explicit_path_suffix(tokens, token_index) {
        let matching = candidates
            .iter()
            .copied()
            .filter(|&index| {
                rows[index].qualified_symbol == suffix
                    || rows[index]
                        .qualified_symbol
                        .ends_with(&format!("::{suffix}"))
            })
            .collect::<Vec<_>>();
        if matching.len() == 1 {
            return ReferenceResolution::Resolved(matching[0]);
        }
    }

    if !previous_is_member {
        let site_scope = owner
            .map(|index| rows[index].module_scope.clone())
            .unwrap_or_else(|| module_path_from_file(file));
        let same_scope = candidates
            .iter()
            .copied()
            .filter(|&index| {
                rows[index].module_scope == site_scope
                    && matches!(rows[index].declaration_kind.as_str(), "function" | "nested_function")
            })
            .collect::<Vec<_>>();
        if same_scope.len() == 1 {
            return ReferenceResolution::Resolved(same_scope[0]);
        }
    }

    ReferenceResolution::Ambiguous(candidates.to_vec())
}

fn explicit_path_suffix(tokens: &[Token], token_index: usize) -> Option<String> {
    if token_index < 2 || tokens[token_index - 1].text != "::" {
        return None;
    }
    let mut segments = vec![tokens[token_index].text.clone()];
    let mut cursor = token_index;
    while cursor >= 2
        && tokens[cursor - 1].text == "::"
        && is_identifier_token(tokens.get(cursor - 2))
    {
        segments.push(tokens[cursor - 2].text.clone());
        cursor -= 2;
    }
    segments.reverse();
    while matches!(segments.first().map(String::as_str), Some("crate" | "self" | "super")) {
        segments.remove(0);
    }
    (segments.len() > 1).then(|| segments.join("::"))
}

fn is_call_reference(tokens: &[Token], token_index: usize) -> bool {
    tokens
        .get(token_index + 1)
        .is_some_and(|token| token.text == "(")
}

fn is_value_binding_reference(tokens: &[Token], token_index: usize) -> bool {
    let previous = token_index
        .checked_sub(1)
        .and_then(|index| tokens.get(index))
        .map(|token| token.text.as_str());
    let next = tokens.get(token_index + 1).map(|token| token.text.as_str());
    if matches!(previous, Some(".")) || matches!(next, Some("::" | ":")) {
        return false;
    }
    matches!(
        previous,
        Some("(" | "," | "=" | "[" | "{" | "=>" | "return" | ":" | "|")
    ) || matches!(next, Some(")" | "," | ";" | "]" | "}" | "?"))
}

fn direct_public_lib_module_names(files: &[SourceFile]) -> BTreeSet<String> {
    let Some(lib) = files.iter().find(|file| file.relative == "src/lib.rs") else {
        return BTreeSet::new();
    };
    let tokens = tokenize(&lib.source);
    let mut names = BTreeSet::new();
    let mut brace_depth = 0isize;
    let mut index = 0usize;
    while index + 2 < tokens.len() {
        match tokens[index].text.as_str() {
            "{" => {
                brace_depth += 1;
                index += 1;
                continue;
            }
            "}" => {
                brace_depth -= 1;
                index += 1;
                continue;
            }
            "pub" if brace_depth == 0
                && tokens[index + 1].text == "mod"
                && is_identifier_token(tokens.get(index + 2)) =>
            {
                names.insert(tokens[index + 2].text.clone());
            }
            _ => {}
        }
        index += 1;
    }
    names
}

fn is_direct_public_api_scope(row: &SymbolRow, public_module_names: &BTreeSet<String>) -> bool {
    if row.relative_source_path == "src/lib.rs" {
        return row.module_scope == "lib";
    }
    let relative = row.relative_source_path.trim_start_matches("src/");
    let first_component = relative.split('/').next().unwrap_or_default();
    let first_stem = first_component.trim_end_matches(".rs");
    let file_stem = Path::new(&row.relative_source_path)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    let exported_name = if public_module_names.contains(first_stem) {
        first_stem
    } else if public_module_names.contains(file_stem) {
        file_stem
    } else {
        return false;
    };
    let scope_segments = row.module_scope.split("::").collect::<Vec<_>>();
    scope_segments.last().copied() == Some(exported_name)
        && (scope_segments.len() == 1
            || row.relative_source_path.ends_with(&format!("/{exported_name}.rs")))
}

#[cfg(test)]
fn closed_unreachable_removal_components(
    rows: &[SymbolRow],
    reference_owners: &[BTreeSet<Option<usize>>],
) -> BTreeMap<usize, usize> {
    let candidates = rows
        .iter()
        .enumerate()
        .filter_map(|(index, row)| removal_component_candidate(row).then_some(index))
        .collect::<BTreeSet<_>>();
    let mut adjacency: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); rows.len()];
    for &target in &candidates {
        for owner in &reference_owners[target] {
            let Some(owner) = *owner else {
                continue;
            };
            if owner != target && candidates.contains(&owner) {
                adjacency[target].insert(owner);
                adjacency[owner].insert(target);
            }
        }
    }

    let mut seen = BTreeSet::new();
    let mut removable = BTreeMap::new();
    for &start in &candidates {
        if !seen.insert(start) {
            continue;
        }
        let mut component = Vec::new();
        let mut queue = VecDeque::from([start]);
        while let Some(index) = queue.pop_front() {
            component.push(index);
            for &next in &adjacency[index] {
                if seen.insert(next) {
                    queue.push_back(next);
                }
            }
        }
        let component_set = component.iter().copied().collect::<BTreeSet<_>>();
        let closed = component.iter().all(|&target| {
            reference_owners[target].iter().all(|owner| {
                owner.as_ref().is_some_and(|owner_index| component_set.contains(owner_index))
            })
        });
        if closed {
            let size = component.len();
            for index in component {
                removable.insert(index, size);
            }
        }
    }
    removable
}

fn removal_component_candidate(row: &SymbolRow) -> bool {
    row.productive_reachability == "NO"
        && row.source_visibility == "private"
        && row.declaration_kind == "function"
        && row.ambiguous_references == 0
        && !row.has_cfg_or_export_attr
}

fn classify_architecture(
    rows: &mut [SymbolRow],
    relationships: &RepositoryRelationships,
) -> ArchCounts {
    let counterpart_index = build_counterpart_index(rows);
    let mut counts = ArchCounts::default();
    for index in 0..rows.len() {
        let (class, rule, evidence, authority, merge_target, reason) =
            classify_row(index, rows, &counterpart_index, relationships);
        let row = &mut rows[index];
        row.class = class.to_string();
        row.class_rule = rule.to_string();
        row.evidence = evidence.to_string();
        row.authority = authority.to_string();
        row.merge_target = merge_target;
        row.reason = reason;
        match row.class.as_str() {
            "A" => counts.a += 1,
            "B" => counts.b += 1,
            "C" => counts.c += 1,
            "D" => counts.d += 1,
            _ => counts.open += 1,
        }
    }
    counts
}

fn inventory_diagnostics(
    rows: &[SymbolRow],
    relationships: &RepositoryRelationships,
) -> InventoryDiagnostics {
    let mut diagnostics = InventoryDiagnostics {
        duplicate_simple_name_groups: relationships.duplicate_simple_name_groups,
        duplicate_simple_name_symbols: relationships.duplicate_simple_name_symbols,
        resolved_reference_sites: relationships.resolved_reference_sites,
        ambiguous_reference_sites: relationships.ambiguous_reference_sites,
        lexical_only_sites: relationships.lexical_only_sites,
        include_bound_reference_sites: relationships.include_bound_reference_sites,
        macro_bound_reference_sites: relationships.macro_bound_reference_sites,
        self_method_resolved_sites: relationships.self_method_resolved_sites,
        public_api_roots: relationships.public_api_roots,
        uncertain_public_roots: relationships.uncertain_public_roots,
        entry_roots: relationships.entry_roots,
        a_candidate_symbols: relationships.a_candidate_symbols,
        a_candidate_components: relationships.a_candidate_components,
        a_closed_components: relationships.a_closed_components,
        a_blocked_ownerless_components: relationships.a_blocked_ownerless_components,
        a_blocked_productive_components: relationships.a_blocked_productive_components,
        a_blocked_test_components: relationships.a_blocked_test_components,
        a_blocked_tool_components: relationships.a_blocked_tool_components,
        a_blocked_unknown_components: relationships.a_blocked_unknown_components,
        a_blocked_other_no_components: relationships.a_blocked_other_no_components,
        b_fingerprint_groups: relationships.b_fingerprint_groups,
        b_candidate_symbols: relationships.b_candidate_symbols,
        b_cross_file_candidate_symbols: relationships.b_cross_file_candidate_symbols,
        b_proven_symbols: relationships.b_proven_symbols,
        b_independence_blocked_symbols: relationships.b_independence_blocked_symbols,
        ..InventoryDiagnostics::default()
    };
    for row in rows {
        match row.productive_reachability.as_str() {
            "YES" => diagnostics.reachability_yes += 1,
            "NO" => diagnostics.reachability_no += 1,
            "UNKNOWN" => diagnostics.reachability_unknown += 1,
            "ONLY_TEST" => diagnostics.reachability_only_test += 1,
            "ONLY_TOOL" => diagnostics.reachability_only_tool += 1,
            _ => {}
        }
        if row.class == "OFFEN" {
            match row.productive_reachability.as_str() {
                "YES" => diagnostics.open_yes += 1,
                "NO" => diagnostics.open_no += 1,
                "UNKNOWN" => diagnostics.open_unknown += 1,
                "ONLY_TEST" => diagnostics.open_only_test += 1,
                "ONLY_TOOL" => diagnostics.open_only_tool += 1,
                _ => {}
            }
        }
        if row.class == "D" {
            match row.class_rule.as_str() {
                "D.ENTRYPOINT" => diagnostics.d_entrypoint += 1,
                "D.WRITER_AUTHORITY_SYMBOL" => diagnostics.d_writer_authority += 1,
                "D.SELECTION_AUTHORITY_SYMBOL" => diagnostics.d_selection_authority += 1,
                "D.CANONICAL_AUTHORITY_SYMBOL" => diagnostics.d_canonical_authority += 1,
                "D.GOVERNANCE_GATE_SYMBOL" => diagnostics.d_governance_gate += 1,
                _ => {}
            }
        }
    }
    diagnostics
}

fn validate_inventory_diagnostics(
    row_count: usize,
    counts: ArchCounts,
    diagnostics: InventoryDiagnostics,
) -> AppResult<()> {
    let reachability_total = diagnostics.reachability_yes
        + diagnostics.reachability_no
        + diagnostics.reachability_unknown
        + diagnostics.reachability_only_test
        + diagnostics.reachability_only_tool;
    if reachability_total != row_count {
        return Err(anyhow!(
            "ESI-3 reachability balance failed: rows={row_count} reachability={reachability_total}"
        ));
    }
    let open_total = diagnostics.open_yes
        + diagnostics.open_no
        + diagnostics.open_unknown
        + diagnostics.open_only_test
        + diagnostics.open_only_tool;
    if open_total != counts.open {
        return Err(anyhow!(
            "ARCH-4 OFFEN diagnostic balance failed: class={} diagnostic={open_total}",
            counts.open
        ));
    }
    let d_total = diagnostics.d_entrypoint
        + diagnostics.d_writer_authority
        + diagnostics.d_selection_authority
        + diagnostics.d_canonical_authority
        + diagnostics.d_governance_gate;
    if d_total != counts.d {
        return Err(anyhow!(
            "ARCH-4 D diagnostic balance failed: class={} diagnostic={d_total}",
            counts.d
        ));
    }
    Ok(())
}

fn build_counterpart_index(rows: &[SymbolRow]) -> BTreeMap<(String, String), usize> {
    let mut map = BTreeMap::new();
    for (index, row) in rows.iter().enumerate() {
        map.entry((row.relative_source_path.clone(), row.simple_name.clone()))
            .or_insert(index);
    }
    map
}

fn classify_row(
    index: usize,
    rows: &[SymbolRow],
    counterpart_index: &BTreeMap<(String, String), usize>,
    relationships: &RepositoryRelationships,
) -> (&'static str, &'static str, &'static str, &'static str, String, String) {
    let row = &rows[index];
    let simple = row.simple_name.to_ascii_lowercase();

    if row.productive_reachability == "YES" && row.simple_name == "main" {
        return (
            "D",
            "D.ENTRYPOINT",
            "PROVEN_RULE",
            "ENTRYPOINT",
            String::new(),
            "Produktiver/Launcher-Einstiegspunkt ist im formalen Root-Set gebunden.".to_string(),
        );
    }
    if row.productive_reachability == "YES" {
        if explicit_writer_authority_symbol(&simple) {
            return (
                "D",
                "D.WRITER_AUTHORITY_SYMBOL",
                "PROVEN_SYMBOL_RULE",
                "WRITER_AUTHORITY",
                String::new(),
                "Der Symbolname selbst traegt explizite Authorize-/Authorized-/Writer-Authority-/Owner-Semantik; ein Modul- oder Dateiname allein reicht in ARCH-4 nicht aus.".to_string(),
            );
        }
        if simple.contains("selection_authority") || simple.contains("selection_owner") {
            return (
                "D",
                "D.SELECTION_AUTHORITY_SYMBOL",
                "PROVEN_SYMBOL_RULE",
                "SELECTION_AUTHORITY",
                String::new(),
                "Der Symbolname selbst traegt explizite Selection-Authority/Owner-Semantik.".to_string(),
            );
        }
        if simple.contains("canonical_authority") || simple.contains("canonical_owner") {
            return (
                "D",
                "D.CANONICAL_AUTHORITY_SYMBOL",
                "PROVEN_SYMBOL_RULE",
                "CANONICAL_AUTHORITY",
                String::new(),
                "Der Symbolname selbst traegt explizite Canonical-Authority/Owner-Semantik.".to_string(),
            );
        }
        if row.context == "Governance / Gate"
            && (simple.starts_with("validate_")
                || simple.starts_with("verify_")
                || simple.contains("_validate_")
                || simple.contains("_verify_"))
        {
            return (
                "D",
                "D.GOVERNANCE_GATE_SYMBOL",
                "PROVEN_SYMBOL_RULE",
                "GOVERNANCE_GATE",
                String::new(),
                "Erreichbares Governance-Symbol benennt selbst Validate-/Verify-Verantwortung; Gate-/Modulpfade allein reichen in ARCH-4 nicht aus.".to_string(),
            );
        }
    }

    if let Some(&component_size) = relationships.removable_component_size.get(&index) {
        if row.resolved_references == 0
            && row.ambiguous_references == 0
            && row.incoming_calls == 0
            && row.value_bindings == 0
        {
            return (
                "A",
                "A.PRIVATE_UNREFERENCED_PROVEN",
                "PROVEN_CLOSED_COMPONENT",
                "NONE",
                String::new(),
                "Private freie Funktion ist im gesamten ESI-3-Scope ohne aufgeloeste oder mehrdeutige Referenz, Call-/Value-Binding-Kante, cfg- oder Exportattribut und Teil einer geschlossenen unerreichbaren Removal-Komponente.".to_string(),
            );
        }
        return (
            "A",
            "A.CLOSED_UNREACHABLE_COMPONENT_PROVEN",
            "PROVEN_CLOSED_COMPONENT",
            "NONE",
            String::new(),
            format!(
                "Private freie Funktion liegt in einer geschlossenen produktiv unerreichbaren Removal-Komponente mit {component_size} Symbol(en); alle eindeutig aufgeloesten eingehenden Referenzen stammen aus derselben Komponente und es existiert keine mehrdeutige Referenz."
            ),
        );
    }

    if row.b_evidence == "PROVEN_LEGACY_EXACT_REPLACEMENT"
        && row.b_blocker.is_empty()
        && let Some((target, rule)) = legacy_counterpart(row, rows, counterpart_index)
    {
        return (
            "B",
            rule,
            "PROVEN_RULE",
            "NONE",
            target.clone(),
            format!("Explizit historisch benannter, noch gebundener Pfad besitzt deterministisch gefundenes Gegenstueck {target}."),
        );
    }

    match row.productive_reachability.as_str() {
        "YES" => (
            "C",
            "C.REACHABLE_SUPPORT",
            "PROVEN_RULE",
            "NONE",
            String::new(),
            "Symbol ist vom ESI-3-Root-/Referenzgraphen produktiv erreichbar und besitzt keine explizite ARCH-4-D-Authorityregel.".to_string(),
        ),
        "ONLY_TEST" if row.has_cfg_or_export_attr
            || row.simple_name.starts_with("test_")
            || row.resolved_references > 0
            || row.ambiguous_references > 0
            || row.incoming_calls > 0
            || row.value_bindings > 0 => (
            "C",
            "C.TEST_SUPPORT",
            "PROVEN_RULE",
            "NONE",
            String::new(),
            "Symbol ist explizit testgebundene Infrastruktur.".to_string(),
        ),
        "ONLY_TOOL" if row.simple_name == "main"
            || row.source_visibility.starts_with("pub")
            || row.resolved_references > 0
            || row.ambiguous_references > 0
            || row.incoming_calls > 0
            || row.value_bindings > 0 => (
            "C",
            "C.TOOL_SUPPORT",
            "PROVEN_RULE",
            "NONE",
            String::new(),
            "Symbol ist gebundener Tool-Einstieg bzw. exportierte Tool-Unterstuetzung.".to_string(),
        ),
        _ => (
            "OFFEN",
            "OFFEN.INSUFFICIENT_PROOF",
            "UNKNOWN",
            "UNKNOWN",
            String::new(),
            "ARCH-4 besitzt fuer dieses Symbol keinen hinreichenden A/B/C/D-Beweis; fail-closed bleibt OFFEN.".to_string(),
        ),
    }
}

fn explicit_writer_authority_symbol(simple: &str) -> bool {
    simple.contains("authorized")
        || simple.starts_with("authorize_")
        || simple.contains("_authorize_")
        || simple.starts_with("authorizes_")
        || simple.contains("_authorizes_")
        || simple.contains("write_authority")
        || simple.contains("writer_authority")
        || simple.contains("writer_owner")
}

fn legacy_counterpart(
    row: &SymbolRow,
    rows: &[SymbolRow],
    counterpart_index: &BTreeMap<(String, String), usize>,
) -> Option<(String, &'static str)> {
    let name = row.simple_name.as_str();
    let candidates = [
        name.strip_prefix("legacy_").map(|value| (value, "B.PARALLEL_LEGACY_PATH")),
        name.strip_suffix("_legacy").map(|value| (value, "B.PARALLEL_LEGACY_PATH")),
        name.strip_prefix("obsolete_").map(|value| (value, "B.OBSOLETE_PARALLEL_PATH")),
        name.strip_prefix("deprecated_").map(|value| (value, "B.DEPRECATED_PARALLEL_PATH")),
        name.strip_prefix("shadow_").map(|value| (value, "B.SHADOW_PARALLEL_PATH")),
        name.strip_suffix("_shadow").map(|value| (value, "B.SHADOW_PARALLEL_PATH")),
    ];
    for candidate in candidates.into_iter().flatten() {
        let same_file = counterpart_index
            .get(&(row.relative_source_path.clone(), candidate.0.to_string()))
            .copied();
        let mut matching = rows
            .iter()
            .enumerate()
            .filter_map(|(index, target)| {
                (target.simple_name == candidate.0
                    && target.productive_reachability == "YES"
                    && !matches!(target.context.as_str(), "Test" | "Tool")
                    && target.signature_shape_fingerprint == row.signature_shape_fingerprint
                    && target.body_fingerprint == row.body_fingerprint)
                    .then_some(index)
            })
            .collect::<Vec<_>>();
        matching.sort_by_key(|index| (Some(*index) != same_file) as usize);
        matching.dedup();
        if matching.len() == 1 {
            return Some((rows[matching[0]].qualified_symbol.clone(), candidate.1));
        }
    }
    None
}

fn read_measurement_projection(
    project_root: &Path,
    scope: InventoryScope<'_>,
    run_id: &str,
    final_exit: i32,
) -> AppResult<MeasurementProjection> {
    match scope {
        InventoryScope::Project(project) => {
            let path = project_root.join(format!("{project}.canonical_measurement_snapshot_v1.json"));
            if !path.is_file() {
                if final_exit == 0 {
                    return Err(anyhow!("Inventory successful project run is missing CMS-1 snapshot: {}", path.display()));
                }
                return Ok(MeasurementProjection {
                    state: "NOT_AVAILABLE_FAILED_RUN",
                    metrics: None,
                    project_count: 0,
                    corpus_manifest_run_id: None,
                });
            }
            let metrics = parse_project_measurement(&path, project, run_id)?;
            Ok(MeasurementProjection {
                state: "BOUND_CURRENT_RUN",
                metrics: Some(metrics),
                project_count: 1,
                corpus_manifest_run_id: None,
            })
        }
        InventoryScope::Corpus => {
            let manifest = read_corpus_run_manifest(&project_root.join(CORPUS_RUN_MANIFEST_FILE))
                .map_err(|error| anyhow!("Inventory corpus manifest not readable: {error}"))?;
            if final_exit == 0
                && (!manifest.completed
                    || !manifest.full_corpus
                    || manifest.expected_projects.len() != manifest.expected_project_count
                    || manifest.projects.len() != manifest.expected_project_count)
            {
                return Err(anyhow!("Inventory corpus CMS-1 binding rejected incomplete/stale corpus manifest"));
            }
            if final_exit != 0 {
                return Ok(MeasurementProjection {
                    state: "NOT_AVAILABLE_FAILED_RUN",
                    metrics: None,
                    project_count: 0,
                    corpus_manifest_run_id: Some(manifest.run_id.clone()),
                });
            }
            let mut aggregate: Option<NativeMetrics> = None;
            let mut seen = BTreeSet::new();
            for project in &manifest.expected_projects {
                if !seen.insert(project.clone()) {
                    return Err(anyhow!("Inventory corpus manifest contains duplicate project {project}"));
                }
                let path = project_root.join(format!("{project}.canonical_measurement_snapshot_v1.json"));
                let metrics = parse_project_measurement(&path, project, run_id)?;
                if let Some(total) = &mut aggregate {
                    if total.producer_binding != metrics.producer_binding
                        || total.producer_version != metrics.producer_version
                        || total.producer_source_signature != metrics.producer_source_signature
                        || total.producer_binary_fingerprint != metrics.producer_binary_fingerprint
                    {
                        return Err(anyhow!("Inventory corpus CMS-1 producer binding differs across projects"));
                    }
                    total.checked_add(&metrics)?;
                } else {
                    aggregate = Some(NativeMetrics::zero_with_binding(
                        metrics.producer_binding.clone(),
                        metrics.producer_version.clone(),
                        metrics.producer_source_signature.clone(),
                        metrics.producer_binary_fingerprint.clone(),
                    ));
                    aggregate.as_mut().expect("aggregate initialized").checked_add(&metrics)?;
                }
            }
            let metrics = aggregate.ok_or_else(|| anyhow!("Inventory corpus CMS-1 has no projects"))?;
            if metrics.canonical_events != metrics.retained.checked_add(metrics.removed).ok_or_else(|| anyhow!("Inventory CMS-1 retained+removed overflow"))? {
                return Err(anyhow!("Inventory corpus CMS-1 canonical != retained + removed"));
            }
            Ok(MeasurementProjection {
                state: "BOUND_CURRENT_CORPUS",
                metrics: Some(metrics),
                project_count: manifest.expected_project_count,
                corpus_manifest_run_id: Some(manifest.run_id.clone()),
            })
        }
    }
}

fn parse_project_measurement(path: &Path, project: &str, run_id: &str) -> AppResult<NativeMetrics> {
    let text = fs::read_to_string(path).map_err(|error| anyhow!("Inventory CMS-1 unreadable {}: {error}", path.display()))?;
    require_json_string(&text, "measurement_contract_version", CMS_CONTRACT)?;
    require_json_string(&text, "run_id", run_id)?;
    let expected = format!("\"expected_projects\": [\"{}\"]", json_escape_local(project));
    let measured = format!("\"measured_projects\": [\"{}\"]", json_escape_local(project));
    if !text.contains(&expected) || !text.contains(&measured) {
        return Err(anyhow!("Inventory CMS-1 snapshot is not bound to project {project}"));
    }
    let binding = json_string_field_local(&text, "binding_state").unwrap_or_else(|| "unknown".to_string());
    if binding != "bound" {
        return Err(anyhow!("Inventory CMS-1 producer is not bound for {project}: {binding}"));
    }
    let version = json_optional_string_field_local(&text, "version").unwrap_or_default();
    let source_signature = json_optional_string_field_local(&text, "source_signature").unwrap_or_default();
    let binary_fingerprint = json_optional_string_field_local(&text, "binary_fingerprint").unwrap_or_default();
    if version.is_empty() || source_signature.is_empty() || binary_fingerprint.is_empty() {
        return Err(anyhow!("Inventory CMS-1 bound producer is incomplete for {project}"));
    }
    let metrics = NativeMetrics {
        blockers: metric_count(&text, "final_findings.blocker.count")?,
        warnings: metric_count(&text, "final_findings.warning.count")?,
        hints: metric_count(&text, "final_findings.hint.count")?,
        canonical_events: metric_count(&text, "source.canonical_event.count")?,
        retained: metric_count(&text, "source.selection.retained.count")?,
        removed: metric_count(&text, "source.selection.removed.count")?,
        producer_binding: binding,
        producer_version: version,
        producer_source_signature: source_signature,
        producer_binary_fingerprint: binary_fingerprint,
    };
    if metrics.canonical_events != metrics.retained.checked_add(metrics.removed).ok_or_else(|| anyhow!("Inventory project CMS-1 retained+removed overflow"))? {
        return Err(anyhow!("Inventory project CMS-1 canonical != retained + removed for {project}"));
    }
    Ok(metrics)
}

fn require_json_string(text: &str, key: &str, expected: &str) -> AppResult<()> {
    let actual = json_string_field_local(text, key)
        .ok_or_else(|| anyhow!("Inventory generated JSON field {key} missing"))?;
    if actual != expected {
        return Err(anyhow!("Inventory JSON field {key} expected {expected:?}, got {actual:?}"));
    }
    Ok(())
}

fn json_string_field_local(text: &str, key: &str) -> Option<String> {
    let prefix = format!("\"{key}\": \"");
    text.lines().find_map(|line| {
        let trimmed = line.trim();
        let tail = trimmed.strip_prefix(&prefix)?;
        let tail = tail.strip_suffix(',').unwrap_or(tail);
        let value = tail.strip_suffix('"')?;
        if value.contains('\\') {
            return None;
        }
        Some(value.to_string())
    })
}

fn json_optional_string_field_local(text: &str, key: &str) -> Option<String> {
    json_string_field_local(text, key)
}

fn metric_count(text: &str, metric_id: &str) -> AppResult<u64> {
    let marker = format!("\"metric_id\": \"{metric_id}\"");
    let start = text.find(&marker).ok_or_else(|| anyhow!("Inventory CMS-1 metric missing: {metric_id}"))?;
    let tail = &text[start..];
    let count_marker = "\"count\":";
    let count_start = tail.find(count_marker).ok_or_else(|| anyhow!("Inventory CMS-1 count missing: {metric_id}"))? + count_marker.len();
    let digits = tail[count_start..]
        .trim_start()
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    digits.parse::<u64>().map_err(|error| anyhow!("Inventory CMS-1 invalid count for {metric_id}: {error}"))
}

fn input_fingerprint(project_root: &Path, scope: InventoryScope<'_>) -> AppResult<String> {
    let projects = match scope {
        InventoryScope::Project(project) => vec![project.to_string()],
        InventoryScope::Corpus => {
            let manifest = read_corpus_run_manifest(&project_root.join(CORPUS_RUN_MANIFEST_FILE))
                .map_err(|error| anyhow!("Inventory input fingerprint manifest unreadable: {error}"))?;
            manifest.expected_projects
        }
    };
    let mut canonical = String::new();
    for project in projects {
        for extension in ["wav", "mid"] {
            let path = project_root.join(format!("{project}.{extension}"));
            let fingerprint = if path.is_file() {
                file_fnv1a64(&path)?
            } else {
                "missing".to_string()
            };
            writeln!(canonical, "{project}.{extension}|{fingerprint}").expect("String write cannot fail");
        }
    }
    Ok(format!("sha256:{}", sha256::hex(canonical.as_bytes())))
}

fn render_inventory_json(model: &InventoryModel) -> String {
    let mut out = String::new();
    writeln!(out, "{{").expect("String write cannot fail");
    writeln!(out, "  \"schema_version\": 3,").expect("String write cannot fail");
    writeln!(out, "  \"inventory_contract_version\": \"{ESI_CONTRACT}\",").expect("String write cannot fail");
    writeln!(out, "  \"architecture_contract_version\": \"{ARCH_CONTRACT}\",").expect("String write cannot fail");
    writeln!(out, "  \"measurement_contract_version\": \"{CMS_CONTRACT}\",").expect("String write cannot fail");
    writeln!(out, "  \"xlsx_contract_version\": \"{XLSX_CONTRACT}\",").expect("String write cannot fail");
    writeln!(out, "  \"version\": \"{}\",", json_escape_local(&model.version)).expect("String write cannot fail");
    writeln!(out, "  \"revision\": \"{}\",", json_escape_local(&model.revision)).expect("String write cannot fail");
    writeln!(out, "  \"take_name\": \"{}\",", json_escape_local(&model.take_name)).expect("String write cannot fail");
    writeln!(out, "  \"run_id\": \"{}\",", json_escape_local(&model.run_id)).expect("String write cannot fail");
    writeln!(out, "  \"run_scope\": \"{}\",", model.scope).expect("String write cannot fail");
    writeln!(out, "  \"scope_project\": \"{}\",", json_escape_local(&model.scope_project)).expect("String write cannot fail");
    writeln!(out, "  \"corpus_manifest_run_id\": \"{}\",", json_escape_local(model.measurement.corpus_manifest_run_id.as_deref().unwrap_or("none"))).expect("String write cannot fail");
    writeln!(out, "  \"source_scope_sha256\": \"{}\",", model.source_scope_sha256).expect("String write cannot fail");
    writeln!(out, "  \"source_file_count\": {},", model.source_file_count).expect("String write cannot fail");
    writeln!(out, "  \"input_fingerprint\": \"{}\",", model.input_fingerprint).expect("String write cannot fail");
    writeln!(out, "  \"binary_fingerprint\": \"{}\",", json_escape_local(&model.binary_fingerprint)).expect("String write cannot fail");
    writeln!(out, "  \"measurement\": {{").expect("String write cannot fail");
    writeln!(out, "    \"state\": \"{}\",", model.measurement.state).expect("String write cannot fail");
    writeln!(out, "    \"project_count\": {},", model.measurement.project_count).expect("String write cannot fail");
    if let Some(metrics) = &model.measurement.metrics {
        writeln!(out, "    \"producer_binding\": \"{}\",", json_escape_local(&metrics.producer_binding)).expect("String write cannot fail");
        writeln!(out, "    \"producer_version\": \"{}\",", json_escape_local(&metrics.producer_version)).expect("String write cannot fail");
        writeln!(out, "    \"producer_source_signature\": \"{}\",", json_escape_local(&metrics.producer_source_signature)).expect("String write cannot fail");
        writeln!(out, "    \"producer_binary_fingerprint\": \"{}\",", json_escape_local(&metrics.producer_binary_fingerprint)).expect("String write cannot fail");
        writeln!(out, "    \"blocker\": {},", metrics.blockers).expect("String write cannot fail");
        writeln!(out, "    \"warning\": {},", metrics.warnings).expect("String write cannot fail");
        writeln!(out, "    \"hint\": {},", metrics.hints).expect("String write cannot fail");
        writeln!(out, "    \"canonical_events\": {},", metrics.canonical_events).expect("String write cannot fail");
        writeln!(out, "    \"retained\": {},", metrics.retained).expect("String write cannot fail");
        writeln!(out, "    \"removed\": {}", metrics.removed).expect("String write cannot fail");
    } else {
        writeln!(out, "    \"producer_binding\": null,").expect("String write cannot fail");
        writeln!(out, "    \"producer_version\": null,").expect("String write cannot fail");
        writeln!(out, "    \"producer_source_signature\": null,").expect("String write cannot fail");
        writeln!(out, "    \"producer_binary_fingerprint\": null,").expect("String write cannot fail");
        writeln!(out, "    \"blocker\": null,").expect("String write cannot fail");
        writeln!(out, "    \"warning\": null,").expect("String write cannot fail");
        writeln!(out, "    \"hint\": null,").expect("String write cannot fail");
        writeln!(out, "    \"canonical_events\": null,").expect("String write cannot fail");
        writeln!(out, "    \"retained\": null,").expect("String write cannot fail");
        writeln!(out, "    \"removed\": null").expect("String write cannot fail");
    }
    writeln!(out, "  }},").expect("String write cannot fail");
    writeln!(out, "  \"architecture_counts\": {{\"total\": {}, \"A\": {}, \"B\": {}, \"C\": {}, \"D\": {}, \"OFFEN\": {}}},", model.counts.total(), model.counts.a, model.counts.b, model.counts.c, model.counts.d, model.counts.open).expect("String write cannot fail");
    writeln!(out, "  \"diagnostics\": {{").expect("String write cannot fail");
    for (key, value, last) in [
        ("duplicate_simple_name_groups", model.diagnostics.duplicate_simple_name_groups, false),
        ("duplicate_simple_name_symbols", model.diagnostics.duplicate_simple_name_symbols, false),
        ("resolved_reference_sites", model.diagnostics.resolved_reference_sites, false),
        ("ambiguous_reference_sites", model.diagnostics.ambiguous_reference_sites, false),
        ("lexical_only_sites", model.diagnostics.lexical_only_sites, false),
        ("include_bound_reference_sites", model.diagnostics.include_bound_reference_sites, false),
        ("macro_bound_reference_sites", model.diagnostics.macro_bound_reference_sites, false),
        ("self_method_resolved_sites", model.diagnostics.self_method_resolved_sites, false),
        ("public_api_roots", model.diagnostics.public_api_roots, false),
        ("uncertain_public_roots", model.diagnostics.uncertain_public_roots, false),
        ("entry_roots", model.diagnostics.entry_roots, false),
        ("reachability_yes", model.diagnostics.reachability_yes, false),
        ("reachability_no", model.diagnostics.reachability_no, false),
        ("reachability_unknown", model.diagnostics.reachability_unknown, false),
        ("reachability_only_test", model.diagnostics.reachability_only_test, false),
        ("reachability_only_tool", model.diagnostics.reachability_only_tool, false),
        ("open_yes", model.diagnostics.open_yes, false),
        ("open_no", model.diagnostics.open_no, false),
        ("open_unknown", model.diagnostics.open_unknown, false),
        ("open_only_test", model.diagnostics.open_only_test, false),
        ("open_only_tool", model.diagnostics.open_only_tool, false),
        ("d_entrypoint", model.diagnostics.d_entrypoint, false),
        ("d_writer_authority", model.diagnostics.d_writer_authority, false),
        ("d_selection_authority", model.diagnostics.d_selection_authority, false),
        ("d_canonical_authority", model.diagnostics.d_canonical_authority, false),
        ("d_governance_gate", model.diagnostics.d_governance_gate, false),
        ("a_candidate_symbols", model.diagnostics.a_candidate_symbols, false),
        ("a_candidate_components", model.diagnostics.a_candidate_components, false),
        ("a_closed_components", model.diagnostics.a_closed_components, false),
        ("a_blocked_ownerless_components", model.diagnostics.a_blocked_ownerless_components, false),
        ("a_blocked_productive_components", model.diagnostics.a_blocked_productive_components, false),
        ("a_blocked_test_components", model.diagnostics.a_blocked_test_components, false),
        ("a_blocked_tool_components", model.diagnostics.a_blocked_tool_components, false),
        ("a_blocked_unknown_components", model.diagnostics.a_blocked_unknown_components, false),
        ("a_blocked_other_no_components", model.diagnostics.a_blocked_other_no_components, false),
        ("b_fingerprint_groups", model.diagnostics.b_fingerprint_groups, false),
        ("b_candidate_symbols", model.diagnostics.b_candidate_symbols, false),
        ("b_cross_file_candidate_symbols", model.diagnostics.b_cross_file_candidate_symbols, false),
        ("b_proven_symbols", model.diagnostics.b_proven_symbols, false),
        ("b_independence_blocked_symbols", model.diagnostics.b_independence_blocked_symbols, true),
    ] {
        writeln!(out, "    \"{key}\": {value}{}", if last { "" } else { "," }).expect("String write cannot fail");
    }
    writeln!(out, "  }},").expect("String write cannot fail");
    writeln!(out, "  \"rows\": [").expect("String write cannot fail");
    for (index, row) in model.rows.iter().enumerate() {
        writeln!(out, "    {{").expect("String write cannot fail");
        for (key, value) in [
            ("class", row.class.as_str()),
            ("class_rule", row.class_rule.as_str()),
            ("evidence", row.evidence.as_str()),
            ("qualified_symbol", row.qualified_symbol.as_str()),
            ("simple_name", row.simple_name.as_str()),
            ("module_scope", row.module_scope.as_str()),
            ("declaration_kind", row.declaration_kind.as_str()),
            ("relative_source_path", row.relative_source_path.as_str()),
        ] {
            writeln!(out, "      \"{key}\": \"{}\",", json_escape_local(value)).expect("String write cannot fail");
        }
        writeln!(out, "      \"start_line\": {},", row.start_line).expect("String write cannot fail");
        writeln!(out, "      \"line_count\": {},", row.line_count).expect("String write cannot fail");
        for (key, value) in [
            ("source_visibility", row.source_visibility.as_str()),
            ("context", row.context.as_str()),
            ("productive_reachability", row.productive_reachability.as_str()),
            ("authority", row.authority.as_str()),
        ] {
            writeln!(out, "      \"{key}\": \"{}\",", json_escape_local(value)).expect("String write cannot fail");
        }
        writeln!(out, "      \"lexical_hits\": {},", row.lexical_hits).expect("String write cannot fail");
        writeln!(out, "      \"resolved_references\": {},", row.resolved_references).expect("String write cannot fail");
        writeln!(out, "      \"ambiguous_references\": {},", row.ambiguous_references).expect("String write cannot fail");
        writeln!(out, "      \"incoming_calls\": {},", row.incoming_calls).expect("String write cannot fail");
        writeln!(out, "      \"value_bindings\": {},", row.value_bindings).expect("String write cannot fail");
        writeln!(out, "      \"sensitive_declaration\": {},", row.has_cfg_or_export_attr).expect("String write cannot fail");
        writeln!(out, "      \"signature_fingerprint\": \"{}\",", json_escape_local(&row.signature_fingerprint)).expect("String write cannot fail");
        writeln!(out, "      \"signature_shape_fingerprint\": \"{}\",", json_escape_local(&row.signature_shape_fingerprint)).expect("String write cannot fail");
        writeln!(out, "      \"body_fingerprint\": \"{}\",", json_escape_local(&row.body_fingerprint)).expect("String write cannot fail");
        writeln!(out, "      \"a_candidate\": {},", row.a_candidate).expect("String write cannot fail");
        writeln!(out, "      \"a_component\": \"{}\",", json_escape_local(&row.a_component)).expect("String write cannot fail");
        writeln!(out, "      \"a_closure\": \"{}\",", json_escape_local(&row.a_closure)).expect("String write cannot fail");
        writeln!(out, "      \"a_blocker\": \"{}\",", json_escape_local(&row.a_blocker)).expect("String write cannot fail");
        writeln!(out, "      \"b_candidate\": {},", row.b_candidate).expect("String write cannot fail");
        writeln!(out, "      \"b_counterpart\": \"{}\",", json_escape_local(&row.b_counterpart)).expect("String write cannot fail");
        writeln!(out, "      \"b_evidence\": \"{}\",", json_escape_local(&row.b_evidence)).expect("String write cannot fail");
        writeln!(out, "      \"b_blocker\": \"{}\",", json_escape_local(&row.b_blocker)).expect("String write cannot fail");
        writeln!(out, "      \"merge_target\": \"{}\",", json_escape_local(&row.merge_target)).expect("String write cannot fail");
        writeln!(out, "      \"reason\": \"{}\"", json_escape_local(&row.reason)).expect("String write cannot fail");
        writeln!(out, "    }}{}", if index + 1 == model.rows.len() { "" } else { "," }).expect("String write cannot fail");
    }
    writeln!(out, "  ]").expect("String write cannot fail");
    writeln!(out, "}}").expect("String write cannot fail");
    out
}

fn validate_inventory_json(path: &Path, model: &InventoryModel) -> AppResult<()> {
    let text = fs::read_to_string(path)?;
    require_json_string(&text, "inventory_contract_version", ESI_CONTRACT)?;
    require_json_string(&text, "architecture_contract_version", ARCH_CONTRACT)?;
    require_json_string(&text, "measurement_contract_version", CMS_CONTRACT)?;
    require_json_string(&text, "xlsx_contract_version", XLSX_CONTRACT)?;
    require_json_string(&text, "run_id", &model.run_id)?;
    if !text.contains("\"schema_version\": 3") {
        return Err(anyhow!("Inventory JSON readback schema_version is not 3"));
    }
    let row_markers = text.matches("\"qualified_symbol\":").count();
    if row_markers != model.rows.len() {
        return Err(anyhow!("Inventory JSON readback row count {} != {}", row_markers, model.rows.len()));
    }
    Ok(())
}

fn write_inventory_xlsx(path: &Path, model: &InventoryModel) -> AppResult<()> {
    let file = File::create(path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    write_zip_entry(&mut zip, options, "[Content_Types].xml", content_types_xml())?;
    write_zip_entry(&mut zip, options, "_rels/.rels", root_rels_xml())?;
    write_zip_entry(&mut zip, options, "docProps/app.xml", app_xml())?;
    write_zip_entry(&mut zip, options, "docProps/core.xml", core_xml(model))?;
    write_zip_entry(&mut zip, options, "xl/workbook.xml", workbook_xml())?;
    write_zip_entry(&mut zip, options, "xl/_rels/workbook.xml.rels", workbook_rels_xml())?;
    write_zip_entry(&mut zip, options, "xl/styles.xml", styles_xml())?;
    write_zip_entry(&mut zip, options, "xl/worksheets/sheet1.xml", worksheet_xml(model))?;
    write_zip_entry(&mut zip, options, "xl/worksheets/_rels/sheet1.xml.rels", sheet_rels_xml())?;
    write_zip_entry(&mut zip, options, "xl/tables/table1.xml", table_xml(model))?;
    zip.finish()?;
    Ok(())
}

fn write_zip_entry<W: Write + io::Seek>(
    zip: &mut ZipWriter<W>,
    options: FileOptions,
    name: &str,
    text: String,
) -> AppResult<()> {
    zip.start_file(name, options)?;
    zip.write_all(text.as_bytes())?;
    Ok(())
}

fn content_types_xml() -> String {
    concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>",
        "<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">",
        "<Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>",
        "<Default Extension=\"xml\" ContentType=\"application/xml\"/>",
        "<Override PartName=\"/xl/workbook.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml\"/>",
        "<Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml\"/>",
        "<Override PartName=\"/xl/styles.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml\"/>",
        "<Override PartName=\"/xl/tables/table1.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml\"/>",
        "<Override PartName=\"/docProps/core.xml\" ContentType=\"application/vnd.openxmlformats-package.core-properties+xml\"/>",
        "<Override PartName=\"/docProps/app.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.extended-properties+xml\"/>",
        "</Types>"
    )
    .to_string()
}

fn root_rels_xml() -> String {
    concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>",
        "<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">",
        "<Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\" Target=\"xl/workbook.xml\"/>",
        "<Relationship Id=\"rId2\" Type=\"http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties\" Target=\"docProps/core.xml\"/>",
        "<Relationship Id=\"rId3\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties\" Target=\"docProps/app.xml\"/>",
        "</Relationships>"
    )
    .to_string()
}

fn workbook_xml() -> String {
    concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>",
        "<workbook xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\">",
        "<sheets><sheet name=\"Inventory\" sheetId=\"1\" r:id=\"rId1\"/></sheets>",
        "</workbook>"
    )
    .to_string()
}

fn workbook_rels_xml() -> String {
    concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>",
        "<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">",
        "<Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet\" Target=\"worksheets/sheet1.xml\"/>",
        "<Relationship Id=\"rId2\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles\" Target=\"styles.xml\"/>",
        "</Relationships>"
    )
    .to_string()
}

fn sheet_rels_xml() -> String {
    concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>",
        "<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">",
        "<Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/table\" Target=\"/xl/tables/table1.xml\"/>",
        "</Relationships>"
    )
    .to_string()
}

fn app_xml() -> String {
    concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>",
        "<Properties xmlns=\"http://schemas.openxmlformats.org/officeDocument/2006/extended-properties\" xmlns:vt=\"http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes\">",
        "<Application>EtuSchlopf</Application><DocSecurity>0</DocSecurity><ScaleCrop>false</ScaleCrop>",
        "<HeadingPairs><vt:vector size=\"2\" baseType=\"variant\"><vt:variant><vt:lpstr>Worksheets</vt:lpstr></vt:variant><vt:variant><vt:i4>1</vt:i4></vt:variant></vt:vector></HeadingPairs>",
        "<TitlesOfParts><vt:vector size=\"1\" baseType=\"lpstr\"><vt:lpstr>Inventory</vt:lpstr></vt:vector></TitlesOfParts>",
        "<Company></Company><LinksUpToDate>false</LinksUpToDate><SharedDoc>false</SharedDoc><HyperlinksChanged>false</HyperlinksChanged><AppVersion>1.0</AppVersion>",
        "</Properties>"
    )
    .to_string()
}

fn core_xml(model: &InventoryModel) -> String {
    format!(
        concat!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>",
            "<cp:coreProperties xmlns:cp=\"http://schemas.openxmlformats.org/package/2006/metadata/core-properties\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:dcterms=\"http://purl.org/dc/terms/\" xmlns:dcmitype=\"http://purl.org/dc/dcmitype/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\">",
            "<dc:title>EtuSchlopf Inventory {}</dc:title><dc:creator>EtuSchlopf</dc:creator><dc:description>{}/{}/{}/{}</dc:description>",
            "</cp:coreProperties>"
        ),
        xml_escape(&model.revision), ESI_CONTRACT, ARCH_CONTRACT, CMS_CONTRACT, XLSX_CONTRACT
    )
}

fn styles_xml() -> String {
    concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>",
        "<styleSheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">",
        "<fonts count=\"3\"><font><sz val=\"10\"/><name val=\"Calibri\"/></font><font><b/><color rgb=\"FFFFFFFF\"/><sz val=\"10\"/><name val=\"Calibri\"/></font><font><b/><sz val=\"10\"/><name val=\"Calibri\"/></font></fonts>",
        "<fills count=\"3\"><fill><patternFill patternType=\"none\"/></fill><fill><patternFill patternType=\"gray125\"/></fill><fill><patternFill patternType=\"solid\"><fgColor rgb=\"FF1F4E78\"/><bgColor indexed=\"64\"/></patternFill></fill></fills>",
        "<borders count=\"1\"><border><left/><right/><top/><bottom/><diagonal/></border></borders>",
        "<cellStyleXfs count=\"1\"><xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\"/></cellStyleXfs>",
        "<cellXfs count=\"3\"><xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\" xfId=\"0\"/><xf numFmtId=\"0\" fontId=\"1\" fillId=\"2\" borderId=\"0\" xfId=\"0\" applyFont=\"1\" applyFill=\"1\"/><xf numFmtId=\"0\" fontId=\"2\" fillId=\"0\" borderId=\"0\" xfId=\"0\" applyFont=\"1\"/></cellXfs>",
        "<cellStyles count=\"1\"><cellStyle name=\"Normal\" xfId=\"0\" builtinId=\"0\"/></cellStyles>",
        "</styleSheet>"
    )
    .to_string()
}

const TABLE_HEADERS: [&str; 31] = [
    "Klasse",
    "Klassenregel",
    "Nachweis",
    "Funktion / Methode",
    "Art",
    "Datei",
    "Startzeile",
    "Zeilenanzahl",
    "Sichtbarkeit",
    "Kontext",
    "Produktive Erreichbarkeit",
    "Authority",
    "Lexical Hits",
    "Resolved References",
    "Ambiguous References",
    "Incoming Calls",
    "Value Bindings",
    "Signatur-Fingerprint",
    "Signatur-Form-Fingerprint",
    "Body-Fingerprint",
    "A Kandidat",
    "A Komponente",
    "A Closure",
    "A Blocker",
    "B Kandidat",
    "B Gegenstück",
    "B Evidence",
    "B Blocker",
    "Ziel / Gegenstück",
    "Grund",
    "Symbol-ID",
];

fn worksheet_xml(model: &InventoryModel) -> String {
    let summary = summary_rows(model);
    let summary_key_column = TABLE_HEADERS.len() + 2;
    let summary_value_column = TABLE_HEADERS.len() + 3;
    let mut out = String::new();
    writeln!(out, "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>").expect("String write cannot fail");
    write!(out, "<worksheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\">").expect("String write cannot fail");
    write!(out, "<sheetViews><sheetView workbookViewId=\"0\"><pane ySplit=\"1\" topLeftCell=\"A2\" activePane=\"bottomLeft\" state=\"frozen\"/></sheetView></sheetViews>").expect("String write cannot fail");
    write!(out, "<sheetFormatPr defaultRowHeight=\"15\"/>").expect("String write cannot fail");
    write!(out, "<cols>").expect("String write cannot fail");
    let widths = [
        9.0, 29.0, 14.0, 44.0, 18.0, 46.0, 11.0, 12.0, 15.0, 19.0, 23.0, 22.0,
        12.0, 19.0, 20.0, 14.0, 14.0, 29.0, 29.0, 29.0, 11.0, 18.0, 14.0, 36.0,
        11.0, 40.0, 34.0, 36.0, 40.0, 64.0, 56.0,
    ];
    for (index, width) in widths.iter().enumerate() {
        write!(out, "<col min=\"{}\" max=\"{}\" width=\"{}\" customWidth=\"1\"/>", index + 1, index + 1, width).expect("String write cannot fail");
    }
    write!(
        out,
        "<col min=\"{}\" max=\"{}\" width=\"28\" customWidth=\"1\"/><col min=\"{}\" max=\"{}\" width=\"52\" customWidth=\"1\"/></cols>",
        summary_key_column,
        summary_key_column,
        summary_value_column,
        summary_value_column
    )
    .expect("String write cannot fail");
    write!(out, "<sheetData>").expect("String write cannot fail");
    write!(out, "<row r=\"1\">").expect("String write cannot fail");
    for (column, header) in TABLE_HEADERS.iter().enumerate() {
        push_text_cell(&mut out, column + 1, 1, header, 1);
    }
    push_text_cell(&mut out, summary_key_column, 1, "EtuSchlopf Inventory", 1);
    push_text_cell(
        &mut out,
        summary_value_column,
        1,
        &format!("{} · {}", model.revision, model.take_name),
        1,
    );
    write!(out, "</row>").expect("String write cannot fail");
    for (offset, row) in model.rows.iter().enumerate() {
        let excel_row = offset + 2;
        write!(out, "<row r=\"{excel_row}\">").expect("String write cannot fail");
        let symbol_id = format!("{}::{}", row.relative_source_path, row.qualified_symbol);
        for (column, value) in [
            row.class.as_str(),
            row.class_rule.as_str(),
            row.evidence.as_str(),
            row.qualified_symbol.as_str(),
            row.declaration_kind.as_str(),
            row.relative_source_path.as_str(),
        ]
        .into_iter()
        .enumerate()
        {
            push_text_cell(&mut out, column + 1, excel_row, value, 0);
        }
        push_number_cell(&mut out, 7, excel_row, row.start_line as u64);
        push_number_cell(&mut out, 8, excel_row, row.line_count as u64);
        for (column, value) in [
            row.source_visibility.as_str(),
            row.context.as_str(),
            row.productive_reachability.as_str(),
            row.authority.as_str(),
        ]
        .into_iter()
        .enumerate()
        {
            push_text_cell(&mut out, column + 9, excel_row, value, 0);
        }
        push_number_cell(&mut out, 13, excel_row, row.lexical_hits as u64);
        push_number_cell(&mut out, 14, excel_row, row.resolved_references as u64);
        push_number_cell(&mut out, 15, excel_row, row.ambiguous_references as u64);
        push_number_cell(&mut out, 16, excel_row, row.incoming_calls as u64);
        push_number_cell(&mut out, 17, excel_row, row.value_bindings as u64);
        push_text_cell(&mut out, 18, excel_row, &row.signature_fingerprint, 0);
        push_text_cell(&mut out, 19, excel_row, &row.signature_shape_fingerprint, 0);
        push_text_cell(&mut out, 20, excel_row, &row.body_fingerprint, 0);
        push_text_cell(&mut out, 21, excel_row, if row.a_candidate { "YES" } else { "NO" }, 0);
        push_text_cell(&mut out, 22, excel_row, &row.a_component, 0);
        push_text_cell(&mut out, 23, excel_row, &row.a_closure, 0);
        push_text_cell(&mut out, 24, excel_row, &row.a_blocker, 0);
        push_text_cell(&mut out, 25, excel_row, if row.b_candidate { "YES" } else { "NO" }, 0);
        push_text_cell(&mut out, 26, excel_row, &row.b_counterpart, 0);
        push_text_cell(&mut out, 27, excel_row, &row.b_evidence, 0);
        push_text_cell(&mut out, 28, excel_row, &row.b_blocker, 0);
        push_text_cell(&mut out, 29, excel_row, &row.merge_target, 0);
        push_text_cell(&mut out, 30, excel_row, &row.reason, 0);
        push_text_cell(&mut out, 31, excel_row, &symbol_id, 0);
        if let Some((label, value)) = summary.get(excel_row - 2) {
            push_text_cell(&mut out, summary_key_column, excel_row, label, 2);
            push_text_cell(&mut out, summary_value_column, excel_row, value, 0);
        }
        write!(out, "</row>").expect("String write cannot fail");
    }
    if summary.len() > model.rows.len() {
        for (offset, (label, value)) in summary.iter().enumerate().skip(model.rows.len()) {
            let excel_row = offset + 2;
            write!(out, "<row r=\"{excel_row}\">").expect("String write cannot fail");
            push_text_cell(&mut out, summary_key_column, excel_row, label, 2);
            push_text_cell(&mut out, summary_value_column, excel_row, value, 0);
            write!(out, "</row>").expect("String write cannot fail");
        }
    }
    write!(out, "</sheetData>").expect("String write cannot fail");
    write!(out, "<tableParts count=\"1\"><tablePart r:id=\"rId1\"/></tableParts></worksheet>").expect("String write cannot fail");
    out
}

fn summary_rows(model: &InventoryModel) -> Vec<(String, String)> {
    let mut rows = vec![
        ("Inventory Contract".to_string(), ESI_CONTRACT.to_string()),
        ("Architecture Contract".to_string(), ARCH_CONTRACT.to_string()),
        ("Measurement Contract".to_string(), CMS_CONTRACT.to_string()),
        ("XLSX Contract".to_string(), XLSX_CONTRACT.to_string()),
        ("Version".to_string(), model.version.clone()),
        ("Revision".to_string(), model.revision.clone()),
        ("Run Scope".to_string(), model.scope.clone()),
        ("Project / Corpus".to_string(), model.scope_project.clone()),
        ("Corpus Manifest Run ID".to_string(), model.measurement.corpus_manifest_run_id.clone().unwrap_or_else(|| "none".to_string())),
        ("Run ID".to_string(), model.run_id.clone()),
        ("Source Scope SHA256".to_string(), model.source_scope_sha256.clone()),
        ("Source Files".to_string(), model.source_file_count.to_string()),
        ("Input Fingerprint".to_string(), model.input_fingerprint.clone()),
        ("Binary Fingerprint".to_string(), model.binary_fingerprint.clone()),
        ("Gesamt".to_string(), model.counts.total().to_string()),
        ("A · Löschbar".to_string(), model.counts.a.to_string()),
        ("A · Kandidatensymbole".to_string(), model.diagnostics.a_candidate_symbols.to_string()),
        ("A · Kandidatenkomponenten".to_string(), model.diagnostics.a_candidate_components.to_string()),
        ("A · Geschlossene Komponenten".to_string(), model.diagnostics.a_closed_components.to_string()),
        ("A · Blocker Ownerless".to_string(), model.diagnostics.a_blocked_ownerless_components.to_string()),
        ("A · Blocker Produktiv".to_string(), model.diagnostics.a_blocked_productive_components.to_string()),
        ("A · Blocker Test".to_string(), model.diagnostics.a_blocked_test_components.to_string()),
        ("A · Blocker Tool".to_string(), model.diagnostics.a_blocked_tool_components.to_string()),
        ("A · Blocker UNKNOWN".to_string(), model.diagnostics.a_blocked_unknown_components.to_string()),
        ("A · Blocker andere NO-Komponente".to_string(), model.diagnostics.a_blocked_other_no_components.to_string()),
        ("B · Zusammenführbar / redundant".to_string(), model.counts.b.to_string()),
        ("B · Fingerprint-Gruppen".to_string(), model.diagnostics.b_fingerprint_groups.to_string()),
        ("B · Kandidatensymbole".to_string(), model.diagnostics.b_candidate_symbols.to_string()),
        ("B · Cross-File-Kandidaten".to_string(), model.diagnostics.b_cross_file_candidate_symbols.to_string()),
        ("B · Bewiesene Symbole".to_string(), model.diagnostics.b_proven_symbols.to_string()),
        ("B · Independence blockiert".to_string(), model.diagnostics.b_independence_blocked_symbols.to_string()),
        ("C · Notwendige Unterstützung".to_string(), model.counts.c.to_string()),
        ("D · Autoritativer Kern".to_string(), model.counts.d.to_string()),
        ("OFFEN".to_string(), model.counts.open.to_string()),
        ("OFFEN · Reachability YES".to_string(), model.diagnostics.open_yes.to_string()),
        ("OFFEN · Reachability NO".to_string(), model.diagnostics.open_no.to_string()),
        ("OFFEN · Reachability UNKNOWN".to_string(), model.diagnostics.open_unknown.to_string()),
        ("OFFEN · ONLY_TEST".to_string(), model.diagnostics.open_only_test.to_string()),
        ("OFFEN · ONLY_TOOL".to_string(), model.diagnostics.open_only_tool.to_string()),
        ("Reachability YES".to_string(), model.diagnostics.reachability_yes.to_string()),
        ("Reachability NO".to_string(), model.diagnostics.reachability_no.to_string()),
        ("Reachability UNKNOWN".to_string(), model.diagnostics.reachability_unknown.to_string()),
        ("Reachability ONLY_TEST".to_string(), model.diagnostics.reachability_only_test.to_string()),
        ("Reachability ONLY_TOOL".to_string(), model.diagnostics.reachability_only_tool.to_string()),
        ("Public API Roots".to_string(), model.diagnostics.public_api_roots.to_string()),
        ("Uncertain Public Roots".to_string(), model.diagnostics.uncertain_public_roots.to_string()),
        ("Entry Roots".to_string(), model.diagnostics.entry_roots.to_string()),
        ("Simple-Name Duplicate Groups".to_string(), model.diagnostics.duplicate_simple_name_groups.to_string()),
        ("Symbols in Duplicate Groups".to_string(), model.diagnostics.duplicate_simple_name_symbols.to_string()),
        ("Lexical-only Sites".to_string(), model.diagnostics.lexical_only_sites.to_string()),
        ("Resolved Reference Sites".to_string(), model.diagnostics.resolved_reference_sites.to_string()),
        ("Ambiguous Reference Sites".to_string(), model.diagnostics.ambiguous_reference_sites.to_string()),
        ("Include-bound Reference Sites".to_string(), model.diagnostics.include_bound_reference_sites.to_string()),
        ("Macro-bound Reference Sites".to_string(), model.diagnostics.macro_bound_reference_sites.to_string()),
        ("self.method Resolved Sites".to_string(), model.diagnostics.self_method_resolved_sites.to_string()),
        ("D · EntryPoint".to_string(), model.diagnostics.d_entrypoint.to_string()),
        ("D · Writer Authority".to_string(), model.diagnostics.d_writer_authority.to_string()),
        ("D · Selection Authority".to_string(), model.diagnostics.d_selection_authority.to_string()),
        ("D · Canonical Authority".to_string(), model.diagnostics.d_canonical_authority.to_string()),
        ("D · Governance Gate".to_string(), model.diagnostics.d_governance_gate.to_string()),
        ("CMS State".to_string(), model.measurement.state.to_string()),
    ];
    if let Some(metrics) = &model.measurement.metrics {
        rows.extend([
            ("Runtime Producer".to_string(), metrics.producer_binding.clone()),
            ("Producer Version".to_string(), metrics.producer_version.clone()),
            ("Producer Source Signature".to_string(), metrics.producer_source_signature.clone()),
            ("Producer Binary Fingerprint".to_string(), metrics.producer_binary_fingerprint.clone()),
            ("Blocker".to_string(), metrics.blockers.to_string()),
            ("Warning".to_string(), metrics.warnings.to_string()),
            ("Hint".to_string(), metrics.hints.to_string()),
            ("Canonical Events".to_string(), metrics.canonical_events.to_string()),
            ("Selection Retained".to_string(), metrics.retained.to_string()),
            ("Selection Removed".to_string(), metrics.removed.to_string()),
        ]);
    }
    rows
}

fn table_xml(model: &InventoryModel) -> String {
    let end_row = model.rows.len() + 1;
    let end_col = excel_column(TABLE_HEADERS.len());
    let reference = format!("A1:{end_col}{end_row}");
    let mut out = String::new();
    writeln!(out, "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>").expect("String write cannot fail");
    write!(out, "<table xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" id=\"1\" name=\"EtuSchlopfInventory\" displayName=\"EtuSchlopfInventory\" ref=\"{reference}\" headerRowCount=\"1\" totalsRowCount=\"0\" totalsRowShown=\"0\">").expect("String write cannot fail");
    write!(out, "<autoFilter ref=\"{reference}\"/>").expect("String write cannot fail");
    write!(out, "<tableColumns count=\"{}\">", TABLE_HEADERS.len()).expect("String write cannot fail");
    for (index, header) in TABLE_HEADERS.iter().enumerate() {
        write!(out, "<tableColumn id=\"{}\" name=\"{}\"/>", index + 1, xml_escape(header)).expect("String write cannot fail");
    }
    write!(out, "</tableColumns><tableStyleInfo name=\"TableStyleMedium2\" showFirstColumn=\"0\" showLastColumn=\"0\" showRowStripes=\"1\" showColumnStripes=\"0\"/></table>").expect("String write cannot fail");
    out
}

fn push_text_cell(out: &mut String, column: usize, row: usize, value: &str, style: usize) {
    let reference = format!("{}{}", excel_column(column), row);
    write!(out, "<c r=\"{reference}\" t=\"inlineStr\" s=\"{style}\"><is><t xml:space=\"preserve\">{}</t></is></c>", xml_escape(value)).expect("String write cannot fail");
}

fn push_number_cell(out: &mut String, column: usize, row: usize, value: u64) {
    let reference = format!("{}{}", excel_column(column), row);
    write!(out, "<c r=\"{reference}\"><v>{value}</v></c>").expect("String write cannot fail");
}

fn excel_column(mut index: usize) -> String {
    let mut chars = Vec::new();
    while index > 0 {
        index -= 1;
        chars.push((b'A' + (index % 26) as u8) as char);
        index /= 26;
    }
    chars.iter().rev().collect()
}

fn xml_escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            ch if ch.is_control() && !matches!(ch, '\t' | '\n' | '\r') => out.push(' '),
            ch => out.push(ch),
        }
    }
    out
}

fn validate_inventory_xlsx(path: &Path, model: &InventoryModel) -> AppResult<()> {
    let file = File::open(path)?;
    let mut zip = ZipArchive::new(file)?;
    for required in [
        "[Content_Types].xml",
        "xl/workbook.xml",
        "xl/worksheets/sheet1.xml",
        "xl/worksheets/_rels/sheet1.xml.rels",
        "xl/tables/table1.xml",
        "xl/styles.xml",
    ] {
        zip.by_name(required)
            .map_err(|error| anyhow!("Inventory XLSX missing {required}: {error}"))?;
    }

    let mut workbook = String::new();
    zip.by_name("xl/workbook.xml")?.read_to_string(&mut workbook)?;
    if workbook.matches("<sheet ").count() != 1 || !workbook.contains("name=\"Inventory\"") {
        return Err(anyhow!(
            "Inventory XLSX-3 readback must contain exactly one Inventory worksheet"
        ));
    }

    let mut sheet = String::new();
    zip.by_name("xl/worksheets/sheet1.xml")?
        .read_to_string(&mut sheet)?;
    if !sheet.contains("ySplit=\"1\"") || !sheet.contains("<tablePart r:id=\"rId1\"") {
        return Err(anyhow!(
            "Inventory XLSX-3 readback missing freeze/table binding"
        ));
    }
    if sheet.contains("<autoFilter") {
        return Err(anyhow!(
            "Inventory XLSX-3 worksheet must not duplicate the table filter with a worksheet autoFilter"
        ));
    }

    let mut relationships = String::new();
    zip.by_name("xl/worksheets/_rels/sheet1.xml.rels")?
        .read_to_string(&mut relationships)?;
    if relationships.matches("relationships/table").count() != 1
        || !relationships.contains("Target=\"/xl/tables/table1.xml\"")
        || !relationships.contains("Id=\"rId1\"")
    {
        return Err(anyhow!(
            "Inventory XLSX-3 worksheet relationship does not bind exactly one table1.xml"
        ));
    }

    let mut table = String::new();
    zip.by_name("xl/tables/table1.xml")?
        .read_to_string(&mut table)?;
    let expected_ref = format!(
        "A1:{}{}",
        excel_column(TABLE_HEADERS.len()),
        model.rows.len() + 1
    );
    let filter = format!("<autoFilter ref=\"{expected_ref}\"/>");
    if !table.contains(&format!("ref=\"{expected_ref}\""))
        || !table.contains("headerRowCount=\"1\"")
        || !table.contains("totalsRowCount=\"0\"")
        || !table.contains("totalsRowShown=\"0\"")
        || table.matches("<tableColumn ").count() != TABLE_HEADERS.len()
        || table.matches("<autoFilter ").count() != 1
        || !table.contains(&filter)
    {
        return Err(anyhow!("Inventory XLSX-3 table/filter readback mismatch"));
    }
    let filter_position = table
        .find(&filter)
        .ok_or_else(|| anyhow!("Inventory XLSX-3 table filter missing"))?;
    let columns_position = table
        .find("<tableColumns ")
        .ok_or_else(|| anyhow!("Inventory XLSX-3 table columns missing"))?;
    if filter_position >= columns_position {
        return Err(anyhow!(
            "Inventory XLSX-3 table autoFilter must precede tableColumns"
        ));
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_row(name: &str) -> SymbolRow {
        SymbolRow {
            class: String::new(),
            class_rule: String::new(),
            evidence: String::new(),
            qualified_symbol: format!("m::{name}"),
            simple_name: name.to_string(),
            module_scope: "m".to_string(),
            declaration_kind: "function".to_string(),
            relative_source_path: "src/m.rs".to_string(),
            start_line: 1,
            line_count: 1,
            source_visibility: "private".to_string(),
            context: "Produktiv".to_string(),
            productive_reachability: "NO".to_string(),
            authority: String::new(),
            lexical_hits: 0,
            resolved_references: 0,
            ambiguous_references: 0,
            incoming_calls: 0,
            value_bindings: 0,
            signature_fingerprint: "sha256:sig".to_string(),
            signature_shape_fingerprint: "sha256:shape".to_string(),
            body_fingerprint: "sha256:body".to_string(),
            a_candidate: false,
            a_component: String::new(),
            a_closure: "NOT_EVALUATED".to_string(),
            a_blocker: String::new(),
            b_candidate: false,
            b_counterpart: String::new(),
            b_evidence: String::new(),
            b_blocker: String::new(),
            merge_target: String::new(),
            reason: String::new(),
            body_start: None,
            body_end: None,
            declaration_name_token: 0,
            has_cfg_or_export_attr: false,
        }
    }

    fn test_model() -> InventoryModel {
        let mut row = test_row("f");
        row.class = "OFFEN".to_string();
        row.class_rule = "OFFEN.INSUFFICIENT_PROOF".to_string();
        row.evidence = "UNKNOWN".to_string();
        row.productive_reachability = "UNKNOWN".to_string();
        row.authority = "UNKNOWN".to_string();
        row.reason = "test".to_string();
        InventoryModel {
            version: "0.10.1839".to_string(),
            revision: "r994".to_string(),
            take_name: "NativeInventoryEvidenceModelCapabilityIdentityClosure".to_string(),
            run_id: "obs-test".to_string(),
            scope: "PROJECT".to_string(),
            scope_project: "Etüde Nr. 6".to_string(),
            source_scope_sha256: "sha256:test".to_string(),
            source_file_count: 1,
            input_fingerprint: "sha256:input".to_string(),
            binary_fingerprint: "fnv64:test".to_string(),
            measurement: MeasurementProjection {
                state: "BOUND_CURRENT_RUN",
                metrics: None,
                project_count: 1,
                corpus_manifest_run_id: None,
            },
            rows: vec![row],
            counts: ArchCounts {
                open: 1,
                ..ArchCounts::default()
            },
            diagnostics: InventoryDiagnostics {
                reachability_unknown: 1,
                open_unknown: 1,
                ..InventoryDiagnostics::default()
            },
        }
    }

    #[test]
    fn table_contract_is_single_sheet_filterable_and_frozen() {
        let model = test_model();
        let sheet = worksheet_xml(&model);
        assert!(sheet.contains("ySplit=\"1\""));
        assert!(!sheet.contains("<autoFilter"));
        assert!(sheet.contains("<tableParts count=\"1\">"));
        let table = table_xml(&model);
        assert!(table.contains("ref=\"A1:AE2\""));
        assert!(table.contains("headerRowCount=\"1\""));
        assert!(table.contains("totalsRowCount=\"0\""));
        assert_eq!(table.matches("<autoFilter ").count(), 1);
        assert!(table.contains("<autoFilter ref=\"A1:AE2\"/>"));
        let relationships = sheet_rels_xml();
        assert!(relationships.contains("Target=\"/xl/tables/table1.xml\""));
        let workbook = workbook_xml();
        assert_eq!(workbook.matches("<sheet ").count(), 1);
        assert!(workbook.contains("name=\"Inventory\""));
    }

    #[test]
    fn lexical_extraction_finds_free_impl_and_trait_methods_without_comments_or_strings() {
        let file = SourceFile {
            relative: "src/sample.rs".to_string(),
            source: concat!(
                "// fn fake() {}\n",
                "fn free() {}\n",
                "trait T { fn declared(&self); fn defaulted(&self) {} }\n",
                "struct S; impl S { pub(crate) fn method(&self) {} }\n",
                "const X: &str = \"fn also_fake() {}\";\n",
                "const OPEN: char = '{'; const CLOSE: u8 = b'}';\n"
            )
            .to_string(),
            context: "Produktiv".to_string(),
        };
        let rows = extract_symbols(&[file]).expect("symbols");
        let mut names = rows
            .iter()
            .map(|row| row.simple_name.as_str())
            .collect::<Vec<_>>();
        names.sort_unstable();
        assert_eq!(names, vec!["declared", "defaulted", "free", "method"]);
        assert!(!names.contains(&"fake"));
        assert!(!names.contains(&"also_fake"));
    }

    #[test]
    fn esi2_test_scope_is_structurally_inherited_without_leaking_to_following_items() {
        let file = SourceFile {
            relative: "src/sample.rs".to_string(),
            source: concat!(
                "#[cfg(test)] mod tests { fn helper() {} #[test] fn case() { helper(); } }\n",
                "fn production() {}\n"
            )
            .to_string(),
            context: "Produktiv".to_string(),
        };
        let rows = extract_symbols(&[file]).expect("symbols");
        let helper = rows.iter().find(|row| row.simple_name == "helper").expect("helper");
        let case = rows.iter().find(|row| row.simple_name == "case").expect("case");
        let production = rows
            .iter()
            .find(|row| row.simple_name == "production")
            .expect("production");
        assert_eq!(helper.context, "Test");
        assert_eq!(case.context, "Test");
        assert_eq!(production.context, "Produktiv");
        assert!(!production.has_cfg_or_export_attr);

        let conditional = SourceFile {
            relative: "src/conditional.rs".to_string(),
            source: concat!(
                "#[cfg(any(test, feature = \"dev\"))] fn maybe_production() {} ",
                "#[cfg(all(test, feature = \"dev\"))] fn test_only() {}"
            )
            .to_string(),
            context: "Produktiv".to_string(),
        };
        let rows = extract_symbols(&[conditional]).expect("conditional symbols");
        let maybe_production = rows
            .iter()
            .find(|row| row.simple_name == "maybe_production")
            .expect("maybe production");
        let test_only = rows
            .iter()
            .find(|row| row.simple_name == "test_only")
            .expect("test only");
        assert_eq!(maybe_production.context, "Produktiv");
        assert!(maybe_production.has_cfg_or_export_attr);
        assert_eq!(test_only.context, "Test");
        assert!(test_only.has_cfg_or_export_attr);
    }

    #[test]
    fn esi2_value_binding_propagates_productive_reachability() {
        let file = SourceFile {
            relative: "src/lib.rs".to_string(),
            source: "pub fn root() { let _mapped = Some(helper); } fn helper() {}".to_string(),
            context: "Produktiv".to_string(),
        };
        let files = vec![file];
        let mut rows = extract_symbols(&files).expect("symbols");
        let relationships = derive_repository_relationships(&files, &mut rows).expect("relationships");
        let helper = rows.iter().find(|row| row.simple_name == "helper").expect("helper");
        assert_eq!(helper.productive_reachability, "YES");
        assert_eq!(helper.value_bindings, 1);
        assert_eq!(relationships.public_api_roots, 1);
    }

    #[test]
    fn esi2_unique_member_name_stays_ambiguous_without_receiver_proof() {
        let file = SourceFile {
            relative: "src/lib.rs".to_string(),
            source: concat!(
                "pub fn root(value: &Thing) { value.helper(); } ",
                "struct Thing; impl Thing { fn helper(&self) {} }"
            )
            .to_string(),
            context: "Produktiv".to_string(),
        };
        let files = vec![file];
        let mut rows = extract_symbols(&files).expect("symbols");
        derive_repository_relationships(&files, &mut rows).expect("relationships");
        let helper = rows
            .iter()
            .find(|row| row.simple_name == "helper")
            .expect("helper");
        assert_eq!(helper.productive_reachability, "UNKNOWN");
        assert_eq!(helper.incoming_calls, 0);
        assert_eq!(helper.ambiguous_references, 1);
    }

    #[test]
    fn esi2_body_fingerprint_tracks_literal_changes_without_exposing_literal_text() {
        let make = |literal: &str| SourceFile {
            relative: "src/sample.rs".to_string(),
            source: format!("fn value() -> &'static str {{ \"{literal}\" }}"),
            context: "Produktiv".to_string(),
        };
        let first = extract_symbols(&[make("alpha")]).expect("first symbols");
        let second = extract_symbols(&[make("beta")]).expect("second symbols");
        assert_ne!(first[0].body_fingerprint, second[0].body_fingerprint);
        assert!(!first[0].body_fingerprint.contains("alpha"));
        assert!(!second[0].body_fingerprint.contains("beta"));
    }

    #[test]
    fn esi2_restricted_visibility_is_not_an_automatic_productive_root() {
        let file = SourceFile {
            relative: "src/lib.rs".to_string(),
            source: "pub(crate) fn internal() { helper(); } fn helper() {}".to_string(),
            context: "Produktiv".to_string(),
        };
        let files = vec![file];
        let mut rows = extract_symbols(&files).expect("symbols");
        let relationships = derive_repository_relationships(&files, &mut rows).expect("relationships");
        assert_eq!(relationships.public_api_roots, 0);
        assert!(rows
            .iter()
            .all(|row| row.productive_reachability == "NO"));
    }

    #[test]
    fn esi3_static_include_fragment_inherits_productive_owner() {
        let files = vec![
            SourceFile {
                relative: "src/lib.rs".to_string(),
                source: "pub fn root() { include!(\"fragment.rs\"); }".to_string(),
                context: "Produktiv".to_string(),
            },
            SourceFile {
                relative: "src/fragment.rs".to_string(),
                source: "helper(); fn helper() {}".to_string(),
                context: "Produktiv".to_string(),
            },
        ];
        let mut rows = extract_symbols(&files).expect("symbols");
        let relationships =
            derive_repository_relationships(&files, &mut rows).expect("relationships");
        let helper = rows
            .iter()
            .find(|row| row.simple_name == "helper")
            .expect("helper");
        assert_eq!(helper.productive_reachability, "YES");
        assert!(relationships.include_bound_reference_sites >= 1);
    }

    #[test]
    fn esi3_macro_body_inherits_invocation_owner() {
        let files = vec![SourceFile {
            relative: "src/lib.rs".to_string(),
            source: concat!(
                "macro_rules! call_helper { () => { helper(); } } ",
                "pub fn root() { call_helper!(); } ",
                "fn helper() {}"
            )
            .to_string(),
            context: "Produktiv".to_string(),
        }];
        let mut rows = extract_symbols(&files).expect("symbols");
        let relationships =
            derive_repository_relationships(&files, &mut rows).expect("relationships");
        let helper = rows
            .iter()
            .find(|row| row.simple_name == "helper")
            .expect("helper");
        assert_eq!(helper.productive_reachability, "YES");
        assert!(relationships.macro_bound_reference_sites >= 1);
    }

    #[test]
    fn esi3_self_method_call_resolves_inside_same_impl() {
        let files = vec![SourceFile {
            relative: "src/lib.rs".to_string(),
            source: concat!(
                "struct Thing; impl Thing { ",
                "pub fn root(&self) { self.helper(); } ",
                "fn helper(&self) {} }"
            )
            .to_string(),
            context: "Produktiv".to_string(),
        }];
        let mut rows = extract_symbols(&files).expect("symbols");
        let relationships =
            derive_repository_relationships(&files, &mut rows).expect("relationships");
        let helper = rows
            .iter()
            .find(|row| row.simple_name == "helper")
            .expect("helper");
        assert_eq!(helper.productive_reachability, "YES");
        assert_eq!(helper.incoming_calls, 1);
        assert_eq!(relationships.self_method_resolved_sites, 1);
    }

    #[test]
    fn esi3_member_field_name_remains_lexical_only() {
        let files = vec![SourceFile {
            relative: "src/lib.rs".to_string(),
            source: concat!(
                "struct S { helper: i32 } ",
                "pub fn root(value: S) { let _ = value.helper; } ",
                "fn helper() {}"
            )
            .to_string(),
            context: "Produktiv".to_string(),
        }];
        let mut rows = extract_symbols(&files).expect("symbols");
        let relationships =
            derive_repository_relationships(&files, &mut rows).expect("relationships");
        let helper = rows
            .iter()
            .find(|row| row.simple_name == "helper")
            .expect("helper function");
        assert_eq!(helper.resolved_references, 0);
        assert_eq!(helper.ambiguous_references, 0);
        assert!(relationships.lexical_only_sites >= 1);
    }

    #[test]
    fn arch4_a_diagnostics_name_external_unknown_blocker() {
        let candidate = test_row("candidate");
        let mut unknown_owner = test_row("unknown_owner");
        unknown_owner.productive_reachability = "UNKNOWN".to_string();
        unknown_owner.source_visibility = "pub".to_string();
        let mut rows = vec![candidate, unknown_owner];
        let reference_owners = vec![BTreeSet::from([Some(1)]), BTreeSet::new()];
        let analysis = analyze_removal_components(&mut rows, &reference_owners);
        assert_eq!(analysis.candidate_symbols, 1);
        assert_eq!(analysis.candidate_components, 1);
        assert_eq!(analysis.blocked_unknown_components, 1);
        assert!(rows[0].a_candidate);
        assert_eq!(rows[0].a_closure, "BLOCKED");
        assert_eq!(rows[0].a_blocker, "UNKNOWN_OWNER");
    }

    #[test]
    fn arch4_b_requires_exact_legacy_fingerprint_and_non_independent_target() {
        let mut legacy = test_row("legacy_value");
        legacy.qualified_symbol = "old::legacy_value".to_string();
        legacy.relative_source_path = "src/old.rs".to_string();
        let mut target = test_row("value");
        target.qualified_symbol = "new::value".to_string();
        target.relative_source_path = "src/new.rs".to_string();
        target.productive_reachability = "YES".to_string();
        let mut rows = vec![legacy, target];
        let analysis = analyze_b_candidates(&mut rows);
        assert_eq!(analysis.proven_symbols, 1);
        assert!(rows[0].b_candidate);
        assert_eq!(rows[0].b_counterpart, "new::value");
        assert_eq!(rows[0].b_evidence, "PROVEN_LEGACY_EXACT_REPLACEMENT");
        assert!(rows[0].b_blocker.is_empty());
        let counts = classify_architecture(&mut rows, &RepositoryRelationships::default());
        assert_eq!(counts.b, 1);
        assert_eq!(rows[0].class, "B");
    }

    #[test]
    fn arch_a_requires_private_free_unreferenced_and_no_sensitive_attribute() {
        let mut rows = vec![test_row("unused")];
        let reference_owners = vec![BTreeSet::new()];
        let relationships = RepositoryRelationships {
            removable_component_size: closed_unreachable_removal_components(
                &rows,
                &reference_owners,
            ),
            ..RepositoryRelationships::default()
        };
        let counts = classify_architecture(&mut rows, &relationships);
        assert_eq!(counts.a, 1);
        assert_eq!(rows[0].class_rule, "A.PRIVATE_UNREFERENCED_PROVEN");
        rows[0].has_cfg_or_export_attr = true;
        let relationships = RepositoryRelationships {
            removable_component_size: closed_unreachable_removal_components(
                &rows,
                &reference_owners,
            ),
            ..RepositoryRelationships::default()
        };
        let counts = classify_architecture(&mut rows, &relationships);
        assert_eq!(counts.a, 0);
        assert_eq!(rows[0].class, "OFFEN");
    }

    #[test]
    fn arch_a_accepts_closed_unreachable_component_with_only_internal_references() {
        let mut rows = ["left", "right"]
            .into_iter()
            .map(|name| {
                let mut row = test_row(name);
                row.resolved_references = 1;
                row.incoming_calls = 1;
                row
            })
            .collect::<Vec<_>>();
        let reference_owners = vec![BTreeSet::from([Some(1)]), BTreeSet::from([Some(0)])];
        let relationships = RepositoryRelationships {
            removable_component_size: closed_unreachable_removal_components(
                &rows,
                &reference_owners,
            ),
            ..RepositoryRelationships::default()
        };
        let counts = classify_architecture(&mut rows, &relationships);
        assert_eq!(counts.a, 2);
        assert!(rows
            .iter()
            .all(|row| row.class_rule == "A.CLOSED_UNREACHABLE_COMPONENT_PROVEN"));
    }

    #[test]
    fn arch_a_is_blocked_by_ambiguous_reference_evidence() {
        let mut rows = vec![test_row("unused")];
        rows[0].ambiguous_references = 1;
        let reference_owners = vec![BTreeSet::new()];
        let relationships = RepositoryRelationships {
            removable_component_size: closed_unreachable_removal_components(
                &rows,
                &reference_owners,
            ),
            ..RepositoryRelationships::default()
        };
        let counts = classify_architecture(&mut rows, &relationships);
        assert_eq!(counts.a, 0);
        assert_eq!(rows[0].class, "OFFEN");
    }

    #[test]
    fn arch_d_requires_symbol_level_authority_instead_of_module_path() {
        let mut base = test_row("child_text");
        base.qualified_symbol =
            "structural_write_authority::authority_primitives::child_text".to_string();
        base.relative_source_path =
            "src/structural_write_authority/authority_primitives.rs".to_string();
        base.productive_reachability = "YES".to_string();
        base.resolved_references = 1;
        base.incoming_calls = 1;
        let mut rows = vec![base.clone()];
        let counts = classify_architecture(&mut rows, &RepositoryRelationships::default());
        assert_eq!(counts.c, 1);
        assert_eq!(rows[0].class, "C");

        let mut authorized = base;
        authorized.qualified_symbol =
            "structural_write_authority::x::apply_change_authorized".to_string();
        authorized.simple_name = "apply_change_authorized".to_string();
        let mut rows = vec![authorized];
        let counts = classify_architecture(&mut rows, &RepositoryRelationships::default());
        assert_eq!(counts.d, 1);
        assert_eq!(rows[0].class_rule, "D.WRITER_AUTHORITY_SYMBOL");
    }
}
