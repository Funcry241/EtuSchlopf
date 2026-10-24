#![allow(dead_code)]

#[path = "../build_gate/formal_green_contract.rs"]
mod formal_green_contract;
use crate::sha256;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub const PUBLIC_ARTIFACT_MARKER_RELATIVE_PATH: &str =
    "target/etuschlopf_public_artifact_transaction_pending.txt";
pub const PUBLIC_ARTIFACT_TRANSACTION_ROOT_RELATIVE_PATH: &str =
    "target/etuschlopf_public_artifact_transactions";

const JOURNAL_SCHEMA_VERSION: u32 = 2;
const JOURNAL_PENDING_FILE: &str = "journal.v2.pending";
const JOURNAL_RESTORING_FILE: &str = "journal.v2.restoring";
const JOURNAL_RESTORED_FILE: &str = "journal.v2.restored";
const JOURNAL_COMMITTED_FILE: &str = "journal.v2.committed";
const JOURNAL_TEMP_FILE: &str = "journal.v2.tmp";
const JOURNAL_MAX_BYTES: u64 = 64 * 1024;
const ARTIFACT_COUNT: usize = 4;
const MAX_PUBLIC_ARTIFACT_BYTES: u64 = 256 * 1024 * 1024;

pub type RecoveryResult<T> = Result<T, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryOutcome {
    NoPendingJournal,
    Restored,
    RestoredMarkerCleared,
    CommittedMarkerCleared,
}

impl RecoveryOutcome {
    pub const fn performed_recovery(self) -> bool {
        !matches!(self, Self::NoPendingJournal)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPendingJournal => "no-pending-journal",
            Self::Restored => "restored",
            Self::RestoredMarkerCleared => "restored-marker-cleared",
            Self::CommittedMarkerCleared => "committed-marker-cleared",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum JournalState {
    Pending,
    Restoring,
    Restored,
    Committed,
}

impl JournalState {
    fn parse(value: &str) -> RecoveryResult<Self> {
        match value {
            "pending" => Ok(Self::Pending),
            "restoring" => Ok(Self::Restoring),
            "restored" => Ok(Self::Restored),
            "committed" => Ok(Self::Committed),
            other => Err(format!(
                "Public-Artifact-Recovery: unbekannter Journalzustand {other:?}; fail-closed"
            )),
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Restoring => "restoring",
            Self::Restored => "restored",
            Self::Committed => "committed",
        }
    }

    const fn file_name(self) -> &'static str {
        match self {
            Self::Pending => JOURNAL_PENDING_FILE,
            Self::Restoring => JOURNAL_RESTORING_FILE,
            Self::Restored => JOURNAL_RESTORED_FILE,
            Self::Committed => JOURNAL_COMMITTED_FILE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArtifactRecord {
    index: usize,
    existed_before: bool,
    backup_bytes: u64,
    backup_sha256: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PublicArtifactJournal {
    schema_version: u32,
    transaction_id: String,
    state: JournalState,
    project_root: PathBuf,
    project_name: String,
    producer_version: String,
    artifacts: Vec<ArtifactRecord>,
}

impl PublicArtifactJournal {
    fn with_state(&self, state: JournalState) -> Self {
        let mut next = self.clone();
        next.state = state;
        next
    }

    fn same_immutable_contract(&self, other: &Self) -> bool {
        self.schema_version == other.schema_version
            && self.transaction_id == other.transaction_id
            && self.project_root == other.project_root
            && self.project_name == other.project_name
            && self.producer_version == other.producer_version
            && self.artifacts == other.artifacts
    }
}

#[derive(Debug)]
pub struct PublicArtifactTransaction {
    journal: PublicArtifactJournal,
    transaction_root: PathBuf,
    marker_path: PathBuf,
}

pub struct ProjectRunLock {
    _file: File,
}

pub fn acquire_project_run_lock(project_root: &Path) -> RecoveryResult<ProjectRunLock> {
    let canonical_project_root = canonical_directory(project_root, "Projektwurzel")?;
    let lock_path = canonical_project_root
        .join("target/etuschlopf_public_artifact_single_project.lock");
    if let Some(parent) = lock_path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "Public-Artifact-Recovery: Lock-Ordner nicht erstellbar {}: {error}",
                parent.display()
            )
        })?;
    }
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        options.share_mode(0);
    }
    let file = options.open(&lock_path).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: exklusiver Einzelprojekt-Lock nicht erwerbbar {}: {error}",
            lock_path.display()
        )
    })?;
    Ok(ProjectRunLock { _file: file })
}

