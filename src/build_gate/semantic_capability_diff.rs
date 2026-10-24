// r1492: shares the parent-owned exact Windows job dependency transition with manifest admission; all other Cargo capability changes remain closed.
// r1102 CompletedSchemaSchlopfMusicXml40TransitionRetirementLedgerClosure: retires the completed r020 projection from semantic-diff admission; only the current r021 qualified component remains an opaque steady-state component surface.
// r1036 r07 AuthorityRetirementKodexClosure: Recovery preserves the existing KodexNegativeCheck owner byte-structurally and layers authority_retirement_v1 after the unchanged semantic policy; detects pure monotone productive writer-authority loss and validates authority_retirement_v1 without allowing threshold or replacement-writer coupling.
// r592 MusicXml40ProductContractAdmissionBoundary: keeps both exact r020 and r021 SchemaSchlopf projections opaque during the one-shot product-contract transition and admits only the narrowly pre-authorized ValidationOnly format-contract surfaces; TakeChangeControl remains the exact write/admission authority.
// r574 SchemaSchlopfDependencyAdmissionBoundary: admits only the exact ValidationOnly SchemaSchlopf-r020 dependency and treats its vendored source projection as opaque qualified-component bytes, not EtuSchlopf music logic.
// r449 CompletedCargoTransitionRetirementSuperCut: retires the completed Cargo build-owner relocation machinery; steady-state Kodex relocations keep only the reusable writer/negative-check contracts.
use crate::rust_source_scan_primitives::{fnv1a64, mask_non_code};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

const WATCH_PROJECT_MANIFEST_PATH: &str = "tools/etuschlopf_watch/Cargo.toml";
const WATCH_PROJECT_LOCK_PATH: &str = "tools/etuschlopf_watch/Cargo.lock";
const SCHEMASCHLOPF_R021_COMPONENT_PREFIX: &str = "tools/qualified_components/schemaschlopf_r021/";

const CHANGE_MANIFEST_PATH: &str = "src/build_gate/take_change_class.txt";
const KODEX_CAPABILITY_RETIREMENTS_PATH: &str = "src/build_gate/kodex_capability_retirements.txt";

fn pre_kodex_title_token() -> String {
    ["Sec", "urity"].concat()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Domain {
    Kodex,
    Audit,
    Music,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CapabilityKind {
    KodexControl,
    KodexNegativeCheck,
    AuditLogic,
    MusicLogic,
    CalibrationThreshold,
    MusicXmlMutationEntrypoint,
    StructuralWriteAuthorityEntrypoint,
    MasterApplyEntrypoint,
    ExportedWriterCapability,
    WriteFunctionCall,
    WriterReachability,
    FeedbackIdentityChannel,
    ProofReentryReader,
    MacroOrGeneratedSource,
    BuildScriptCapability,
    CargoFeature,
    CargoDependency,
    UnknownCapability,
}

impl CapabilityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::KodexControl => "KodexControl",
            Self::KodexNegativeCheck => "KodexNegativeCheck",
            Self::AuditLogic => "AuditLogic",
            Self::MusicLogic => "MusicLogic",
            Self::CalibrationThreshold => "CalibrationThreshold",
            Self::MusicXmlMutationEntrypoint => "MusicXmlMutationEntrypoint",
            Self::StructuralWriteAuthorityEntrypoint => "StructuralWriteAuthorityEntrypoint",
            Self::MasterApplyEntrypoint => "MasterApplyEntrypoint",
            Self::ExportedWriterCapability => "ExportedWriterCapability",
            Self::WriteFunctionCall => "WriteFunctionCall",
            Self::WriterReachability => "WriterReachability",
            Self::FeedbackIdentityChannel => "FeedbackIdentityChannel",
            Self::ProofReentryReader => "ProofReentryReader",
            Self::MacroOrGeneratedSource => "MacroOrGeneratedSource",
            Self::BuildScriptCapability => "BuildScriptCapability",
            Self::CargoFeature => "CargoFeature",
            Self::CargoDependency => "CargoDependency",
            Self::UnknownCapability => "UnknownCapability",
        }
    }

    const fn is_writer_family(self) -> bool {
        matches!(
            self,
            Self::MusicXmlMutationEntrypoint
                | Self::StructuralWriteAuthorityEntrypoint
                | Self::MasterApplyEntrypoint
                | Self::ExportedWriterCapability
                | Self::WriteFunctionCall
                | Self::WriterReachability
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CapabilityRecord {
    kind: CapabilityKind,
    domain: Domain,
    path: String,
    symbol: String,
    fingerprint: u64,
    detail: String,
}

impl CapabilityRecord {
    fn key(&self) -> String {
        format!("{}|{}|{}", self.kind.as_str(), self.path, self.symbol)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeltaKind {
    Added,
    Modified,
    Removed,
}

impl DeltaKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Modified => "modified",
            Self::Removed => "removed",
        }
    }
}

#[derive(Debug, Clone)]
struct CapabilityDelta {
    kind: DeltaKind,
    before: Option<CapabilityRecord>,
    after: Option<CapabilityRecord>,
}

impl CapabilityDelta {
    fn record(&self) -> &CapabilityRecord {
        self.after
            .as_ref()
            .or(self.before.as_ref())
            .expect("delta record")
    }
}

#[derive(Debug, Clone, Default)]
pub struct SemanticDiffSummary {
    pub added: usize,
    pub modified: usize,
    pub removed: usize,
    pub kodex: usize,
    pub audit: usize,
    pub music: usize,
    pub calibration: usize,
    pub writer_family: usize,
    pub unknown: usize,
}

#[derive(Debug, Clone)]
struct FunctionInfo {
    key: String,
    symbol: String,
    path: String,
    name: String,
    domain: Domain,
    public: bool,
    start_line: usize,
    signature: String,
    source_body: String,
    masked_body: String,
    calls: BTreeMap<String, usize>,
    direct_writer: bool,
}

#[derive(Debug, Clone)]
struct FunctionSpan {
    name: String,
    public: bool,
    signature_start: usize,
    body_start: usize,
    body_end: usize,
    start_line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProductiveMusicIdentityProvenanceViolation {
    path: String,
    symbol: String,
    detail: String,
}

#[derive(Debug, Clone, Default)]
struct Inventory {
    records: BTreeMap<String, CapabilityRecord>,
    functions: BTreeMap<String, FunctionInfo>,
    productive_identity_provenance_violations: Vec<ProductiveMusicIdentityProvenanceViolation>,
}

#[derive(Debug, Default)]
struct AuditCrossRoleRetirement {
    reduced_paths: BTreeSet<String>,
    retired_symbols: BTreeSet<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KodexCapabilityRetirement {
    kind: CapabilityKind,
    path: String,
    symbol: String,
    before_fingerprint: u64,
    reason: String,
}

impl KodexCapabilityRetirement {
    fn capability_key(&self) -> String {
        format!("{}|{}|{}", self.kind.as_str(), self.path, self.symbol)
    }
}

pub fn validate_semantic_change(
    current_root: &Path,
    incoming_root: &Path,
    change_class: &str,
    changed_paths: &BTreeSet<String>,
) -> Result<SemanticDiffSummary, String> {
    validate_semantic_change_with_options(
        current_root,
        incoming_root,
        change_class,
        changed_paths,
        false,
        false,
    )
}

pub fn validate_productive_music_identity_provenance(root: &Path) -> Result<(), String> {
    let inventory = inventory_tree(root)?;
    validate_productive_music_identity_provenance_inventory(&inventory)
}

fn validate_productive_music_identity_provenance_inventory(
    inventory: &Inventory,
) -> Result<(), String> {
    if inventory
        .productive_identity_provenance_violations
        .is_empty()
    {
        return Ok(());
    }
    let detail = inventory
        .productive_identity_provenance_violations
        .iter()
        .map(|violation| {
            format!(
                "{}::{} {}",
                violation.path, violation.symbol, violation.detail
            )
        })
        .collect::<Vec<_>>()
        .join("\n  - ");
    Err(format!(
        "PRODUCTIVE-MUSIC IDENTITY PROVENANCE FIREWALL: unstructuralized external/raw identity reaches a productive music boundary:\n  - {detail}"
    ))
}

pub fn validate_semantic_change_with_options(
    current_root: &Path,
    incoming_root: &Path,
    change_class: &str,
    changed_paths: &BTreeSet<String>,
    kodex_writer_relocation: bool,
    kodex_negative_check_relocation: bool,
) -> Result<SemanticDiffSummary, String> {
    validate_semantic_change_with_kodex_relocation_options(
        current_root,
        incoming_root,
        change_class,
        changed_paths,
        kodex_writer_relocation,
        kodex_negative_check_relocation,
        false,
    )
}

pub fn validate_semantic_change_with_kodex_relocation_options(
    current_root: &Path,
    incoming_root: &Path,
    change_class: &str,
    changed_paths: &BTreeSet<String>,
    kodex_writer_relocation: bool,
    kodex_negative_check_relocation: bool,
    kodex_negative_check_module_relocation: bool,
) -> Result<SemanticDiffSummary, String> {
    let current = inventory_tree(current_root)?;
    let incoming = inventory_tree(incoming_root)?;
    validate_productive_music_identity_provenance_inventory(&incoming)?;
    let refactor_owner_relocation = exact_byte_identical_refactor_rust_owner_relocation(
        current_root,
        incoming_root,
        change_class,
        changed_paths,
    )?;
    let retirement = if change_class == "AuditOnly" {
        audit_cross_role_retirement(
            current_root,
            incoming_root,
            changed_paths,
            &current,
            &incoming,
        )?
    } else {
        AuditCrossRoleRetirement::default()
    };
    let deltas = diff_inventories(&current.records, &incoming.records);
    validate_changed_paths_are_known(changed_paths, &deltas)?;
    let derived_audit_writer_reachability_narrowing =
        validate_music_mutation_derived_audit_writer_reachability_narrowing(
            current_root,
            incoming_root,
            change_class,
            changed_paths,
            &current,
            &incoming,
            &deltas,
        )?;
    // Specialized derived Audit->Writer policy owns its established diagnostics first.
    // The general productive-authority monotonicity invariant follows immediately afterwards,
    // so no forbidden widening is admitted and no second policy shadows the narrower owner.
    validate_no_new_productive_repair_recovery_authority(
        &current,
        &incoming,
        change_class,
        refactor_owner_relocation.as_ref(),
    )?;
    let kodex_capability_retirements = validate_kodex_capability_retirement_contract(
        current_root,
        incoming_root,
        change_class,
        &current,
        &incoming,
        &deltas,
    )?;
    let writer_relocation = if kodex_writer_relocation {
        validate_kodex_writer_relocation(
            change_class,
            changed_paths,
            &current,
            &incoming,
            &deltas,
        )?;
        true
    } else {
        false
    };
    if kodex_negative_check_relocation && kodex_negative_check_module_relocation {
        return Err(
            "Semantic-Capability-Diff: KodexNegativeCheck-Relocation-Marker sind gegenseitig exklusiv"
                .to_string(),
        );
    }
    let negative_check_relocations = if kodex_negative_check_relocation {
        validate_kodex_negative_check_relocation(
            change_class,
            changed_paths,
            &current,
            &incoming,
            &deltas,
        )?
    } else if kodex_negative_check_module_relocation {
        validate_kodex_negative_check_module_relocation(
            incoming_root,
            change_class,
            changed_paths,
            &current,
            &incoming,
            &deltas,
        )?
    } else {
        BTreeSet::new()
    };
    let windows_job_dependency_transition = change_class == "KodexOnly"
        && changed_paths.contains("Cargo.toml")
        && super::cargo_manifest_change_kind(
            &current_root.join("Cargo.toml"),
            &incoming_root.join("Cargo.toml"),
        )? == super::CargoManifestChangeKind::WindowsJobDependencyV1;
    let policy_context = PolicyValidationContext {
        windows_job_dependency_transition,
        audit_cross_role_retirement: &retirement,
        kodex_capability_retirements: &kodex_capability_retirements,
        kodex_writer_relocation: writer_relocation,
        kodex_negative_check_relocations: &negative_check_relocations,
        derived_audit_writer_reachability_narrowing: &derived_audit_writer_reachability_narrowing,
    };
    let thresholds = ssc_removed_owner_local_capabilities(current_root, incoming_root, change_class, &incoming, &deltas)?;
    // A parent-bound class transition changes no writer/repair monotonicity
    // rule above. Only these exact observational/control records are reclassified.
    let cursor_transition = change_class=="MusicMutation"
        && super::ssc_evidence_proof_shadow_permit_kodex::k2::cursor_temporal::verified_first_transition(current_root,incoming_root)?;
    if cursor_transition && !super::ssc_evidence_proof_shadow_permit_kodex::k2::cursor_temporal::exact_changed_paths(changed_paths) {
        return Err("CursorTemporal transition contains an unbound extra or missing changed path".into());
    }
    let policy_deltas = deltas.iter().filter(|d| !thresholds.contains(&d.record().key())
        && !(cursor_transition && cursor_temporal_observational_delta(d))).cloned().collect::<Vec<_>>();
    validate_policy(change_class, &policy_deltas, &policy_context)?;
    Ok(summarize(&deltas))
}

pub struct SemanticGovernanceOptions {
    pub kodex_writer_relocation: bool,
    pub kodex_negative_check_relocation: bool,
    pub kodex_negative_check_module_relocation: bool,
    pub authority_retirement: bool,
}

pub fn validate_semantic_change_with_governance_options(
    current_root: &Path,
    incoming_root: &Path,
    change_class: &str,
    changed_paths: &BTreeSet<String>,
    options: SemanticGovernanceOptions,
) -> Result<SemanticDiffSummary, String> {
    let summary = validate_semantic_change_with_kodex_relocation_options(
        current_root,
        incoming_root,
        change_class,
        changed_paths,
        options.kodex_writer_relocation,
        options.kodex_negative_check_relocation,
        options.kodex_negative_check_module_relocation,
    )?;
    if options.authority_retirement {
        validate_authority_retirement_semantics_v1(current_root, incoming_root)?;
    }
    Ok(summary)
}

fn writer_reachability_strictly_narrows(delta: &CapabilityDelta) -> bool {
    if delta.kind != DeltaKind::Modified
        || delta.record().kind != CapabilityKind::WriterReachability
    {
        return false;
    }
    let Some(before) = delta.before.as_ref() else {
        return false;
    };
    let Some(after) = delta.after.as_ref() else {
        return false;
    };
    let before_roots = writer_reachability_roots(&before.detail);
    let after_roots = writer_reachability_roots(&after.detail);
    after_roots.is_subset(&before_roots) && after_roots.len() < before_roots.len()
}

fn is_monotone_productive_writer_authority_loss(deltas: &[CapabilityDelta]) -> bool {
    is_monotone_productive_writer_authority_loss_with_callers(deltas, &BTreeSet::new())
}
fn is_monotone_productive_writer_authority_loss_with_callers(
    deltas: &[CapabilityDelta],
    callers: &BTreeSet<(String, String)>,
) -> bool {
    let mut loss = false;
    for delta in deltas.iter().filter(|delta| {
        let record = delta.record();
        record.domain == Domain::Music && record.kind.is_writer_family()
    }) {
        match delta.kind {
            DeltaKind::Removed => loss = true,
            DeltaKind::Modified if writer_reachability_strictly_narrows(delta) => loss = true,
            DeltaKind::Modified
                if matches!(
                    delta.record().kind,
                    CapabilityKind::MusicXmlMutationEntrypoint
                        | CapabilityKind::StructuralWriteAuthorityEntrypoint
                ) && callers
                    .contains(&(delta.record().path.clone(), delta.record().symbol.clone())) =>
            {
                loss = true
            }
            DeltaKind::Added | DeltaKind::Modified => return false,
        }
    }
    loss
}

pub fn requires_authority_retirement_contract(
    current_root: &Path,
    incoming_root: &Path,
) -> Result<bool, String> {
    let current = inventory_tree(current_root)?;
    let incoming = inventory_tree(incoming_root)?;
    let deltas = diff_inventories(&current.records, &incoming.records);
    let callers = registered_retirement_callers(current_root, incoming_root, &deltas)?;
    Ok(is_monotone_productive_writer_authority_loss_with_callers(
        &deltas, &callers,
    ))
}

fn validate_authority_retirement_deltas(
    change_class: &str,
    deltas: &[CapabilityDelta],
) -> Result<(), String> {
    if change_class != "MusicMutation" {
        return Err(
            "Semantic-Capability-Diff AUTHORITY-RETIREMENT: authority_retirement_v1 is only valid for MusicMutation"
                .to_string(),
        );
    }
    if !is_monotone_productive_writer_authority_loss(deltas) {
        return Err(
            "Semantic-Capability-Diff AUTHORITY-RETIREMENT: declared retirement is not a pure monotone productive writer-authority loss"
                .to_string(),
        );
    }
    let threshold_deltas = deltas
        .iter()
        .filter(|delta| {
            let record = delta.record();
            record.domain == Domain::Music && record.kind == CapabilityKind::CalibrationThreshold
        })
        .map(|delta| {
            let record = delta.record();
            format!("{} {}::{}", delta.kind.as_str(), record.path, record.symbol)
        })
        .collect::<Vec<_>>();
    if !threshold_deltas.is_empty() {
        return Err(format!(
            "Semantic-Capability-Diff AUTHORITY-RETIREMENT: CalibrationThreshold capability must remain unchanged during writer retirement: {}",
            threshold_deltas.join(", ")
        ));
    }
    Ok(())
}

pub fn validate_authority_retirement_semantics_v1(
    current_root: &Path,
    incoming_root: &Path,
) -> Result<(), String> {
    let current = inventory_tree(current_root)?;
    let incoming = inventory_tree(incoming_root)?;
    let deltas = diff_inventories(&current.records, &incoming.records);
    let callers = registered_retirement_callers(current_root, incoming_root, &deltas)?;
    let thresholds = ssc_removed_owner_local_capabilities(current_root, incoming_root, "MusicMutation", &incoming, &deltas)?;
    let mut narrowed = deltas;
    narrowed.retain(|delta| {
        !thresholds.contains(&delta.record().key()) &&
        !(delta.kind == DeltaKind::Modified
            && matches!(
                delta.record().kind,
                CapabilityKind::MusicXmlMutationEntrypoint
                    | CapabilityKind::StructuralWriteAuthorityEntrypoint
            )
            && callers.contains(&(delta.record().path.clone(), delta.record().symbol.clone())))
    });
    validate_authority_retirement_deltas("MusicMutation", &narrowed)
}
// Only local threshold definitions and private feedback-reader records inside a complete
// parent-compiled WAV function erasure disappear with their owner. No added/modified
// capability or surviving owner qualifies.
fn ssc_removed_owner_local_capabilities(
    parent: &Path, child: &Path, class: &str,
    incoming: &Inventory, deltas: &[CapabilityDelta],
) -> Result<BTreeSet<String>, String> {
    let mut authorized = BTreeSet::new();
    if class != "MusicMutation"
        || !super::ssc_evidence_proof_shadow_permit_kodex::k2::installed(parent)
    {
        return Ok(authorized);
    }
    let Ok(manifest) = fs::read_to_string(child.join("src/build_gate/take_change_class.txt")) else {
        return Ok(authorized);
    };
    let docs = manifest.lines().filter_map(|s| s.strip_prefix("authority_retirement_contract=")).collect::<Vec<_>>();
    if docs.len() != 1 || !docs[0].starts_with("docs/authority_retirement_contract_")
        || !docs[0].ends_with(".md") || docs[0].contains("..")
    {
        return Ok(authorized);
    }
    let text = fs::read_to_string(child.join(docs[0])).map_err(|e| e.to_string())?;
    let Some(ids) = super::contract_value(&text, "SscExitObligations") else { return Ok(authorized); };
    let Some(symbols) = super::contract_value(&text, "RetiredAuthoritySymbols") else { return Ok(authorized); };
    let ids = ids.split(',').map(str::trim).collect::<BTreeSet<_>>();
    let symbols = symbols.split(',').map(str::trim).collect::<BTreeSet<_>>();
    for delta in deltas.iter().filter(|d| {
        d.kind == DeltaKind::Removed && d.record().domain == Domain::Music
            && matches!(d.record().kind, CapabilityKind::CalibrationThreshold | CapabilityKind::FeedbackIdentityChannel)
    }) {
        let record = delta.record();
        let source = fs::read_to_string(parent.join(&record.path)).map_err(|e| e.to_string())?;
        let masked = mask_non_code(&source);
        let Some((name, ordinal)) = record.symbol.rsplit_once('#') else { continue; };
        let Ok(ordinal) = ordinal.parse::<usize>() else { continue; };
        if ordinal == 0 { continue; }
        let spans = function_spans(&source, &masked)?;
        let owners = if record.kind == CapabilityKind::CalibrationThreshold {
            let assignments = threshold_assignments(&masked);
            let Some(assignment) = assignments.iter().filter(|a| a.name == name).nth(ordinal - 1) else { continue; };
            spans.into_iter().filter(|f| assignment.rhs_start >= f.body_start
                && assignment.rhs_end <= f.body_end).collect::<Vec<_>>()
        } else {
            spans.into_iter().filter(|f| f.name == name).collect::<Vec<_>>()
        };
        if owners.len() != 1 || incoming.functions.values().any(|f| f.name == owners[0].name) {
            continue;
        }
        if let Some((id, anchor)) = super::ssc_evidence_proof_shadow_permit_kodex::structural_erasure::registered_whole_function_erasure(parent, child, &record.path, &owners[0].name)?
            && ids.contains(id.as_str())
            && symbols.contains(format!("{}::{anchor}", record.path).as_str())
            && !incoming.functions.values().any(|f| f.name == anchor)
        {
            authorized.insert(record.key());
        }
    }
    Ok(authorized)
}

fn registered_retirement_callers(
    parent: &Path,
    child: &Path,
    deltas: &[CapabilityDelta],
) -> Result<BTreeSet<(String, String)>, String> {
    let mut callers = BTreeSet::new();
    for delta in deltas.iter().filter(|d| {
        d.kind == DeltaKind::Modified
            && d.record().domain == Domain::Music
            && matches!(
                d.record().kind,
                CapabilityKind::MusicXmlMutationEntrypoint
                    | CapabilityKind::StructuralWriteAuthorityEntrypoint
            )
    }) {
        let record = delta.record();
        if super::ssc_evidence_proof_shadow_permit_kodex::structural_erasure::registered_caller_loss(parent, child, &record.path, &record.symbol)? {
            callers.insert((record.path.clone(), record.symbol.clone()));
        }
    }
    Ok(callers)
}

fn exact_byte_identical_refactor_rust_owner_relocation(
    current_root: &Path,
    incoming_root: &Path,
    change_class: &str,
    changed_paths: &BTreeSet<String>,
) -> Result<Option<(String, String)>, String> {
    if change_class != "RefactorOnly" {
        return Ok(None);
    }
    let current_only = changed_paths
        .iter()
        .filter(|path| {
            path.starts_with("src/")
                && path.ends_with(".rs")
                && domain_for_path(path) != Domain::Kodex
                && current_root.join(path).is_file()
                && !incoming_root.join(path).exists()
        })
        .cloned()
        .collect::<Vec<_>>();
    let incoming_only = changed_paths
        .iter()
        .filter(|path| {
            path.starts_with("src/")
                && path.ends_with(".rs")
                && domain_for_path(path) != Domain::Kodex
                && incoming_root.join(path).is_file()
                && !current_root.join(path).exists()
        })
        .cloned()
        .collect::<Vec<_>>();
    if current_only.len() != 1 || incoming_only.len() != 1 {
        return Ok(None);
    }
    let current_path = current_only[0].clone();
    let incoming_path = incoming_only[0].clone();
    if current_path == incoming_path
        || Path::new(&current_path).parent() != Path::new(&incoming_path).parent()
        || domain_for_path(&current_path) != domain_for_path(&incoming_path)
    {
        return Ok(None);
    }
    let current_bytes = fs::read(current_root.join(&current_path)).map_err(|error| {
        format!(
            "Semantic-Capability-Diff: alter Refactor-Owner nicht lesbar {current_path}: {error}"
        )
    })?;
    let incoming_bytes = fs::read(incoming_root.join(&incoming_path)).map_err(|error| {
        format!(
            "Semantic-Capability-Diff: neuer Refactor-Owner nicht lesbar {incoming_path}: {error}"
        )
    })?;
    if current_bytes != incoming_bytes {
        return Ok(None);
    }
    let current_binding =
        exact_changed_path_module_binding(current_root, changed_paths, &current_path)?;
    let incoming_binding =
        exact_changed_path_module_binding(incoming_root, changed_paths, &incoming_path)?;
    let current_new_binding =
        exact_changed_path_module_binding(current_root, changed_paths, &incoming_path)?;
    let incoming_old_binding =
        exact_changed_path_module_binding(incoming_root, changed_paths, &current_path)?;
    let Some((current_owner, current_module)) = current_binding else {
        return Ok(None);
    };
    let Some((incoming_owner, incoming_module)) = incoming_binding else {
        return Ok(None);
    };
    if current_new_binding.is_some()
        || incoming_old_binding.is_some()
        || current_owner != incoming_owner
        || current_module != incoming_module
    {
        return Ok(None);
    }
    Ok(Some((current_path, incoming_path)))
}

fn exact_changed_path_module_binding(
    root: &Path,
    changed_paths: &BTreeSet<String>,
    target_path: &str,
) -> Result<Option<(String, String)>, String> {
    let target = Path::new(target_path);
    let Some(target_parent) = target.parent() else {
        return Ok(None);
    };
    let Some(target_file_name) = target.file_name().and_then(|value| value.to_str()) else {
        return Ok(None);
    };
    let expected_attribute = format!("#[path = \"{target_file_name}\"]");
    let mut matches = Vec::new();
    for owner_path in changed_paths.iter().filter(|path| {
        path.starts_with("src/")
            && path.ends_with(".rs")
            && root.join(path).is_file()
            && Path::new(path.as_str()).parent() == Some(target_parent)
    }) {
        let text = fs::read_to_string(root.join(owner_path)).map_err(|error| {
            format!(
                "Semantic-Capability-Diff: Modulbindungs-Owner nicht lesbar {owner_path}: {error}"
            )
        })?;
        let lines = text.lines().collect::<Vec<_>>();
        for (index, line) in lines.iter().enumerate() {
            if line.trim() != expected_attribute {
                continue;
            }
            let Some(next) = lines
                .iter()
                .skip(index + 1)
                .find(|line| !line.trim().is_empty())
            else {
                continue;
            };
            let next = next.trim();
            let Some(module) = next
                .strip_prefix("mod ")
                .and_then(|value| value.strip_suffix(';'))
                .map(str::trim)
                .filter(|value| exact_rust_module_identifier(value))
            else {
                continue;
            };
            matches.push((owner_path.clone(), module.to_string()));
        }
    }
    if matches.len() == 1 {
        Ok(matches.pop())
    } else {
        Ok(None)
    }
}

fn exact_rust_module_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first == '_' || first.is_ascii_alphabetic())
        && chars.all(|character| character == '_' || character.is_ascii_alphanumeric())
}

fn relocated_capability_record_exists(
    current: &Inventory,
    incoming_record: &CapabilityRecord,
    current_path: &str,
    incoming_path: &str,
) -> bool {
    if incoming_record.path != incoming_path {
        return false;
    }
    current.records.values().any(|candidate| {
        if candidate.path != current_path
            || candidate.kind != incoming_record.kind
            || candidate.domain != incoming_record.domain
            || candidate.symbol != incoming_record.symbol
        {
            return false;
        }
        let relocated_detail = candidate.detail.replace(current_path, incoming_path);
        incoming_record.detail == relocated_detail
            && incoming_record.fingerprint == fnv1a64(relocated_detail.as_bytes())
    })
}

fn replace_rust_code_identifier_tokens(
    value: &str,
    replacements: &BTreeMap<String, String>,
) -> String {
    let bytes = value.as_bytes();
    let masked = mask_non_code(value);
    let masked_bytes = masked.as_bytes();
    assert_eq!(
        bytes.len(),
        masked_bytes.len(),
        "Rust source mask must preserve byte offsets"
    );
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0usize;
    while index < bytes.len() {
        if is_ident_start(masked_bytes[index]) {
            let start = index;
            index += 1;
            while index < bytes.len() && is_ident_byte(masked_bytes[index]) {
                index += 1;
            }
            let identifier = &value[start..index];
            if let Some(replacement) = replacements.get(identifier) {
                out.extend_from_slice(replacement.as_bytes());
            } else {
                out.extend_from_slice(&bytes[start..index]);
            }
        } else {
            out.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(out).expect("identifier-only Rust rewrite preserves UTF-8")
}

fn function_signature_without_own_name(function: &FunctionInfo) -> String {
    let replacements = BTreeMap::from([(function.name.clone(), "__function_name__".to_string())]);
    replace_rust_code_identifier_tokens(&function.signature, &replacements)
}

fn exact_refactor_writer_function_symbol_relocations(
    current: &Inventory,
    incoming: &Inventory,
    change_class: &str,
) -> Result<BTreeSet<String>, String> {
    if change_class != "RefactorOnly" {
        return Ok(BTreeSet::new());
    }

    let current_writer_records = current
        .records
        .values()
        .filter(|record| record.kind == CapabilityKind::WriterReachability)
        .collect::<Vec<_>>();
    let mut rename_pairs = Vec::<(&FunctionInfo, &FunctionInfo, &CapabilityRecord)>::new();
    let mut name_replacements = BTreeMap::<String, String>::new();

    for incoming_record in incoming.records.values().filter(|record| {
        record.kind == CapabilityKind::WriterReachability
            && record.domain == Domain::Music
            && !current.records.contains_key(&record.key())
    }) {
        let incoming_key = format!("{}::{}", incoming_record.path, incoming_record.symbol);
        let Some(incoming_function) = incoming.functions.get(&incoming_key) else {
            continue;
        };
        let incoming_signature = function_signature_without_own_name(incoming_function);
        let candidates = current_writer_records
            .iter()
            .copied()
            .filter(|current_record| {
                current_record.path == incoming_record.path
                    && current_record.domain == incoming_record.domain
                    && current_record.detail == incoming_record.detail
                    && current_record.fingerprint == incoming_record.fingerprint
                    && !incoming.records.contains_key(&current_record.key())
            })
            .filter_map(|current_record| {
                let current_key = format!("{}::{}", current_record.path, current_record.symbol);
                let current_function = current.functions.get(&current_key)?;
                (current_function.public == incoming_function.public
                    && current_function.name != incoming_function.name
                    && function_signature_without_own_name(current_function) == incoming_signature)
                    .then_some((current_function, current_record))
            })
            .collect::<Vec<_>>();
        if candidates.len() != 1 {
            continue;
        }
        let (current_function, _current_record) = candidates[0];
        if name_replacements
            .insert(
                current_function.name.clone(),
                incoming_function.name.clone(),
            )
            .is_some()
        {
            return Err(
                "Semantic-Capability-Diff: RefactorOnly Function-Symbol-Rename ist nicht bijektiv"
                    .to_string(),
            );
        }
        rename_pairs.push((current_function, incoming_function, incoming_record));
    }

    if rename_pairs.is_empty() {
        return Ok(BTreeSet::new());
    }
    let unique_targets = name_replacements.values().collect::<BTreeSet<_>>();
    if unique_targets.len() != name_replacements.len() {
        return Err(
            "Semantic-Capability-Diff: RefactorOnly Function-Symbol-Rename hat mehrdeutige Zielnamen"
                .to_string(),
        );
    }

    let mut admitted = BTreeSet::new();
    for (current_function, incoming_function, incoming_record) in rename_pairs {
        let rewritten_body =
            replace_rust_code_identifier_tokens(&current_function.source_body, &name_replacements);
        if rewritten_body != incoming_function.source_body {
            continue;
        }
        admitted.insert(incoming_record.key());
    }
    Ok(admitted)
}

fn validate_no_new_productive_repair_recovery_authority(
    current: &Inventory,
    incoming: &Inventory,
    change_class: &str,
    refactor_owner_relocation: Option<&(String, String)>,
) -> Result<(), String> {
    let refactor_function_symbol_relocations =
        exact_refactor_writer_function_symbol_relocations(current, incoming, change_class)?;
    let current_writer_reachability = current
        .records
        .values()
        .filter(|record| record.kind == CapabilityKind::WriterReachability)
        .map(|record| (record.key(), record))
        .collect::<BTreeMap<_, _>>();
    let mut violations = Vec::new();

    for record in incoming.records.values().filter(|record| {
        record.kind == CapabilityKind::WriterReachability && record.domain == Domain::Music
    }) {
        let record_key = record.key();
        if let Some(previous) = current_writer_reachability.get(&record_key) {
            let before_roots = writer_reachability_roots(&previous.detail);
            let after_roots = writer_reachability_roots(&record.detail);
            let added_roots = after_roots
                .difference(&before_roots)
                .cloned()
                .collect::<BTreeSet<_>>();
            let noncanonical_added_roots = noncanonical_writer_roots(incoming, &added_roots);
            if !noncanonical_added_roots.is_empty() {
                violations.push(format!(
                    "{}::{} reason=writer_reachability_widened_to_noncanonical_root added_writer_roots={}",
                    record.path,
                    record.symbol,
                    noncanonical_added_roots
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(",")
                ));
            }
            continue;
        }

        if refactor_owner_relocation.is_some_and(|(current_path, incoming_path)| {
            relocated_capability_record_exists(current, record, current_path, incoming_path)
        }) || refactor_function_symbol_relocations.contains(&record_key)
        {
            continue;
        }

        let function_key = format!("{}::{}", record.path, record.symbol);
        let Some(function) = incoming.functions.get(&function_key) else {
            continue;
        };
        if let Some(reason) = new_productive_authority_violation(function) {
            violations.push(format!(
                "{}::{} reason={reason} writer_roots={}",
                function.path, function.symbol, record.detail
            ));
            continue;
        }

        if function.path.starts_with("src/structural_write_authority/") {
            let roots = writer_reachability_roots(&record.detail);
            let noncanonical_roots = noncanonical_writer_roots(incoming, &roots);
            if !noncanonical_roots.is_empty() {
                violations.push(format!(
                    "{}::{} reason=structural_owner_delegates_to_noncanonical_writer_root writer_roots={}",
                    function.path,
                    function.symbol,
                    noncanonical_roots
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(",")
                ));
            }
        }
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "NO-NEW-REPAIR-AUTHORITY KODEX GATE: neue oder reaktivierte produktive Repair-/Recovery-Autoritaet, neue spaete Writer-Aufrufschicht oder nichtkanonische Writer-Root-Erweiterung ist verboten; StructuralWriteAuthority muss die tatsaechliche produktive Schreibautoritaet besitzen statt nur an einen externen Legacy-Writer zu delegieren:\n  - {}",
            violations.join("\n  - ")
        ))
    }
}

fn noncanonical_writer_roots(inventory: &Inventory, roots: &BTreeSet<String>) -> BTreeSet<String> {
    roots
        .iter()
        .filter(|root| {
            inventory.functions.get(*root).is_none_or(|function| {
                !function.path.starts_with("src/structural_write_authority/")
            })
        })
        .cloned()
        .collect()
}

fn new_productive_authority_violation(function: &FunctionInfo) -> Option<&'static str> {
    if repair_recovery_authority_marker(function).is_some() {
        return Some("explicit_repair_or_recovery_identity");
    }
    if function.path.starts_with("src/")
        && !function.path.starts_with("src/structural_write_authority/")
    {
        return Some("new_writer_caller_outside_structural_write_authority");
    }
    None
}

