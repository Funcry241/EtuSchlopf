use roxmltree::Document;
use crate::sha256;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

pub const PRODUCTIVE_BASELINE_VERSION: &str = "3.1";
pub const PRODUCTIVE_BASELINE_SPECIFICATION_DATE: &str = "2017-12-07";
pub const PRODUCTIVE_BASELINE_RELEASE_TAG: &str = "v3.1";
pub const PRODUCTIVE_BASELINE_RELEASE_COMMIT: &str =
    "8bbe8e50606bf99317a86e9e1637618d6bdd1997";
pub const PRODUCTIVE_BASELINE_SCHEMA_SOURCE: &str =
    "https://github.com/w3c-cg/musicxml/tree/v3.1/schema";
pub const TARGET_VERSION: &str = "4.0";
pub const SPECIFICATION_DATE: &str = "2021-06-01";
pub const RELEASE_TAG: &str = "v4.0";
pub const RELEASE_COMMIT: &str = "799e2defb2ece0ae7bafe08dcbcac25b2c631d53";
pub const RELEASE_ASSET_NAME: &str = "musicxml-4.0.zip";
pub const RELEASE_ASSET_URL: &str = "https://github.com/w3c-cg/musicxml/releases/download/v4.0/musicxml-4.0.zip";
pub const RELEASE_ASSET_BYTES: u64 = 192_235;
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PublishedSpecification";
pub const WRITER_AUTHORITY: &str = "none";
pub const NETWORK_ALLOWED: bool = false;

const PRODUCTIVE_BASELINE_SCHEMA_FILES: [&str; 3] =
    ["musicxml.xsd", "xml.xsd", "xlink.xsd"];
pub const PRODUCTIVE_BASELINE_LOCAL_SCHEMA_ROOT: &str =
    "docs/normative_sources/musicxml/3.1/schema";
const PRODUCTIVE_BASELINE_SCHEMA_EXPECTATIONS: [(&str, usize, &str); 3] = [
    (
        "musicxml.xsd",
        335_762,
        "49ae9e34e04ba5b72004687994f70d598323eb3353eadb62faedcc9e9dc4bd7f",
    ),
    (
        "xml.xsd",
        5_695,
        "616a3077df5cfc954ac74a75abe9697b95eef7a85dbe09367d995a483e840eb5",
    ),
    (
        "xlink.xsd",
        2_280,
        "67c7f7a02491babc51e11c10268150b1a76070da36d5abe88e68515009611966",
    ),
];

pub const PRODUCTIVE_VERSION: &str = "4.0";
pub const PRODUCTIVE_SCHEMA_LOCAL_ROOT: &str =
    "docs/normative_sources/musicxml/4.0/schema";