pub fn begin_public_artifact_transaction(
    project_root: &Path,
    project_name: &str,
    producer_version: &str,
) -> RecoveryResult<PublicArtifactTransaction> {
    validate_project_name(project_name)?;
    validate_producer_version(producer_version)?;
    let canonical_project_root = canonical_directory(project_root, "Projektwurzel")?;
    let marker_path = canonical_project_root.join(PUBLIC_ARTIFACT_MARKER_RELATIVE_PATH);
    if marker_path.exists() {
        return Err(
            "Public-Artifact-Recovery: neuer Snapshot bei offenem Pending-Marker verweigert; zuerst Recovery"
                .to_string(),
        );
    }

    let transaction_parent =
        canonical_project_root.join(PUBLIC_ARTIFACT_TRANSACTION_ROOT_RELATIVE_PATH);
    fs::create_dir_all(&transaction_parent).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: Transaktionswurzel nicht erstellbar {}: {error}",
            transaction_parent.display()
        )
    })?;
    let canonical_transaction_parent =
        canonical_directory(&transaction_parent, "Public-Artifact-Transaktionswurzel")?;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("Public-Artifact-Recovery: Systemzeit vor UNIX-Epoch: {error}"))?
        .as_nanos();
    let transaction_id = format!(
        "v{}_p{}_{}",
        producer_version.replace('.', "_"),
        std::process::id(),
        nanos
    );
    validate_transaction_id(&transaction_id)?;
    let transaction_root = canonical_transaction_parent.join(&transaction_id);
    fs::create_dir(&transaction_root).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: Transaktionsordner nicht exklusiv erstellbar {}: {error}",
            transaction_root.display()
        )
    })?;
    let backup_root = transaction_root.join("backup");
    fs::create_dir(&backup_root).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: Backup-Ordner nicht erstellbar {}: {error}",
            backup_root.display()
        )
    })?;
    let canonical_transaction_root =
        canonical_directory(&transaction_root, "Public-Artifact-Transaktion")?;
    let canonical_backup_root =
        canonical_directory(&backup_root, "Public-Artifact-Backupwurzel")?;

    let targets = public_artifact_paths(&canonical_project_root, project_name);
    let mut artifacts = Vec::with_capacity(ARTIFACT_COUNT);
    for (index, target_path) in targets.into_iter().enumerate() {
        match fs::symlink_metadata(&target_path) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() || !metadata.is_file() {
                    return Err(format!(
                        "Public-Artifact-Recovery: öffentliches Ziel ist keine reguläre Datei {}; Snapshot verweigert",
                        target_path.display()
                    ));
                }
                if metadata.len() > MAX_PUBLIC_ARTIFACT_BYTES {
                    return Err(format!(
                        "Public-Artifact-Recovery: öffentliches Ziel ist mit {} Bytes zu groß {}; Snapshot verweigert",
                        metadata.len(),
                        target_path.display()
                    ));
                }
                let bytes = fs::read(&target_path).map_err(|error| {
                    format!(
                        "Public-Artifact-Recovery: öffentliches Ziel nicht lesbar {}: {error}",
                        target_path.display()
                    )
                })?;
                let backup_path = canonical_backup_root.join(format!("artifact_{index:02}.bak"));
                write_new_synced_file(&backup_path, &bytes, "Public-Artifact-Backup")?;
                let readback = fs::read(&backup_path).map_err(|error| {
                    format!(
                        "Public-Artifact-Recovery: Backup-Readback fehlgeschlagen {}: {error}",
                        backup_path.display()
                    )
                })?;
                if readback != bytes {
                    return Err(format!(
                        "Public-Artifact-Recovery: Backup-Readback ist nicht bytegleich {}",
                        backup_path.display()
                    ));
                }
                artifacts.push(ArtifactRecord {
                    index,
                    existed_before: true,
                    backup_bytes: bytes.len() as u64,
                    backup_sha256: Some(sha256::hex(&bytes)),
                });
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                artifacts.push(ArtifactRecord {
                    index,
                    existed_before: false,
                    backup_bytes: 0,
                    backup_sha256: None,
                });
            }
            Err(error) => {
                return Err(format!(
                    "Public-Artifact-Recovery: öffentliches Ziel nicht prüfbar {}: {error}",
                    target_path.display()
                ));
            }
        }
    }

    let journal = PublicArtifactJournal {
        schema_version: JOURNAL_SCHEMA_VERSION,
        transaction_id,
        state: JournalState::Pending,
        project_root: canonical_project_root,
        project_name: project_name.to_string(),
        producer_version: producer_version.to_string(),
        artifacts,
    };
    validate_journal_binding(&journal, &journal.project_root, &canonical_transaction_root)?;
    validate_all_restore_inputs_before_mutation(&journal, &canonical_transaction_root)?;
    persist_state_journal(&canonical_transaction_root, &journal)?;
    let parsed = parse_journal_file(&canonical_transaction_root.join(JOURNAL_PENDING_FILE))?;
    if parsed != journal {
        return Err(
            "Public-Artifact-Recovery: Pending-Journal-Readback ist nicht exakt".to_string(),
        );
    }
    write_marker_atomically(&marker_path, &canonical_transaction_root)?;
    let marker_target = read_and_validate_marker(&journal.project_root, &marker_path)?;
    if marker_target != canonical_transaction_root {
        return Err(
            "Public-Artifact-Recovery: Pending-Marker-Readback bindet falsche Transaktion"
                .to_string(),
        );
    }
    Ok(PublicArtifactTransaction {
        journal,
        transaction_root: canonical_transaction_root,
        marker_path,
    })
}

pub fn commit_public_artifact_transaction(
    transaction: &PublicArtifactTransaction,
    expected_version: &str,
) -> RecoveryResult<()> {
    validate_new_public_set(&transaction.journal, expected_version)?;
    let committed = transaction.journal.with_state(JournalState::Committed);
    persist_state_journal(&transaction.transaction_root, &committed)?;
    let readback = parse_journal_file(
        &transaction
            .transaction_root
            .join(JOURNAL_COMMITTED_FILE),
    )?;
    if readback != committed {
        return Err(
            "Public-Artifact-Recovery: Commit-Journal-Readback ist nicht exakt".to_string(),
        );
    }
    remove_regular_file_if_exists(
        &transaction.marker_path,
        "Public-Artifact-Pending-Marker",
    )?;
    let _ = fs::remove_dir_all(&transaction.transaction_root);
    Ok(())
}

