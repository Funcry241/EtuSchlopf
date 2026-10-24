use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::sha256;

const LEDGER_RELATIVE_ROOT: &str = "target/etuschlopf_take_rejection_ledger_v1";
const SNAPSHOT_FILE: &str = "semantic_effect_snapshot.tsv";
const METADATA_FILE: &str = "metadata.txt";
const REASON_FILE: &str = "reason.txt";
const CANDIDATE_INTENT_FILE: &str = "candidate_intent.txt";
const ACCEPTANCE_EVIDENCE_RELATIVE_ROOT: &str = "target/etuschlopf_take_acceptance_evidence_v1";

pub const REJECTION_STAGE_DEAD_CODE_INTEGRITY: &str = "dead_code_integrity";
pub const REJECTION_STAGE_DEPENDENCY_PREFLIGHT: &str = "dependency_preflight";
pub const REJECTION_STAGE_EFFECT_SNAPSHOT: &str = "effect_snapshot";
pub const REJECTION_STAGE_INCOMING_TAKE: &str = "incoming_take";
pub const REJECTION_STAGE_WATCH_PREFLIGHT: &str = "watch_preflight";
pub const REJECTION_STAGE_APPLY_VERIFY: &str = "apply_verify";
pub const REJECTION_STAGE_CARGO_LOCK_RESOLUTION: &str = "cargo_lock_resolution";
pub const REJECTION_STAGE_REEXEC_LAUNCH: &str = "reexec_launch";
pub const REJECTION_STAGE_REEXEC_EXIT: &str = "reexec_exit";
pub const REJECTION_STAGE_CRASH_RECOVERY: &str = "crash_recovery";

#[derive(Debug, Clone)]
pub struct RejectedRevision {
    pub take_revision: Option<u64>,
    pub zip_sha256: String,
    pub rejection_stage: String,
    pub snapshot: String,
    pub reason: String,
    pub attempt_epoch_millis: u128,
}

#[derive(Debug, Clone, Default)]
pub struct RevisionChainSummary {
    pub prior_rejections: usize,
    pub latest_rejected_sha256: String,
}

#[derive(Debug, Clone, Copy)]
pub struct RejectionRecord<'a> {
    pub from_version: &'a str,
    pub to_version: &'a str,
    pub take_revision: u64,
    pub zip_sha256: &'a str,
    pub rejection_stage: &'a str,
    pub candidate_intent: Option<&'a [u8]>,
    pub snapshot: &'a str,
    pub reason: &'a str,
}

pub fn sha256_file(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path)
        .map_err(|error| format!("MultiRevision-Evasion: ZIP nicht lesbar {}: {error}", path.display()))?;
    Ok(sha256::hex(&bytes))
}

pub fn load_rejected_revisions(
    project_root: &Path,
    from_version: &str,
    requested_to_version: &str,
) -> Result<Vec<RejectedRevision>, String> {
    let safe_from = safe_component(from_version)?;
    safe_component(requested_to_version)?;
    let ledger_root = project_root.join(LEDGER_RELATIVE_ROOT);
    if !ledger_root.exists() {
        return Ok(Vec::new());
    }
    let pair_prefix = format!("{safe_from}__");
    let mut out = Vec::new();
    for pair_entry in fs::read_dir(&ledger_root).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Rejection-Ledger nicht lesbar {}: {error}",
            ledger_root.display()
        )
    })? {
        let pair_entry = pair_entry.map_err(|error| {
            format!("MultiRevision-Evasion: Ledger-Paar nicht lesbar: {error}")
        })?;
        let pair_type = pair_entry.file_type().map_err(|error| {
            format!(
                "MultiRevision-Evasion: Ledger-Paartyp nicht lesbar {}: {error}",
                pair_entry.path().display()
            )
        })?;
        let pair_name = pair_entry.file_name().to_string_lossy().to_string();
        if !pair_type.is_dir() || !pair_name.starts_with(&pair_prefix) {
            continue;
        }
        for entry in fs::read_dir(pair_entry.path()).map_err(|error| {
            format!(
                "MultiRevision-Evasion: Rejection-Paar nicht lesbar {}: {error}",
                pair_entry.path().display()
            )
        })? {
            let entry = entry.map_err(|error| {
                format!("MultiRevision-Evasion: Ledger-Eintrag nicht lesbar: {error}")
            })?;
            let file_type = entry.file_type().map_err(|error| {
                format!(
                    "MultiRevision-Evasion: Ledger-Dateityp nicht lesbar {}: {error}",
                    entry.path().display()
                )
            })?;
            if !file_type.is_dir() {
                continue;
            }
            if entry.file_name().to_string_lossy().starts_with(".pending-") {
                return Err(format!(
                    "MultiRevision-Evasion: unvollstaendiger pending Ledger-Eintrag {} gefunden; fail-closed",
                    entry.path().display()
                ));
            }
            let metadata = fs::read_to_string(entry.path().join(METADATA_FILE)).map_err(|error| {
                format!(
                    "MultiRevision-Evasion: Metadaten nicht lesbar {}: {error}",
                    entry.path().display()
                )
            })?;
            if !metadata_value(&metadata, "from_version")?.eq_ignore_ascii_case(from_version) {
                return Err(format!(
                    "MultiRevision-Evasion: Ledger-Paar {} enthaelt fremde Ausgangsversion; fail-closed",
                    pair_entry.path().display()
                ));
            }
            let take_revision = metadata_optional_value(&metadata, "take_revision")
                .map(|value| {
                    value.parse::<u64>().map_err(|error| {
                        format!(
                            "MultiRevision-Evasion: ungueltige Take-Revision in {}: {error}",
                            entry.path().display()
                        )
                    })
                })
                .transpose()?;
            let zip_sha256 = metadata_value(&metadata, "zip_sha256")?;
            let rejection_stage = metadata_optional_value(&metadata, "rejection_stage")
                .unwrap_or_else(|| "legacy_unknown".to_string());
            if rejection_stage != "legacy_unknown" {
                validate_rejection_stage(&rejection_stage)?;
            }
            let attempt_epoch_millis = metadata_value(&metadata, "attempt_epoch_millis")?
                .parse::<u128>()
                .map_err(|error| {
                    format!(
                        "MultiRevision-Evasion: ungueltiger Zeitwert in {}: {error}",
                        entry.path().display()
                    )
                })?;
            let snapshot = fs::read_to_string(entry.path().join(SNAPSHOT_FILE)).map_err(|error| {
                format!(
                    "MultiRevision-Evasion: Snapshot nicht lesbar {}: {error}",
                    entry.path().display()
                )
            })?;
            let reason = fs::read_to_string(entry.path().join(REASON_FILE)).map_err(|error| {
                format!(
                    "MultiRevision-Evasion: Ablehnungsgrund nicht lesbar {}: {error}",
                    entry.path().display()
                )
            })?;
            out.push(RejectedRevision {
                take_revision,
                zip_sha256,
                rejection_stage,
                snapshot,
                reason,
                attempt_epoch_millis,
            });
        }
    }
    out.sort_by(|left, right| {
        left.attempt_epoch_millis
            .cmp(&right.attempt_epoch_millis)
            .then_with(|| left.zip_sha256.cmp(&right.zip_sha256))
    });
    Ok(out)
}

