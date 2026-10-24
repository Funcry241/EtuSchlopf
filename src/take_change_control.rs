// r1492 WindowsJobDependencyAdmissionBoundary: freezes only the exact future KodexOnly Windows job dependency addition; no Cargo, guard, unsafe or music change in this take.
// r1102 CompletedSchemaSchlopfMusicXml40TransitionRetirementLedgerClosure: retires the completed one-shot SchemaSchlopf-r020 dependency admission and r020->r021 MusicXML-4.0 cutover machinery; r021 qualified-component bytes remain closed in ValidationOnly steady state and root Cargo dependency changes remain fail-closed.
// r1036 r09 AuthorityRetirementKodexClosure: pure productive writer-authority loss requires authority_retirement_v1; candidate/selector/multiplicity/threshold semantics remain unchanged and causal_class_first_v1 is never bypassed.
// r1031 ProxyIdentityTargetConditionedSelectorKodexClosure: rejects selector-sensitive new decision logic unless its causal error-class definition was frozen without selector code in an already accepted parent; classification and repair scopes may only consume that unchanged parent contract, so syntactic identity-freedom cannot hide target-conditioned proxy identity.
// r575 SchemaSchlopfIntegrationManifestWarningFreeClosure: keeps SchemaSchlopf r020 source bytes exact while hardening only the future EtuSchlopf-local integration Cargo manifest to warnings=deny; the rejected r575 runtime candidate is not authority and productive runtime remains unchanged in this take.
// r596 MusicXml40ProductContractCutoverTestCompileRecoveryAdmission: rebinds the exact one-shot MusicXML-4.0 cutover target from rejected r595 to r597 after a test-only Rust lifetime compile failure; ValidationOnly and productive runtime remain unchanged.
// r594 MusicXml40ProductContractCutoverRecoveryAdmission: rebinds the exact one-shot MusicXML-4.0 cutover target from rejected r593 to r595 without widening ValidationOnly or changing runtime.
// r592 MusicXml40ProductContractAdmissionBoundary: admits exactly one future ValidationOnly transition from the qualified SchemaSchlopf-r020 / MusicXML-3.1 product contract to the qualified SchemaSchlopf-r021 / MusicXML-4.0 product contract; this take changes governance only and also closes steady-state mutation of qualified component bytes.
// r574 SchemaSchlopfDependencyAdmissionBoundary: adds an exact ValidationOnly/SchemaSchlopf-r020 dependency admission contract without changing Cargo.toml or productive runtime in this take.
// r528 ChangeClassPathRoleChangeKindColdStartReadbackClosure: explicit Audit owner roots precede lexical music-subject markers in PathRole while existing semantic/writer defenses remain fail-closed; adds regression coverage for the r526 rejection class.
// r449 CompletedCargoTransitionRetirementSuperCut: removes completed one-shot Cargo/build-owner transition capabilities; steady-state root Cargo changes are version-only and Watch Cargo metadata is closed.
use crate::rust_source_scan_primitives::fnv1a64;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[path = "build_gate/dead_code_detection_integrity.rs"]
mod dead_code_detection_integrity;
#[path = "build_gate/semantic_capability_diff.rs"]
mod semantic_capability_diff;

#[path = "build_gate/render_sovereignty_kodex.rs"]
pub(crate) mod render_sovereignty_kodex;
#[path = "build_gate/ssc_evidence_proof_shadow_permit_kodex.rs"]
pub(crate) mod ssc_evidence_proof_shadow_permit_kodex;

#[path = "build_gate/receipt_surface_normalization_kodex.rs"]
pub(crate) mod receipt_surface_normalization_kodex;