pub fn restore_public_artifact_transaction(
    transaction: &PublicArtifactTransaction,
) -> RecoveryResult<()> {
    let outcome = recover_pending_public_artifacts(&transaction.journal.project_root)?;
    if !outcome.performed_recovery() {
        return Err(
            "Public-Artifact-Recovery: expliziter Restore fand kein offenes Journal".to_string(),
        );
    }
    Ok(())
}

pub fn recover_pending_public_artifacts(
    project_root: &Path,
) -> RecoveryResult<RecoveryOutcome> {
    let marker_path = project_root.join(PUBLIC_ARTIFACT_MARKER_RELATIVE_PATH);
    if !marker_path.exists() {
        return Ok(RecoveryOutcome::NoPendingJournal);
    }
    reject_symlink_or_non_file(&marker_path, "Public-Artifact-Pending-Marker")?;
    let canonical_project_root = canonical_directory(project_root, "Projektwurzel")?;
    let transaction_root = read_and_validate_marker(&canonical_project_root, &marker_path)?;
    discard_interrupted_temp_journal_if_safe(&transaction_root)?;
    let journal = load_effective_journal(&transaction_root)?;
    validate_journal_binding(&journal, &canonical_project_root, &transaction_root)?;

    match journal.state {
        JournalState::Committed => {
            remove_regular_file_if_exists(&marker_path, "Public-Artifact-Pending-Marker")?;
            let _ = fs::remove_dir_all(&transaction_root);
            Ok(RecoveryOutcome::CommittedMarkerCleared)
        }
        JournalState::Restored => {
            verify_restored_public_set(&journal, &transaction_root)?;
            remove_regular_file_if_exists(&marker_path, "Public-Artifact-Pending-Marker")?;
            let _ = fs::remove_dir_all(&transaction_root);
            Ok(RecoveryOutcome::RestoredMarkerCleared)
        }
        JournalState::Pending | JournalState::Restoring => {
            validate_all_restore_inputs_before_mutation(&journal, &transaction_root)?;
            let restoring = journal.with_state(JournalState::Restoring);
            persist_state_journal(&transaction_root, &restoring)?;
            restore_public_set(&restoring, &transaction_root)?;
            verify_restored_public_set(&restoring, &transaction_root)?;
            let restored = restoring.with_state(JournalState::Restored);
            persist_state_journal(&transaction_root, &restored)?;
            remove_regular_file_if_exists(&marker_path, "Public-Artifact-Pending-Marker")?;
            let _ = fs::remove_dir_all(&transaction_root);
            Ok(RecoveryOutcome::Restored)
        }
    }
}

fn public_artifact_paths(project_root: &Path, project_name: &str) -> [PathBuf; ARTIFACT_COUNT] {
    [
        project_root.join(format!("{project_name}.musicxml")),
        project_root.join(format!("{project_name}.render.musicxml")),
        project_root.join(format!("{project_name}.pdf")),
        project_root.join(format!("{project_name}.result.txt")),
    ]
}

fn validate_project_name(value: &str) -> RecoveryResult<()> {
    if value.trim() != value
        || value.is_empty()
        || value.chars().count() > 180
        || value == "."
        || value == ".."
        || value.ends_with(' ')
        || value.ends_with('.')
        || value
            .chars()
            .any(|character| character.is_control() || "\\/:*?\"<>|".contains(character))
    {
        return Err(format!(
            "Public-Artifact-Recovery: Projektname {value:?} ist kein sicherer einzelner Dateistamm; fail-closed"
        ));
    }
    Ok(())
}

fn validate_new_public_set(
    journal: &PublicArtifactJournal,
    expected_version: &str,
) -> RecoveryResult<()> {
    validate_producer_version(expected_version)?;
    if journal.producer_version != expected_version {
        return Err(format!(
            "Public-Artifact-Recovery: Commit-Version {expected_version} stimmt nicht mit {} überein",
            journal.producer_version
        ));
    }
    let targets = public_artifact_paths(&journal.project_root, &journal.project_name);
    for target in &targets {
        reject_symlink_or_non_file(target, "neues Public-Artefakt")?;
        let metadata = fs::metadata(target).map_err(|error| {
            format!(
                "Public-Artifact-Recovery: neues Artefakt nicht prüfbar {}: {error}",
                target.display()
            )
        })?;
        if metadata.len() == 0 || metadata.len() > MAX_PUBLIC_ARTIFACT_BYTES {
            return Err(format!(
                "Public-Artifact-Recovery: neues Artefakt hat unzulässige Größe {}: {} Bytes",
                target.display(),
                metadata.len()
            ));
        }
    }

    let master = read_small_utf8_file(&targets[0], MAX_PUBLIC_ARTIFACT_BYTES, "neues Master-MusicXML")?;
    let render = read_small_utf8_file(&targets[1], MAX_PUBLIC_ARTIFACT_BYTES, "neues Render-MusicXML")?;
    let report = read_small_utf8_file(&targets[3], MAX_PUBLIC_ARTIFACT_BYTES, "neuer Ergebnisbericht")?;
    let version_marker = format!("<software>etuschlopf v{expected_version}</software>");
    if !master.contains(&version_marker) || !render.contains(&version_marker) {
        return Err(
            "Public-Artifact-Recovery: Master/Render sind nicht an die aktuelle Producer-Version gebunden"
                .to_string(),
        );
    }
    let pdf = fs::read(&targets[2])
        .map_err(|error| format!("Public-Artifact-Recovery: neues PDF nicht lesbar: {error}"))?;
    if !pdf.starts_with(b"%PDF-") {
        return Err(
            "Public-Artifact-Recovery: neues PDF hat keinen PDF-Dateikopf".to_string(),
        );
    }
    if !report.starts_with("ETUSCHLOPF-ERGEBNIS\n")
        || !report.contains(&format!("Werkzeug: etuschlopf {expected_version}"))
        || !report.contains(&format!("Partitur: {}", journal.project_name))
        || !formal_green_contract::report_header_is_formally_green(&report)
    {
        return Err(
            "Public-Artifact-Recovery: neuer Bericht ist nicht aktuell blocker-/warnungsfrei und an den laufenden Projektstamm gebunden"
                .to_string(),
        );
    }
    let pdf_hash = sha256::hex(&pdf);
    if !report.contains(&format!("PdfFileSha256 {pdf_hash}")) {
        return Err(
            "Public-Artifact-Recovery: Bericht bindet nicht den tatsächlich erzeugten PDF-Dateihash"
                .to_string(),
        );
    }
    Ok(())
}