pub fn load_accepted_zip_sha256(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    take_revision: u64,
) -> Result<BTreeSet<String>, String> {
    let pair_root = pair_root(project_root, from_version, to_version)?;
    if !pair_root.exists() {
        return Ok(BTreeSet::new());
    }
    let mut accepted = BTreeSet::new();
    for entry in fs::read_dir(&pair_root).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Acceptance-Paar nicht lesbar {}: {error}",
            pair_root.display()
        )
    })? {
        let entry = entry.map_err(|error| {
            format!("MultiRevision-Evasion: Acceptance-Eintrag nicht lesbar: {error}")
        })?;
        if !entry
            .file_type()
            .map_err(|error| {
                format!(
                    "MultiRevision-Evasion: Acceptance-Dateityp nicht lesbar {}: {error}",
                    entry.path().display()
                )
            })?
            .is_file()
        {
            continue;
        }
        let file_name = entry.file_name().to_string_lossy().to_string();
        if !file_name.starts_with("accepted-") || !file_name.ends_with(".txt") {
            continue;
        }
        let metadata = fs::read_to_string(entry.path()).map_err(|error| {
            format!(
                "MultiRevision-Evasion: Acceptance-Provenienz nicht lesbar {}: {error}",
                entry.path().display()
            )
        })?;
        if !metadata_value(&metadata, "from_version")?.eq_ignore_ascii_case(from_version)
            || !metadata_value(&metadata, "to_version")?.eq_ignore_ascii_case(to_version)
        {
            return Err(format!(
                "MultiRevision-Evasion: Acceptance-Paar {} enthaelt fremde Versionsidentitaet; fail-closed",
                pair_root.display()
            ));
        }
        let Some(recorded_revision) = metadata_optional_value(&metadata, "take_revision") else {
            // Legacy acceptance provenance predates revision-bound Take identity. It remains
            // historical evidence, but must not canonize a current (Version, Revision) peer.
            continue;
        };
        let recorded_revision = recorded_revision.parse::<u64>().map_err(|error| {
            format!(
                "MultiRevision-Evasion: ungueltige Take-Revision in {}: {error}",
                entry.path().display()
            )
        })?;
        if recorded_revision != take_revision {
            continue;
        }
        let sha256 = metadata_value(&metadata, "accepted_zip_sha256")?;
        validate_sha256(&sha256)?;
        accepted.insert(sha256.to_ascii_lowercase());
    }
    Ok(accepted)
}

pub fn summarize_chain(rejections: &[RejectedRevision]) -> RevisionChainSummary {
    RevisionChainSummary {
        prior_rejections: rejections.len(),
        latest_rejected_sha256: rejections
            .last()
            .map(|entry| entry.zip_sha256.clone())
            .unwrap_or_default(),
    }
}

pub fn active_diagnostic_rejections(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    rejections: &[RejectedRevision],
) -> Result<Vec<RejectedRevision>, String> {
    let Some(prefix_len) =
        latest_acceptance_rejection_prefix_len(project_root, from_version, to_version)?
    else {
        return Ok(rejections.to_vec());
    };
    if prefix_len > rejections.len() {
        return Err(format!(
            "MultiRevision-Evasion: Acceptance-Provenienz bindet {prefix_len} fruehere Rejections, Ledger enthaelt aber nur {}; fail-closed",
            rejections.len()
        ));
    }
    Ok(rejections.iter().skip(prefix_len).cloned().collect())
}

fn latest_acceptance_rejection_prefix_len(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
) -> Result<Option<usize>, String> {
    let pair_root = pair_root(project_root, from_version, to_version)?;
    if !pair_root.exists() {
        return Ok(None);
    }
    let mut latest: Option<(u64, usize)> = None;
    for entry in fs::read_dir(&pair_root).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Acceptance-Paar nicht lesbar {}: {error}",
            pair_root.display()
        )
    })? {
        let entry = entry.map_err(|error| {
            format!("MultiRevision-Evasion: Acceptance-Eintrag nicht lesbar: {error}")
        })?;
        if !entry
            .file_type()
            .map_err(|error| {
                format!(
                    "MultiRevision-Evasion: Acceptance-Dateityp nicht lesbar {}: {error}",
                    entry.path().display()
                )
            })?
            .is_file()
        {
            continue;
        }
        let file_name = entry.file_name().to_string_lossy().to_string();
        if !file_name.starts_with("accepted-") || !file_name.ends_with(".txt") {
            continue;
        }
        let metadata = fs::read_to_string(entry.path()).map_err(|error| {
            format!(
                "MultiRevision-Evasion: Acceptance-Provenienz nicht lesbar {}: {error}",
                entry.path().display()
            )
        })?;
        if !metadata_value(&metadata, "from_version")?.eq_ignore_ascii_case(from_version)
            || !metadata_value(&metadata, "to_version")?.eq_ignore_ascii_case(to_version)
        {
            return Err(format!(
                "MultiRevision-Evasion: Acceptance-Paar {} enthaelt fremde Versionsidentitaet; fail-closed",
                pair_root.display()
            ));
        }
        let Some(revision_text) = metadata_optional_value(&metadata, "take_revision") else {
            continue;
        };
        let Some(prior_rejections_text) =
            metadata_optional_value(&metadata, "prior_rejections")
        else {
            continue;
        };
        let revision = revision_text.parse::<u64>().map_err(|error| {
            format!(
                "MultiRevision-Evasion: ungueltige Take-Revision in {}: {error}",
                entry.path().display()
            )
        })?;
        let prior_rejections = prior_rejections_text.parse::<usize>().map_err(|error| {
            format!(
                "MultiRevision-Evasion: ungueltige Rejection-Anzahl in {}: {error}",
                entry.path().display()
            )
        })?;
        match latest {
            Some((latest_revision, _)) if revision < latest_revision => {}
            Some((latest_revision, latest_prefix)) if revision == latest_revision => {
                if prior_rejections != latest_prefix {
                    return Err(format!(
                        "MultiRevision-Evasion: widerspruechliche Acceptance-Provenienz fuer Revision r{revision}: Rejection-Prefix {latest_prefix} vs. {prior_rejections}; fail-closed"
                    ));
                }
            }
            _ => latest = Some((revision, prior_rejections)),
        }
    }
    Ok(latest.map(|(_, prefix)| prefix))
}