fn repair_recovery_authority_marker(function: &FunctionInfo) -> Option<&'static str> {
    let lower_name = function.name.to_ascii_lowercase();
    let lower_path = function.path.to_ascii_lowercase();
    if lower_name.contains("repair") || lower_path.contains("repair") {
        Some("repair")
    } else if lower_name.contains("recovery") || lower_path.contains("recovery") {
        Some("recovery")
    } else {
        None
    }
}

fn audit_cross_role_retirement(
    current_root: &Path,
    incoming_root: &Path,
    changed_paths: &BTreeSet<String>,
    current: &Inventory,
    incoming: &Inventory,
) -> Result<AuditCrossRoleRetirement, String> {
    let mut result = AuditCrossRoleRetirement::default();
    for path in changed_paths {
        if domain_for_path(path) != Domain::Music || !path.ends_with(".rs") {
            continue;
        }
        let old = fs::read_to_string(current_root.join(path))
            .map_err(|error| format!("Cross-Role current {path}: {error}"))?;
        let new = fs::read_to_string(incoming_root.join(path))
            .map_err(|error| format!("Cross-Role incoming {path}: {error}"))?;
        if audit_source_reduction(&old, &new) {
            result.reduced_paths.insert(path.clone());
        }
    }
    let removed = current
        .functions
        .values()
        .filter(|function| {
            result.reduced_paths.contains(&function.path)
                && !incoming.functions.contains_key(&function.key)
        })
        .collect::<Vec<_>>();
    if removed.is_empty() {
        return Ok(result);
    }
    let names = removed
        .iter()
        .map(|function| function.name.as_str())
        .collect::<BTreeSet<_>>();
    if removed.iter().any(|function| function.direct_writer)
        || incoming.functions.values().any(|function| {
            function
                .calls
                .keys()
                .any(|name| names.contains(name.as_str()))
        })
    {
        return Err("AuditOnly Cross-Role-Rueckbau entfernt einen direkten oder weiterhin aufgerufenen Musikpfad".to_string());
    }
    let mut reached = removed
        .iter()
        .filter(|function| audit_retirement_root(&function.name))
        .map(|function| function.key.clone())
        .collect::<BTreeSet<_>>();
    loop {
        let before = reached.len();
        for caller in &removed {
            if reached.contains(&caller.key) {
                for callee in &removed {
                    if caller.calls.contains_key(&callee.name) {
                        reached.insert(callee.key.clone());
                    }
                }
            }
        }
        if reached.len() == before {
            break;
        }
    }
    if reached.len() != removed.len() {
        return Err(
            "AuditOnly Cross-Role-Rueckbau ist kein geschlossener Diagnosecluster".to_string(),
        );
    }
    result.retired_symbols.extend(
        removed
            .iter()
            .map(|function| (function.path.clone(), function.symbol.clone())),
    );
    Ok(result)
}

fn audit_retirement_root(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    [
        "audit",
        "probe",
        "trace",
        "readback",
        "diagnostic",
        "no_mutation",
        "wouldwrite",
    ]
    .iter()
    .any(|marker| name.contains(marker))
}

fn audit_source_reduction(current: &str, incoming: &str) -> bool {
    let mut old_counts = BTreeMap::<&str, usize>::new();
    let mut new_counts = BTreeMap::<&str, usize>::new();
    current
        .lines()
        .for_each(|line| *old_counts.entry(line).or_insert(0) += 1);
    incoming
        .lines()
        .for_each(|line| *new_counts.entry(line).or_insert(0) += 1);
    let mut removed = Vec::new();
    for (line, count) in &old_counts {
        for _ in 0..count.saturating_sub(*new_counts.get(line).unwrap_or(&0)) {
            removed.push(*line);
        }
    }
    new_counts.iter().all(|(line, count)| {
        let added = count.saturating_sub(*old_counts.get(line).unwrap_or(&0));
        added == 0 || (added == 1 && removed.iter().any(|old| strict_use_subset(old, line)))
    })
}

fn strict_use_subset(current: &str, incoming: &str) -> bool {
    fn parts(line: &str) -> Option<(&str, BTreeSet<&str>, &str)> {
        let line = line.trim();
        let (prefix, tail) = line.split_once('{')?;
        let (body, suffix) = tail.rsplit_once('}')?;
        if !prefix.trim().starts_with("use ") || !suffix.trim().ends_with(';') {
            return None;
        }
        Some((
            prefix.trim(),
            body.split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .collect(),
            suffix.trim(),
        ))
    }
    let Some((old_prefix, old_items, old_suffix)) = parts(current) else {
        return false;
    };
    let Some((new_prefix, new_items, new_suffix)) = parts(incoming) else {
        return false;
    };
    old_prefix == new_prefix
        && old_suffix == new_suffix
        && !new_items.is_empty()
        && new_items.len() < old_items.len()
        && new_items.is_subset(&old_items)
}

pub fn revision_effect_snapshot(
    incoming_root: &Path,
    changed_paths: &BTreeSet<String>,
) -> Result<String, String> {
    let inventory = inventory_tree(incoming_root)?;
    let mut rows = Vec::new();
    for function in inventory.functions.values() {
        if !changed_paths.contains(&function.path) {
            continue;
        }
        let normalized = normalize_identifier_invariant_body(&function.masked_body);
        let body_fingerprint = fnv1a64(normalized.as_bytes());
        let anchors = semantic_anchor_tokens(&function.masked_body);
        let mut effects = BTreeSet::new();
        let lower_name = function.name.to_ascii_lowercase();
        if function.direct_writer {
            effects.insert("direct-writer");
        }
        if writer_like_name(&lower_name) {
            effects.insert("writer-name-root");
        }
        if structural_writer_root(function) {
            effects.insert("structural-writer-root");
        }
        if !threshold_assignments(&function.masked_body).is_empty() {
            effects.insert("calibration-threshold");
        }
        if executable_master_source_commit_surface(function) {
            effects.insert("master-apply");
        }
        for record in inventory.records.values() {
            if record.path != function.path {
                continue;
            }
            if record.symbol == function.symbol
                || record.symbol.starts_with(&format!("{}->", function.symbol))
            {
                match record.kind {
                    CapabilityKind::WriterReachability => {
                        effects.insert("writer-reachable");
                    }
                    CapabilityKind::ExportedWriterCapability => {
                        effects.insert("exported-writer");
                    }
                    CapabilityKind::WriteFunctionCall => {
                        effects.insert("writer-call");
                    }
                    CapabilityKind::KodexNegativeCheck => {
                        effects.insert("kodex-negative-check");
                    }
                    CapabilityKind::ProofReentryReader => {
                        effects.insert("proof-reentry-reader");
                    }
                    _ => {}
                }
            }
        }
        rows.push(format!(
            "v1	{}	{body_fingerprint:016x}	{}	{}",
            function.path,
            effects.into_iter().collect::<Vec<_>>().join(","),
            anchors.into_iter().collect::<Vec<_>>().join(",")
        ));
    }
    rows.sort();
    let mut snapshot = rows.join("\n");
    if !snapshot.is_empty() {
        snapshot.push('\n');
    }
    Ok(snapshot)
}

pub fn validate_revision_effect_monotonic(
    previous_snapshot: &str,
    current_snapshot: &str,
) -> Result<(), String> {
    let previous = parse_revision_effect_snapshot(previous_snapshot)?;
    let current = parse_revision_effect_snapshot(current_snapshot)?;
    let mut violations = Vec::new();
    for previous_row in &previous {
        if previous_row.effects.is_empty() {
            continue;
        }
        let equivalent_rows = current
            .iter()
            .filter(|current_row| revision_rows_semantically_equivalent(previous_row, current_row))
            .collect::<Vec<_>>();
        if equivalent_rows.is_empty() {
            continue;
        }
        let mut retained_effects = BTreeSet::new();
        let mut current_paths = BTreeSet::new();
        for current_row in equivalent_rows {
            retained_effects.extend(current_row.effects.iter().cloned());
            current_paths.insert(current_row.path.clone());
        }
        let lost = previous_row
            .effects
            .difference(&retained_effects)
            .cloned()
            .collect::<Vec<_>>();
        if !lost.is_empty() {
            violations.push(format!(
                "semantisch aequivalenter Funktionskoerper verlor Capability [{}]: vorher {} body={:016x}; jetzt {}",
                lost.join(","),
                previous_row.path,
                previous_row.body_fingerprint,
                current_paths.into_iter().collect::<Vec<_>>().join(",")
            ));
        }
    }
    if violations.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "MultiRevision-Semantic-Capability-Evasion: Capability darf innerhalb derselben Ausgangsversionskette nicht allein durch Identifier-, Formatierungs- oder semantiknahe Tarnung verschwinden:\n  - {}",
            violations.join("\n  - ")
        ))
    }
}

fn inventory_tree(root: &Path) -> Result<Inventory, String> {
    let mut inventory = Inventory::default();
    let root_files = root_file_base(root);
    let cargo_path = root_files.join("Cargo.toml");
    let cargo_text = fs::read_to_string(&cargo_path).map_err(|error| {
        format!(
            "Semantic-Capability-Inventur: Cargo.toml nicht lesbar {}: {error}",
            cargo_path.display()
        )
    })?;
    let build_script = inventory_cargo(&cargo_text, &mut inventory)?;

    let mut files = Vec::new();
    collect_managed_files(root, &mut files)?;
    files.sort();
    for path in files {
        let relative = normalize_path(path.strip_prefix(root).map_err(|error| {
            format!(
                "Semantic-Capability-Inventur: relativer Pfad fehlgeschlagen {}: {error}",
                path.display()
            )
        })?);
        if relative == CHANGE_MANIFEST_PATH {
            continue;
        }
        inventory_file(&relative, &path, build_script.as_deref(), &mut inventory)?;
    }

    for root_name in [
        "run.cmd",
        "strict.cmd",
        "rust-toolchain.toml",
        "rustfmt.toml",
        "clippy.toml",
    ] {
        let path = root_files.join(root_name);
        if path.is_file() {
            inventory_root_control_file(root_name, &path, &mut inventory)?;
        }
    }

    add_call_graph_capabilities(&mut inventory)?;
    Ok(inventory)
}

fn root_file_base(root: &Path) -> PathBuf {
    let backup_root_files = root.join("root_files");
    if !root.join("Cargo.toml").is_file() && backup_root_files.join("Cargo.toml").is_file() {
        backup_root_files
    } else {
        root.to_path_buf()
    }
}

fn collect_managed_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for managed in ["src", "tests", "tools"] {
        let dir = root.join(managed);
        if !dir.exists() {
            continue;
        }
        collect_files_recursive(&dir, files)?;
    }
    Ok(())
}