fn read_and_validate_marker(
    canonical_project_root: &Path,
    marker_path: &Path,
) -> RecoveryResult<PathBuf> {
    let raw = read_small_utf8_file(marker_path, JOURNAL_MAX_BYTES, "Public-Artifact-Marker")?;
    let value = single_protocol_line(&raw, "Public-Artifact-Marker")?;
    if value.is_empty() {
        return Err("Public-Artifact-Recovery: leerer Pending-Marker; fail-closed".to_string());
    }
    let referenced = PathBuf::from(value);
    if !referenced.is_absolute() {
        return Err(format!(
            "Public-Artifact-Recovery: Pending-Marker muss einen absoluten Transaktionspfad enthalten: {}",
            referenced.display()
        ));
    }
    let canonical_transaction = canonical_directory(&referenced, "Public-Artifact-Transaktion")?;
    if referenced != canonical_transaction {
        return Err(format!(
            "Public-Artifact-Recovery: Pending-Marker enthält keinen kanonischen Transaktionspfad: {}; fail-closed",
            referenced.display()
        ));
    }
    let expected_parent = canonical_directory(
        &canonical_project_root.join(PUBLIC_ARTIFACT_TRANSACTION_ROOT_RELATIVE_PATH),
        "Public-Artifact-Transaktionswurzel",
    )?;
    if canonical_transaction.parent() != Some(expected_parent.as_path()) {
        return Err(format!(
            "Public-Artifact-Recovery: Pending-Marker verweist ausserhalb der gebundenen Transaktionswurzel: {}",
            canonical_transaction.display()
        ));
    }
    Ok(canonical_transaction)
}

fn discard_interrupted_temp_journal_if_safe(transaction_root: &Path) -> RecoveryResult<()> {
    let temp = transaction_root.join(JOURNAL_TEMP_FILE);
    if !temp.exists() {
        return Ok(());
    }
    reject_symlink_or_non_file(&temp, "temporäres Public-Artifact-Journal")?;
    let stable_exists = [
        JOURNAL_PENDING_FILE,
        JOURNAL_RESTORING_FILE,
        JOURNAL_RESTORED_FILE,
        JOURNAL_COMMITTED_FILE,
    ]
    .iter()
    .any(|name| transaction_root.join(name).is_file());
    if !stable_exists {
        return Err(format!(
            "Public-Artifact-Recovery: temporäres Journal ohne stabiles Journal gefunden {}; fail-closed",
            temp.display()
        ));
    }
    fs::remove_file(&temp).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: unterbrochenes temporäres Journal konnte nicht entfernt werden {}: {error}",
            temp.display()
        )
    })
}

fn load_effective_journal(transaction_root: &Path) -> RecoveryResult<PublicArtifactJournal> {
    let mut journals = Vec::new();
    for state in [
        JournalState::Pending,
        JournalState::Restoring,
        JournalState::Restored,
        JournalState::Committed,
    ] {
        let path = transaction_root.join(state.file_name());
        if path.exists() {
            reject_symlink_or_non_file(&path, "Public-Artifact-Journal")?;
            let parsed = parse_journal_file(&path)?;
            if parsed.state != state {
                return Err(format!(
                    "Public-Artifact-Recovery: Journaldatei {} trägt Zustand {}; fail-closed",
                    path.display(),
                    parsed.state.as_str()
                ));
            }
            journals.push(parsed);
        }
    }
    if journals.is_empty() {
        return Err(
            "Public-Artifact-Recovery: Pending-Marker ohne stabiles Journal; fail-closed"
                .to_string(),
        );
    }
    let first = journals[0].clone();
    if journals
        .iter()
        .skip(1)
        .any(|journal| !first.same_immutable_contract(journal))
    {
        return Err(
            "Public-Artifact-Recovery: Journalzustände widersprechen sich im unveränderlichen Vertrag; fail-closed"
                .to_string(),
        );
    }
    journals
        .into_iter()
        .max_by_key(|journal| journal.state)
        .ok_or_else(|| "Public-Artifact-Recovery: kein wirksames Journal".to_string())
}

fn parse_journal_file(path: &Path) -> RecoveryResult<PublicArtifactJournal> {
    let text = read_small_utf8_file(path, JOURNAL_MAX_BYTES, "Public-Artifact-Journal")?;
    parse_journal(&text)
}