pub fn reject_exact_replay(
    rejections: &[RejectedRevision],
    candidate_sha256: &str,
) -> Result<(), String> {
    if let Some(previous) = rejections
        .iter()
        .find(|entry| entry.zip_sha256.eq_ignore_ascii_case(candidate_sha256))
    {
        return Err(format!(
            "MultiRevision-Semantic-Capability-Evasion: bereits abgewiesener identischer Kandidat erneut angeboten; SHA-256 {}; frueherer Grund: {}",
            candidate_sha256,
            compact_reason(&previous.reason)
        ));
    }
    Ok(())
}

pub fn record_rejection(
    project_root: &Path,
    rejection: &RejectionRecord<'_>,
) -> Result<(), String> {
    let RejectionRecord {
        from_version,
        to_version,
        take_revision,
        zip_sha256,
        rejection_stage,
        candidate_intent,
        snapshot,
        reason,
    } = *rejection;
    validate_sha256(zip_sha256)?;
    validate_rejection_stage(rejection_stage)?;
    let pair_root = pair_root(project_root, from_version, to_version)?;
    fs::create_dir_all(&pair_root).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Ledger-Ordner nicht anlegbar {}: {error}",
            pair_root.display()
        )
    })?;
    let final_dir = pair_root.join(zip_sha256);
    if final_dir.exists() {
        return Ok(());
    }
    let epoch = now_epoch_millis()?;
    let temp_dir = pair_root.join(format!(".pending-{zip_sha256}-{epoch}"));
    fs::create_dir(&temp_dir).map_err(|error| {
        format!(
            "MultiRevision-Evasion: temporaerer Ledger-Eintrag nicht anlegbar {}: {error}",
            temp_dir.display()
        )
    })?;
    let metadata = format!(
        "schema=2\nfrom_version={}\nto_version={}\ntake_revision={}\nzip_sha256={}\nrejection_stage={}\nverdict=rejected\nattempt_epoch_millis={}\n",
        ledger_one_line(from_version),
        ledger_one_line(to_version),
        take_revision,
        zip_sha256,
        rejection_stage,
        epoch
    );
    let write_result = (|| -> Result<(), String> {
        fs::write(temp_dir.join(METADATA_FILE), metadata).map_err(|error| {
            format!("MultiRevision-Evasion: Metadaten nicht schreibbar: {error}")
        })?;
        fs::write(temp_dir.join(SNAPSHOT_FILE), snapshot).map_err(|error| {
            format!("MultiRevision-Evasion: Snapshot nicht schreibbar: {error}")
        })?;
        fs::write(temp_dir.join(REASON_FILE), reason).map_err(|error| {
            format!("MultiRevision-Evasion: Ablehnungsgrund nicht schreibbar: {error}")
        })?;
        if let Some(intent) = candidate_intent {
            fs::write(temp_dir.join(CANDIDATE_INTENT_FILE), intent).map_err(|error| {
                format!("MultiRevision-Evasion: Candidate-Intent nicht schreibbar: {error}")
            })?;
        }
        fs::rename(&temp_dir, &final_dir).map_err(|error| {
            format!(
                "MultiRevision-Evasion: Ledger-Commit fehlgeschlagen {} -> {}: {error}",
                temp_dir.display(),
                final_dir.display()
            )
        })?;
        Ok(())
    })();
    if write_result.is_err() {
        let _ = fs::remove_dir_all(&temp_dir);
    }
    write_result
}

#[derive(Debug, Clone)]
pub struct RejectionEvidenceReadback {
    pub dir: PathBuf,
    pub metadata: Vec<u8>,
    pub candidate_intent: Option<Vec<u8>>,
    pub snapshot: Vec<u8>,
    pub reason: Vec<u8>,
    pub rejection_stage: String,
    pub attempt_epoch_millis: u128,
}

pub fn read_rejection_record_exact(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    take_revision: u64,
    zip_sha256: &str,
) -> Result<RejectionEvidenceReadback, String> {
    validate_sha256(zip_sha256)?;
    let dir = pair_root(project_root, from_version, to_version)?.join(zip_sha256);
    let metadata = fs::read(dir.join(METADATA_FILE)).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Rejection-Metadaten nicht lesbar {}: {error}",
            dir.display()
        )
    })?;
    let metadata_text = std::str::from_utf8(&metadata).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Rejection-Metadaten sind nicht UTF-8 {}: {error}",
            dir.display()
        )
    })?;
    if metadata_value(metadata_text, "from_version")? != from_version
        || metadata_value(metadata_text, "to_version")? != to_version
        || metadata_value(metadata_text, "take_revision")? != take_revision.to_string()
        || !metadata_value(metadata_text, "zip_sha256")?.eq_ignore_ascii_case(zip_sha256)
        || metadata_value(metadata_text, "verdict")? != "rejected"
    {
        return Err(format!(
            "MultiRevision-Evasion: Rejection-Provenienz-Konflikt in {}; fail-closed",
            dir.display()
        ));
    }
    let rejection_stage = metadata_value(metadata_text, "rejection_stage")?;
    validate_rejection_stage(&rejection_stage)?;
    let attempt_epoch_millis = metadata_value(metadata_text, "attempt_epoch_millis")?
        .parse::<u128>()
        .map_err(|error| {
            format!(
                "MultiRevision-Evasion: ungueltiger Rejection-Zeitwert in {}: {error}",
                dir.display()
            )
        })?;
    let snapshot = fs::read(dir.join(SNAPSHOT_FILE)).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Rejection-Snapshot nicht lesbar {}: {error}",
            dir.display()
        )
    })?;
    let reason = fs::read(dir.join(REASON_FILE)).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Rejection-Grund nicht lesbar {}: {error}",
            dir.display()
        )
    })?;
    let intent_path = dir.join(CANDIDATE_INTENT_FILE);
    let candidate_intent = match fs::read(&intent_path) {
        Ok(bytes) => Some(bytes),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => {
            return Err(format!(
                "MultiRevision-Evasion: Rejection-Candidate-Intent nicht lesbar {}: {error}",
                intent_path.display()
            ));
        }
    };
    Ok(RejectionEvidenceReadback {
        dir,
        metadata,
        candidate_intent,
        snapshot,
        reason,
        rejection_stage,
        attempt_epoch_millis,
    })
}