pub const CHANGE_MANIFEST_RELATIVE_PATH: &str = "src/build_gate/take_change_class.txt";
pub const WATCH_PROJECT_DIR: &str = "tools/etuschlopf_watch";
pub const WATCH_PROJECT_MANIFEST_RELATIVE_PATH: &str = "tools/etuschlopf_watch/Cargo.toml";
pub const WATCH_PROJECT_LOCK_RELATIVE_PATH: &str = "tools/etuschlopf_watch/Cargo.lock";
use ssc_evidence_proof_shadow_permit_kodex::k2::governed;
pub use crate::managed_source_tree_definition::{
    MANAGED_DIRS, MANAGED_ROOT_FILES, REQUIRED_MANAGED_ROOT_FILES,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeClass {
    KodexOnly,
    AuditOnly,
    OutcomeProjection,
    RefactorOnly,
    MusicMutation,
    CalibrationOnly,
    MechanicalOnly,
    ValidationOnly,
    /// Contract v4 Teil B: Kodex, runtime, test and metadata paths together; Kodex paths
    /// only for governed data files, every rule change admitted by the parent.
    GovernedChange,
}

impl ChangeClass {
    pub fn parse(value: &str) -> Result<Self, String> {
        let normalized = value.trim();
        match normalized {
            "KodexOnly" => Ok(Self::KodexOnly),
            "AuditOnly" => Ok(Self::AuditOnly),
            "OutcomeProjection" => Ok(Self::OutcomeProjection),
            "RefactorOnly" => Ok(Self::RefactorOnly),
            "MusicMutation" => Ok(Self::MusicMutation),
            "CalibrationOnly" => Ok(Self::CalibrationOnly),
            "MechanicalOnly" => Ok(Self::MechanicalOnly),
            "ValidationOnly" => Ok(Self::ValidationOnly),
            "GovernedChange" => Ok(Self::GovernedChange),
            other => Err(format!(
                "unbekannte change_class {other:?}; erlaubt sind KodexOnly, AuditOnly, OutcomeProjection, RefactorOnly, MusicMutation, CalibrationOnly, MechanicalOnly, ValidationOnly, GovernedChange"
            )),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KodexOnly => "KodexOnly",
            Self::AuditOnly => "AuditOnly",
            Self::OutcomeProjection => "OutcomeProjection",
            Self::RefactorOnly => "RefactorOnly",
            Self::MusicMutation => "MusicMutation",
            Self::CalibrationOnly => "CalibrationOnly",
            Self::MechanicalOnly => "MechanicalOnly",
            Self::ValidationOnly => "ValidationOnly",
            Self::GovernedChange => "GovernedChange",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChangeKind {
    Added,
    Modified,
    Deleted,
}

impl ChangeKind {
    fn parse(value: &str) -> Result<Self, String> {
        match value.trim() {
            "added" => Ok(Self::Added),
            "modified" => Ok(Self::Modified),
            "deleted" => Ok(Self::Deleted),
            other => Err(format!("unbekannte change kind {other:?}")),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Modified => "modified",
            Self::Deleted => "deleted",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredChange {
    pub kind: ChangeKind,
    pub path: String,
    pub incoming_fnv1a64: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ChangeManifest {
    pub schema_version: u32,
    pub change_class: ChangeClass,
    pub from_version: String,
    pub to_version: String,
    pub reason: String,
    pub kodex_writer_relocation: bool,
    pub kodex_negative_check_relocation: bool,
    pub kodex_negative_check_module_relocation: bool,
    pub warning_debt_closure: bool,
    pub authority_retirement_contract: Option<String>,
    /// GovernedChange only: `kind|target|reason` per declared governance change.
    pub governed_changes: Vec<String>,
    /// GovernedChange only: `none` or `declared|<reason>`.
    pub music_effect: Option<String>,
    pub changes: BTreeMap<String, DeclaredChange>,
}

#[derive(Debug, Clone)]
pub struct ChangeDependencyPreflightSummary {
    pub change_class: ChangeClass,
    pub declared_change_count: usize,
    pub actual_change_count: usize,
    pub kodex_change_count: usize,
    pub audit_change_count: usize,
    pub music_sensitive_change_count: usize,
    pub metadata_change_count: usize,
    pub pinned_architecture_change_count: usize,
}

#[derive(Debug, Clone)]
pub struct ChangeValidationSummary {
    pub schema_version: u32,
    pub change_class: ChangeClass,
    pub reason: String,
    pub from_version: String,
    pub to_version: String,
    pub declared_change_count: usize,
    pub actual_change_count: usize,
    pub kodex_change_count: usize,
    pub audit_change_count: usize,
    pub music_sensitive_change_count: usize,
    pub metadata_change_count: usize,
    pub semantic_added_count: usize,
    pub semantic_modified_count: usize,
    pub semantic_removed_count: usize,
    pub semantic_kodex_count: usize,
    pub semantic_audit_count: usize,
    pub semantic_music_count: usize,
    pub semantic_calibration_count: usize,
    pub semantic_writer_family_count: usize,
    pub semantic_unknown_count: usize,
    pub warning_debt_closure: bool,
    pub governed_changes: Vec<String>,
    pub music_effect: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PathRole {
    Kodex,
    Audit,
    MusicSensitive,
    Metadata,
    OtherSource,
    Unknown,
}

pub fn validate_dead_code_detection_integrity_transition(
    current_root: &Path,
    incoming_root: &Path,
) -> Result<usize, String> {
    dead_code_detection_integrity::validate_transition(current_root, incoming_root)
}

pub fn validate_dead_code_detection_integrity_installed(
    project_root: &Path,
) -> Result<usize, String> {
    dead_code_detection_integrity::validate_installed(project_root)
}

pub fn build_revision_effect_snapshot(
    source_root: &Path,
    current_root: &Path,
) -> Result<String, String> {
    validate_complete_incoming_tree(source_root, current_root)?;
    let actual = collect_actual_changes(source_root, current_root)?;
    let changed_paths = actual.keys().cloned().collect::<BTreeSet<_>>();
    semantic_capability_diff::revision_effect_snapshot(source_root, &changed_paths)
}

pub fn validate_rejection_chain_effect_monotonic(
    previous_snapshot: &str,
    current_snapshot: &str,
) -> Result<(), String> {
    semantic_capability_diff::validate_revision_effect_monotonic(
        previous_snapshot,
        current_snapshot,
    )
}

pub fn validate_incoming_change_dependencies(
    source_root: &Path,
    current_root: &Path,
    current_version: &str,
    incoming_version: &str,
) -> Result<ChangeDependencyPreflightSummary, String> {
    validate_complete_incoming_tree(source_root, current_root)?;
    let manifest = read_manifest(&source_root.join(CHANGE_MANIFEST_RELATIVE_PATH))?;
    validate_versions(
        &manifest,
        current_version,
        incoming_version,
        Some("dependency-preflight"),
    )?;
    let actual = collect_actual_changes(source_root, current_root)?;
    validate_declared_changes(&manifest, source_root, &actual)?;
    validate_change_class_policy(&manifest, source_root, current_root, &actual)?;
    let pinned_architecture_change_count =
        architecture_pinned_changed_paths(current_root, &actual)?.len();
    let (
        kodex_change_count,
        audit_change_count,
        music_sensitive_change_count,
        metadata_change_count,
    ) = count_path_roles(&actual);
    Ok(ChangeDependencyPreflightSummary {
        change_class: manifest.change_class,
        declared_change_count: manifest.changes.len(),
        actual_change_count: actual.len(),
        kodex_change_count,
        audit_change_count,
        music_sensitive_change_count,
        metadata_change_count,
        pinned_architecture_change_count,
    })
}

pub fn validate_productive_music_identity_provenance_tree(root: &Path) -> Result<(), String> {
    semantic_capability_diff::validate_productive_music_identity_provenance(root)
}

pub fn validate_incoming_take(
    source_root: &Path,
    current_root: &Path,
    current_version: &str,
    incoming_version: &str,
) -> Result<ChangeValidationSummary, String> {
    validate_complete_incoming_tree(source_root, current_root)?;
    let manifest = read_manifest(&source_root.join(CHANGE_MANIFEST_RELATIVE_PATH))?;
    validate_versions(
        &manifest,
        current_version,
        incoming_version,
        Some("pre-take"),
    )?;
    let actual = collect_actual_changes(source_root, current_root)?;
    validate_declared_changes(&manifest, source_root, &actual)?;
    validate_change_class_policy(&manifest, source_root, current_root, &actual)?;
    let changed_paths = actual.keys().cloned().collect::<BTreeSet<_>>();
    let semantic = semantic_capability_diff::validate_semantic_change_with_governance_options(
        current_root,
        source_root,
        manifest.change_class.as_str(),
        &changed_paths,
        semantic_capability_diff::SemanticGovernanceOptions {
            kodex_writer_relocation: manifest.kodex_writer_relocation,
            kodex_negative_check_relocation: manifest.kodex_negative_check_relocation,
            kodex_negative_check_module_relocation:
                manifest.kodex_negative_check_module_relocation,
            authority_retirement: manifest.authority_retirement_contract.is_some(),
        },
    )?;
    Ok(summarize_with_semantic(&manifest, &actual, &semantic))
}

pub fn validate_complete_incoming_tree(
    source_root: &Path,
    current_root: &Path,
) -> Result<(), String> {
    validate_known_top_level_entries(source_root)?;
    for managed_dir in MANAGED_DIRS {
        let source_dir = source_root.join(managed_dir);
        if !source_dir.is_dir() {
            return Err(format!(
                "Take-Tree-Integrity: verwalteter Quellordner fehlt oder ist kein Ordner: {managed_dir}"
            ));
        }
        validate_regular_tree(&source_dir, managed_dir)?;
    }

    for required in REQUIRED_MANAGED_ROOT_FILES {
        if !source_root.join(required).is_file() {
            return Err(format!(
                "Take-Tree-Integrity: erforderliche verwaltete Root-Datei fehlt: {required}"
            ));
        }
    }

    for managed_file in MANAGED_ROOT_FILES {
        let source_file = source_root.join(managed_file);
        let current_file = current_root.join(managed_file);
        if current_file.exists() && !source_file.is_file() {
            return Err(format!(
                "Take-Tree-Integrity: eingehender Vollbaum laesst vorhandene verwaltete Root-Datei aus: {managed_file}"
            ));
        }
        if source_file.exists() && !source_file.is_file() {
            return Err(format!(
                "Take-Tree-Integrity: verwalteter Root-Pfad ist keine regulaere Datei: {managed_file}"
            ));
        }
    }

    validate_case_insensitive_uniqueness(source_root)?;
    Ok(())
}

fn validate_known_top_level_entries(source_root: &Path) -> Result<(), String> {
    let known_dirs = MANAGED_DIRS.into_iter().collect::<BTreeSet<_>>();
    let known_files = MANAGED_ROOT_FILES.into_iter().collect::<BTreeSet<_>>();
    for entry in fs::read_dir(source_root).map_err(|error| {
        format!(
            "Take-Tree-Integrity: eingehende Wurzel nicht lesbar {}: {error}",
            source_root.display()
        )
    })? {
        let entry = entry.map_err(|error| {
            format!("Take-Tree-Integrity: Root-Directory-entry nicht lesbar: {error}")
        })?;
        let name = entry.file_name().to_string_lossy().to_string();
        let file_type = entry.file_type().map_err(|error| {
            format!(
                "Take-Tree-Integrity: Root-Dateityp nicht lesbar {}: {error}",
                entry.path().display()
            )
        })?;
        let known = if file_type.is_dir() {
            known_dirs.contains(name.as_str())
        } else if file_type.is_file() {
            known_files.contains(name.as_str())
        } else {
            false
        };
        if !known {
            return Err(format!(
                "Take-Tree-Integrity: unbekannter Top-Level-Pfad {name:?}; fail-closed"
            ));
        }
    }
    Ok(())
}

pub fn verify_installed_tree_matches_source(
    source_root: &Path,
    installed_root: &Path,
) -> Result<(), String> {
    for managed_dir in MANAGED_DIRS {
        let source_dir = source_root.join(managed_dir);
        let installed_dir = installed_root.join(managed_dir);
        let source_files = relative_file_set(&source_dir)?;
        let installed_files = relative_file_set(&installed_dir)?;
        if source_files != installed_files {
            let missing = source_files
                .difference(&installed_files)
                .map(|path| path_to_slashes(path))
                .collect::<Vec<_>>();
            let extra = installed_files
                .difference(&source_files)
                .map(|path| path_to_slashes(path))
                .collect::<Vec<_>>();
            return Err(format!(
                "Take-Tree-Integrity: installierter Ordner {managed_dir} stimmt nicht mit dem eingehenden Vollbaum ueberein: missing={missing:?}; extra={extra:?}"
            ));
        }
        for relative in source_files {
            let source_file = source_dir.join(&relative);
            let installed_file = installed_dir.join(&relative);
            if files_differ(&source_file, &installed_file)? {
                return Err(format!(
                    "Take-Tree-Integrity: installierte Datei weicht vom eingehenden Vollbaum ab: {}/{}",
                    managed_dir,
                    path_to_slashes(&relative)
                ));
            }
        }
    }

    for managed_file in MANAGED_ROOT_FILES {
        let source_file = source_root.join(managed_file);
        let installed_file = installed_root.join(managed_file);
        match (source_file.is_file(), installed_file.is_file()) {
            (true, true) => {
                if files_differ(&source_file, &installed_file)? {
                    return Err(format!(
                        "Take-Tree-Integrity: installierte Root-Datei weicht ab: {managed_file}"
                    ));
                }
            }
            (false, false) => {}
            (true, false) => {
                return Err(format!(
                    "Take-Tree-Integrity: installierte Root-Datei fehlt: {managed_file}"
                ));
            }
            (false, true) => {
                return Err(format!(
                    "Take-Tree-Integrity: installierte Root-Datei ist nicht im eingehenden Vollbaum enthalten: {managed_file}"
                ));
            }
        }
    }
    Ok(())
}

pub fn validate_installed_manifest(
    project_root: &Path,
    current_version: &str,
    take_from_env: Option<&str>,
) -> Result<ChangeValidationSummary, String> {
    let manifest = read_manifest(&project_root.join(CHANGE_MANIFEST_RELATIVE_PATH))?;
    validate_versions(
        &manifest,
        take_from_env.unwrap_or(&manifest.from_version),
        current_version,
        Some("installed-build"),
    )?;
    validate_installed_hashes(&manifest, project_root)?;
    if !ssc_evidence_proof_shadow_permit_kodex::k2::checkpoint_prepare::writer_retirement_installed_manifest(project_root,manifest.change_class.as_str())? {
        validate_manifest_path_policy(&manifest)?;
    }
    let semantic = validate_pending_transaction_semantics(project_root, &manifest)?;
    Ok(summarize_declared_with_semantic(&manifest, &semantic))
}

pub fn read_manifest(path: &Path) -> Result<ChangeManifest, String> {
    let text = fs::read_to_string(path).map_err(|error| {
        format!(
            "Take-Change-Manifest nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    parse_manifest(&text)
}

fn parse_manifest(text: &str) -> Result<ChangeManifest, String> {
    let mut schema_version = None;
    let mut change_class = None;
    let mut from_version = None;
    let mut to_version = None;
    let mut reason = None;
    let mut kodex_writer_relocation = false;
    let mut kodex_negative_check_relocation = false;
    let mut kodex_negative_check_module_relocation = false;
    let mut warning_debt_closure = false;
    let mut authority_retirement_contract = None;
    let mut governed_changes = Vec::new();
    let mut music_effect = None;
    let mut changes = BTreeMap::new();

    for (index, raw_line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("Take-Change-Manifest Zeile {line_number}: '=' fehlt"))?;
        match key.trim() {
            "schema_version" => {
                schema_version = Some(value.trim().parse::<u32>().map_err(|error| {
                    format!("Take-Change-Manifest Zeile {line_number}: schema_version ungueltig: {error}")
                })?);
            }
            "change_class" => change_class = Some(ChangeClass::parse(value)?),
            "from_version" => from_version = Some(value.trim().to_string()),
            "to_version" => to_version = Some(value.trim().to_string()),
            "reason" => reason = Some(value.trim().to_string()),
            "kodex_writer_relocation" => {
                if value.trim() != "closed_internal_graph_v1" {
                    return Err(format!(
                        "Take-Change-Manifest Zeile {line_number}: kodex_writer_relocation erwartet closed_internal_graph_v1"
                    ));
                }
                kodex_writer_relocation = true;
            }
            "kodex_negative_check_relocation" => {
                if value.trim() != "closed_exact_negative_check_relocation_v1" {
                    return Err(format!(
                        "Take-Change-Manifest Zeile {line_number}: kodex_negative_check_relocation erwartet closed_exact_negative_check_relocation_v1"
                    ));
                }
                kodex_negative_check_relocation = true;
            }
            "kodex_negative_check_module_relocation" => {
                if value.trim() != "closed_private_to_parent_module_relocation_v1" {
                    return Err(format!(
                        "Take-Change-Manifest Zeile {line_number}: kodex_negative_check_module_relocation erwartet closed_private_to_parent_module_relocation_v1"
                    ));
                }
                kodex_negative_check_module_relocation = true;
            }
            "warning_debt_closure" => {
                if value.trim() != "closed_current_warning_debt_v1" {
                    return Err(format!(
                        "Take-Change-Manifest Zeile {line_number}: warning_debt_closure erwartet closed_current_warning_debt_v1"
                    ));
                }
                warning_debt_closure = true;
            }
            "authority_retirement_contract" => {
                let path = normalize_relative_path(value.trim())?;
                if !is_authority_retirement_contract_path(&path) {
                    return Err(format!(
                        "Take-Change-Manifest Zeile {line_number}: authority_retirement_contract erwartet einen konkreten docs/authority_retirement_contract_*.md Pfad"
                    ));
                }
                if authority_retirement_contract.replace(path).is_some() {
                    return Err(format!(
                        "Take-Change-Manifest Zeile {line_number}: authority_retirement_contract darf nur einmal vorkommen"
                    ));
                }
            }
            "govern" => {
                let line = governed::parse_govern_value(value).map_err(|error| {
                    format!("Take-Change-Manifest Zeile {line_number}: {error}")
                })?;
                governed_changes.push(format!("{}|{}|{}", line.kind, line.target, line.reason));
            }
            "music_effect" => {
                let effect = governed::parse_music_effect(value).map_err(|error| {
                    format!("Take-Change-Manifest Zeile {line_number}: {error}")
                })?;
                if music_effect.replace(effect).is_some() {
                    return Err(format!(
                        "Take-Change-Manifest Zeile {line_number}: music_effect darf nur einmal vorkommen"
                    ));
                }
            }
            "change" => {
                let fields = value.split('|').map(str::trim).collect::<Vec<_>>();
                if fields.len() != 3 {
                    return Err(format!(
                        "Take-Change-Manifest Zeile {line_number}: change erwartet kind|path|fnv64:<hex> oder '-'"
                    ));
                }
                let kind = ChangeKind::parse(fields[0])?;
                let path = normalize_relative_path(fields[1])?;
                if path == CHANGE_MANIFEST_RELATIVE_PATH {
                    return Err(format!(
                        "Take-Change-Manifest Zeile {line_number}: das Manifest darf sich nicht selbst deklarieren"
                    ));
                }
                let incoming_fnv1a64 = if fields[2] == "-" {
                    None
                } else {
                    let hex = fields[2].strip_prefix("fnv64:").ok_or_else(|| {
                        format!(
                            "Take-Change-Manifest Zeile {line_number}: Hash erwartet fnv64:<hex>"
                        )
                    })?;
                    Some(u64::from_str_radix(hex, 16).map_err(|error| {
                        format!("Take-Change-Manifest Zeile {line_number}: Hash ungueltig: {error}")
                    })?)
                };
                if kind == ChangeKind::Deleted && incoming_fnv1a64.is_some() {
                    return Err(format!(
                        "Take-Change-Manifest Zeile {line_number}: deleted muss Hash '-' tragen"
                    ));
                }
                if kind != ChangeKind::Deleted && incoming_fnv1a64.is_none() {
                    return Err(format!(
                        "Take-Change-Manifest Zeile {line_number}: added/modified braucht einen incoming FNV64"
                    ));
                }
                let declared = DeclaredChange {
                    kind,
                    path: path.clone(),
                    incoming_fnv1a64,
                };
                if changes.insert(path.clone(), declared).is_some() {
                    return Err(format!(
                        "Take-Change-Manifest Zeile {line_number}: doppelter Pfad {path}"
                    ));
                }
            }
            other => {
                return Err(format!(
                    "Take-Change-Manifest Zeile {line_number}: unbekannter Schluessel {other:?}"
                ));
            }
        }
    }

    let schema_version = schema_version.ok_or_else(|| "schema_version fehlt".to_string())?;
    if schema_version != 1 {
        return Err(format!(
            "nicht unterstuetzte schema_version {schema_version}"
        ));
    }
    let change_class = change_class.ok_or_else(|| "change_class fehlt".to_string())?;
    let from_version = nonempty(from_version, "from_version")?;
    let to_version = nonempty(to_version, "to_version")?;
    let reason = nonempty(reason, "reason")?;
    if changes.is_empty() {
        return Err("Take-Change-Manifest enthaelt keine deklarierten Aenderungen".to_string());
    }
    if change_class == ChangeClass::GovernedChange {
        if music_effect.is_none() {
            return Err(
                "Take-Change-Manifest: GovernedChange braucht music_effect=none oder music_effect=declared|<Begruendung>"
                    .to_string(),
            );
        }
    } else if !governed_changes.is_empty() || music_effect.is_some() {
        return Err(
            "Take-Change-Manifest: govern= und music_effect= gelten nur fuer GovernedChange"
                .to_string(),
        );
    }

    Ok(ChangeManifest {
        schema_version,
        change_class,
        from_version,
        to_version,
        reason,
        kodex_writer_relocation,
        kodex_negative_check_relocation,
        kodex_negative_check_module_relocation,
        warning_debt_closure,
        authority_retirement_contract,
        governed_changes,
        music_effect,
        changes,
    })
}

fn nonempty(value: Option<String>, name: &str) -> Result<String, String> {
    let value = value.ok_or_else(|| format!("{name} fehlt"))?;
    if value.trim().is_empty() {
        return Err(format!("{name} ist leer"));
    }
    Ok(value)
}

fn validate_versions(
    manifest: &ChangeManifest,
    current_version: &str,
    incoming_version: &str,
    context: Option<&str>,
) -> Result<(), String> {
    if !manifest.from_version.eq_ignore_ascii_case(current_version) {
        return Err(format!(
            "Take-Change-Control {}: from_version {} passt nicht zum aktuellen Stand {}",
            context.unwrap_or(""),
            manifest.from_version,
            current_version
        ));
    }
    if !manifest.to_version.eq_ignore_ascii_case(incoming_version) {
        return Err(format!(
            "Take-Change-Control {}: to_version {} passt nicht zum eingehenden Stand {}",
            context.unwrap_or(""),
            manifest.to_version,
            incoming_version
        ));
    }
    Ok(())
}

fn collect_actual_changes(
    source_root: &Path,
    current_root: &Path,
) -> Result<BTreeMap<String, DeclaredChange>, String> {
    let mut changes = BTreeMap::new();
    for managed_dir in MANAGED_DIRS {
        let source_dir = source_root.join(managed_dir);
        let current_dir = current_root.join(managed_dir);
        let source_files = relative_file_set(&source_dir)?;
        let current_files = relative_file_set(&current_dir)?;
        let all = source_files
            .union(&current_files)
            .cloned()
            .collect::<BTreeSet<_>>();
        for relative in all {
            let source_file = source_dir.join(&relative);
            let current_file = current_dir.join(&relative);
            let path = path_to_slashes(&Path::new(managed_dir).join(&relative));
            if path == CHANGE_MANIFEST_RELATIVE_PATH {
                continue;
            }
            let change = classify_file_change(&source_file, &current_file, path)?;
            if let Some(change) = change {
                changes.insert(change.path.clone(), change);
            }
        }
    }

    for managed_file in MANAGED_ROOT_FILES {
        let source_file = source_root.join(managed_file);
        let current_file = current_root.join(managed_file);
        if let Some(change) =
            classify_file_change(&source_file, &current_file, managed_file.to_string())?
        {
            changes.insert(change.path.clone(), change);
        }
    }
    Ok(changes)
}

fn validate_regular_tree(root: &Path, label: &str) -> Result<(), String> {
    let metadata = fs::symlink_metadata(root).map_err(|error| {
        format!(
            "Take-Tree-Integrity: Metadaten nicht lesbar {}: {error}",
            root.display()
        )
    })?;
    if metadata.file_type().is_symlink() {
        return Err(format!(
            "Take-Tree-Integrity: symbolischer Link im verwalteten Baum verboten: {label}"
        ));
    }
    if !metadata.is_dir() {
        return Err(format!(
            "Take-Tree-Integrity: verwalteter Baum ist kein Ordner: {label}"
        ));
    }
    validate_regular_tree_recursive(root, root, label)
}

fn validate_regular_tree_recursive(root: &Path, dir: &Path, label: &str) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|error| {
        format!(
            "Take-Tree-Integrity: Ordner nicht lesbar {}: {error}",
            dir.display()
        )
    })? {
        let entry = entry
            .map_err(|error| format!("Take-Tree-Integrity: Directory entry unlesbar: {error}"))?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            format!(
                "Take-Tree-Integrity: Metadaten nicht lesbar {}: {error}",
                path.display()
            )
        })?;
        let relative = path.strip_prefix(root).map_err(|error| {
            format!(
                "Take-Tree-Integrity: relativer Pfad fehlgeschlagen {}: {error}",
                path.display()
            )
        })?;
        let shown = format!("{label}/{}", path_to_slashes(relative));
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "Take-Tree-Integrity: symbolischer Link im verwalteten Baum verboten: {shown}"
            ));
        }
        if metadata.is_dir() {
            validate_regular_tree_recursive(root, &path, label)?;
        } else if !metadata.is_file() {
            return Err(format!(
                "Take-Tree-Integrity: nur regulaere Dateien und Ordner erlaubt: {shown}"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
fn validate_case_insensitive_path_set<I>(paths: I) -> Result<(), String>
where
    I: IntoIterator<Item = String>,
{
    let mut seen = BTreeMap::<String, String>::new();
    for path in paths {
        let key = path.to_ascii_lowercase();
        if let Some(previous) = seen.insert(key, path.clone())
            && previous != path {
            return Err(format!(
                "Take-Tree-Integrity: Windows-Pfadkollision durch Gross-/Kleinschreibung: {previous:?} und {path:?}"
            ));
        }
    }
    Ok(())
}

fn validate_case_insensitive_uniqueness(source_root: &Path) -> Result<(), String> {
    let mut seen = BTreeMap::<String, String>::new();
    for managed_dir in MANAGED_DIRS {
        let root = source_root.join(managed_dir);
        for relative in relative_file_set(&root)? {
            let path = path_to_slashes(&Path::new(managed_dir).join(relative));
            let key = path.to_ascii_lowercase();
            if let Some(previous) = seen.insert(key, path.clone())
                && previous != path
            {
                return Err(format!(
                    "Take-Tree-Integrity: Windows-Pfadkollision durch Gross-/Kleinschreibung: {previous:?} und {path:?}"
                ));
            }
        }
    }
    for managed_file in MANAGED_ROOT_FILES {
        if source_root.join(managed_file).is_file() {
            let key = managed_file.to_ascii_lowercase();
            if let Some(previous) = seen.insert(key, managed_file.to_string())
                && previous != managed_file
            {
                return Err(format!(
                    "Take-Tree-Integrity: Windows-Pfadkollision durch Gross-/Kleinschreibung: {previous:?} und {managed_file:?}"
                ));
            }
        }
    }
    Ok(())
}

fn files_differ(left: &Path, right: &Path) -> Result<bool, String> {
    if !left.is_file() || !right.is_file() {
        return Ok(true);
    }
    let left_meta = fs::metadata(left)
        .map_err(|error| format!("Dateimetadaten nicht lesbar {}: {error}", left.display()))?;
    let right_meta = fs::metadata(right)
        .map_err(|error| format!("Dateimetadaten nicht lesbar {}: {error}", right.display()))?;
    if left_meta.len() != right_meta.len() {
        return Ok(true);
    }
    let left_bytes = fs::read(left)
        .map_err(|error| format!("Datei nicht lesbar {}: {error}", left.display()))?;
    let right_bytes = fs::read(right)
        .map_err(|error| format!("Datei nicht lesbar {}: {error}", right.display()))?;
    Ok(left_bytes != right_bytes)
}

fn classify_file_change(
    source_file: &Path,
    current_file: &Path,
    path: String,
) -> Result<Option<DeclaredChange>, String> {
    match (source_file.is_file(), current_file.is_file()) {
        (true, false) => Ok(Some(DeclaredChange {
            kind: ChangeKind::Added,
            incoming_fnv1a64: Some(file_fnv1a64(source_file)?),
            path,
        })),
        (false, true) => Ok(Some(DeclaredChange {
            kind: ChangeKind::Deleted,
            incoming_fnv1a64: None,
            path,
        })),
        (true, true) => {
            let source_bytes = fs::read(source_file).map_err(|error| {
                format!("Quelldatei nicht lesbar {}: {error}", source_file.display())
            })?;
            let current_bytes = fs::read(current_file).map_err(|error| {
                format!("Zieldatei nicht lesbar {}: {error}", current_file.display())
            })?;
            if source_bytes == current_bytes {
                Ok(None)
            } else {
                Ok(Some(DeclaredChange {
                    kind: ChangeKind::Modified,
                    incoming_fnv1a64: Some(fnv1a64(&source_bytes)),
                    path,
                }))
            }
        }
        (false, false) => Ok(None),
    }
}

fn validate_declared_changes(
    manifest: &ChangeManifest,
    source_root: &Path,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<(), String> {
    let declared_paths = manifest.changes.keys().cloned().collect::<BTreeSet<_>>();
    let actual_paths = actual.keys().cloned().collect::<BTreeSet<_>>();
    if declared_paths != actual_paths {
        let undeclared = actual_paths
            .difference(&declared_paths)
            .cloned()
            .collect::<Vec<_>>();
        let missing = declared_paths
            .difference(&actual_paths)
            .cloned()
            .collect::<Vec<_>>();
        return Err(format!(
            "Take-Change-Control: deklarierter und tatsaechlicher Diff stimmen nicht. undeclared={undeclared:?}; declared_but_unchanged={missing:?}"
        ));
    }

    for (path, declared) in &manifest.changes {
        let actual_change = actual
            .get(path)
            .ok_or_else(|| format!("interner Diff fehlt fuer {path}"))?;
        if declared.kind != actual_change.kind {
            return Err(format!(
                "Take-Change-Control: {path} deklariert {}, tatsaechlich {}",
                declared.kind.as_str(),
                actual_change.kind.as_str()
            ));
        }
        if declared.incoming_fnv1a64 != actual_change.incoming_fnv1a64 {
            return Err(format!(
                "Take-Change-Control: incoming FNV64 stimmt fuer {path} nicht: declared={:?}, actual={:?}",
                declared.incoming_fnv1a64,
                actual_change.incoming_fnv1a64
            ));
        }
        if declared.kind != ChangeKind::Deleted {
            let file = source_root.join(path.replace('/', std::path::MAIN_SEPARATOR_STR));
            if !file.is_file() {
                return Err(format!(
                    "Take-Change-Control: deklarierte Quelldatei fehlt {path}"
                ));
            }
        }
    }
    Ok(())
}

fn validate_installed_hashes(manifest: &ChangeManifest, project_root: &Path) -> Result<(), String> {
    for change in manifest.changes.values() {
        let path = project_root.join(change.path.replace('/', std::path::MAIN_SEPARATOR_STR));
        match change.kind {
            ChangeKind::Deleted => {
                if path.exists() {
                    return Err(format!(
                        "Take-Change-Control installed-build: als deleted deklarierter Pfad existiert noch {}",
                        change.path
                    ));
                }
            }
            ChangeKind::Added | ChangeKind::Modified => {
                let actual = file_fnv1a64(&path)?;
                if Some(actual) != change.incoming_fnv1a64 {
                    return Err(format!(
                        "Take-Change-Control installed-build: FNV64 stimmt fuer {} nicht: declared={:?}, actual={actual:016x}",
                        change.path, change.incoming_fnv1a64
                    ));
                }
            }
        }
    }
    Ok(())
}

fn validate_pending_transaction_semantics(
    project_root: &Path,
    manifest: &ChangeManifest,
) -> Result<semantic_capability_diff::SemanticDiffSummary, String> {
    const MARKER: &str = "target/etuschlopf_take_transaction_pending.txt";
    const TRANSACTION_PARENT: &str = "target/etuschlopf_take_transactions";
    let marker = project_root.join(MARKER);
    let transaction_active = std::env::var("ETUSCHLOPF_TAKE_TRANSACTION_ACTIVE")
        .ok()
        .as_deref()
        == Some("1");
    if !marker.is_file() {
        if transaction_active {
            return Err(format!(
                "Semantic-Capability-Diff installed-build: aktive Take-Transaktion ohne Pending-Marker {}",
                marker.display()
            ));
        }
        return Ok(semantic_capability_diff::SemanticDiffSummary::default());
    }
    if !transaction_active {
        return Err(format!(
            "Semantic-Capability-Diff installed-build: Pending-Marker ohne aktive kontrollierte Take-Transaktion {}; zuerst Recovery ausfuehren",
            marker.display()
        ));
    }

    let raw = fs::read_to_string(&marker).map_err(|error| {
        format!(
            "Semantic-Capability-Diff installed-build: Pending-Marker nicht lesbar {}: {error}",
            marker.display()
        )
    })?;
    let transaction_root = PathBuf::from(raw.trim());
    let canonical_project = fs::canonicalize(project_root).map_err(|error| {
        format!(
            "Semantic-Capability-Diff installed-build: Projektwurzel nicht kanonisierbar {}: {error}",
            project_root.display()
        )
    })?;
    let expected_parent = fs::canonicalize(project_root.join(TRANSACTION_PARENT)).map_err(|error| {
        format!(
            "Semantic-Capability-Diff installed-build: Transaktionswurzel nicht kanonisierbar: {error}"
        )
    })?;
    let canonical_transaction = fs::canonicalize(&transaction_root).map_err(|error| {
        format!(
            "Semantic-Capability-Diff installed-build: Transaktionspfad nicht kanonisierbar {}: {error}",
            transaction_root.display()
        )
    })?;
    if !expected_parent.starts_with(&canonical_project)
        || canonical_transaction.parent() != Some(expected_parent.as_path())
    {
        return Err(format!(
            "Semantic-Capability-Diff installed-build: Pending-Marker verweist ausserhalb der verwalteten Transaktionswurzel: {}",
            canonical_transaction.display()
        ));
    }
    let state_path = canonical_transaction.join("state.txt");
    let state = fs::read_to_string(&state_path).map_err(|error| {
        format!(
            "Semantic-Capability-Diff installed-build: Transaktionszustand nicht lesbar {}: {error}",
            state_path.display()
        )
    })?;
    if state.trim() != "reexec" {
        return Err(format!(
            "Semantic-Capability-Diff installed-build: semantische Selbstpruefung ist nur im Zustand reexec erlaubt, erhalten {:?}",
            state.trim()
        ));
    }
    let backup_root = canonical_transaction.join("backup");
    for managed in MANAGED_DIRS {
        if !backup_root.join(managed).is_dir() {
            return Err(format!(
                "Semantic-Capability-Diff installed-build: Backup-Vollbaum fehlt {}",
                backup_root.join(managed).display()
            ));
        }
    }
    if !backup_root.join("root_files/Cargo.toml").is_file() {
        return Err(format!(
            "Semantic-Capability-Diff installed-build: Backup-Cargo.toml fehlt {}",
            backup_root.join("root_files/Cargo.toml").display()
        ));
    }
    let changed_paths = manifest.changes.keys().cloned().collect::<BTreeSet<_>>();
    receipt_surface_normalization_kodex::validate_transition(
        &backup_root,
        project_root,
        manifest.change_class.as_str(),
    )?;
    render_sovereignty_kodex::validate_transition(
        &backup_root,
        project_root,
        manifest.change_class.as_str(),
    )?;
    ssc_evidence_proof_shadow_permit_kodex::validate_transition(
        &backup_root,
        project_root,
        manifest.change_class.as_str(),
    )?;
    semantic_capability_diff::validate_semantic_change_with_governance_options(
        &backup_root,
        project_root,
        manifest.change_class.as_str(),
        &changed_paths,
        semantic_capability_diff::SemanticGovernanceOptions {
            kodex_writer_relocation: manifest.kodex_writer_relocation,
            kodex_negative_check_relocation: manifest.kodex_negative_check_relocation,
            kodex_negative_check_module_relocation:
                manifest.kodex_negative_check_module_relocation,
            authority_retirement: manifest.authority_retirement_contract.is_some(),
        },
    )
}

fn validate_change_class_policy(
    manifest: &ChangeManifest,
    source_root: &Path,
    current_root: &Path,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<(), String> {
    validate_manifest_path_policy_for_tree(manifest, current_root, source_root)?;
    receipt_surface_normalization_kodex::validate_transition(
        current_root,
        source_root,
        manifest.change_class.as_str(),
    )?;
    render_sovereignty_kodex::validate_transition(
        current_root,
        source_root,
        manifest.change_class.as_str(),
    )?;
    ssc_evidence_proof_shadow_permit_kodex::validate_transition(
        current_root,
        source_root,
        manifest.change_class.as_str(),
    )?;
    if manifest.warning_debt_closure {
        if !matches!(
            manifest.change_class,
            ChangeClass::RefactorOnly | ChangeClass::KodexOnly
        ) {
            return Err(
                "Take-Change-Control: warning_debt_closure ist nur fuer RefactorOnly oder KodexOnly erlaubt"
                    .to_string(),
            );
        }
        if actual.keys().any(|path| path == "Cargo.toml") {
            return Err(
                "Take-Change-Control: warning_debt_closure darf Cargo.toml nicht aendern"
                    .to_string(),
            );
        }
    }
    validate_architecture_pin_coordination(current_root, actual)?;
    validate_corpus_independent_test_admission(source_root, current_root, actual)?;
    if manifest.change_class == ChangeClass::AuditOnly {
        validate_audit_only_added_lines(source_root, current_root, actual)?;
        validate_audit_only_cross_role_retirement(source_root, current_root, actual)?;
    }
    validate_authority_retirement_governance(manifest, source_root, current_root, actual)?;
    validate_selector_derivation_governance(source_root, current_root, actual)?;
    if manifest.kodex_writer_relocation {
        if manifest.change_class != ChangeClass::KodexOnly {
            return Err(
                "Take-Change-Control: kodex_writer_relocation ist nur fuer KodexOnly erlaubt"
                    .to_string(),
            );
        }
        let foreign_source_paths = actual
            .keys()
            .filter(|path| path.ends_with(".rs") || path.ends_with(".cmd"))
            .filter(|path| path_role(path) != PathRole::Kodex)
            .cloned()
            .collect::<Vec<_>>();
        if !foreign_source_paths.is_empty() {
            return Err(format!(
                "Take-Change-Control: kodex_writer_relocation enthaelt fremde Source-Pfade {foreign_source_paths:?}"
            ));
        }
    }
    if manifest.kodex_negative_check_relocation {
        if manifest.change_class != ChangeClass::KodexOnly {
            return Err(
                "Take-Change-Control: kodex_negative_check_relocation ist nur fuer KodexOnly erlaubt"
                    .to_string(),
            );
        }
        let foreign_source_paths = actual
            .keys()
            .filter(|path| path.ends_with(".rs") || path.ends_with(".cmd"))
            .filter(|path| path_role(path) != PathRole::Kodex)
            .cloned()
            .collect::<Vec<_>>();
        if !foreign_source_paths.is_empty() {
            return Err(format!(
                "Take-Change-Control: kodex_negative_check_relocation enthaelt fremde Source-Pfade {foreign_source_paths:?}"
            ));
        }
    }

    if manifest.kodex_negative_check_relocation
        && manifest.kodex_negative_check_module_relocation
    {
        return Err(
            "Take-Change-Control: exacte und private-to-parent KodexNegativeCheck-Relocation duerfen nicht gleichzeitig deklariert werden"
                .to_string(),
        );
    }
    if manifest.kodex_negative_check_module_relocation {
        if manifest.change_class != ChangeClass::KodexOnly {
            return Err(
                "Take-Change-Control: kodex_negative_check_module_relocation ist nur fuer KodexOnly erlaubt"
                    .to_string(),
            );
        }
        let foreign_source_paths = actual
            .keys()
            .filter(|path| path.ends_with(".rs") || path.ends_with(".cmd"))
            .filter(|path| path_role(path) != PathRole::Kodex)
            .cloned()
            .collect::<Vec<_>>();
        if !foreign_source_paths.is_empty() {
            return Err(format!(
                "Take-Change-Control: kodex_negative_check_module_relocation enthaelt fremde Source-Pfade {foreign_source_paths:?}"
            ));
        }
    }
    validate_validation_only_contract(manifest, actual)?;
    validate_cargo_manifest_change_contract(manifest, source_root, current_root, actual)?;
    validate_watch_cargo_manifest_change_contract(actual)?;

    if manifest.change_class == ChangeClass::MechanicalOnly {
        let source_changes = actual
            .values()
            .filter(|change| change.path.starts_with("src/"))
            .collect::<Vec<_>>();
        if source_changes.len() > 3 {
            return Err(format!(
                "Take-Change-Control: MechanicalOnly erlaubt hoechstens drei Source-Dateien, erhalten {}",
                source_changes.len()
            ));
        }
        if source_changes
            .iter()
            .any(|change| change.kind != ChangeKind::Modified)
        {
            return Err(
                "Take-Change-Control: MechanicalOnly darf Source-Dateien weder anlegen noch loeschen"
                    .to_string(),
            );
        }
    }
    Ok(())
}

fn architecture_pinned_changed_paths(
    current_root: &Path,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<Vec<String>, String> {
    const TRUST_ANCHOR: &str = "src/identity_gate_trust_anchor.rs";
    let anchor = fs::read_to_string(current_root.join(TRUST_ANCHOR)).map_err(|error| {
        format!("Take-Change-Control: Architecture-Anchor nicht lesbar: {error}")
    })?;
    let pinned = anchor
        .split("include_bytes!(\"")
        .skip(1)
        .map(|tail| {
            let (relative, _) = tail.split_once("\")").ok_or_else(|| {
                "Take-Change-Control: unvollstaendiger include_bytes!-Pin".to_string()
            })?;
            Ok(relative
                .strip_prefix("../")
                .map_or_else(|| format!("src/{relative}"), str::to_string))
        })
        .collect::<Result<BTreeSet<_>, String>>()?;
    Ok(actual
        .keys()
        .filter(|path| pinned.contains(*path))
        .cloned()
        .collect())
}

fn validate_architecture_pin_coordination(
    current_root: &Path,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<(), String> {
    const TRUST_ANCHOR: &str = "src/identity_gate_trust_anchor.rs";
    let changed = architecture_pinned_changed_paths(current_root, actual)?;
    if changed.is_empty() || actual.contains_key(TRUST_ANCHOR) {
        return Ok(());
    }
    // The installed parent binds both exact retired-population/origin data
    // successors. The child changes no trust-anchor or validator source.
    if changed.iter().all(|path| actual.get(path).is_some_and(|change|
        ssc_evidence_proof_shadow_permit_kodex::k2::cursor_temporal::bound_path_admissible(path, change.incoming_fnv1a64))) {
        return Ok(());
    }
    Err(format!(
        "Take-Change-Control: gepinnte Architekturpfade {changed:?} ohne synchronisierten Pin-Eigentuemer {TRUST_ANCHOR}; Ablehnung vor Kopieren"
    ))
}

fn count_path_roles(actual: &BTreeMap<String, DeclaredChange>) -> (usize, usize, usize, usize) {
    let mut kodex = 0usize;
    let mut audit = 0usize;
    let mut music_sensitive = 0usize;
    let mut metadata = 0usize;
    for path in actual.keys() {
        match path_role(path) {
            PathRole::Kodex => kodex += 1,
            PathRole::Audit => audit += 1,
            PathRole::MusicSensitive => music_sensitive += 1,
            PathRole::Metadata => metadata += 1,
            PathRole::OtherSource | PathRole::Unknown => {}
        }
    }
    (kodex, audit, music_sensitive, metadata)
}

fn validate_manifest_path_policy_for_tree(manifest: &ChangeManifest, parent: &Path, child: &Path) -> Result<(),String> {
    if !ssc_evidence_proof_shadow_permit_kodex::k2::checkpoint_prepare::writer_retirement_transition(parent,child,manifest.change_class.as_str())? {
        return validate_manifest_path_policy(manifest);
    }
    // The full exact first transition has been verified, including the anchor's
    // parent-derived bytes and complete changed-path set. Preserve normal policy
    // for every path outside the reviewed tuple (which must be empty).
    for change in manifest.changes.values() {
        if !ssc_evidence_proof_shadow_permit_kodex::k2::checkpoint_prepare::writer_retirement_bound_path(parent,child,&change.path,manifest.change_class.as_str()) {
            return Err(format!("Take-Change-Control: unbound writer-retirement path {}",change.path));
        }
    }
    Ok(())
}

fn validate_manifest_path_policy(manifest: &ChangeManifest) -> Result<(), String> {
    let governed_lines = manifest
        .governed_changes
        .iter()
        .map(|line| governed::parse_govern_value(line))
        .collect::<Result<Vec<_>, _>>()?;
    for change in manifest.changes.values() {
        let role = path_role(&change.path);
        let allowed = match manifest.change_class {
            ChangeClass::KodexOnly => matches!(role, PathRole::Kodex | PathRole::Metadata),
            ChangeClass::AuditOnly => {
                matches!(role, PathRole::Audit | PathRole::Metadata)
                    || (change.kind == ChangeKind::Modified
                        && matches!(role, PathRole::MusicSensitive | PathRole::OtherSource))
            }
            ChangeClass::OutcomeProjection => {
                matches!(role, PathRole::Kodex | PathRole::Audit | PathRole::Metadata)
            }
            ChangeClass::RefactorOnly => !matches!(role, PathRole::Kodex | PathRole::Unknown),
            ChangeClass::MusicMutation => {
                ssc_evidence_proof_shadow_permit_kodex::k2::cursor_temporal::bound_path_admissible(&change.path, change.incoming_fnv1a64)
                || matches!(
                    role,
                    PathRole::MusicSensitive | PathRole::OtherSource | PathRole::Metadata
                ) || (role == PathRole::Audit
                    && change.kind == ChangeKind::Modified
                    && change.path == "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs"
                    && manifest.authority_retirement_contract.is_some())
            }
            ChangeClass::CalibrationOnly => {
                matches!(
                    role,
                    PathRole::MusicSensitive | PathRole::OtherSource | PathRole::Metadata
                )
            }
            ChangeClass::MechanicalOnly => {
                matches!(
                    role,
                    PathRole::OtherSource | PathRole::Audit | PathRole::Metadata
                ) && !mechanical_forbidden_path(&change.path)
            }
            ChangeClass::ValidationOnly => {
                matches!(
                    role,
                    PathRole::OtherSource
                        | PathRole::MusicSensitive
                        | PathRole::Audit
                        | PathRole::Metadata
                )
            }
            ChangeClass::GovernedChange => {
                matches!(
                    role,
                    PathRole::OtherSource
                        | PathRole::MusicSensitive
                        | PathRole::Audit
                        | PathRole::Metadata
                ) || (role == PathRole::Kodex
                    && governed::kodex_path_admissible(&change.path, &governed_lines))
            }
        };
        if !allowed {
            return Err(format!(
                "Take-Change-Control: Klasse {} darf Pfad {} ({role:?}) nicht aendern",
                manifest.change_class.as_str(),
                change.path
            ));
        }
    }
    Ok(())
}

fn path_role(path: &str) -> PathRole {
    if path == "Cargo.toml"
        || path == "README.md"
        || path == "ZIP_KIND.txt"
        || path.starts_with("docs/")
    {
        return PathRole::Metadata;
    }
    if is_kodex_path(path) {
        return PathRole::Kodex;
    }
    // r528: An explicit owner root defines PathRole before lexical subject-risk markers.
    // Music-sensitive words still classify otherwise unowned source paths below.
    if is_audit_path(path) {
        return PathRole::Audit;
    }
    if is_music_sensitive_path(path) {
        return PathRole::MusicSensitive;
    }
    if path.starts_with("src/") || path.starts_with("tests/") || path.starts_with("tools/") {
        return PathRole::OtherSource;
    }
    PathRole::Unknown
}

fn is_kodex_path(path: &str) -> bool {
    path == WATCH_PROJECT_MANIFEST_RELATIVE_PATH
        || path == WATCH_PROJECT_LOCK_RELATIVE_PATH
        || path == "run.cmd"
        || path == "strict.cmd"
        || path == "rust-toolchain.toml"
        || path == "rustfmt.toml"
        || path == "clippy.toml"
        || path == "src/bin/etuschlopf_take_bootstrap.rs"
        || path == "src/bin/etuschlopf_launcher.rs"
        || path.starts_with("src/bin/etuschlopf_launcher/")
        || path == "src/visual_preflight.rs"
        || path.starts_with("src/visual_preflight/")
        || path == "src/kodex_control_plane/public_artifact_recovery.rs"
        || path == "src/take_change_control.rs"
        || path == "src/identity_debt_build.rs"
        || path == "src/kodex_build_orchestrator.rs"
        || path == "src/identity_gate_trust_anchor.rs"
        || path == "src/identity_debt_inventory.rs"
        || path == "src/feedback_structuralization_gate.rs"
        || path == "src/feedback_proof_transport.rs"
        || path.starts_with("src/build_gate/")
        || path == "tests/bootstrap_gate_isolation.rs"
        || path == "tests/global_identity_debt_inventory.rs"
        || path.starts_with("tools/RUST_")
}

fn is_music_sensitive_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    path.starts_with("src/structural_write_authority/")
        || lower.contains("writer")
        || lower.contains("write_authority")
        || lower.contains("musicxml")
        || lower.contains("notation")
        || lower.contains("render.rs")
        || lower.contains("rules.rs")
        || lower.contains("pass_order")
        || lower.contains("threshold")
        || lower.contains("apply.rs")
        || lower.contains("repair.rs")
}

fn is_audit_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    path.starts_with("src/audit_only/")
        || path.starts_with("src/audits/")
        || path.starts_with("src/proof_artifacts/")
        || path == "src/main.rs"
        || path == "src/report.rs"
        || path == "src/final_report.rs"
        || path == "src/corpus_run_manifest.rs"
        || path == "src/pipeline_shards/final_report_projection_owner.rs"
        || path == "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs"
        || ((path.starts_with("tests/") || path.starts_with("tools/"))
            && (lower.contains("audit")
                || lower.contains("proof")
                || lower.contains("readback")
                || lower.contains("diagnostic")))
}

fn mechanical_forbidden_path(path: &str) -> bool {
    is_kodex_path(path) || is_music_sensitive_path(path)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CargoManifestChangeKind {
    VersionOnly,
    WindowsJobDependencyV1,
    Other,
}

fn validate_cargo_manifest_change_contract(
    manifest: &ChangeManifest,
    source_root: &Path,
    current_root: &Path,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<(), String> {
    if !actual.contains_key("Cargo.toml") {
        return Ok(());
    }
    match cargo_manifest_change_kind(
        &current_root.join("Cargo.toml"),
        &source_root.join("Cargo.toml"),
    )? {
        CargoManifestChangeKind::VersionOnly => {}
        CargoManifestChangeKind::WindowsJobDependencyV1
            if manifest.change_class == ChangeClass::KodexOnly => {}
        CargoManifestChangeKind::WindowsJobDependencyV1 | CargoManifestChangeKind::Other => {
            return Err(format!(
                "Take-Change-Control: {} darf Cargo.toml im steady-state nur in der package-Version aendern; abgeschlossene Dependency-/Produktvertragsmigrationen besitzen keine aktive Admission-Ausnahme",
                manifest.change_class.as_str()
            ));
        }
    }
    Ok(())
}

const SCHEMASCHLOPF_R021_COMPONENT_PREFIX: &str =
    "tools/qualified_components/schemaschlopf_r021/";

fn validate_validation_only_contract(
    manifest: &ChangeManifest,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<(), String> {
    if manifest.change_class == ChangeClass::ValidationOnly {
        for path in actual.keys() {
            let component_path = path.starts_with(SCHEMASCHLOPF_R021_COMPONENT_PREFIX);
            let allowed = !component_path
                && (path == "Cargo.toml"
                    || path == "ZIP_KIND.txt"
                    || path.starts_with("docs/")
                    || matches!(
                        path.as_str(),
                        "src/persisted_schema_validation.rs"
                            | "src/pipeline.rs"
                            | "src/render.rs"
                            | "tests/persisted_schema_validation_gate.rs"
                    ));
            if !allowed {
                return Err(format!(
                    "Take-Change-Control: ValidationOnly darf Pfad {path} nicht aendern; qualifizierte SchemaSchlopf-r021-Komponentenbytes bleiben im steady-state geschlossen"
                ));
            }
        }
    }
    Ok(())
}

fn validate_watch_cargo_manifest_change_contract(
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<(), String> {
    if actual.contains_key(WATCH_PROJECT_LOCK_RELATIVE_PATH) {
        return Err(
            "Take-Change-Control: Watch-Cargo.lock ist Kodex-Source und bleibt ohne eigenen expliziten Dependency-/Lock-Vertrag unveraendert"
                .to_string(),
        );
    }
    if actual.contains_key(WATCH_PROJECT_MANIFEST_RELATIVE_PATH) {
        return Err(
            "Take-Change-Control: tools/etuschlopf_watch/Cargo.toml ist steady-state Kodex-Source und bleibt ohne neuen expliziten Zukunftsvertrag unveraendert"
                .to_string(),
        );
    }
    Ok(())
}

// One exact additive transition, consumed by both manifest and semantic admission.
// All pre-existing lines (including package version) must remain unchanged.
// The target table closes replay: removal or modification of it is never admitted.
const WINDOWS_JOB_DEPENDENCY_V1: &str = concat!(
    "[target.'cfg(windows)'.dependencies]\n",
    "windows-sys = { version = \"=0.61.2\", default-features = false, features = [",
    "\"Win32_Foundation\", \"Win32_Security\", \"Win32_System_JobObjects\", ",
    "\"Win32_System_SystemServices\", \"Win32_System_Threading\"] }\n",
);

fn windows_job_dependency_transition_v1(current: &str, incoming: &str) -> bool {
    if current.lines().any(|line| {
        line == "[target.'cfg(windows)'.dependencies]"
            || line.trim_start().starts_with("windows-sys")
    }) {
        return false;
    }
    current
        .lines()
        .chain(WINDOWS_JOB_DEPENDENCY_V1.lines())
        .eq(incoming.lines())
}

fn cargo_manifest_change_kind(
    current: &Path,
    incoming: &Path,
) -> Result<CargoManifestChangeKind, String> {
    let current_text = fs::read_to_string(current)
        .map_err(|error| format!("Cargo.toml current nicht lesbar: {error}"))?;
    let incoming_text = fs::read_to_string(incoming)
        .map_err(|error| format!("Cargo.toml incoming nicht lesbar: {error}"))?;
    if windows_job_dependency_transition_v1(&current_text, &incoming_text) {
        return Ok(CargoManifestChangeKind::WindowsJobDependencyV1);
    }
    let current_lines = current_text.lines().collect::<Vec<_>>();
    let incoming_lines = incoming_text.lines().collect::<Vec<_>>();
    if current_lines.len() != incoming_lines.len() {
        return Ok(CargoManifestChangeKind::Other);
    }
    let mut section = "";
    let mut version_changes = 0usize;
    for (current_line, incoming_line) in current_lines.iter().zip(incoming_lines.iter()) {
        let current_trimmed = current_line.trim();
        let incoming_trimmed = incoming_line.trim();
        if current_trimmed.starts_with('[') && current_trimmed.ends_with(']') {
            if current_trimmed != incoming_trimmed {
                return Ok(CargoManifestChangeKind::Other);
            }
            section = current_trimmed;
            continue;
        }
        if current_line == incoming_line {
            continue;
        }
        let package_version_line = section == "[package]"
            && current_trimmed.starts_with("version = ")
            && incoming_trimmed.starts_with("version = ");
        if package_version_line {
            version_changes += 1;
            continue;
        }
        return Ok(CargoManifestChangeKind::Other);
    }
    Ok(if version_changes == 1 {
        CargoManifestChangeKind::VersionOnly
    } else {
        CargoManifestChangeKind::Other
    })
}

fn validate_audit_only_added_lines(
    source_root: &Path,
    current_root: &Path,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<(), String> {
    const FORBIDDEN_ADDED_TOKENS: [&str; 12] = [
        "structural_write_authority::",
        "crate::structural_write_authority",
        "replace_range(",
        "replace_xml",
        "rewrite_",
        "apply_authorized",
        "apply_music",
        "render_xml_write: true",
        "music_mutation: true",
        "master_apply: true",
        "writer_eligible: true",
        "write_capability_defined: true",
    ];
    for change in actual.values() {
        if change.kind == ChangeKind::Deleted
            || !change.path.starts_with("src/")
            || !change.path.ends_with(".rs")
        {
            continue;
        }
        let incoming_path =
            source_root.join(change.path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let incoming = fs::read_to_string(&incoming_path).map_err(|error| {
            format!(
                "AuditOnly incoming source nicht lesbar {}: {error}",
                change.path
            )
        })?;
        let current_path =
            current_root.join(change.path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let current = fs::read_to_string(&current_path).unwrap_or_default();
        for added_line in added_lines(&current, &incoming) {
            let lower = added_line.to_ascii_lowercase();
            if let Some(token) = FORBIDDEN_ADDED_TOKENS
                .iter()
                .find(|token| lower.contains(**token))
            {
                return Err(format!(
                    "Take-Change-Control: AuditOnly fuegt in {} verbotenen Mutations-/Writer-Token {token:?} ein: {}",
                    change.path,
                    added_line.trim()
                ));
            }
        }
    }
    Ok(())
}

fn validate_audit_only_cross_role_retirement(
    source_root: &Path,
    current_root: &Path,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<(), String> {
    for change in actual.values() {
        let role = path_role(&change.path);
        if matches!(role, PathRole::Audit | PathRole::Metadata) {
            continue;
        }
        if !matches!(role, PathRole::MusicSensitive | PathRole::OtherSource)
            || change.kind != ChangeKind::Modified
            || !change.path.starts_with("src/")
            || !change.path.ends_with(".rs")
        {
            return Err(format!(
                "Take-Change-Control: AuditOnly Cross-Role-Rueckbau erlaubt nur modifizierte bestehende Rust-Source-Dateien, erhalten {} ({role:?}, {})",
                change.path,
                change.kind.as_str()
            ));
        }

        let incoming_path =
            source_root.join(change.path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let current_path =
            current_root.join(change.path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let incoming = fs::read_to_string(&incoming_path).map_err(|error| {
            format!(
                "AuditOnly Cross-Role incoming source nicht lesbar {}: {error}",
                change.path
            )
        })?;
        let current = fs::read_to_string(&current_path).map_err(|error| {
            format!(
                "AuditOnly Cross-Role current source nicht lesbar {}: {error}",
                change.path
            )
        })?;
        let removed = removed_lines(&current, &incoming);
        for added in added_lines(&current, &incoming) {
            if !removed
                .iter()
                .any(|old| strict_braced_use_reduction(old, added))
            {
                return Err(format!(
                    "Take-Change-Control: AuditOnly darf in gemischter Source {} nur loeschen oder eine bestehende geschweifte use-Liste strikt verkleinern; unzulaessige neue Zeile: {}",
                    change.path,
                    added.trim()
                ));
            }
        }
    }
    Ok(())
}

const AUTHORITY_RETIREMENT_CONTRACT_PREFIX: &str = "docs/authority_retirement_contract_";
const AUTHORITY_RETIREMENT_CONTRACT_TEMPLATE: &str = "docs/authority_retirement_contract_template.md";
const AUTHORITY_RETIREMENT_CONTRACT_VERSION: &str = "authority_retirement_v1";

fn is_authority_retirement_contract_path(path: &str) -> bool {
    path.starts_with(AUTHORITY_RETIREMENT_CONTRACT_PREFIX)
        && path.ends_with(".md")
        && path != AUTHORITY_RETIREMENT_CONTRACT_TEMPLATE
}

fn contract_value<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    text.lines().find_map(|raw| {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }
        let (name, value) = line.split_once(':')?;
        (name.trim() == key).then_some(value.trim())
    })
}

fn require_authority_retirement_value(
    text: &str,
    path: &str,
    key: &str,
    expected: &str,
) -> Result<(), String> {
    let actual = contract_value(text, key).unwrap_or("<missing>");
    if actual != expected {
        return Err(format!(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: {path} requires {key}: {expected}, got {actual:?}"
        ));
    }
    Ok(())
}

fn authority_retirement_scope_paths(text: &str, path: &str) -> Result<BTreeSet<String>, String> {
    let raw = contract_value(text, "RetirementScopePaths").ok_or_else(|| {
        format!(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: {path} requires RetirementScopePaths"
        )
    })?;
    let mut out = BTreeSet::new();
    for item in raw.split(',').map(str::trim).filter(|item| !item.is_empty()) {
        let normalized = normalize_relative_path(item)?;
        if !normalized.starts_with("src/") || !normalized.ends_with(".rs") {
            return Err(format!(
                "Take-Change-Control AUTHORITY-RETIREMENT KODEX: {path} RetirementScopePaths contains non-Rust/non-src path {normalized:?}"
            ));
        }
        out.insert(normalized);
    }
    if out.is_empty() {
        return Err(format!(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: {path} RetirementScopePaths must not be empty"
        ));
    }
    Ok(out)
}

fn selector_or_multiplicity_relation_changed(current: &str, incoming: &str) -> bool {
    let added = added_lines(current, incoming);
    let removed = removed_lines(current, incoming);
    added_lines_are_selector_sensitive(&added)
        || added_lines_are_multiplicity_sensitive(&added)
        || added_lines_are_selector_sensitive(&removed)
        || added_lines_are_multiplicity_sensitive(&removed)
}

fn validate_authority_retirement_contract_text(text: &str, path: &str) -> Result<(), String> {
    for (key, expected) in [
        (
            "AuthorityRetirementContract",
            AUTHORITY_RETIREMENT_CONTRACT_VERSION,
        ),
        ("AuthorityChange", "retirement_only"),
        ("CandidateClassChange", "forbidden"),
        ("SelectorRelationChange", "forbidden"),
        ("TargetSelectionChange", "forbidden"),
        ("MultiplicityDecisionChange", "forbidden"),
        ("NewPositiveWriteDecision", "forbidden"),
        ("ReplacementWriter", "forbidden"),
        ("WriterCapabilityDirection", "decrease_only"),
        ("CalibrationThresholdChange", "forbidden"),
        ("ProductDeltaScope", "removal_of_parent_writes_only"),
        ("CausalClassFirstBypass", "forbidden"),
        ("CandidateDuplicationBypass", "forbidden"),
    ] {
        require_authority_retirement_value(text, path, key, expected)?;
    }
    let surface = contract_value(text, "ParentAuthoritySurface").unwrap_or("");
    let Some((surface_path, symbol)) = surface.rsplit_once("::") else {
        return Err(format!(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: {path} ParentAuthoritySurface must be path::symbol"
        ));
    };
    let surface_path = normalize_relative_path(surface_path)?;
    if !surface_path.starts_with("src/") || symbol.trim().is_empty() {
        return Err(format!(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: {path} ParentAuthoritySurface is invalid"
        ));
    }
    let _ = authority_retirement_scope_paths(text, path)?;
    Ok(())
}

fn validate_authority_retirement_governance(
    manifest: &ChangeManifest,
    source_root: &Path,
    current_root: &Path,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<(), String> {
    let requires_retirement = semantic_capability_diff::requires_authority_retirement_contract(
        current_root,
        source_root,
    )?;
    let changed_contracts = actual
        .values()
        .filter(|change| change.kind != ChangeKind::Deleted)
        .map(|change| change.path.as_str())
        .filter(|path| is_authority_retirement_contract_path(path))
        .collect::<Vec<_>>();

    let Some(contract_path) = manifest.authority_retirement_contract.as_deref() else {
        if !changed_contracts.is_empty() {
            return Err(format!(
                "Take-Change-Control AUTHORITY-RETIREMENT KODEX: changed retirement contract {changed_contracts:?} requires explicit authority_retirement_contract manifest binding"
            ));
        }
        if requires_retirement {
            return Err(
                "Take-Change-Control AUTHORITY-RETIREMENT KODEX: pure productive writer-authority loss requires an explicit authority_retirement_v1 contract; selector/threshold rules may not be bypassed by deletion-only changes"
                    .to_string(),
            );
        }
        return Ok(());
    };

    if manifest.change_class != ChangeClass::MusicMutation {
        return Err(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: authority_retirement_v1 is only valid for MusicMutation"
                .to_string(),
        );
    }
    if changed_contracts.len() != 1 || changed_contracts[0] != contract_path {
        return Err(format!(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: manifest binds {contract_path:?}, but changed implementation contracts are {changed_contracts:?}"
        ));
    }
    if !requires_retirement {
        return Err(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: retirement contract declared without a pure monotone productive writer-authority loss"
                .to_string(),
        );
    }

    let text = fs::read_to_string(
        source_root.join(contract_path.replace('/', std::path::MAIN_SEPARATOR_STR)),
    )
    .map_err(|error| {
        format!(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: contract unreadable {contract_path}: {error}"
        )
    })?;
    validate_authority_retirement_contract_text(&text, contract_path)?;
    let scope_paths = authority_retirement_scope_paths(&text, contract_path)?;

    let changed_source_paths = actual
        .values()
        .filter(|change| change.path.starts_with("src/") && change.path.ends_with(".rs"))
        .filter(|change| path_role(&change.path) != PathRole::Kodex)
        .map(|change| change.path.clone())
        .collect::<BTreeSet<_>>();
    if changed_source_paths != scope_paths {
        return Err(format!(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: RetirementScopePaths must exactly equal the non-Kodex changed Rust source set; contract={scope_paths:?}; actual={changed_source_paths:?}"
        ));
    }

    let surface = contract_value(&text, "ParentAuthoritySurface").unwrap_or("");
    let (surface_path, symbol) = surface.rsplit_once("::").expect("validated surface");
    let surface_path = normalize_relative_path(surface_path)?;
    if !scope_paths.contains(&surface_path) {
        return Err(format!(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: ParentAuthoritySurface path {surface_path} is outside RetirementScopePaths"
        ));
    }
    let parent_surface = fs::read_to_string(
        current_root.join(surface_path.replace('/', std::path::MAIN_SEPARATOR_STR)),
    )
    .map_err(|error| {
        format!(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: parent authority surface unreadable {surface_path}: {error}"
        )
    })?;
    if !parent_surface.contains(&format!("fn {}", symbol.trim())) {
        return Err(format!(
            "Take-Change-Control AUTHORITY-RETIREMENT KODEX: ParentAuthoritySurface {surface} does not bind an existing parent function"
        ));
    }

    if scope_paths.iter().any(|path| path_role(path) == PathRole::Audit)
        && contract_value(&text, "SscExitObligations").is_none()
    {
        return Err("SSC cross-role retirement requires exact declared structural erasure; generic authority retirement cannot change an Audit owner".into());
    }
    let ssc_erasure = contract_value(&text, "SscExitObligations").is_some();
    let ssc_context = if ssc_erasure {
        Some(validate_ssc_retirement_erasure(&text, source_root, current_root, &scope_paths)?)
    } else {
        None
    };
    for source_path in &scope_paths {
        let current = fs::read_to_string(
            current_root.join(source_path.replace('/', std::path::MAIN_SEPARATOR_STR)),
        )
        .unwrap_or_default();
        let incoming = fs::read_to_string(
            source_root.join(source_path.replace('/', std::path::MAIN_SEPARATOR_STR)),
        )
        .unwrap_or_default();
        if let Some(context) = &ssc_context {
            verify_ssc_retirement_erasure_in_path(context, source_path, &current, &incoming)?;
        }
        if selector_or_multiplicity_relation_changed(&current, &incoming) && !ssc_erasure {
            return Err(format!(
                "Take-Change-Control AUTHORITY-RETIREMENT KODEX: {source_path} changes selector/candidate/multiplicity decision semantics. Retirement may remove write authority only; causal_class_first_v1 and candidate_duplication_v1 remain fully binding."
            ));
        }
    }

    semantic_capability_diff::validate_authority_retirement_semantics_v1(current_root, source_root)
}


struct SscRetirementErasureContext {
    retired: BTreeSet<String>,
    regions: BTreeMap<String, Vec<String>>,
}

fn validate_ssc_retirement_erasure(
    text: &str, root: &Path, parent_root: &Path, paths: &BTreeSet<String>,
) -> Result<SscRetirementErasureContext, String> {
    if !ssc_evidence_proof_shadow_permit_kodex::k2::installed(root) {
        return Err("SSC retirement needs installed K2 governance".into());
    }
    let ids=contract_value(text,"SscExitObligations").ok_or("SSC obligations missing")?;
    let allowed=include_str!("build_gate/ssc_exit_dependencies_r1238_v1.tsv");
    for id in ids.split(',').map(str::trim) {
        if id.is_empty() || !allowed.lines().skip(2).any(|line|line.split('\t').next()==Some(id)) {
            return Err("SSC retirement binds an unknown obligation".into());
        }
    }
    if ids.split(',').any(|id| id.trim() == "e031_tenuto") {
        require_authority_retirement_value(text,"SSC retirement","SscRetirementEvidenceEffects","e031_probe_receipt_mints_and_discards_removed")?;
    }
    let symbols=contract_value(text,"RetiredAuthoritySymbols").ok_or("SSC retired symbols missing")?;
    ssc_evidence_proof_shadow_permit_kodex::k2::validate_obligation_symbol_binding(parent_root,ids,symbols)?;
    let retired=symbols.split(',').map(str::trim).map(|symbol|symbol.rsplit("::").next().unwrap_or("").to_string()).collect::<BTreeSet<_>>();
    if retired.is_empty() || retired.contains("") {return Err("SSC retired symbol set empty".into());}
    let edges=contract_value(text,"RemovedCallerEdges").ok_or("SSC removed edges missing")?;
    if edges.trim().is_empty(){return Err("SSC retired edges empty".into());}
    ssc_evidence_proof_shadow_permit_kodex::k2::validate_removed_edges(parent_root,root,symbols,edges)?;
    let raw=contract_value(text,"SurvivingCallerStructuralWitness").ok_or("SSC structural witness missing")?;
    let mut regions: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut all_regions=Vec::new();
    for region in raw.split(',').map(str::trim) {
        let (bound,node)=region.split_once('|').ok_or("SSC structural node must be path|kind::name::ordinal")?;
        all_regions.push(node.to_owned());
        regions.entry(bound.to_owned()).or_default().push(node.to_owned());
    }
    for path in paths {
        if !regions.contains_key(path) {
            return Err(format!("SSC retirement has no declared erasure nodes for {path}"));
        }
    }
    let ids=ids.split(',').map(|id|id.trim().to_owned()).collect::<BTreeSet<_>>();
    ssc_evidence_proof_shadow_permit_kodex::structural_erasure::validate_closed_contexts(parent_root,&all_regions,&ids)?;
    Ok(SscRetirementErasureContext { retired, regions })
}

fn verify_ssc_retirement_erasure_in_path(
    context: &SscRetirementErasureContext, path: &str, parent: &str, child: &str,
) -> Result<(), String> {
    let regions = context.regions.get(path)
        .ok_or_else(|| format!("SSC retirement has no declared erasure nodes for {path}"))?;
    ssc_evidence_proof_shadow_permit_kodex::structural_erasure::verify_in_path(
        parent, child, regions, &context.retired, Some(path),
    )
}

fn validate_corpus_independent_test_admission(
    source_root: &Path,
    current_root: &Path,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<(), String> {
    let mut violations = Vec::<String>::new();
    for change in actual.values() {
        if change.kind == ChangeKind::Deleted
            || !corpus_independence_admission_path(&change.path)
        {
            continue;
        }

        let incoming_path =
            source_root.join(change.path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let incoming = fs::read_to_string(&incoming_path).map_err(|error| {
            format!(
                "Take-Change-Control CORPUS-INDEPENDENT TEST ADMISSION: incoming text unreadable {}: {error}",
                change.path
            )
        })?;
        let current_path =
            current_root.join(change.path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let current = fs::read_to_string(&current_path).unwrap_or_default();

        for added_line in added_lines(&current, &incoming) {
            if let Some(reason) =
                corpus_independence_admission_line_violation(&change.path, added_line)
            {
                let evidence = added_line.trim().chars().take(180).collect::<String>();
                violations.push(format!(
                    "{} :: {} :: {}",
                    change.path, reason, evidence
                ));
            }
        }
    }

    if violations.is_empty() {
        return Ok(());
    }

    Err(format!(
        "Take-Change-Control CORPUS-INDEPENDENT TEST ADMISSION KODEX: neue normative Test-/Source-Evidenz darf keinen konkreten Etueden-/Takt-Witness, keine historische Regression als Testidentitaet und kein nichtsynthetisches Corpus-Fixture in die ausfuehrbare Spezifikation aufnehmen. Reale Fundstellen gehoeren in Evaluation/Diagnose; dauerhafte Tests muessen synthetische, relocatable Invarianten beweisen. Verstoesse:\n  - {}",
        violations.join("\n  - ")
    ))
}

fn corpus_independence_admission_path(path: &str) -> bool {
    // Only explicit gate-self-test owners may construct forbidden payloads.
    // A broad Kodex exemption would itself become a bypass around Corpus Independence.
    if matches!(
        path,
        "src/take_change_control.rs"
            | "src/identity_debt_inventory.rs"
            | "tests/global_identity_debt_inventory.rs"
    ) {
        return false;
    }

    if path.starts_with("tests/") {
        return true;
    }
    path.ends_with(".rs")
        && (path.starts_with("src/") || path.starts_with("tools/"))
}

fn corpus_independence_admission_line_violation(path: &str, line: &str) -> Option<&'static str> {
    if contains_concrete_etude_identity(line) {
        return Some("concrete_etude_identity");
    }
    if contains_non_synthetic_test_fixture(path, line) {
        return Some("non_synthetic_or_historical_fixture_dependency");
    }
    if contains_concrete_measure_identity(line) {
        return Some("concrete_measure_identity");
    }

    let lower = line.to_ascii_lowercase();
    if lower.contains("fn ") && lower.contains("regression") {
        return Some("historical_regression_test_symbol");
    }
    None
}

fn contains_concrete_etude_identity(line: &str) -> bool {
    let lower = line.to_lowercase();
    [
        "etüde nr.",
        "etude nr.",
        "etuede nr.",
        "etüde",
        "etude",
        "etuede",
    ]
    .iter()
    .any(|marker| ascii_digit_after_marker(&lower, marker))
}

fn contains_concrete_measure_identity(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    [
        "<measure number=",
        "target_measure",
        "target_takt",
        "measure_number",
        "takt_number",
        "find_measure_span_by_number",
        "measure_",
        "takt_",
        "measure ==",
        "measure==",
        "measure !=",
        "measure!=",
        "takt ==",
        "takt==",
    ]
    .iter()
    .any(|marker| ascii_digit_after_marker(&lower, marker))
}

fn contains_non_synthetic_test_fixture(path: &str, line: &str) -> bool {
    let normalized_path = path.replace('\\', "/").to_ascii_lowercase();
    let normalized_line = line.replace('\\', "/").to_ascii_lowercase();

    if normalized_path.starts_with("tests/fixtures/")
        && !normalized_path.starts_with("tests/fixtures/synthetic/")
    {
        return true;
    }

    if normalized_line.contains("tests/fixtures/")
        && !normalized_line.contains("tests/fixtures/synthetic/")
    {
        return true;
    }

    if normalized_line.contains("real_pre_operator_snapshot")
        || normalized_line.contains("real_snapshot")
        || normalized_line.contains("real-snapshot")
    {
        return true;
    }

    let loads_artifact = normalized_line.contains("include_str!(")
        || normalized_line.contains("include_bytes!(")
        || normalized_line.contains("read_to_string(");
    let corpus_artifact = [
        ".musicxml",
        ".render.musicxml",
        ".pdf",
        ".result.txt",
        ".wav",
        ".mid",
    ]
    .iter()
    .any(|suffix| normalized_line.contains(suffix));

    loads_artifact && corpus_artifact && !normalized_line.contains("synthetic")
}

fn ascii_digit_after_marker(text: &str, marker: &str) -> bool {
    let mut offset = 0usize;
    while let Some(relative) = text[offset..].find(marker) {
        let start = offset + relative + marker.len();
        let tail = &text[start..];
        let candidate = tail.trim_start_matches(|character: char| {
            character.is_ascii_whitespace()
                || matches!(character, '"' | '\'' | '=' | ':' | '_' | '.' | '\\')
        });
        if candidate
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_digit())
        {
            return true;
        }
        offset = start;
        if offset >= text.len() {
            break;
        }
    }
    false
}

const SELECTOR_DERIVATION_CONTRACT_PREFIX: &str = "docs/selector_derivation_contract_";
const SELECTOR_DERIVATION_CONTRACT_TEMPLATE: &str = "docs/selector_derivation_contract_template.md";
const SELECTOR_DERIVATION_CONTRACT_VERSION: &str = "causal_class_first_v1";

fn is_selector_derivation_contract_path(path: &str) -> bool {
    path.starts_with(SELECTOR_DERIVATION_CONTRACT_PREFIX)
        && path.ends_with(".md")
        && path != SELECTOR_DERIVATION_CONTRACT_TEMPLATE
}

fn validate_selector_derivation_governance(
    source_root: &Path,
    current_root: &Path,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<(), String> {
    let selector_sensitive_paths = selector_sensitive_changed_paths(source_root, current_root, actual)?;
    let contract_paths = actual
        .values()
        .filter(|change| change.kind != ChangeKind::Deleted)
        .map(|change| change.path.as_str())
        .filter(|path| is_selector_derivation_contract_path(path))
        .collect::<Vec<_>>();

    if selector_sensitive_paths.is_empty() {
        for contract_path in contract_paths {
            let contract_text = read_selector_derivation_contract(source_root, contract_path)?;
            validate_selector_error_class_definition_contract(&contract_text, contract_path)?;
        }
        return Ok(());
    }

    if contract_paths.len() != 1 {
        return Err(format!(
            "Take-Change-Control PROXY-IDENTITY SELECTOR KODEX: selector-sensitive new decision logic in {selector_sensitive_paths:?} requires exactly one changed {SELECTOR_DERIVATION_CONTRACT_PREFIX}*.md implementation contract; found {contract_paths:?}. Syntactic identity-freedom is insufficient, and the causal error-class contract must already exist in the accepted parent."
        ));
    }

    let contract_path = contract_paths[0];
    let contract_text = read_selector_derivation_contract(source_root, contract_path)?;
    validate_selector_implementation_contract(&contract_text, contract_path, current_root, actual)
}

fn read_selector_derivation_contract(root: &Path, path: &str) -> Result<String, String> {
    fs::read_to_string(root.join(path.replace('/', std::path::MAIN_SEPARATOR_STR))).map_err(|error| {
        format!(
            "Take-Change-Control PROXY-IDENTITY SELECTOR KODEX: selector derivation contract unreadable {path}: {error}"
        )
    })
}

fn selector_sensitive_changed_paths(
    source_root: &Path,
    current_root: &Path,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<Vec<String>, String> {
    let mut paths = Vec::new();
    for change in actual.values() {
        if change.kind == ChangeKind::Deleted
            || !change.path.ends_with(".rs")
            || matches!(path_role(&change.path), PathRole::Kodex | PathRole::Metadata)
        {
            continue;
        }
        let incoming_path =
            source_root.join(change.path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let incoming = fs::read_to_string(&incoming_path).map_err(|error| {
            format!(
                "Take-Change-Control PROXY-IDENTITY SELECTOR KODEX: incoming source unreadable {}: {error}",
                change.path
            )
        })?;
        let current_path =
            current_root.join(change.path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let current = fs::read_to_string(&current_path).unwrap_or_default();
        let added = added_lines(&current, &incoming);
        if added_lines_are_selector_sensitive(&added)
            || added_lines_are_multiplicity_sensitive(&added)
        {
            paths.push(change.path.clone());
        }
    }
    Ok(paths)
}

fn added_lines_are_selector_sensitive(lines: &[&str]) -> bool {
    let mut has_decision_control = false;
    let mut has_selector_semantics = false;
    for raw in lines {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        let lower = trimmed.to_ascii_lowercase();
        has_decision_control |= [
            "if ",
            "if(",
            "match ",
            ".filter(",
            ".filter_map(",
            ".retain(",
            ".find(",
            ".find_map(",
            ".is_some_and(",
            ".any(",
            ".all(",
            "&&",
            "||",
        ]
        .iter()
        .any(|token| lower.contains(token));
        has_selector_semantics |= [
            "candidate",
            "selector",
            "select_",
            "selected",
            "qualif",
            "eligib",
            "scope",
            "verdict",
            "decision",
            "repair",
            "target",
            "intersection",
            "predicate",
            "blocker",
        ]
        .iter()
        .any(|token| lower.contains(token));
    }
    has_decision_control && has_selector_semantics
}

fn added_lines_are_multiplicity_sensitive(lines: &[&str]) -> bool {
    let mut has_multiplicity_control = false;
    let mut has_candidate_semantics = false;
    for raw in lines {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        let lower = trimmed.to_ascii_lowercase();
        has_multiplicity_control |= [
            "len() == 1",
            "len()==1",
            "len() != 1",
            "len()!=1",
            ".take(1)",
            ".take( 1 )",
            ".into_iter().next()",
            "max_actions",
            "max_initial_rests",
            "unique_candidate",
            "unique_cross_staff_target",
        ]
        .iter()
        .any(|token| lower.contains(token));
        has_candidate_semantics |= [
            "candidate",
            "repair",
            "writer",
            "write_allowed",
            "materializ",
            "selector",
            "select_",
            "selected",
            "target",
            "scope",
        ]
        .iter()
        .any(|token| lower.contains(token));
    }
    has_multiplicity_control && has_candidate_semantics
}

fn selector_contract_value<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    text.lines().find_map(|raw| {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }
        let (name, value) = line.split_once(':')?;
        (name.trim() == key).then_some(value.trim())
    })
}

fn require_selector_contract_value(
    text: &str,
    path: &str,
    key: &str,
    expected: &str,
) -> Result<(), String> {
    let actual = selector_contract_value(text, key).unwrap_or("<missing>");
    if actual != expected {
        return Err(format!(
            "Take-Change-Control PROXY-IDENTITY SELECTOR KODEX: {path} requires {key}: {expected}, got {actual:?}"
        ));
    }
    Ok(())
}

fn validate_selector_contract_common(text: &str, path: &str) -> Result<(), String> {
    require_selector_contract_value(
        text,
        path,
        "SelectorDerivationContract",
        SELECTOR_DERIVATION_CONTRACT_VERSION,
    )?;
    for (key, expected) in [
        ("KnownWitnessUsedForFeatureDerivation", "no"),
        ("TargetConditionedFeatureNarrowing", "no"),
        ("ClassDefinitionPrecedesCorpusDistribution", "yes"),
        ("UniquenessIsOutcomeNotGoal", "yes"),
        ("RepairSelectorSeparatedFromErrorClass", "yes"),
        ("CausalFeatureNecessityReview", "complete"),
        ("CorpusDistributionMayRefineClassToKnownWitness", "no"),
    ] {
        require_selector_contract_value(text, path, key, expected)?;
    }
    if !text.lines().any(|line| line.trim_start().starts_with("CausalFeature:")) {
        return Err(format!(
            "Take-Change-Control PROXY-IDENTITY SELECTOR KODEX: {path} must name at least one CausalFeature independently justified by the defect class."
        ));
    }
    if !text
        .lines()
        .any(|line| line.trim_start().starts_with("NonCausalContextExcluded:"))
    {
        return Err(format!(
            "Take-Change-Control PROXY-IDENTITY SELECTOR KODEX: {path} must name at least one NonCausalContextExcluded so target-discriminating context is reviewed explicitly."
        ));
    }
    Ok(())
}

fn validate_selector_error_class_definition_contract(text: &str, path: &str) -> Result<(), String> {
    validate_selector_contract_common(text, path)?;
    require_selector_contract_value(text, path, "SelectorChange", "no")?;
    require_selector_contract_value(text, path, "SelectorRole", "error_class_definition")?;
    require_selector_contract_value(text, path, "ParentErrorClassContract", "none")?;
    require_selector_contract_value(
        text,
        path,
        "ErrorClassDefinitionIndependentOfKnownWitness",
        "yes",
    )?;
    require_selector_contract_value(text, path, "CorpusDistributionPurpose", "not_yet_measured")?;
    Ok(())
}

fn validate_multiplicity_invariance_contract(text: &str, path: &str) -> Result<(), String> {
    for (key, expected) in [
        ("MultiplicityInvarianceContract", "candidate_duplication_v1"),
        ("IndependentCandidateDuplicationInvariant", "yes"),
        ("ScorewideMultiplicityAsWriteAuthority", "forbidden"),
        ("GlobalRankingOfIndependentCandidates", "forbidden"),
        (
            "MultiplicityDecisionScope",
            "local_candidate_or_explicit_conflict_group",
        ),
        (
            "CandidateDuplicationProbe",
            "required_before_productive_write",
        ),
    ] {
        require_selector_contract_value(text, path, key, expected)?;
    }
    Ok(())
}

fn validate_selector_implementation_contract(
    text: &str,
    path: &str,
    current_root: &Path,
    actual: &BTreeMap<String, DeclaredChange>,
) -> Result<(), String> {
    validate_selector_contract_common(text, path)?;
    require_selector_contract_value(text, path, "SelectorChange", "yes")?;
    require_selector_contract_value(text, path, "CorpusDistributionPurpose", "measurement_only")?;
    validate_multiplicity_invariance_contract(text, path)?;

    let parent = selector_contract_value(text, "ParentErrorClassContract")
        .filter(|value| *value != "none" && !value.is_empty())
        .ok_or_else(|| {
            format!(
                "Take-Change-Control PROXY-IDENTITY SELECTOR KODEX: {path} requires a ParentErrorClassContract frozen in an already accepted parent before selector-sensitive code may be introduced."
            )
        })?;
    let parent = normalize_relative_path(parent)?;
    if !is_selector_derivation_contract_path(&parent) {
        return Err(format!(
            "Take-Change-Control PROXY-IDENTITY SELECTOR KODEX: {path} ParentErrorClassContract must reference a concrete {SELECTOR_DERIVATION_CONTRACT_PREFIX}*.md contract, got {parent:?}."
        ));
    }
    if actual.contains_key(&parent) {
        return Err(format!(
            "Take-Change-Control PROXY-IDENTITY SELECTOR KODEX: {path} may not create or modify its parent error-class contract in the same take; {parent} must pre-exist unchanged in the accepted parent."
        ));
    }
    let parent_text = read_selector_derivation_contract(current_root, &parent).map_err(|error| {
        format!(
            "Take-Change-Control PROXY-IDENTITY SELECTOR KODEX: {path} parent error-class contract is not readable from the installed parent {parent}: {error}"
        )
    })?;
    validate_selector_error_class_definition_contract(&parent_text, &parent)?;

    let role = selector_contract_value(text, "SelectorRole").unwrap_or("<missing>");
    match role {
        "error_classification" => {
            require_selector_contract_value(
                text,
                path,
                "ScopeNarrowingBasis",
                "causal_class_only",
            )?;
            require_selector_contract_value(
                text,
                path,
                "DiscriminativeContextOnlyNarrowing",
                "forbidden",
            )?;
        }
        "repair_scope" => {
            require_selector_contract_value(
                text,
                path,
                "ScopeNarrowingBasis",
                "authority_or_safety_only",
            )?;
            require_selector_contract_value(
                text,
                path,
                "DiscriminativeContextOnlyNarrowing",
                "forbidden",
            )?;
        }
        other => {
            return Err(format!(
                "Take-Change-Control PROXY-IDENTITY SELECTOR KODEX: {path} has unsupported SelectorRole {other:?}; expected error_classification or repair_scope. Error-class definitions must be frozen in a prior accepted parent without selector-sensitive code."
            ));
        }
    }
    Ok(())
}

fn strict_braced_use_reduction(current: &str, incoming: &str) -> bool {
    let current = current.trim();
    let incoming = incoming.trim();
    if current == incoming
        || !current.starts_with("use ")
        || !incoming.starts_with("use ")
        || !current.ends_with(';')
        || !incoming.ends_with(';')
    {
        return false;
    }
    let Some((current_prefix, current_tail)) = current.split_once('{') else {
        return false;
    };
    let Some((incoming_prefix, incoming_tail)) = incoming.split_once('{') else {
        return false;
    };
    let Some((current_body, current_suffix)) = current_tail.rsplit_once('}') else {
        return false;
    };
    let Some((incoming_body, incoming_suffix)) = incoming_tail.rsplit_once('}') else {
        return false;
    };
    if current_prefix.trim() != incoming_prefix.trim()
        || current_suffix.trim() != incoming_suffix.trim()
    {
        return false;
    }
    let current_items = current_body
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .collect::<BTreeSet<_>>();
    let incoming_items = incoming_body
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .collect::<BTreeSet<_>>();
    !incoming_items.is_empty()
        && incoming_items.len() < current_items.len()
        && incoming_items.is_subset(&current_items)
}

fn removed_lines<'a>(current: &'a str, incoming: &str) -> Vec<&'a str> {
    let mut incoming_counts = BTreeMap::<&str, usize>::new();
    for line in incoming.lines() {
        *incoming_counts.entry(line).or_insert(0) += 1;
    }
    let mut removed = Vec::new();
    for line in current.lines() {
        match incoming_counts.get_mut(line) {
            Some(count) if *count > 0 => *count -= 1,
            _ => removed.push(line),
        }
    }
    removed
}

fn added_lines<'a>(current: &'a str, incoming: &'a str) -> Vec<&'a str> {
    let mut current_counts = BTreeMap::<&str, usize>::new();
    for line in current.lines() {
        *current_counts.entry(line).or_insert(0) += 1;
    }
    let mut added = Vec::new();
    for line in incoming.lines() {
        match current_counts.get_mut(line) {
            Some(count) if *count > 0 => *count -= 1,
            _ => added.push(line),
        }
    }
    added
}

fn summarize(
    manifest: &ChangeManifest,
    actual: &BTreeMap<String, DeclaredChange>,
) -> ChangeValidationSummary {
    let mut kodex = 0usize;
    let mut audit = 0usize;
    let mut music = 0usize;
    let mut metadata = 0usize;
    for path in actual.keys() {
        match path_role(path) {
            PathRole::Kodex => kodex += 1,
            PathRole::Audit => audit += 1,
            PathRole::MusicSensitive => music += 1,
            PathRole::Metadata => metadata += 1,
            PathRole::OtherSource | PathRole::Unknown => {}
        }
    }
    ChangeValidationSummary {
        schema_version: manifest.schema_version,
        change_class: manifest.change_class,
        reason: manifest.reason.clone(),
        from_version: manifest.from_version.clone(),
        to_version: manifest.to_version.clone(),
        declared_change_count: manifest.changes.len(),
        actual_change_count: actual.len(),
        kodex_change_count: kodex,
        audit_change_count: audit,
        music_sensitive_change_count: music,
        metadata_change_count: metadata,
        semantic_added_count: 0,
        semantic_modified_count: 0,
        semantic_removed_count: 0,
        semantic_kodex_count: 0,
        semantic_audit_count: 0,
        semantic_music_count: 0,
        semantic_calibration_count: 0,
        semantic_writer_family_count: 0,
        semantic_unknown_count: 0,
        warning_debt_closure: manifest.warning_debt_closure,
        governed_changes: manifest.governed_changes.clone(),
        music_effect: manifest.music_effect.clone(),
    }
}

fn summarize_with_semantic(
    manifest: &ChangeManifest,
    actual: &BTreeMap<String, DeclaredChange>,
    semantic: &semantic_capability_diff::SemanticDiffSummary,
) -> ChangeValidationSummary {
    let mut summary = summarize(manifest, actual);
    apply_semantic_summary(&mut summary, semantic);
    summary
}

fn summarize_declared_with_semantic(
    manifest: &ChangeManifest,
    semantic: &semantic_capability_diff::SemanticDiffSummary,
) -> ChangeValidationSummary {
    summarize_with_semantic(manifest, &manifest.changes, semantic)
}

fn apply_semantic_summary(
    summary: &mut ChangeValidationSummary,
    semantic: &semantic_capability_diff::SemanticDiffSummary,
) {
    summary.semantic_added_count = semantic.added;
    summary.semantic_modified_count = semantic.modified;
    summary.semantic_removed_count = semantic.removed;
    summary.semantic_kodex_count = semantic.kodex;
    summary.semantic_audit_count = semantic.audit;
    summary.semantic_music_count = semantic.music;
    summary.semantic_calibration_count = semantic.calibration;
    summary.semantic_writer_family_count = semantic.writer_family;
    summary.semantic_unknown_count = semantic.unknown;
}

fn relative_file_set(root: &Path) -> Result<BTreeSet<PathBuf>, String> {
    let mut files = BTreeSet::new();
    collect_relative_files(root, root, &mut files)?;
    Ok(files)
}

fn collect_relative_files(
    root: &Path,
    dir: &Path,
    files: &mut BTreeSet<PathBuf>,
) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)
        .map_err(|error| format!("Ordner nicht lesbar {}: {error}", dir.display()))?
    {
        let entry = entry.map_err(|error| format!("Directory entry nicht lesbar: {error}"))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|error| format!("Dateityp nicht lesbar {}: {error}", path.display()))?;
        if file_type.is_dir() {
            collect_relative_files(root, &path, files)?;
        } else if file_type.is_file() {
            files.insert(
                path.strip_prefix(root)
                    .map_err(|error| {
                        format!("relativer Pfad fehlgeschlagen {}: {error}", path.display())
                    })?
                    .to_path_buf(),
            );
        }
    }
    Ok(())
}

fn normalize_relative_path(value: &str) -> Result<String, String> {
    let normalized = value.replace('\\', "/");
    let path = Path::new(&normalized);
    if path.is_absolute()
        || normalized.starts_with('/')
        || normalized
            .split('/')
            .any(|part| part == ".." || part.is_empty())
    {
        return Err(format!("ungueltiger relativer Pfad {value:?}"));
    }
    Ok(normalized)
}

fn path_to_slashes(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn file_fnv1a64(path: &Path) -> Result<u64, String> {
    let bytes = fs::read(path)
        .map_err(|error| format!("Datei fuer FNV64 nicht lesbar {}: {error}", path.display()))?;
    Ok(fnv1a64(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_root(label: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_take_change_control_{label}_{}_{}",
            std::process::id(),
            stamp
        ));
        for dir in MANAGED_DIRS {
            fs::create_dir_all(root.join(dir)).expect("mkdir");
        }
        fs::create_dir_all(root.join("src/build_gate")).expect("mkdir build gate");
        write(
            &root.join("Cargo.toml"),
            "[package]\nname = \"test\"\nversion = \"1.0.0\"\n",
        );
        write(&root.join("README.md"), "test\n");
        write(&root.join("ZIP_KIND.txt"), "test\n");
        write(&root.join("run.cmd"), "@echo off\n");
        write(&root.join("strict.cmd"), "@echo off\n");
        // Success fixtures model the current architecture-pin owner as part of the managed tree.
        // Individual pin-coordination tests may overwrite this with an explicit include_bytes! graph.
        write(
            &root.join("src/identity_gate_trust_anchor.rs"),
            "const TAKE_CHANGE_CONTROL_BYTES: &[u8] = include_bytes!(\"take_change_control.rs\");\n",
        );
        root
    }

    fn write(path: &Path, text: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent");
        }
        fs::write(path, text).expect("write");
    }

    fn manifest_for(class: &str, path: &str, kind: &str, hash: Option<u64>) -> String {
        format!(
            "schema_version=1\nchange_class={class}\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nchange={kind}|{path}|{}\n",
            hash.map(|value| format!("fnv64:{value:016x}"))
                .unwrap_or_else(|| "-".to_string())
        )
    }

    #[test]
    fn kodex_only_is_a_transition_alias_for_current_kodex_only_identity() {
        let manifest = parse_manifest(
            "schema_version=1\nchange_class=KodexOnly\nfrom_version=1.0.0\nto_version=1.0.0\nreason=steady_state\nchange=modified|src/take_change_control.rs|fnv64:0000000000000001\n",
        )
        .expect("steady-state KodexOnly manifest");
        assert_eq!(manifest.change_class, ChangeClass::KodexOnly);
        assert!(ChangeClass::parse("LegacyOnly").is_err());
    }

    #[test]
    fn kodex_namespace_transition_installed_path_policy_defers_to_exact_tree_proof() {
        let retired_marker = [
            "kodex_namespace_",
            "rename=closed_architecture_namespace_cutover_v1",
        ]
        .concat();
        let manifest = format!(
            "schema_version=1\nchange_class=KodexOnly\nfrom_version=1.0.0\nto_version=1.0.0\nreason=retired_transition\n{retired_marker}\nchange=modified|src/take_change_control.rs|fnv64:0000000000000000\n"
        );
        assert!(parse_manifest(&manifest).is_err());
    }

    #[test]
    fn kodex_only_accepts_exact_kodex_diff() {
        let current = temp_root("kodex_current");
        let incoming = temp_root("kodex_incoming");
        write(
            &current.join("Cargo.toml"),
            "[package]\nversion = \"1.0.0\"\n",
        );
        write(
            &incoming.join("Cargo.toml"),
            "[package]\nversion = \"1.0.1\"\n",
        );
        write(
            &current.join("src/identity_debt_inventory.rs"),
            "// old kodex fixture\n",
        );
        write(
            &incoming.join("src/identity_debt_inventory.rs"),
            "// new kodex fixture\n",
        );
        let cargo_hash = file_fnv1a64(&incoming.join("Cargo.toml")).expect("hash");
        let source_hash =
            file_fnv1a64(&incoming.join("src/identity_debt_inventory.rs")).expect("hash");
        let manifest = format!(
            "schema_version=1\nchange_class=KodexOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nchange=modified|Cargo.toml|fnv64:{cargo_hash:016x}\nchange=modified|src/identity_debt_inventory.rs|fnv64:{source_hash:016x}\n"
        );
        write(&incoming.join(CHANGE_MANIFEST_RELATIVE_PATH), &manifest);
        let result = validate_incoming_take(&incoming, &current, "1.0.0", "1.0.1");
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn manifest_accepts_closed_kodex_writer_relocation_marker() {
        let text = "schema_version=1\nchange_class=KodexOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nkodex_writer_relocation=closed_internal_graph_v1\nchange=modified|Cargo.toml|fnv64:0000000000000001\n";
        let manifest = parse_manifest(text).expect("manifest");
        assert!(manifest.kodex_writer_relocation);
    }

    #[test]
    fn manifest_rejects_unknown_kodex_writer_relocation_value() {
        let text = "schema_version=1\nchange_class=KodexOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nkodex_writer_relocation=loose\nchange=modified|Cargo.toml|fnv64:0000000000000001\n";
        assert!(parse_manifest(text).is_err());
    }

    #[test]
    fn manifest_accepts_closed_kodex_negative_check_relocation_marker() {
        let text = "schema_version=1\nchange_class=KodexOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nkodex_negative_check_relocation=closed_exact_negative_check_relocation_v1\nchange=modified|Cargo.toml|fnv64:0000000000000001\n";
        let manifest = parse_manifest(text).expect("manifest");
        assert!(manifest.kodex_negative_check_relocation);
    }

    #[test]
    fn manifest_rejects_unknown_kodex_negative_check_relocation_value() {
        let text = "schema_version=1\nchange_class=KodexOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nkodex_negative_check_relocation=loose\nchange=modified|Cargo.toml|fnv64:0000000000000001\n";
        assert!(parse_manifest(text).is_err());
    }

    #[test]
    fn manifest_accepts_closed_kodex_negative_check_module_relocation_marker() {
        let text = "schema_version=1\nchange_class=KodexOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nkodex_negative_check_module_relocation=closed_private_to_parent_module_relocation_v1\nchange=modified|Cargo.toml|fnv64:0000000000000001\n";
        let manifest = parse_manifest(text).expect("manifest");
        assert!(manifest.kodex_negative_check_module_relocation);
    }

    #[test]
    fn manifest_rejects_unknown_kodex_negative_check_module_relocation_value() {
        let text = "schema_version=1\nchange_class=KodexOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nkodex_negative_check_module_relocation=loose\nchange=modified|Cargo.toml|fnv64:0000000000000001\n";
        assert!(parse_manifest(text).is_err());
    }

    #[test]
fn manifest_accepts_only_closed_warning_debt_markers() {
        let closure = "schema_version=1\nchange_class=RefactorOnly\nfrom_version=1.0.0\nto_version=1.0.0\nreason=test\nwarning_debt_closure=closed_current_warning_debt_v1\nchange=modified|src/a.rs|fnv64:0000000000000001\n";
        let manifest = parse_manifest(closure).expect("warning debt closure marker");
        assert!(manifest.warning_debt_closure);
        assert!(parse_manifest(&closure.replace("closed_current_warning_debt_v1", "loose")).is_err());
        assert!(parse_manifest(&closure.replace(
            "warning_debt_closure=closed_current_warning_debt_v1",
            "rust_warning_debt_contract=closed_quarantined_warning_debt_v1",
        ))
        .is_err());
    }

    #[test]
    fn manifest_rejects_unknown_kodex_build_owner_rename_value() {
        let text = "schema_version=1\nchange_class=KodexOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nkodex_build_owner_rename=loose\nchange=modified|Cargo.toml|fnv64:0000000000000001\n";
        assert!(parse_manifest(text).is_err());
    }

    #[test]
    fn manifest_rejects_unknown_rust_edition_transition_value() {
        let text = "schema_version=1\nchange_class=KodexOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nrust_edition_transition=loose\nchange=modified|Cargo.toml|fnv64:0000000000000001\n";
        assert!(parse_manifest(text).is_err());
    }

    #[test]
    fn retired_one_shot_transition_markers_are_rejected() {
        for marker in [
            "kodex_build_owner_rename=closed_exact_cargo_build_owner_rename_v1",
            "rust_edition_transition=closed_2021_to_2024_v1",
            "rust_2024_resolver_contract=closed_resolver3_msrv_1_88_v1",
            "rust_dead_code_lint_contract=closed_root_package_deny_v1",
            "rust_warning_lint_contract=closed_root_package_warnings_deny_v1",
            "rust_warning_debt_contract=closed_quarantined_warning_debt_v1",
            "watch_rust_2024_resolver_contract=closed_watch_2021_to_2024_resolver3_msrv_1_88_v1",
            "schemaschlopf_r020_dependency_admission=closed_exact_local_path_0_1_0_r020_warning_deny_v2",
            "musicxml40_product_contract_cutover=closed_exact_r020_to_r021_persisted_musicxml40_v1",
        ] {
            let text = format!(
                "schema_version=1\nchange_class=KodexOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\n{marker}\nchange=modified|Cargo.toml|fnv64:0000000000000001\n"
            );
            assert!(parse_manifest(&text).is_err(), "retired marker accepted: {marker}");
        }
    }

    #[test]
    fn cargo_manifest_steady_state_allows_only_package_version_change() {
        let current = temp_root("cargo_steady_current");
        let incoming = temp_root("cargo_steady_incoming");
        write(
            &current.join("Cargo.toml"),
            "[package]\nname = \"x\"\nversion = \"1.0.0\"\nedition = \"2024\"\nrust-version = \"1.88\"\nresolver = \"3\"\n",
        );
        write(
            &incoming.join("Cargo.toml"),
            "[package]\nname = \"x\"\nversion = \"1.0.1\"\nedition = \"2024\"\nrust-version = \"1.88\"\nresolver = \"3\"\n",
        );
        assert_eq!(
            cargo_manifest_change_kind(&current.join("Cargo.toml"), &incoming.join("Cargo.toml"))
                .expect("version-only kind"),
            CargoManifestChangeKind::VersionOnly
        );
        write(
            &incoming.join("Cargo.toml"),
            "[package]\nname = \"x\"\nversion = \"1.0.1\"\nedition = \"2024\"\nrust-version = \"1.89\"\nresolver = \"3\"\n",
        );
        assert_eq!(
            cargo_manifest_change_kind(&current.join("Cargo.toml"), &incoming.join("Cargo.toml"))
                .expect("non-version kind"),
            CargoManifestChangeKind::Other
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn watch_cargo_steady_state_rejects_manifest_and_lock_changes() {
        let change = |path: &str| DeclaredChange {
            kind: ChangeKind::Modified,
            path: path.to_string(),
            incoming_fnv1a64: Some(1),
        };
        let manifest_change = BTreeMap::from([(
            WATCH_PROJECT_MANIFEST_RELATIVE_PATH.to_string(),
            change(WATCH_PROJECT_MANIFEST_RELATIVE_PATH),
        )]);
        assert!(validate_watch_cargo_manifest_change_contract(&manifest_change).is_err());
        let lock_change = BTreeMap::from([(
            WATCH_PROJECT_LOCK_RELATIVE_PATH.to_string(),
            change(WATCH_PROJECT_LOCK_RELATIVE_PATH),
        )]);
        assert!(validate_watch_cargo_manifest_change_contract(&lock_change).is_err());
    }

    #[test]
    fn kodex_only_rejects_music_sensitive_path() {
        let text = manifest_for(
            "KodexOnly",
            "src/structural_write_authority/example.rs",
            "added",
            Some(1),
        );
        let manifest = parse_manifest(&text).expect("manifest");
        assert!(validate_manifest_path_policy(&manifest).is_err());
    }

    #[test]
    fn launcher_is_an_exact_kodex_control_plane_path() {
        let text = manifest_for(
            "KodexOnly",
            "src/bin/etuschlopf_launcher.rs",
            "modified",
            Some(1),
        );
        let manifest = parse_manifest(&text).expect("manifest");
        assert!(validate_manifest_path_policy(&manifest).is_ok());
        assert_eq!(path_role("src/bin/etuschlopf_launcher.rs"), PathRole::Kodex);
        assert_eq!(
            path_role("src/bin/etuschlopf_launcher/assistant_export.rs"),
            PathRole::Kodex
        );
        assert_eq!(path_role("src/visual_preflight.rs"), PathRole::Kodex);
        assert_eq!(path_role("src/visual_preflight/no_seed.rs"), PathRole::Kodex);
        assert_ne!(path_role("src/bin/other_launcher.rs"), PathRole::Kodex);
    }

    #[test]
    fn music_mutation_cannot_claim_launcher_control_plane_changes() {
        let text = manifest_for(
            "MusicMutation",
            "src/bin/etuschlopf_launcher.rs",
            "modified",
            Some(1),
        );
        let manifest = parse_manifest(&text).expect("manifest");
        assert!(validate_manifest_path_policy(&manifest).is_err());
    }

    #[test]
    fn public_artifact_recovery_is_an_exact_kodex_control_plane_path() {
        let text = manifest_for(
            "KodexOnly",
            "src/kodex_control_plane/public_artifact_recovery.rs",
            "added",
            Some(1),
        );
        let manifest = parse_manifest(&text).expect("manifest");
        assert!(validate_manifest_path_policy(&manifest).is_ok());
        assert_eq!(
            path_role("src/kodex_control_plane/public_artifact_recovery.rs"),
            PathRole::Kodex
        );
        assert_ne!(
            path_role("src/kodex_control_plane/public_artifact_recovery_shadow.rs"),
            PathRole::Kodex
        );
        assert_ne!(
            path_role("src/kodex_control_plane/other.rs"),
            PathRole::Kodex
        );
    }

    #[test]
    fn music_mutation_cannot_claim_public_artifact_recovery_control_plane() {
        let text = manifest_for(
            "MusicMutation",
            "src/kodex_control_plane/public_artifact_recovery.rs",
            "modified",
            Some(1),
        );
        let manifest = parse_manifest(&text).expect("manifest");
        assert!(validate_manifest_path_policy(&manifest).is_err());
    }

    #[test]
    fn validation_only_class_roundtrips() {
        let class = ChangeClass::parse("ValidationOnly").expect("class");
        assert_eq!(class, ChangeClass::ValidationOnly);
        assert_eq!(class.as_str(), "ValidationOnly");
    }

    #[test]
    fn steady_state_validation_only_cannot_mutate_qualified_component_bytes() {
        let text = manifest_for(
            "ValidationOnly",
            "tools/qualified_components/schemaschlopf_r021/src/lib.rs",
            "modified",
            Some(1),
        );
        let manifest = parse_manifest(&text).expect("manifest");
        assert!(
            validate_validation_only_contract(&manifest, &manifest.changes).is_err()
        );
    }

    #[test]
    fn validation_only_audit_role_stays_closed_without_exact_musicxml40_marker() {
        let text = manifest_for(
            "ValidationOnly",
            "src/audit_only/format40_conformance_foundation.rs",
            "modified",
            Some(1),
        );
        let manifest = parse_manifest(&text).expect("manifest");
        assert!(validate_manifest_path_policy(&manifest).is_ok());
        assert!(
            validate_validation_only_contract(&manifest, &manifest.changes).is_err()
        );
    }

    #[test]
    fn outcome_projection_parses_and_has_exact_name() {
        let class = ChangeClass::parse("OutcomeProjection").expect("class");
        assert_eq!(class, ChangeClass::OutcomeProjection);
        assert_eq!(class.as_str(), "OutcomeProjection");
    }

    #[test]
    fn audit_only_rejects_writer_path() {
        let text = manifest_for(
            "AuditOnly",
            "src/pipeline_shards/new_writer_core.rs",
            "added",
            Some(1),
        );
        let manifest = parse_manifest(&text).expect("manifest");
        assert!(validate_manifest_path_policy(&manifest).is_err());
    }

    #[test]
    fn dedicated_audit_owner_precedes_music_sensitive_subject_markers() {
        for path in [
            "src/audit_only/final_notation_accidental_state_clearance_geometry.rs",
            "src/audits/musicxml_readback.rs",
            "src/proof_artifacts/writer_proof.rs",
        ] {
            assert_eq!(path_role(path), PathRole::Audit, "{path}");
        }
    }

    #[test]
    fn final_report_projection_owners_are_explicit_audit_paths() {
        for path in [
            "src/final_report.rs",
            "src/pipeline_shards/final_report_projection_owner.rs",
        ] {
            assert_eq!(path_role(path), PathRole::Audit, "{path}");
        }
    }

    #[test]
    fn audit_only_accepts_added_dedicated_audit_with_music_subject_name() {
        let text = manifest_for(
            "AuditOnly",
            "src/audit_only/final_notation_accidental_state_clearance_geometry.rs",
            "added",
            Some(1),
        );
        let manifest = parse_manifest(&text).expect("manifest");
        assert!(validate_manifest_path_policy(&manifest).is_ok());
    }

    #[test]
    fn music_sensitive_subject_marker_still_applies_outside_dedicated_audit_owner() {
        assert_eq!(
            path_role("src/final_notation_accidental_state_clearance_geometry.rs"),
            PathRole::MusicSensitive
        );
        assert_eq!(
            path_role("src/pipeline_shards/musicxml_probe.rs"),
            PathRole::MusicSensitive
        );
    }

    #[test]
    fn music_mutation_rejects_kodex_path() {
        let text = manifest_for(
            "MusicMutation",
            "src/identity_debt_build.rs",
            "modified",
            Some(1),
        );
        let manifest = parse_manifest(&text).expect("manifest");
        assert!(validate_manifest_path_policy(&manifest).is_err());
    }

    #[test]
    fn calibration_only_accepts_music_threshold_path_policy() {
        let text = manifest_for(
            "CalibrationOnly",
            "src/pipeline_shards/tonal_threshold_core.rs",
            "modified",
            Some(1),
        );
        let manifest = parse_manifest(&text).expect("manifest");
        assert!(validate_manifest_path_policy(&manifest).is_ok());
    }

    #[test]
    fn music_mutation_rejects_audit_path() {
        let text = manifest_for("MusicMutation", "src/audits/proof.rs", "modified", Some(1));
        let manifest = parse_manifest(&text).expect("manifest");
        assert!(validate_manifest_path_policy(&manifest).is_err());
    }

    #[test]
    fn unknown_root_path_is_fail_closed() {
        let current = temp_root("unknown_root_current");
        let incoming = temp_root("unknown_root_incoming");
        write(&incoming.join("unexpected.txt"), "unknown\n");
        let result = validate_complete_incoming_tree(&incoming, &current);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn mechanical_only_rejects_new_source_file() {
        let current = temp_root("mechanical_current");
        let incoming = temp_root("mechanical_incoming");
        write(
            &current.join("Cargo.toml"),
            "[package]\nversion = \"1.0.0\"\n",
        );
        write(
            &incoming.join("Cargo.toml"),
            "[package]\nversion = \"1.0.1\"\n",
        );
        write(&incoming.join("src/helper.rs"), "fn helper() {}\n");
        let cargo_hash = file_fnv1a64(&incoming.join("Cargo.toml")).expect("hash");
        let helper_hash = file_fnv1a64(&incoming.join("src/helper.rs")).expect("hash");
        let manifest = format!(
            "schema_version=1\nchange_class=MechanicalOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nchange=modified|Cargo.toml|fnv64:{cargo_hash:016x}\nchange=added|src/helper.rs|fnv64:{helper_hash:016x}\n"
        );
        write(&incoming.join(CHANGE_MANIFEST_RELATIVE_PATH), &manifest);
        let result = validate_incoming_take(&incoming, &current, "1.0.0", "1.0.1");
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn audit_only_accepts_dedicated_identity_free_proof_file() {
        let current = temp_root("audit_current");
        let incoming = temp_root("audit_incoming");
        write(
            &current.join("Cargo.toml"),
            "[package]\nversion = \"1.0.0\"\n",
        );
        write(
            &incoming.join("Cargo.toml"),
            "[package]\nversion = \"1.0.1\"\n",
        );
        write(
            &incoming.join("src/proof_artifacts/typed_proof.rs"),
            "pub fn proof_green() -> bool { true }\n",
        );
        let cargo_hash = file_fnv1a64(&incoming.join("Cargo.toml")).expect("hash");
        let proof_hash =
            file_fnv1a64(&incoming.join("src/proof_artifacts/typed_proof.rs")).expect("hash");
        let manifest = format!(
            "schema_version=1\nchange_class=AuditOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nchange=modified|Cargo.toml|fnv64:{cargo_hash:016x}\nchange=added|src/proof_artifacts/typed_proof.rs|fnv64:{proof_hash:016x}\n"
        );
        write(&incoming.join(CHANGE_MANIFEST_RELATIVE_PATH), &manifest);
        let result = validate_incoming_take(&incoming, &current, "1.0.0", "1.0.1");
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn audit_only_rejects_added_mutation_token_even_in_audit_root() {
        let current = temp_root("audit_mutation_current");
        let incoming = temp_root("audit_mutation_incoming");
        write(
            &current.join("Cargo.toml"),
            "[package]\nversion = \"1.0.0\"\n",
        );
        write(
            &incoming.join("Cargo.toml"),
            "[package]\nversion = \"1.0.1\"\n",
        );
        write(
            &incoming.join("src/audit_only/bad.rs"),
            "pub fn bad() { rewrite_score(); }\n",
        );
        let cargo_hash = file_fnv1a64(&incoming.join("Cargo.toml")).expect("hash");
        let bad_hash = file_fnv1a64(&incoming.join("src/audit_only/bad.rs")).expect("hash");
        let manifest = format!(
            "schema_version=1\nchange_class=AuditOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nchange=modified|Cargo.toml|fnv64:{cargo_hash:016x}\nchange=added|src/audit_only/bad.rs|fnv64:{bad_hash:016x}\n"
        );
        write(&incoming.join(CHANGE_MANIFEST_RELATIVE_PATH), &manifest);
        let result = validate_incoming_take(&incoming, &current, "1.0.0", "1.0.1");
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn dedicated_music_subject_audit_still_rejects_added_mutation_token() {
        let current = temp_root("audit_music_subject_mutation_current");
        let incoming = temp_root("audit_music_subject_mutation_incoming");
        write(
            &current.join("Cargo.toml"),
            "[package]\nversion = \"1.0.0\"\n",
        );
        write(
            &incoming.join("Cargo.toml"),
            "[package]\nversion = \"1.0.1\"\n",
        );
        let audit_path =
            "src/audit_only/final_notation_accidental_state_clearance_geometry.rs";
        write(
            &incoming.join(audit_path),
            "pub fn bad(xml: &mut String) { xml.replace_range(0..0, \"\"); }\n",
        );
        let cargo_hash = file_fnv1a64(&incoming.join("Cargo.toml")).expect("hash");
        let audit_hash = file_fnv1a64(&incoming.join(audit_path)).expect("hash");
        let manifest = format!(
            "schema_version=1\nchange_class=AuditOnly\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nchange=modified|Cargo.toml|fnv64:{cargo_hash:016x}\nchange=added|{audit_path}|fnv64:{audit_hash:016x}\n"
        );
        write(&incoming.join(CHANGE_MANIFEST_RELATIVE_PATH), &manifest);
        let result = validate_incoming_take(&incoming, &current, "1.0.0", "1.0.1");
        assert!(result.is_err());
        let error = result.expect_err("mutation token must remain blocked");
        assert!(error.contains("verbotenen Mutations-/Writer-Token"), "{error}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    // Synthetic Cargo-only transitions exercise both independent admission layers.
    fn assert_windows_job_transition(
        current_text: &str,
        incoming_text: &str,
        class: &str,
        admitted: bool,
    ) {
        let current = temp_root("windows_job_current");
        let incoming = temp_root("windows_job_incoming");
        write(&current.join("Cargo.toml"), current_text);
        write(&incoming.join("Cargo.toml"), incoming_text);
        let hash = file_fnv1a64(&incoming.join("Cargo.toml")).expect("hash");
        let manifest = manifest_for(class, "Cargo.toml", "modified", Some(hash))
            .replace("to_version=1.0.1", "to_version=1.0.0");
        write(&incoming.join(CHANGE_MANIFEST_RELATIVE_PATH), &manifest);
        let preflight =
            validate_incoming_change_dependencies(&incoming, &current, "1.0.0", "1.0.0");
        let full = validate_incoming_take(&incoming, &current, "1.0.0", "1.0.0");
        let semantic = semantic_capability_diff::validate_semantic_change(
            &current,
            &incoming,
            class,
            &BTreeSet::from(["Cargo.toml".to_string()]),
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
        assert_eq!(
            preflight.is_ok(),
            admitted,
            "preflight {class}: {preflight:?}"
        );
        assert_eq!(full.is_ok(), admitted, "full {class}: {full:?}");
        assert_eq!(semantic.is_ok(), admitted, "semantic {class}: {semantic:?}");
    }

    #[test]
    fn windows_job_dependency_exact_addition_passes_both_admission_layers() {
        let current = "[package]\nname = \"test\"\nversion = \"1.0.0\"\n[dependencies]\nfoo = \"1\"\n";
        let incoming = format!("{current}{WINDOWS_JOB_DEPENDENCY_V1}");
        assert_windows_job_transition(current, &incoming, "KodexOnly", true);
        // Only line-ending representation is immaterial, as in version-only admission.
        assert_windows_job_transition(
            &current.replace('\n', "\r\n"),
            &incoming.replace('\n', "\r\n"),
            "KodexOnly",
            true,
        );
    }

    #[test]
    fn windows_job_dependency_variations_fail_both_admission_layers() {
        let current = "[package]\nname = \"test\"\nversion = \"1.0.0\"\n[dependencies]\nfoo = \"1\"\n";
        let exact = format!("{current}{WINDOWS_JOB_DEPENDENCY_V1}");
        let variations = [
            exact.replace("=0.61.2", "0.61.2"),
            exact.replace("=0.61.2", "=0.61.1"),
            exact.replace("default-features = false", "default-features = true"),
            exact.replace("default-features = false, ", ""),
            exact.replace("[target.'cfg(windows)'.dependencies]", "[dependencies]"),
            exact.replace("cfg(windows)", "cfg(unix)"),
            exact.replace(
                "Win32_System_Threading\"]",
                "Win32_System_Threading\", \"Win32_System_Console\"]",
            ),
            exact.replace("\"Win32_System_SystemServices\", ", ""),
            exact.replace("windows-sys =", "windows ="),
            exact.replace("foo = \"1\"", "foo = \"2\""),
            exact.replace("Win32_Foundation\", \"Win32_Security", "Win32_Security\", \"Win32_Foundation"),
            format!("{exact}other = \"1\"\n"),
            format!("{exact}[features]\nextra = []\n"),
            exact.replace("name = \"test\"", "name = \"changed\""),
            exact.replace("version = \"1.0.0\"", "version = \"1.0.1\""),
            format!("{exact}{WINDOWS_JOB_DEPENDENCY_V1}"),
        ];
        for incoming in variations {
            assert_windows_job_transition(current, &incoming, "KodexOnly", false);
        }
        for class in [
            "RefactorOnly",
            "ValidationOnly",
            "GovernedChange",
            "MusicMutation",
        ] {
            assert_windows_job_transition(current, &exact, class, false);
        }
    }

    #[test]
    fn windows_job_dependency_cannot_be_removed_changed_or_added_twice() {
        let base = "[package]\nname = \"test\"\nversion = \"1.0.0\"\n";
        let installed = format!("{base}{WINDOWS_JOB_DEPENDENCY_V1}");
        let changed = installed.replace("=0.61.2", "=0.61.3");
        let duplicated = format!("{installed}{WINDOWS_JOB_DEPENDENCY_V1}");
        for incoming in [base, changed.as_str(), duplicated.as_str()] {
            assert_windows_job_transition(&installed, incoming, "KodexOnly", false);
        }
        assert!(!windows_job_dependency_transition_v1(
            &installed, &installed
        ));
        // Ordinary future version bumps stay legal, with the dependency byte-identical.
        let current = temp_root("windows_job_installed");
        let incoming = temp_root("windows_job_version");
        write(&current.join("Cargo.toml"), &installed);
        write(
            &incoming.join("Cargo.toml"),
            &installed.replace("version = \"1.0.0\"", "version = \"1.0.1\""),
        );
        assert_eq!(
            cargo_manifest_change_kind(&current.join("Cargo.toml"), &incoming.join("Cargo.toml"))
                .expect("version-only"),
            CargoManifestChangeKind::VersionOnly
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn non_music_class_rejects_dependency_version_change() {
        let current = temp_root("cargo_current");
        let incoming = temp_root("cargo_incoming");
        write(
            &current.join("Cargo.toml"),
            "[package]\nversion = \"1.0.0\"\n[dependencies.foo]\nversion = \"1\"\n",
        );
        write(
            &incoming.join("Cargo.toml"),
            "[package]\nversion = \"1.0.1\"\n[dependencies.foo]\nversion = \"2\"\n",
        );
        let cargo_hash = file_fnv1a64(&incoming.join("Cargo.toml")).expect("hash");
        write(
            &incoming.join(CHANGE_MANIFEST_RELATIVE_PATH),
            &manifest_for("KodexOnly", "Cargo.toml", "modified", Some(cargo_hash)),
        );
        let result = validate_incoming_take(&incoming, &current, "1.0.0", "1.0.1");
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn undeclared_changed_path_is_rejected() {
        let current = temp_root("undeclared_current");
        let incoming = temp_root("undeclared_incoming");
        write(
            &current.join("Cargo.toml"),
            "[package]\nversion = \"1.0.0\"\n",
        );
        write(
            &incoming.join("Cargo.toml"),
            "[package]\nversion = \"1.0.1\"\n",
        );
        write(&incoming.join("README.md"), "undeclared\n");
        let cargo_hash = file_fnv1a64(&incoming.join("Cargo.toml")).expect("hash");
        write(
            &incoming.join(CHANGE_MANIFEST_RELATIVE_PATH),
            &manifest_for("KodexOnly", "Cargo.toml", "modified", Some(cargo_hash)),
        );
        let result = validate_incoming_take(&incoming, &current, "1.0.0", "1.0.1");
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn declared_hash_must_match_actual_incoming_file() {
        let current = temp_root("hash_current");
        let incoming = temp_root("hash_incoming");
        write(
            &current.join("Cargo.toml"),
            "[package]\nversion = \"1.0.0\"\n",
        );
        write(
            &incoming.join("Cargo.toml"),
            "[package]\nversion = \"1.0.1\"\n",
        );
        write(
            &incoming.join(CHANGE_MANIFEST_RELATIVE_PATH),
            &manifest_for("KodexOnly", "Cargo.toml", "modified", Some(7)),
        );
        let result = validate_incoming_take(&incoming, &current, "1.0.0", "1.0.1");
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn complete_tree_rejects_missing_managed_directory() {
        let current = temp_root("missing_dir_current");
        let incoming = temp_root("missing_dir_incoming");
        fs::remove_dir_all(incoming.join("tools")).expect("remove tools");
        let result = validate_complete_incoming_tree(&incoming, &current);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn complete_tree_rejects_missing_existing_root_file() {
        let current = temp_root("missing_root_current");
        let incoming = temp_root("missing_root_incoming");
        fs::remove_file(incoming.join("strict.cmd")).expect("remove strict");
        let result = validate_complete_incoming_tree(&incoming, &current);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn complete_tree_rejects_case_insensitive_collision() {
        let result = validate_case_insensitive_path_set([
            "docs/Case.md".to_string(),
            "docs/case.md".to_string(),
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn installed_tree_verifier_detects_extra_file() {
        let source = temp_root("verify_source");
        let installed = temp_root("verify_installed");
        write(&installed.join("src/unexpected.rs"), "fn unexpected() {}\n");
        let result = verify_installed_tree_matches_source(&source, &installed);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(source);
        let _ = fs::remove_dir_all(installed);
    }

    #[test]
    fn installed_tree_verifier_accepts_exact_tree() {
        let source = temp_root("verify_exact_source");
        let installed = temp_root("verify_exact_installed");
        let result = verify_installed_tree_matches_source(&source, &installed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(source);
        let _ = fs::remove_dir_all(installed);
    }

    #[test]
    fn audit_cross_role_accepts_strict_braced_use_reduction() {
        assert!(strict_braced_use_reduction(
            "use notation::{alpha, audit_probe, omega};",
            "use notation::{alpha, omega};",
        ));
    }

    #[test]
    fn audit_cross_role_rejects_import_path_or_symbol_addition() {
        assert!(!strict_braced_use_reduction(
            "use notation::{alpha, omega};",
            "use render::{alpha};",
        ));
        assert!(!strict_braced_use_reduction(
            "use notation::{alpha, omega};",
            "use notation::{alpha, beta, omega};",
        ));
    }

    // KodexNegativeCheck identity intentionally retains its historical symbol.
    // The regression now proves that the former RefactorOnly admission is closed.
    #[test]
    fn watch_project_manifest_and_lock_are_refactor_only_other_source_paths() {
        assert_eq!(path_role(WATCH_PROJECT_MANIFEST_RELATIVE_PATH), PathRole::Kodex);
        assert_eq!(path_role(WATCH_PROJECT_LOCK_RELATIVE_PATH), PathRole::Kodex);
        let changes = [
            WATCH_PROJECT_MANIFEST_RELATIVE_PATH,
            WATCH_PROJECT_LOCK_RELATIVE_PATH,
        ]
        .into_iter()
        .map(|path| {
            (
                path.to_string(),
                DeclaredChange {
                    kind: ChangeKind::Modified,
                    path: path.to_string(),
                    incoming_fnv1a64: Some(1),
                },
            )
        })
        .collect();
        let mut manifest = ChangeManifest {
            schema_version: 1,
            change_class: ChangeClass::RefactorOnly,
            from_version: "1.0.0".to_string(),
            to_version: "1.0.1".to_string(),
            reason: "watch cargo kodex boundary".to_string(),
            kodex_writer_relocation: false,
            kodex_negative_check_relocation: false,
            kodex_negative_check_module_relocation: false,
            warning_debt_closure: false,
            authority_retirement_contract: None,
            governed_changes: Vec::new(),
            music_effect: None,
            changes,
        };
        assert!(validate_manifest_path_policy(&manifest).is_err());
        manifest.change_class = ChangeClass::KodexOnly;
        assert!(validate_manifest_path_policy(&manifest).is_ok());
    }

    #[test]
fn kodex_namespace_reexec_accepts_only_exact_legacy_to_kodex_class_identity() {
        assert_eq!(
            ChangeClass::parse("KodexOnly").expect("steady-state KodexOnly"),
            ChangeClass::KodexOnly
        );
        assert!(ChangeClass::parse("LegacyOnly").is_err());
    }

    #[test]
    fn architecture_pin_coordination_requires_the_actual_trust_anchor_owner() {
        let current = temp_root("pin_coordination");
        write(
            &current.join("src/identity_gate_trust_anchor.rs"),
            "const LAUNCHER: &[u8] = include_bytes!(\"bin/etuschlopf_launcher.rs\");\n",
        );
        let change = |path: &str| DeclaredChange {
            kind: ChangeKind::Modified,
            path: path.to_string(),
            incoming_fnv1a64: Some(1),
        };
        let mut actual = BTreeMap::from([(
            "src/bin/etuschlopf_launcher.rs".to_string(),
            change("src/bin/etuschlopf_launcher.rs"),
        )]);
        assert!(validate_architecture_pin_coordination(&current, &actual).is_err());
        actual.insert(
            "src/kodex_build_orchestrator.rs".to_string(),
            change("src/kodex_build_orchestrator.rs"),
        );
        assert!(validate_architecture_pin_coordination(&current, &actual).is_err());
        actual.insert(
            "src/identity_gate_trust_anchor.rs".to_string(),
            change("src/identity_gate_trust_anchor.rs"),
        );
        assert!(validate_architecture_pin_coordination(&current, &actual).is_ok());
        let _ = fs::remove_dir_all(current);
    }


    #[test]
    fn corpus_independent_test_admission_rejects_new_concrete_corpus_identity() {
        let current = temp_root("corpus_test_identity_current");
        let incoming = temp_root("corpus_test_identity_incoming");
        write(
            &incoming.join("tests/new_invariant.rs"),
            "#[test]\nfn concrete_case() { let project = \"Etüde Nr. 6\"; assert!(!project.is_empty()); }\n",
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let error =
            validate_corpus_independent_test_admission(&incoming, &current, &actual)
                .expect_err("new concrete corpus identity must fail closed");
        assert!(error.contains("CORPUS-INDEPENDENT TEST ADMISSION KODEX"));
        assert!(error.contains("concrete_etude_identity"));
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn corpus_independent_test_admission_rejects_numeric_measure_witness() {
        let current = temp_root("corpus_test_measure_current");
        let incoming = temp_root("corpus_test_measure_incoming");
        write(
            &incoming.join("tests/new_invariant.rs"),
            "#[test]\nfn concrete_measure_case() { let xml = r#\"<measure number=\\\"33\\\"></measure>\"#; assert!(!xml.is_empty()); }\n",
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let error =
            validate_corpus_independent_test_admission(&incoming, &current, &actual)
                .expect_err("new absolute measure witness must fail closed");
        assert!(error.contains("concrete_measure_identity"));
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn corpus_independent_test_admission_rejects_non_synthetic_fixture_dependency() {
        let current = temp_root("corpus_test_fixture_current");
        let incoming = temp_root("corpus_test_fixture_incoming");
        write(
            &incoming.join("src/structural_write_authority/demo.rs"),
            "const SNAPSHOT: &str = include_str!(\"../../tests/fixtures/real_case.xml\");\n",
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let error =
            validate_corpus_independent_test_admission(&incoming, &current, &actual)
                .expect_err("new real fixture dependency must fail closed");
        assert!(error.contains("non_synthetic_or_historical_fixture_dependency"));
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn corpus_independent_test_admission_rejects_regression_named_test() {
        let current = temp_root("corpus_test_regression_current");
        let incoming = temp_root("corpus_test_regression_incoming");
        write(
            &incoming.join("tests/new_invariant.rs"),
            "#[test]\nfn old_bug_regression() { assert!(true); }\n",
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let error =
            validate_corpus_independent_test_admission(&incoming, &current, &actual)
                .expect_err("historical regression test identity must fail closed");
        assert!(error.contains("historical_regression_test_symbol"));
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn corpus_independent_test_admission_accepts_synthetic_symbolic_invariant() {
        let current = temp_root("corpus_test_synthetic_current");
        let incoming = temp_root("corpus_test_synthetic_incoming");
        write(
            &incoming.join("tests/new_invariant.rs"),
            "#[test]\nfn relocation_duplication_invariant() {\n    let left = r#\"<measure number=\\\"alpha\\\"></measure>\"#;\n    let right = r#\"<measure number=\\\"beta\\\"></measure>\"#;\n    assert_ne!(left, right);\n}\n",
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        validate_corpus_independent_test_admission(&incoming, &current, &actual)
            .expect("synthetic symbolic invariant must remain admissible");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }


    fn error_class_definition_contract_text() -> &'static str {
        "SelectorDerivationContract: causal_class_first_v1\n\
SelectorChange: no\n\
SelectorRole: error_class_definition\n\
KnownWitnessUsedForFeatureDerivation: no\n\
TargetConditionedFeatureNarrowing: no\n\
ClassDefinitionPrecedesCorpusDistribution: yes\n\
UniquenessIsOutcomeNotGoal: yes\n\
RepairSelectorSeparatedFromErrorClass: yes\n\
CausalFeatureNecessityReview: complete\n\
CorpusDistributionPurpose: not_yet_measured\n\
CorpusDistributionMayRefineClassToKnownWitness: no\n\
ParentErrorClassContract: none\n\
ErrorClassDefinitionIndependentOfKnownWitness: yes\n\
CausalFeature: semantic_line_fragmented_by_serialization_boundary\n\
NonCausalContextExcluded: preceding_rest_or_exact_beat_count\n"
    }

    fn selector_implementation_contract_text(role: &str, narrowing_basis: &str) -> String {
        format!(
            "SelectorDerivationContract: causal_class_first_v1\n\
SelectorChange: yes\n\
SelectorRole: {role}\n\
KnownWitnessUsedForFeatureDerivation: no\n\
TargetConditionedFeatureNarrowing: no\n\
ClassDefinitionPrecedesCorpusDistribution: yes\n\
UniquenessIsOutcomeNotGoal: yes\n\
RepairSelectorSeparatedFromErrorClass: yes\n\
CausalFeatureNecessityReview: complete\n\
CorpusDistributionPurpose: measurement_only\n\
CorpusDistributionMayRefineClassToKnownWitness: no\n\
MultiplicityInvarianceContract: candidate_duplication_v1\n\
IndependentCandidateDuplicationInvariant: yes\n\
ScorewideMultiplicityAsWriteAuthority: forbidden\n\
GlobalRankingOfIndependentCandidates: forbidden\n\
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group\n\
CandidateDuplicationProbe: required_before_productive_write\n\
ParentErrorClassContract: docs/selector_derivation_contract_parent.md\n\
ScopeNarrowingBasis: {narrowing_basis}\n\
DiscriminativeContextOnlyNarrowing: forbidden\n\
CausalFeature: inherited_parent_error_class\n\
NonCausalContextExcluded: witness_specific_layout_context\n"
        )
    }

    fn selector_fixture(current: &Path, incoming: &Path) {
        write(
            &current.join("src/audit_only/selector_fixture.rs"),
            "fn classify(candidates: &[usize]) -> Vec<usize> { candidates.to_vec() }\n",
        );
        write(
            &incoming.join("src/audit_only/selector_fixture.rs"),
            "fn classify(candidates: &[usize]) -> Vec<usize> {\n    candidates.iter().copied().filter(|candidate| *candidate > 1).collect()\n}\n",
        );
    }

    #[test]
    fn proxy_identity_selector_change_requires_derivation_contract() {
        let current = temp_root("proxy_identity_contract_missing_current");
        let incoming = temp_root("proxy_identity_contract_missing_incoming");
        selector_fixture(&current, &incoming);
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let error = validate_selector_derivation_governance(&incoming, &current, &actual)
            .expect_err("selector-sensitive change without contract must fail closed");
        assert!(error.contains("PROXY-IDENTITY SELECTOR KODEX"));
        assert!(error.contains("requires exactly one changed"));
    }

    #[test]
    fn causal_error_class_definition_is_admitted_only_without_selector_code() {
        let current = temp_root("causal_definition_current");
        let incoming = temp_root("causal_definition_incoming");
        write(
            &incoming.join("docs/selector_derivation_contract_parent.md"),
            error_class_definition_contract_text(),
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        validate_selector_derivation_governance(&incoming, &current, &actual)
            .expect("witness-blind class definition without selector code must pass");

        selector_fixture(&current, &incoming);
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let error = validate_selector_derivation_governance(&incoming, &current, &actual)
            .expect_err("class definition and selector code in one take must fail closed");
        assert!(error.contains("requires SelectorChange: yes"));
    }

    #[test]
    fn error_classification_requires_frozen_parent_class_contract() {
        let current = temp_root("classification_parent_current");
        let incoming = temp_root("classification_parent_incoming");
        selector_fixture(&current, &incoming);
        write(
            &incoming.join("docs/selector_derivation_contract_classifier.md"),
            &selector_implementation_contract_text("error_classification", "causal_class_only"),
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let error = validate_selector_derivation_governance(&incoming, &current, &actual)
            .expect_err("classifier without frozen parent class must fail closed");
        assert!(error.contains("parent error-class contract is not readable"));

        write(
            &current.join("docs/selector_derivation_contract_parent.md"),
            error_class_definition_contract_text(),
        );
        write(
            &incoming.join("docs/selector_derivation_contract_parent.md"),
            error_class_definition_contract_text(),
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        validate_selector_derivation_governance(&incoming, &current, &actual)
            .expect("classifier may consume an unchanged class contract from the accepted parent");
    }

    #[test]
    fn repair_scope_requires_frozen_parent_error_class_contract() {
        let current = temp_root("repair_scope_parent_current");
        let incoming = temp_root("repair_scope_parent_incoming");
        selector_fixture(&current, &incoming);
        write(
            &incoming.join("docs/selector_derivation_contract_repair.md"),
            &selector_implementation_contract_text("repair_scope", "authority_or_safety_only"),
        );
        write(
            &incoming.join("docs/selector_derivation_contract_parent.md"),
            error_class_definition_contract_text(),
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let error = validate_selector_derivation_governance(&incoming, &current, &actual)
            .expect_err("repair scope may not create its parent class in the same take");
        assert!(error.contains("requires exactly one changed"));

        write(
            &current.join("docs/selector_derivation_contract_parent.md"),
            error_class_definition_contract_text(),
        );
        write(
            &incoming.join("docs/selector_derivation_contract_parent.md"),
            "SelectorDerivationContract: causal_class_first_v1\nSelectorChange: no\nSelectorRole: error_class_definition\n",
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let error = validate_selector_derivation_governance(&incoming, &current, &actual)
            .expect_err("modified parent class remains forbidden");
        assert!(error.contains("requires exactly one changed"));

        fs::copy(
            current.join("docs/selector_derivation_contract_parent.md"),
            incoming.join("docs/selector_derivation_contract_parent.md"),
        )
        .expect("copy frozen parent contract");
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        validate_selector_derivation_governance(&incoming, &current, &actual)
            .expect("repair scope may narrow only against an unchanged accepted parent class");
    }

    #[test]
    fn multiplicity_sensitive_unique_candidate_change_requires_derivation_contract() {
        let current = temp_root("multiplicity_contract_missing_current");
        let incoming = temp_root("multiplicity_contract_missing_incoming");
        write(
            &current.join("src/audit_only/multiplicity_fixture.rs"),
            "fn eligible(repair_intents: &[usize]) -> bool { !repair_intents.is_empty() }\n",
        );
        write(
            &incoming.join("src/audit_only/multiplicity_fixture.rs"),
            "fn eligible(repair_intents: &[usize]) -> bool { let unique_candidate_green = repair_intents.len() == 1; unique_candidate_green }\n",
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let error = validate_selector_derivation_governance(&incoming, &current, &actual)
            .expect_err("scorewide candidate uniqueness must require a derivation contract");
        assert!(error.contains("PROXY-IDENTITY SELECTOR KODEX"));
        assert!(error.contains("requires exactly one changed"));
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn selector_implementation_contract_requires_candidate_duplication_invariance() {
        let current = temp_root("multiplicity_fields_current");
        let incoming = temp_root("multiplicity_fields_incoming");
        selector_fixture(&current, &incoming);
        write(
            &current.join("docs/selector_derivation_contract_parent.md"),
            error_class_definition_contract_text(),
        );
        write(
            &incoming.join("docs/selector_derivation_contract_parent.md"),
            error_class_definition_contract_text(),
        );
        let legacy =
            selector_implementation_contract_text("error_classification", "causal_class_only")
                .replace(
                    "MultiplicityInvarianceContract: candidate_duplication_v1\n",
                    "",
                )
                .replace("IndependentCandidateDuplicationInvariant: yes\n", "")
                .replace("ScorewideMultiplicityAsWriteAuthority: forbidden\n", "")
                .replace("GlobalRankingOfIndependentCandidates: forbidden\n", "")
                .replace(
                    "MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group\n",
                    "",
                )
                .replace(
                    "CandidateDuplicationProbe: required_before_productive_write\n",
                    "",
                );
        write(
            &incoming.join("docs/selector_derivation_contract_classifier.md"),
            &legacy,
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let error = validate_selector_derivation_governance(&incoming, &current, &actual)
            .expect_err("selector implementation without multiplicity contract must fail closed");
        assert!(error.contains("MultiplicityInvarianceContract"));
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn selector_implementation_contract_forbids_scorewide_multiplicity_authority() {
        let current = temp_root("multiplicity_scorewide_current");
        let incoming = temp_root("multiplicity_scorewide_incoming");
        selector_fixture(&current, &incoming);
        write(
            &current.join("docs/selector_derivation_contract_parent.md"),
            error_class_definition_contract_text(),
        );
        write(
            &incoming.join("docs/selector_derivation_contract_parent.md"),
            error_class_definition_contract_text(),
        );
        let invalid =
            selector_implementation_contract_text("repair_scope", "authority_or_safety_only")
                .replace(
                    "ScorewideMultiplicityAsWriteAuthority: forbidden",
                    "ScorewideMultiplicityAsWriteAuthority: allowed",
                );
        write(
            &incoming.join("docs/selector_derivation_contract_repair.md"),
            &invalid,
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let error = validate_selector_derivation_governance(&incoming, &current, &actual)
            .expect_err("scorewide multiplicity must not become write authority");
        assert!(error.contains("ScorewideMultiplicityAsWriteAuthority"));
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }


    fn retirement_contract_text(scope: &str, surface: &str) -> String {
        format!(
            "AuthorityRetirementContract: authority_retirement_v1\n\
AuthorityChange: retirement_only\n\
ParentAuthoritySurface: {surface}\n\
RetirementScopePaths: {scope}\n\
CandidateClassChange: forbidden\n\
SelectorRelationChange: forbidden\n\
TargetSelectionChange: forbidden\n\
MultiplicityDecisionChange: forbidden\n\
NewPositiveWriteDecision: forbidden\n\
ReplacementWriter: forbidden\n\
WriterCapabilityDirection: decrease_only\n\
CalibrationThresholdChange: forbidden\n\
ProductDeltaScope: removal_of_parent_writes_only\n\
CausalClassFirstBypass: forbidden\n\
CandidateDuplicationBypass: forbidden\n"
        )
    }

    fn retirement_fixture(
        current: &Path,
        incoming: &Path,
        selector_change: bool,
        threshold_change: bool,
    ) {
        let path = "src/pipeline_shards/retirement_fixture_apply.rs";
        let current_selector = "let Some(candidate) = candidates.iter().next() else { return; };";
        let incoming_selector = if selector_change {
            "let Some(candidate) = candidates.iter().find(|candidate| **candidate > 0) else { return; };"
        } else {
            current_selector
        };
        let current_threshold = "let max_size = 4usize;";
        let incoming_threshold = if threshold_change {
            "let max_size = 5usize;"
        } else {
            current_threshold
        };
        write(
            &current.join(path),
            &format!(
                "fn apply_candidate(candidates: &[usize], xml: &mut String) {{\n    {current_selector}\n    {current_threshold}\n    let _ = (candidate, max_size);\n    write_musicxml(xml);\n}}\nfn write_musicxml(xml: &mut String) {{ xml.insert_str(0, \"x\"); }}\n"
            ),
        );
        write(
            &incoming.join(path),
            &format!(
                "fn apply_candidate(candidates: &[usize], xml: &mut String) {{\n    {incoming_selector}\n    {incoming_threshold}\n    let _ = (candidate, max_size);\n}}\nfn write_musicxml(xml: &mut String) {{ xml.insert_str(0, \"x\"); }}\n"
            ),
        );
    }

    #[test]
    fn pure_writer_authority_loss_requires_retirement_contract() {
        let current = temp_root("authority_retirement_required_current");
        let incoming = temp_root("authority_retirement_required_incoming");
        retirement_fixture(&current, &incoming, false, false);
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let manifest = parse_manifest(
            "schema_version=1\nchange_class=MusicMutation\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nchange=modified|src/pipeline_shards/retirement_fixture_apply.rs|fnv64:0000000000000001\n",
        )
        .expect("manifest");
        let error = validate_authority_retirement_governance(&manifest, &incoming, &current, &actual)
            .expect_err("pure writer loss without retirement contract must fail closed");
        assert!(error.contains("AUTHORITY-RETIREMENT KODEX"));
        assert!(error.contains("requires an explicit authority_retirement_v1 contract"));
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_retirement_cross_role_admission_is_manifest_bound_and_path_specific() {
        let accepts = |text: &str| validate_manifest_path_policy(&parse_manifest(text).unwrap()).is_ok();
        let prefix = "schema_version=1\nchange_class=MusicMutation\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\n";
        let path = "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs";
        let change = format!("change=modified|{path}|fnv64:0000000000000001\n");
        assert!(!accepts(&format!("{prefix}{change}")));
        let bound = "authority_retirement_contract=docs/authority_retirement_contract_fixture.md\n";
        assert!(accepts(&format!("{prefix}{bound}{change}")));
        assert!(!accepts(&format!("{prefix}{bound}{}", change.replace(path, "src/audit_only/unrelated.rs"))));
        assert!(!accepts(&format!("{prefix}{bound}{}", change.replace("modified|", "added|"))));
    }

    #[test]
    fn generic_retirement_cannot_use_the_ssc_cross_role_exception() {
        let current = temp_root("ssc_cross_role_parent");
        let incoming = temp_root("ssc_cross_role_child");
        retirement_fixture(&current, &incoming, false, false);
        let audit = "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs";
        write(&current.join(audit), "fn report() { observe(); }\n");
        write(&incoming.join(audit), "fn report() {}\n");
        let doc = "docs/authority_retirement_contract_fixture.md";
        write(&incoming.join(doc), &retirement_contract_text(
            &format!("src/pipeline_shards/retirement_fixture_apply.rs, {audit}"),
            "src/pipeline_shards/retirement_fixture_apply.rs::apply_candidate",
        ));
        let manifest = parse_manifest(&format!(
            "schema_version=1\nchange_class=MusicMutation\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nauthority_retirement_contract={doc}\nchange=modified|{audit}|fnv64:0000000000000001\n"
        )).unwrap();
        let actual = collect_actual_changes(&incoming, &current).unwrap();
        assert!(validate_authority_retirement_governance(&manifest, &incoming, &current, &actual)
            .unwrap_err().contains("SSC cross-role retirement requires exact declared structural erasure"));
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn authority_retirement_keeps_selector_and_threshold_semantics_frozen() {
        let current = temp_root("authority_retirement_frozen_current");
        let incoming = temp_root("authority_retirement_frozen_incoming");
        retirement_fixture(&current, &incoming, false, false);
        let contract_path = "docs/authority_retirement_contract_fixture.md";
        write(
            &incoming.join(contract_path),
            &retirement_contract_text(
                "src/pipeline_shards/retirement_fixture_apply.rs",
                "src/pipeline_shards/retirement_fixture_apply.rs::apply_candidate",
            ),
        );
        let actual = collect_actual_changes(&incoming, &current).expect("actual diff");
        let manifest = parse_manifest(&format!(
            "schema_version=1\nchange_class=MusicMutation\nfrom_version=1.0.0\nto_version=1.0.1\nreason=test\nauthority_retirement_contract={contract_path}\nchange=modified|src/pipeline_shards/retirement_fixture_apply.rs|fnv64:0000000000000001\nchange=added|{contract_path}|fnv64:0000000000000002\n"
        ))
        .expect("manifest");
        validate_authority_retirement_governance(&manifest, &incoming, &current, &actual)
            .expect("pure writer loss with frozen selector/threshold semantics must pass");

        let bad_selector = temp_root("authority_retirement_selector_incoming");
        retirement_fixture(&current, &bad_selector, true, false);
        write(
            &bad_selector.join(contract_path),
            &retirement_contract_text(
                "src/pipeline_shards/retirement_fixture_apply.rs",
                "src/pipeline_shards/retirement_fixture_apply.rs::apply_candidate",
            ),
        );
        let actual = collect_actual_changes(&bad_selector, &current).expect("selector actual");
        let error = validate_authority_retirement_governance(&manifest, &bad_selector, &current, &actual)
            .expect_err("selector relation change must remain forbidden");
        assert!(error.contains("selector/candidate/multiplicity"));

        let bad_threshold = temp_root("authority_retirement_threshold_incoming");
        retirement_fixture(&current, &bad_threshold, false, true);
        write(
            &bad_threshold.join(contract_path),
            &retirement_contract_text(
                "src/pipeline_shards/retirement_fixture_apply.rs",
                "src/pipeline_shards/retirement_fixture_apply.rs::apply_candidate",
            ),
        );
        let actual = collect_actual_changes(&bad_threshold, &current).expect("threshold actual");
        let error = validate_authority_retirement_governance(&manifest, &bad_threshold, &current, &actual)
            .expect_err("threshold coupling must remain forbidden");
        assert!(error.contains("CalibrationThreshold"));

        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
        let _ = fs::remove_dir_all(bad_selector);
        let _ = fs::remove_dir_all(bad_threshold);
    }

    #[test]
    fn governed_change_manifest_requires_music_effect_and_known_kinds() {
        let base = "schema_version=1\nchange_class=GovernedChange\nfrom_version=1.0.0\nto_version=1.0.0\nreason=test\nchange=modified|src/ssc_evidence_proof.rs|fnv64:0000000000000001\n";
        assert!(parse_manifest(base).unwrap_err().contains("music_effect"));
        let manifest = parse_manifest(&format!(
            "{base}music_effect=none\ngovern=proof_rule|src/ssc_evidence_proof.rs|containment verdicts v4\n"
        ))
        .expect("GovernedChange manifest");
        assert_eq!(manifest.change_class, ChangeClass::GovernedChange);
        assert_eq!(manifest.music_effect.as_deref(), Some("none"));
        assert_eq!(manifest.governed_changes.len(), 1);
        assert!(parse_manifest(&format!("{base}music_effect=none\ngovern=relax|src/ssc_evidence_proof.rs|containment verdicts v4\n")).is_err());
        assert!(parse_manifest(&format!("{base}music_effect=none\nmusic_effect=none\n")).is_err());
        let refactor = base.replace("GovernedChange", "RefactorOnly");
        assert!(parse_manifest(&format!("{refactor}music_effect=none\n")).unwrap_err().contains("GovernedChange"));
        assert!(parse_manifest(&format!("{refactor}govern=proof_rule|src/ssc_evidence_proof.rs|containment verdicts v4\n")).is_err());
    }

    #[test]
    fn governed_change_path_policy_never_admits_validator_code() {
        let manifest_for = |paths: &[&str], extra: &str| {
            let mut text = format!("schema_version=1\nchange_class=GovernedChange\nfrom_version=1.0.0\nto_version=1.0.0\nreason=test\nmusic_effect=none\n{extra}");
            for path in paths {
                text.push_str(&format!("change=modified|{path}|fnv64:0000000000000001\n"));
            }
            parse_manifest(&text).expect("manifest")
        };
        let runtime = manifest_for(
            &["src/ssc_evidence_proof.rs", "src/master_state_identity.rs", "src/structural_write_authority/mod.rs", "tests/support/x.rs", "docs/x.md", "src/build_gate/ssc_containment_envelopes_v1.tsv"],
            "",
        );
        assert!(validate_manifest_path_policy(&runtime).is_ok());
        for validator in [
            "src/take_change_control.rs",
            "src/build_gate/ssc_governed_change.rs",
            "src/build_gate/ssc_k2_retirement_governance.rs",
            "src/build_gate/semantic_capability_diff.rs",
            "src/identity_gate_trust_anchor.rs",
            "src/build_gate/kodex_capability_retirements.txt",
            "run.cmd",
        ] {
            assert!(validate_manifest_path_policy(&manifest_for(&[validator], "")).is_err(), "{validator}");
        }
        let retire = manifest_for(
            &["src/build_gate/kodex_capability_retirements.txt", "src/identity_gate_trust_anchor.rs"],
            "govern=retire|src/build_gate/synthetic_check.rs::old_check|synthetic check retired\n",
        );
        assert!(validate_manifest_path_policy(&retire).is_ok());
    }
}