fn parse_journal(text: &str) -> RecoveryResult<PublicArtifactJournal> {
    let mut fields = BTreeMap::new();
    for (index, raw) in text.lines().enumerate() {
        let (key, value) = raw.split_once('=').ok_or_else(|| {
            format!(
                "Public-Artifact-Recovery: Journalzeile {} ohne '='; fail-closed",
                index + 1
            )
        })?;
        if key.is_empty() || fields.insert(key.to_string(), value.to_string()).is_some() {
            return Err(format!(
                "Public-Artifact-Recovery: Journalfeld {key:?} leer oder doppelt; fail-closed"
            ));
        }
    }
    let expected = expected_field_names();
    let actual = fields.keys().cloned().collect::<Vec<_>>();
    if actual != expected {
        return Err(
            "Public-Artifact-Recovery: Journalfeldmenge ist nicht exakt; fail-closed".to_string(),
        );
    }

    let schema_version = required(&fields, "schema_version")?
        .parse::<u32>()
        .map_err(|error| format!("Public-Artifact-Recovery: schema_version ungültig: {error}"))?;
    if schema_version != JOURNAL_SCHEMA_VERSION {
        return Err(format!(
            "Public-Artifact-Recovery: Journalschema {schema_version} statt {JOURNAL_SCHEMA_VERSION}; fail-closed"
        ));
    }
    let transaction_id = required(&fields, "transaction_id")?.to_string();
    validate_transaction_id(&transaction_id)?;
    let state = JournalState::parse(required(&fields, "state")?)?;
    let project_root = PathBuf::from(decode_hex_utf8(
        required(&fields, "project_root_hex")?,
        "project_root_hex",
    )?);
    let project_name = decode_hex_utf8(
        required(&fields, "project_name_hex")?,
        "project_name_hex",
    )?;
    validate_project_name(&project_name)?;
    let producer_version = required(&fields, "producer_version")?.to_string();
    validate_producer_version(&producer_version)?;
    let mut artifacts = Vec::with_capacity(ARTIFACT_COUNT);
    for index in 0..ARTIFACT_COUNT {
        let existed_before = parse_bool(required(
            &fields,
            &format!("artifact.{index}.existed_before"),
        )?)?;
        let backup_bytes = required(&fields, &format!("artifact.{index}.backup_bytes"))?
            .parse::<u64>()
            .map_err(|error| {
                format!("Public-Artifact-Recovery: backup_bytes ungültig: {error}")
            })?;
        let hash = required(&fields, &format!("artifact.{index}.backup_sha256"))?;
        let backup_sha256 = if hash == "-" {
            None
        } else if is_sha256(hash) {
            Some(hash.to_string())
        } else {
            return Err(
                "Public-Artifact-Recovery: ungültiger Backup-SHA-256; fail-closed".to_string(),
            );
        };
        if existed_before != backup_sha256.is_some()
            || (!existed_before && backup_bytes != 0)
            || backup_bytes > MAX_PUBLIC_ARTIFACT_BYTES
        {
            return Err(format!(
                "Public-Artifact-Recovery: inkonsistenter Backupvertrag für Artefakt {index}; fail-closed"
            ));
        }
        artifacts.push(ArtifactRecord {
            index,
            existed_before,
            backup_bytes,
            backup_sha256,
        });
    }
    Ok(PublicArtifactJournal {
        schema_version,
        transaction_id,
        state,
        project_root,
        project_name,
        producer_version,
        artifacts,
    })
}

fn expected_field_names() -> Vec<String> {
    let mut out = vec![
        "producer_version".to_string(),
        "project_name_hex".to_string(),
        "project_root_hex".to_string(),
        "schema_version".to_string(),
        "state".to_string(),
        "transaction_id".to_string(),
    ];
    for index in 0..ARTIFACT_COUNT {
        out.push(format!("artifact.{index}.backup_bytes"));
        out.push(format!("artifact.{index}.backup_sha256"));
        out.push(format!("artifact.{index}.existed_before"));
    }
    out.sort();
    out
}

fn required<'a>(fields: &'a BTreeMap<String, String>, key: &str) -> RecoveryResult<&'a str> {
    fields
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("Public-Artifact-Recovery: Pflichtfeld {key:?} fehlt"))
}

fn validate_journal_binding(
    journal: &PublicArtifactJournal,
    canonical_project_root: &Path,
    transaction_root: &Path,
) -> RecoveryResult<()> {
    if journal.project_root != canonical_project_root {
        return Err(format!(
            "Public-Artifact-Recovery: Journal-Projektwurzel {} ist nicht die aktuelle kanonische Projektwurzel {}; fail-closed",
            journal.project_root.display(),
            canonical_project_root.display()
        ));
    }
    validate_project_name(&journal.project_name)?;
    let dir_name = transaction_root
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            "Public-Artifact-Recovery: Transaktionsordnername ist nicht UTF-8; fail-closed"
                .to_string()
        })?;
    if dir_name != journal.transaction_id {
        return Err(format!(
            "Public-Artifact-Recovery: transaction_id {:?} bindet nicht den Ordner {:?}; fail-closed",
            journal.transaction_id, dir_name
        ));
    }
    if journal.artifacts.len() != ARTIFACT_COUNT
        || journal
            .artifacts
            .iter()
            .enumerate()
            .any(|(index, artifact)| artifact.index != index)
    {
        return Err(
            "Public-Artifact-Recovery: Artefaktvertrag ist nicht kanonisch".to_string(),
        );
    }
    let expected_parent = canonical_directory(
        &canonical_project_root.join(PUBLIC_ARTIFACT_TRANSACTION_ROOT_RELATIVE_PATH),
        "Public-Artifact-Transaktionswurzel",
    )?;
    if transaction_root.parent() != Some(expected_parent.as_path()) {
        return Err(
            "Public-Artifact-Recovery: Transaktion liegt nicht direkt unter der gebundenen Transaktionswurzel; fail-closed"
                .to_string(),
        );
    }
    let backup_root = canonical_directory(&transaction_root.join("backup"), "Public-Artifact-Backupwurzel")?;
    if backup_root.parent() != Some(transaction_root) {
        return Err(
            "Public-Artifact-Recovery: Backupwurzel liegt nicht direkt in der gebundenen Transaktion; fail-closed"
                .to_string(),
        );
    }
    Ok(())
}