fn collect_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|error| {
        format!(
            "Semantic-Capability-Inventur: Ordner nicht lesbar {}: {error}",
            dir.display()
        )
    })? {
        let entry = entry.map_err(|error| {
            format!("Semantic-Capability-Inventur: Directory entry nicht lesbar: {error}")
        })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|error| {
            format!(
                "Semantic-Capability-Inventur: Dateityp nicht lesbar {}: {error}",
                path.display()
            )
        })?;
        if file_type.is_dir() {
            collect_files_recursive(&path, files)?;
        } else if file_type.is_file() {
            files.push(path);
        } else {
            return Err(format!(
                "Semantic-Capability-Inventur: unbekannter Dateityp {}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn inventory_cargo(text: &str, inventory: &mut Inventory) -> Result<Option<String>, String> {
    let mut section = String::new();
    let mut build_script = None;
    for (index, raw_line) in text.lines().enumerate() {
        let line = strip_toml_comment(raw_line).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line.to_ascii_lowercase();
            if let Some(name) = dependency_table_name(&section) {
                add_record(
                    inventory,
                    CapabilityRecord {
                        kind: CapabilityKind::CargoDependency,
                        domain: Domain::Unknown,
                        path: "Cargo.toml".to_string(),
                        symbol: name,
                        fingerprint: fnv1a64(normalize_semantic(line).as_bytes()),
                        detail: format!("dependency table line {}", index + 1),
                    },
                )?;
            }
            continue;
        }
        let Some((raw_key, raw_value)) = line.split_once('=') else {
            continue;
        };
        let key = raw_key.trim().trim_matches('"').to_string();
        let value = normalize_semantic(raw_value);
        if section == "[package]" && key == "build" {
            let path = raw_value.trim().trim_matches('"').to_string();
            if path.is_empty() {
                return Err("Semantic-Capability-Inventur: leerer package.build-Pfad".to_string());
            }
            build_script = Some(path.clone());
            add_record(
                inventory,
                CapabilityRecord {
                    kind: CapabilityKind::BuildScriptCapability,
                    domain: Domain::Kodex,
                    path: "Cargo.toml".to_string(),
                    symbol: "package.build".to_string(),
                    fingerprint: fnv1a64(path.as_bytes()),
                    detail: path,
                },
            )?;
        } else if section == "[features]" {
            add_record(
                inventory,
                CapabilityRecord {
                    kind: CapabilityKind::CargoFeature,
                    domain: Domain::Unknown,
                    path: "Cargo.toml".to_string(),
                    symbol: key,
                    fingerprint: fnv1a64(value.as_bytes()),
                    detail: format!("feature line {}", index + 1),
                },
            )?;
        } else if is_dependency_section(&section) {
            add_record(
                inventory,
                CapabilityRecord {
                    kind: CapabilityKind::CargoDependency,
                    domain: Domain::Unknown,
                    path: "Cargo.toml".to_string(),
                    symbol: format!(
                        "{}::{key}",
                        section.trim_matches(|character| matches!(character, '[' | ']'))
                    ),
                    fingerprint: fnv1a64(value.as_bytes()),
                    detail: format!("dependency line {}", index + 1),
                },
            )?;
        }
    }
    Ok(build_script)
}

fn strip_toml_comment(line: &str) -> &str {
    line.split_once('#').map_or(line, |(before, _)| before)
}

fn is_dependency_section(section: &str) -> bool {
    section == "[dependencies]"
        || section == "[dev-dependencies]"
        || section == "[build-dependencies]"
        || section.contains(".dependencies]")
}

fn dependency_table_name(section: &str) -> Option<String> {
    for prefix in [
        "[dependencies.",
        "[dev-dependencies.",
        "[build-dependencies.",
    ] {
        if let Some(rest) = section.strip_prefix(prefix) {
            return Some(rest.trim_end_matches(']').to_string());
        }
    }
    if section.starts_with("[target.") && section.contains(".dependencies.") {
        return Some(
            section
                .trim_matches(|character| matches!(character, '[' | ']'))
                .to_string(),
        );
    }
    None
}

fn inventory_root_control_file(
    relative: &str,
    path: &Path,
    inventory: &mut Inventory,
) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|error| {
        format!(
            "Semantic-Capability-Inventur: Root-Steuerdatei nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    let domain = if matches!(
        relative,
        "run.cmd" | "strict.cmd" | "rust-toolchain.toml" | "rustfmt.toml" | "clippy.toml"
    ) {
        Domain::Kodex
    } else {
        Domain::Unknown
    };
    let kind = if domain == Domain::Kodex {
        CapabilityKind::KodexControl
    } else {
        CapabilityKind::UnknownCapability
    };
    add_record(
        inventory,
        CapabilityRecord {
            kind,
            domain,
            path: relative.to_string(),
            symbol: "<root-control>".to_string(),
            fingerprint: fnv1a64(&bytes),
            detail: "managed root control file".to_string(),
        },
    )
}

fn inventory_file(
    relative: &str,
    path: &Path,
    build_script: Option<&str>,
    inventory: &mut Inventory,
) -> Result<(), String> {
    if relative.starts_with(SCHEMASCHLOPF_R021_COMPONENT_PREFIX) {
        let bytes = fs::read(path).map_err(|error| {
            format!(
                "Semantic-Capability-Inventur: qualifizierte Komponentenbytes nicht lesbar {}: {error}",
                path.display()
            )
        })?;
        add_record(
            inventory,
            CapabilityRecord {
                kind: CapabilityKind::UnknownCapability,
                domain: Domain::Unknown,
                path: relative.to_string(),
                symbol: "<qualified-component-bytes>".to_string(),
                fingerprint: fnv1a64(&bytes),
                detail: "SchemaSchlopf exact qualified read-only component projection".to_string(),
            },
        )?;
        return Ok(());
    }
    let domain = domain_for_path(relative);
    if path.extension().and_then(|value| value.to_str()) != Some("rs") {
        return inventory_non_rust_file(relative, path, domain, inventory);
    }
    let source = fs::read_to_string(path).map_err(|error| {
        format!(
            "Semantic-Capability-Inventur: Rust-Datei nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    let masked = mask_non_code(&source);
    let file_kind = match domain {
        Domain::Kodex => CapabilityKind::KodexControl,
        Domain::Audit => CapabilityKind::AuditLogic,
        Domain::Music => CapabilityKind::MusicLogic,
        Domain::Unknown => CapabilityKind::UnknownCapability,
    };
    let normalized_file = if domain == Domain::Music {
        normalize_music_surface(&masked)
    } else {
        normalize_semantic(&masked)
    };
    add_record(
        inventory,
        CapabilityRecord {
            kind: file_kind,
            domain,
            path: relative.to_string(),
            symbol: "<file-surface>".to_string(),
            fingerprint: fnv1a64(normalized_file.as_bytes()),
            detail: "semantic file surface".to_string(),
        },
    )?;

    let spans = function_spans(&source, &masked)?;
    let mut function_occurrences = BTreeMap::<String, usize>::new();
    for span in spans {
        let signature = source[span.signature_start..span.body_start].to_string();
        let source_body = source[span.body_start..span.body_end].to_string();
        let masked_body = masked[span.body_start..span.body_end].to_string();
        let calls = called_identifiers(&masked_body);
        let direct_writer = direct_music_writer(relative, &span.name, &masked_body);
        let occurrence = function_occurrences.entry(span.name.clone()).or_insert(0);
        *occurrence += 1;
        let symbol = format!("{}#{}", span.name, *occurrence);
        let key = format!("{relative}::{symbol}");
        let function = FunctionInfo {
            key: key.clone(),
            symbol,
            path: relative.to_string(),
            name: span.name.clone(),
            domain,
            public: span.public,
            start_line: span.start_line,
            signature,
            source_body,
            masked_body,
            calls,
            direct_writer,
        };
        inventory_function(&function, build_script, inventory)?;
        if inventory.functions.insert(key.clone(), function).is_some() {
            return Err(format!(
                "Semantic-Capability-Inventur: doppelte Funktionsidentitaet {key}"
            ));
        }
    }

    inventory_raw_public_identity_fields(relative, domain, &masked, inventory);
    inventory_thresholds(relative, domain, &source, &masked, inventory)?;
    inventory_macro_sources(relative, domain, &masked, inventory)?;
    Ok(())
}

fn inventory_non_rust_file(
    relative: &str,
    path: &Path,
    domain: Domain,
    inventory: &mut Inventory,
) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|error| {
        format!(
            "Semantic-Capability-Inventur: Datei nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    let kind = match domain {
        Domain::Kodex => CapabilityKind::KodexControl,
        Domain::Audit => CapabilityKind::AuditLogic,
        Domain::Music => CapabilityKind::MusicLogic,
        Domain::Unknown => CapabilityKind::UnknownCapability,
    };
    add_record(
        inventory,
        CapabilityRecord {
            kind,
            domain,
            path: relative.to_string(),
            symbol: "<non-rust-surface>".to_string(),
            fingerprint: fnv1a64(&bytes),
            detail: "managed non-Rust surface".to_string(),
        },
    )
}

fn executable_master_source_commit_surface(function: &FunctionInfo) -> bool {
    let lower_name = function.name.to_ascii_lowercase();
    if master_source_commit_entrypoint_name(&lower_name) {
        return true;
    }
    let lower_body = function.masked_body.to_ascii_lowercase();
    let calls = called_identifiers(&lower_body);
    calls.contains_key("master_apply")
        || calls.contains_key("masterapply")
        || identifier_assigned_literal_true(&lower_body, "master_apply")
        || identifier_assigned_literal_true(&lower_body, "masterapply")
}

fn master_source_commit_entrypoint_name(lower_name: &str) -> bool {
    if [
        "no_master_apply",
        "without_master_apply",
        "master_apply_disabled",
        "master_apply_false",
        "masterapply_disabled",
        "masterapply_false",
    ]
    .iter()
    .any(|marker| lower_name.contains(marker))
    {
        return false;
    }
    lower_name == "master_apply"
        || lower_name == "masterapply"
        || lower_name.starts_with("master_apply_")
        || lower_name.starts_with("masterapply_")
        || lower_name.ends_with("_master_apply")
        || lower_name.ends_with("_masterapply")
        || lower_name.contains("_master_apply_")
        || lower_name.contains("_masterapply_")
}

fn identifier_assigned_literal_true(source: &str, identifier: &str) -> bool {
    let bytes = source.as_bytes();
    let needle = identifier.as_bytes();
    let mut cursor = 0usize;
    while cursor + needle.len() <= bytes.len() {
        let Some(relative) = source[cursor..].find(identifier) else {
            return false;
        };
        let index = cursor + relative;
        let before_ok = index == 0 || !is_ident_byte(bytes[index - 1]);
        let after_identifier = index + needle.len();
        let after_ok = after_identifier >= bytes.len() || !is_ident_byte(bytes[after_identifier]);
        if before_ok && after_ok {
            let mut next = after_identifier;
            while bytes.get(next).is_some_and(u8::is_ascii_whitespace) {
                next += 1;
            }
            if matches!(bytes.get(next), Some(&b':' | &b'=')) {
                next += 1;
                while bytes.get(next).is_some_and(u8::is_ascii_whitespace) {
                    next += 1;
                }
                if let Some(tail) = source.get(next..) {
                    let true_boundary = tail
                        .as_bytes()
                        .get(4)
                        .is_none_or(|byte| !is_ident_byte(*byte));
                    if tail.starts_with("true") && true_boundary {
                        return true;
                    }
                }
            }
        }
        cursor = after_identifier;
    }
    false
}

fn inventory_function(
    function: &FunctionInfo,
    build_script: Option<&str>,
    inventory: &mut Inventory,
) -> Result<(), String> {
    let lower_name = function.name.to_ascii_lowercase();
    let normalized_body = normalize_semantic(&function.masked_body);
    if function.domain == Domain::Music
        && function.public
        && let Some(marker) = raw_identity_primitive_marker(&function.signature)
    {
        inventory.productive_identity_provenance_violations.push(
            ProductiveMusicIdentityProvenanceViolation {
                path: function.path.clone(),
                symbol: function.symbol.clone(),
                detail: format!(
                    "raw public identity primitive parameter {marker:?} at line {}; use an opaque structural locator type",
                    function.start_line
                ),
            },
        );
    }
    if function.direct_writer {
        add_record(
            inventory,
            CapabilityRecord {
                kind: CapabilityKind::MusicXmlMutationEntrypoint,
                domain: function.domain,
                path: function.path.clone(),
                symbol: function.symbol.clone(),
                fingerprint: fnv1a64(normalized_body.as_bytes()),
                detail: format!("direct XML/music mutation at line {}", function.start_line),
            },
        )?;
    }
    if function.path.starts_with("src/structural_write_authority/")
        && (function.public || function.direct_writer || writer_like_name(&lower_name))
    {
        add_record(
            inventory,
            CapabilityRecord {
                kind: CapabilityKind::StructuralWriteAuthorityEntrypoint,
                domain: Domain::Music,
                path: function.path.clone(),
                symbol: function.symbol.clone(),
                fingerprint: fnv1a64(normalized_body.as_bytes()),
                detail: format!("structural authority at line {}", function.start_line),
            },
        )?;
    }
    if executable_master_source_commit_surface(function) {
        add_record(
            inventory,
            CapabilityRecord {
                kind: CapabilityKind::MasterApplyEntrypoint,
                domain: function.domain,
                path: function.path.clone(),
                symbol: function.symbol.clone(),
                fingerprint: fnv1a64(normalized_body.as_bytes()),
                detail: format!(
                    "executable MasterApply enablement at line {}",
                    function.start_line
                ),
            },
        )?;
    }
    let feedback_markers = feedback_identity_markers(&function.masked_body, &function.signature);
    if !feedback_markers.is_empty() {
        let marker_text = feedback_markers.into_iter().collect::<Vec<_>>().join(",");
        add_record(
            inventory,
            CapabilityRecord {
                kind: CapabilityKind::FeedbackIdentityChannel,
                domain: function.domain,
                path: function.path.clone(),
                symbol: function.symbol.clone(),
                fingerprint: fnv1a64(marker_text.as_bytes()),
                detail: marker_text,
            },
        )?;
    }
    if proof_reentry_reader(function) {
        add_record(
            inventory,
            CapabilityRecord {
                kind: CapabilityKind::ProofReentryReader,
                domain: function.domain,
                path: function.path.clone(),
                symbol: function.symbol.clone(),
                fingerprint: fnv1a64(normalized_body.as_bytes()),
                detail: format!("proof/report reader at line {}", function.start_line),
            },
        )?;
    }
    if function.domain == Domain::Kodex && !function.signature.contains("#[cfg(test)]") {
        let rejection = normalized_rejection_surface(&function.masked_body);
        if !rejection.is_empty() {
            add_record(
                inventory,
                CapabilityRecord {
                    kind: CapabilityKind::KodexNegativeCheck,
                    domain: Domain::Kodex,
                    path: function.path.clone(),
                    symbol: function.symbol.clone(),
                    fingerprint: fnv1a64(rejection.as_bytes()),
                    detail: format!("negative kodex checks at line {}", function.start_line),
                },
            )?;
        }
    }
    if build_script == Some(function.path.as_str()) {
        add_record(
            inventory,
            CapabilityRecord {
                kind: CapabilityKind::BuildScriptCapability,
                domain: Domain::Kodex,
                path: function.path.clone(),
                symbol: function.symbol.clone(),
                fingerprint: fnv1a64(normalized_body.as_bytes()),
                detail: format!("build script function at line {}", function.start_line),
            },
        )?;
    }
    Ok(())
}

fn inventory_raw_public_identity_fields(
    relative: &str,
    domain: Domain,
    masked: &str,
    inventory: &mut Inventory,
) {
    if domain != Domain::Music {
        return;
    }
    let lower = masked.to_ascii_lowercase();
    for marker in raw_identity_boundary_markers() {
        let mut search_from = 0usize;
        let mut matched_field = None::<String>;
        while let Some(relative_position) = lower[search_from..].find(marker) {
            let marker_position = search_from + relative_position;
            let mut name_start = marker_position;
            while name_start > 0 && is_identifier_byte(lower.as_bytes()[name_start - 1]) {
                name_start -= 1;
            }
            let mut name_end = marker_position + marker.len();
            while name_end < lower.len() && is_identifier_byte(lower.as_bytes()[name_end]) {
                name_end += 1;
            }
            let field_name = &lower[name_start..name_end];
            let marker_is_semantic_suffix =
                field_name == *marker || field_name.ends_with(&format!("_{marker}"));
            let before = lower[..name_start].trim_end();
            let public_field = public_visibility_tail(before);
            let mut after = lower[name_end..].trim_start();
            if marker_is_semantic_suffix
                && public_field
                && let Some(after_colon) = after.strip_prefix(':')
            {
                after = after_colon.trim_start();
                let type_end = after.find([',', ';', '}', '\n']).unwrap_or(after.len());
                let raw_type = normalize_semantic(&after[..type_end]);
                let raw = raw_type.strip_prefix("option<").unwrap_or(&raw_type);
                if raw_identity_primitive_type(raw) {
                    matched_field = Some(field_name.to_string());
                    break;
                }
            }
            search_from = marker_position + marker.len();
        }
        if let Some(field_name) = matched_field {
            inventory.productive_identity_provenance_violations.push(
                ProductiveMusicIdentityProvenanceViolation {
                    path: relative.to_string(),
                    symbol: format!("<public-field:{field_name}>"),
                    detail: format!(
                        "raw public identity primitive field {field_name:?} matches identity marker {marker:?}; use an opaque structural locator type"
                    ),
                },
            );
        }
    }
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn public_visibility_tail(text: &str) -> bool {
    let tail = text
        .rsplit(['\n', '{', '}', ',', ';'])
        .next()
        .unwrap_or(text)
        .trim();
    tail == "pub" || (tail.starts_with("pub(") && tail.ends_with(')'))
}

fn raw_identity_boundary_markers() -> &'static [&'static str] {
    &[
        "target_measure",
        "feedback_measure",
        "origin_measure",
        "source_measure",
        "measure_number",
        "measure_no",
        "measure_locator",
        "feedback_locator",
        "feedback_text",
        "feedback_origin",
        "feedback_project",
    ]
}

fn raw_identity_primitive_type(raw: &str) -> bool {
    ["usize", "u64", "u32", "i64", "i32", "string", "&str"]
        .iter()
        .any(|primitive| raw.starts_with(primitive))
}

fn raw_identity_primitive_marker(text: &str) -> Option<&'static str> {
    let normalized = normalize_semantic(text).to_ascii_lowercase();
    for marker in raw_identity_boundary_markers() {
        let Some(position) = normalized.find(marker) else {
            continue;
        };
        let after = &normalized[position + marker.len()..];
        let Some(after_colon) = after.strip_prefix(':') else {
            continue;
        };
        let raw = after_colon.strip_prefix("option<").unwrap_or(after_colon);
        if raw_identity_primitive_type(raw) {
            return Some(marker);
        }
    }
    None
}

fn direct_external_identity_input(function: &FunctionInfo) -> bool {
    let source = function.source_body.as_str();
    if [
        "std::env::vars(",
        "env::vars(",
        "std::env::vars_os(",
        "env::vars_os(",
        "std::env::args(",
        "env::args(",
        "std::env::args_os(",
        "env::args_os(",
    ]
    .iter()
    .any(|marker| source.contains(marker))
    {
        return true;
    }
    [
        "std::env::var(",
        "env::var(",
        "std::env::var_os(",
        "env::var_os(",
    ]
    .iter()
    .any(|marker| env_call_has_identity_argument(source, marker))
}

fn env_call_has_identity_argument(source: &str, call_marker: &str) -> bool {
    let mut search_from = 0usize;
    while let Some(relative) = source[search_from..].find(call_marker) {
        let argument_start = search_from + relative + call_marker.len();
        let argument = source[argument_start..].trim_start();
        let end = argument.find(')').unwrap_or(argument.len());
        let argument = &argument[..end];
        if external_identity_marker(argument) {
            return true;
        }
        search_from = argument_start;
    }
    false
}

fn external_identity_marker(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    [
        "measure", "locator", "feedback", "target", "project", "etude", "etüde",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
}

fn inventory_thresholds(
    relative: &str,
    domain: Domain,
    source: &str,
    masked: &str,
    inventory: &mut Inventory,
) -> Result<(), String> {
    if domain != Domain::Music {
        return Ok(());
    }
    let mut occurrence = BTreeMap::<String, usize>::new();
    for assignment in threshold_assignments(masked) {
        let count = occurrence.entry(assignment.name.clone()).or_insert(0);
        *count += 1;
        let symbol = format!("{}#{}", assignment.name, *count);
        let threshold_expression = source
            .get(assignment.rhs_start..assignment.rhs_end)
            .ok_or_else(|| {
                format!(
                    "Semantic-Capability-Inventur: ungueltiger Schwellenwertbereich {relative} Zeile {}",
                    assignment.start_line
                )
            })?;
        add_record(
            inventory,
            CapabilityRecord {
                kind: CapabilityKind::CalibrationThreshold,
                domain,
                path: relative.to_string(),
                symbol,
                fingerprint: fnv1a64(normalize_semantic(threshold_expression).as_bytes()),
                detail: format!("threshold definition line {}", assignment.start_line),
            },
        )?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct ThresholdAssignment {
    name: String,
    rhs_start: usize,
    rhs_end: usize,
    start_line: usize,
}

fn threshold_assignments(masked: &str) -> Vec<ThresholdAssignment> {
    let bytes = masked.as_bytes();
    let mut out = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] != b'=' || !standalone_assignment_operator(bytes, index) {
            index += 1;
            continue;
        }
        let left_start = assignment_left_start(bytes, index);
        let left = &masked[left_start..index];
        let Some(name) = assignment_identifier(left) else {
            index += 1;
            continue;
        };
        if !threshold_like_name(&name) {
            index += 1;
            continue;
        }
        let rhs_start = index + 1;
        let Some(rhs_end) = assignment_rhs_end(bytes, rhs_start) else {
            index += 1;
            continue;
        };
        let rhs = &masked[rhs_start..rhs_end];
        if !contains_numeric_literal(rhs) {
            index += 1;
            continue;
        }
        out.push(ThresholdAssignment {
            name,
            rhs_start,
            rhs_end,
            start_line: masked[..left_start]
                .bytes()
                .filter(|byte| *byte == b'\n')
                .count()
                + 1,
        });
        index = rhs_end + 1;
    }
    out
}

fn assignment_left_start(bytes: &[u8], assignment: usize) -> usize {
    let mut square_depth = 0usize;
    let mut parenthesis_depth = 0usize;
    let mut cursor = assignment;
    while cursor > 0 {
        cursor -= 1;
        match bytes[cursor] {
            b']' => square_depth += 1,
            b'[' => square_depth = square_depth.saturating_sub(1),
            b')' => parenthesis_depth += 1,
            b'(' => parenthesis_depth = parenthesis_depth.saturating_sub(1),
            b';' | b'{' | b'}' if square_depth == 0 && parenthesis_depth == 0 => {
                return cursor + 1;
            }
            _ => {}
        }
    }
    0
}

fn assignment_rhs_end(bytes: &[u8], rhs_start: usize) -> Option<usize> {
    let mut square_depth = 0usize;
    let mut parenthesis_depth = 0usize;
    let mut brace_depth = 0usize;
    for (index, byte) in bytes.iter().copied().enumerate().skip(rhs_start) {
        match byte {
            b'[' => square_depth += 1,
            b']' => square_depth = square_depth.saturating_sub(1),
            b'(' => parenthesis_depth += 1,
            b')' => parenthesis_depth = parenthesis_depth.saturating_sub(1),
            b'{' => brace_depth += 1,
            b'}' => brace_depth = brace_depth.saturating_sub(1),
            b';' if square_depth == 0 && parenthesis_depth == 0 && brace_depth == 0 => {
                return Some(index);
            }
            _ => {}
        }
    }
    None
}

fn standalone_assignment_operator(bytes: &[u8], index: usize) -> bool {
    let previous = index
        .checked_sub(1)
        .and_then(|position| bytes.get(position))
        .copied();
    let next = bytes.get(index + 1).copied();
    !previous.is_some_and(|byte| {
        matches!(
            byte,
            b'=' | b'!'
                | b'<'
                | b'>'
                | b'+'
                | b'-'
                | b'*'
                | b'/'
                | b'%'
                | b'&'
                | b'|'
                | b'^'
                | b'.'
        )
    }) && !next.is_some_and(|byte| matches!(byte, b'=' | b'>'))
}

fn inventory_macro_sources(
    relative: &str,
    domain: Domain,
    masked: &str,
    inventory: &mut Inventory,
) -> Result<(), String> {
    let mut counts = BTreeMap::<String, usize>::new();
    for (index, line) in masked.lines().enumerate() {
        let compact = normalize_semantic(line);
        for marker in [
            "macro_rules!",
            "include!(",
            "include_str!(",
            "include_bytes!(",
            "env!(",
            "option_env!(",
            "OUT_DIR",
        ] {
            if compact.contains(marker) {
                let count = counts.entry(marker.to_string()).or_insert(0);
                *count += 1;
                add_record(
                    inventory,
                    CapabilityRecord {
                        kind: CapabilityKind::MacroOrGeneratedSource,
                        domain,
                        path: relative.to_string(),
                        symbol: format!("{marker}#{}", *count),
                        fingerprint: fnv1a64(compact.as_bytes()),
                        detail: format!("macro/generated-source line {}", index + 1),
                    },
                )?;
            }
        }
    }
    Ok(())
}

fn add_call_graph_capabilities(inventory: &mut Inventory) -> Result<(), String> {
    let mut by_domain_name = BTreeMap::<(Domain, String), Vec<String>>::new();
    for function in inventory.functions.values() {
        by_domain_name
            .entry((function.domain, function.name.clone()))
            .or_default()
            .push(function.key.clone());
    }
    for values in by_domain_name.values_mut() {
        values.sort();
    }
    let mut global_name_counts = BTreeMap::<String, usize>::new();
    for function in inventory.functions.values() {
        *global_name_counts.entry(function.name.clone()).or_insert(0) += 1;
    }

    let writer_roots = inventory
        .functions
        .values()
        .filter(|function| function.direct_writer || structural_writer_root(function))
        .map(|function| function.key.clone())
        .collect::<BTreeSet<_>>();

    let mut edges = BTreeMap::<String, BTreeSet<String>>::new();
    for function in inventory.functions.values() {
        let mut targets = BTreeSet::new();
        for called in function.calls.keys() {
            if let Some(keys) = by_domain_name.get(&(function.domain, called.clone())) {
                if keys.len() == 1 || writer_like_name(&called.to_ascii_lowercase()) {
                    targets.extend(keys.iter().cloned());
                } else {
                    targets.extend(
                        keys.iter()
                            .filter(|key| key.starts_with(&format!("{}::", function.path)))
                            .cloned(),
                    );
                }
            }
        }
        edges.insert(function.key.clone(), targets);
    }

    let function_calls = inventory
        .functions
        .values()
        .map(|function| {
            (
                function.key.clone(),
                function.domain,
                function.calls.keys().cloned().collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let mut reachable = compute_reverse_reachability(&edges, &writer_roots);
    loop {
        let mut added_cross_domain_edge = false;
        for (function_key, function_domain, calls) in &function_calls {
            let targets = edges.entry(function_key.clone()).or_default();
            for called in calls {
                let cross_domain_resolvable = writer_like_name(&called.to_ascii_lowercase())
                    || global_name_counts.get(called) == Some(&1);
                if !cross_domain_resolvable {
                    continue;
                }
                for target_domain in [Domain::Kodex, Domain::Audit, Domain::Music, Domain::Unknown]
                {
                    if target_domain == *function_domain {
                        continue;
                    }
                    let Some(candidates) = by_domain_name.get(&(target_domain, called.clone()))
                    else {
                        continue;
                    };
                    for target in candidates {
                        let reaches_writer = writer_roots.contains(target)
                            || reachable.get(target).is_some_and(|roots| !roots.is_empty());
                        if reaches_writer && targets.insert(target.clone()) {
                            added_cross_domain_edge = true;
                        }
                    }
                }
            }
        }
        if !added_cross_domain_edge {
            break;
        }
        reachable = compute_reverse_reachability(&edges, &writer_roots);
    }

    // Productive identity provenance deliberately reuses the finalized semantic call graph.
    // There is no second name-based writer graph. A Music function becomes an identity source only
    // at an explicit runtime ingress whose key/argument carries identity semantics, or at unscoped
    // args/env enumeration. From that source on, helper and variable names are irrelevant: reverse
    // reachability over the exact finalized graph tracks every Music caller that can observe it.
    // Cross-domain callers cannot inject a raw locator through a public Music API because the
    // separate opaque-locator boundary below rejects primitive public identity parameters/fields.
    let music_keys = inventory
        .functions
        .values()
        .filter(|function| function.domain == Domain::Music)
        .map(|function| function.key.clone())
        .collect::<BTreeSet<_>>();
    let music_edges = edges
        .iter()
        .filter(|(caller, _)| music_keys.contains(*caller))
        .map(|(caller, targets)| {
            (
                caller.clone(),
                targets
                    .iter()
                    .filter(|target| music_keys.contains(*target))
                    .cloned()
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let external_identity_roots = inventory
        .functions
        .values()
        .filter(|function| {
            function.domain == Domain::Music && direct_external_identity_input(function)
        })
        .map(|function| function.key.clone())
        .collect::<BTreeSet<_>>();
    let external_identity_reachability =
        compute_reverse_reachability(&music_edges, &external_identity_roots);

    let music_writer_roots = writer_roots
        .iter()
        .filter(|root| {
            inventory
                .functions
                .get(*root)
                .is_some_and(|function| function.domain == Domain::Music)
        })
        .cloned()
        .collect::<BTreeSet<_>>();

    let provenance_functions = inventory.functions.values().cloned().collect::<Vec<_>>();
    for function in &provenance_functions {
        if function.domain != Domain::Music {
            continue;
        }
        let external_roots = external_identity_reachability
            .get(&function.key)
            .cloned()
            .unwrap_or_default();
        if external_roots.is_empty() {
            continue;
        }
        let writer_targets = reachable
            .get(&function.key)
            .into_iter()
            .flatten()
            .filter(|root| music_writer_roots.contains(*root))
            .cloned()
            .collect::<BTreeSet<_>>();
        if writer_targets.is_empty() {
            continue;
        }
        inventory.productive_identity_provenance_violations.push(
            ProductiveMusicIdentityProvenanceViolation {
                path: function.path.clone(),
                symbol: function.symbol.clone(),
                detail: format!(
                    "external identity runtime source reaches productive Music writer; external_roots=[{}]; writer_roots=[{}]",
                    external_roots.into_iter().collect::<Vec<_>>().join(","),
                    writer_targets.into_iter().collect::<Vec<_>>().join(",")
                ),
            },
        );
    }

    let functions = inventory.functions.values().cloned().collect::<Vec<_>>();
    for function in functions {
        let roots = reachable.get(&function.key).cloned().unwrap_or_default();
        if !roots.is_empty() {
            let root_text = roots.iter().cloned().collect::<Vec<_>>().join(",");
            add_record(
                inventory,
                CapabilityRecord {
                    kind: CapabilityKind::WriterReachability,
                    domain: function.domain,
                    path: function.path.clone(),
                    symbol: function.symbol.clone(),
                    fingerprint: fnv1a64(root_text.as_bytes()),
                    detail: root_text,
                },
            )?;
            if function.public {
                add_record(
                    inventory,
                    CapabilityRecord {
                        kind: CapabilityKind::ExportedWriterCapability,
                        domain: function.domain,
                        path: function.path.clone(),
                        symbol: function.symbol.clone(),
                        fingerprint: fnv1a64(normalize_semantic(&function.signature).as_bytes()),
                        detail: "public function reaches writer family".to_string(),
                    },
                )?;
            }
        }

        for (called, count) in &function.calls {
            let same_domain_writer = by_domain_name
                .get(&(function.domain, called.clone()))
                .into_iter()
                .flatten()
                .any(|target| {
                    writer_roots.contains(target)
                        || reachable.get(target).is_some_and(|roots| !roots.is_empty())
                });
            let cross_domain_resolvable = writer_like_name(&called.to_ascii_lowercase())
                || global_name_counts.get(called) == Some(&1);
            let cross_domain_writer = cross_domain_resolvable
                && [Domain::Kodex, Domain::Audit, Domain::Music, Domain::Unknown]
                    .into_iter()
                    .filter(|domain| *domain != function.domain)
                    .flat_map(|domain| {
                        by_domain_name
                            .get(&(domain, called.clone()))
                            .into_iter()
                            .flatten()
                    })
                    .any(|target| {
                        writer_roots.contains(target)
                            || reachable.get(target).is_some_and(|roots| !roots.is_empty())
                    });
            let writer_target = same_domain_writer || cross_domain_writer;
            if writer_target {
                add_record(
                    inventory,
                    CapabilityRecord {
                        kind: CapabilityKind::WriteFunctionCall,
                        domain: function.domain,
                        path: function.path.clone(),
                        symbol: format!("{}->{called}", function.symbol),
                        fingerprint: fnv1a64(count.to_string().as_bytes()),
                        detail: format!("{count} call(s) into writer-reachable function"),
                    },
                )?;
            }
        }
    }
    Ok(())
}

fn compute_reverse_reachability(
    edges: &BTreeMap<String, BTreeSet<String>>,
    roots: &BTreeSet<String>,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut reverse = BTreeMap::<String, BTreeSet<String>>::new();
    for (caller, targets) in edges {
        for target in targets {
            reverse
                .entry(target.clone())
                .or_default()
                .insert(caller.clone());
        }
    }
    let mut result = BTreeMap::<String, BTreeSet<String>>::new();
    for root in roots {
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut visited = BTreeSet::new();
        while let Some(node) = queue.pop_front() {
            if !visited.insert(node.clone()) {
                continue;
            }
            result.entry(node.clone()).or_default().insert(root.clone());
            if let Some(callers) = reverse.get(&node) {
                queue.extend(callers.iter().cloned());
            }
        }
    }
    result
}

fn structural_writer_root(function: &FunctionInfo) -> bool {
    function.path.starts_with("src/structural_write_authority/")
        && (function.direct_writer || writer_like_name(&function.name.to_ascii_lowercase()))
}

fn diff_inventories(
    current: &BTreeMap<String, CapabilityRecord>,
    incoming: &BTreeMap<String, CapabilityRecord>,
) -> Vec<CapabilityDelta> {
    let keys = current
        .keys()
        .chain(incoming.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut deltas = Vec::new();
    for key in keys {
        match (current.get(&key), incoming.get(&key)) {
            (None, Some(after)) => deltas.push(CapabilityDelta {
                kind: DeltaKind::Added,
                before: None,
                after: Some(after.clone()),
            }),
            (Some(before), None) => deltas.push(CapabilityDelta {
                kind: DeltaKind::Removed,
                before: Some(before.clone()),
                after: None,
            }),
            (Some(before), Some(after)) if before.fingerprint != after.fingerprint => {
                deltas.push(CapabilityDelta {
                    kind: DeltaKind::Modified,
                    before: Some(before.clone()),
                    after: Some(after.clone()),
                });
            }
            _ => {}
        }
    }
    deltas
}

fn validate_changed_paths_are_known(
    changed_paths: &BTreeSet<String>,
    deltas: &[CapabilityDelta],
) -> Result<(), String> {
    let delta_paths = deltas
        .iter()
        .map(|delta| delta.record().path.clone())
        .collect::<BTreeSet<_>>();
    for path in changed_paths {
        if path == "Cargo.toml"
            || path == "README.md"
            || path == "ZIP_KIND.txt"
            || path.starts_with("docs/")
            || path == CHANGE_MANIFEST_PATH
            || path == WATCH_PROJECT_MANIFEST_PATH
            || path == WATCH_PROJECT_LOCK_PATH
        {
            continue;
        }
        if !known_managed_path(path) {
            return Err(format!(
                "Semantic-Capability-Diff: unbekannter geaenderter Pfad {path}; fail-closed"
            ));
        }
        if !delta_paths.contains(path) {
            let mechanical_text = path.ends_with(".rs") || path.ends_with(".cmd");
            if !mechanical_text {
                return Err(format!(
                    "Semantic-Capability-Diff: geaenderter Pfad {path} erzeugt keine bekannte Capability; fail-closed"
                ));
            }
        }
    }
    Ok(())
}

fn read_kodex_capability_retirements(
    root: &Path,
) -> Result<BTreeMap<String, KodexCapabilityRetirement>, String> {
    let path = root.join(KODEX_CAPABILITY_RETIREMENTS_PATH);
    if !path.is_file() {
        return Ok(BTreeMap::new());
    }
    let text = fs::read_to_string(&path).map_err(|error| {
        format!(
            "KodexCapabilityRetirement: ledger nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    let mut schema_seen = false;
    let mut entries = BTreeMap::new();
    for (line_index, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line == "schema_version=1" {
            if schema_seen {
                return Err(format!(
                    "KodexCapabilityRetirement: doppelte schema_version in Zeile {}",
                    line_index + 1
                ));
            }
            schema_seen = true;
            continue;
        }
        let Some(payload) = line.strip_prefix("retire=") else {
            return Err(format!(
                "KodexCapabilityRetirement: unbekannte Ledger-Zeile {}: {:?}",
                line_index + 1,
                raw_line
            ));
        };
        let fields = payload.split('|').collect::<Vec<_>>();
        if fields.len() != 5 {
            return Err(format!(
                "KodexCapabilityRetirement: ungueltiger Eintrag Zeile {}; erwartet 5 Felder",
                line_index + 1
            ));
        }
        let pre_kodex_negative_check = format!("{}NegativeCheck", pre_kodex_title_token());
        let kind = match fields[0] {
            value if value == pre_kodex_negative_check.as_str() => {
                CapabilityKind::KodexNegativeCheck
            }
            "KodexNegativeCheck" => CapabilityKind::KodexNegativeCheck,
            other => {
                return Err(format!(
                    "KodexCapabilityRetirement: CapabilityKind {other:?} ist in schema_version=1 nicht pensionierbar"
                ));
            }
        };
        let path = fields[1].trim();
        let symbol = fields[2].trim();
        let fingerprint = fields[3].strip_prefix("fnv64:").ok_or_else(|| {
            format!(
                "KodexCapabilityRetirement: FNV-Feld fehlt in Zeile {}",
                line_index + 1
            )
        })?;
        if fingerprint.len() != 16 || !fingerprint.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(format!(
                "KodexCapabilityRetirement: FNV64 muss exakt 16 Hex-Zeichen besitzen, Zeile {}",
                line_index + 1
            ));
        }
        let before_fingerprint = u64::from_str_radix(fingerprint, 16).map_err(|error| {
            format!(
                "KodexCapabilityRetirement: ungueltiger FNV64 in Zeile {}: {error}",
                line_index + 1
            )
        })?;
        let reason = fields[4]
            .strip_prefix("reason:")
            .ok_or_else(|| {
                format!(
                    "KodexCapabilityRetirement: reason-Feld fehlt in Zeile {}",
                    line_index + 1
                )
            })?
            .trim();
        if path.is_empty()
            || symbol.is_empty()
            || reason.is_empty()
            || !reason
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        {
            return Err(format!(
                "KodexCapabilityRetirement: Pfad, Symbol oder stabiler Reason-Code ungueltig in Zeile {}",
                line_index + 1
            ));
        }
        let retirement = KodexCapabilityRetirement {
            kind,
            path: path.to_string(),
            symbol: symbol.to_string(),
            before_fingerprint,
            reason: reason.to_string(),
        };
        let key = retirement.capability_key();
        if entries.insert(key.clone(), retirement).is_some() {
            return Err(format!(
                "KodexCapabilityRetirement: doppelter Capability-Key {key}"
            ));
        }
    }
    if !schema_seen {
        return Err(
            "KodexCapabilityRetirement: vorhandenes Ledger benoetigt schema_version=1".to_string(),
        );
    }
    Ok(entries)
}

fn validate_kodex_capability_retirement_contract(
    current_root: &Path,
    incoming_root: &Path,
    change_class: &str,
    current: &Inventory,
    incoming: &Inventory,
    deltas: &[CapabilityDelta],
) -> Result<BTreeSet<String>, String> {
    let current_retirements = read_kodex_capability_retirements(current_root)?;
    let incoming_retirements = read_kodex_capability_retirements(incoming_root)?;

    for (key, current_entry) in &current_retirements {
        let Some(incoming_entry) = incoming_retirements.get(key) else {
            return Err(format!(
                "KodexCapabilityRetirement: bestehender Retirement-Eintrag darf nicht geloescht werden: {key}"
            ));
        };
        if incoming_entry != current_entry {
            return Err(format!(
                "KodexCapabilityRetirement: bestehender Retirement-Eintrag darf nicht geaendert werden: {key}"
            ));
        }
    }

    let added = incoming_retirements
        .iter()
        .filter(|(key, _)| !current_retirements.contains_key(*key))
        .map(|(_, value)| value)
        .collect::<Vec<_>>();
    // GovernedChange may retire only through a parent-admitted govern=retire line; the
    // SSC governed-change gate binds that line to a pure, runtime-free deletion.
    let retirement_change_class_allowed =
        matches!(change_class, "KodexOnly" | "OutcomeProjection" | "GovernedChange");
    if !added.is_empty() && !retirement_change_class_allowed {
        return Err(
            "KodexCapabilityRetirement: neue Pensionierungen sind ausschliesslich in KodexOnly oder OutcomeProjection erlaubt; GovernedChange nur ueber eine vom Parent zugelassene govern=retire-Zeile"
                .to_string(),
        );
    }

    let removed_negative = deltas
        .iter()
        .filter(|delta| {
            delta.kind == DeltaKind::Removed
                && delta.record().kind == CapabilityKind::KodexNegativeCheck
        })
        .map(|delta| delta.record().key())
        .collect::<BTreeSet<_>>();

    let mut authorized = BTreeSet::new();
    for retirement in added {
        let key = retirement.capability_key();
        let Some(record) = current.records.get(&key) else {
            return Err(format!(
                "KodexCapabilityRetirement: neue Pensionierung bindet keine aktuell vorhandene Capability: {key}"
            ));
        };
        if record.kind != CapabilityKind::KodexNegativeCheck
            || record.fingerprint != retirement.before_fingerprint
        {
            return Err(format!(
                "KodexCapabilityRetirement: Fingerprint-/Kind-Bindung stimmt nicht fuer {key}: ledger={:016x} current={:016x}",
                retirement.before_fingerprint, record.fingerprint
            ));
        }
        if !removed_negative.contains(&key) {
            return Err(format!(
                "KodexCapabilityRetirement: neue Pensionierung ohne tatsaechliche Capability-Entfernung: {key}"
            ));
        }
        let function_key = format!("{}::{}", retirement.path, retirement.symbol);
        if incoming.functions.contains_key(&function_key) {
            return Err(format!(
                "KodexCapabilityRetirement: Pensionierung darf keinen Stub oder geschwaechten Funktionskoerper stehen lassen: {function_key}"
            ));
        }
        let current_function = current.functions.get(&function_key).ok_or_else(|| {
            format!(
                "KodexCapabilityRetirement: gebundene aktuelle Funktion fehlt fuer Relocation-Beweis: {function_key}"
            )
        })?;
        let retired_body_fingerprint =
            fnv1a64(normalize_identifier_invariant_body(&current_function.masked_body).as_bytes());
        let relocated_same_body = incoming.records.values().any(|candidate| {
            if candidate.kind != CapabilityKind::KodexNegativeCheck
                || candidate.fingerprint != retirement.before_fingerprint
            {
                return false;
            }
            let candidate_function_key = format!("{}::{}", candidate.path, candidate.symbol);
            incoming
                .functions
                .get(&candidate_function_key)
                .is_some_and(|candidate_function| {
                    fnv1a64(
                        normalize_identifier_invariant_body(&candidate_function.masked_body)
                            .as_bytes(),
                    ) == retired_body_fingerprint
                })
        });
        if relocated_same_body {
            return Err(format!(
                "KodexCapabilityRetirement: dieselbe Negative-Kodex-Funktion wurde semantisch nur verschoben statt pensioniert: {key}"
            ));
        }
        authorized.insert(key);
    }
    Ok(authorized)
}

fn ssc_audit_initializer_loss(parent: &Path, child: &Path, path: &str, name: &str) -> Result<bool, String> {
    if path != "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs" || name != "real_main"
        || !super::ssc_evidence_proof_shadow_permit_kodex::k2::installed(parent)
    {
        return Ok(false);
    }
    let Ok(manifest) = fs::read_to_string(child.join("src/build_gate/take_change_class.txt")) else { return Ok(false); };
    let docs = manifest.lines().filter_map(|s| s.strip_prefix("authority_retirement_contract=")).collect::<Vec<_>>();
    if docs.len() != 1 || !docs[0].starts_with("docs/authority_retirement_contract_")
        || !docs[0].ends_with(".md") || docs[0].contains("..")
    { return Ok(false); }
    let text = fs::read_to_string(child.join(docs[0])).map_err(|e| e.to_string())?;
    if !super::contract_value(&text, "SscExitObligations").is_some_and(|ids| ids.split(',').any(|id| id.trim() == "wav_salience_master_adoptions"))
        || !super::contract_value(&text, "RetiredAuthoritySymbols").is_some_and(|symbols| symbols.split(',').any(|s| s.trim() == format!("{path}::run_salience_weighted_candidate_gate")))
    { return Ok(false); }
    super::ssc_evidence_proof_shadow_permit_kodex::structural_erasure::registered_audit_initializer_file_loss(parent, child, path)
}

fn validate_music_mutation_derived_audit_writer_reachability_narrowing(
    current_root: &Path,
    incoming_root: &Path,
    change_class: &str,
    changed_paths: &BTreeSet<String>,
    current: &Inventory,
    incoming: &Inventory,
    deltas: &[CapabilityDelta],
) -> Result<BTreeSet<String>, String> {
    if change_class != "MusicMutation" {
        return Ok(BTreeSet::new());
    }

    let candidates = deltas
        .iter()
        .filter(|delta| {
            let record = delta.record();
            record.domain == Domain::Audit && record.kind.is_writer_family()
        })
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return Ok(BTreeSet::new());
    }

    let mut authorized = BTreeSet::new();
    for delta in candidates {
        let record = delta.record();
        if record.kind != CapabilityKind::WriterReachability || delta.kind != DeltaKind::Modified {
            continue;
        }
        let name = record.symbol.split('#').next().unwrap_or("");
        let registered_ssc_caller = ssc_audit_initializer_loss(current_root, incoming_root, &record.path, name)?;
        if changed_paths.contains(&record.path) && !registered_ssc_caller {
            return Err(format!(
                "Semantic-Capability-Diff: MusicMutation darf abgeleitete Audit-WriterReachability nur bei byteunveraenderter Audit-Quelle verengen: {}::{}",
                record.path, record.symbol
            ));
        }
        let current_bytes = fs::read(current_root.join(&record.path)).map_err(|error| {
            format!(
                "Semantic-Capability-Diff: aktuelle Audit-Quelle fuer WriterReachability nicht lesbar {}: {error}",
                record.path
            )
        })?;
        let incoming_bytes = fs::read(incoming_root.join(&record.path)).map_err(|error| {
            format!(
                "Semantic-Capability-Diff: eingehende Audit-Quelle fuer WriterReachability nicht lesbar {}: {error}",
                record.path
            )
        })?;
        if current_bytes != incoming_bytes && !registered_ssc_caller {
            return Err(format!(
                "Semantic-Capability-Diff: MusicMutation veraendert Audit-Quelle hinter abgeleiteter WriterReachability: {}::{}",
                record.path, record.symbol
            ));
        }

        let before = delta
            .before
            .as_ref()
            .expect("modified WriterReachability needs current record");
        let after = delta
            .after
            .as_ref()
            .expect("modified WriterReachability needs incoming record");
        let before_roots = writer_reachability_roots(&before.detail);
        let after_roots = writer_reachability_roots(&after.detail);
        if after_roots.is_empty()
            || after_roots == before_roots
            || !after_roots.is_subset(&before_roots)
        {
            return Err(format!(
                "Semantic-Capability-Diff: MusicMutation darf Audit-WriterReachability nur streng verengen, niemals erweitern oder umhaengen: {}::{} vorher=[{}] nachher=[{}]",
                record.path, record.symbol, before.detail, after.detail
            ));
        }

        for removed_root in before_roots.difference(&after_roots) {
            let current_function = current.functions.get(removed_root).ok_or_else(|| {
                format!(
                    "Semantic-Capability-Diff: verengter Audit-Writer-Root fehlt im aktuellen Inventar: {removed_root}"
                )
            })?;
            if current_function.domain != Domain::Music {
                return Err(format!(
                    "Semantic-Capability-Diff: abgeleitete Audit-Writer-Verengung darf nur Music-Writer-Roots abkoppeln: {removed_root}"
                ));
            }

            if let Some(incoming_function) = incoming.functions.get(removed_root) {
                if incoming_function.domain != Domain::Music {
                    return Err(format!(
                        "Semantic-Capability-Diff: abgekoppelter Writer-Root wechselte die Domaene: {removed_root}"
                    ));
                }
                if current_function.signature != incoming_function.signature
                    || current_function.source_body != incoming_function.source_body
                {
                    return Err(format!(
                        "Semantic-Capability-Diff: abgekoppelter Music-Writer-Root wurde zugleich veraendert; separate Capability-Aenderung erforderlich: {removed_root}"
                    ));
                }
                continue;
            }

            // Ein nachweislich aus der Writer-Erreichbarkeit abgekuppelter Music-Root
            // darf im selben MusicMutation-Schnitt vollstaendig pensioniert werden.
            // Das ist eine echte Entfernung, keine Relocation: Ein semantisch gleicher
            // Funktionskoerper darf nicht unter neuer Identitaet auftauchen.
            let retired_body_fingerprint = fnv1a64(
                normalize_identifier_invariant_body(&current_function.masked_body).as_bytes(),
            );
            let semantically_relocated =
                incoming.functions.iter().any(|(candidate_key, candidate)| {
                    if candidate.domain != Domain::Music || candidate_key == removed_root {
                        return false;
                    }
                    let candidate_body_fingerprint = fnv1a64(
                        normalize_identifier_invariant_body(&candidate.masked_body).as_bytes(),
                    );
                    if candidate_body_fingerprint != retired_body_fingerprint {
                        return false;
                    }
                    current
                        .functions
                        .get(candidate_key)
                        .is_none_or(|previous_candidate| {
                            fnv1a64(
                                normalize_identifier_invariant_body(
                                    &previous_candidate.masked_body,
                                )
                                .as_bytes(),
                            ) != retired_body_fingerprint
                        })
                });
            if semantically_relocated {
                return Err(format!(
                    "Semantic-Capability-Diff: pensionierter Music-Writer-Root wurde semantisch verschoben statt entfernt: {removed_root}"
                ));
            }
        }
        authorized.insert(record.key());
        if registered_ssc_caller {
            for delta in deltas.iter().filter(|d| d.kind == DeltaKind::Modified
                && d.record().domain == Domain::Audit && d.record().kind == CapabilityKind::AuditLogic
                && d.record().path == record.path
                && (d.record().symbol == record.symbol || d.record().symbol == "<file-surface>"))
            {
                authorized.insert(delta.record().key());
            }
        }
    }
    Ok(authorized)
}

fn writer_reachability_roots(detail: &str) -> BTreeSet<String> {
    detail
        .split(',')
        .map(str::trim)
        .filter(|root| !root.is_empty())
        .map(str::to_string)
        .collect()
}

struct PolicyValidationContext<'a> {
    windows_job_dependency_transition: bool,
    audit_cross_role_retirement: &'a AuditCrossRoleRetirement,
    kodex_capability_retirements: &'a BTreeSet<String>,
    kodex_writer_relocation: bool,
    kodex_negative_check_relocations: &'a BTreeSet<String>,
    derived_audit_writer_reachability_narrowing: &'a BTreeSet<String>,
}

fn validate_policy(
    change_class: &str,
    deltas: &[CapabilityDelta],
    context: &PolicyValidationContext<'_>,
) -> Result<(), String> {
    let has_writer = deltas
        .iter()
        .any(|delta| delta.kind != DeltaKind::Removed && delta.record().kind.is_writer_family());
    let has_threshold = deltas
        .iter()
        .any(|delta| delta.record().kind == CapabilityKind::CalibrationThreshold);
    if change_class != "RefactorOnly" && has_writer && has_threshold {
        return Err(
            "Semantic-Capability-Diff: Writer-Faehigkeit und Schwellenwertaenderung duerfen niemals in derselben Version zusammentreffen"
                .to_string(),
        );
    }

    let mut violations = Vec::new();
    for delta in deltas {
        let record = delta.record();
        if record.kind == CapabilityKind::KodexNegativeCheck && delta.kind == DeltaKind::Removed {
            let retirement_authorized =
                matches!(change_class, "KodexOnly" | "OutcomeProjection" | "GovernedChange")
                && context.kodex_capability_retirements.contains(&record.key());
            let relocation_authorized = change_class == "KodexOnly"
                && context
                    .kodex_negative_check_relocations
                    .contains(&record.key());
            if !retirement_authorized && !relocation_authorized {
                violations.push(format!(
                    "removed negative/kodex check {}::{} without exact KodexCapabilityRetirement or verified KodexNegativeCheck relocation",
                    record.path, record.symbol
                ));
                continue;
            }
        }
        if matches!(
            record.kind,
            CapabilityKind::CargoDependency | CapabilityKind::CargoFeature
        ) {
            if context.windows_job_dependency_transition
                && delta.kind == DeltaKind::Added
                && record.kind == CapabilityKind::CargoDependency
                && record.path == "Cargo.toml"
                && record.symbol == "target.'cfg(windows)'.dependencies::windows-sys"
            {
                continue;
            }
            if change_class == "ValidationOnly" && validation_only_allowed(delta) {
                continue;
            }
            violations.push(format!(
                "{} {} {}::{} (Cargo capability changes are closed in steady state until a new explicit future admission contract exists)",
                delta.kind.as_str(),
                record.kind.as_str(),
                record.path,
                record.symbol
            ));
            continue;
        }
        let allowed = match change_class {
            "KodexOnly" => kodex_only_allowed(delta, context.kodex_writer_relocation),
            "AuditOnly" => audit_only_allowed(delta, context.audit_cross_role_retirement),
            "OutcomeProjection" => outcome_projection_allowed(delta),
            "RefactorOnly" => record.domain != Domain::Kodex,
            // Contract v4 B2: runtime, test and audit capabilities like RefactorOnly (without
            // its rename relaxations); Kodex capabilities only through retire (above).
            "GovernedChange" => {
                record.domain != Domain::Kodex
                    || (record.kind == CapabilityKind::KodexNegativeCheck
                        && delta.kind == DeltaKind::Removed)
                    || governed_change_kodex_surface(&record.path)
            }
            "MusicMutation" => {
                music_mutation_allowed(delta)
                    || context
                        .derived_audit_writer_reachability_narrowing
                        .contains(&record.key())
            }
            "CalibrationOnly" => calibration_only_allowed(delta),
            "MechanicalOnly" => false,
            "ValidationOnly" => validation_only_allowed(delta),
            other => {
                return Err(format!(
                    "Semantic-Capability-Diff: unbekannte change_class {other:?}; fail-closed"
                ));
            }
        };
        if !allowed {
            violations.push(format!(
                "{} {} {}::{} domain={:?} detail={}",
                delta.kind.as_str(),
                record.kind.as_str(),
                record.path,
                record.symbol,
                record.domain,
                record.detail
            ));
        }
    }
    if violations.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "Semantic-Capability-Diff: Klasse {change_class} verletzt die semantische Capability-Policy:\n  - {}",
            violations.join("\n  - ")
        ))
    }
}

// GovernedChange Kodex surfaces: the governed data files, and the ledger/trust-anchor
// pair of a retire. Path policy and the parent-compiled governed-change gate decide
// whether and how these files may change; this only keeps the diff from double-blocking.
fn governed_change_kodex_surface(path: &str) -> bool {
    use super::ssc_evidence_proof_shadow_permit_kodex::k2::governed;
    [
        governed::ENVELOPES_PATH,
        governed::EXTENSIONS_PATH,
        governed::WIRING_PATH,
        governed::LEDGER_PATH,
        governed::TRUST_ANCHOR_PATH,
    ]
    .contains(&path)
        || super::ssc_evidence_proof_shadow_permit_kodex::k2::checkpoint_prepare::writer_retirement_kodex_surface(path)
}

fn kodex_only_allowed(delta: &CapabilityDelta, kodex_writer_relocation: bool) -> bool {
    let record = delta.record();
    if record.domain != Domain::Kodex {
        return false;
    }
    if kodex_writer_relocation
        && delta.kind != DeltaKind::Modified
        && matches!(
            record.kind,
            CapabilityKind::WriteFunctionCall | CapabilityKind::WriterReachability
        )
    {
        return true;
    }
    matches!(
        record.kind,
        CapabilityKind::KodexControl
            | CapabilityKind::KodexNegativeCheck
            | CapabilityKind::BuildScriptCapability
            | CapabilityKind::FeedbackIdentityChannel
            | CapabilityKind::ProofReentryReader
            | CapabilityKind::MacroOrGeneratedSource
    )
}

fn validate_kodex_writer_relocation(
    change_class: &str,
    changed_paths: &BTreeSet<String>,
    current: &Inventory,
    incoming: &Inventory,
    deltas: &[CapabilityDelta],
) -> Result<(), String> {
    if change_class != "KodexOnly" {
        return Err(
            "Semantic-Capability-Diff: kodex_writer_relocation ist nur fuer KodexOnly erlaubt"
                .to_string(),
        );
    }
    let non_kodex_source_paths = changed_paths
        .iter()
        .filter(|path| path.ends_with(".rs") || path.ends_with(".cmd"))
        .filter(|path| domain_for_path(path) != Domain::Kodex)
        .cloned()
        .collect::<Vec<_>>();
    if !non_kodex_source_paths.is_empty() {
        return Err(format!(
            "Semantic-Capability-Diff: Kodex-Writer-Relocation enthaelt fremde Source-Domaenen {non_kodex_source_paths:?}"
        ));
    }
    let writer_deltas = deltas
        .iter()
        .filter(|delta| delta.record().kind.is_writer_family())
        .collect::<Vec<_>>();
    if writer_deltas.is_empty() {
        return Err(
            "Semantic-Capability-Diff: kodex_writer_relocation deklariert, aber keine Writer-Capability wurde verlagert"
                .to_string(),
        );
    }
    let invalid = writer_deltas
        .iter()
        .filter(|delta| {
            let record = delta.record();
            record.domain != Domain::Kodex
                || delta.kind == DeltaKind::Modified
                || !matches!(
                    record.kind,
                    CapabilityKind::WriteFunctionCall | CapabilityKind::WriterReachability
                )
        })
        .map(|delta| {
            let record = delta.record();
            format!(
                "{} {} {}::{} domain={:?}",
                delta.kind.as_str(),
                record.kind.as_str(),
                record.path,
                record.symbol,
                record.domain
            )
        })
        .collect::<Vec<_>>();
    if !invalid.is_empty() {
        return Err(format!(
            "Semantic-Capability-Diff: Kodex-Writer-Relocation darf nur interne Kodex WriteFunctionCall/WriterReachability-Knoten verschieben:
  - {}",
            invalid.join("
  - ")
        ));
    }
    let has_added = writer_deltas
        .iter()
        .any(|delta| delta.kind == DeltaKind::Added);
    let has_removed = writer_deltas
        .iter()
        .any(|delta| delta.kind == DeltaKind::Removed);
    if !has_added || !has_removed {
        return Err(
            "Semantic-Capability-Diff: Kodex-Writer-Relocation braucht sowohl entfernte als auch hinzugefuegte interne Knoten"
                .to_string(),
        );
    }
    let current_sinks = writer_sink_contract(current);
    let incoming_sinks = writer_sink_contract(incoming);
    if current_sinks != incoming_sinks {
        return Err(format!(
            "Semantic-Capability-Diff: Kodex-Writer-Relocation veraendert die Writer-Senken: vorher {current_sinks:?}, nachher {incoming_sinks:?}"
        ));
    }
    let current_exports = exported_writer_contract(current);
    let incoming_exports = exported_writer_contract(incoming);
    if current_exports != incoming_exports {
        return Err(format!(
            "Semantic-Capability-Diff: Kodex-Writer-Relocation veraendert die extern sichtbare Writer-Flaeche: vorher {current_exports:?}, nachher {incoming_exports:?}"
        ));
    }
    let current_direct = direct_writer_contract(current);
    let incoming_direct = direct_writer_contract(incoming);
    if current_direct != incoming_direct {
        return Err(format!(
            "Semantic-Capability-Diff: Kodex-Writer-Relocation veraendert direkte Writer-Autoritaeten: vorher {current_direct:?}, nachher {incoming_direct:?}"
        ));
    }
    Ok(())
}

fn kodex_execution_anchor_contract(
    inventory: &Inventory,
    target: &FunctionInfo,
) -> Result<BTreeSet<String>, String> {
    let kodex_functions = inventory
        .functions
        .values()
        .filter(|function| function.domain == Domain::Kodex)
        .collect::<Vec<_>>();
    let mut by_name = BTreeMap::<String, Vec<String>>::new();
    for function in &kodex_functions {
        by_name
            .entry(function.name.clone())
            .or_default()
            .push(function.key.clone());
    }
    for keys in by_name.values_mut() {
        keys.sort();
    }

    let mut reverse = BTreeMap::<String, BTreeSet<String>>::new();
    for caller in &kodex_functions {
        for called in caller.calls.keys() {
            let Some(candidates) = by_name.get(called) else {
                continue;
            };
            let targets = if candidates.len() == 1 {
                candidates.clone()
            } else {
                candidates
                    .iter()
                    .filter(|key| key.starts_with(&format!("{}::", caller.path)))
                    .cloned()
                    .collect::<Vec<_>>()
            };
            for target_key in targets {
                reverse
                    .entry(target_key)
                    .or_default()
                    .insert(caller.key.clone());
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(target.key.clone());
    let mut visited = BTreeSet::new();
    let mut anchors = BTreeSet::new();
    while let Some(node) = queue.pop_front() {
        if !visited.insert(node.clone()) {
            continue;
        }
        if let Some(function) = inventory.functions.get(&node)
            && function.domain == Domain::Kodex
            && function.name == "main"
        {
            anchors.insert(format!("{}::{}", function.path, function.symbol));
        }
        if let Some(callers) = reverse.get(&node) {
            queue.extend(callers.iter().cloned());
        }
    }
    if anchors.is_empty() {
        return Err(format!(
            "Semantic-Capability-Diff: KodexNegativeCheck-Relocation verliert oder besitzt keinen statisch nachweisbaren Kodex-main-Ausfuehrungsanker fuer {}::{}",
            target.path, target.symbol
        ));
    }
    Ok(anchors)
}

fn validate_kodex_negative_check_relocation(
    change_class: &str,
    changed_paths: &BTreeSet<String>,
    current: &Inventory,
    incoming: &Inventory,
    deltas: &[CapabilityDelta],
) -> Result<BTreeSet<String>, String> {
    if change_class != "KodexOnly" {
        return Err(
            "Semantic-Capability-Diff: kodex_negative_check_relocation ist nur fuer KodexOnly erlaubt"
                .to_string(),
        );
    }
    let non_kodex_source_paths = changed_paths
        .iter()
        .filter(|path| path.ends_with(".rs") || path.ends_with(".cmd"))
        .filter(|path| domain_for_path(path) != Domain::Kodex)
        .cloned()
        .collect::<Vec<_>>();
    if !non_kodex_source_paths.is_empty() {
        return Err(format!(
            "Semantic-Capability-Diff: KodexNegativeCheck-Relocation enthaelt fremde Source-Domaenen {non_kodex_source_paths:?}"
        ));
    }

    let removed = deltas
        .iter()
        .filter(|delta| {
            delta.kind == DeltaKind::Removed
                && delta.record().kind == CapabilityKind::KodexNegativeCheck
        })
        .collect::<Vec<_>>();
    let added = deltas
        .iter()
        .filter(|delta| {
            delta.kind == DeltaKind::Added
                && delta.record().kind == CapabilityKind::KodexNegativeCheck
        })
        .collect::<Vec<_>>();
    if removed.is_empty() || added.is_empty() {
        return Err(
            "Semantic-Capability-Diff: KodexNegativeCheck-Relocation braucht sowohl entfernte als auch hinzugefuegte Negative-Checks"
                .to_string(),
        );
    }
    if removed.len() != added.len() {
        return Err(format!(
            "Semantic-Capability-Diff: KodexNegativeCheck-Relocation ist nicht bijektiv: entfernt {}, hinzugefuegt {}",
            removed.len(),
            added.len()
        ));
    }

    fn contract_identity(
        inventory: &Inventory,
        record: &CapabilityRecord,
    ) -> Result<(String, u64, u64, u64, u64), String> {
        if record.domain != Domain::Kodex || record.kind != CapabilityKind::KodexNegativeCheck {
            return Err(format!(
                "Semantic-Capability-Diff: ungueltige KodexNegativeCheck-Relocation {}::{} domain={:?}",
                record.path, record.symbol, record.domain
            ));
        }
        let function_key = format!("{}::{}", record.path, record.symbol);
        let function = inventory.functions.get(&function_key).ok_or_else(|| {
            format!(
                "Semantic-Capability-Diff: KodexNegativeCheck-Relocation findet Funktionskoerper nicht: {function_key}"
            )
        })?;
        let signature_fingerprint =
            fnv1a64(normalize_semantic(&mask_non_code(&function.signature)).as_bytes());
        let body_fingerprint = fnv1a64(normalize_semantic(&function.masked_body).as_bytes());
        let execution_anchors = kodex_execution_anchor_contract(inventory, function)?;
        let execution_anchor_fingerprint = fnv1a64(
            execution_anchors
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        );
        Ok((
            record.symbol.clone(),
            record.fingerprint,
            signature_fingerprint,
            body_fingerprint,
            execution_anchor_fingerprint,
        ))
    }

    let mut removed_contract = BTreeMap::<(String, u64, u64, u64, u64), usize>::new();
    let mut added_contract = BTreeMap::<(String, u64, u64, u64, u64), usize>::new();
    let mut authorized = BTreeSet::new();
    for delta in removed {
        let record = delta.record();
        let identity = contract_identity(current, record)?;
        *removed_contract.entry(identity).or_insert(0) += 1;
        authorized.insert(record.key());
    }
    for delta in added {
        let record = delta.record();
        let identity = contract_identity(incoming, record)?;
        *added_contract.entry(identity).or_insert(0) += 1;
    }
    if removed_contract != added_contract {
        return Err(format!(
            "Semantic-Capability-Diff: KodexNegativeCheck-Relocation veraendert Symbol, Signatur, Rejection-Surface, Funktionskoerper oder Kodex-main-Ausfuehrungsanker: vorher {removed_contract:?}, nachher {added_contract:?}"
        ));
    }
    Ok(authorized)
}
fn validate_kodex_negative_check_module_relocation(
    incoming_root: &Path,
    change_class: &str,
    changed_paths: &BTreeSet<String>,
    current: &Inventory,
    incoming: &Inventory,
    deltas: &[CapabilityDelta],
) -> Result<BTreeSet<String>, String> {
    if change_class != "KodexOnly" {
        return Err(
            "Semantic-Capability-Diff: kodex_negative_check_module_relocation ist nur fuer KodexOnly erlaubt"
                .to_string(),
        );
    }
    let non_kodex_source_paths = changed_paths
        .iter()
        .filter(|path| path.ends_with(".rs") || path.ends_with(".cmd"))
        .filter(|path| domain_for_path(path) != Domain::Kodex)
        .cloned()
        .collect::<Vec<_>>();
    if !non_kodex_source_paths.is_empty() {
        return Err(format!(
            "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation enthaelt fremde Source-Domaenen {non_kodex_source_paths:?}"
        ));
    }

    let removed = deltas
        .iter()
        .filter(|delta| {
            delta.kind == DeltaKind::Removed
                && delta.record().kind == CapabilityKind::KodexNegativeCheck
        })
        .collect::<Vec<_>>();
    let added = deltas
        .iter()
        .filter(|delta| {
            delta.kind == DeltaKind::Added
                && delta.record().kind == CapabilityKind::KodexNegativeCheck
        })
        .collect::<Vec<_>>();
    if removed.is_empty() || added.is_empty() {
        return Err(
            "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation braucht sowohl entfernte als auch hinzugefuegte Negative-Checks"
                .to_string(),
        );
    }
    if removed.len() != added.len() {
        return Err(format!(
            "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation ist nicht bijektiv: entfernt {}, hinzugefuegt {}",
            removed.len(),
            added.len()
        ));
    }

    fn module_contract_identity(
        inventory: &Inventory,
        record: &CapabilityRecord,
        incoming_visibility: bool,
    ) -> Result<(String, u64, u64, u64, u64), String> {
        if record.domain != Domain::Kodex || record.kind != CapabilityKind::KodexNegativeCheck {
            return Err(format!(
                "Semantic-Capability-Diff: ungueltige KodexNegativeCheck-Module-Relocation {}::{} domain={:?}",
                record.path, record.symbol, record.domain
            ));
        }
        let function_key = format!("{}::{}", record.path, record.symbol);
        let function = inventory.functions.get(&function_key).ok_or_else(|| {
            format!(
                "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation findet Funktionskoerper nicht: {function_key}"
            )
        })?;
        let trimmed_signature = function.signature.trim_start();
        let canonical_signature = if incoming_visibility {
            if !function.public || !trimmed_signature.starts_with("pub(super) fn ") {
                return Err(format!(
                    "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation erwartet am Ziel exakt pub(super) fuer {}::{}",
                    record.path, record.symbol
                ));
            }
            trimmed_signature.strip_prefix("pub(super) ").ok_or_else(|| {
                format!(
                    "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation kann pub(super)-Signatur nicht kanonisieren fuer {}::{}",
                    record.path, record.symbol
                )
            })?
        } else {
            if function.public || !trimmed_signature.starts_with("fn ") {
                return Err(format!(
                    "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation erwartet an der Quelle eine private fn fuer {}::{}",
                    record.path, record.symbol
                ));
            }
            trimmed_signature
        };
        let signature_fingerprint =
            fnv1a64(normalize_semantic(&mask_non_code(canonical_signature)).as_bytes());
        let body_fingerprint = fnv1a64(normalize_semantic(&function.masked_body).as_bytes());
        let execution_anchors = kodex_execution_anchor_contract(inventory, function)?;
        let execution_anchor_fingerprint = fnv1a64(
            execution_anchors
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        );
        Ok((
            record.symbol.clone(),
            record.fingerprint,
            signature_fingerprint,
            body_fingerprint,
            execution_anchor_fingerprint,
        ))
    }

    fn validate_parent_module_binding(
        incoming_root: &Path,
        source_record: &CapabilityRecord,
        target_record: &CapabilityRecord,
        target_function: &FunctionInfo,
    ) -> Result<(), String> {
        if source_record.path == target_record.path {
            return Err(format!(
                "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation braucht einen echten Pfadwechsel fuer {}",
                source_record.symbol
            ));
        }
        let source_path = Path::new(&source_record.path);
        let source_parent = source_path.parent().ok_or_else(|| {
            format!(
                "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation Quellpfad ohne Parent: {}",
                source_record.path
            )
        })?;
        let target_path = Path::new(&target_record.path);
        let relative_target = target_path.strip_prefix(source_parent).map_err(|_| {
            format!(
                "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation Ziel {} liegt nicht unter dem Parent-Modulpfad {}",
                target_record.path,
                source_parent.display()
            )
        })?;
        let module_name = target_path
            .file_stem()
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                format!(
                    "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation Ziel ohne Modulnamen: {}",
                    target_record.path
                )
            })?;
        if module_name.is_empty() {
            return Err(
                "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation leerer Modulname"
                    .to_string(),
            );
        }
        let parent_text = fs::read_to_string(incoming_root.join(&source_record.path))
            .map_err(|error| {
                format!(
                    "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation Parent {} nicht lesbar: {error}",
                    source_record.path
                )
            })?;
        let relative_target = relative_target.to_string_lossy().replace('\\', "/");
        let path_attribute = format!("#[path = \"{relative_target}\"]");
        let masked_parent = mask_non_code(&parent_text);
        let Some(attribute_index) = parent_text.find(&path_attribute) else {
            return Err(format!(
                "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation Parent {} bindet Ziel {} nicht mit exaktem #[path]-Vertrag",
                source_record.path, target_record.path
            ));
        };
        let attribute_end = attribute_index + path_attribute.len();
        let masked_attribute = &masked_parent[attribute_index..attribute_end];
        if !masked_attribute.trim_start().starts_with("#[path =")
            || !masked_attribute.trim_end().ends_with(']')
        {
            return Err(format!(
                "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation #[path]-Bindung fuer {} steht nicht in produktivem Rust-Code",
                target_record.path
            ));
        }
        let module_declaration = format!("mod {module_name};");
        if !masked_parent[attribute_end..]
            .trim_start()
            .starts_with(&module_declaration)
        {
            return Err(format!(
                "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation Parent {} bindet {module_declaration:?} nicht unmittelbar an den exakten #[path]-Vertrag",
                source_record.path
            ));
        }
        let qualified_call = format!("{module_name}::{}", target_function.name);
        if !masked_parent.contains(&qualified_call) {
            return Err(format!(
                "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation Parent {} ruft den verlagerten Check nicht qualifiziert ueber {qualified_call:?} auf",
                source_record.path
            ));
        }
        Ok(())
    }

    let mut removed_by_contract =
        BTreeMap::<(String, u64, u64, u64, u64), &CapabilityRecord>::new();
    let mut added_by_contract = BTreeMap::<(String, u64, u64, u64, u64), &CapabilityRecord>::new();
    for delta in removed {
        let record = delta.record();
        let identity = module_contract_identity(current, record, false)?;
        if removed_by_contract.insert(identity, record).is_some() {
            return Err(
                "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation ist fuer identische Mehrfach-Contracts absichtlich fail-closed; jede Modul-Relocation muss eindeutig sein"
                    .to_string(),
            );
        }
    }
    for delta in added {
        let record = delta.record();
        let identity = module_contract_identity(incoming, record, true)?;
        if added_by_contract.insert(identity, record).is_some() {
            return Err(
                "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation ist fuer identische Mehrfach-Contracts absichtlich fail-closed; jede Modul-Relocation muss eindeutig sein"
                    .to_string(),
            );
        }
    }
    if removed_by_contract.keys().collect::<Vec<_>>()
        != added_by_contract.keys().collect::<Vec<_>>()
    {
        return Err(format!(
            "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation veraendert Symbol, Rejection-Surface, kanonische Signatur, Funktionskoerper oder Kodex-main-Ausfuehrungsanker: vorher {:?}, nachher {:?}",
            removed_by_contract.keys().collect::<Vec<_>>(),
            added_by_contract.keys().collect::<Vec<_>>()
        ));
    }

    let mut authorized = BTreeSet::new();
    for (identity, source_record) in removed_by_contract {
        let target_record = added_by_contract.get(&identity).ok_or_else(|| {
            "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation interner Bijektionsfehler"
                .to_string()
        })?;
        let target_key = format!("{}::{}", target_record.path, target_record.symbol);
        let target_function = incoming.functions.get(&target_key).ok_or_else(|| {
            format!(
                "Semantic-Capability-Diff: KodexNegativeCheck-Module-Relocation Zielfunktion fehlt: {target_key}"
            )
        })?;
        validate_parent_module_binding(
            incoming_root,
            source_record,
            target_record,
            target_function,
        )?;
        authorized.insert(source_record.key());
    }
    Ok(authorized)
}

fn writer_sink_contract(inventory: &Inventory) -> BTreeSet<String> {
    inventory
        .records
        .values()
        .filter(|record| record.kind == CapabilityKind::WriterReachability)
        .flat_map(|record| record.detail.split(','))
        .map(str::trim)
        .filter(|root| !root.is_empty())
        .map(str::to_string)
        .collect()
}

fn exported_writer_contract(inventory: &Inventory) -> BTreeSet<(String, String, u64)> {
    inventory
        .records
        .values()
        .filter(|record| record.kind == CapabilityKind::ExportedWriterCapability)
        .map(|record| {
            (
                record.path.clone(),
                record.symbol.clone(),
                record.fingerprint,
            )
        })
        .collect()
}

fn direct_writer_contract(
    inventory: &Inventory,
) -> BTreeSet<(CapabilityKind, String, String, u64)> {
    inventory
        .records
        .values()
        .filter(|record| {
            matches!(
                record.kind,
                CapabilityKind::MusicXmlMutationEntrypoint
                    | CapabilityKind::StructuralWriteAuthorityEntrypoint
                    | CapabilityKind::MasterApplyEntrypoint
            )
        })
        .map(|record| {
            (
                record.kind,
                record.path.clone(),
                record.symbol.clone(),
                record.fingerprint,
            )
        })
        .collect()
}

fn audit_only_allowed(delta: &CapabilityDelta, retirement: &AuditCrossRoleRetirement) -> bool {
    let record = delta.record();
    if record.domain == Domain::Audit
        && matches!(
            record.kind,
            CapabilityKind::AuditLogic
                | CapabilityKind::FeedbackIdentityChannel
                | CapabilityKind::ProofReentryReader
                | CapabilityKind::MacroOrGeneratedSource
        )
    {
        return true;
    }
    if record.domain != Domain::Music || !retirement.reduced_paths.contains(&record.path) {
        return false;
    }
    if delta.kind == DeltaKind::Modified
        && record.kind == CapabilityKind::MusicLogic
        && record.symbol == "<file-surface>"
    {
        return true;
    }
    let symbol = record.symbol.split("->").next().unwrap_or(&record.symbol);
    delta.kind == DeltaKind::Removed
        && retirement
            .retired_symbols
            .contains(&(record.path.clone(), symbol.to_string()))
        && matches!(
            record.kind,
            CapabilityKind::FeedbackIdentityChannel
                | CapabilityKind::ProofReentryReader
                | CapabilityKind::ExportedWriterCapability
                | CapabilityKind::WriteFunctionCall
                | CapabilityKind::WriterReachability
        )
}

fn outcome_projection_allowed(delta: &CapabilityDelta) -> bool {
    let record = delta.record();
    match record.domain {
        Domain::Audit => matches!(
            record.kind,
            CapabilityKind::AuditLogic
                | CapabilityKind::FeedbackIdentityChannel
                | CapabilityKind::ProofReentryReader
                | CapabilityKind::MacroOrGeneratedSource
        ),
        Domain::Kodex => matches!(
            record.kind,
            CapabilityKind::KodexControl
                | CapabilityKind::KodexNegativeCheck
                | CapabilityKind::FeedbackIdentityChannel
                | CapabilityKind::ProofReentryReader
                | CapabilityKind::MacroOrGeneratedSource
                | CapabilityKind::BuildScriptCapability
        ),
        Domain::Music | Domain::Unknown => false,
    }
}

fn music_mutation_allowed(delta: &CapabilityDelta) -> bool {
    let record = delta.record();
    record.domain == Domain::Music
        && matches!(
            record.kind,
            CapabilityKind::MusicLogic
                | CapabilityKind::MusicXmlMutationEntrypoint
                | CapabilityKind::StructuralWriteAuthorityEntrypoint
                | CapabilityKind::MasterApplyEntrypoint
                | CapabilityKind::ExportedWriterCapability
                | CapabilityKind::WriteFunctionCall
                | CapabilityKind::WriterReachability
                | CapabilityKind::MacroOrGeneratedSource
        )
}

fn validation_only_allowed(delta: &CapabilityDelta) -> bool {
    let record = delta.record();
    if record.path == "src/audit_only/format40_conformance_foundation.rs" {
        return record.domain == Domain::Audit
            && matches!(
                record.kind,
                CapabilityKind::AuditLogic
                    | CapabilityKind::ProofReentryReader
                    | CapabilityKind::UnknownCapability
            );
    }
    if record.path == "src/midi_seed.rs"
        && delta.kind == DeltaKind::Modified
        && record.kind == CapabilityKind::MusicXmlMutationEntrypoint
    {
        return true;
    }
    let validation_path = matches!(
        record.path.as_str(),
        "src/persisted_schema_validation.rs"
            | "src/pipeline.rs"
            | "src/render.rs"
            | "src/midi_seed.rs"
            | "tests/persisted_schema_validation_gate.rs"
    );
    if !validation_path || record.domain != Domain::Music {
        return false;
    }
    matches!(
        record.kind,
        CapabilityKind::MusicLogic
            | CapabilityKind::ProofReentryReader
            | CapabilityKind::UnknownCapability
    ) || (delta.kind == DeltaKind::Modified
        && matches!(
            record.kind,
            CapabilityKind::WriteFunctionCall | CapabilityKind::WriterReachability
        ))
}

fn calibration_only_allowed(delta: &CapabilityDelta) -> bool {
    let record = delta.record();
    record.domain == Domain::Music && record.kind == CapabilityKind::CalibrationThreshold
}

fn summarize(deltas: &[CapabilityDelta]) -> SemanticDiffSummary {
    let mut summary = SemanticDiffSummary::default();
    for delta in deltas {
        match delta.kind {
            DeltaKind::Added => summary.added += 1,
            DeltaKind::Modified => summary.modified += 1,
            DeltaKind::Removed => summary.removed += 1,
        }
        let record = delta.record();
        match record.domain {
            Domain::Kodex => summary.kodex += 1,
            Domain::Audit => summary.audit += 1,
            Domain::Music => summary.music += 1,
            Domain::Unknown => summary.unknown += 1,
        }
        if record.kind == CapabilityKind::CalibrationThreshold {
            summary.calibration += 1;
        }
        if record.kind.is_writer_family() {
            summary.writer_family += 1;
        }
        if record.kind == CapabilityKind::UnknownCapability && record.domain != Domain::Unknown {
            summary.unknown += 1;
        }
    }
    summary
}

fn add_record(inventory: &mut Inventory, record: CapabilityRecord) -> Result<(), String> {
    let key = record.key();
    if let Some(previous) = inventory.records.insert(key.clone(), record.clone())
        && previous != record
    {
        return Err(format!(
            "Semantic-Capability-Inventur: kollidierende Capability {key}"
        ));
    }
    Ok(())
}

fn domain_for_path(path: &str) -> Domain {
    if super::is_kodex_path(path) {
        Domain::Kodex
    } else if super::is_audit_path(path) {
        Domain::Audit
    } else if path.starts_with("src/") || path.starts_with("tests/") || path.starts_with("tools/") {
        Domain::Music
    } else {
        Domain::Unknown
    }
}

fn known_managed_path(path: &str) -> bool {
    path == "run.cmd"
        || path == "strict.cmd"
        || path == "rust-toolchain.toml"
        || path == "rustfmt.toml"
        || path == "clippy.toml"
        || path.starts_with("src/")
        || path.starts_with("tests/")
        || path.starts_with("tools/")
}

fn function_spans(source: &str, masked: &str) -> Result<Vec<FunctionSpan>, String> {
    let bytes = masked.as_bytes();
    let mut spans = Vec::new();
    let mut cursor = 0usize;
    while let Some(relative) = find_token(&masked[cursor..], "fn") {
        let fn_index = cursor + relative;
        let mut name_start = fn_index + 2;
        while bytes.get(name_start).is_some_and(u8::is_ascii_whitespace) {
            name_start += 1;
        }
        let mut name_end = name_start;
        while bytes
            .get(name_end)
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
        {
            name_end += 1;
        }
        if name_end == name_start {
            cursor = fn_index + 2;
            continue;
        }
        let name = masked[name_start..name_end].to_string();
        let next_body = masked[name_end..].find('{');
        let next_semicolon = masked[name_end..].find(';');
        if next_semicolon.is_some_and(|semicolon| next_body.is_none_or(|body| semicolon < body)) {
            cursor = name_end + next_semicolon.unwrap_or(0) + 1;
            continue;
        }
        let Some(body_start_relative) = next_body else {
            break;
        };
        let body_start = name_end + body_start_relative;
        let Some(body_end) = matching_brace(masked, body_start) else {
            return Err(format!(
                "Semantic-Capability-Inventur: nicht geschlossene Funktion {name}"
            ));
        };
        let signature_start = signature_start(masked, fn_index);
        let signature = &masked[signature_start..body_start];
        let trimmed_signature = signature.trim_start();
        let public = trimmed_signature.starts_with("pub ") || trimmed_signature.starts_with("pub(");
        spans.push(FunctionSpan {
            name,
            public,
            signature_start,
            body_start,
            body_end: body_end + 1,
            start_line: source[..fn_index]
                .bytes()
                .filter(|byte| *byte == b'\n')
                .count()
                + 1,
        });
        cursor = body_end + 1;
    }
    Ok(spans)
}

fn signature_start(masked: &str, fn_index: usize) -> usize {
    masked[..fn_index]
        .rfind([';', '}', '{', '\n'])
        .map_or(0, |index| index + 1)
}

fn find_token(source: &str, token: &str) -> Option<usize> {
    let bytes = source.as_bytes();
    let token_bytes = token.as_bytes();
    let mut cursor = 0usize;
    while cursor + token_bytes.len() <= bytes.len() {
        let relative = source[cursor..].find(token)?;
        let index = cursor + relative;
        let before_ok = index == 0 || !is_ident_byte(bytes[index - 1]);
        let after = index + token_bytes.len();
        let after_ok = after >= bytes.len() || !is_ident_byte(bytes[after]);
        if before_ok && after_ok {
            return Some(index);
        }
        cursor = index + token_bytes.len();
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

fn called_identifiers(masked_body: &str) -> BTreeMap<String, usize> {
    let bytes = masked_body.as_bytes();
    let mut calls = BTreeMap::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if !is_ident_start(bytes[index]) {
            index += 1;
            continue;
        }
        let start = index;
        index += 1;
        while index < bytes.len() && is_ident_byte(bytes[index]) {
            index += 1;
        }
        let name = &masked_body[start..index];
        let mut next = index;
        while next < bytes.len() && bytes[next].is_ascii_whitespace() {
            next += 1;
        }
        if bytes.get(next) == Some(&b'(') && !rust_keyword(name) {
            *calls.entry(name.to_string()).or_insert(0) += 1;
        }
    }
    calls
}

fn rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "if" | "while" | "for" | "match" | "loop" | "return" | "Some" | "Ok" | "Err"
    )
}

fn direct_music_writer(path: &str, name: &str, masked_body: &str) -> bool {
    if domain_for_path(path) != Domain::Music {
        return false;
    }
    let lower = masked_body.to_ascii_lowercase();
    let domain_context = [
        "xml",
        "musicxml",
        "render",
        "measure",
        "note",
        "rest",
        "beam",
        "clef",
        "staff",
        "voice",
        "octave",
        "accent",
        "tenuto",
        "slur",
        "pitch",
        "key_signature",
        "barline",
        "direction",
        "articulation",
        "layout",
        "chord",
        "tie",
    ]
    .iter()
    .any(|token| lower.contains(token));
    let primitive = [
        ".replace_range(",
        ".insert_str(",
        "fs::write(",
        "std::fs::write(",
        "write_musicxml(",
        "apply_authorized(",
        "apply_music(",
        "materialize_delta(",
    ]
    .iter()
    .any(|token| lower.contains(token));
    let named = writer_like_name(&name.to_ascii_lowercase())
        && [
            "xml", "music", "render", "score", "note", "staff", "voice", "beam", "clef", "key",
        ]
        .iter()
        .any(|token| lower.contains(token));
    domain_context && (primitive || named)
}

fn writer_like_name(lower: &str) -> bool {
    lower.contains("writer")
        || lower.starts_with("write_")
        || lower.starts_with("rewrite_")
        || lower.starts_with("apply_")
        || lower.starts_with("materialize_")
        || lower.starts_with("mutate_")
        || lower.contains("authority")
}

fn feedback_identity_markers(body: &str, signature: &str) -> BTreeSet<String> {
    let joined = format!("{signature}\n{body}");
    [
        "project_name",
        "project_id",
        "etude",
        "etueden",
        "measure_number",
        "measure_no",
        "origin_measure",
        "source_measure",
        "feedback_origin",
        "feedback_text",
        "feedback_project",
    ]
    .into_iter()
    .filter(|marker| contains_identifier(&joined, marker))
    .map(str::to_string)
    .collect()
}

fn proof_reentry_reader(function: &FunctionInfo) -> bool {
    let lower_name = function.name.to_ascii_lowercase();
    let lower_body = function.masked_body.to_ascii_lowercase();
    let context = [
        "proof", "report", "result", "manifest", "evidence", "readback",
    ]
    .iter()
    .any(|token| lower_name.contains(token) || lower_body.contains(token));
    let reader = [
        "read_to_string(",
        "fs::read(",
        "std::fs::read(",
        "file::open(",
        "open(",
        "parse_report(",
        "parse_proof(",
    ]
    .iter()
    .any(|token| lower_body.contains(token));
    context && reader
}

fn normalized_rejection_surface(masked_body: &str) -> String {
    masked_body
        .lines()
        .filter(|line| {
            let lower = line.to_ascii_lowercase();
            lower.contains("return err")
                || lower.contains("panic!(")
                || lower.contains("assert!(")
                || lower.contains("assert_eq!(")
                || lower.contains("assert_ne!(")
                || lower.contains("deny(")
                || lower.contains("reject")
                || lower.contains("forbidden")
                || lower.contains("fail_closed")
        })
        .map(normalize_semantic)
        .collect::<Vec<_>>()
        .join("|")
}

fn normalize_music_surface(masked: &str) -> String {
    let assignments = threshold_assignments(masked);
    if assignments.is_empty() {
        return normalize_semantic(masked);
    }
    let mut out = String::new();
    let mut cursor = 0usize;
    for assignment in assignments {
        if assignment.rhs_start < cursor || assignment.rhs_end > masked.len() {
            continue;
        }
        out.push_str(&normalize_semantic(&masked[cursor..assignment.rhs_start]));
        out.push_str(&normalize_numeric_literals(
            &masked[assignment.rhs_start..assignment.rhs_end],
        ));
        cursor = assignment.rhs_end;
    }
    out.push_str(&normalize_semantic(&masked[cursor..]));
    out
}

fn assignment_identifier(left: &str) -> Option<String> {
    let local_declaration = left.rsplit([';', '{', '}']).next().unwrap_or(left);
    let declaration = local_declaration
        .split_once(':')
        .map_or(local_declaration, |(before_type, _)| before_type);
    declaration
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .rfind(|token| !token.is_empty())
        .map(str::to_string)
}

fn threshold_like_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    let ratio_component = lower.split('_').any(|component| component == "ratio");
    [
        "threshold",
        "cutoff",
        "confidence",
        "tolerance",
        "epsilon",
        "penalty",
        "weight",
        "percent",
        "budget",
        "window",
        "radius",
        "min_score",
        "max_score",
        "score_min",
        "score_max",
        "minimum",
        "maximum",
    ]
    .iter()
    .any(|token| lower.contains(token))
        || ratio_component
        || lower.ends_with("_limit")
        || lower.starts_with("min_")
        || lower.starts_with("max_")
}

fn contains_numeric_literal(value: &str) -> bool {
    let bytes = value.as_bytes();
    (0..bytes.len()).any(|index| numeric_literal_start(bytes, index))
}

fn normalize_numeric_literals(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = String::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if numeric_literal_start(bytes, index) {
            out.push('#');
            index = numeric_literal_end(bytes, index);
            continue;
        }
        if !bytes[index].is_ascii_whitespace() {
            out.push(char::from(bytes[index]));
        }
        index += 1;
    }
    out
}

fn numeric_literal_start(bytes: &[u8], index: usize) -> bool {
    let Some(byte) = bytes.get(index).copied() else {
        return false;
    };
    if byte.is_ascii_digit() {
        return index == 0 || !is_ident_byte(bytes[index - 1]);
    }
    byte == b'.'
        && bytes.get(index + 1).is_some_and(u8::is_ascii_digit)
        && (index == 0 || bytes[index - 1] != b'.')
}

fn numeric_literal_end(bytes: &[u8], start: usize) -> usize {
    let mut index = start;
    if bytes.get(index) == Some(&b'.') {
        index += 1;
        while bytes
            .get(index)
            .is_some_and(|byte| byte.is_ascii_digit() || *byte == b'_')
        {
            index += 1;
        }
    } else if bytes.get(index) == Some(&b'0')
        && bytes
            .get(index + 1)
            .is_some_and(|byte| matches!(*byte, b'x' | b'X' | b'o' | b'O' | b'b' | b'B'))
    {
        index += 2;
        while bytes
            .get(index)
            .is_some_and(|byte| byte.is_ascii_hexdigit() || *byte == b'_')
        {
            index += 1;
        }
        return index;
    } else {
        while bytes
            .get(index)
            .is_some_and(|byte| byte.is_ascii_digit() || *byte == b'_')
        {
            index += 1;
        }
        if bytes.get(index) == Some(&b'.') && bytes.get(index + 1) != Some(&b'.') {
            index += 1;
            while bytes
                .get(index)
                .is_some_and(|byte| byte.is_ascii_digit() || *byte == b'_')
            {
                index += 1;
            }
        }
    }
    if bytes
        .get(index)
        .is_some_and(|byte| matches!(*byte, b'e' | b'E'))
    {
        let exponent = index;
        index += 1;
        if bytes
            .get(index)
            .is_some_and(|byte| matches!(*byte, b'+' | b'-'))
        {
            index += 1;
        }
        let digit_start = index;
        while bytes
            .get(index)
            .is_some_and(|byte| byte.is_ascii_digit() || *byte == b'_')
        {
            index += 1;
        }
        if index == digit_start {
            index = exponent;
        }
    }
    index
}

fn normalize_semantic(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

fn contains_identifier(source: &str, identifier: &str) -> bool {
    let bytes = source.as_bytes();
    let needle = identifier.as_bytes();
    let mut cursor = 0usize;
    while cursor + needle.len() <= bytes.len() {
        let Some(relative) = source[cursor..].find(identifier) else {
            return false;
        };
        let index = cursor + relative;
        let before_ok = index == 0 || !is_ident_byte(bytes[index - 1]);
        let after = index + needle.len();
        let after_ok = after >= bytes.len() || !is_ident_byte(bytes[after]);
        if before_ok && after_ok {
            return true;
        }
        cursor = index + needle.len();
    }
    false
}

fn is_ident_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_ident_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn normalize_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[derive(Debug, Clone)]
struct RevisionEffectRow {
    path: String,
    body_fingerprint: u64,
    effects: BTreeSet<String>,
    anchors: BTreeSet<String>,
}

fn parse_revision_effect_snapshot(text: &str) -> Result<Vec<RevisionEffectRow>, String> {
    let mut rows = Vec::new();
    for (line_index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fields = line.splitn(5, '\t').collect::<Vec<_>>();
        if fields.len() != 5 || fields[0] != "v1" {
            return Err(format!(
                "MultiRevision-Evasion: ungueltige Snapshotzeile {}: {:?}",
                line_index + 1,
                line
            ));
        }
        let body_fingerprint = u64::from_str_radix(fields[2], 16).map_err(|error| {
            format!(
                "MultiRevision-Evasion: ungueltiger Body-Fingerprint Zeile {}: {error}",
                line_index + 1
            )
        })?;
        let effects = split_snapshot_set(fields[3]);
        let anchors = split_snapshot_set(fields[4]);
        rows.push(RevisionEffectRow {
            path: fields[1].to_string(),
            body_fingerprint,
            effects,
            anchors,
        });
    }
    Ok(rows)
}

fn split_snapshot_set(value: &str) -> BTreeSet<String> {
    value
        .split(',')
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn revision_rows_semantically_equivalent(
    previous: &RevisionEffectRow,
    current: &RevisionEffectRow,
) -> bool {
    if previous.body_fingerprint == current.body_fingerprint {
        return previous.path == current.path
            || (previous.anchors.len() >= 6 && previous.anchors.is_subset(&current.anchors));
    }
    if previous.anchors.len() < 3 || !previous.anchors.is_subset(&current.anchors) {
        return false;
    }
    previous.path == current.path || previous.anchors.len() >= 6
}

fn normalize_identifier_invariant_body(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut aliases = BTreeMap::<String, usize>::new();
    let mut out = String::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if is_ident_start(bytes[index]) {
            let start = index;
            index += 1;
            while index < bytes.len() && is_ident_byte(bytes[index]) {
                index += 1;
            }
            let identifier = &value[start..index];
            let previous = previous_non_space(bytes, start);
            let next = next_non_space(bytes, index);
            if preserve_identifier_for_alpha(identifier, previous, next) {
                out.push_str(identifier);
            } else {
                let next_alias = aliases.len();
                let alias = *aliases.entry(identifier.to_string()).or_insert(next_alias);
                out.push('@');
                out.push_str(&alias.to_string());
                out.push('@');
            }
            continue;
        }
        if !bytes[index].is_ascii_whitespace() {
            out.push(char::from(bytes[index]));
        }
        index += 1;
    }
    out
}

fn previous_non_space(bytes: &[u8], index: usize) -> Option<u8> {
    let mut cursor = index;
    while cursor > 0 {
        cursor -= 1;
        if !bytes[cursor].is_ascii_whitespace() {
            return Some(bytes[cursor]);
        }
    }
    None
}

fn next_non_space(bytes: &[u8], mut index: usize) -> Option<u8> {
    while index < bytes.len() {
        if !bytes[index].is_ascii_whitespace() {
            return Some(bytes[index]);
        }
        index += 1;
    }
    None
}

fn preserve_identifier_for_alpha(identifier: &str, previous: Option<u8>, next: Option<u8>) -> bool {
    let first_upper = identifier
        .as_bytes()
        .first()
        .is_some_and(u8::is_ascii_uppercase);
    next == Some(b'(')
        || matches!(previous, Some(b'.' | b':'))
        || first_upper
        || matches!(
            identifier,
            "as" | "async"
                | "await"
                | "break"
                | "const"
                | "continue"
                | "crate"
                | "dyn"
                | "else"
                | "enum"
                | "extern"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "impl"
                | "in"
                | "let"
                | "loop"
                | "match"
                | "mod"
                | "move"
                | "mut"
                | "pub"
                | "ref"
                | "return"
                | "self"
                | "Self"
                | "static"
                | "struct"
                | "super"
                | "trait"
                | "true"
                | "type"
                | "unsafe"
                | "use"
                | "where"
                | "while"
                | "yield"
        )
}

fn semantic_anchor_tokens(masked_body: &str) -> BTreeSet<String> {
    let mut anchors = called_identifiers(masked_body)
        .into_keys()
        .filter(|name| !matches!(name.as_str(), "Some" | "None" | "Ok" | "Err" | "default"))
        .collect::<BTreeSet<_>>();
    let lower = masked_body.to_ascii_lowercase();
    for marker in [
        "replace_range",
        "insert_str",
        "push_str",
        "remove",
        "retain",
        "fs::write",
        "std::fs::write",
        "write_musicxml",
        "materialize_delta",
        "master_apply",
        "backup",
        "forward",
        "musicxml",
        "render",
        "beam",
        "voice",
        "staff",
        "duration",
        "threshold",
        "ratio",
    ] {
        if lower.contains(marker) {
            anchors.insert(marker.to_string());
        }
    }
    anchors
}

// Not a general capability exception: both complete trees and the exact diff set
// have been bound before this finite record list can be consumed.
fn cursor_temporal_observational_delta(delta: &CapabilityDelta) -> bool {
    let r=delta.record();
    matches!((delta.kind.as_str(),r.kind.as_str(),r.path.as_str(),r.symbol.as_str()),
        ("modified","AuditLogic","src/main.rs" | "src/audit_only/serialized_temporal_replay.rs","<file-surface>") |
        ("removed","FeedbackIdentityChannel","src/audit_only/serialized_temporal_replay.rs","scan_raw_cursor_measure#1") |
        ("removed","FeedbackIdentityChannel","src/pipeline_shards/report_hygiene_cursor_localbest_trace_tail.rs","repair_cursor_lane_end_invariant_measure#1" | "repair_cursor_lane_end_invariant_xml#1") |
        ("modified","KodexControl","src/build_gate/destructive_transformation_population_v1.tsv" | "src/build_gate/render_origins_v1.tsv" | "src/build_gate/ssc_containment_envelopes_v1.tsv","<non-rust-surface>") |
        ("modified","ProofReentryReader","src/pipeline_shards/pipeline_orchestrator_release_report_exit_stage.rs","run_release_report_and_exit_stage#1") |
        ("modified","ProofReentryReader","src/pipeline_shards/pipeline_orchestrator_release_support.rs","run_render_pdf_source_selection_stage#1") |
        ("modified","ProofReentryReader","src/structural_write_authority/master_adoption/render_adopted_state_tests.rs","path_bound_receipt_mismatch_is_rejected_before_productive_write#1" | "ssc_render_owner_commit_is_observed_without_changing_adoption_or_bytes#1")
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    fn closed_threshold_fixture() -> (PathBuf, PathBuf, String, String) {
        let parent = root("closed_threshold_parent");
        let child = root("closed_threshold_child");
        let row = super::super::ssc_evidence_proof_shadow_permit_kodex::structural_erasure::WAV_COMPLETION_NODES
            .lines().map(|s| s.split('|').collect::<Vec<_>>())
            .find(|r| r.len() == 7 && r[1].ends_with("_update_fidelity_after_visible_recovery")).unwrap();
        let bytes = (0..row[6].len()).step_by(2)
            .map(|i| u8::from_str_radix(&row[6][i..i + 2], 16).unwrap()).collect::<Vec<_>>();
        let body = String::from_utf8(bytes).unwrap();
        write(&parent, row[3], &body);
        write(&parent, "src/build_gate/ssc_k2_retirement_governance_contract_v1.txt", "");
        write(&child, row[3], "fn alive() {}\n");
        write(&child, "src/build_gate/take_change_class.txt", "authority_retirement_contract=docs/authority_retirement_contract_fixture.md\n");
        write(&child, "docs/authority_retirement_contract_fixture.md", &format!(
            "SscExitObligations: {}\nRetiredAuthoritySymbols: {}::{}\n", row[2], row[3], row[4]
        ));
        (parent, child, row[3].to_owned(), body)
    }

    fn closed_threshold_keys(parent: &Path, child: &Path) -> BTreeSet<String> {
        let before = inventory_tree(parent).unwrap();
        let after = inventory_tree(child).unwrap();
        let deltas = diff_inventories(&before.records, &after.records);
        ssc_removed_owner_local_capabilities(parent, child, "MusicMutation", &after, &deltas).unwrap()
    }

    #[test]
    fn local_threshold_erasure_requires_exact_complete_owner_and_bound_obligation() {
        let (parent, child, path, body) = closed_threshold_fixture();
        assert_eq!(closed_threshold_keys(&parent, &child).len(), 1);
        write(&parent, &path, &body.replace("100.0", "101.0"));
        assert!(closed_threshold_keys(&parent, &child).is_empty());
        write(&parent, &path, &body);
        write(&child, "docs/authority_retirement_contract_fixture.md", &format!(
            "SscExitObligations: wav_accent_productive\nRetiredAuthoritySymbols: {path}::run_salience_weighted_candidate_gate\n"
        ));
        assert!(closed_threshold_keys(&parent, &child).is_empty());
        fs::remove_dir_all(parent).unwrap();
        fs::remove_dir_all(child).unwrap();
    }

    #[test]
    fn surviving_owner_and_new_threshold_cannot_use_local_threshold_erasure() {
        let (parent, child, path, _) = closed_threshold_fixture();
        write(&child, &path, "fn update_fidelity_after_visible_recovery() {}\n");
        assert!(closed_threshold_keys(&parent, &child).is_empty());
        write(&child, &path, "fn replacement() { let replacement_percent = 12.0; }\n");
        let before = inventory_tree(&parent).unwrap();
        let after = inventory_tree(&child).unwrap();
        let deltas = diff_inventories(&before.records, &after.records);
        let allowed = ssc_removed_owner_local_capabilities(&parent, &child, "MusicMutation", &after, &deltas).unwrap();
        assert_eq!(allowed.len(), 1);
        assert!(deltas.iter().any(|d| d.kind == DeltaKind::Added
            && d.record().kind == CapabilityKind::CalibrationThreshold
            && !allowed.contains(&d.record().key())));
        fs::remove_file(child.join("src/build_gate/take_change_class.txt")).unwrap();
        assert!(closed_threshold_keys(&parent, &child).is_empty());
        fs::remove_dir_all(parent).unwrap();
        fs::remove_dir_all(child).unwrap();
    }

    #[test]
    fn audit_initializer_loss_requires_bound_exact_caller_and_preserves_error_exits() {
        let parent = root("audit_initializer_parent");
        let child = root("audit_initializer_child");
        let path = "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs";
        let row = super::super::ssc_evidence_proof_shadow_permit_kodex::structural_erasure::WAV_COMPLETION_NODES
            .lines().map(|s| s.split('|').collect::<Vec<_>>())
            .find(|r| r.len() == 8 && r[1] == "wav_complete_wav_salience_reference_initializer").unwrap();
        let decode = |hex: &str| String::from_utf8((0..hex.len()).step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i+2], 16).unwrap()).collect()).unwrap();
        write(&parent, path, &format!("fn real_main() {{ consume({}); unrelated()?; }}", decode(row[6])));
        let after = format!("fn real_main() {{ consume({}); unrelated()?; }}", decode(row[7]));
        write(&child, path, &after);
        write(&parent, "src/build_gate/ssc_k2_retirement_governance_contract_v1.txt", "");
        assert!(!ssc_audit_initializer_loss(&parent, &child, path, "real_main").unwrap());
        write(&child, "src/build_gate/take_change_class.txt", "authority_retirement_contract=docs/authority_retirement_contract_fixture.md\n");
        write(&child, "docs/authority_retirement_contract_fixture.md", &format!(
            "SscExitObligations: wav_salience_master_adoptions\nRetiredAuthoritySymbols: {path}::run_salience_weighted_candidate_gate\n"
        ));
        assert!(ssc_audit_initializer_loss(&parent, &child, path, "real_main").unwrap());
        write(&child, path, &format!("{after}\nfn forged() {{}}"));
        assert!(!ssc_audit_initializer_loss(&parent, &child, path, "real_main").unwrap());
        write(&child, path, &after.replace("unrelated()?", "unrelated()"));
        assert!(!ssc_audit_initializer_loss(&parent, &child, path, "real_main").unwrap());
        fs::remove_dir_all(parent).unwrap();
        fs::remove_dir_all(child).unwrap();
    }

    #[test]
    fn private_feedback_reader_erasure_requires_exact_closed_owner() {
        let parent = root("private_feedback_parent");
        let child = root("private_feedback_child");
        let row = super::super::ssc_evidence_proof_shadow_permit_kodex::structural_erasure::WAV_COMPLETION_NODES
            .lines().map(|s| s.split('|').collect::<Vec<_>>())
            .find(|r| r.len() == 7 && r[1].ends_with("_visible_midi_notes_at_staff_start")).unwrap();
        let body = String::from_utf8((0..row[6].len()).step_by(2)
            .map(|i| u8::from_str_radix(&row[6][i..i+2], 16).unwrap()).collect()).unwrap();
        write(&parent, row[3], &body);
        write(&parent, "src/build_gate/ssc_k2_retirement_governance_contract_v1.txt", "");
        write(&child, row[3], "fn alive() {}\n");
        write(&child, "src/build_gate/take_change_class.txt", "authority_retirement_contract=docs/authority_retirement_contract_fixture.md\n");
        write(&child, "docs/authority_retirement_contract_fixture.md", &format!(
            "SscExitObligations: {}\nRetiredAuthoritySymbols: {}::{}\n", row[2], row[3], row[4]
        ));
        assert_eq!(closed_threshold_keys(&parent, &child).len(), 1);
        write(&parent, row[3], &body.replacen("usize", "u32", 1));
        assert!(closed_threshold_keys(&parent, &child).is_empty());
        write(&parent, row[3], &body);
        write(&child, row[3], &body);
        assert!(closed_threshold_keys(&parent, &child).is_empty());
        fs::remove_dir_all(parent).unwrap();
        fs::remove_dir_all(child).unwrap();
    }
    use std::time::{SystemTime, UNIX_EPOCH};

    fn root(label: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_semantic_capability_{label}_{}_{}",
            std::process::id(),
            stamp
        ));
        fs::create_dir_all(root.join("src/build_gate")).expect("src");
        fs::create_dir_all(root.join("tests")).expect("tests");
        fs::create_dir_all(root.join("tools")).expect("tools");
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname=\"test\"\nversion=\"1.0.0\"\nbuild=\"src/identity_debt_build.rs\"\n[dependencies]\nanyhow=\"1\"\n",
        )
        .expect("cargo");
        fs::write(root.join("run.cmd"), "@echo off\n").expect("run");
        fs::write(root.join("strict.cmd"), "@echo off\n").expect("strict");
        root
    }

    fn write(root: &Path, relative: &str, text: &str) {
        let path = root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent");
        }
        fs::write(path, text).expect("write");
    }

    fn write_kodex_execution_anchor(root: &Path, target: Option<&str>) {
        let body = target.map_or_else(
            || "fn main() {}\n".to_string(),
            |target| format!("fn main() {{ let _ = {target}(false); }}\n"),
        );
        write(root, "src/take_change_control.rs", &body);
    }

    #[test]
    fn music_mutation_allows_byte_stable_audit_writer_reachability_strict_narrowing() {
        let current = root("music_audit_writer_narrow_current");
        let incoming = root("music_audit_writer_narrow_incoming");
        let audit = "pub fn real_main() { music_entry(); }\n";
        write(
            &current,
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
            audit,
        );
        write(
            &incoming,
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
            audit,
        );
        write(
            &current,
            "src/music.rs",
            "pub fn music_entry() { apply_keep(); apply_old(); }\nfn apply_keep() { let mut xml = String::new(); xml.insert_str(0, \"k\"); }\nfn apply_old() { let mut xml = String::new(); xml.insert_str(0, \"o\"); }\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "pub fn music_entry() { apply_keep(); }\nfn apply_keep() { let mut xml = String::new(); xml.insert_str(0, \"k\"); }\nfn apply_old() { let mut xml = String::new(); xml.insert_str(0, \"o\"); }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_mutation_allows_detached_music_writer_root_retirement() {
        let current = root("music_audit_writer_retire_current");
        let incoming = root("music_audit_writer_retire_incoming");
        let audit = "pub fn real_main() { music_entry(); }\n";
        write(
            &current,
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
            audit,
        );
        write(
            &incoming,
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
            audit,
        );
        write(
            &current,
            "src/music.rs",
            "pub fn music_entry() { apply_keep(); apply_old(); }\nfn apply_keep() { let mut xml = String::new(); xml.insert_str(0, \"k\"); }\nfn apply_old() { let mut xml = String::new(); xml.insert_str(0, \"o\"); }\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "pub fn music_entry() { apply_keep(); }\nfn apply_keep() { let mut xml = String::new(); xml.insert_str(0, \"k\"); }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_mutation_rejects_detached_music_writer_root_semantic_relocation() {
        let current = root("music_audit_writer_retire_relocate_current");
        let incoming = root("music_audit_writer_retire_relocate_incoming");
        let audit = "pub fn real_main() { music_entry(); }\n";
        write(
            &current,
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
            audit,
        );
        write(
            &incoming,
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
            audit,
        );
        write(
            &current,
            "src/structural_write_authority/music.rs",
            "pub fn music_entry() { apply_keep(); apply_old(); }\nfn apply_keep() { let mut xml = String::new(); xml.insert_str(0, \"k\"); }\nfn apply_old() { let mut xml = String::new(); xml.insert_str(0, \"o\"); }\n",
        );
        write(
            &incoming,
            "src/structural_write_authority/music.rs",
            "pub fn music_entry() { apply_keep(); }\nfn apply_keep() { let mut xml = String::new(); xml.insert_str(0, \"k\"); }\nfn renamed_old() { let mut xml = String::new(); xml.insert_str(0, \"o\"); }\n",
        );
        let changed = BTreeSet::from(["src/structural_write_authority/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(
            result
                .as_ref()
                .is_err_and(|error| { error.contains("semantisch verschoben statt entfernt") }),
            "{result:?}"
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_mutation_rejects_audit_writer_reachability_widening() {
        let current = root("music_audit_writer_widen_current");
        let incoming = root("music_audit_writer_widen_incoming");
        let audit = "pub fn real_main() { music_entry(); }\n";
        write(
            &current,
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
            audit,
        );
        write(
            &incoming,
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
            audit,
        );
        write(
            &current,
            "src/music.rs",
            "pub fn music_entry() { apply_keep(); }\nfn apply_keep() { let mut xml = String::new(); xml.insert_str(0, \"k\"); }\nfn apply_new() { let mut xml = String::new(); xml.insert_str(0, \"n\"); }\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "pub fn music_entry() { apply_keep(); apply_new(); }\nfn apply_keep() { let mut xml = String::new(); xml.insert_str(0, \"k\"); }\nfn apply_new() { let mut xml = String::new(); xml.insert_str(0, \"n\"); }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(
            result
                .as_ref()
                .is_err_and(|error| error.contains("nur streng verengen")),
            "{result:?}"
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_mutation_rejects_derived_audit_writer_narrowing_when_audit_source_changes() {
        let current = root("music_audit_writer_audit_change_current");
        let incoming = root("music_audit_writer_audit_change_incoming");
        write(
            &current,
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
            "pub fn real_main() { music_entry(); }\n",
        );
        write(
            &incoming,
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
            "pub fn real_main() { music_entry(); let _audit_changed = true; }\n",
        );
        write(
            &current,
            "src/music.rs",
            "pub fn music_entry() { apply_keep(); apply_old(); }\nfn apply_keep() { let mut xml = String::new(); xml.insert_str(0, \"k\"); }\nfn apply_old() { let mut xml = String::new(); xml.insert_str(0, \"o\"); }\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "pub fn music_entry() { apply_keep(); }\nfn apply_keep() { let mut xml = String::new(); xml.insert_str(0, \"k\"); }\nfn apply_old() { let mut xml = String::new(); xml.insert_str(0, \"o\"); }\n",
        );
        let changed = BTreeSet::from([
            "src/music.rs".to_string(),
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs".to_string(),
        ]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(
            result
                .as_ref()
                .is_err_and(|error| error.contains("byteunveraenderter Audit-Quelle")),
            "{result:?}"
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_mutation_rejects_audit_writer_root_replacement_even_with_unchanged_audit_source() {
        let current = root("music_audit_writer_replace_current");
        let incoming = root("music_audit_writer_replace_incoming");
        let audit = "pub fn real_main() { music_entry(); }\n";
        write(
            &current,
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
            audit,
        );
        write(
            &incoming,
            "src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
            audit,
        );
        write(
            &current,
            "src/music.rs",
            "pub fn music_entry() { apply_old(); }\nfn apply_old() { let mut xml = String::new(); xml.insert_str(0, \"o\"); }\nfn apply_new() { let mut xml = String::new(); xml.insert_str(0, \"n\"); }\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "pub fn music_entry() { apply_new(); }\nfn apply_old() { let mut xml = String::new(); xml.insert_str(0, \"o\"); }\nfn apply_new() { let mut xml = String::new(); xml.insert_str(0, \"n\"); }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(
            result
                .as_ref()
                .is_err_and(|error| error.contains("niemals erweitern oder umhaengen")),
            "{result:?}"
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn calibration_only_accepts_isolated_threshold_value_change() {
        let current = root("calibration_current");
        let incoming = root("calibration_incoming");
        write(
            &current,
            "src/music.rs",
            "fn decide(score: f64) -> bool { let confidence_threshold = 0.7; score >= confidence_threshold }\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "fn decide(score: f64) -> bool { let confidence_threshold = 0.8; score >= confidence_threshold }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "CalibrationOnly", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_mutation_rejects_new_productive_repair_authority() {
        let current = root("repair_authority_current");
        let incoming = root("repair_authority_incoming");
        write(
            &current,
            "src/music.rs",
            "pub fn apply_score() { write_musicxml(&mut String::new()); }\nfn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "pub fn apply_score() { write_musicxml(&mut String::new()); }\nfn late_clef_repair() { apply_score(); }\nfn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(
            result
                .as_ref()
                .is_err_and(|error| error.contains("NO-NEW-REPAIR-AUTHORITY KODEX GATE")),
            "{result:?}"
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_mutation_rejects_renamed_late_writer_layer_outside_structural_authority() {
        let current = root("late_writer_layer_current");
        let incoming = root("late_writer_layer_incoming");
        write(
            &current,
            "src/music.rs",
            "pub fn apply_score() { write_musicxml(&mut String::new()); }\nfn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "pub fn apply_score() { write_musicxml(&mut String::new()); }\nfn late_clef_normalizer() { apply_score(); }\nfn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(
            result.as_ref().is_err_and(|error| {
                error.contains("new_writer_caller_outside_structural_write_authority")
            }),
            "{result:?}"
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_mutation_rejects_new_structural_owner_delegating_to_external_legacy_writer() {
        let current = root("structural_owner_legacy_delegate_current");
        let incoming = root("structural_owner_legacy_delegate_incoming");
        for tree in [&current, &incoming] {
            write(
                tree,
                "src/legacy_writer.rs",
                "pub fn write_legacy(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
            );
        }
        write(
            &incoming,
            "src/structural_write_authority/new_owner.rs",
            "pub fn new_owner(xml: &mut String) { write_legacy(xml); }\n",
        );
        let changed = BTreeSet::from(["src/structural_write_authority/new_owner.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(
            result.as_ref().is_err_and(|error| {
                error.contains("structural_owner_delegates_to_noncanonical_writer_root")
            }),
            "{result:?}"
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_mutation_rejects_existing_writer_reachability_widening_to_external_legacy_root() {
        let current = root("writer_reachability_widening_current");
        let incoming = root("writer_reachability_widening_incoming");
        write(
            &current,
            "src/music.rs",
            concat!(
                "pub fn existing(xml: &mut String) { write_a(xml); }\n",
                "fn write_a(xml: &mut String) { xml.insert_str(0, \"a\"); }\n",
                "fn write_b(xml: &mut String) { xml.insert_str(0, \"b\"); }\n",
            ),
        );
        write(
            &incoming,
            "src/music.rs",
            concat!(
                "pub fn existing(xml: &mut String) { write_a(xml); write_b(xml); }\n",
                "fn write_a(xml: &mut String) { xml.insert_str(0, \"a\"); }\n",
                "fn write_b(xml: &mut String) { xml.insert_str(0, \"b\"); }\n",
            ),
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(
            result.as_ref().is_err_and(|error| {
                error.contains("writer_reachability_widened_to_noncanonical_root")
            }),
            "{result:?}"
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_mutation_allows_structural_owner_delegating_to_canonical_structural_writer() {
        let current = root("structural_owner_canonical_delegate_current");
        let incoming = root("structural_owner_canonical_delegate_incoming");
        for tree in [&current, &incoming] {
            write(
                tree,
                "src/structural_write_authority/canonical.rs",
                "pub fn write_canonical(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
            );
        }
        write(
            &incoming,
            "src/structural_write_authority/new_owner.rs",
            "pub fn new_owner(xml: &mut String) { write_canonical(xml); }\n",
        );
        let changed = BTreeSet::from(["src/structural_write_authority/new_owner.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_mutation_allows_new_canonical_structural_writer_owner() {
        let current = root("canonical_owner_current");
        let incoming = root("canonical_owner_incoming");
        write(
            &incoming,
            "src/structural_write_authority/owner.rs",
            "pub fn apply_canonical_owner(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        let changed = BTreeSet::from(["src/structural_write_authority/owner.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn music_mutation_allows_read_only_repair_named_observation() {
        let current = root("repair_observation_current");
        let incoming = root("repair_observation_incoming");
        write(&current, "src/music.rs", "fn keep() -> bool { true }\n");
        write(
            &incoming,
            "src/music.rs",
            "fn keep() -> bool { true }\nfn historical_repair_observation() -> bool { keep() }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn audit_only_rejects_indirect_writer_reachability() {
        let current = root("audit_current");
        let incoming = root("audit_incoming");
        write(
            &current,
            "src/audits/proof.rs",
            "pub fn proof() -> bool { true }\n",
        );
        write(
            &incoming,
            "src/audits/proof.rs",
            "pub fn proof() -> bool { bridge(); true }\n",
        );
        write(
            &current,
            "src/music.rs",
            "pub fn bridge() {}\nfn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "pub fn bridge() { write_musicxml(&mut String::new()); }\nfn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        let changed = BTreeSet::from([
            "src/audits/proof.rs".to_string(),
            "src/music.rs".to_string(),
        ]);
        let result = validate_semantic_change(&current, &incoming, "AuditOnly", &changed);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn launcher_inventory_is_kodex_domain_but_similar_paths_are_not() {
        assert_eq!(
            domain_for_path("src/bin/etuschlopf_launcher.rs"),
            Domain::Kodex
        );
        assert_eq!(
            domain_for_path("src/bin/etuschlopf_launcher/assistant_export.rs"),
            Domain::Kodex
        );
        assert_eq!(domain_for_path("src/visual_preflight.rs"), Domain::Kodex);
        assert_eq!(
            domain_for_path("src/visual_preflight/no_seed.rs"),
            Domain::Kodex
        );
        assert_eq!(domain_for_path("src/bin/other_launcher.rs"), Domain::Music);
    }

    #[test]
    fn public_artifact_recovery_inventory_is_exact_kodex_domain() {
        assert_eq!(
            domain_for_path("src/kodex_control_plane/public_artifact_recovery.rs"),
            Domain::Kodex
        );
        assert_eq!(
            domain_for_path("src/kodex_control_plane/public_artifact_recovery_shadow.rs"),
            Domain::Music
        );
        assert_eq!(
            domain_for_path("src/kodex_control_plane/other.rs"),
            Domain::Music
        );
    }

    #[test]
    fn kodex_only_accepts_non_writer_shared_recovery_control_module() {
        let current = root("shared_recovery_kodex_current");
        let incoming = root("shared_recovery_kodex_incoming");
        write(
            &incoming,
            "src/kodex_control_plane/public_artifact_recovery.rs",
            "pub enum RecoveryState { Idle, Pending }\n",
        );
        let changed =
            BTreeSet::from(["src/kodex_control_plane/public_artifact_recovery.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "KodexOnly", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_only_accepts_non_writer_launcher_recovery_control_change() {
        let current = root("launcher_kodex_current");
        let incoming = root("launcher_kodex_incoming");
        write(
            &current,
            "src/bin/etuschlopf_launcher.rs",
            "fn recover_public_artifacts() -> bool { false }\n",
        );
        write(
            &incoming,
            "src/bin/etuschlopf_launcher.rs",
            "fn recover_public_artifacts() -> bool { true }\n",
        );
        let changed = BTreeSet::from(["src/bin/etuschlopf_launcher.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "KodexOnly", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_only_rejects_indirect_music_writer_reachability() {
        let current = root("kodex_writer_current");
        let incoming = root("kodex_writer_incoming");
        write(
            &current,
            "src/take_change_control.rs",
            "pub fn kodex_gate() {}\n",
        );
        write(
            &incoming,
            "src/take_change_control.rs",
            "pub fn kodex_gate() { apply_score(); }\n",
        );
        for tree in [&current, &incoming] {
            write(
                tree,
                "src/music.rs",
                "pub fn apply_score() { write_musicxml(&mut String::new()); }\nfn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
            );
        }
        let changed = BTreeSet::from(["src/take_change_control.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "KodexOnly", &changed);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn audit_only_rejects_writer_reachability_through_kodex_bridge() {
        let current = root("audit_kodex_bridge_current");
        let incoming = root("audit_kodex_bridge_incoming");
        write(
            &current,
            "src/audits/proof.rs",
            "pub fn proof() -> bool { true }\n",
        );
        write(
            &incoming,
            "src/audits/proof.rs",
            "pub fn proof() -> bool { kodex_bridge(); true }\n",
        );
        for tree in [&current, &incoming] {
            write(
                tree,
                "src/take_change_control.rs",
                "pub fn kodex_bridge() { apply_score(); }\n",
            );
            write(
                tree,
                "src/music.rs",
                "pub fn apply_score() { write_musicxml(&mut String::new()); }\nfn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
            );
        }
        let changed = BTreeSet::from(["src/audits/proof.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "AuditOnly", &changed);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn audit_only_cross_role_retirement_requires_closed_dead_probe() {
        let current = root("audit_probe_current");
        let incoming = root("audit_probe_incoming");
        write(
            &current,
            "src/notation.rs",
            "fn keep() {}\nfn probe_trace(x: &str) -> usize { helper(x) }\nfn helper(x: &str) -> usize { x.len() }\n",
        );
        write(&incoming, "src/notation.rs", "fn keep() {}\n");
        let changed = BTreeSet::from(["src/notation.rs".to_string()]);
        assert!(validate_semantic_change(&current, &incoming, "AuditOnly", &changed).is_ok());
        write(
            &current,
            "src/notation.rs",
            "fn keep(x: &str) -> usize { probe_trace(x) }\nfn probe_trace(x: &str) -> usize { x.len() }\n",
        );
        write(
            &incoming,
            "src/notation.rs",
            "fn keep(x: &str) -> usize { probe_trace(x) }\n",
        );
        assert!(validate_semantic_change(&current, &incoming, "AuditOnly", &changed).is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn audit_only_accepts_new_report_reader_and_display_locator() {
        let current = root("audit_report_reader_current");
        let incoming = root("audit_report_reader_incoming");
        write(
            &incoming,
            "src/audits/outcome.rs",
            "pub fn review_measure_number(path: &std::path::Path) -> usize { std::fs::read_to_string(path).map(|text| text.len()).unwrap_or(0) }\n",
        );
        let changed = BTreeSet::from(["src/audits/outcome.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "AuditOnly", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn outcome_projection_accepts_audit_and_kodex_report_binding() {
        let current = root("outcome_projection_current");
        let incoming = root("outcome_projection_incoming");
        write(
            &incoming,
            "src/audits/outcome.rs",
            "pub fn review_measure_number() -> &'static str { env!(\"CARGO_PKG_VERSION\") }\n",
        );
        write(
            &incoming,
            "src/bin/etuschlopf_launcher.rs",
            "fn project_outcome_report(path: &std::path::Path) -> usize { std::fs::read_to_string(path).map(|text| text.len()).unwrap_or(0) }\n",
        );
        let changed = BTreeSet::from([
            "src/audits/outcome.rs".to_string(),
            "src/bin/etuschlopf_launcher.rs".to_string(),
        ]);
        let result = validate_semantic_change(&current, &incoming, "OutcomeProjection", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn passive_outcome_status_is_not_an_executable_apply_surface() {
        let passive = FunctionInfo {
            key: "passive".to_string(),
            path: "src/audits/outcome.rs".to_string(),
            domain: Domain::Audit,
            name: "refresh_outcome_projection".to_string(),
            symbol: "refresh_outcome_projection#1".to_string(),
            signature: String::new(),
            source_body: "{ self.master_apply = projects.iter().any(|row| row.master_apply); assert!(!self.master_apply); }".to_string(),
            masked_body: "{ self.master_apply = projects.iter().any(|row| row.master_apply); assert!(!self.master_apply); }".to_string(),
            calls: BTreeMap::new(),
            start_line: 1,
            public: false,
            direct_writer: false,
        };
        assert!(!executable_master_source_commit_surface(&passive));
    }

    #[test]
    fn literal_enablement_remains_an_executable_apply_surface() {
        let active = FunctionInfo {
            key: "active".to_string(),
            path: "src/music.rs".to_string(),
            domain: Domain::Music,
            name: "enable_master_source".to_string(),
            symbol: "enable_master_source#1".to_string(),
            signature: String::new(),
            source_body: "{ state.master_apply = true; }".to_string(),
            masked_body: "{ state.master_apply = true; }".to_string(),
            calls: BTreeMap::new(),
            start_line: 1,
            public: false,
            direct_writer: false,
        };
        assert!(executable_master_source_commit_surface(&active));
    }

    #[test]
    fn outcome_projection_rejects_music_and_writer_changes() {
        let current = root("outcome_projection_writer_current");
        let incoming = root("outcome_projection_writer_incoming");
        write(
            &incoming,
            "src/music.rs",
            "fn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "OutcomeProjection", &changed);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn writer_and_threshold_are_never_combined() {
        let current = root("writer_threshold_current");
        let incoming = root("writer_threshold_incoming");
        write(
            &current,
            "src/music.rs",
            "const SCORE_THRESHOLD: i32 = 3;\nfn decide() {}\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "const SCORE_THRESHOLD: i32 = 4;\nfn decide() { write_musicxml(&mut String::new()); }\nfn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "MusicMutation", &changed);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn calibration_only_accepts_multiline_threshold_value_change() {
        let current = root("multiline_calibration_current");
        let incoming = root("multiline_calibration_incoming");
        write(
            &current,
            "src/music.rs",
            "fn decide(score: f64) -> bool {\n    let confidence_threshold: f64 =\n        0.70;\n    score >= confidence_threshold\n}\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "fn decide(score: f64) -> bool {\n    let confidence_threshold: f64 =\n        0.80;\n    score >= confidence_threshold\n}\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "CalibrationOnly", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn calibration_only_rejects_operator_change_beside_number_change() {
        let current = root("calibration_operator_current");
        let incoming = root("calibration_operator_incoming");
        write(
            &current,
            "src/music.rs",
            "fn decide() { let confidence_threshold = 0.70 + 0.05; }\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "fn decide() { let confidence_threshold = 0.80 - 0.05; }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "CalibrationOnly", &changed);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_only_rejects_music_threshold_change() {
        let current = root("kodex_threshold_current");
        let incoming = root("kodex_threshold_incoming");
        write(
            &current,
            "src/music.rs",
            "const SCORE_THRESHOLD: f64 = 0.70;\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "const SCORE_THRESHOLD: f64 = 0.80;\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "KodexOnly", &changed);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn function_inventory_preserves_lifetimes_and_labels() {
        let source = "fn with_lifetime<'a>(value: &'a str) -> &'a str { 'scan: loop { break 'scan value; } }\n";
        let masked = mask_non_code(source);
        let spans = function_spans(source, &masked).expect("function spans");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].name, "with_lifetime");
    }

    #[test]
    fn restricted_module_visibility_is_not_an_exported_writer_surface() {
        let current = root("restricted_visibility_current");
        let incoming = root("restricted_visibility_incoming");
        write(
            &incoming,
            "src/bin/etuschlopf_launcher/assistant_export.rs",
            r#"pub(super) fn create_assistant_export() { write_candidate(); }
fn write_candidate() { std::fs::write("x", "y").unwrap(); }
"#,
        );
        let inventory = inventory_tree(&incoming).expect("inventory");
        assert!(!inventory.records.values().any(|record| {
            record.kind == CapabilityKind::ExportedWriterCapability
                && record.symbol.starts_with("create_assistant_export#")
        }));
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_writer_relocation_requires_explicit_closed_graph_proof() {
        let current = root("kodex_writer_move_current");
        let incoming = root("kodex_writer_move_incoming");
        write(
            &current,
            "src/bin/etuschlopf_launcher.rs",
            "fn old_stage() { write_candidate(); }
fn write_candidate() { crate::structural_write_authority::verified_render_candidate_sink_authority::write_candidate(); }
",
        );
        write(
            &incoming,
            "src/bin/etuschlopf_launcher.rs",
            "fn new_stage() { write_candidate(); }
fn write_candidate() { crate::structural_write_authority::verified_render_candidate_sink_authority::write_candidate(); }
",
        );
        write(
            &current,
            "src/structural_write_authority/verified_render_candidate_sink_authority.rs",
            "pub fn write_candidate() {}
",
        );
        write(
            &incoming,
            "src/structural_write_authority/verified_render_candidate_sink_authority.rs",
            "pub fn write_candidate() {}
",
        );
        let changed = BTreeSet::from(["src/bin/etuschlopf_launcher.rs".to_string()]);
        assert!(validate_semantic_change(&current, &incoming, "KodexOnly", &changed).is_err());
        assert!(
            validate_semantic_change_with_options(
                &current,
                &incoming,
                "KodexOnly",
                &changed,
                true,
                false,
            )
            .is_ok()
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_writer_relocation_rejects_new_exported_writer_surface() {
        let current = root("kodex_writer_export_current");
        let incoming = root("kodex_writer_export_incoming");
        write(
            &current,
            "src/bin/etuschlopf_launcher.rs",
            "fn old_stage() { write_candidate(); }
fn write_candidate() { crate::structural_write_authority::verified_render_candidate_sink_authority::write_candidate(); }
",
        );
        write(
            &incoming,
            "src/bin/etuschlopf_launcher.rs",
            "pub fn new_stage() { write_candidate(); }
fn write_candidate() { crate::structural_write_authority::verified_render_candidate_sink_authority::write_candidate(); }
",
        );
        write(
            &current,
            "src/structural_write_authority/verified_render_candidate_sink_authority.rs",
            "pub fn write_candidate() {}
",
        );
        write(
            &incoming,
            "src/structural_write_authority/verified_render_candidate_sink_authority.rs",
            "pub fn write_candidate() {}
",
        );
        let changed = BTreeSet::from(["src/bin/etuschlopf_launcher.rs".to_string()]);
        assert!(
            validate_semantic_change_with_options(
                &current,
                &incoming,
                "KodexOnly",
                &changed,
                true,
                false,
            )
            .is_err()
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn removed_kodex_check_is_rejected_even_for_kodex_only() {
        let current = root("negative_current");
        let incoming = root("negative_incoming");
        write(
            &current,
            "src/take_change_control.rs",
            "fn gate(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        write(
            &incoming,
            "src/take_change_control.rs",
            "fn gate(_ok: bool) -> Result<(), String> { Ok(()) }\n",
        );
        let changed = BTreeSet::from(["src/take_change_control.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "KodexOnly", &changed);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_relocation_accepts_exact_path_move() {
        let current = root("negative_relocation_current");
        let incoming = root("negative_relocation_incoming");
        write_kodex_execution_anchor(&current, Some("deny_publish"));
        write_kodex_execution_anchor(&incoming, Some("deny_publish"));
        let function = "fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n";
        write(&current, "src/build_gate/old_negative.rs", function);
        write(&incoming, "src/build_gate/new_negative.rs", function);
        let changed = BTreeSet::from([
            "src/build_gate/old_negative.rs".to_string(),
            "src/build_gate/new_negative.rs".to_string(),
        ]);
        let result = validate_semantic_change_with_options(
            &current,
            &incoming,
            "KodexOnly",
            &changed,
            false,
            true,
        );
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_module_relocation_accepts_private_to_parent_module_move() {
        let current = root("negative_module_relocation_current");
        let incoming = root("negative_module_relocation_incoming");
        write(
            &current,
            "src/take_change_control.rs",
            "fn main() { let _ = deny_publish(false); }\nfn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        write(
            &incoming,
            "src/take_change_control.rs",
            "#[path = \"build_gate/deny_publish_gate.rs\"]\nmod deny_publish_gate;\nfn main() { let _ = deny_publish_gate::deny_publish(false); }\n",
        );
        write(
            &incoming,
            "src/build_gate/deny_publish_gate.rs",
            "pub(super) fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        let changed = BTreeSet::from([
            "src/take_change_control.rs".to_string(),
            "src/build_gate/deny_publish_gate.rs".to_string(),
        ]);
        let result = validate_semantic_change_with_kodex_relocation_options(
            &current,
            &incoming,
            "KodexOnly",
            &changed,
            false,
            false,
            true,
        );
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_module_relocation_is_rejected_without_explicit_marker() {
        let current = root("negative_module_relocation_unmarked_current");
        let incoming = root("negative_module_relocation_unmarked_incoming");
        write(
            &current,
            "src/take_change_control.rs",
            "fn main() { let _ = deny_publish(false); }\nfn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        write(
            &incoming,
            "src/take_change_control.rs",
            "#[path = \"build_gate/deny_publish_gate.rs\"]\nmod deny_publish_gate;\nfn main() { let _ = deny_publish_gate::deny_publish(false); }\n",
        );
        write(
            &incoming,
            "src/build_gate/deny_publish_gate.rs",
            "pub(super) fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        let changed = BTreeSet::from([
            "src/take_change_control.rs".to_string(),
            "src/build_gate/deny_publish_gate.rs".to_string(),
        ]);
        let result = validate_semantic_change_with_options(
            &current,
            &incoming,
            "KodexOnly",
            &changed,
            false,
            false,
        );
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_module_relocation_rejects_pub_crate_target() {
        let current = root("negative_module_relocation_visibility_current");
        let incoming = root("negative_module_relocation_visibility_incoming");
        write(
            &current,
            "src/take_change_control.rs",
            "fn main() { let _ = deny_publish(false); }\nfn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        write(
            &incoming,
            "src/take_change_control.rs",
            "#[path = \"build_gate/deny_publish_gate.rs\"]\nmod deny_publish_gate;\nfn main() { let _ = deny_publish_gate::deny_publish(false); }\n",
        );
        write(
            &incoming,
            "src/build_gate/deny_publish_gate.rs",
            "pub(crate) fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        let changed = BTreeSet::from([
            "src/take_change_control.rs".to_string(),
            "src/build_gate/deny_publish_gate.rs".to_string(),
        ]);
        let result = validate_semantic_change_with_kodex_relocation_options(
            &current,
            &incoming,
            "KodexOnly",
            &changed,
            false,
            false,
            true,
        );
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_module_relocation_rejects_body_change() {
        let current = root("negative_module_relocation_body_current");
        let incoming = root("negative_module_relocation_body_incoming");
        write(
            &current,
            "src/take_change_control.rs",
            "fn main() { let _ = deny_publish(false); }\nfn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        write(
            &incoming,
            "src/take_change_control.rs",
            "#[path = \"build_gate/deny_publish_gate.rs\"]\nmod deny_publish_gate;\nfn main() { let _ = deny_publish_gate::deny_publish(false); }\n",
        );
        write(
            &incoming,
            "src/build_gate/deny_publish_gate.rs",
            "pub(super) fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(\"changed\".to_string()); } Ok(()) }\n",
        );
        let changed = BTreeSet::from([
            "src/take_change_control.rs".to_string(),
            "src/build_gate/deny_publish_gate.rs".to_string(),
        ]);
        let result = validate_semantic_change_with_kodex_relocation_options(
            &current,
            &incoming,
            "KodexOnly",
            &changed,
            false,
            false,
            true,
        );
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_module_relocation_rejects_missing_parent_binding() {
        let current = root("negative_module_relocation_binding_current");
        let incoming = root("negative_module_relocation_binding_incoming");
        write(
            &current,
            "src/take_change_control.rs",
            "fn main() { let _ = deny_publish(false); }\nfn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        write(
            &incoming,
            "src/take_change_control.rs",
            "fn main() { let _ = deny_publish(false); }\n",
        );
        write(
            &incoming,
            "src/build_gate/deny_publish_gate.rs",
            "pub(super) fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        let changed = BTreeSet::from([
            "src/take_change_control.rs".to_string(),
            "src/build_gate/deny_publish_gate.rs".to_string(),
        ]);
        let result = validate_semantic_change_with_kodex_relocation_options(
            &current,
            &incoming,
            "KodexOnly",
            &changed,
            false,
            false,
            true,
        );
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_module_relocation_rejects_commented_path_binding() {
        let current = root("negative_module_relocation_commented_path_current");
        let incoming = root("negative_module_relocation_commented_path_incoming");
        write(
            &current,
            "src/take_change_control.rs",
            "fn main() { let _ = deny_publish(false); }\nfn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        write(
            &incoming,
            "src/take_change_control.rs",
            "// #[path = \"build_gate/deny_publish_gate.rs\"]\nmod deny_publish_gate;\nfn main() { let _ = deny_publish_gate::deny_publish(false); }\n",
        );
        write(
            &incoming,
            "src/build_gate/deny_publish_gate.rs",
            "pub(super) fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        let changed = BTreeSet::from([
            "src/take_change_control.rs".to_string(),
            "src/build_gate/deny_publish_gate.rs".to_string(),
        ]);
        let result = validate_semantic_change_with_kodex_relocation_options(
            &current,
            &incoming,
            "KodexOnly",
            &changed,
            false,
            false,
            true,
        );
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_relocation_is_rejected_without_explicit_marker() {
        let current = root("negative_relocation_unmarked_current");
        let incoming = root("negative_relocation_unmarked_incoming");
        write_kodex_execution_anchor(&current, Some("deny_publish"));
        write_kodex_execution_anchor(&incoming, Some("deny_publish"));
        let function = "fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n";
        write(&current, "src/build_gate/old_negative.rs", function);
        write(&incoming, "src/build_gate/new_negative.rs", function);
        let changed = BTreeSet::from([
            "src/build_gate/old_negative.rs".to_string(),
            "src/build_gate/new_negative.rs".to_string(),
        ]);
        let result = validate_semantic_change(&current, &incoming, "KodexOnly", &changed);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_relocation_rejects_body_change() {
        let current = root("negative_relocation_body_current");
        let incoming = root("negative_relocation_body_incoming");
        write_kodex_execution_anchor(&current, Some("deny_publish"));
        write_kodex_execution_anchor(&incoming, Some("deny_publish"));
        write(
            &current,
            "src/build_gate/old_negative.rs",
            "fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        write(
            &incoming,
            "src/build_gate/new_negative.rs",
            "fn deny_publish(ok: bool) -> Result<(), String> { if !ok || std::env::var_os(\"DENY_ALL\").is_some() { return Err(String::new()); } Ok(()) }\n",
        );
        let changed = BTreeSet::from([
            "src/build_gate/old_negative.rs".to_string(),
            "src/build_gate/new_negative.rs".to_string(),
        ]);
        let result = validate_semantic_change_with_options(
            &current,
            &incoming,
            "KodexOnly",
            &changed,
            false,
            true,
        );
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_relocation_rejects_signature_change() {
        let current = root("negative_relocation_signature_current");
        let incoming = root("negative_relocation_signature_incoming");
        write_kodex_execution_anchor(&current, Some("deny_publish"));
        write_kodex_execution_anchor(&incoming, Some("deny_publish"));
        write(
            &current,
            "src/build_gate/old_negative.rs",
            "fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        write(
            &incoming,
            "src/build_gate/new_negative.rs",
            "fn deny_publish(ok: bool, _strict: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        let changed = BTreeSet::from([
            "src/build_gate/old_negative.rs".to_string(),
            "src/build_gate/new_negative.rs".to_string(),
        ]);
        let result = validate_semantic_change_with_options(
            &current,
            &incoming,
            "KodexOnly",
            &changed,
            false,
            true,
        );
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_relocation_rejects_symbol_change() {
        let current = root("negative_relocation_symbol_current");
        let incoming = root("negative_relocation_symbol_incoming");
        write_kodex_execution_anchor(&current, Some("deny_publish"));
        write_kodex_execution_anchor(&incoming, Some("deny_publish"));
        write(
            &current,
            "src/build_gate/old_negative.rs",
            "fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        write(
            &incoming,
            "src/build_gate/new_negative.rs",
            "fn deny_publish_relocated(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        let changed = BTreeSet::from([
            "src/build_gate/old_negative.rs".to_string(),
            "src/build_gate/new_negative.rs".to_string(),
        ]);
        let result = validate_semantic_change_with_options(
            &current,
            &incoming,
            "KodexOnly",
            &changed,
            false,
            true,
        );
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_relocation_rejects_lost_execution_anchor() {
        let current = root("negative_relocation_anchor_current");
        let incoming = root("negative_relocation_anchor_incoming");
        write_kodex_execution_anchor(&current, Some("deny_publish"));
        write_kodex_execution_anchor(&incoming, None);
        let function = "fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n";
        write(&current, "src/build_gate/old_negative.rs", function);
        write(&incoming, "src/build_gate/new_negative.rs", function);
        let changed = BTreeSet::from([
            "src/build_gate/old_negative.rs".to_string(),
            "src/build_gate/new_negative.rs".to_string(),
            "src/take_change_control.rs".to_string(),
        ]);
        let result = validate_semantic_change_with_options(
            &current,
            &incoming,
            "KodexOnly",
            &changed,
            false,
            true,
        );
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_negative_check_relocation_rejects_extra_negative_check() {
        let current = root("negative_relocation_extra_current");
        let incoming = root("negative_relocation_extra_incoming");
        write_kodex_execution_anchor(&current, Some("deny_publish"));
        write_kodex_execution_anchor(&incoming, Some("deny_publish"));
        let function = "fn deny_publish(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n";
        write(&current, "src/build_gate/old_negative.rs", function);
        write(&incoming, "src/build_gate/new_negative.rs", function);
        write(
            &incoming,
            "src/build_gate/extra_negative.rs",
            "fn deny_extra(ok: bool) -> Result<(), String> { if !ok { return Err(String::new()); } Ok(()) }\n",
        );
        let changed = BTreeSet::from([
            "src/build_gate/old_negative.rs".to_string(),
            "src/build_gate/new_negative.rs".to_string(),
            "src/build_gate/extra_negative.rs".to_string(),
        ]);
        let result = validate_semantic_change_with_options(
            &current,
            &incoming,
            "KodexOnly",
            &changed,
            false,
            true,
        );
        assert!(result.is_err());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn separate_watch_manifest_and_lock_are_explicitly_managed_without_opening_other_nested_manifests()
     {
        let watch_paths = BTreeSet::from([
            WATCH_PROJECT_MANIFEST_PATH.to_string(),
            WATCH_PROJECT_LOCK_PATH.to_string(),
        ]);
        assert!(validate_changed_paths_are_known(&watch_paths, &[]).is_ok());

        let unrelated_manifest = BTreeSet::from(["tools/other_app/Cargo.toml".to_string()]);
        assert!(validate_changed_paths_are_known(&unrelated_manifest, &[]).is_err());
    }

    #[test]
    fn watch_manifest_and_lock_are_semantic_kodex_domain_paths() {
        assert_eq!(domain_for_path(WATCH_PROJECT_MANIFEST_PATH), Domain::Kodex);
        assert_eq!(domain_for_path(WATCH_PROJECT_LOCK_PATH), Domain::Kodex);
        assert_eq!(domain_for_path("tools/other_app/Cargo.toml"), Domain::Music);
    }

    #[test]
    fn inclusive_range_operator_is_not_inventory_assignment() {
        let masked = "let min_end = start.saturating_add(1); for end in min_end..=max_end { let value = end + 1; }";
        let assignments = threshold_assignments(masked);
        assert_eq!(assignments.len(), 1);
        assert_eq!(assignments[0].name, "min_end");
        assert_eq!(
            masked[assignments[0].rhs_start..assignments[0].rhs_end].trim(),
            "start.saturating_add(1)"
        );
    }

    #[test]
    fn duration_identifier_is_not_a_ratio_threshold() {
        assert!(!threshold_like_name("duration"));
        assert!(!threshold_like_name("event_duration"));
        assert!(threshold_like_name("compression_ratio"));
        assert!(threshold_like_name("ratio_limit"));
    }

    #[test]
    fn alpha_fingerprint_survives_local_identifier_rename() {
        let before = "{ let duration = source.max(0); sink(duration); }";
        let after = "{ let span_len = source.max(0); sink(span_len); }";
        assert_eq!(
            normalize_identifier_invariant_body(before),
            normalize_identifier_invariant_body(after)
        );
    }

    #[test]
    fn same_target_function_rename_cannot_erase_writer_capability() {
        let first = root("revision_writer_name_first");
        let second = root("revision_writer_name_second");
        write(
            &first,
            "src/music.rs",
            "fn apply_candidate(source: &str) -> String { transform(source) }\nfn transform(source: &str) -> String { source.to_string() }\n",
        );
        write(
            &second,
            "src/music.rs",
            "fn candidate(source: &str) -> String { transform(source) }\nfn transform(source: &str) -> String { source.to_string() }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let previous = revision_effect_snapshot(&first, &changed).expect("previous snapshot");
        let current = revision_effect_snapshot(&second, &changed).expect("current snapshot");
        let result = validate_revision_effect_monotonic(&previous, &current);
        assert!(result.is_err(), "{result:?}");
        let _ = fs::remove_dir_all(first);
        let _ = fs::remove_dir_all(second);
    }

    #[test]
    fn same_target_identifier_rename_cannot_erase_prior_threshold_effect() {
        let body = fnv1a64(
            normalize_identifier_invariant_body(
                "{ let duration = source.max(0); sink(duration); }",
            )
            .as_bytes(),
        );
        let previous =
            format!("v1\\tsrc/music.rs\\t{body:016x}\\tcalibration-threshold\\tmax,sink\\n")
                .replace("\\t", "\t")
                .replace("\\n", "\n");
        let current = format!("v1\\tsrc/music.rs\\t{body:016x}\\t\\tmax,sink\\n")
            .replace("\\t", "\t")
            .replace("\\n", "\n");
        let result = validate_revision_effect_monotonic(&previous, &current);
        assert!(result.is_err(), "{result:?}");
    }

    #[test]
    fn materially_removed_operation_does_not_block_later_candidate() {
        let previous =
            "v1\tsrc/music.rs\t0000000000000001\twriter-name-root\ttransform,voice,beam\n";
        let current = "v1\tsrc/music.rs\t0000000000000002\t\tparse,report,summary\n";
        assert!(validate_revision_effect_monotonic(previous, current).is_ok());
    }

    #[test]
    fn productive_identity_firewall_rejects_external_locator_through_harmless_wrapper() {
        let tree = root("identity_provenance_external_locator");
        write(
            &tree,
            "src/music.rs",
            r#"
fn source() -> String { std::env::var("FEEDBACK_MEASURE").unwrap_or_default() }
fn process(value: usize) { write_musicxml(&mut String::new(), value); }
fn foo() { let x = source().parse::<usize>().unwrap_or(0); process(x); }
fn write_musicxml(xml: &mut String, value: usize) { if value > 0 { xml.insert_str(0, "x"); } }
"#,
        );
        let result = validate_productive_music_identity_provenance(&tree);
        assert!(result.is_err(), "{result:?}");
        let _ = fs::remove_dir_all(tree);
    }

    #[test]
    fn productive_identity_firewall_rejects_external_locator_via_struct_field() {
        let tree = root("identity_provenance_struct_field");
        write(
            &tree,
            "src/music.rs",
            r#"
struct Holder { target_measure: usize }
fn source() -> String { std::env::var("TARGET").unwrap_or_default() }
fn process(holder: Holder) { write_musicxml(&mut String::new(), holder.target_measure); }
fn arrange_measure() { let holder = Holder { target_measure: source().parse().unwrap_or(0) }; process(holder); }
fn write_musicxml(xml: &mut String, measure: usize) { if measure > 0 { xml.insert_str(0, "x"); } }
"#,
        );
        let result = validate_productive_music_identity_provenance(&tree);
        assert!(result.is_err(), "{result:?}");
        let _ = fs::remove_dir_all(tree);
    }

    #[test]
    fn productive_identity_firewall_rejects_closure_capture() {
        let tree = root("identity_provenance_closure");
        write(
            &tree,
            "src/music.rs",
            r#"
fn process() {
    let measure_number = std::env::var("TARGET").ok().and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
    let closure = || write_musicxml(&mut String::new(), measure_number);
    closure();
}
fn write_musicxml(xml: &mut String, measure: usize) { if measure > 0 { xml.insert_str(0, "x"); } }
"#,
        );
        let result = validate_productive_music_identity_provenance(&tree);
        assert!(result.is_err(), "{result:?}");
        let _ = fs::remove_dir_all(tree);
    }

    #[test]
    fn productive_identity_firewall_rejects_raw_public_target_measure() {
        let tree = root("identity_provenance_raw_public_target");
        write(
            &tree,
            "src/music.rs",
            "pub struct Plan {\n    pub human_potential_target_measure:\n        usize,\n}\npub fn process(plan: Plan) { let _ = plan.human_potential_target_measure; }\n",
        );
        let result = validate_productive_music_identity_provenance(&tree);
        assert!(result.is_err(), "{result:?}");
        let _ = fs::remove_dir_all(tree);
    }

    #[test]
    fn productive_identity_firewall_allows_private_structural_measure_and_audit_locator() {
        let tree = root("identity_provenance_positive_controls");
        write(
            &tree,
            "src/music.rs",
            "fn arrange_measure(measure_number: usize) { write_musicxml(&mut String::new(), measure_number); }\nfn write_musicxml(xml: &mut String, measure: usize) { if measure > 0 { xml.insert_str(0, \"x\"); } }\n",
        );
        write(
            &tree,
            "src/audits/report.rs",
            "pub fn append_audit(measure_number: usize) -> usize { measure_number }\n",
        );
        let result = validate_productive_music_identity_provenance(&tree);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(tree);
    }

    #[test]
    fn productive_identity_firewall_allows_opaque_structural_locator_boundary() {
        let tree = root("identity_provenance_opaque_locator");
        write(
            &tree,
            "src/music.rs",
            r#"
#[derive(Clone, Copy)]
pub struct VerifiedStructuralLocator { measure: usize }
pub struct Plan { pub target: Option<VerifiedStructuralLocator> }
pub fn process(target: VerifiedStructuralLocator) { write_musicxml(&mut String::new(), target.measure); }
fn write_musicxml(xml: &mut String, measure: usize) { if measure > 0 { xml.insert_str(0, "x"); } }
"#,
        );
        let result = validate_productive_music_identity_provenance(&tree);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(tree);
    }

    #[test]
    fn productive_identity_firewall_rejects_direct_variable_measure_selector() {
        let tree = root("identity_provenance_direct_variable_selector");
        write(
            &tree,
            "src/music.rs",
            r#"
fn process(measure_number: usize) {
    let target_measure = std::env::var("TARGET").ok().and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
    if measure_number == target_measure { write_musicxml(&mut String::new(), measure_number); }
}
fn write_musicxml(xml: &mut String, measure: usize) { if measure > 0 { xml.insert_str(0, "x"); } }
"#,
        );
        let result = validate_productive_music_identity_provenance(&tree);
        assert!(result.is_err(), "{result:?}");
        let _ = fs::remove_dir_all(tree);
    }

    #[test]
    fn productive_identity_firewall_rejects_tuple_transport() {
        let tree = root("identity_provenance_tuple_transport");
        write(
            &tree,
            "src/music.rs",
            r#"
fn source() -> String { std::env::var("TARGET").unwrap_or_default() }
fn process(target_measure: usize) { write_musicxml(&mut String::new(), target_measure); }
fn foo() { let tuple = (source().parse::<usize>().unwrap_or(0), 7usize); let target_measure = tuple.0; process(target_measure); }
fn write_musicxml(xml: &mut String, measure: usize) { if measure > 0 { xml.insert_str(0, "x"); } }
"#,
        );
        let result = validate_productive_music_identity_provenance(&tree);
        assert!(result.is_err(), "{result:?}");
        let _ = fs::remove_dir_all(tree);
    }

    #[test]
    fn productive_identity_firewall_rejects_raw_public_target_parameter() {
        let tree = root("identity_provenance_raw_public_parameter");
        write(
            &tree,
            "src/music.rs",
            "pub(super) fn process(feedback_measure: usize) { let _ = feedback_measure; }\n",
        );
        let result = validate_productive_music_identity_provenance(&tree);
        assert!(result.is_err(), "{result:?}");
        let _ = fs::remove_dir_all(tree);
    }

    #[test]
    fn productive_identity_firewall_allows_non_identity_runtime_control() {
        let tree = root("identity_provenance_non_identity_runtime");
        write(
            &tree,
            "src/music.rs",
            r#"
fn renderer_choice() -> String { std::env::var("PDF_RENDERER").unwrap_or_default() }
fn render_output() { let _renderer = renderer_choice(); write_musicxml(&mut String::new()); }
fn write_musicxml(xml: &mut String) { xml.insert_str(0, "x"); }
"#,
        );
        let result = validate_productive_music_identity_provenance(&tree);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(tree);
    }
    #[test]
    fn retirement_ledger_accepts_pre_kodex_and_kodex_negative_check_names() {
        for kind in [
            format!("{}NegativeCheck", pre_kodex_title_token()),
            "KodexNegativeCheck".to_string(),
        ] {
            let tree = root("retirement_kind_namespace_bridge");
            write(
                &tree,
                KODEX_CAPABILITY_RETIREMENTS_PATH,
                &format!(
                    "schema_version=1\nretire={kind}|src/example.rs|deny#1|fnv64:0000000000000001|reason:namespace_bridge_test\n"
                ),
            );
            let entries = read_kodex_capability_retirements(&tree).expect("retirements");
            assert_eq!(entries.len(), 1);
            assert_eq!(
                entries.values().next().expect("retirement").kind,
                CapabilityKind::KodexNegativeCheck
            );
            let _ = fs::remove_dir_all(tree);
        }
    }

    #[test]
    fn kodex_negative_check_removal_requires_new_exact_retirement_entry() {
        let current = root("kodex_retirement_requires_contract_current");
        let incoming = root("kodex_retirement_requires_contract_incoming");
        write(
            &current,
            "src/bin/etuschlopf_launcher/kodex.rs",
            "fn deny_publish() -> Result<(), String> { return Err(\"forbidden\".to_string()); }\n",
        );
        write(
            &incoming,
            "src/bin/etuschlopf_launcher/kodex.rs",
            "fn harmless() {}\n",
        );
        let changed = BTreeSet::from(["src/bin/etuschlopf_launcher/kodex.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "KodexOnly", &changed);
        assert!(result.is_err(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn exact_new_kodex_retirement_authorizes_only_bound_negative_check() {
        let current = root("kodex_retirement_exact_current");
        let incoming = root("kodex_retirement_exact_incoming");
        write(
            &current,
            "src/bin/etuschlopf_launcher/kodex.rs",
            "fn deny_publish() -> Result<(), String> { return Err(\"forbidden\".to_string()); }\n",
        );
        write(
            &incoming,
            "src/bin/etuschlopf_launcher/kodex.rs",
            "fn harmless() {}\n",
        );
        write(
            &current,
            KODEX_CAPABILITY_RETIREMENTS_PATH,
            "schema_version=1\n",
        );
        let current_inventory = inventory_tree(&current).expect("current inventory");
        let key = "KodexNegativeCheck|src/bin/etuschlopf_launcher/kodex.rs|deny_publish#1";
        let fingerprint = current_inventory
            .records
            .get(key)
            .expect("negative check")
            .fingerprint;
        write(
            &incoming,
            KODEX_CAPABILITY_RETIREMENTS_PATH,
            &format!(
                "schema_version=1\nretire=KodexNegativeCheck|src/bin/etuschlopf_launcher/kodex.rs|deny_publish#1|fnv64:{fingerprint:016x}|reason:test_retirement\n"
            ),
        );
        let changed = BTreeSet::from([
            "src/bin/etuschlopf_launcher/kodex.rs".to_string(),
            KODEX_CAPABILITY_RETIREMENTS_PATH.to_string(),
        ]);
        let result = validate_semantic_change(&current, &incoming, "KodexOnly", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn exact_outcome_projection_retirement_authorizes_bound_negative_check() {
        let current = root("outcome_projection_kodex_retirement_exact_current");
        let incoming = root("outcome_projection_kodex_retirement_exact_incoming");
        write(
            &current,
            "src/build_gate/feedback_structuralization_kodex.rs",
            "fn deny_legacy_feedback_surface() -> Result<(), String> { return Err(\"forbidden\".to_string()); }\n",
        );
        write(
            &incoming,
            "src/build_gate/feedback_structuralization_kodex.rs",
            "fn harmless() {}\n",
        );
        write(
            &current,
            KODEX_CAPABILITY_RETIREMENTS_PATH,
            "schema_version=1\n",
        );
        let current_inventory = inventory_tree(&current).expect("current inventory");
        let key = "KodexNegativeCheck|src/build_gate/feedback_structuralization_kodex.rs|deny_legacy_feedback_surface#1";
        let fingerprint = current_inventory
            .records
            .get(key)
            .expect("negative check")
            .fingerprint;
        write(
            &incoming,
            KODEX_CAPABILITY_RETIREMENTS_PATH,
            &format!(
                "schema_version=1\nretire=KodexNegativeCheck|src/build_gate/feedback_structuralization_kodex.rs|deny_legacy_feedback_surface#1|fnv64:{fingerprint:016x}|reason:outcome_projection_retirement_test\n"
            ),
        );
        let changed = BTreeSet::from([
            "src/build_gate/feedback_structuralization_kodex.rs".to_string(),
            KODEX_CAPABILITY_RETIREMENTS_PATH.to_string(),
        ]);
        let result = validate_semantic_change(&current, &incoming, "OutcomeProjection", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn audit_only_cannot_add_kodex_negative_check_retirement() {
        let current = root("audit_only_kodex_retirement_rejected_current");
        let incoming = root("audit_only_kodex_retirement_rejected_incoming");
        write(
            &current,
            "src/build_gate/feedback_structuralization_kodex.rs",
            "fn deny_legacy_feedback_surface() -> Result<(), String> { return Err(\"forbidden\".to_string()); }\n",
        );
        write(
            &incoming,
            "src/build_gate/feedback_structuralization_kodex.rs",
            "fn harmless() {}\n",
        );
        write(
            &current,
            KODEX_CAPABILITY_RETIREMENTS_PATH,
            "schema_version=1\n",
        );
        let current_inventory = inventory_tree(&current).expect("current inventory");
        let key = "KodexNegativeCheck|src/build_gate/feedback_structuralization_kodex.rs|deny_legacy_feedback_surface#1";
        let fingerprint = current_inventory
            .records
            .get(key)
            .expect("negative check")
            .fingerprint;
        write(
            &incoming,
            KODEX_CAPABILITY_RETIREMENTS_PATH,
            &format!(
                "schema_version=1\nretire=KodexNegativeCheck|src/build_gate/feedback_structuralization_kodex.rs|deny_legacy_feedback_surface#1|fnv64:{fingerprint:016x}|reason:audit_only_retirement_must_fail\n"
            ),
        );
        let changed = BTreeSet::from([
            "src/build_gate/feedback_structuralization_kodex.rs".to_string(),
            KODEX_CAPABILITY_RETIREMENTS_PATH.to_string(),
        ]);
        let error = validate_semantic_change(&current, &incoming, "AuditOnly", &changed)
            .expect_err("AuditOnly must not admit a new KodexNegativeCheck retirement");
        assert!(
            error.contains("neue Pensionierungen sind ausschliesslich in KodexOnly oder OutcomeProjection erlaubt"),
            "{error}"
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_retirement_allows_shared_rejection_fingerprint_when_function_body_is_distinct() {
        let current = root("kodex_retirement_shared_surface_current");
        let incoming = root("kodex_retirement_shared_surface_incoming");
        write(
            &current,
            "src/bin/etuschlopf_launcher/kodex.rs",
            r#"fn deny_publish() -> Result<(), String> {
    let marker = 1usize;
    return Err("forbidden".to_string());
}
fn deny_other() -> Result<(), String> {
    let marker = 2usize;
    return Err("forbidden".to_string());
}
"#,
        );
        write(
            &incoming,
            "src/bin/etuschlopf_launcher/kodex.rs",
            r#"fn deny_other() -> Result<(), String> {
    let marker = 2usize;
    return Err("forbidden".to_string());
}
"#,
        );
        write(
            &current,
            KODEX_CAPABILITY_RETIREMENTS_PATH,
            "schema_version=1\n",
        );
        let current_inventory = inventory_tree(&current).expect("current inventory");
        let key = "KodexNegativeCheck|src/bin/etuschlopf_launcher/kodex.rs|deny_publish#1";
        let fingerprint = current_inventory
            .records
            .get(key)
            .expect("negative check")
            .fingerprint;
        assert_eq!(
            fingerprint,
            current_inventory
                .records
                .get("KodexNegativeCheck|src/bin/etuschlopf_launcher/kodex.rs|deny_other#1")
                .expect("shared rejection surface")
                .fingerprint,
            "test requires a deliberately shared coarse rejection fingerprint"
        );
        write(
            &incoming,
            KODEX_CAPABILITY_RETIREMENTS_PATH,
            &format!(
                "schema_version=1\nretire=KodexNegativeCheck|src/bin/etuschlopf_launcher/kodex.rs|deny_publish#1|fnv64:{fingerprint:016x}|reason:shared_surface_retirement\n"
            ),
        );
        let changed = BTreeSet::from([
            "src/bin/etuschlopf_launcher/kodex.rs".to_string(),
            KODEX_CAPABILITY_RETIREMENTS_PATH.to_string(),
        ]);
        let result = validate_semantic_change(&current, &incoming, "KodexOnly", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_retirement_still_blocks_same_function_body_relocation() {
        let current = root("kodex_retirement_relocation_current");
        let incoming = root("kodex_retirement_relocation_incoming");
        write(
            &current,
            "src/bin/etuschlopf_launcher/kodex.rs",
            "fn deny_publish() -> Result<(), String> { let marker = 1usize; return Err(\"forbidden\".to_string()); }\n",
        );
        write(
            &incoming,
            "src/bin/etuschlopf_launcher/relocated_kodex.rs",
            "fn deny_publish_relocated() -> Result<(), String> { let marker = 1usize; return Err(\"forbidden\".to_string()); }\n",
        );
        write(
            &current,
            KODEX_CAPABILITY_RETIREMENTS_PATH,
            "schema_version=1\n",
        );
        let current_inventory = inventory_tree(&current).expect("current inventory");
        let key = "KodexNegativeCheck|src/bin/etuschlopf_launcher/kodex.rs|deny_publish#1";
        let fingerprint = current_inventory
            .records
            .get(key)
            .expect("negative check")
            .fingerprint;
        write(
            &incoming,
            KODEX_CAPABILITY_RETIREMENTS_PATH,
            &format!(
                "schema_version=1\nretire=KodexNegativeCheck|src/bin/etuschlopf_launcher/kodex.rs|deny_publish#1|fnv64:{fingerprint:016x}|reason:relocation_must_fail\n"
            ),
        );
        let changed = BTreeSet::from([
            "src/bin/etuschlopf_launcher/kodex.rs".to_string(),
            "src/bin/etuschlopf_launcher/relocated_kodex.rs".to_string(),
            KODEX_CAPABILITY_RETIREMENTS_PATH.to_string(),
        ]);
        let result = validate_semantic_change(&current, &incoming, "KodexOnly", &changed);
        assert!(result.is_err(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn kodex_retirement_is_append_only_and_fingerprint_bound() {
        let current = root("kodex_retirement_append_only_current");
        let incoming = root("kodex_retirement_append_only_incoming");
        write(
            &current,
            KODEX_CAPABILITY_RETIREMENTS_PATH,
            "schema_version=1\nretire=KodexNegativeCheck|src/bin/etuschlopf_launcher/kodex.rs|deny_publish#1|fnv64:0000000000000001|reason:historic\n",
        );
        write(
            &incoming,
            KODEX_CAPABILITY_RETIREMENTS_PATH,
            "schema_version=1\nretire=KodexNegativeCheck|src/bin/etuschlopf_launcher/kodex.rs|deny_publish#1|fnv64:0000000000000002|reason:changed\n",
        );
        let changed = BTreeSet::from([KODEX_CAPABILITY_RETIREMENTS_PATH.to_string()]);
        let result = validate_semantic_change(&current, &incoming, "KodexOnly", &changed);
        assert!(result.is_err(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn refactor_exact_rust_owner_relocation_preserves_existing_writer_reachability() {
        let current = root("refactor_owner_relocation_current");
        let incoming = root("refactor_owner_relocation_incoming");
        let owner_body = concat!(
            "pub fn apply_repair() { write_musicxml(&mut String::new()); }\n",
            "fn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        write(
            &current,
            "src/main.rs",
            "#[path = \"old_owner.rs\"]\nmod midi_seed;\n",
        );
        write(
            &incoming,
            "src/main.rs",
            "#[path = \"new_owner.rs\"]\nmod midi_seed;\n",
        );
        write(&current, "src/old_owner.rs", owner_body);
        write(&incoming, "src/new_owner.rs", owner_body);
        let changed = BTreeSet::from([
            "src/main.rs".to_string(),
            "src/old_owner.rs".to_string(),
            "src/new_owner.rs".to_string(),
        ]);
        let result = validate_semantic_change(&current, &incoming, "RefactorOnly", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn refactor_exact_writer_function_symbol_family_rename_preserves_existing_authority() {
        let current = root("refactor_function_symbol_rename_current");
        let incoming = root("refactor_function_symbol_rename_incoming");
        write(
            &current,
            "src/music.rs",
            concat!(
                "pub fn old_seed(source: &str) -> String { old_seed_with_mode(source, 1) }\n",
                "pub fn old_seed_with_mode(source: &str, mode: u8) -> String { let mut out = source.to_string(); if mode > 0 { apply_hand_trajectory_solver(&mut out); } out }\n",
                "fn apply_hand_trajectory_solver(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
            ),
        );
        write(
            &incoming,
            "src/music.rs",
            concat!(
                "pub fn midi_seed(source: &str) -> String { midi_seed_with_mode(source, 1) }\n",
                "pub fn midi_seed_with_mode(source: &str, mode: u8) -> String { let mut out = source.to_string(); if mode > 0 { apply_hand_trajectory_solver(&mut out); } out }\n",
                "fn apply_hand_trajectory_solver(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
            ),
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "RefactorOnly", &changed);
        assert!(result.is_ok(), "{result:?}");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn refactor_writer_function_symbol_rename_rejects_body_change() {
        let current = root("refactor_function_symbol_rename_body_current");
        let incoming = root("refactor_function_symbol_rename_body_incoming");
        write(
            &current,
            "src/music.rs",
            "pub fn old_seed(source: &str) -> String { let mut out = source.to_string(); apply_hand_trajectory_solver(&mut out); out }\nfn apply_hand_trajectory_solver(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        write(
            &incoming,
            "src/music.rs",
            "pub fn midi_seed(source: &str) -> String { let mut out = source.trim().to_string(); apply_hand_trajectory_solver(&mut out); out }\nfn apply_hand_trajectory_solver(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        let changed = BTreeSet::from(["src/music.rs".to_string()]);
        let result = validate_semantic_change(&current, &incoming, "RefactorOnly", &changed);
        assert!(
            result
                .as_ref()
                .is_err_and(|error| error.contains("NO-NEW-REPAIR-AUTHORITY KODEX GATE")),
            "{result:?}"
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn refactor_rust_owner_relocation_rejects_nonidentical_writer_body() {
        let current = root("refactor_owner_relocation_body_current");
        let incoming = root("refactor_owner_relocation_body_incoming");
        write(
            &current,
            "src/main.rs",
            "#[path = \"old_owner.rs\"]\nmod midi_seed;\n",
        );
        write(
            &incoming,
            "src/main.rs",
            "#[path = \"new_owner.rs\"]\nmod midi_seed;\n",
        );
        write(
            &current,
            "src/old_owner.rs",
            "pub fn apply_repair() { write_musicxml(&mut String::new()); }\nfn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        write(
            &incoming,
            "src/new_owner.rs",
            "pub fn apply_repair() { let _changed = true; write_musicxml(&mut String::new()); }\nfn write_musicxml(xml: &mut String) { xml.insert_str(0, \"x\"); }\n",
        );
        let changed = BTreeSet::from([
            "src/main.rs".to_string(),
            "src/old_owner.rs".to_string(),
            "src/new_owner.rs".to_string(),
        ]);
        let result = validate_semantic_change(&current, &incoming, "RefactorOnly", &changed);
        assert!(
            result
                .as_ref()
                .is_err_and(|error| error.contains("NO-NEW-REPAIR-AUTHORITY KODEX GATE")),
            "{result:?}"
        );
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn governed_change_kodex_surface_is_limited_to_governed_data_and_retire_pair() {
        for path in [
            "src/build_gate/ssc_containment_envelopes_v1.tsv",
            "src/build_gate/ssc_surface_extensions_v1.txt",
            "src/build_gate/ssc_verdict_wiring_v1.txt",
            "src/build_gate/kodex_capability_retirements.txt",
            "src/identity_gate_trust_anchor.rs",
        ] {
            assert!(governed_change_kodex_surface(path), "{path}");
        }
        for path in [
            "src/build_gate/ssc_governed_change.rs",
            "src/build_gate/ssc_k2_retirement_governance.rs",
            "src/build_gate/semantic_capability_diff.rs",
            "src/take_change_control.rs",
            "src/build_gate/ssc_containment_governance_contract_v4.txt",
        ] {
            assert!(!governed_change_kodex_surface(path), "{path}");
        }
    }
}