const PRODUCTIVE_SCHEMA_FILES: [&str; 3] = ["musicxml.xsd", "xml.xsd", "xlink.xsd"];
const PRODUCTIVE_SCHEMA_EXPECTATIONS: [(&str, usize, &str); 3] = [
    (
        "musicxml.xsd",
        379_505,
        "bfe37ed25a9ec00e6f2591d53df260b84efe12aed209ba3ac0a76f9287665a99",
    ),
    (
        "xml.xsd",
        5_695,
        "616a3077df5cfc954ac74a75abe9697b95eef7a85dbe09367d995a483e840eb5",
    ),
    (
        "xlink.xsd",
        2_280,
        "6e601f8eeb41618b50e4c7f944dff754e57ea43b602755470dda24c9c2f6df92",
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedBaselineSchemaLocalFileObservation {
    pub file_name: &'static str,
    pub path: PathBuf,
    pub present: bool,
    pub byte_len: usize,
    pub sha256: Option<String>,
    pub expected_byte_len: usize,
    pub expected_sha256: &'static str,
    pub published_digest_matches: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedBaselineSchemaByteIngressObservation {
    pub root: PathBuf,
    pub files: Vec<PublishedBaselineSchemaLocalFileObservation>,
    pub all_required_local_bytes_present: bool,
    pub all_required_published_digests_match: bool,
    pub originality_qualified: bool,
    pub official_schema_byte_provenance_bound: bool,
}

pub fn observe_published_baseline_schema_byte_ingress(
    project_root: &Path,
) -> PublishedBaselineSchemaByteIngressObservation {
    let root = project_root.join(PRODUCTIVE_BASELINE_LOCAL_SCHEMA_ROOT);
    let files = PRODUCTIVE_BASELINE_SCHEMA_EXPECTATIONS
        .iter()
        .copied()
        .map(|(file_name, expected_byte_len, expected_sha256)| {
            let path = root.join(file_name);
            match fs::read(&path) {
                Ok(bytes) => {
                    let actual_sha256 = sha256::hex(&bytes);
                    let published_digest_matches =
                        bytes.len() == expected_byte_len && actual_sha256 == expected_sha256;
                    PublishedBaselineSchemaLocalFileObservation {
                        file_name,
                        path,
                        present: true,
                        byte_len: bytes.len(),
                        sha256: Some(actual_sha256),
                        expected_byte_len,
                        expected_sha256,
                        published_digest_matches,
                    }
                }
                Err(_) => PublishedBaselineSchemaLocalFileObservation {
                    file_name,
                    path,
                    present: false,
                    byte_len: 0,
                    sha256: None,
                    expected_byte_len,
                    expected_sha256,
                    published_digest_matches: false,
                },
            }
        })
        .collect::<Vec<_>>();
    let all_required_local_bytes_present = files.iter().all(|file| file.present);
    let all_required_published_digests_match = files
        .iter()
        .all(|file| file.present && file.published_digest_matches);
    PublishedBaselineSchemaByteIngressObservation {
        root,
        files,
        all_required_local_bytes_present,
        all_required_published_digests_match,
        originality_qualified: all_required_published_digests_match,
        official_schema_byte_provenance_bound: all_required_published_digests_match,
    }
}

pub fn observe_productive_schema_byte_ingress(
    project_root: &Path,
) -> PublishedBaselineSchemaByteIngressObservation {
    let root = project_root.join(PRODUCTIVE_SCHEMA_LOCAL_ROOT);
    let files = PRODUCTIVE_SCHEMA_EXPECTATIONS
        .iter()
        .copied()
        .map(|(file_name, expected_byte_len, expected_sha256)| {
            let path = root.join(file_name);
            match fs::read(&path) {
                Ok(bytes) => {
                    let actual_sha256 = sha256::hex(&bytes);
                    let published_digest_matches =
                        bytes.len() == expected_byte_len && actual_sha256 == expected_sha256;
                    PublishedBaselineSchemaLocalFileObservation {
                        file_name,
                        path,
                        present: true,
                        byte_len: bytes.len(),
                        sha256: Some(actual_sha256),
                        expected_byte_len,
                        expected_sha256,
                        published_digest_matches,
                    }
                }
                Err(_) => PublishedBaselineSchemaLocalFileObservation {
                    file_name,
                    path,
                    present: false,
                    byte_len: 0,
                    sha256: None,
                    expected_byte_len,
                    expected_sha256,
                    published_digest_matches: false,
                },
            }
        })
        .collect::<Vec<_>>();
    let all_required_local_bytes_present = files.iter().all(|file| file.present);
    let all_required_published_digests_match = files
        .iter()
        .all(|file| file.present && file.published_digest_matches);
    PublishedBaselineSchemaByteIngressObservation {
        root,
        files,
        all_required_local_bytes_present,
        all_required_published_digests_match,
        originality_qualified: all_required_published_digests_match,
        official_schema_byte_provenance_bound: all_required_published_digests_match,
    }
}

const SCHEMA_FILES: [&str; 3] = ["musicxml.xsd", "xml.xsd", "xlink.xsd"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Xsd31Readiness {
    HistoricalProfileQualifiedNotProductAuthority,
}

impl Xsd31Readiness {
    fn as_str(self) -> &'static str {
        match self {
            Self::HistoricalProfileQualifiedNotProductAuthority => {
                "HISTORICAL_PROFILE_QUALIFIED_NOT_PRODUCT_AUTHORITY"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Xsd40Readiness {
    DelegatedToPersistedProductContractGate,
}

impl Xsd40Readiness {
    fn as_str(self) -> &'static str {
        match self {
            Self::DelegatedToPersistedProductContractGate => {
                "DELEGATED_TO_ETUSCHLOPF_PERSISTED_PRODUCT_CONTRACT_GATE_AND_SCHEMASCHLOPF_R021"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactAudit {
    pub path: PathBuf,
    pub present: bool,
    pub well_formed: bool,
    pub root: Option<String>,
    pub declared_version: Option<String>,
    pub byte_len: usize,
    pub sha256: Option<String>,
    pub note_count: usize,
    pub chord_note_count: usize,
    pub chord_predecessor_duration_violations: usize,
    pub chord_violations_with_intervening_forward: usize,
    pub chord_violations_with_intervening_backup: usize,
    pub temporal_replay_supported: bool,
    pub cursor_overflows: usize,
    pub note_end_overflows: usize,
    pub backup_underflows: usize,
    pub chord_temporal_conflicts: usize,
    pub max_cursor_overflow: i64,
    pub max_note_end_overflow: i64,
    pub parse_error: Option<String>,
}

impl ArtifactAudit {
    fn missing(path: PathBuf) -> Self {
        Self {
            path,
            present: false,
            well_formed: false,
            root: None,
            declared_version: None,
            byte_len: 0,
            sha256: None,
            note_count: 0,
            chord_note_count: 0,
            chord_predecessor_duration_violations: 0,
            chord_violations_with_intervening_forward: 0,
            chord_violations_with_intervening_backup: 0,
            temporal_replay_supported: false,
            cursor_overflows: 0,
            note_end_overflows: 0,
            backup_underflows: 0,
            chord_temporal_conflicts: 0,
            max_cursor_overflow: 0,
            max_note_end_overflow: 0,
            parse_error: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadinessAudit {
    pub master: ArtifactAudit,
    pub render: ArtifactAudit,
    pub xsd31_readiness: Xsd31Readiness,
    pub xsd40_readiness: Xsd40Readiness,
}

pub fn audit_xml_text(path: PathBuf, xml: &str) -> ArtifactAudit {
    let document = match Document::parse(xml) {
        Ok(document) => document,
        Err(error) => {
            return ArtifactAudit {
                path,
                present: true,
                well_formed: false,
                root: None,
                declared_version: None,
                byte_len: xml.len(),
                sha256: Some(sha256::hex(xml.as_bytes())),
                note_count: 0,
                chord_note_count: 0,
                chord_predecessor_duration_violations: 0,
                chord_violations_with_intervening_forward: 0,
                chord_violations_with_intervening_backup: 0,
                temporal_replay_supported: false,
                cursor_overflows: 0,
                note_end_overflows: 0,
                backup_underflows: 0,
                chord_temporal_conflicts: 0,
                max_cursor_overflow: 0,
                max_note_end_overflow: 0,
                parse_error: Some(error.to_string()),
            };
        }
    };

    let root = document.root_element();
    let temporal = crate::audit_only::serialized_temporal_replay::observe_serialized_temporal_xml(
        "PublishedArtifact",
        xml,
    );
    ArtifactAudit {
        path,
        present: true,
        well_formed: true,
        root: Some(root.tag_name().name().to_string()),
        declared_version: root.attribute("version").map(str::to_string),
        byte_len: xml.len(),
        sha256: Some(sha256::hex(xml.as_bytes())),
        note_count: document
            .descendants()
            .filter(|node| node.is_element() && node.tag_name().name() == "note")
            .count(),
        chord_note_count: document
            .descendants()
            .filter(|node| node.is_element() && node.tag_name().name() == "note")
            .filter(|note| {
                note.children()
                    .any(|child| child.is_element() && child.tag_name().name() == "chord")
            })
            .count(),
        chord_predecessor_duration_violations: temporal
            .rows
            .iter()
            .map(|row| row.chord_duration_conflicts)
            .sum(),
        chord_violations_with_intervening_forward: temporal
            .rows
            .iter()
            .map(|row| row.chord_conflicts_after_forward)
            .sum(),
        chord_violations_with_intervening_backup: temporal
            .rows
            .iter()
            .map(|row| row.chord_conflicts_after_backup)
            .sum(),
        temporal_replay_supported: temporal.parse_ok && temporal.score_partwise,
        cursor_overflows: temporal.rows.iter().map(|row| row.overflow_events).sum(),
        note_end_overflows: temporal
            .rows
            .iter()
            .map(|row| row.note_end_overflow_events)
            .sum(),
        backup_underflows: temporal
            .rows
            .iter()
            .map(|row| row.backup_underflow_events)
            .sum(),
        chord_temporal_conflicts: temporal
            .rows
            .iter()
            .map(|row| row.chord_temporal_conflicts)
            .sum(),
        max_cursor_overflow: temporal
            .rows
            .iter()
            .map(|row| row.cursor_max.saturating_sub(row.expected_duration))
            .max()
            .unwrap_or(0),
        max_note_end_overflow: temporal
            .rows
            .iter()
            .map(|row| row.max_note_end.saturating_sub(row.expected_duration))
            .max()
            .unwrap_or(0),
        parse_error: None,
    }
}

pub fn audit_file(path: &Path) -> ArtifactAudit {
    if !path.is_file() {
        return ArtifactAudit::missing(path.to_path_buf());
    }
    match fs::read_to_string(path) {
        Ok(xml) => audit_xml_text(path.to_path_buf(), &xml),
        Err(error) => ArtifactAudit {
            path: path.to_path_buf(),
            present: true,
            well_formed: false,
            root: None,
            declared_version: None,
            byte_len: 0,
            sha256: None,
            note_count: 0,
            chord_note_count: 0,
            chord_predecessor_duration_violations: 0,
            chord_violations_with_intervening_forward: 0,
            chord_violations_with_intervening_backup: 0,
            temporal_replay_supported: false,
            cursor_overflows: 0,
            note_end_overflows: 0,
            backup_underflows: 0,
            chord_temporal_conflicts: 0,
            max_cursor_overflow: 0,
            max_note_end_overflow: 0,
            parse_error: Some(error.to_string()),
        },
    }
}

pub fn build_for_score_title(score_title: Option<&str>) -> Option<ReadinessAudit> {
    let title = score_title?.trim();
    if title.is_empty() {
        return None;
    }
    let master = PathBuf::from(format!("{title}.musicxml"));
    let render = PathBuf::from(format!("{title}.render.musicxml"));
    Some(ReadinessAudit {
        master: audit_file(&master),
        render: audit_file(&render),
        xsd31_readiness:
            Xsd31Readiness::HistoricalProfileQualifiedNotProductAuthority,
        xsd40_readiness: Xsd40Readiness::DelegatedToPersistedProductContractGate,
    })
}

pub fn build_for_artifact_locators(master: &Path, render: &Path) -> ReadinessAudit {
    ReadinessAudit {
        master: audit_file(master),
        render: audit_file(render),
        xsd31_readiness:
            Xsd31Readiness::HistoricalProfileQualifiedNotProductAuthority,
        xsd40_readiness: Xsd40Readiness::DelegatedToPersistedProductContractGate,
    }
}

pub fn append_report_section(
    out: &mut String,
    score_title: Option<&str>,
    authoritative_master: Option<&Path>,
    authoritative_render: Option<&Path>,
    generator_identity: Option<
        &crate::audit_only::musicxml_generator_snapshot_identity::GeneratorSnapshotIdentity,
    >,
) {
    let (audit, locator_source, authoritative_locator_bound) =
        match (authoritative_master, authoritative_render) {
            (Some(master), Some(render)) => (
                build_for_artifact_locators(master, render),
                "source_plan",
                true,
            ),
            _ => {
                let Some(audit) = build_for_score_title(score_title) else {
                    return;
                };
                (audit, "reconstructed_from_score_title", false)
            }
        };

    out.push_str("MUSICXML-4.0-CONFORMANCE-FOUNDATION-AUDIT\n");
    out.push_str("=========================================\n");
    let _ = writeln!(
        out,
        "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; ProductiveVersion: {PRODUCTIVE_VERSION}; MigrationTarget: none; SpecificationDate: {SPECIFICATION_DATE}; Release: {RELEASE_TAG}; ReleaseCommit: {RELEASE_COMMIT}; HistoricalBaselineVersion: {PRODUCTIVE_BASELINE_VERSION}."
    );
    out.push_str("HISTORICAL-MUSICXML-3.1-BASELINE-PROVENANCE\n");
    out.push_str("------------------------------------------------\n");
    let _ = writeln!(
        out,
        "HistoricalPublishedBaseline: Version {PRODUCTIVE_BASELINE_VERSION}; SpecificationDate {PRODUCTIVE_BASELINE_SPECIFICATION_DATE}; Release {PRODUCTIVE_BASELINE_RELEASE_TAG}; ReleaseCommit {PRODUCTIVE_BASELINE_RELEASE_COMMIT}; SchemaSource {PRODUCTIVE_BASELINE_SCHEMA_SOURCE}."
    );
    let _ = writeln!(
        out,
        "HistoricalBaselineValidationBoundary: PublishedMusicXml31SemanticInvariant; Xsd31: {}; ProductAuthority: none; NetworkAllowed: {}; WriterAuthority: {WRITER_AUTHORITY}; MusicMutation: no.",
        audit.xsd31_readiness.as_str(),
        if NETWORK_ALLOWED { "yes" } else { "no" },
    );
    let _ = writeln!(
        out,
        "HistoricalBaselineSchemaContract: required {}; exact local published bytes + SHA256 remain bound as historical regression evidence; productive format authority is MusicXML 4.0.",
        PRODUCTIVE_BASELINE_SCHEMA_FILES.join(", ")
    );
    let schema_byte_ingress = observe_published_baseline_schema_byte_ingress(Path::new("."));
    let _ = writeln!(
        out,
        "HistoricalBaselineSchemaByteIngress: CanonicalLocalRoot {}; RequiredFiles {}; LocalBytesComplete {}; PublishedDigestSetQualified {}; OriginalityQualified {}; OfficialSchemaByteProvenanceBound {}; UpstreamTag {}; UpstreamCommit {}; NetworkAllowed {}; FallbackReconstructionAllowed no.",
        PRODUCTIVE_BASELINE_LOCAL_SCHEMA_ROOT,
        PRODUCTIVE_BASELINE_SCHEMA_FILES.join(", "),
        yes_no(schema_byte_ingress.all_required_local_bytes_present),
        yes_no(schema_byte_ingress.all_required_published_digests_match),
        yes_no(schema_byte_ingress.originality_qualified),
        yes_no(schema_byte_ingress.official_schema_byte_provenance_bound),
        PRODUCTIVE_BASELINE_RELEASE_TAG,
        PRODUCTIVE_BASELINE_RELEASE_COMMIT,
        if NETWORK_ALLOWED { "yes" } else { "no" },
    );
    for file in &schema_byte_ingress.files {
        let _ = writeln!(
            out,
            "HistoricalBaselineSchemaLocalFile: File {}; Path {}; Present {}; Bytes {}; Sha256 {}; ExpectedBytes {}; ExpectedSha256 {}; PublishedDigestMatches {}; OriginalityQualified {}.",
            file.file_name,
            file.path.display(),
            yes_no(file.present),
            file.byte_len,
            file.sha256.as_deref().unwrap_or("none"),
            file.expected_byte_len,
            file.expected_sha256,
            yes_no(file.published_digest_matches),
            yes_no(file.published_digest_matches),
        );
    }
    let productive_schema_byte_ingress =
        observe_productive_schema_byte_ingress(Path::new("."));
    let _ = writeln!(
        out,
        "ProductiveSchemaByteIngress: Version {PRODUCTIVE_VERSION}; CanonicalLocalRoot {PRODUCTIVE_SCHEMA_LOCAL_ROOT}; RequiredFiles {}; LocalBytesComplete {}; PublishedDigestSetQualified {}; OriginalityQualified {}; OfficialSchemaByteProvenanceBound {}; UpstreamTag {RELEASE_TAG}; UpstreamCommit {RELEASE_COMMIT}; NetworkAllowed {}; FallbackReconstructionAllowed no.",
        PRODUCTIVE_SCHEMA_FILES.join(", "),
        yes_no(productive_schema_byte_ingress.all_required_local_bytes_present),
        yes_no(productive_schema_byte_ingress.all_required_published_digests_match),
        yes_no(productive_schema_byte_ingress.originality_qualified),
        yes_no(productive_schema_byte_ingress.official_schema_byte_provenance_bound),
        if NETWORK_ALLOWED { "yes" } else { "no" },
    );
    for file in &productive_schema_byte_ingress.files {
        let _ = writeln!(
            out,
            "ProductiveSchemaLocalFile: File {}; Path {}; Present {}; Bytes {}; Sha256 {}; ExpectedBytes {}; ExpectedSha256 {}; PublishedDigestMatches {}; OriginalityQualified {}.",
            file.file_name,
            file.path.display(),
            yes_no(file.present),
            file.byte_len,
            file.sha256.as_deref().unwrap_or("none"),
            file.expected_byte_len,
            file.expected_sha256,
            yes_no(file.published_digest_matches),
            yes_no(file.published_digest_matches),
        );
    }
    let _ = writeln!(
        out,
        "ArtifactLocatorContract: LocatorSource {locator_source}; AuthoritativeSourcePlanLocatorBound {}; LocatorQualified {}; MasterLocator {}; RenderLocator {}.",
        yes_no(authoritative_locator_bound),
        yes_no(authoritative_locator_bound),
        audit.master.path.display(),
        audit.render.path.display(),
    );
    let artifact_snapshot_qualified = authoritative_locator_bound
        && audit.master.present
        && audit.render.present
        && audit.master.sha256.is_some()
        && audit.render.sha256.is_some();
    let _ = writeln!(
        out,
        "ArtifactSnapshotProvenance: Stage {}; PersistedBytesObserved yes; MasterBytes {}; MasterSha256 {}; RenderBytes {}; RenderSha256 {}; SnapshotQualified {}.",
        crate::audit_only::musicxml_generator_snapshot_identity::pipeline_closure_state(),
        audit.master.byte_len,
        audit.master.sha256.as_deref().unwrap_or("none"),
        audit.render.byte_len,
        audit.render.sha256.as_deref().unwrap_or("none"),
        yes_no(artifact_snapshot_qualified),
    );
    let generator_identity_qualified =
        generator_identity.is_some_and(|identity| identity.qualified);
    if let Some(identity) = generator_identity {
        let _ = writeln!(
            out,
            "GeneratorIdentityContract: SoftwareVersion {}; SoftwareVersionOnly {}; Revision {}; TakeName {}; RevisionBound {}; SourceFingerprintScope {}; SourceFileCount {}; SourceFingerprintSha256 {}; SourceFingerprintBound {}; ExecutableSha256 {}; ExecutableFingerprintBound {}; GeneratorIdentitySha256 {}; GeneratorIdentityQualified {}; Reason {}.",
            identity.software_version,
            if identity.qualified { "no" } else { "yes" },
            identity.revision.as_deref().unwrap_or("none"),
            identity.take_name.as_deref().unwrap_or("none"),
            yes_no(identity.revision_bound()),
            crate::audit_only::musicxml_generator_snapshot_identity::source_fingerprint_scope(),
            identity.source_file_count,
            identity.source_fingerprint_sha256.as_deref().unwrap_or("none"),
            yes_no(identity.source_fingerprint_bound()),
            identity.executable_sha256.as_deref().unwrap_or("none"),
            yes_no(identity.executable_fingerprint_bound()),
            identity.generator_identity_sha256.as_deref().unwrap_or("none"),
            yes_no(identity.qualified),
            identity.reason,
        );
    } else {
        out.push_str(
            "GeneratorIdentityContract: SoftwareVersionOnly yes; RevisionBound no; SourceFingerprintBound no; ExecutableFingerprintBound no; GeneratorIdentityQualified no; Reason generator_identity_not_bound_by_caller.\n",
        );
    }
    let generator_identity_sha256 = generator_identity
        .and_then(|identity| identity.generator_identity_sha256.as_deref())
        .unwrap_or("none");
    let master_snapshot_identity_bound = authoritative_locator_bound
        && audit.master.present
        && audit.master.sha256.is_some()
        && generator_identity_qualified;
    let render_snapshot_identity_bound = authoritative_locator_bound
        && audit.render.present
        && audit.render.sha256.is_some()
        && generator_identity_qualified;
    let snapshot_identity_qualified =
        master_snapshot_identity_bound && render_snapshot_identity_bound;
    let _ = writeln!(
        out,
        "ArtifactSnapshotIdentityBinding: ArtifactRole Master; PipelineClosureState {}; SnapshotSha256 {}; GeneratorIdentitySha256 {}; Bound {}.",
        crate::audit_only::musicxml_generator_snapshot_identity::pipeline_closure_state(),
        audit.master.sha256.as_deref().unwrap_or("none"),
        generator_identity_sha256,
        yes_no(master_snapshot_identity_bound),
    );
    let _ = writeln!(
        out,
        "ArtifactSnapshotIdentityBinding: ArtifactRole Render; PipelineClosureState {}; SnapshotSha256 {}; GeneratorIdentitySha256 {}; Bound {}.",
        crate::audit_only::musicxml_generator_snapshot_identity::pipeline_closure_state(),
        audit.render.sha256.as_deref().unwrap_or("none"),
        generator_identity_sha256,
        yes_no(render_snapshot_identity_bound),
    );
    let _ = writeln!(
        out,
        "ArtifactSnapshotIdentityGate: MasterBound {}; RenderBound {}; SnapshotIdentityQualified {}.",
        yes_no(master_snapshot_identity_bound),
        yes_no(render_snapshot_identity_bound),
        yes_no(snapshot_identity_qualified),
    );
    let _ = writeln!(
        out,
        "ValidationBoundary: GateA EtuSchlopf persisted DeclaredVersion=={PRODUCTIVE_VERSION}; GateB SchemaSchlopf r021 persisted XSD40; Xsd40: {}; SamePersistedBytes yes; ShadowValidation none; NetworkAllowed: {}; WriterAuthority: {WRITER_AUTHORITY}; MusicMutation: no.",
        audit.xsd40_readiness.as_str(),
        if NETWORK_ALLOWED { "yes" } else { "no" },
    );
    let _ = writeln!(
        out,
        "PinnedSchemaContract: required {}; CanonicalLocalRoot {PRODUCTIVE_SCHEMA_LOCAL_ROOT}; exact local bytes + SHA256 are product evidence; container.xsd/catalog.xml/MXL are out of scope.",
        SCHEMA_FILES.join(", ")
    );
    let _ = writeln!(
        out,
        "PublishedSchemaReleaseAsset: Name {RELEASE_ASSET_NAME}; Bytes {RELEASE_ASSET_BYTES}; URL {RELEASE_ASSET_URL}; Release {RELEASE_TAG}; Commit {RELEASE_COMMIT}; LocalSchemaDigestSetQualified {}.",
        yes_no(productive_schema_byte_ingress.all_required_published_digests_match),
    );
    append_artifact(out, "Master", &audit.master);
    append_artifact(out, "Render", &audit.render);
    let total_violations = audit
        .master
        .chord_predecessor_duration_violations
        .saturating_add(audit.render.chord_predecessor_duration_violations);
    let declared40 = artifact_declares_40(&audit.master) && artifact_declares_40(&audit.render);
    let _ = writeln!(
        out,
        "ProductiveContractObservation: DeclaredVersion40 {}; ChordPredecessorDurationViolations {}; Xsd40Authority SchemaSchlopf-r021-PersistedByteXsd40Gate; Xsd40Qualification delegated_to_product_gate; LocatorQualified {}; SnapshotQualified {}; GeneratorIdentityQualified {}; WriteAuthority none.",
        if declared40 { "yes" } else { "no" },
        total_violations,
        yes_no(authoritative_locator_bound),
        yes_no(artifact_snapshot_qualified),
        yes_no(generator_identity_qualified),
    );
    out.push_str("RuleId: MusicXmlChordPredecessorDuration; VersionScope: MusicXML 3.1 and 4.0 product evidence; MusicXml40Exclusive: no; HistoricalFirstVersion: unqualified; EvidenceClass: PublishedSpecification; Invariant: chord_duration <= predecessor_note_duration; RepairAuthority: none.\n\n");
    out.push_str("FinalSerializedTemporalPolicy: persisted Master and Render artifacts are observed independently of PDF-source selection; cursor extent and note/chord extent share the same read-only replay; partial backups are diagnostic rather than errors; implicit measures do not gain a universal final-cursor equality requirement; RepairAuthority none.\n\n");
    out.push_str(&crate::audit_only::pianistic_execution_score::build_for_score_title(score_title).text);
    out.push_str(
        &crate::audit_only::pianistic_configuration_constraint_kernel::build_for_score_title(
            score_title,
        ),
    );
    out.push_str(
        &crate::audit_only::pianistic_execution_plan_witness::build_for_score_title(score_title),
    );
    out.push_str(
        &crate::audit_only::pianistic_symbolic_capability_reachability::build_for_score_title(
            score_title,
        ),
    );
    out.push_str(
        &crate::audit_only::pianistic_capability_evidence_demand::build_for_score_title(
            score_title,
        ),
    );
    out.push_str(
        &crate::audit_only::pianistic_capability_model_binding::build_for_score_title(score_title),
    );
    out.push_str(
        &crate::audit_only::pianistic_reference_assumption_model::build_for_score_title(
            score_title,
        ),
    );
    out.push_str(
        &crate::audit_only::pianistic_reference_assumption_witness_integration::build_for_score_title(
            score_title,
        ),
    );
    out.push_str(
        &crate::audit_only::pianistic_static_capability_frontier::build_for_score_title(
            score_title,
        ),
    );
    out.push_str(
        &crate::audit_only::pianistic_temporal_transition_demand_frontier::build_for_score_title(
            score_title,
        ),
    );
    out.push_str(
        &crate::audit_only::pianistic_keyboard_state_transition_frontier::build_for_score_title(
            score_title,
        ),
    );
    out.push_str(
        &crate::audit_only::pianistic_hand_configuration_reference_bracket::build_for_score_title(
            score_title,
        ),
    );
    out.push_str(
        &crate::audit_only::pianistic_empirical_calibration_profile_contract::build_for_score_title(
            score_title,
        ),
    );
}

fn append_artifact(out: &mut String, label: &str, audit: &ArtifactAudit) {
    let _ = writeln!(
        out,
        "{label}: Path {}; Present {}; WellFormed {}; Root {}; DeclaredVersion {}; Bytes {}; Sha256 {}; Notes {}; ChordNotes {}; ChordPredecessorDurationViolations {}; InterveningForwardViolations {}; InterveningBackupViolations {}; TemporalReplaySupported {}; CursorOverflow {}; MaxCursorOverflow {}; NoteEndOverflow {}; MaxNoteEndOverflow {}; BackupUnderflow {}; ChordTemporalConflict {}; ParseError {}.",
        audit.path.display(),
        yes_no(audit.present),
        yes_no(audit.well_formed),
        audit.root.as_deref().unwrap_or("none"),
        audit.declared_version.as_deref().unwrap_or("none"),
        audit.byte_len,
        audit.sha256.as_deref().unwrap_or("none"),
        audit.note_count,
        audit.chord_note_count,
        audit.chord_predecessor_duration_violations,
        audit.chord_violations_with_intervening_forward,
        audit.chord_violations_with_intervening_backup,
        yes_no(audit.temporal_replay_supported),
        audit.cursor_overflows,
        audit.max_cursor_overflow,
        audit.note_end_overflows,
        audit.max_note_end_overflow,
        audit.backup_underflows,
        audit.chord_temporal_conflicts,
        audit.parse_error.as_deref().unwrap_or("none"),
    );
}

fn artifact_declares_version(audit: &ArtifactAudit, version: &str) -> bool {
    audit.present
        && audit.well_formed
        && matches!(
            audit.root.as_deref(),
            Some("score-partwise") | Some("score-timewise")
        )
        && audit.declared_version.as_deref() == Some(version)
}

fn artifact_declares_40(audit: &ArtifactAudit) -> bool {
    artifact_declares_version(audit, TARGET_VERSION)
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score(notes: &str, version: &str) -> String {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><score-partwise version=\"{version}\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>24</divisions></attributes>{notes}</measure></part></score-partwise>"
        )
    }

    fn note(duration: u64) -> String {
        format!("<note><pitch><step>C</step><octave>4</octave></pitch><duration>{duration}</duration><voice>1</voice><type>quarter</type></note>")
    }

    fn chord(duration: u64) -> String {
        format!("<note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>{duration}</duration><voice>1</voice><type>quarter</type></note>")
    }

    #[test]
    fn equal_duration_chord_is_valid_for_published_duration_invariant() {
        let xml = score(&(note(24) + &chord(24)), "4.0");
        let audit = audit_xml_text(PathBuf::from("equal.musicxml"), &xml);
        assert!(audit.well_formed);
        assert_eq!(audit.chord_predecessor_duration_violations, 0);
    }

    #[test]
    fn shorter_chord_tone_is_valid_and_must_not_be_forced_equal() {
        let xml = score(&(note(24) + &chord(12)), "4.0");
        let audit = audit_xml_text(PathBuf::from("shorter.musicxml"), &xml);
        assert_eq!(audit.chord_predecessor_duration_violations, 0);
    }

    #[test]
    fn longer_chord_tone_is_detected() {
        let xml = score(&(note(12) + &chord(24)), "4.0");
        let audit = audit_xml_text(PathBuf::from("longer.musicxml"), &xml);
        assert_eq!(audit.chord_predecessor_duration_violations, 1);
    }

    #[test]
    fn intervening_forward_is_classified_without_becoming_a_repair_rule() {
        let xml = score(
            &(note(12) + "<forward><duration>12</duration></forward>" + &chord(24)),
            "4.0",
        );
        let audit = audit_xml_text(PathBuf::from("forward.musicxml"), &xml);
        assert_eq!(audit.chord_predecessor_duration_violations, 1);
        assert_eq!(audit.chord_violations_with_intervening_forward, 1);
    }

    #[test]
    fn productive_31_baseline_preserves_published_chord_duration_invariant() {
        let equal = score(&(note(24) + &chord(24)), PRODUCTIVE_BASELINE_VERSION);
        let shorter = score(&(note(24) + &chord(12)), PRODUCTIVE_BASELINE_VERSION);
        let longer = score(&(note(12) + &chord(24)), PRODUCTIVE_BASELINE_VERSION);

        let equal_audit = audit_xml_text(PathBuf::from("baseline31_equal.musicxml"), &equal);
        let shorter_audit =
            audit_xml_text(PathBuf::from("baseline31_shorter.musicxml"), &shorter);
        let longer_audit = audit_xml_text(PathBuf::from("baseline31_longer.musicxml"), &longer);

        assert_eq!(equal_audit.chord_predecessor_duration_violations, 0);
        assert_eq!(shorter_audit.chord_predecessor_duration_violations, 0);
        assert_eq!(longer_audit.chord_predecessor_duration_violations, 1);
    }

    #[test]
    fn productive_31_baseline_forward_case_is_additive_to_40_horizon_fixture() {
        let xml = score(
            &(note(12) + "<forward><duration>12</duration></forward>" + &chord(24)),
            PRODUCTIVE_BASELINE_VERSION,
        );
        let audit = audit_xml_text(PathBuf::from("baseline31_forward.musicxml"), &xml);
        assert_eq!(audit.chord_predecessor_duration_violations, 1);
        assert_eq!(audit.chord_violations_with_intervening_forward, 1);
    }

    #[test]
    fn declared_version_is_a_separate_readiness_dimension() {
        let xml = score(&(note(24) + &chord(24)), "3.1");
        let audit = audit_xml_text(PathBuf::from("version.musicxml"), &xml);
        assert!(audit.well_formed);
        assert_eq!(audit.declared_version.as_deref(), Some("3.1"));
        assert!(artifact_declares_version(
            &audit,
            PRODUCTIVE_BASELINE_VERSION
        ));
        assert!(!artifact_declares_40(&audit));
    }

    #[test]
    fn malformed_xml_is_reported_without_mutation() {
        let audit = audit_xml_text(PathBuf::from("broken.musicxml"), "<score-partwise>");
        assert!(audit.present);
        assert!(!audit.well_formed);
        assert!(audit.parse_error.is_some());
    }

    #[test]
    fn xsd_readiness_reports_historical_and_delegated_authorities() {
        assert_eq!(
            Xsd31Readiness::HistoricalProfileQualifiedNotProductAuthority
                .as_str(),
            "HISTORICAL_PROFILE_QUALIFIED_NOT_PRODUCT_AUTHORITY"
        );
        assert_eq!(PRODUCTIVE_BASELINE_VERSION, "3.1");
        assert_eq!(PRODUCTIVE_BASELINE_RELEASE_TAG, "v3.1");
        assert_eq!(
            PRODUCTIVE_BASELINE_RELEASE_COMMIT,
            "8bbe8e50606bf99317a86e9e1637618d6bdd1997"
        );
        assert_eq!(
            PRODUCTIVE_BASELINE_SCHEMA_FILES,
            ["musicxml.xsd", "xml.xsd", "xlink.xsd"]
        );
        assert_eq!(
            Xsd40Readiness::DelegatedToPersistedProductContractGate.as_str(),
            "DELEGATED_TO_ETUSCHLOPF_PERSISTED_PRODUCT_CONTRACT_GATE_AND_SCHEMASCHLOPF_R021"
        );
        assert_eq!(RELEASE_ASSET_NAME, "musicxml-4.0.zip");
        assert_eq!(RELEASE_ASSET_BYTES, 192_235);
        assert!(RELEASE_ASSET_URL.ends_with("/v4.0/musicxml-4.0.zip"));
        const { assert!(!NETWORK_ALLOWED); }
        assert_eq!(WRITER_AUTHORITY, "none");
    }

    #[test]
    fn published_31_schema_byte_ingress_is_fail_closed_and_hashes_only_local_bytes() {
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_musicxml31_schema_byte_ingress_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        let schema_root = root.join(PRODUCTIVE_BASELINE_LOCAL_SCHEMA_ROOT);
        std::fs::create_dir_all(&schema_root).expect("create schema ingress fixture directory");
        std::fs::write(schema_root.join("musicxml.xsd"), b"musicxml-31")
            .expect("write musicxml fixture");
        std::fs::write(schema_root.join("xml.xsd"), b"xml-namespace")
            .expect("write xml fixture");

        let incomplete = observe_published_baseline_schema_byte_ingress(&root);
        assert!(!incomplete.all_required_local_bytes_present);
        assert_eq!(incomplete.files.len(), 3);
        assert!(incomplete.files[0].present);
        let musicxml_hash = sha256::hex(b"musicxml-31");
        assert_eq!(
            incomplete.files[0].sha256.as_deref(),
            Some(musicxml_hash.as_str())
        );
        assert!(!incomplete.files[2].present);
        assert_eq!(incomplete.files[2].sha256, None);

        std::fs::write(schema_root.join("xlink.xsd"), b"xlink-namespace")
            .expect("write xlink fixture");
        let complete = observe_published_baseline_schema_byte_ingress(&root);
        let _ = std::fs::remove_dir_all(&root);

        assert!(complete.all_required_local_bytes_present);
        assert!(complete.files.iter().all(|file| file.present));
        assert!(complete.files.iter().all(|file| file.sha256.is_some()));
        assert!(!complete.all_required_published_digests_match);
        assert!(!complete.originality_qualified);
        assert!(!complete.official_schema_byte_provenance_bound);
        const { assert!(!NETWORK_ALLOWED); }
    }

    #[test]
    fn published_31_schema_repository_bytes_match_pinned_upstream_digests() {
        let observed = observe_published_baseline_schema_byte_ingress(Path::new(env!(
            "CARGO_MANIFEST_DIR"
        )));

        assert!(observed.all_required_local_bytes_present);
        assert!(observed.all_required_published_digests_match);
        assert!(observed.originality_qualified);
        assert!(observed.official_schema_byte_provenance_bound);
        assert_eq!(observed.files.len(), 3);
        assert!(observed.files.iter().all(|file| {
            file.present
                && file.byte_len == file.expected_byte_len
                && file.sha256.as_deref() == Some(file.expected_sha256)
                && file.published_digest_matches
        }));
        assert_eq!(
            PRODUCTIVE_BASELINE_RELEASE_COMMIT,
            "8bbe8e50606bf99317a86e9e1637618d6bdd1997"
        );
        const { assert!(!NETWORK_ALLOWED); }
    }

    #[test]
    fn published_31_schema_byte_ingress_contract_uses_fixed_repository_relative_paths() {
        assert_eq!(
            PRODUCTIVE_BASELINE_LOCAL_SCHEMA_ROOT,
            "docs/normative_sources/musicxml/3.1/schema"
        );
        assert!(!PRODUCTIVE_BASELINE_LOCAL_SCHEMA_ROOT.starts_with('/'));
        assert!(!PRODUCTIVE_BASELINE_LOCAL_SCHEMA_ROOT.contains(".."));
        assert_eq!(
            PRODUCTIVE_BASELINE_SCHEMA_FILES,
            ["musicxml.xsd", "xml.xsd", "xlink.xsd"]
        );
        assert_eq!(
            PRODUCTIVE_BASELINE_SCHEMA_EXPECTATIONS,
            [
                (
                    "musicxml.xsd",
                    335_762,
                    "49ae9e34e04ba5b72004687994f70d598323eb3353eadb62faedcc9e9dc4bd7f",
                ),
                (
                    "xml.xsd",
                    5_695,
                    "616a3077df5cfc954ac74a75abe9697b95eef7a85dbe09367d995a483e840eb5",
                ),
                (
                    "xlink.xsd",
                    2_280,
                    "67c7f7a02491babc51e11c10268150b1a76070da36d5abe88e68515009611966",
                ),
            ]
        );
        assert!(PRODUCTIVE_BASELINE_SCHEMA_FILES
            .iter()
            .all(|file| !file.contains('/') && !file.contains('\\')));
    }

    #[test]
    fn productive_40_schema_contract_is_exactly_three_score_xsds() {
        assert_eq!(
            PRODUCTIVE_SCHEMA_LOCAL_ROOT,
            "docs/normative_sources/musicxml/4.0/schema"
        );
        assert_eq!(
            PRODUCTIVE_SCHEMA_FILES,
            ["musicxml.xsd", "xml.xsd", "xlink.xsd"]
        );
        assert_eq!(
            PRODUCTIVE_SCHEMA_EXPECTATIONS,
            [
                (
                    "musicxml.xsd",
                    379_505,
                    "bfe37ed25a9ec00e6f2591d53df260b84efe12aed209ba3ac0a76f9287665a99",
                ),
                (
                    "xml.xsd",
                    5_695,
                    "616a3077df5cfc954ac74a75abe9697b95eef7a85dbe09367d995a483e840eb5",
                ),
                (
                    "xlink.xsd",
                    2_280,
                    "6e601f8eeb41618b50e4c7f944dff754e57ea43b602755470dda24c9c2f6df92",
                ),
            ]
        );
        assert!(!SCHEMA_FILES.contains(&"container.xsd"));
        assert!(!SCHEMA_FILES.contains(&"catalog.xml"));
    }

    #[test]
    fn composed_report_does_not_claim_deferred_reference_integration_when_integration_runs() {
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_reference_integration_projection_truth_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create temp report fixture directory");
        let title = root.join("projection_truth");
        let xml = "<?xml version=\"1.0\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><direction><sound tempo=\"120\"/></direction><note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        std::fs::write(title.with_extension("musicxml"), xml).expect("write master fixture");
        std::fs::write(format!("{}.render.musicxml", title.display()), xml)
            .expect("write render fixture");

        let mut report = String::new();
        let title_text = title.to_string_lossy();
        append_report_section(&mut report, Some(title_text.as_ref()), None, None, None);
        let _ = std::fs::remove_dir_all(&root);

        assert!(report.contains("HISTORICAL-MUSICXML-3.1-BASELINE-PROVENANCE"));
        assert!(report.contains("ProductiveVersion: 4.0; MigrationTarget: none;"));
        assert!(report.contains("ProductiveContractObservation: DeclaredVersion40 yes;"));
        assert!(report.contains("AuthoritativeSourcePlanLocatorBound no; LocatorQualified no;"));
        assert!(report.contains("GeneratorIdentityQualified no;"));
        assert!(report.contains(
            "VersionScope: MusicXML 3.1 and 4.0 product evidence; MusicXml40Exclusive: no;"
        ));
        assert!(!report.contains(
            "VersionScope: MusicXML 4.0; EvidenceClass: PublishedSpecification;"
        ));
        assert!(report.contains("PIANISTIC-REFERENCE-ASSUMPTION-WITNESS-INTEGRATION-AUDIT"));
        assert!(report.contains("ReferenceWitnessIntegrationRuntimeIntegrity: PASS;"));
        assert!(report.contains(
            "IntegrationStateOwner PianisticReferenceAssumptionWitnessIntegration; IntegrationStateClaim none"
        ));
        assert!(!report.contains("WitnessIntegration DEFERRED_TO_NEXT_TAKE"));
        assert!(!report.contains("solver integration follows separately"));
    }
    #[test]
    fn authoritative_locators_bind_exact_persisted_bytes_and_snapshot_identity() {
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_musicxml_source_plan_locator_binding_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create locator fixture directory");
        let master = root.join("authoritative-master.musicxml");
        let render = root.join("authoritative-render.musicxml");
        let xml = "<?xml version=\"1.0\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        std::fs::write(&master, xml).expect("write authoritative master fixture");
        std::fs::write(&render, xml).expect("write authoritative render fixture");

        let mut report = String::new();
        append_report_section(
            &mut report,
            Some("wrong-title"),
            Some(&master),
            Some(&render),
            None,
        );
        let _ = std::fs::remove_dir_all(&root);

        assert!(report.contains("LocatorSource source_plan; AuthoritativeSourcePlanLocatorBound yes; LocatorQualified yes;"));
        assert!(report.contains("ArtifactSnapshotProvenance: Stage final_report_generation_after_final_notation_layout_snapshot_close_and_renderer;"));
        assert!(report.contains("SnapshotQualified yes."));
        assert!(report.contains("ProductiveContractObservation: DeclaredVersion40 yes; ChordPredecessorDurationViolations 0; Xsd40Authority SchemaSchlopf-r021-PersistedByteXsd40Gate; Xsd40Qualification delegated_to_product_gate; LocatorQualified yes; SnapshotQualified yes; GeneratorIdentityQualified no;"));
        let locator_line = report
            .lines()
            .find(|line| line.starts_with("ArtifactLocatorContract:"))
            .expect("artifact locator contract line");
        assert!(locator_line.contains(&format!("MasterLocator {}", master.display())));
        assert!(locator_line.contains(&format!("RenderLocator {}", render.display())));
        assert!(!locator_line.contains("wrong-title.musicxml"));
    }

    #[test]
    fn qualified_generator_identity_binds_master_and_render_snapshot_roles() {
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_musicxml_generator_snapshot_binding_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create snapshot binding fixture directory");
        let master = root.join("master.musicxml");
        let render = root.join("render.musicxml");
        let xml = "<?xml version=\"1.0\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        std::fs::write(&master, xml).expect("write master fixture");
        std::fs::write(&render, xml).expect("write render fixture");
        let identity =
            crate::audit_only::musicxml_generator_snapshot_identity::GeneratorSnapshotIdentity {
                software_version: "0.10.1839".to_string(),
                revision: Some("r550".to_string()),
                take_name: Some("MusicXmlGeneratorSnapshotIdentityProvenanceClosure".to_string()),
                source_file_count: 760,
                source_fingerprint_sha256: Some("11".repeat(32)),
                executable_sha256: Some("22".repeat(32)),
                generator_identity_sha256: Some("33".repeat(32)),
                qualified: true,
                reason: "test_identity_qualified".to_string(),
            };

        let mut report = String::new();
        append_report_section(
            &mut report,
            Some("fixture"),
            Some(&master),
            Some(&render),
            Some(&identity),
        );
        let _ = std::fs::remove_dir_all(&root);

        assert!(report.contains("SoftwareVersionOnly no; Revision r550;"));
        assert!(report.contains("SourceFingerprintBound yes;"));
        assert!(report.contains("ExecutableFingerprintBound yes;"));
        assert!(report.contains("GeneratorIdentityQualified yes;"));
        assert!(report.contains("ArtifactSnapshotIdentityBinding: ArtifactRole Master;"));
        assert!(report.contains("ArtifactSnapshotIdentityBinding: ArtifactRole Render;"));
        assert!(report.contains(
            "ArtifactSnapshotIdentityGate: MasterBound yes; RenderBound yes; SnapshotIdentityQualified yes."
        ));
        assert!(report.contains(
            "GeneratorIdentitySha256 3333333333333333333333333333333333333333333333333333333333333333; Bound yes."
        ));
        assert!(report.contains(
            "ProductiveContractObservation: DeclaredVersion40 yes; ChordPredecessorDurationViolations 0; Xsd40Authority SchemaSchlopf-r021-PersistedByteXsd40Gate; Xsd40Qualification delegated_to_product_gate; LocatorQualified yes; SnapshotQualified yes; GeneratorIdentityQualified yes; WriteAuthority none."
        ));
    }

}