fn validate_all_restore_inputs_before_mutation(
    journal: &PublicArtifactJournal,
    transaction_root: &Path,
) -> RecoveryResult<()> {
    for artifact in &journal.artifacts {
        if !artifact.existed_before {
            continue;
        }
        let backup_path = transaction_root
            .join("backup")
            .join(format!("artifact_{:02}.bak", artifact.index));
        reject_symlink_or_non_file(&backup_path, "Public-Artifact-Backup")?;
        let bytes = fs::read(&backup_path).map_err(|error| {
            format!(
                "Public-Artifact-Recovery: Backup nicht lesbar {}: {error}",
                backup_path.display()
            )
        })?;
        verify_bytes_against_record(&bytes, artifact, "Backup")?;
    }
    Ok(())
}

fn restore_public_set(
    journal: &PublicArtifactJournal,
    transaction_root: &Path,
) -> RecoveryResult<()> {
    let targets = public_artifact_paths(&journal.project_root, &journal.project_name);
    for (artifact, target) in journal.artifacts.iter().zip(targets.iter()) {
        if artifact.existed_before {
            let backup_path = transaction_root
                .join("backup")
                .join(format!("artifact_{:02}.bak", artifact.index));
            let bytes = fs::read(&backup_path).map_err(|error| {
                format!(
                    "Public-Artifact-Recovery: Backup nicht lesbar {}: {error}",
                    backup_path.display()
                )
            })?;
            verify_bytes_against_record(&bytes, artifact, "Backup vor Restore")?;
            replace_target_from_verified_bytes(target, &bytes, artifact)?;
        } else {
            remove_regular_file_if_exists(target, "neu erzeugtes Public-Artefakt")?;
        }
    }
    Ok(())
}

fn replace_target_from_verified_bytes(
    target: &Path,
    bytes: &[u8],
    artifact: &ArtifactRecord,
) -> RecoveryResult<()> {
    let temp = target.with_extension(format!("restore-{:02}.tmp", artifact.index));
    remove_regular_file_if_exists(&temp, "Restore-Tempdatei")?;
    write_new_synced_file(&temp, bytes, "Restore-Tempdatei")?;
    let staged = fs::read(&temp).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: Restore-Tempdatei nicht rücklesbar {}: {error}",
            temp.display()
        )
    })?;
    verify_bytes_against_record(&staged, artifact, "Restore-Tempdatei")?;
    remove_regular_file_if_exists(target, "zu ersetzendes Public-Artefakt")?;
    fs::rename(&temp, target).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: Restore-Tempdatei konnte nicht nach {} verschoben werden: {error}",
            target.display()
        )
    })
}

fn verify_restored_public_set(
    journal: &PublicArtifactJournal,
    transaction_root: &Path,
) -> RecoveryResult<()> {
    let targets = public_artifact_paths(&journal.project_root, &journal.project_name);
    for (artifact, target) in journal.artifacts.iter().zip(targets.iter()) {
        if artifact.existed_before {
            reject_symlink_or_non_file(target, "wiederhergestelltes Public-Artefakt")?;
            let bytes = fs::read(target).map_err(|error| {
                format!(
                    "Public-Artifact-Recovery: wiederhergestelltes Artefakt nicht lesbar {}: {error}",
                    target.display()
                )
            })?;
            verify_bytes_against_record(&bytes, artifact, "wiederhergestelltes Artefakt")?;
            let backup = transaction_root
                .join("backup")
                .join(format!("artifact_{:02}.bak", artifact.index));
            reject_symlink_or_non_file(&backup, "Public-Artifact-Backup")?;
        } else {
            match fs::symlink_metadata(target) {
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Ok(_) => {
                    return Err(format!(
                        "Public-Artifact-Recovery: Artefakt {} existiert nach Restore, obwohl es vorher nicht existierte",
                        target.display()
                    ));
                }
                Err(error) => {
                    return Err(format!(
                        "Public-Artifact-Recovery: Abwesenheit von {} nicht prüfbar: {error}",
                        target.display()
                    ));
                }
            }
        }
    }
    Ok(())
}

fn verify_bytes_against_record(
    bytes: &[u8],
    artifact: &ArtifactRecord,
    label: &str,
) -> RecoveryResult<()> {
    if bytes.len() as u64 != artifact.backup_bytes {
        return Err(format!(
            "Public-Artifact-Recovery: {label} für Artefakt {} hat {} statt {} Bytes; fail-closed",
            artifact.index,
            bytes.len(),
            artifact.backup_bytes
        ));
    }
    let actual = sha256::hex(bytes);
    let expected = artifact
        .backup_sha256
        .as_deref()
        .ok_or_else(|| "Public-Artifact-Recovery: erwarteter SHA-256 fehlt".to_string())?;
    if actual != expected {
        return Err(format!(
            "Public-Artifact-Recovery: {label} für Artefakt {} hat SHA-256 {actual} statt {expected}; fail-closed",
            artifact.index
        ));
    }
    Ok(())
}