#[derive(Debug, Clone)]
pub struct CurrentAcceptanceReadback {
    pub path: PathBuf,
    pub bytes: Vec<u8>,
    pub from_version: String,
    pub to_version: String,
    pub take_revision: u64,
    pub zip_sha256: String,
    pub prior_rejections: usize,
}

pub fn read_unique_current_acceptance_record_exact(
    project_root: &Path,
    current_version: &str,
    take_revision: u64,
) -> Result<CurrentAcceptanceReadback, String> {
    safe_component(current_version)?;
    let ledger_root = project_root.join(LEDGER_RELATIVE_ROOT);
    if !ledger_root.is_dir() {
        return Err(format!(
            "MultiRevision-Evasion: Parent-Acceptance-Ledger fehlt {}; fail-closed",
            ledger_root.display()
        ));
    }
    let mut matches = Vec::new();
    for pair_entry in fs::read_dir(&ledger_root).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Parent-Acceptance-Ledger nicht lesbar {}: {error}",
            ledger_root.display()
        )
    })? {
        let pair_entry = pair_entry
            .map_err(|error| format!("MultiRevision-Evasion: Ledger-Paar nicht lesbar: {error}"))?;
        if !pair_entry
            .file_type()
            .map_err(|error| {
                format!(
                    "MultiRevision-Evasion: Ledger-Paartyp nicht lesbar {}: {error}",
                    pair_entry.path().display()
                )
            })?
            .is_dir()
        {
            continue;
        }
        for entry in fs::read_dir(pair_entry.path()).map_err(|error| {
            format!(
                "MultiRevision-Evasion: Parent-Acceptance-Paar nicht lesbar {}: {error}",
                pair_entry.path().display()
            )
        })? {
            let entry = entry.map_err(|error| {
                format!("MultiRevision-Evasion: Parent-Acceptance-Eintrag nicht lesbar: {error}")
            })?;
            if !entry
                .file_type()
                .map_err(|error| {
                    format!(
                        "MultiRevision-Evasion: Parent-Acceptance-Dateityp nicht lesbar {}: {error}",
                        entry.path().display()
                    )
                })?
                .is_file()
            {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if !name.starts_with("accepted-") || !name.ends_with(".txt") {
                continue;
            }
            let bytes = fs::read(entry.path()).map_err(|error| {
                format!(
                    "MultiRevision-Evasion: Parent-Acceptance nicht lesbar {}: {error}",
                    entry.path().display()
                )
            })?;
            let text = std::str::from_utf8(&bytes).map_err(|error| {
                format!(
                    "MultiRevision-Evasion: Parent-Acceptance ist nicht UTF-8 {}: {error}",
                    entry.path().display()
                )
            })?;
            let recorded_to = metadata_value(text, "to_version")?;
            let Some(recorded_revision) = metadata_optional_value(text, "take_revision") else {
                continue;
            };
            let recorded_revision = recorded_revision.parse::<u64>().map_err(|error| {
                format!(
                    "MultiRevision-Evasion: ungueltige Parent-Take-Revision in {}: {error}",
                    entry.path().display()
                )
            })?;
            if !recorded_to.eq_ignore_ascii_case(current_version)
                || recorded_revision != take_revision
            {
                continue;
            }
            let from_version = metadata_value(text, "from_version")?;
            let zip_sha256 = metadata_value(text, "accepted_zip_sha256")?;
            let prior_rejections = metadata_value(text, "prior_rejections")?
                .parse::<usize>()
                .map_err(|error| {
                    format!(
                        "MultiRevision-Evasion: ungueltiger Parent-Rejection-Prefix in {}: {error}",
                        entry.path().display()
                    )
                })?;
            let (path, exact_bytes) = read_acceptance_record_exact(
                project_root,
                &from_version,
                current_version,
                take_revision,
                &zip_sha256,
                prior_rejections,
            )?;
            matches.push(CurrentAcceptanceReadback {
                path,
                bytes: exact_bytes,
                from_version,
                to_version: current_version.to_string(),
                take_revision,
                zip_sha256,
                prior_rejections,
            });
        }
    }
    if matches.len() != 1 {
        return Err(format!(
            "MultiRevision-Evasion: fuer aktuellen Parent {} r{} wurden {} exakte Acceptance-Records gefunden; genau einer ist erforderlich; fail-closed",
            current_version,
            take_revision,
            matches.len()
        ));
    }
    Ok(matches.pop().expect("exactly one current acceptance"))
}

pub fn acceptance_record_path(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    take_revision: u64,
    zip_sha256: &str,
) -> Result<PathBuf, String> {
    validate_sha256(zip_sha256)?;
    Ok(pair_root(project_root, from_version, to_version)?
        .join(format!("accepted-r{take_revision:02}-{zip_sha256}.txt")))
}

pub fn read_acceptance_record_exact(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    take_revision: u64,
    zip_sha256: &str,
    prior_rejections: usize,
) -> Result<(PathBuf, Vec<u8>), String> {
    let path = acceptance_record_path(
        project_root,
        from_version,
        to_version,
        take_revision,
        zip_sha256,
    )?;
    let bytes = fs::read(&path).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Acceptance-Provenienz nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    let text = std::str::from_utf8(&bytes).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Acceptance-Provenienz ist nicht UTF-8 {}: {error}",
            path.display()
        )
    })?;
    if metadata_value(text, "from_version")? != from_version
        || metadata_value(text, "to_version")? != to_version
        || metadata_value(text, "take_revision")? != take_revision.to_string()
        || !metadata_value(text, "accepted_zip_sha256")?.eq_ignore_ascii_case(zip_sha256)
        || metadata_value(text, "prior_rejections")? != prior_rejections.to_string()
    {
        return Err(format!(
            "MultiRevision-Evasion: Acceptance-Provenienz-Konflikt in {}; fail-closed",
            path.display()
        ));
    }
    metadata_value(text, "accepted_epoch_millis")?
        .parse::<u128>()
        .map_err(|error| {
            format!(
                "MultiRevision-Evasion: ungueltiger Acceptance-Zeitwert in {}: {error}",
                path.display()
            )
        })?;
    Ok((path, bytes))
}

