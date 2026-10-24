//! r1182 DestructiveCarrierEffectInventoryOutcomeProjection.
//!
//! Build-time, read-only observation of the productive transformation surfaces that can
//! alter musical or structural carriers in MusicXML, of the persisting file sinks that form
//! the observable render epoch boundaries, and of the static region granularity with which
//! a runtime boundary delta can be attributed. The scan never decides music, never grants
//! or withdraws any capability and never names a corpus project, measure or witness.
//!
//! Classification is deliberately syntactic and conservative: whenever a region cannot be
//! resolved cleanly the result is `SINGLE_OR_BLOCK_UNRESOLVED`, never a stronger claim.
//!
//! r1201 adds, with the same read-only stance, the identity coverage class of every surface,
//! the exact callsites of identity-minting entries and a syntactic nominal-gate inventory.

#[cfg(not(test))]
use super::rust_source_scan_primitives::{
    collect_rust_files, fnv1a64_update, mask_non_code,
    normalize_path, token_boundary_after, token_boundary_before, FNV1A64_OFFSET_BASIS,
};
#[cfg(test)]
use crate::identity_debt_inventory::rust_source_scan_primitives::{
    collect_rust_files, fnv1a64_update, mask_non_code,
    normalize_path, token_boundary_after, token_boundary_before, FNV1A64_OFFSET_BASIS,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::io;
use std::path::Path;

const EFFECT_OWNER_ROOT: &str = "src/structural_write_authority/";
const GRAPH_EXCLUDED_PREFIXES: [&str; 3] = ["src/build_gate/", "src/bin/", "src/kodex_control_plane/"];
const GRAPH_EXCLUDED_FILES: [&str; 2] = ["src/kodex_build_orchestrator.rs", "src/take_change_control.rs"];
const PERSISTED_READ_MARKERS: [&str; 4] = [
    "read_or_fallback(",
    "read_to_string(",
    "current_xml(",
    "select_after_write(",
];
const BUFFER_MARKERS: [&str; 2] = [".replace_range(", ".insert_str("];
const FILE_MARKER: &str = "fs::write(";
const RECORDER_MARKER: &str = "record_after(";
const RECORDER_NAME: &str = "record_after";

pub const GRANULARITY_SINGLE: &str = "SINGLE_TRANSFORMATION";
pub const GRANULARITY_BLOCK: &str = "BLOCK";
pub const GRANULARITY_UNRESOLVED: &str = "SINGLE_OR_BLOCK_UNRESOLVED";

// r1201 identity coverage classes, fixed from the architecture before any corpus run.
pub const COVERAGE_PRESENT: &str = "IDENTITY_PRESENT";
pub const COVERAGE_MISSING: &str = "IDENTITY_MISSING";
pub const COVERAGE_NOT_APPLICABLE: &str = "NOT_APPLICABLE";
pub const COVERAGE_RETIRED: &str = "RETIRED";
const IDENTITY_MINT_MARKER: &str = "begin_destructive_edit_batch_attribution(";

// r1201 nominal-gate syntactic rules, fixed from the architecture before any corpus run.
pub const RULE_LITERAL_COORDINATE_TABLE: &str = "LITERAL_COORDINATE_TABLE";
pub const RULE_LITERAL_MEASURE_BINDING: &str = "LITERAL_MEASURE_BINDING";
pub const RULE_CONSTANT_TRUE_FIELD: &str = "CONSTANT_TRUE_CHECK_FIELD";
pub const RULE_CONSTANT_ZERO_FIELD: &str = "CONSTANT_ZERO_GATE_FIELD";
pub const RULE_CONSTANT_OPEN_PARAMETER: &str = "CONSTANT_OPEN_GATE_PARAMETER";
const CHECK_OUTCOME_NAME_PARTS: [&str; 10] = [
    "invariant", "stable", "green", "match", "valid", "consistent", "safe", "guard", "proof", "_ok",
];
const MEASURE_BINDING_NAMES: [&str; 4] = ["measure", "measure_number", "measure_index", "measure_no"];
const PRIMITIVE_TYPE_NAMES: [&str; 16] = [
    "bool", "usize", "isize", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128",
    "f32", "f64", "char",
];

#[derive(Debug, Clone)]
struct ScannedFunction {
    path: String,
    name: String,
    signature: String,
    masked_body: String,
    literals: Vec<String>,
    calls: Vec<(String, usize, bool)>,
    keepalive: BTreeSet<String>,
    test_only: bool,
    macro_generated: bool,
    macro_literal_args: Vec<String>,
    macro_ident_args: Vec<String>,
    body_start_line: usize,
    body_newlines: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EffectSurfaceRow {
    pub locator: u64,
    pub surface: String,
    pub physical: String,
    pub reachability: String,
    pub observability: String,
    pub recorder: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SinkKindRow {
    pub kind: String,
    pub granularity: String,
    pub call_sites: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdentityCoverageRow {
    pub locator: u64,
    pub owner: String,
    pub surface: String,
    pub physical: String,
    pub reachability: String,
    pub coverage: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MintingSiteRow {
    pub path: String,
    pub line: usize,
    pub entry: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NominalGateRow {
    pub rule: String,
    pub owner: String,
    pub locator: u64,
    pub reachability: String,
    pub subject: String,
    pub linked_sites: Vec<(String, usize)>,
}

#[derive(Debug, Clone, Default)]
pub struct EffectInventory {
    pub source_files: usize,
    pub functions_scanned: usize,
    pub surfaces: Vec<EffectSurfaceRow>,
    pub sink_kinds: Vec<SinkKindRow>,
    pub untraced_file_sinks: usize,
    pub fingerprint: u64,
    pub identity_coverage: Vec<IdentityCoverageRow>,
    pub minting_sites: Vec<MintingSiteRow>,
    pub nominal_gates: Vec<NominalGateRow>,
    pub coverage_fingerprint: u64,
}

/// Scans every Rust file below the given source roots of one project tree.
pub fn observe_effect_inventory_roots(
    project_root: &Path,
    source_roots: &[&Path],
) -> io::Result<EffectInventory> {
    let mut files = Vec::new();
    for source_root in source_roots {
        collect_rust_files(source_root, &mut files)?;
    }
    files.sort();
    files.dedup();
    let mut sources = Vec::new();
    for path in &files {
        let text = fs::read_to_string(path)?;
        let relative = normalize_path(path.strip_prefix(project_root).unwrap_or(path));
        sources.push((relative, text));
    }
    Ok(observe_effect_inventory_text(&sources))
}

/// Pure scan over `(relative path, source text)` pairs. Used by the build gate and by the
/// synthetic instrument tests, so both exercise the same measurement code path.
pub fn observe_effect_inventory_text(sources: &[(String, String)]) -> EffectInventory {
    let mut functions = Vec::new();
    let mut variant_tags: BTreeMap<String, BTreeMap<String, BTreeSet<String>>> = BTreeMap::new();
    for (path, text) in sources {
        if graph_excluded(path) {
            continue;
        }
        let masked = mask_non_code(text);
        functions.extend(scan_functions(path, text, &masked));
        functions.extend(scan_macro_generated_functions(path, text, &masked));
        variant_tags.insert(path.clone(), scan_variant_tags(text, &masked));
    }
    let functions_scanned = functions.len();

    let mut by_name: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, function) in functions.iter().enumerate() {
        if !function.test_only {
            by_name.entry(function.name.clone()).or_default().push(index);
        }
    }
    let callees: Vec<Vec<usize>> = functions
        .iter()
        .map(|function| {
            let mut out = BTreeSet::new();
            for (name, _, method) in &function.calls {
                if name == &function.name {
                    continue;
                }
                if let Some(targets) = by_name.get(name) {
                    out.extend(resolve_targets(targets, functions.as_slice(), &function.path, *method));
                }
            }
            out.into_iter().collect()
        })
        .collect();

    let reachable = productive_reach(&functions, &callees);
    let keepalive_names: BTreeSet<String> = functions
        .iter()
        .filter(|function| !function.test_only)
        .flat_map(|function| function.keepalive.iter().cloned())
        .collect();

    // Effect-owner closure facts: buffer marker, file marker, recorder kinds.
    let owner_member: Vec<bool> = functions
        .iter()
        .map(|function| function.path.starts_with(EFFECT_OWNER_ROOT) && !function.test_only)
        .collect();
    let mut entries = BTreeSet::new();
    for (index, function) in functions.iter().enumerate() {
        if owner_member[index] || function.test_only {
            continue;
        }
        for (name, _, method) in &function.calls {
            if *method {
                continue;
            }
            for target in by_name.get(name).into_iter().flatten() {
                let candidate = &functions[*target];
                if owner_member[*target] && candidate.signature.trim_start().starts_with("pub") {
                    entries.insert(*target);
                }
            }
        }
    }

    let mut surfaces: BTreeMap<usize, SurfaceFacts> = BTreeMap::new();
    for entry in &entries {
        let closure = owner_closure(*entry, &callees, &owner_member);
        let has_file = closure
            .iter()
            .any(|index| functions[*index].masked_body.contains(FILE_MARKER));
        let has_buffer = closure.iter().any(|index| {
            BUFFER_MARKERS
                .iter()
                .any(|marker| functions[*index].masked_body.contains(marker))
        });
        if !has_file && !has_buffer {
            continue;
        }
        let traced = closure
            .iter()
            .any(|index| functions[*index].masked_body.contains(RECORDER_MARKER));
        let kinds = if has_file && traced {
            recorder_kinds(*entry, &closure, &functions, &variant_tags)
        } else {
            BTreeSet::new()
        };
        let recorder = match (has_file, traced, kinds.is_empty()) {
            (false, _, _) => "NOT_APPLICABLE",
            (true, false, _) => "NO",
            (true, true, true) => "UNRESOLVED",
            (true, true, false) => "YES",
        };
        let literals: Vec<&String> = functions[*entry].literals.iter().collect();
        let signature_batch = functions[*entry]
            .signature
            .replace(char::is_whitespace, "")
            .contains("Vec<(usize,usize,String)>");
        let physical = if has_file {
            "WHOLE_FILE_WRITE".to_string()
        } else {
            physical_from_literals(&literals, signature_batch)
        };
        surfaces.insert(
            *entry,
            SurfaceFacts {
                surface: if has_file { "FILE_SINK" } else { "TYPED_AUTHORITY" },
                physical,
                recorder,
                kinds,
            },
        );
    }
    for (index, function) in functions.iter().enumerate() {
        if function.test_only
            || owner_member[index]
        {
            continue;
        }
        let returns_string = function.signature.contains("-> String") || function.signature.contains("-> (String");
        if !returns_string || !function.masked_body.contains(".push_str(") {
            continue;
        }
        let emits_carrier_markup = function.literals.iter().any(|literal| {
            literal.contains("<forward") || literal.contains("print-object")
        });
        if !emits_carrier_markup {
            continue;
        }
        let literals: Vec<&String> = function.literals.iter().collect();
        surfaces.insert(
            index,
            SurfaceFacts {
                surface: "XML_REBUILD",
                physical: physical_from_literals(&literals, false),
                recorder: "NOT_APPLICABLE",
                kinds: BTreeSet::new(),
            },
        );
    }

    let transformation: BTreeSet<usize> = surfaces
        .iter()
        .filter(|(_, facts)| facts.surface != "FILE_SINK")
        .map(|(index, _)| *index)
        .collect();

    // Region analysis per productive call site of every traced file sink.
    let mut region_classes: BTreeMap<usize, Vec<(&'static str, BTreeSet<usize>)>> = BTreeMap::new();
    for (sink, facts) in &surfaces {
        if facts.surface != "FILE_SINK" {
            continue;
        }
        for (caller, function) in functions.iter().enumerate() {
            if function.test_only || owner_member[caller] || !reachable.contains(&caller) {
                continue;
            }
            let sink_name = &functions[*sink].name;
            for (name, offset, method) in &function.calls {
                if name != sink_name || *method {
                    continue;
                }
                let (class, region) = classify_region(
                    caller,
                    *offset,
                    &functions,
                    &callees,
                    &by_name,
                    &transformation,
                );
                region_classes.entry(*sink).or_default().push((class, region));
            }
        }
    }

    let mut observability: BTreeMap<usize, &'static str> = BTreeMap::new();
    for regions in region_classes.values() {
        for (class, region) in regions {
            for member in region {
                let current = observability.entry(*member).or_insert("UNKNOWN");
                let rank = |value: &str| match value {
                    "EXACT" => 3,
                    "BLOCK" => 2,
                    _ => 1,
                };
                let candidate = match *class {
                    GRANULARITY_SINGLE => "EXACT",
                    GRANULARITY_BLOCK => "BLOCK",
                    _ => "UNKNOWN",
                };
                if rank(candidate) > rank(current) {
                    *current = candidate;
                }
            }
        }
    }

    let mut kind_sites: BTreeMap<String, Vec<&'static str>> = BTreeMap::new();
    for (sink, facts) in &surfaces {
        for kind in &facts.kinds {
            let classes = kind_sites.entry(kind.clone()).or_default();
            if let Some(regions) = region_classes.get(sink) {
                classes.extend(regions.iter().map(|(class, _)| *class));
            }
        }
    }
    let sink_kinds = kind_sites
        .into_iter()
        .map(|(kind, classes)| SinkKindRow {
            granularity: kind_granularity(&classes).to_string(),
            call_sites: classes.len(),
            kind,
        })
        .collect::<Vec<_>>();

    let mut rows = surfaces
        .iter()
        .map(|(index, facts)| {
            let function = &functions[*index];
            let reach = if reachable.contains(index) {
                "PRODUCTIVE"
            } else if keepalive_names.contains(&function.name) {
                "KEEPALIVE_ONLY"
            } else {
                "RETIRED"
            };
            let observed = if facts.surface == "FILE_SINK" {
                "NOT_APPLICABLE"
            } else {
                observability.get(index).copied().unwrap_or("NOT_INSTRUMENTED")
            };
            EffectSurfaceRow {
                locator: locator_hash(&function.path, &function.name),
                surface: facts.surface.to_string(),
                physical: facts.physical.clone(),
                reachability: reach.to_string(),
                observability: observed.to_string(),
                recorder: facts.recorder.to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows.dedup();
    let untraced_file_sinks = rows
        .iter()
        .filter(|row| row.surface == "FILE_SINK" && row.reachability == "PRODUCTIVE" && row.recorder == "NO")
        .count();

    let reach_label = |index: usize| {
        if reachable.contains(&index) {
            "PRODUCTIVE"
        } else if keepalive_names.contains(&functions[index].name) {
            "KEEPALIVE_ONLY"
        } else {
            "RETIRED"
        }
    };

    // r1201 identity coverage: an entry mints identity when its owner closure calls the
    // opaque identity constructor. Nothing about musical content is read.
    let minting_entries: BTreeSet<usize> = entries
        .iter()
        .copied()
        .filter(|entry| {
            owner_closure(*entry, &callees, &owner_member)
                .iter()
                .any(|index| functions[*index].masked_body.contains(IDENTITY_MINT_MARKER))
        })
        .collect();
    let mut identity_coverage = surfaces
        .iter()
        .map(|(index, facts)| {
            let function = &functions[*index];
            let reach = reach_label(*index);
            let coverage = if reach != "PRODUCTIVE" {
                COVERAGE_RETIRED
            } else if facts.surface == "FILE_SINK" {
                COVERAGE_NOT_APPLICABLE
            } else if minting_entries.contains(index) {
                COVERAGE_PRESENT
            } else {
                COVERAGE_MISSING
            };
            IdentityCoverageRow {
                locator: locator_hash(&function.path, &function.name),
                owner: format!("{}::{}", function.path, function.name),
                surface: facts.surface.to_string(),
                physical: facts.physical.clone(),
                reachability: reach.to_string(),
                coverage: coverage.to_string(),
            }
        })
        .collect::<Vec<_>>();
    identity_coverage.sort();
    identity_coverage.dedup();

    let mut site_callers: Vec<(usize, MintingSiteRow)> = Vec::new();
    for (caller, function) in functions.iter().enumerate() {
        if function.test_only || owner_member[caller] || function.macro_generated {
            continue;
        }
        for (name, offset, method) in &function.calls {
            if *method {
                continue;
            }
            let Some(targets) = by_name.get(name) else {
                continue;
            };
            for target in resolve_targets(targets, &functions, &function.path, false) {
                if !minting_entries.contains(&target) {
                    continue;
                }
                let entry = &functions[target];
                site_callers.push((
                    caller,
                    MintingSiteRow {
                        path: function.path.clone(),
                        line: source_line(function, path_expression_start(&function.masked_body, *offset)),
                        entry: locator_hash(&entry.path, &entry.name),
                    },
                ));
            }
        }
    }
    let mut minting_sites = site_callers.iter().map(|(_, row)| row.clone()).collect::<Vec<_>>();
    minting_sites.sort();
    minting_sites.dedup();

    let nominal_gates = nominal_gate_rows(&functions, &callees, &by_name, &owner_member, &site_callers, &reach_label);

    let mut inventory = EffectInventory {
        source_files: sources.iter().filter(|(path, _)| !graph_excluded(path)).count(),
        functions_scanned,
        surfaces: rows,
        sink_kinds,
        untraced_file_sinks,
        fingerprint: 0,
        identity_coverage,
        minting_sites,
        nominal_gates,
        coverage_fingerprint: 0,
    };
    inventory.fingerprint = inventory_fingerprint(&inventory);
    inventory.coverage_fingerprint = coverage_fingerprint(&inventory);
    inventory
}

/// Rust source for the generated include consumed by the audit-side projection.
pub fn effect_inventory_generated_source(inventory: &EffectInventory) -> String {
    let mut out = String::new();
    out.push_str("// Generated by src/build_gate/destructive_carrier_effect_scan.rs. Read-only observation.\n");
    out.push_str(&format!(
        "pub const DESTRUCTIVE_EFFECT_SCAN_SOURCE_FILES: usize = {};\n",
        inventory.source_files
    ));
    out.push_str(&format!(
        "pub const DESTRUCTIVE_EFFECT_SCAN_FUNCTIONS: usize = {};\n",
        inventory.functions_scanned
    ));
    out.push_str(&format!(
        "pub const DESTRUCTIVE_EFFECT_UNTRACED_FILE_SINKS: usize = {};\n",
        inventory.untraced_file_sinks
    ));
    out.push_str(&format!(
        "pub const DESTRUCTIVE_EFFECT_INVENTORY_FNV1A64: u64 = 0x{:016x};\n",
        inventory.fingerprint
    ));
    out.push_str("pub const DESTRUCTIVE_EFFECT_SURFACES: &[StaticEffectSurfaceRow] = &[\n");
    for row in &inventory.surfaces {
        out.push_str(&format!(
            "    StaticEffectSurfaceRow {{ locator: 0x{:016x}, surface: {:?}, physical: {:?}, reachability: {:?}, observability: {:?}, recorder: {:?} }},\n",
            row.locator, row.surface, row.physical, row.reachability, row.observability, row.recorder
        ));
    }
    out.push_str("];\n");
    out.push_str("pub const RENDER_SINK_KIND_ROWS: &[StaticSinkKindRow] = &[\n");
    for row in &inventory.sink_kinds {
        out.push_str(&format!(
            "    StaticSinkKindRow {{ kind: {:?}, granularity: {:?}, call_sites: {} }},\n",
            row.kind, row.granularity, row.call_sites
        ));
    }
    out.push_str("];\n");
    out.push_str(&format!(
        "pub const IDENTITY_COVERAGE_FNV1A64: u64 = 0x{:016x};\n",
        inventory.coverage_fingerprint
    ));
    out.push_str("pub const IDENTITY_COVERAGE_ROWS: &[StaticIdentityCoverageRow] = &[\n");
    for row in &inventory.identity_coverage {
        out.push_str(&format!(
            "    StaticIdentityCoverageRow {{ locator: 0x{:016x}, owner: {:?}, surface: {:?}, physical: {:?}, reachability: {:?}, coverage: {:?} }},\n",
            row.locator, row.owner, row.surface, row.physical, row.reachability, row.coverage
        ));
    }
    out.push_str("];\n");
    out.push_str("pub const IDENTITY_MINTING_SITES: &[StaticMintingSiteRow] = &[\n");
    for row in &inventory.minting_sites {
        out.push_str(&format!(
            "    StaticMintingSiteRow {{ path: {:?}, line: {}, entry: 0x{:016x} }},\n",
            row.path, row.line, row.entry
        ));
    }
    out.push_str("];\n");
    out.push_str("pub const NOMINAL_GATE_ROWS: &[StaticNominalGateRow] = &[\n");
    for row in &inventory.nominal_gates {
        let sites = row
            .linked_sites
            .iter()
            .map(|(path, line)| format!("({path:?}, {line})"))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "    StaticNominalGateRow {{ rule: {:?}, owner: {:?}, locator: 0x{:016x}, reachability: {:?}, subject: {:?}, linked_sites: &[{}] }},\n",
            row.rule, row.owner, row.locator, row.reachability, row.subject, sites
        ));
    }
    out.push_str("];\n");
    out
}

/// 1-based source line of a byte offset inside a scanned function body.
fn source_line(function: &ScannedFunction, body_offset: usize) -> usize {
    function.body_start_line
        + function
            .body_newlines
            .iter()
            .filter(|newline| **newline < body_offset)
            .count()
}

/// Start of a `a::b::name` path expression ending at the identifier offset, which is the
/// position `#[track_caller]` reports for the call.
fn path_expression_start(masked: &str, identifier_start: usize) -> usize {
    let bytes = masked.as_bytes();
    let mut start = identifier_start;
    loop {
        if start < 2 || &bytes[start - 2..start] != b"::" {
            return start;
        }
        let mut cursor = start - 2;
        while cursor > 0 && (bytes[cursor - 1].is_ascii_alphanumeric() || bytes[cursor - 1] == b'_') {
            cursor -= 1;
        }
        if cursor == start - 2 {
            return start;
        }
        start = cursor;
    }
}

fn coverage_fingerprint(inventory: &EffectInventory) -> u64 {
    let mut hash = FNV1A64_OFFSET_BASIS;
    for row in &inventory.identity_coverage {
        hash = fnv1a64_update(
            hash,
            format!(
                "{:016x}|{}|{}|{}|{}|{}|{}\n",
                row.locator, row.owner, row.surface, row.physical, row.reachability, row.coverage, "coverage"
            )
            .as_bytes(),
        );
    }
    for row in &inventory.minting_sites {
        hash = fnv1a64_update(hash, format!("{}|{}|{:016x}\n", row.path, row.line, row.entry).as_bytes());
    }
    for row in &inventory.nominal_gates {
        hash = fnv1a64_update(
            hash,
            format!("{}|{}|{}|{}|{:?}\n", row.rule, row.owner, row.reachability, row.subject, row.linked_sites).as_bytes(),
        );
    }
    hash
}

#[derive(Debug, Default, Clone, Copy)]
struct FieldFacts {
    true_literal: usize,
    false_literal: usize,
    zero_literal: usize,
    other_literal: usize,
    computed: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueShape {
    True,
    False,
    Zero,
    OtherLiteral,
    TypeName,
    Computed,
}

fn integer_literal_value(text: &str) -> Option<i128> {
    let trimmed = text.trim();
    let (negative, rest) = match trimmed.strip_prefix('-') {
        Some(rest) => (true, rest.trim_start()),
        None => (false, trimmed),
    };
    let digits_end = rest
        .char_indices()
        .find(|(_, ch)| !(ch.is_ascii_digit() || *ch == '_'))
        .map_or(rest.len(), |(index, _)| index);
    let digits = rest[..digits_end].replace('_', "");
    if digits.is_empty() || !rest.starts_with(|ch: char| ch.is_ascii_digit()) {
        return None;
    }
    let suffix = &rest[digits_end..];
    if !suffix.is_empty() && !PRIMITIVE_TYPE_NAMES.contains(&suffix) {
        return None;
    }
    let value = digits.parse::<i128>().ok()?;
    Some(if negative { -value } else { value })
}

fn value_shape(text: &str) -> ValueShape {
    let value = text.trim();
    match value {
        "true" => return ValueShape::True,
        "false" => return ValueShape::False,
        _ => {}
    }
    if let Some(number) = integer_literal_value(value) {
        return if number == 0 { ValueShape::Zero } else { ValueShape::OtherLiteral };
    }
    let head = value
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .next()
        .unwrap_or_default();
    if PRIMITIVE_TYPE_NAMES.contains(&head)
        || head == "str"
        || value.starts_with("&'")
        || head.starts_with(|ch: char| ch.is_ascii_uppercase())
    {
        ValueShape::TypeName
    } else {
        ValueShape::Computed
    }
}

fn previous_non_space(bytes: &[u8], index: usize) -> Option<usize> {
    (0..index).rev().find(|position| !bytes[*position].is_ascii_whitespace())
}

fn next_non_space(bytes: &[u8], index: usize) -> Option<usize> {
    (index..bytes.len()).find(|position| !bytes[*position].is_ascii_whitespace())
}

/// Text of a value expression starting at `start`, ending before the first top-level
/// `,`, `;`, `}` or `)`.
fn value_text(masked: &str, start: usize) -> &str {
    let bytes = masked.as_bytes();
    let mut depth = 0i32;
    for (offset, byte) in bytes[start..].iter().enumerate() {
        match *byte {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' if depth == 0 => return &masked[start..start + offset],
            b')' | b']' | b'}' => depth -= 1,
            b',' | b';' if depth == 0 => return &masked[start..start + offset],
            _ => {}
        }
    }
    &masked[start..]
}

/// Identifier token spans of a masked body.
fn identifier_tokens(masked: &str) -> Vec<(usize, usize)> {
    let bytes = masked.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if !(bytes[index].is_ascii_alphabetic() || bytes[index] == b'_') || !token_boundary_before(bytes, index) {
            index += 1;
            continue;
        }
        let start = index;
        while index < bytes.len() && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_') {
            index += 1;
        }
        tokens.push((start, index));
    }
    tokens
}

fn record_field_facts(masked: &str, facts: &mut BTreeMap<String, FieldFacts>) {
    let bytes = masked.as_bytes();
    let mut last_keyword = "";
    for (start, end) in identifier_tokens(masked) {
        let token = &masked[start..end];
        let previous = previous_non_space(bytes, start);
        let member = previous.is_some_and(|position| {
            bytes[position] == b'.' && (position == 0 || bytes[position - 1] != b'.')
        });
        let path_segment = previous.is_some_and(|position| position > 0 && &bytes[position - 1..=position] == b"::");
        let next = next_non_space(bytes, end);
        let rest = next.map_or("", |position| &masked[position..]);
        let entry = facts.entry(token.to_string()).or_default();
        let shape = if member {
            if rest.starts_with("==") || rest.starts_with("=>") {
                None
            } else if let Some(value) = rest.strip_prefix('=') {
                let offset = masked.len() - value.len();
                Some(value_shape(value_text(masked, offset)))
            } else if ["+=", "-=", "*=", "/=", "|=", "&=", "^=", "%="]
                .iter()
                .any(|operator| rest.starts_with(operator))
            {
                Some(ValueShape::Computed)
            } else {
                None
            }
        } else if path_segment {
            None
        } else if rest.starts_with(':') && !rest.starts_with("::") {
            let opens_field = previous.is_some_and(|position| matches!(bytes[position], b'{' | b','));
            if last_keyword == "let" || last_keyword == "mut" {
                Some(ValueShape::Computed)
            } else if opens_field {
                let offset = masked.len() - rest.len() + 1;
                Some(value_shape(value_text(masked, offset)))
            } else {
                None
            }
        } else {
            Some(ValueShape::Computed)
        };
        match shape {
            Some(ValueShape::True) => entry.true_literal += 1,
            Some(ValueShape::False) => entry.false_literal += 1,
            Some(ValueShape::Zero) => entry.zero_literal += 1,
            Some(ValueShape::OtherLiteral) => entry.other_literal += 1,
            Some(ValueShape::Computed) => entry.computed += 1,
            Some(ValueShape::TypeName) | None => {}
        }
        last_keyword = token;
    }
}

/// Member reads `.field`: (field, its masked line, rest of the line after the field).
fn member_reads(masked: &str) -> Vec<(String, String, String)> {
    let bytes = masked.as_bytes();
    let mut reads = Vec::new();
    for (start, end) in identifier_tokens(masked) {
        let Some(previous) = previous_non_space(bytes, start) else {
            continue;
        };
        if bytes[previous] != b'.' || (previous > 0 && bytes[previous - 1] == b'.') {
            continue;
        }
        let line_start = masked[..start].rfind('\n').map_or(0, |position| position + 1);
        let line_end = masked[end..].find('\n').map_or(masked.len(), |position| end + position);
        let after = masked[end..line_end].trim_start().to_string();
        if after.starts_with('=') && !after.starts_with("==") {
            continue;
        }
        reads.push((masked[start..end].to_string(), masked[line_start..line_end].to_string(), after));
    }
    reads
}

fn literal_coordinate_table(masked: &str) -> bool {
    let compares_coordinate = masked
        .lines()
        .any(|line| line.contains("==") && (line.contains("measure") || line.contains("scope")));
    if !compares_coordinate {
        return false;
    }
    let bytes = masked.as_bytes();
    for (open, byte) in bytes.iter().enumerate() {
        if *byte != b'[' {
            continue;
        }
        let Some(close) = matching_delimiter(masked, open, b'[', b']') else {
            continue;
        };
        let rows = split_top_level(&masked[open + 1..close])
            .iter()
            .filter(|element| {
                let element = element.trim();
                let Some(inner) = element.strip_prefix('(').and_then(|rest| rest.strip_suffix(')')) else {
                    return false;
                };
                let parts = split_top_level(inner);
                parts.len() >= 3 && parts.iter().all(|part| integer_literal_value(part).is_some())
            })
            .count();
        if rows >= 2 {
            return true;
        }
    }
    false
}

fn literal_measure_binding(masked: &str) -> bool {
    let bytes = masked.as_bytes();
    identifier_tokens(masked).into_iter().any(|(start, end)| {
        if !MEASURE_BINDING_NAMES.contains(&&masked[start..end]) {
            return false;
        }
        let Some(next) = next_non_space(bytes, end) else {
            return false;
        };
        let rest = &masked[next..];
        let value = if let Some(value) = rest.strip_prefix("==") {
            value
        } else if rest.starts_with(':') && !rest.starts_with("::") {
            &rest[1..]
        } else {
            return false;
        };
        let offset = masked.len() - value.len();
        integer_literal_value(value_text(masked, offset)).is_some_and(|number| number > 1)
    })
}

fn signature_parameters(function: &ScannedFunction) -> Vec<(String, String)> {
    let needle = format!("fn {}", function.name);
    let Some(name_at) = function.signature.find(&needle) else {
        return Vec::new();
    };
    let Some(open_relative) = function.signature[name_at..].find('(') else {
        return Vec::new();
    };
    let open = name_at + open_relative;
    let Some(close) = matching_delimiter(&function.signature, open, b'(', b')') else {
        return Vec::new();
    };
    split_top_level(&function.signature[open + 1..close])
        .into_iter()
        .map(|parameter| match parameter.split_once(':') {
            Some((name, kind)) => (
                name.trim().trim_start_matches("mut ").trim().to_string(),
                kind.trim().to_string(),
            ),
            None => (parameter.trim().to_string(), String::new()),
        })
        .collect()
}

fn call_arguments(masked: &str, identifier_start: usize) -> Option<Vec<String>> {
    let open = masked[identifier_start..].find('(')? + identifier_start;
    let close = matching_delimiter(masked, open, b'(', b')')?;
    Some(split_top_level(&masked[open + 1..close]))
}

fn gate_rule_scope(function: &ScannedFunction) -> bool {
    !function.test_only
        && !function.macro_generated
}

fn nominal_gate_rows(
    functions: &[ScannedFunction],
    callees: &[Vec<usize>],
    by_name: &BTreeMap<String, Vec<usize>>,
    owner_member: &[bool],
    site_callers: &[(usize, MintingSiteRow)],
    reach_label: &dyn Fn(usize) -> &'static str,
) -> Vec<NominalGateRow> {
    let mut callers: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); functions.len()];
    for (caller, targets) in callees.iter().enumerate() {
        for target in targets {
            callers[*target].insert(caller);
        }
    }
    let mut facts = BTreeMap::<String, FieldFacts>::new();
    for function in functions.iter().filter(|function| !function.test_only) {
        record_field_facts(&function.masked_body, &mut facts);
    }
    let constant_true = |field: &str| {
        facts.get(field).is_some_and(|fact| {
            fact.true_literal > 0
                && fact.false_literal + fact.zero_literal + fact.other_literal + fact.computed == 0
        })
    };
    let constant_zero = |field: &str| {
        facts.get(field).is_some_and(|fact| {
            fact.zero_literal > 0
                && fact.true_literal + fact.false_literal + fact.other_literal + fact.computed == 0
        })
    };

    let mut flagged: Vec<(usize, &'static str, String)> = Vec::new();
    for (index, function) in functions.iter().enumerate() {
        if !gate_rule_scope(function) {
            continue;
        }
        if literal_coordinate_table(&function.masked_body) {
            flagged.push((index, RULE_LITERAL_COORDINATE_TABLE, "-".to_string()));
        }
        if literal_measure_binding(&function.masked_body) {
            flagged.push((index, RULE_LITERAL_MEASURE_BINDING, "-".to_string()));
        }
        let mut seen_fields = BTreeSet::new();
        for (field, line, after) in member_reads(&function.masked_body) {
            if seen_fields.contains(&field) {
                continue;
            }
            let logical_gate = line.contains("&&") || line.contains("||") || line.trim_start().starts_with("if ");
            let zero_gate = ["== 0", "!= 0", "> 0"].iter().any(|comparison| {
                after.starts_with(comparison)
                    && !after[comparison.len()..].starts_with(|ch: char| ch.is_ascii_alphanumeric() || ch == '_' || ch == '.')
            });
            let check_outcome_name = CHECK_OUTCOME_NAME_PARTS.iter().any(|part| field.contains(part));
            if logical_gate && check_outcome_name && constant_true(&field) {
                flagged.push((index, RULE_CONSTANT_TRUE_FIELD, field.clone()));
                seen_fields.insert(field);
            } else if zero_gate && constant_zero(&field) {
                flagged.push((index, RULE_CONSTANT_ZERO_FIELD, field.clone()));
                seen_fields.insert(field);
            }
        }
        let parameters = signature_parameters(function);
        if parameters.first().is_some_and(|(name, _)| name.contains("self")) {
            continue;
        }
        for (position, (name, kind)) in parameters.iter().enumerate() {
            if kind != "bool" || !name.contains("gate") {
                continue;
            }
            let mut sites = 0usize;
            let mut open_literal = 0usize;
            for (caller, other) in functions.iter().enumerate() {
                if other.test_only || !callees[caller].contains(&index) {
                    continue;
                }
                for (call, offset, method) in &other.calls {
                    if *method || call != &function.name {
                        continue;
                    }
                    let Some(targets) = by_name.get(call) else {
                        continue;
                    };
                    if !resolve_targets(targets, functions, &other.path, false).contains(&index) {
                        continue;
                    }
                    sites += 1;
                    if call_arguments(&other.masked_body, *offset)
                        .and_then(|arguments| arguments.get(position).map(|argument| argument.trim() == "true"))
                        .unwrap_or(false)
                    {
                        open_literal += 1;
                    }
                }
            }
            if sites > 0 && sites == open_literal {
                flagged.push((index, RULE_CONSTANT_OPEN_PARAMETER, name.clone()));
            }
        }
    }

    let mut rows = flagged
        .into_iter()
        .map(|(index, rule, subject)| {
            let function = &functions[index];
            let mut region: BTreeSet<usize> = BTreeSet::from([index]);
            region.extend(callers[index].iter().copied());
            region.extend(callees[index].iter().copied());
            region.retain(|member| !owner_member[*member]);
            let mut linked_sites = site_callers
                .iter()
                .filter(|(caller, _)| region.contains(caller))
                .map(|(_, site)| (site.path.clone(), site.line))
                .collect::<Vec<_>>();
            linked_sites.sort();
            linked_sites.dedup();
            NominalGateRow {
                rule: rule.to_string(),
                owner: format!("{}::{}", function.path, function.name),
                locator: locator_hash(&function.path, &function.name),
                reachability: reach_label(index).to_string(),
                subject,
                linked_sites,
            }
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows.dedup();
    rows
}

#[derive(Debug, Clone)]
struct SurfaceFacts {
    surface: &'static str,
    physical: String,
    recorder: &'static str,
    kinds: BTreeSet<String>,
}

fn graph_excluded(path: &str) -> bool {
    GRAPH_EXCLUDED_PREFIXES
        .iter()
        .any(|prefix| path.starts_with(prefix))
        || GRAPH_EXCLUDED_FILES.contains(&path)
}

fn locator_hash(path: &str, name: &str) -> u64 {
    let mut hash = fnv1a64_update(FNV1A64_OFFSET_BASIS, path.as_bytes());
    hash = fnv1a64_update(hash, b"::");
    fnv1a64_update(hash, name.as_bytes())
}

fn inventory_fingerprint(inventory: &EffectInventory) -> u64 {
    let mut hash = FNV1A64_OFFSET_BASIS;
    for row in &inventory.surfaces {
        hash = fnv1a64_update(
            hash,
            format!(
                "{:016x}|{}|{}|{}|{}|{}\n",
                row.locator, row.surface, row.physical, row.reachability, row.observability, row.recorder
            )
            .as_bytes(),
        );
    }
    for row in &inventory.sink_kinds {
        hash = fnv1a64_update(
            hash,
            format!("{}|{}|{}\n", row.kind, row.granularity, row.call_sites).as_bytes(),
        );
    }
    hash
}

fn kind_granularity(classes: &[&'static str]) -> &'static str {
    match classes {
        [] => GRANULARITY_UNRESOLVED,
        [single] => single,
        many if many.iter().all(|class| *class == GRANULARITY_BLOCK) => GRANULARITY_BLOCK,
        _ => GRANULARITY_UNRESOLVED,
    }
}

fn physical_from_literals(literals: &[&String], caller_supplied_batch: bool) -> String {
    let mut effects = BTreeSet::new();
    if literals.iter().any(|literal| literal.contains("<forward")) {
        effects.insert("REST_TO_FORWARD");
    }
    if literals
        .iter()
        .any(|literal| literal.contains("print-object") && literal.contains("no"))
    {
        effects.insert("HIDE_CARRIER");
    }
    if caller_supplied_batch {
        effects.insert("CALLER_SUPPLIED_REPLACEMENT");
    }
    if effects.is_empty() {
        "OTHER".to_string()
    } else {
        effects.into_iter().collect::<Vec<_>>().join("+")
    }
}

fn productive_reach(functions: &[ScannedFunction], callees: &[Vec<usize>]) -> BTreeSet<usize> {
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::new();
    for (index, function) in functions.iter().enumerate() {
        let root = (function.name == "main" && function.path == "src/main.rs")
            || (function.name == "run" && function.path == "src/pipeline.rs")
            || function.name == "real_main";
        if root && !function.test_only && seen.insert(index) {
            queue.push_back(index);
        }
    }
    while let Some(index) = queue.pop_front() {
        for target in &callees[index] {
            if !functions[*target].test_only && seen.insert(*target) {
                queue.push_back(*target);
            }
        }
    }
    seen
}

/// Name resolution: a method call binds only inside its own file; a free or path call binds
/// to same-file definitions when they exist and to every same-named definition otherwise.
fn resolve_targets(targets: &[usize], functions: &[ScannedFunction], path: &str, method: bool) -> Vec<usize> {
    let local: Vec<usize> = targets
        .iter()
        .copied()
        .filter(|target| functions[*target].path == path)
        .collect();
    if method || !local.is_empty() {
        local
    } else {
        targets.to_vec()
    }
}

fn owner_closure(entry: usize, callees: &[Vec<usize>], owner_member: &[bool]) -> BTreeSet<usize> {
    let mut seen = BTreeSet::from([entry]);
    let mut queue = VecDeque::from([entry]);
    while let Some(index) = queue.pop_front() {
        for target in &callees[index] {
            if owner_member[*target] && seen.insert(*target) {
                queue.push_back(*target);
            }
        }
    }
    seen
}

fn recorder_kinds(
    entry: usize,
    closure: &BTreeSet<usize>,
    functions: &[ScannedFunction],
    variant_tags: &BTreeMap<String, BTreeMap<String, BTreeSet<String>>>,
) -> BTreeSet<String> {
    let mut kinds = BTreeSet::new();
    let mut dynamic = false;
    for index in closure {
        let function = &functions[*index];
        for (name, offset, _) in &function.calls {
            if name != RECORDER_NAME {
                continue;
            }
            match first_argument_literal(function, *offset) {
                Some(literal) => {
                    kinds.insert(literal);
                }
                None => dynamic = true,
            }
        }
    }
    if dynamic {
        let entry_function = &functions[entry];
        if entry_function.macro_generated {
            kinds.extend(entry_function.macro_literal_args.iter().cloned());
        }
        if let Some(tags) = variant_tags.get(&entry_function.path) {
            let mut identifiers: BTreeSet<String> = entry_function.macro_ident_args.iter().cloned().collect();
            identifiers.extend(identifiers_in(&entry_function.masked_body));
            for identifier in identifiers {
                if let Some(values) = tags.get(&identifier) {
                    kinds.extend(values.iter().cloned());
                }
            }
        }
    }
    kinds
}

fn first_argument_literal(function: &ScannedFunction, call_offset: usize) -> Option<String> {
    // The masked body blanks literal contents, so the literal is recovered from the
    // parallel literal table: the first literal that starts inside the argument list.
    let masked = &function.masked_body;
    let open = masked[call_offset..].find('(')? + call_offset;
    let close = matching_delimiter(masked, open, b'(', b')')?;
    let argument_text = &masked[open + 1..close];
    let first_argument_end = top_level_comma(argument_text).unwrap_or(argument_text.len());
    let first = &argument_text[..first_argument_end];
    let only_literal = first.contains('\u{1f}')
        && first.chars().all(|ch| ch == '\u{1f}' || ch.is_whitespace());
    if !only_literal {
        return None;
    }
    let literal_index = masked[..open]
        .bytes()
        .filter(|byte| *byte == 0x1f)
        .count();
    function.literals.get(literal_index).cloned()
}

fn classify_region(
    caller: usize,
    sink_offset: usize,
    functions: &[ScannedFunction],
    callees: &[Vec<usize>],
    by_name: &BTreeMap<String, Vec<usize>>,
    transformation: &BTreeSet<usize>,
) -> (&'static str, BTreeSet<usize>) {
    let function = &functions[caller];
    let mut region = BTreeSet::new();
    let mut straight = BTreeSet::new();
    let depth = brace_depths(&function.masked_body);
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::new();
    for (name, offset, method) in &function.calls {
        if *offset >= sink_offset {
            continue;
        }
        let Some(targets) = by_name.get(name) else {
            continue;
        };
        for target in resolve_targets(targets, functions, &function.path, *method) {
            let target = &target;
            if transformation.contains(target) && depth.get(*offset).copied() == Some(1) {
                straight.insert(*target);
            }
            if seen.insert(*target) {
                queue.push_back(*target);
            }
        }
    }
    while let Some(index) = queue.pop_front() {
        if transformation.contains(&index) {
            region.insert(index);
        }
        for target in &callees[index] {
            if seen.insert(*target) {
                queue.push_back(*target);
            }
        }
    }
    let class = if straight.len() >= 2 {
        GRANULARITY_BLOCK
    } else if region.len() == 1 && persisted_input(caller, functions) {
        GRANULARITY_SINGLE
    } else {
        GRANULARITY_UNRESOLVED
    };
    (class, region)
}

fn persisted_input(caller: usize, functions: &[ScannedFunction]) -> bool {
    let function = &functions[caller];
    if PERSISTED_READ_MARKERS
        .iter()
        .any(|marker| function.masked_body.contains(marker))
    {
        return true;
    }
    let mut sites = 0usize;
    let mut marked = 0usize;
    for other in functions.iter().filter(|other| !other.test_only) {
        for (name, offset, method) in &other.calls {
            if name != &function.name || *method {
                continue;
            }
            sites += 1;
            let masked = &other.masked_body;
            let Some(open_relative) = masked[*offset..].find('(') else {
                continue;
            };
            let open = offset + open_relative;
            let Some(close) = matching_delimiter(masked, open, b'(', b')') else {
                continue;
            };
            let arguments = &masked[open..=close];
            if PERSISTED_READ_MARKERS
                .iter()
                .any(|marker| arguments.contains(marker))
            {
                marked += 1;
            }
        }
    }
    sites > 0 && sites == marked
}

fn brace_depths(masked: &str) -> Vec<usize> {
    let mut depths = Vec::with_capacity(masked.len());
    let mut depth = 0usize;
    for byte in masked.bytes() {
        if byte == b'{' {
            depth += 1;
        }
        depths.push(depth);
        if byte == b'}' {
            depth = depth.saturating_sub(1);
        }
    }
    depths
}

fn scan_functions(path: &str, source: &str, masked: &str) -> Vec<ScannedFunction> {
    let marked = literal_marked_text(source, masked);
    let literals = literal_contents(source, masked);
    let test_spans = test_only_spans(masked);
    let bytes = masked.as_bytes();
    let mut out = Vec::new();
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
        while cursor < bytes.len() && (bytes[cursor].is_ascii_alphanumeric() || bytes[cursor] == b'_') {
            cursor += 1;
        }
        if cursor == name_start {
            index += 2;
            continue;
        }
        let name = masked[name_start..cursor].to_string();
        let Some(open) = open_brace_before_semicolon(masked, cursor) else {
            index = cursor;
            continue;
        };
        let Some(close) = matching_delimiter(masked, open, b'{', b'}') else {
            index = open + 1;
            continue;
        };
        let test_only = test_spans
            .iter()
            .any(|(start, end)| index >= *start && close < *end)
            || masked[index.saturating_sub(320)..index].contains("#[test]");
        let body_masked = marked[open..=close].to_string();
        let body_literals = literals
            .iter()
            .filter(|(position, _)| *position >= open && *position <= close)
            .map(|(_, content)| content.clone())
            .collect::<Vec<_>>();
        let line_start = masked[..index].rfind('\n').map_or(0, |position| position + 1);
        let mut function = scanned(
            path,
            name,
            masked[line_start..open].trim_start().to_string(),
            body_masked,
            body_literals,
            test_only,
        );
        function.body_start_line = source.as_bytes()[..open].iter().filter(|byte| **byte == b'\n').count() + 1;
        function.body_newlines = source.as_bytes()[open..=close]
            .iter()
            .enumerate()
            .filter(|(_, byte)| **byte == b'\n')
            .map(|(offset, _)| offset)
            .collect();
        out.push(function);
        index = close + 1;
    }
    out
}

fn scan_macro_generated_functions(path: &str, source: &str, masked: &str) -> Vec<ScannedFunction> {
    let marked = literal_marked_text(source, masked);
    let literals = literal_contents(source, masked);
    let mut out = Vec::new();
    let mut scan = 0usize;
    while let Some(relative) = masked[scan..].find("macro_rules!") {
        let start = scan + relative;
        let after = start + "macro_rules!".len();
        let name = masked[after..]
            .trim_start()
            .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
            .next()
            .unwrap_or_default()
            .to_string();
        let Some(open_relative) = masked[after..].find('{') else {
            break;
        };
        let open = after + open_relative;
        let Some(close) = matching_delimiter(masked, open, b'{', b'}') else {
            break;
        };
        scan = close + 1;
        let body = &masked[open..=close];
        let Some(fn_param) = generated_fn_parameter(body) else {
            continue;
        };
        let Some(position) = matcher_parameters(body).iter().position(|param| *param == fn_param) else {
            continue;
        };
        let body_marked = marked[open..=close].to_string();
        let body_literals = literals
            .iter()
            .filter(|(literal_position, _)| *literal_position >= open && *literal_position <= close)
            .map(|(_, content)| content.clone())
            .collect::<Vec<_>>();
        let needle = format!("{name}!");
        let mut invocation_scan = 0usize;
        while let Some(found) = masked[invocation_scan..].find(&needle) {
            let invocation = invocation_scan + found;
            invocation_scan = invocation + needle.len();
            if invocation >= start && invocation <= close {
                continue;
            }
            if !token_boundary_before(masked.as_bytes(), invocation) {
                continue;
            }
            let Some(paren_relative) = masked[invocation_scan..].find('(') else {
                continue;
            };
            let paren = invocation_scan + paren_relative;
            if !masked[invocation_scan..paren].trim().is_empty() {
                continue;
            }
            let Some(paren_close) = matching_delimiter(masked, paren, b'(', b')') else {
                continue;
            };
            let arguments = split_top_level(&masked[paren + 1..paren_close]);
            let Some(generated) = arguments.get(position).map(|argument| argument.trim().to_string()) else {
                continue;
            };
            if generated.is_empty() || !generated.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
                continue;
            }
            let invocation_literals = literals
                .iter()
                .filter(|(literal_position, _)| *literal_position > paren && *literal_position < paren_close)
                .map(|(_, content)| content.clone())
                .collect::<Vec<_>>();
            let ident_args = arguments
                .iter()
                .map(|argument| argument.trim().rsplit("::").next().unwrap_or_default().trim().to_string())
                .filter(|argument| !argument.is_empty() && argument.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_'))
                .collect::<Vec<_>>();
            let visibility = if body.contains("pub fn $") { "pub fn " } else { "fn " };
            let mut function = scanned(
                path,
                generated,
                format!("{visibility}{body}"),
                format!("{body_marked}{}", &marked[paren..=paren_close]),
                body_literals.iter().cloned().chain(invocation_literals.iter().cloned()).collect(),
                false,
            );
            function.macro_generated = true;
            function.macro_literal_args = invocation_literals;
            function.macro_ident_args = ident_args;
            out.push(function);
            invocation_scan = paren_close + 1;
        }
    }
    out
}

fn scanned(
    path: &str,
    name: String,
    signature: String,
    masked_body: String,
    literals: Vec<String>,
    test_only: bool,
) -> ScannedFunction {
    let (calls, keepalive) = calls_and_keepalive(&masked_body);
    ScannedFunction {
        path: path.to_string(),
        name,
        signature,
        masked_body,
        literals,
        calls,
        keepalive,
        test_only,
        macro_generated: false,
        macro_literal_args: Vec::new(),
        macro_ident_args: Vec::new(),
        body_start_line: 0,
        body_newlines: Vec::new(),
    }
}

/// Masked text in which every blanked string literal keeps one 0x1f marker byte at its
/// opening position, so literal order can be related to call argument positions.
fn literal_marked_text(source: &str, masked: &str) -> String {
    let mut bytes = masked.as_bytes().to_vec();
    for (position, _) in literal_contents(source, masked) {
        if let Some(byte) = bytes.get_mut(position) {
            *byte = 0x1f;
        }
    }
    String::from_utf8(bytes).unwrap_or_else(|_| masked.to_string())
}

fn literal_contents(source: &str, _masked: &str) -> Vec<(usize, String)> {
    let bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'/' && bytes.get(index + 1) == Some(&b'/') {
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
            continue;
        }
        if byte == b'/' && bytes.get(index + 1) == Some(&b'*') {
            let mut depth = 0usize;
            while index < bytes.len() {
                if bytes.get(index..index + 2) == Some(b"/*") {
                    depth += 1;
                    index += 2;
                } else if bytes.get(index..index + 2) == Some(b"*/") {
                    depth -= 1;
                    index += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    index += 1;
                }
            }
            continue;
        }
        if byte == b'\'' {
            index = skip_char_literal(bytes, index);
            continue;
        }
        let boundary = token_boundary_before(bytes, index);
        if boundary && matches!(byte, b'r' | b'b' | b'c') {
            let mut cursor = index + 1;
            if byte == b'b' && bytes.get(cursor) == Some(&b'r') {
                cursor += 1;
            }
            let raw_form = bytes[index] == b'r' || bytes.get(index + 1) == Some(&b'r');
            let hashes_start = cursor;
            while raw_form && bytes.get(cursor) == Some(&b'#') {
                cursor += 1;
            }
            let hashes = cursor - hashes_start;
            if bytes.get(cursor) == Some(&b'"') {
                let content_start = cursor + 1;
                let (content_end, next) = if raw_form {
                    raw_literal_end(bytes, content_start, hashes)
                } else {
                    escaped_literal_end(bytes, content_start)
                };
                out.push((index, source[content_start..content_end].to_string()));
                index = next;
                continue;
            }
        }
        if byte == b'"' {
            let (content_end, next) = escaped_literal_end(bytes, index + 1);
            out.push((index, source[index + 1..content_end].to_string()));
            index = next;
            continue;
        }
        index += 1;
    }
    out
}

fn escaped_literal_end(bytes: &[u8], start: usize) -> (usize, usize) {
    let mut cursor = start;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'\\' => cursor += 2,
            b'"' => return (cursor, cursor + 1),
            _ => cursor += 1,
        }
    }
    (bytes.len(), bytes.len())
}

fn raw_literal_end(bytes: &[u8], start: usize, hashes: usize) -> (usize, usize) {
    let mut cursor = start;
    while cursor < bytes.len() {
        if bytes[cursor] == b'"'
            && bytes
                .get(cursor + 1..cursor + 1 + hashes)
                .is_some_and(|suffix| suffix.iter().all(|byte| *byte == b'#'))
        {
            return (cursor, cursor + 1 + hashes);
        }
        cursor += 1;
    }
    (bytes.len(), bytes.len())
}

fn skip_char_literal(bytes: &[u8], quote: usize) -> usize {
    if bytes.get(quote + 1) == Some(&b'\\') {
        let mut cursor = quote + 2;
        while cursor < bytes.len() && cursor < quote + 12 {
            if bytes[cursor] == b'\'' {
                return cursor + 1;
            }
            cursor += 1;
        }
        return quote + 1;
    }
    for width in 1..=4 {
        if bytes.get(quote + 1 + width) == Some(&b'\'') {
            return quote + 2 + width;
        }
    }
    quote + 1
}

fn scan_variant_tags(source: &str, masked: &str) -> BTreeMap<String, BTreeSet<String>> {
    let literals = literal_contents(source, masked);
    let bytes = masked.as_bytes();
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut scan = 0usize;
    while let Some(relative) = masked[scan..].find("=>") {
        let arrow = scan + relative;
        scan = arrow + 2;
        let mut end = arrow;
        while end > 0 && bytes[end - 1].is_ascii_whitespace() {
            end -= 1;
        }
        let mut start = end;
        while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
            start -= 1;
        }
        if start == end {
            continue;
        }
        let identifier = &masked[start..end];
        let raw = source.as_bytes();
        let mut next = arrow + 2;
        while next < raw.len() && raw[next].is_ascii_whitespace() {
            next += 1;
        }
        if let Some((_, content)) = literals.iter().find(|(position, _)| *position == next) {
            out.entry(identifier.to_string()).or_default().insert(content.clone());
        }
    }
    out
}

fn generated_fn_parameter(macro_body: &str) -> Option<String> {
    let found = macro_body.find("fn $")?;
    let rest = &macro_body[found + "fn $".len()..];
    let name: String = rest
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .collect();
    (!name.is_empty()).then_some(name)
}

fn matcher_parameters(macro_body: &str) -> Vec<String> {
    let Some(open) = macro_body.find('(') else {
        return Vec::new();
    };
    let Some(close) = matching_delimiter(macro_body, open, b'(', b')') else {
        return Vec::new();
    };
    macro_body[open + 1..close]
        .split('$')
        .skip(1)
        .map(|part| {
            part.chars()
                .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
                .collect::<String>()
        })
        .collect()
}

fn calls_and_keepalive(masked: &str) -> (Vec<(String, usize, bool)>, BTreeSet<String>) {
    let bytes = masked.as_bytes();
    let mut calls = Vec::new();
    let mut keepalive = BTreeSet::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if !(bytes[index].is_ascii_alphabetic() || bytes[index] == b'_') || !token_boundary_before(bytes, index) {
            index += 1;
            continue;
        }
        let start = index;
        while index < bytes.len() && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_') {
            index += 1;
        }
        let identifier = &masked[start..index];
        let mut cursor = index;
        while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if bytes.get(cursor) == Some(&b'(') && identifier != "fn" {
            let mut before = start;
            while before > 0 && bytes[before - 1].is_ascii_whitespace() {
                before -= 1;
            }
            let method = before > 0 && bytes[before - 1] == b'.';
            if !method && std_qualified(masked, start) {
                continue;
            }
            calls.push((identifier.to_string(), start, method));
            continue;
        }
        if identifier == "let" {
            let statement_end = masked[cursor..].find(';').map_or(bytes.len(), |relative| cursor + relative);
            let statement = masked[cursor..statement_end].trim();
            if let Some((binding, value)) = statement.split_once('=')
                && binding.trim().starts_with('_')
                && !value.contains('(')
            {
                let target = value.trim().rsplit("::").next().unwrap_or_default().trim();
                if !target.is_empty() && target.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
                    keepalive.insert(target.to_string());
                }
                index = statement_end;
            }
        }
    }
    (calls, keepalive)
}

/// A path call whose qualifier is a standard-library module or type never binds to a
/// project function of the same name (`std::fs::write(` is not the sink method `write`).
fn std_qualified(masked: &str, identifier_start: usize) -> bool {
    const STD_QUALIFIERS: [&str; 22] = [
        "std", "fs", "io", "env", "mem", "fmt", "cmp", "iter", "str", "char", "String", "Vec",
        "Option", "Result", "Box", "Path", "PathBuf", "BTreeMap", "BTreeSet", "VecDeque", "process", "thread",
    ];
    let prefix = masked[..identifier_start].trim_end();
    let Some(before_path) = prefix.strip_suffix("::") else {
        return false;
    };
    let qualifier: String = before_path
        .chars()
        .rev()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    STD_QUALIFIERS.contains(&qualifier.as_str())
}

fn identifiers_in(masked: &str) -> BTreeSet<String> {
    masked
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .filter(|token| token.chars().next().is_some_and(|ch| ch.is_ascii_uppercase()))
        .map(str::to_string)
        .collect()
}

fn split_top_level(text: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for ch in text.chars() {
        match ch {
            '(' | '[' | '{' | '<' => depth += 1,
            ')' | ']' | '}' | '>' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(std::mem::take(&mut current));
                continue;
            }
            _ => {}
        }
        current.push(ch);
    }
    if !current.trim().is_empty() {
        parts.push(current);
    }
    parts
}