fn persist_state_journal(
    transaction_root: &Path,
    journal: &PublicArtifactJournal,
) -> RecoveryResult<()> {
    let temp = transaction_root.join(JOURNAL_TEMP_FILE);
    remove_regular_file_if_exists(&temp, "Public-Artifact-Journal-Tempdatei")?;
    let serialized = serialize_journal(journal)?;
    write_new_synced_file(&temp, serialized.as_bytes(), "Public-Artifact-Journal-Tempdatei")?;
    let readback = parse_journal_file(&temp)?;
    if readback != *journal {
        return Err(
            "Public-Artifact-Recovery: Journal-Temp-Readback ist nicht exakt".to_string(),
        );
    }
    let target = transaction_root.join(journal.state.file_name());
    remove_regular_file_if_exists(&target, "altes Public-Artifact-Zustandsjournal")?;
    fs::rename(&temp, &target).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: Zustandsjournal konnte nicht atomar veröffentlicht werden {}: {error}",
            target.display()
        )
    })
}

fn serialize_journal(journal: &PublicArtifactJournal) -> RecoveryResult<String> {
    validate_no_protocol_newline(&journal.transaction_id, "transaction_id")?;
    validate_no_protocol_newline(&journal.producer_version, "producer_version")?;
    let project_root = journal.project_root.to_str().ok_or_else(|| {
        "Public-Artifact-Recovery: kanonische Projektwurzel ist nicht UTF-8; fail-closed"
            .to_string()
    })?;
    let mut out = String::new();
    writeln!(&mut out, "schema_version={}", journal.schema_version)
        .expect("writing to String cannot fail");
    writeln!(&mut out, "transaction_id={}", journal.transaction_id)
        .expect("writing to String cannot fail");
    writeln!(&mut out, "state={}", journal.state.as_str())
        .expect("writing to String cannot fail");
    writeln!(&mut out, "project_root_hex={}", encode_hex(project_root.as_bytes()))
        .expect("writing to String cannot fail");
    writeln!(&mut out, "project_name_hex={}", encode_hex(journal.project_name.as_bytes()))
        .expect("writing to String cannot fail");
    writeln!(&mut out, "producer_version={}", journal.producer_version)
        .expect("writing to String cannot fail");
    for artifact in &journal.artifacts {
        writeln!(
            &mut out,
            "artifact.{}.existed_before={}",
            artifact.index, artifact.existed_before
        )
        .expect("writing to String cannot fail");
        writeln!(
            &mut out,
            "artifact.{}.backup_bytes={}",
            artifact.index, artifact.backup_bytes
        )
        .expect("writing to String cannot fail");
        writeln!(
            &mut out,
            "artifact.{}.backup_sha256={}",
            artifact.index,
            artifact.backup_sha256.as_deref().unwrap_or("-")
        )
        .expect("writing to String cannot fail");
    }
    Ok(out)
}

fn write_marker_atomically(marker_path: &Path, transaction_root: &Path) -> RecoveryResult<()> {
    let temp = marker_path.with_extension("txt.tmp");
    remove_regular_file_if_exists(&temp, "Public-Artifact-Marker-Tempdatei")?;
    let root = transaction_root.to_str().ok_or_else(|| {
        "Public-Artifact-Recovery: Transaktionspfad ist nicht UTF-8; fail-closed".to_string()
    })?;
    validate_no_protocol_newline(root, "transaction_root")?;
    write_new_synced_file(
        &temp,
        format!("{root}\n").as_bytes(),
        "Public-Artifact-Marker-Tempdatei",
    )?;
    fs::rename(&temp, marker_path).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: Pending-Marker konnte nicht atomar veröffentlicht werden {}: {error}",
            marker_path.display()
        )
    })
}

fn write_new_synced_file(path: &Path, bytes: &[u8], label: &str) -> RecoveryResult<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            format!(
                "Public-Artifact-Recovery: {label} nicht erstellbar {}: {error}",
                path.display()
            )
        })?;
    file.write_all(bytes).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: {label} nicht schreibbar {}: {error}",
            path.display()
        )
    })?;
    file.flush().map_err(|error| {
        format!(
            "Public-Artifact-Recovery: {label} nicht flushbar {}: {error}",
            path.display()
        )
    })?;
    file.sync_all().map_err(|error| {
        format!(
            "Public-Artifact-Recovery: {label} nicht synchronisierbar {}: {error}",
            path.display()
        )
    })
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(char::from(HEX[(byte >> 4) as usize]));
        out.push(char::from(HEX[(byte & 0x0f) as usize]));
    }
    out
}

fn decode_hex_utf8(value: &str, label: &str) -> RecoveryResult<String> {
    if !value.len().is_multiple_of(2) || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!(
            "Public-Artifact-Recovery: {label} ist kein gültiges Hexfeld; fail-closed"
        ));
    }
    let mut bytes = Vec::with_capacity(value.len() / 2);
    for pair in value.as_bytes().as_chunks::<2>().0 {
        let high = hex_value(pair[0]).ok_or_else(|| {
            format!("Public-Artifact-Recovery: {label} enthält ungültiges Hex")
        })?;
        let low = hex_value(pair[1]).ok_or_else(|| {
            format!("Public-Artifact-Recovery: {label} enthält ungültiges Hex")
        })?;
        bytes.push((high << 4) | low);
    }
    String::from_utf8(bytes).map_err(|error| {
        format!("Public-Artifact-Recovery: {label} ist nicht UTF-8: {error}")
    })
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn validate_no_protocol_newline(value: &str, label: &str) -> RecoveryResult<()> {
    if value
        .chars()
        .any(|character| matches!(character, '\r' | '\n' | '\0'))
    {
        return Err(format!(
            "Public-Artifact-Recovery: {label} enthält unzulässige Steuerzeichen; fail-closed"
        ));
    }
    Ok(())
}