pub fn record_acceptance(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    take_revision: u64,
    zip_sha256: &str,
    prior_rejections: usize,
) -> Result<(), String> {
    validate_sha256(zip_sha256)?;
    let pair_root = pair_root(project_root, from_version, to_version)?;
    fs::create_dir_all(&pair_root).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Ledger-Ordner nicht anlegbar {}: {error}",
            pair_root.display()
        )
    })?;
    let path = acceptance_record_path(
        project_root,
        from_version,
        to_version,
        take_revision,
        zip_sha256,
    )?;
    if path.exists() {
        read_acceptance_record_exact(
            project_root,
            from_version,
            to_version,
            take_revision,
            zip_sha256,
            prior_rejections,
        )?;
        return Ok(());
    }
    let text = format!(
        "schema=2\nfrom_version={}\nto_version={}\ntake_revision={}\naccepted_zip_sha256={}\nprior_rejections={}\naccepted_epoch_millis={}\n",
        ledger_one_line(from_version),
        ledger_one_line(to_version),
        take_revision,
        zip_sha256,
        prior_rejections,
        now_epoch_millis()?
    );
    fs::write(&path, text).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Acceptance-Provenienz nicht schreibbar: {error}"
        )
    })
}

pub type AcceptanceEvidenceReadback = (PathBuf, Vec<u8>, Vec<u8>, Vec<u8>);

pub fn read_acceptance_evidence_exact(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    take_revision: u64,
    zip_sha256: &str,
) -> Result<AcceptanceEvidenceReadback, String> {
    validate_sha256(zip_sha256)?;
    let safe_from = safe_component(from_version)?;
    let safe_to = safe_component(to_version)?;
    let dir = project_root
        .join(ACCEPTANCE_EVIDENCE_RELATIVE_ROOT)
        .join(format!("{safe_from}__{safe_to}"))
        .join(format!("accepted-evidence-r{take_revision:02}-{zip_sha256}"));
    let metadata = fs::read(dir.join(METADATA_FILE)).map_err(|error| {
        format!(
            "MultiRevision-Evidence: Acceptance-Metadaten nicht lesbar {}: {error}",
            dir.display()
        )
    })?;
    let metadata_text = std::str::from_utf8(&metadata).map_err(|error| {
        format!(
            "MultiRevision-Evidence: Acceptance-Metadaten sind nicht UTF-8 {}: {error}",
            dir.display()
        )
    })?;
    if metadata_value(metadata_text, "from_version")? != from_version
        || metadata_value(metadata_text, "to_version")? != to_version
        || metadata_value(metadata_text, "take_revision")? != take_revision.to_string()
        || !metadata_value(metadata_text, "accepted_zip_sha256")?
            .eq_ignore_ascii_case(zip_sha256)
        || metadata_value(metadata_text, "verdict")? != "accepted"
        || metadata_value(metadata_text, "authority")? != "advisory_evidence_only"
    {
        return Err(format!(
            "MultiRevision-Evidence: Acceptance-Evidence-Konflikt in {}; fail-closed",
            dir.display()
        ));
    }
    let intent = fs::read(dir.join(CANDIDATE_INTENT_FILE)).map_err(|error| {
        format!(
            "MultiRevision-Evidence: Candidate-Intent nicht lesbar {}: {error}",
            dir.display()
        )
    })?;
    let snapshot = fs::read(dir.join(SNAPSHOT_FILE)).map_err(|error| {
        format!(
            "MultiRevision-Evidence: Semantic-Effect-Snapshot nicht lesbar {}: {error}",
            dir.display()
        )
    })?;
    Ok((dir, metadata, intent, snapshot))
}

pub fn previous_acceptance_rejection_prefix_len(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    before_revision: u64,
) -> Result<usize, String> {
    let pair_root = pair_root(project_root, from_version, to_version)?;
    if !pair_root.exists() {
        return Ok(0);
    }
    let mut previous: Option<(u64, usize)> = None;
    for entry in fs::read_dir(&pair_root).map_err(|error| {
        format!(
            "MultiRevision-Evasion: Acceptance-Paar nicht lesbar {}: {error}",
            pair_root.display()
        )
    })? {
        let entry = entry.map_err(|error| {
            format!("MultiRevision-Evasion: Acceptance-Eintrag nicht lesbar: {error}")
        })?;
        if !entry
            .file_type()
            .map_err(|error| {
                format!(
                    "MultiRevision-Evasion: Acceptance-Dateityp nicht lesbar {}: {error}",
                    entry.path().display()
                )
            })?
            .is_file()
        {
            continue;
        }
        let file_name = entry.file_name().to_string_lossy().to_string();
        if !file_name.starts_with("accepted-") || !file_name.ends_with(".txt") {
            continue;
        }
        let metadata = fs::read_to_string(entry.path()).map_err(|error| {
            format!(
                "MultiRevision-Evasion: Acceptance-Provenienz nicht lesbar {}: {error}",
                entry.path().display()
            )
        })?;
        if metadata_value(&metadata, "from_version")? != from_version
            || metadata_value(&metadata, "to_version")? != to_version
        {
            return Err(format!(
                "MultiRevision-Evasion: Acceptance-Paar {} enthaelt fremde Versionsidentitaet; fail-closed",
                pair_root.display()
            ));
        }
        let Some(revision) = metadata_optional_value(&metadata, "take_revision") else {
            continue;
        };
        let revision = revision.parse::<u64>().map_err(|error| {
            format!(
                "MultiRevision-Evasion: ungueltige Take-Revision in {}: {error}",
                entry.path().display()
            )
        })?;
        if revision >= before_revision {
            continue;
        }
        let Some(prefix) = metadata_optional_value(&metadata, "prior_rejections") else {
            continue;
        };
        let prefix = prefix.parse::<usize>().map_err(|error| {
            format!(
                "MultiRevision-Evasion: ungueltige Rejection-Anzahl in {}: {error}",
                entry.path().display()
            )
        })?;
        match previous {
            Some((previous_revision, _)) if revision < previous_revision => {}
            Some((previous_revision, previous_prefix)) if revision == previous_revision => {
                if prefix != previous_prefix {
                    return Err(format!(
                        "MultiRevision-Evasion: widerspruechliche Acceptance-Provenienz fuer Revision r{revision}; fail-closed"
                    ));
                }
            }
            _ => previous = Some((revision, prefix)),
        }
    }
    Ok(previous.map(|(_, prefix)| prefix).unwrap_or(0))
}