fn top_level_comma(text: &str) -> Option<usize> {
    let mut depth = 0i32;
    for (index, ch) in text.char_indices() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => return Some(index),
            _ => {}
        }
    }
    None
}

fn test_only_spans(masked: &str) -> Vec<(usize, usize)> {
    // r1201: a `#[cfg(test)]` attribute covers exactly the item it annotates. The r1182
    // version searched forward for the next `mod` text and could therefore mark unrelated
    // productive items as test-only after a `#[cfg(test)] fn` accessor.
    const ITEM_KEYWORDS: [&str; 9] = ["mod", "fn", "impl", "struct", "enum", "trait", "const", "unsafe", "async"];
    let bytes = masked.as_bytes();
    let mut spans = Vec::new();
    let mut scan = 0usize;
    while let Some(relative) = masked[scan..].find("#[cfg(test)]") {
        let attribute = scan + relative;
        let mut cursor = attribute + "#[cfg(test)]".len();
        scan = cursor;
        loop {
            while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
                cursor += 1;
            }
            if masked[cursor..].starts_with("#[") {
                let Some(close) = matching_delimiter(masked, cursor + 1, b'[', b']') else {
                    break;
                };
                cursor = close + 1;
                continue;
            }
            if masked[cursor..].starts_with("pub") && token_boundary_after(bytes, cursor + 3) {
                cursor += 3;
                while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
                    cursor += 1;
                }
                if bytes.get(cursor) == Some(&b'(') {
                    let Some(close) = matching_delimiter(masked, cursor, b'(', b')') else {
                        break;
                    };
                    cursor = close + 1;
                }
                continue;
            }
            break;
        }
        let item_keyword = ITEM_KEYWORDS.iter().any(|keyword| {
            masked[cursor..].starts_with(keyword) && token_boundary_after(bytes, cursor + keyword.len())
        });
        if !item_keyword {
            continue;
        }
        let Some(open) = open_brace_before_semicolon(masked, cursor) else {
            continue;
        };
        let Some(close) = matching_delimiter(masked, open, b'{', b'}') else {
            continue;
        };
        spans.push((attribute, close + 1));
        scan = close + 1;
    }
    spans
}

fn open_brace_before_semicolon(masked: &str, start: usize) -> Option<usize> {
    for (offset, byte) in masked.as_bytes()[start..].iter().enumerate() {
        match *byte {
            b'{' => return Some(start + offset),
            b';' => return None,
            _ => {}
        }
    }
    None
}

fn matching_delimiter(masked: &str, open: usize, opening: u8, closing: u8) -> Option<usize> {
    let bytes = masked.as_bytes();
    if bytes.get(open) != Some(&opening) {
        return None;
    }
    let mut depth = 0usize;
    for (index, byte) in bytes.iter().enumerate().skip(open) {
        if *byte == opening {
            depth += 1;
        } else if *byte == closing {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(index);
            }
        }
    }
    None
}