fn validate_transaction_id(value: &str) -> RecoveryResult<()> {
    if value.is_empty()
        || value.len() > 96
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
    {
        return Err(format!(
            "Public-Artifact-Recovery: transaction_id {value:?} ist nicht sicher; fail-closed"
        ));
    }
    Ok(())
}

fn validate_producer_version(value: &str) -> RecoveryResult<()> {
    if value.is_empty()
        || value.len() > 32
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b'.')
        || value.split('.').count() != 3
        || value.split('.').any(|part| part.is_empty())
    {
        return Err(format!(
            "Public-Artifact-Recovery: producer_version {value:?} ist nicht exakt dreiteilig numerisch; fail-closed"
        ));
    }
    Ok(())
}

fn parse_bool(value: &str) -> RecoveryResult<bool> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!(
            "Public-Artifact-Recovery: ungültiger boolescher Wert {other:?}; fail-closed"
        )),
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn canonical_directory(path: &Path, label: &str) -> RecoveryResult<PathBuf> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: {label} nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(format!(
            "Public-Artifact-Recovery: {label} ist kein echter Ordner {}",
            path.display()
        ));
    }
    fs::canonicalize(path).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: {label} nicht kanonisierbar {}: {error}",
            path.display()
        )
    })
}

fn reject_symlink_or_non_file(path: &Path, label: &str) -> RecoveryResult<()> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: {label} nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(format!(
            "Public-Artifact-Recovery: {label} ist keine echte reguläre Datei {}; fail-closed",
            path.display()
        ));
    }
    Ok(())
}

fn remove_regular_file_if_exists(path: &Path, label: &str) -> RecoveryResult<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                return Err(format!(
                    "Public-Artifact-Recovery: {label} ist keine reguläre Datei {}; Entfernen verweigert",
                    path.display()
                ));
            }
            fs::remove_file(path).map_err(|error| {
                format!(
                    "Public-Artifact-Recovery: {label} konnte nicht entfernt werden {}: {error}",
                    path.display()
                )
            })
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "Public-Artifact-Recovery: {label} nicht prüfbar {}: {error}",
            path.display()
        )),
    }
}

fn read_small_utf8_file(path: &Path, max_bytes: u64, label: &str) -> RecoveryResult<String> {
    let metadata = fs::metadata(path).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: {label} Metadaten nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    if metadata.len() > max_bytes {
        return Err(format!(
            "Public-Artifact-Recovery: {label} ist mit {} Bytes größer als {max_bytes}; fail-closed",
            metadata.len()
        ));
    }
    let mut file = File::open(path).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: {label} nicht öffnbar {}: {error}",
            path.display()
        )
    })?;
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    file.read_to_end(&mut bytes).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: {label} nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    String::from_utf8(bytes).map_err(|error| {
        format!(
            "Public-Artifact-Recovery: {label} ist nicht UTF-8 {}: {error}",
            path.display()
        )
    })
}

fn single_protocol_line<'a>(raw: &'a str, label: &str) -> RecoveryResult<&'a str> {
    let without_newline = raw
        .strip_suffix("\r\n")
        .or_else(|| raw.strip_suffix('\n'))
        .unwrap_or(raw);
    if without_newline
        .chars()
        .any(|character| matches!(character, '\r' | '\n' | '\0'))
    {
        return Err(format!(
            "Public-Artifact-Recovery: {label} muss exakt eine Protokollzeile enthalten; fail-closed"
        ));
    }
    Ok(without_newline)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_names_are_derived_only_from_project_stem() {
        let root = Path::new("C:/project");
        let paths = public_artifact_paths(root, "Synthetic Score");
        assert_eq!(paths[0], root.join("Synthetic Score.musicxml"));
        assert_eq!(paths[1], root.join("Synthetic Score.render.musicxml"));
        assert_eq!(paths[2], root.join("Synthetic Score.pdf"));
        assert_eq!(paths[3], root.join("Synthetic Score.result.txt"));
    }

    #[test]
    fn project_name_rejects_path_injection() {
        assert!(validate_project_name("Synthetic Score").is_ok());
        assert!(validate_project_name("../Synthetic Score").is_err());
        assert!(validate_project_name("Synthetic/Score").is_err());
        assert!(validate_project_name("Synthetic\\Score").is_err());
    }

    #[test]
    fn formally_green_summary_accepts_nonblocking_hints() {
        assert!(formal_green_contract::report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 0 | Warnungen: 0 | Hinweise: 0\n"
        ));
        assert!(formal_green_contract::report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 0 | Warnungen: 0 | Hinweise: 1\n"
        ));
        assert!(formal_green_contract::report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 0 | Warnungen: 0 | Hinweise: 18446744073709551615\n"
        ));
    }

    #[test]
    fn formally_green_summary_rejects_warnings_blockers_and_malformed_hints() {
        assert!(!formal_green_contract::report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 0 | Warnungen: 1 | Hinweise: 0\n"
        ));
        assert!(!formal_green_contract::report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 1 | Warnungen: 0 | Hinweise: 0\n"
        ));
        assert!(!formal_green_contract::report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 0 | Warnungen: 0 | Hinweise: -1\n"
        ));
        assert!(!formal_green_contract::report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 0 | Warnungen: 0 | Hinweise: eins\n"
        ));
    }

    #[test]
    fn sha256_matches_known_vector() {
        assert_eq!(
            sha256::hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