pub fn record_acceptance_evidence(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    take_revision: u64,
    zip_sha256: &str,
    candidate_intent: Option<&[u8]>,
    snapshot: &str,
) -> Result<(), String> {
    validate_sha256(zip_sha256)?;
    let intent = candidate_intent.ok_or_else(|| {
        "MultiRevision-Evidence: Candidate-Intent fehlt; advisory Acceptance-Evidence wird nicht erzeugt"
            .to_string()
    })?;
    let safe_from = safe_component(from_version)?;
    let safe_to = safe_component(to_version)?;
    let pair_root = project_root
        .join(ACCEPTANCE_EVIDENCE_RELATIVE_ROOT)
        .join(format!("{safe_from}__{safe_to}"));
    fs::create_dir_all(&pair_root).map_err(|error| {
        format!(
            "MultiRevision-Evidence: Acceptance-Evidence-Paar nicht anlegbar {}: {error}",
            pair_root.display()
        )
    })?;
    let dir_name = format!("accepted-evidence-r{take_revision:02}-{zip_sha256}");
    let final_dir = pair_root.join(&dir_name);
    let metadata = format!(
        "schema=1\nfrom_version={}\nto_version={}\ntake_revision={}\naccepted_zip_sha256={}\nverdict=accepted\nauthority=advisory_evidence_only\n",
        ledger_one_line(from_version),
        ledger_one_line(to_version),
        take_revision,
        zip_sha256
    );
    let expected = [
        (METADATA_FILE, metadata.as_bytes()),
        (CANDIDATE_INTENT_FILE, intent),
        (SNAPSHOT_FILE, snapshot.as_bytes()),
    ];
    if final_dir.exists() {
        verify_exact_evidence_directory(&final_dir, &expected)?;
        return Ok(());
    }
    let epoch = now_epoch_millis()?;
    let temp_dir = pair_root.join(format!(".pending-{dir_name}-{epoch}"));
    fs::create_dir(&temp_dir).map_err(|error| {
        format!(
            "MultiRevision-Evidence: temporaerer Acceptance-Evidence-Eintrag nicht anlegbar {}: {error}",
            temp_dir.display()
        )
    })?;
    let write_result = (|| -> Result<(), String> {
        for (name, bytes) in expected {
            fs::write(temp_dir.join(name), bytes).map_err(|error| {
                format!("MultiRevision-Evidence: {name} nicht schreibbar: {error}")
            })?;
        }
        fs::rename(&temp_dir, &final_dir).map_err(|error| {
            format!(
                "MultiRevision-Evidence: Acceptance-Evidence-Commit fehlgeschlagen {} -> {}: {error}",
                temp_dir.display(),
                final_dir.display()
            )
        })?;
        Ok(())
    })();
    if write_result.is_err() {
        let _ = fs::remove_dir_all(&temp_dir);
    }
    write_result
}

fn verify_exact_evidence_directory(
    directory: &Path,
    expected: &[(&str, &[u8])],
) -> Result<(), String> {
    let actual_names = fs::read_dir(directory)
        .map_err(|error| {
            format!(
                "MultiRevision-Evidence: bestehender Evidence-Eintrag nicht lesbar {}: {error}",
                directory.display()
            )
        })?
        .map(|entry| {
            entry
                .map(|entry| entry.file_name().to_string_lossy().to_string())
                .map_err(|error| {
                    format!("MultiRevision-Evidence: Evidence-Datei nicht lesbar: {error}")
                })
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_names = expected
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect::<BTreeSet<_>>();
    if actual_names != expected_names {
        return Err(format!(
            "MultiRevision-Evidence: bestehender Acceptance-Sidecar {} besitzt abweichende Dateimenge; Evidence-Konflikt",
            directory.display()
        ));
    }
    for &(name, bytes) in expected {
        let actual = fs::read(directory.join(name)).map_err(|error| {
            format!(
                "MultiRevision-Evidence: bestehende Evidence-Datei {} nicht lesbar: {error}",
                directory.join(name).display()
            )
        })?;
        if actual.as_slice() != bytes {
            return Err(format!(
                "MultiRevision-Evidence: bestehende Evidence-Datei {} weicht bytegenau ab; Evidence-Konflikt",
                directory.join(name).display()
            ));
        }
    }
    Ok(())
}

fn validate_rejection_stage(stage: &str) -> Result<(), String> {
    if matches!(
        stage,
        REJECTION_STAGE_DEAD_CODE_INTEGRITY
            | REJECTION_STAGE_DEPENDENCY_PREFLIGHT
            | REJECTION_STAGE_EFFECT_SNAPSHOT
            | REJECTION_STAGE_INCOMING_TAKE
            | REJECTION_STAGE_WATCH_PREFLIGHT
            | REJECTION_STAGE_APPLY_VERIFY
            | REJECTION_STAGE_CARGO_LOCK_RESOLUTION
            | REJECTION_STAGE_REEXEC_LAUNCH
            | REJECTION_STAGE_REEXEC_EXIT
            | REJECTION_STAGE_CRASH_RECOVERY
    ) {
        Ok(())
    } else {
        Err(format!(
            "MultiRevision-Evasion: unbekannte rejection_stage {stage:?}; fail-closed"
        ))
    }
}

fn pair_root(project_root: &Path, from_version: &str, to_version: &str) -> Result<PathBuf, String> {
    let safe_from = safe_component(from_version)?;
    let safe_to = safe_component(to_version)?;
    Ok(project_root
        .join(LEDGER_RELATIVE_ROOT)
        .join(format!("{safe_from}__{safe_to}")))
}

fn safe_component(value: &str) -> Result<String, String> {
    if value.is_empty()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'))
    {
        return Err(format!(
            "MultiRevision-Evasion: ungueltige Versionskomponente {value:?}"
        ));
    }
    Ok(value.to_string())
}

fn validate_sha256(value: &str) -> Result<(), String> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!(
            "MultiRevision-Evasion: ungueltiger SHA-256-Wert {value:?}"
        ));
    }
    Ok(())
}

fn metadata_value(text: &str, key: &str) -> Result<String, String> {
    let prefix = format!("{key}=");
    text.lines()
        .find_map(|line| line.strip_prefix(&prefix))
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("MultiRevision-Evasion: Metadatenfeld {key:?} fehlt"))
}

fn metadata_optional_value(text: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    text.lines()
        .find_map(|line| line.strip_prefix(&prefix))
        .map(ToOwned::to_owned)
}

fn compact_reason(reason: &str) -> String {
    let compact = reason.split_whitespace().collect::<Vec<_>>().join(" ");
    compact.chars().take(240).collect()
}

fn ledger_one_line(value: &str) -> String {
    value.replace(['\r', '\n'], " ")
}

fn now_epoch_millis() -> Result<u128, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .map_err(|error| format!("MultiRevision-Evasion: Systemzeit vor Epoch: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_known_vectors() {
        assert_eq!(
            sha256::hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256::hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn unsafe_versions_cannot_escape_ledger_root() {
        assert!(safe_component("0.10.1318").is_ok());
        assert!(safe_component("../escape").is_err());
        assert!(safe_component("a/b").is_err());
    }

    #[test]
    fn accepted_payload_hashes_are_recoverable_as_project_memory() {
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_multi_revision_acceptance_test_{}_{}",
            std::process::id(),
            now_epoch_millis().expect("clock")
        ));
        fs::create_dir_all(&root).expect("root");
        let first = format!("{:064x}", 11);
        let second = format!("{:064x}", 12);
        record_acceptance(&root, "0.10.20", "0.10.20", 7, &first, 0)
            .expect("first acceptance");
        record_acceptance(&root, "0.10.20", "0.10.21", 8, &second, 1)
            .expect("second acceptance");
        let accepted_r7 = load_accepted_zip_sha256(&root, "0.10.20", "0.10.20", 7)
            .expect("acceptance ledger r7");
        let accepted_r8 = load_accepted_zip_sha256(&root, "0.10.20", "0.10.21", 8)
            .expect("acceptance ledger r8");
        let wrong_revision = load_accepted_zip_sha256(&root, "0.10.20", "0.10.20", 8)
            .expect("wrong revision ledger");
        assert_eq!(accepted_r7, BTreeSet::from([first]));
        assert_eq!(accepted_r8, BTreeSet::from([second]));
        assert!(wrong_revision.is_empty());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn accepted_parent_closes_prior_semantic_diagnostic_chain_without_forgetting_rejections() {
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_multi_revision_active_chain_test_{}_{}",
            std::process::id(),
            now_epoch_millis().expect("clock")
        ));
        fs::create_dir_all(&root).expect("root");
        let first = format!("{:064x}", 21);
        let accepted = format!("{:064x}", 22);
        let second = format!("{:064x}", 23);
        record_rejection(
            &root,
            &RejectionRecord {
                from_version: "0.10.20",
                to_version: "0.10.20",
                take_revision: 7,
                zip_sha256: &first,
                rejection_stage: REJECTION_STAGE_INCOMING_TAKE,
                candidate_intent: Some(b"intent-first\n"),
                snapshot: "v1\tsrc/security.rs\t0000000000000001\tsecurity-negative-check\tdeny\n",
                reason: "first rejection",
            },
        )
        .expect("first rejection");
        record_acceptance(&root, "0.10.20", "0.10.20", 7, &accepted, 1)
            .expect("acceptance");
        record_rejection(
            &root,
            &RejectionRecord {
                from_version: "0.10.20",
                to_version: "0.10.20",
                take_revision: 8,
                zip_sha256: &second,
                rejection_stage: REJECTION_STAGE_REEXEC_EXIT,
                candidate_intent: Some(b"intent-second\n"),
                snapshot: "v1\tsrc/security.rs\t0000000000000002\tsecurity-negative-check\tdeny\n",
                reason: "second rejection",
            },
        )
        .expect("second rejection");

        let all = load_rejected_revisions(&root, "0.10.20", "0.10.20").expect("all");
        let active = active_diagnostic_rejections(&root, "0.10.20", "0.10.20", &all)
            .expect("active diagnostics");
        assert_eq!(
            previous_acceptance_rejection_prefix_len(&root, "0.10.20", "0.10.20", 8)
                .expect("parent rejection prefix"),
            1
        );
        assert_eq!(all.len(), 2);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].zip_sha256, second);
        assert!(reject_exact_replay(&all, &first).is_err());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn ten_rejected_iterations_remain_one_ordered_chain() {
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_multi_revision_test_{}_{}",
            std::process::id(),
            now_epoch_millis().expect("clock")
        ));
        fs::create_dir_all(&root).expect("root");
        for index in 1u64..=10 {
            let hash = format!("{index:064x}");
            let target = if index % 2 == 0 { "0.10.21" } else { "0.10.22" };
            record_rejection(
                &root,
                &RejectionRecord {
                    from_version: "0.10.20",
                    to_version: target,
                    take_revision: index,
                    zip_sha256: &hash,
                    rejection_stage: REJECTION_STAGE_INCOMING_TAKE,
                    candidate_intent: None,
                    snapshot: "v1\tsrc/music.rs\t0000000000000001\twriter-name-root\ttransform,voice,beam\n",
                    reason: &format!("rejection {index}"),
                },
            )
            .expect("record rejection");
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        let chain = load_rejected_revisions(&root, "0.10.20", "0.10.99").expect("chain");
        assert_eq!(chain.len(), 10);
        assert_eq!(chain.last().expect("latest").zip_sha256, format!("{:064x}", 10));
        assert!(reject_exact_replay(&chain, &format!("{:064x}", 4)).is_err());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn rejection_schema_v2_persists_stage_verdict_and_candidate_intent() {
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_rejection_v2_test_{}_{}",
            std::process::id(),
            now_epoch_millis().expect("clock")
        ));
        fs::create_dir_all(&root).expect("root");
        let hash = format!("{:064x}", 31);
        let intent = b"schema_version=1\nchange_class=KodexOnly\n";
        record_rejection(
            &root,
            &RejectionRecord {
                from_version: "0.10.20",
                to_version: "0.10.20",
                take_revision: 9,
                zip_sha256: &hash,
                rejection_stage: REJECTION_STAGE_DEPENDENCY_PREFLIGHT,
                candidate_intent: Some(intent),
                snapshot: "",
                reason: "dependency preflight reject",
            },
        )
        .expect("rejection");
        let dir = pair_root(&root, "0.10.20", "0.10.20")
            .expect("pair")
            .join(&hash);
        let metadata = fs::read_to_string(dir.join(METADATA_FILE)).expect("metadata");
        assert!(metadata.contains("schema=2\n"));
        assert!(metadata.contains("take_revision=9\n"));
        assert!(metadata.contains("rejection_stage=dependency_preflight\n"));
        assert!(metadata.contains("verdict=rejected\n"));
        assert_eq!(
            fs::read(dir.join(CANDIDATE_INTENT_FILE)).expect("intent"),
            intent
        );
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn legacy_schema1_rejection_remains_readable() {
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_rejection_legacy_test_{}_{}",
            std::process::id(),
            now_epoch_millis().expect("clock")
        ));
        let pair = pair_root(&root, "0.10.20", "0.10.20").expect("pair");
        fs::create_dir_all(&pair).expect("pair dir");
        let hash = format!("{:064x}", 32);
        let dir = pair.join(&hash);
        fs::create_dir(&dir).expect("entry");
        fs::write(
            dir.join(METADATA_FILE),
            format!(
                "schema=1\nfrom_version=0.10.20\nto_version=0.10.20\nzip_sha256={hash}\nattempt_epoch_millis=1\n"
            ),
        )
        .expect("metadata");
        fs::write(dir.join(SNAPSHOT_FILE), "legacy snapshot\n").expect("snapshot");
        fs::write(dir.join(REASON_FILE), "legacy reason\n").expect("reason");
        let loaded =
            load_rejected_revisions(&root, "0.10.20", "0.10.20").expect("load");
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].zip_sha256, hash);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn rejection_stage_is_first_authoritative_stage_for_candidate_sha() {
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_rejection_first_stage_test_{}_{}",
            std::process::id(),
            now_epoch_millis().expect("clock")
        ));
        fs::create_dir_all(&root).expect("root");
        let hash = format!("{:064x}", 34);
        record_rejection(
            &root,
            &RejectionRecord {
                from_version: "0.10.20",
                to_version: "0.10.20",
                take_revision: 11,
                zip_sha256: &hash,
                rejection_stage: REJECTION_STAGE_DEPENDENCY_PREFLIGHT,
                candidate_intent: Some(b"first intent\n"),
                snapshot: "first snapshot\n",
                reason: "first reason",
            },
        )
        .expect("first rejection");
        record_rejection(
            &root,
            &RejectionRecord {
                from_version: "0.10.20",
                to_version: "0.10.20",
                take_revision: 12,
                zip_sha256: &hash,
                rejection_stage: REJECTION_STAGE_REEXEC_EXIT,
                candidate_intent: Some(b"later intent\n"),
                snapshot: "later snapshot\n",
                reason: "later reason",
            },
        )
        .expect("same sha remains first rejection");
        let dir = pair_root(&root, "0.10.20", "0.10.20")
            .expect("pair")
            .join(&hash);
        let metadata = fs::read_to_string(dir.join(METADATA_FILE)).expect("metadata");
        assert!(metadata.contains("take_revision=11\n"));
        assert!(metadata.contains("rejection_stage=dependency_preflight\n"));
        assert_eq!(
            fs::read(dir.join(CANDIDATE_INTENT_FILE)).expect("intent"),
            b"first intent\n"
        );
        assert_eq!(
            fs::read_to_string(dir.join(SNAPSHOT_FILE)).expect("snapshot"),
            "first snapshot\n"
        );
        assert_eq!(
            fs::read_to_string(dir.join(REASON_FILE)).expect("reason"),
            "first reason"
        );
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn acceptance_evidence_is_create_or_verify_exact() {
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_acceptance_evidence_test_{}_{}",
            std::process::id(),
            now_epoch_millis().expect("clock")
        ));
        fs::create_dir_all(&root).expect("root");
        let hash = format!("{:064x}", 33);
        let intent = b"schema_version=1\nchange_class=KodexOnly\n";
        let snapshot = "v1\tsrc/x.rs\t0000000000000001\tkodex-negative-check\tdeny\n";
        record_acceptance_evidence(
            &root,
            "0.10.20",
            "0.10.20",
            10,
            &hash,
            Some(intent),
            snapshot,
        )
        .expect("create");
        record_acceptance_evidence(
            &root,
            "0.10.20",
            "0.10.20",
            10,
            &hash,
            Some(intent),
            snapshot,
        )
        .expect("verify exact");
        let conflict = record_acceptance_evidence(
            &root,
            "0.10.20",
            "0.10.20",
            10,
            &hash,
            Some(b"different intent\n"),
            snapshot,
        )
        .expect_err("conflict");
        assert!(conflict.contains("Evidence-Konflikt"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn acceptance_record_is_create_or_verify_without_rewriting_epoch() {
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_acceptance_idempotence_test_{}_{}",
            std::process::id(),
            now_epoch_millis().expect("clock")
        ));
        fs::create_dir_all(&root).expect("root");
        let hash = format!("{:064x}", 35);
        record_acceptance(&root, "0.10.20", "0.10.20", 12, &hash, 3)
            .expect("create acceptance");
        let (_, first) = read_acceptance_record_exact(
            &root, "0.10.20", "0.10.20", 12, &hash, 3,
        )
        .expect("first bytes");
        std::thread::sleep(std::time::Duration::from_millis(2));
        record_acceptance(&root, "0.10.20", "0.10.20", 12, &hash, 3)
            .expect("verify existing acceptance");
        let (_, second) = read_acceptance_record_exact(
            &root, "0.10.20", "0.10.20", 12, &hash, 3,
        )
        .expect("second bytes");
        assert_eq!(first, second);
        let conflict = record_acceptance(&root, "0.10.20", "0.10.20", 12, &hash, 4)
            .expect_err("prior rejection conflict must fail closed");
        assert!(conflict.contains("Acceptance-Provenienz-Konflikt"));
        let _ = fs::remove_dir_all(root);
    }

}
