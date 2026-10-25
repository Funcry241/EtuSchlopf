use crate::sha256;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use zip::write::FileOptions;

const NEURO_EVIDENCE_OUTPUT_DIR: &str = "target/etuschlopf_neuro_evidence";
const ARTIFACT_KIND: &str = "etuschlopf_neuro_evidence";
const SCHEMA_VERSION: u32 = 2;
const POPULATION_SCOPE: &str = "qualified_key_change_candidates_v1";
const CORPUS_COMMIT_ARTIFACT_KIND: &str = "etuschlopf_neuro_corpus_commit";
const CORPUS_COMMIT_SCHEMA_VERSION: u32 = 1;
const CORPUS_COMMIT_MAX_BYTES: usize = 128 * 1024;
const EMPTY_PAYLOAD_SHA256: &str =
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
const REQUIRED_CORPUS_PROJECTS: [&str; 10] = [
    "Etüde Nr. 1",
    "Etüde Nr. 2",
    "Etüde Nr. 3",
    "Etüde Nr. 4",
    "Etüde Nr. 5",
    "Etüde Nr. 6",
    "Etüde Nr. 7",
    "Etüde Nr. 8",
    "Etüde Nr. 9",
    "Etüde Nr. 10",
];
const PRODUCER: &str = "EtuSchlopf";
const CORPUS_ID: &str = "etuschlopf-tonal-audit";
const OWNER: &str = "TonalSegmentationEvidenceAudit";
const DECISION_CLASS: &str = "tonal_segmentation_boundary_audit_v1";
const DECISION_CONTRACT_FINGERPRINT: &str = "fnv64:4f2c765d00c527e6";
const PROMISING_CANDIDATE: &str = "promising_candidate_more_corpus_calibration";
const HELD_CANDIDATE: &str = "held_as_tonicization_or_chromatic_colour";
const FULL_EVIDENCE_CANDIDATE: &str = "full_evidence_candidate_write_still_closed";
const DECISIONS_PAYLOAD: &str = "decisions.jsonl";
const MANIFEST: &str = "manifest.json";
const DECISION_NULL_FIELDS_JSON_FRAGMENT: &str = r#""label":null,"proof":null"#;
const TONAL_LEDGER_HEADER: &str = concat!(
    "TonalBoundaryLedger: BoundaryM | Window | SourceFifths | Candidate | LBDM | Bee | ",
    "Margin | Stable | BassCadence | AccidentalCostBeforeAfter | HMM | GreenGroups | ",
    "Counterevidence | Verdict"
);
const ZIP_MAX_BYTES: u64 = 300 * 1024 * 1024;
const MANIFEST_MAX_BYTES: usize = 64 * 1024;
const PAYLOAD_MAX_BYTES: usize = 128 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TonalAuditSummary {
    boundaries_checked: usize,
    key_change_candidates: usize,
    full_evidence_candidates: usize,
    promising_candidates: usize,
    held_candidates: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TonalAuditRow {
    boundary_measure: usize,
    pre_start_measure: usize,
    post_end_measure: usize,
    source_fifths: i32,
    candidate_fifths: i32,
    lbdm: i32,
    bee: i32,
    margin: i64,
    stable: usize,
    bass_cadence: i32,
    accidental_cost_before: i64,
    accidental_cost_after: i64,
    hmm: i64,
    green_groups: usize,
    counter_boundary_weak: bool,
    counter_profile_margin_low: bool,
    counter_short_tonicization_risk: bool,
    counter_bass_cadence_unconfirmed: bool,
    counter_notation_saving_small: bool,
    counter_transition_prior_rejects: bool,
    counter_none_but_corpus_calibration_open: bool,
    verdict: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UtcStamp {
    filename: String,
    rfc3339: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EvidenceProvenance<'a> {
    producer_build: &'a str,
    source_identity: &'a str,
    corpus_run_id: &'a str,
    created_at_utc: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreparedCorpusArchive {
    project_id: String,
    file_name: String,
    staged_path: PathBuf,
    archive_sha256: String,
    qualified_decision_count: usize,
    manifest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RetentionEvidenceIdentity {
    project_id: String,
    corpus_run_id: String,
    producer_build: String,
    source_identity: String,
    created_at_utc: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RetentionCorpusCohort {
    commit_path: PathBuf,
    created_at_utc: String,
    members: Vec<PreparedCorpusArchive>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RetentionStandalone {
    path: PathBuf,
    project_id: String,
    created_at_utc: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TransferRetentionPlan {
    cohort_count: usize,
    standalone_count: usize,
    stale: Vec<PathBuf>,
    managed_before: usize,
}

pub(super) fn publish_neuro_evidence_for_successful_normal_run(
    project_root: &Path,
    result_path: &Path,
    project_id: &str,
    version: &str,
    corpus_run_id: &str,
    final_exit: i32,
) -> Result<Option<PathBuf>, String> {
    if final_exit != 0 {
        return Ok(None);
    }
    if corpus_run_id.trim().is_empty() {
        return Err("Neuro-Evidence: corpus_run_id fehlt".to_string());
    }
    let revision = read_take_revision(project_root)?;
    let producer_build = format!("{version}-{revision}");
    let source_identity = super::source_build_signature(project_root).map_err(|error| {
        format!("Neuro-Evidence: Source-Identity nicht bestimmbar: {error}")
    })?;
    let stamp = utc_stamp(SystemTime::now())?;
    let output_dir = project_root.join(NEURO_EVIDENCE_OUTPUT_DIR);
    fs::create_dir_all(&output_dir).map_err(|error| {
        format!(
            "Neuro-Evidence: Evidence-Verzeichnis nicht anlegbar {}: {error}",
            output_dir.display()
        )
    })?;
    let file_name = format!("etuschlopf_neuro_evidence_{}.zip", stamp.filename);
    let final_path = output_dir.join(&file_name);
    let temp_path = output_dir.join(format!(
        ".etuschlopf_neuro_evidence_{}_{}.tmp",
        stamp.filename,
        std::process::id()
    ));
    let provenance = EvidenceProvenance {
        producer_build: &producer_build,
        source_identity: &source_identity,
        corpus_run_id,
        created_at_utc: &stamp.rfc3339,
    };
    let _prepared = prepare_project_archive(
        result_path,
        project_id,
        provenance,
        file_name,
        temp_path.clone(),
    )?;
    if final_path.exists() {
        let _ = fs::remove_file(&temp_path);
        return Err(format!(
            "Neuro-Evidence: Ziel existiert bereits, keine Ueberschreibung {}",
            final_path.display()
        ));
    }
    fs::rename(&temp_path, &final_path).map_err(|error| {
        let _ = fs::remove_file(&temp_path);
        format!(
            "Neuro-Evidence: atomare Publikation fehlgeschlagen {}: {error}",
            final_path.display()
        )
    })?;
    let protected = BTreeSet::from([final_path.clone()]);
    match plan_transfer_retention(&output_dir, &protected) {
        Ok(plan) => {
            let mut removed = 0usize;
            let mut failed = 0usize;
            for path in &plan.stale {
                if fs::remove_file(path).is_ok() {
                    removed += 1;
                } else {
                    failed += 1;
                }
            }
            report_transfer_retention(&plan, removed, failed);
        }
        Err(error) => report_transfer_retention_error(&error),
    }
    Ok(Some(final_path))
}

pub(super) fn publish_neuro_evidence_for_successful_corpus_run(
    project_root: &Path,
    version: &str,
) -> Result<PathBuf, String> {
    let validation = super::corpus_run_manifest::validate_manifest(project_root, version);
    if !validation.valid {
        return Err(format!(
            "Neuro-Evidence-Corpus: hashgebundenes Korpusmanifest ungueltig: {}",
            validation.reason
        ));
    }
    let corpus = validation.manifest;
    if !corpus.completed || !corpus.full_corpus {
        return Err(
            "Neuro-Evidence-Corpus: nur ein abgeschlossenes Full-Corpus-Manifest darf publizieren"
                .to_string(),
        );
    }
    validate_required_corpus_members(&corpus)?;

    let corpus_run_id = corpus.run_id.trim();
    if corpus_run_id.is_empty() {
        return Err("Neuro-Evidence-Corpus: corpus_run_id fehlt".to_string());
    }
    let revision = read_take_revision(project_root)?;
    let producer_build = format!("{version}-{revision}");
    let source_identity = super::source_build_signature(project_root).map_err(|error| {
        format!("Neuro-Evidence-Corpus: Source-Identity nicht bestimmbar: {error}")
    })?;
    let stamp = utc_stamp(SystemTime::now())?;
    let provenance = EvidenceProvenance {
        producer_build: &producer_build,
        source_identity: &source_identity,
        corpus_run_id,
        created_at_utc: &stamp.rfc3339,
    };
    let stage_token = sha256::hex(corpus_run_id.as_bytes());
    let stage_dir = project_root
        .join("target")
        .join("etuschlopf_neuro_evidence_staging")
        .join(&stage_token[..16]);
    if stage_dir.exists() {
        fs::remove_dir_all(&stage_dir).map_err(|error| {
            format!(
                "Neuro-Evidence-Corpus: privates Alt-Staging nicht entfernbar {}: {error}",
                stage_dir.display()
            )
        })?;
    }
    fs::create_dir_all(&stage_dir).map_err(|error| {
        format!(
            "Neuro-Evidence-Corpus: privates Staging nicht anlegbar {}: {error}",
            stage_dir.display()
        )
    })?;

    let staged_result = (|| {
        let mut prepared = Vec::with_capacity(REQUIRED_CORPUS_PROJECTS.len());
        for (index, project_id) in REQUIRED_CORPUS_PROJECTS.iter().enumerate() {
            let evidence = corpus
                .projects
                .iter()
                .find(|row| row.project == *project_id)
                .ok_or_else(|| {
                    format!(
                        "Neuro-Evidence-Corpus: Projekt fehlt im validierten Manifest: {project_id}"
                    )
                })?;
            let result_path = resolve_manifest_result_path(project_root, &evidence.result_file);
            let file_name = format!(
                "etuschlopf_neuro_evidence_{}_{:02}.zip",
                stamp.filename,
                index + 1
            );
            let staged_path = stage_dir.join(&file_name);
            prepared.push(prepare_project_archive(
                &result_path,
                project_id,
                provenance,
                file_name,
                staged_path,
            )?);
        }
        validate_prepared_corpus(
            &prepared,
            &producer_build,
            &source_identity,
            corpus_run_id,
        )?;
        let commit = corpus_commit_json(
            &prepared,
            &producer_build,
            &source_identity,
            corpus_run_id,
            &stamp.rfc3339,
        );
        validate_corpus_commit_bytes(
            &commit,
            &prepared,
            &producer_build,
            &source_identity,
            corpus_run_id,
        )?;
        let staged_commit = stage_dir.join(format!(
            "etuschlopf_neuro_corpus_commit_{}.json",
            stamp.filename
        ));
        fs::write(&staged_commit, commit.as_bytes()).map_err(|error| {
            format!(
                "Neuro-Evidence-Corpus: Commit-Staging fehlgeschlagen {}: {error}",
                staged_commit.display()
            )
        })?;
        let readback = fs::read(&staged_commit).map_err(|error| {
            format!(
                "Neuro-Evidence-Corpus: Commit-Readback fehlgeschlagen {}: {error}",
                staged_commit.display()
            )
        })?;
        if readback != commit.as_bytes() {
            return Err(
                "Neuro-Evidence-Corpus: Commit-Readback weicht von geschriebenen Bytes ab"
                    .to_string(),
            );
        }
        publish_prepared_corpus(project_root, &prepared, &staged_commit)
    })();

    match staged_result {
        Ok(commit_path) => {
            let _ = fs::remove_dir_all(&stage_dir);
            Ok(commit_path)
        }
        Err(error) => {
            let _ = fs::remove_dir_all(&stage_dir);
            Err(error)
        }
    }
}

fn validate_required_corpus_members(
    corpus: &super::corpus_run_manifest::CorpusRunManifest,
) -> Result<(), String> {
    if corpus.expected_project_count != REQUIRED_CORPUS_PROJECTS.len()
        || corpus.expected_projects.len() != REQUIRED_CORPUS_PROJECTS.len()
        || corpus.projects.len() != REQUIRED_CORPUS_PROJECTS.len()
    {
        return Err(format!(
            "Neuro-Evidence-Corpus: 10/10 erforderlich, Manifest meldet expected {} / names {} / evidence {}",
            corpus.expected_project_count,
            corpus.expected_projects.len(),
            corpus.projects.len()
        ));
    }
    let required = REQUIRED_CORPUS_PROJECTS
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let expected = corpus
        .expected_projects
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let evidenced = corpus
        .projects
        .iter()
        .map(|row| row.project.as_str())
        .collect::<BTreeSet<_>>();
    if expected != required || evidenced != required {
        return Err(
            "Neuro-Evidence-Corpus: Projektmenge ist nicht exakt Etuede Nr. 1 bis 10"
                .to_string(),
        );
    }
    Ok(())
}

fn resolve_manifest_result_path(project_root: &Path, result_file: &str) -> PathBuf {
    let path = PathBuf::from(result_file);
    if path.is_absolute() {
        path
    } else {
        project_root.join(path)
    }
}

fn prepare_project_archive(
    result_path: &Path,
    project_id: &str,
    provenance: EvidenceProvenance<'_>,
    file_name: String,
    staged_path: PathBuf,
) -> Result<PreparedCorpusArchive, String> {
    let report = fs::read_to_string(result_path).map_err(|error| {
        format!(
            "Neuro-Evidence: Ergebnisbericht nicht lesbar {}: {error}",
            result_path.display()
        )
    })?;
    let summary = parse_tonal_audit_summary(&report)?;
    let rows = parse_tonal_audit_rows(&report, summary)?;
    let qualified_decision_count = summary.key_change_candidates;
    if rows.len() != qualified_decision_count {
        return Err(format!(
            "Neuro-Evidence: Population-Closure verletzt: qualified {} != DecisionEvidence {}",
            qualified_decision_count,
            rows.len()
        ));
    }
    let decisions = decisions_jsonl(
        &rows,
        project_id,
        provenance.producer_build,
        provenance.source_identity,
        provenance.corpus_run_id,
    )?;
    validate_decisions_payload(
        &decisions,
        qualified_decision_count,
        project_id,
        provenance.producer_build,
        provenance.source_identity,
        provenance.corpus_run_id,
    )?;
    let payload_hash = sha256::hex(decisions.as_bytes());
    if qualified_decision_count == 0 && payload_hash != EMPTY_PAYLOAD_SHA256 {
        return Err(
            "Neuro-Evidence: Zero-Case-Payload besitzt nicht den SHA-256 der leeren Payload"
                .to_string(),
        );
    }
    let manifest = manifest_json(
        project_id,
        provenance,
        qualified_decision_count,
        decisions.len(),
        &payload_hash,
    );
    validate_manifest_bytes(
        &manifest,
        project_id,
        provenance.producer_build,
        provenance.source_identity,
        provenance.corpus_run_id,
        qualified_decision_count,
    )?;
    if let Some(parent) = staged_path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "Neuro-Evidence: Staging-Ordner nicht anlegbar {}: {error}",
                parent.display()
            )
        })?;
    }
    if staged_path.exists() {
        fs::remove_file(&staged_path).map_err(|error| {
            format!(
                "Neuro-Evidence: alte Staging-Datei nicht entfernbar {}: {error}",
                staged_path.display()
            )
        })?;
    }
    write_closed_archive(&staged_path, &manifest, decisions.as_bytes())?;
    validate_closed_archive(
        &staged_path,
        manifest.as_bytes(),
        decisions.as_bytes(),
        &payload_hash,
    )?;
    let zip_bytes = fs::metadata(&staged_path)
        .map_err(|error| format!("Neuro-Evidence: ZIP-Metadaten fehlen: {error}"))?
        .len();
    if zip_bytes > ZIP_MAX_BYTES {
        let _ = fs::remove_file(&staged_path);
        return Err(format!(
            "Neuro-Evidence: ZIP-Groesse {zip_bytes} ueberschreitet {ZIP_MAX_BYTES}"
        ));
    }
    let archive_sha256 = file_sha256(&staged_path)?;
    Ok(PreparedCorpusArchive {
        project_id: project_id.to_string(),
        file_name,
        staged_path,
        archive_sha256,
        qualified_decision_count,
        manifest,
    })
}

fn validate_prepared_corpus(
    prepared: &[PreparedCorpusArchive],
    producer_build: &str,
    source_identity: &str,
    corpus_run_id: &str,
) -> Result<(), String> {
    if prepared.len() != REQUIRED_CORPUS_PROJECTS.len() {
        return Err(format!(
            "Neuro-Evidence-Corpus: vorbereitet {} statt 10 Archive",
            prepared.len()
        ));
    }
    let mut projects = BTreeSet::new();
    let mut files = BTreeSet::new();
    for archive in prepared {
        if !projects.insert(archive.project_id.as_str()) {
            return Err(format!(
                "Neuro-Evidence-Corpus: doppelte project-id {}",
                archive.project_id
            ));
        }
        if !files.insert(archive.file_name.as_str()) {
            return Err(format!(
                "Neuro-Evidence-Corpus: doppelter Archivname {}",
                archive.file_name
            ));
        }
        validate_manifest_bytes(
            &archive.manifest,
            &archive.project_id,
            producer_build,
            source_identity,
            corpus_run_id,
            archive.qualified_decision_count,
        )?;
        if file_sha256(&archive.staged_path)? != archive.archive_sha256 {
            return Err(format!(
                "Neuro-Evidence-Corpus: Archivhash im Staging instabil {}",
                archive.file_name
            ));
        }
    }
    let required = REQUIRED_CORPUS_PROJECTS
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if projects != required {
        return Err("Neuro-Evidence-Corpus: vorbereitete Projektmenge ist nicht 10/10".to_string());
    }
    Ok(())
}

fn corpus_commit_json(
    prepared: &[PreparedCorpusArchive],
    producer_build: &str,
    source_identity: &str,
    corpus_run_id: &str,
    created_at_utc: &str,
) -> String {
    let mut out = String::new();
    let _ = writeln!(&mut out, "{{");
    let _ = writeln!(
        &mut out,
        "  \"artifact-kind\": \"{CORPUS_COMMIT_ARTIFACT_KIND}\","
    );
    let _ = writeln!(
        &mut out,
        "  \"schema-version\": {CORPUS_COMMIT_SCHEMA_VERSION},"
    );
    let _ = writeln!(&mut out, "  \"producer\": \"{PRODUCER}\",");
    let _ = writeln!(
        &mut out,
        "  \"producer-build\": \"{}\",",
        json_escape(producer_build)
    );
    let _ = writeln!(
        &mut out,
        "  \"created-at-utc\": \"{}\",",
        json_escape(created_at_utc)
    );
    let _ = writeln!(
        &mut out,
        "  \"corpus-run-id\": \"{}\",",
        json_escape(corpus_run_id)
    );
    let _ = writeln!(
        &mut out,
        "  \"etuschlopf-source-identity\": \"{}\",",
        json_escape(source_identity)
    );
    let _ = writeln!(&mut out, "  \"corpus-id\": \"{CORPUS_ID}\",");
    let _ = writeln!(&mut out, "  \"decision-class\": \"{DECISION_CLASS}\",");
    let _ = writeln!(
        &mut out,
        "  \"decision-contract-fingerprint\": \"{DECISION_CONTRACT_FINGERPRINT}\","
    );
    let _ = writeln!(
        &mut out,
        "  \"population-scope\": \"{POPULATION_SCOPE}\","
    );
    let _ = writeln!(&mut out, "  \"projects\": [");
    for (index, archive) in prepared.iter().enumerate() {
        let comma = if index + 1 == prepared.len() { "" } else { "," };
        let _ = writeln!(
            &mut out,
            "    {{\"project-id\":\"{}\",\"evidence-archive\":\"{}\",\"archive-sha-256\":\"{}\",\"qualified-decision-count\":{}}}{comma}",
            json_escape(&archive.project_id),
            json_escape(&archive.file_name),
            archive.archive_sha256,
            archive.qualified_decision_count
        );
    }
    let _ = writeln!(&mut out, "  ]");
    let _ = writeln!(&mut out, "}}");
    out
}

fn validate_corpus_commit_bytes(
    commit: &str,
    prepared: &[PreparedCorpusArchive],
    producer_build: &str,
    source_identity: &str,
    corpus_run_id: &str,
) -> Result<(), String> {
    if commit.is_empty() || commit.len() > CORPUS_COMMIT_MAX_BYTES {
        return Err("Neuro-Evidence-Corpus: Commit-Groesse ausserhalb Vertrag".to_string());
    }
    for required in [
        format!("\"artifact-kind\": \"{CORPUS_COMMIT_ARTIFACT_KIND}\""),
        format!("\"schema-version\": {CORPUS_COMMIT_SCHEMA_VERSION}"),
        format!("\"producer-build\": \"{}\"", json_escape(producer_build)),
        format!("\"corpus-run-id\": \"{}\"", json_escape(corpus_run_id)),
        format!(
            "\"etuschlopf-source-identity\": \"{}\"",
            json_escape(source_identity)
        ),
        format!("\"corpus-id\": \"{CORPUS_ID}\""),
        format!("\"decision-class\": \"{DECISION_CLASS}\""),
        format!(
            "\"decision-contract-fingerprint\": \"{DECISION_CONTRACT_FINGERPRINT}\""
        ),
        format!("\"population-scope\": \"{POPULATION_SCOPE}\""),
    ] {
        if !commit.contains(&required) {
            return Err(format!(
                "Neuro-Evidence-Corpus: Commit-Pflicht fehlt: {required}"
            ));
        }
    }
    if prepared.len() != REQUIRED_CORPUS_PROJECTS.len()
        || commit.matches("\"project-id\":").count() != REQUIRED_CORPUS_PROJECTS.len()
        || commit.matches("\"evidence-archive\":").count() != REQUIRED_CORPUS_PROJECTS.len()
        || commit.matches("\"archive-sha-256\":").count() != REQUIRED_CORPUS_PROJECTS.len()
        || commit
            .matches("\"qualified-decision-count\":")
            .count()
            != REQUIRED_CORPUS_PROJECTS.len()
    {
        return Err("Neuro-Evidence-Corpus: Commit enthaelt nicht exakt zehn Mitglieder".to_string());
    }
    for archive in prepared {
        for required in [
            format!("\"project-id\":\"{}\"", json_escape(&archive.project_id)),
            format!(
                "\"evidence-archive\":\"{}\"",
                json_escape(&archive.file_name)
            ),
            format!("\"archive-sha-256\":\"{}\"", archive.archive_sha256),
            format!(
                "\"qualified-decision-count\":{}",
                archive.qualified_decision_count
            ),
        ] {
            if !commit.contains(&required) {
                return Err(format!(
                    "Neuro-Evidence-Corpus: Mitgliedspflicht fehlt: {required}"
                ));
            }
        }
    }
    Ok(())
}

fn publish_prepared_corpus(
    project_root: &Path,
    prepared: &[PreparedCorpusArchive],
    staged_commit: &Path,
) -> Result<PathBuf, String> {
    let output_dir = project_root.join(NEURO_EVIDENCE_OUTPUT_DIR);
    fs::create_dir_all(&output_dir).map_err(|error| {
        format!(
            "Neuro-Evidence-Corpus: Evidence-Verzeichnis nicht anlegbar {}: {error}",
            output_dir.display()
        )
    })?;
    let commit_name = staged_commit
        .file_name()
        .ok_or_else(|| "Neuro-Evidence-Corpus: Commit-Dateiname fehlt".to_string())?
        .to_owned();
    let final_commit = output_dir.join(&commit_name);
    if final_commit.exists() {
        return Err(format!(
            "Neuro-Evidence-Corpus: Commit-Ziel existiert bereits {}",
            final_commit.display()
        ));
    }
    for archive in prepared {
        let final_path = output_dir.join(&archive.file_name);
        if final_path.exists() {
            return Err(format!(
                "Neuro-Evidence-Corpus: Archiv-Ziel existiert bereits {}",
                final_path.display()
            ));
        }
    }

    let mut published = Vec::new();
    let result = (|| {
        for archive in prepared {
            let final_path = output_dir.join(&archive.file_name);
            let temp_path = output_dir.join(format!(
                ".{}.{}.tmp",
                archive.file_name,
                std::process::id()
            ));
            if temp_path.exists() {
                fs::remove_file(&temp_path).map_err(|error| {
                    format!(
                        "Neuro-Evidence-Corpus: alte Publication-Tempdatei nicht entfernbar {}: {error}",
                        temp_path.display()
                    )
                })?;
            }
            fs::copy(&archive.staged_path, &temp_path).map_err(|error| {
                format!(
                    "Neuro-Evidence-Corpus: Archiv nicht in Publication-Staging kopierbar {}: {error}",
                    archive.file_name
                )
            })?;
            if file_sha256(&temp_path)? != archive.archive_sha256 {
                let _ = fs::remove_file(&temp_path);
                return Err(format!(
                    "Neuro-Evidence-Corpus: Publication-Hash weicht ab {}",
                    archive.file_name
                ));
            }
            fs::rename(&temp_path, &final_path).map_err(|error| {
                let _ = fs::remove_file(&temp_path);
                format!(
                    "Neuro-Evidence-Corpus: atomare Archivpublikation fehlgeschlagen {}: {error}",
                    final_path.display()
                )
            })?;
            published.push(final_path);
        }

        let commit_bytes = fs::read(staged_commit).map_err(|error| {
            format!(
                "Neuro-Evidence-Corpus: staged Commit nicht lesbar {}: {error}",
                staged_commit.display()
            )
        })?;
        let commit_temp = output_dir.join(format!(
            ".{}.{}.tmp",
            commit_name.to_string_lossy(),
            std::process::id()
        ));
        if commit_temp.exists() {
            fs::remove_file(&commit_temp).map_err(|error| {
                format!(
                    "Neuro-Evidence-Corpus: alte Commit-Tempdatei nicht entfernbar {}: {error}",
                    commit_temp.display()
                )
            })?;
        }
        fs::write(&commit_temp, &commit_bytes).map_err(|error| {
            format!(
                "Neuro-Evidence-Corpus: Commit-Publication-Staging fehlgeschlagen {}: {error}",
                commit_temp.display()
            )
        })?;
        if fs::read(&commit_temp).map_err(|error| {
            format!(
                "Neuro-Evidence-Corpus: Commit-Publication-Readback fehlgeschlagen {}: {error}",
                commit_temp.display()
            )
        })? != commit_bytes
        {
            let _ = fs::remove_file(&commit_temp);
            return Err(
                "Neuro-Evidence-Corpus: Commit-Publication-Readback inkonsistent".to_string(),
            );
        }
        fs::rename(&commit_temp, &final_commit).map_err(|error| {
            let _ = fs::remove_file(&commit_temp);
            format!(
                "Neuro-Evidence-Corpus: logischer Commit konnte nicht publiziert werden {}: {error}",
                final_commit.display()
            )
        })?;
        Ok(final_commit.clone())
    })();

    if result.is_err() {
        for path in published {
            let _ = fs::remove_file(path);
        }
        let _ = fs::remove_file(&final_commit);
    } else {
        let mut protected = published.iter().cloned().collect::<BTreeSet<_>>();
        protected.insert(final_commit.clone());
        match plan_transfer_retention(&output_dir, &protected) {
            Ok(plan) => {
                let mut removed = 0usize;
                let mut failed = 0usize;
                for path in &plan.stale {
                    if fs::remove_file(path).is_ok() {
                        removed += 1;
                    } else {
                        failed += 1;
                    }
                }
                report_transfer_retention(&plan, removed, failed);
            }
            Err(error) => report_transfer_retention_error(&error),
        }
    }
    result
}


fn plan_transfer_retention(
    output_dir: &Path,
    protected: &BTreeSet<PathBuf>,
) -> Result<TransferRetentionPlan, String> {
    let mut managed = Vec::new();
    let mut commits = Vec::new();
    let mut evidence = Vec::new();
    for entry in fs::read_dir(output_dir).map_err(|error| {
        format!("Transfer-Retention: Verzeichnis nicht lesbar: {error}")
    })? {
        let entry = entry.map_err(|error| format!("Transfer-Retention: Eintrag unlesbar: {error}"))?;
        if !entry
            .file_type()
            .map_err(|error| format!("Transfer-Retention: Dateityp unlesbar: {error}"))?
            .is_file()
        {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if is_managed_commit_name(&name) {
            managed.push(entry.path());
            commits.push(entry.path());
        } else if is_managed_evidence_name(&name) {
            managed.push(entry.path());
            evidence.push(entry.path());
        }
    }
    let cohorts = commits
        .iter()
        .filter_map(|path| validate_retention_cohort(output_dir, path).ok())
        .collect::<Vec<_>>();
    let committed_names = cohorts
        .iter()
        .flat_map(|cohort| cohort.members.iter().map(|member| member.file_name.clone()))
        .collect::<BTreeSet<_>>();
    let standalone = evidence
        .iter()
        .filter_map(|path| {
            let name = path.file_name()?.to_string_lossy().into_owned();
            if committed_names.contains(&name) {
                return None;
            }
            let identity = read_retention_evidence_identity(path).ok()?;
            Some(RetentionStandalone {
                path: path.clone(),
                project_id: identity.project_id,
                created_at_utc: identity.created_at_utc,
            })
        })
        .collect::<Vec<_>>();
    Ok(select_transfer_retention(
        managed,
        cohorts,
        standalone,
        protected,
    ))
}

fn select_transfer_retention(
    managed: Vec<PathBuf>,
    mut cohorts: Vec<RetentionCorpusCohort>,
    standalone: Vec<RetentionStandalone>,
    protected: &BTreeSet<PathBuf>,
) -> TransferRetentionPlan {
    cohorts.sort_by(|left, right| {
        right
            .created_at_utc
            .cmp(&left.created_at_utc)
            .then_with(|| right.commit_path.cmp(&left.commit_path))
    });
    let mut keep = protected.clone();
    for cohort in cohorts.iter().take(2) {
        keep.insert(cohort.commit_path.clone());
        if let Some(parent) = cohort.commit_path.parent() {
            keep.extend(cohort.members.iter().map(|member| parent.join(&member.file_name)));
        }
    }
    let committed_names = cohorts
        .iter()
        .flat_map(|cohort| cohort.members.iter().map(|member| member.file_name.clone()))
        .collect::<BTreeSet<_>>();
    let mut by_project = BTreeMap::<String, Vec<RetentionStandalone>>::new();
    let mut valid_standalone = BTreeSet::new();
    for candidate in standalone {
        let name = candidate
            .path
            .file_name()
            .map(|value| value.to_string_lossy().into_owned());
        if name.as_ref().is_some_and(|value| committed_names.contains(value)) {
            continue;
        }
        valid_standalone.insert(candidate.path.clone());
        by_project
            .entry(candidate.project_id.clone())
            .or_default()
            .push(candidate);
    }
    let mut standalone_count = 0usize;
    for candidates in by_project.values_mut() {
        candidates.sort_by(|left, right| {
            right
                .created_at_utc
                .cmp(&left.created_at_utc)
                .then_with(|| right.path.cmp(&left.path))
        });
        if let Some(newest) = candidates.first() {
            keep.insert(newest.path.clone());
            standalone_count += 1;
        }
    }
    let legacy_cleanup = cohorts.len() >= 2;
    let stale = managed
        .iter()
        .filter(|path| {
            !keep.contains(*path) && (legacy_cleanup || valid_standalone.contains(*path))
        })
        .cloned()
        .collect();
    TransferRetentionPlan {
        cohort_count: cohorts.len().min(2),
        standalone_count,
        stale,
        managed_before: managed.len(),
    }
}

fn report_transfer_retention(plan: &TransferRetentionPlan, removed: usize, failed: usize) {
    let managed = plan.managed_before.saturating_sub(removed);
    if failed == 0 {
        println!(
            "[EtuSchlopf] Transfer-Retention ........ GREEN · {} cohorts · {} standalone · {} removed · {managed} managed",
            plan.cohort_count, plan.standalone_count, removed
        );
    } else {
        eprintln!(
            "[EtuSchlopf] Transfer-Retention ........ YELLOW · {} cohorts · {} standalone · {} removed · {failed} stale locked · {managed} managed",
            plan.cohort_count, plan.standalone_count, removed
        );
    }
}

fn report_transfer_retention_error(error: &str) {
    eprintln!(
        "[EtuSchlopf] Transfer-Retention ........ YELLOW · {error}; Publication bleibt gueltig"
    );
}

fn is_managed_evidence_name(name: &str) -> bool {
    name.starts_with("etuschlopf_neuro_evidence_")
        && name.ends_with(".zip")
        && !name.contains('/')
        && !name.contains('\\')
}

fn is_managed_commit_name(name: &str) -> bool {
    name.starts_with("etuschlopf_neuro_corpus_commit_")
        && name.ends_with(".json")
        && !name.contains('/')
        && !name.contains('\\')
}

fn validate_retention_cohort(
    output_dir: &Path,
    commit_path: &Path,
) -> Result<RetentionCorpusCohort, String> {
    let commit = fs::read_to_string(commit_path)
        .map_err(|error| format!("Transfer-Retention: Commit unlesbar: {error}"))?;
    if commit.len() > CORPUS_COMMIT_MAX_BYTES {
        return Err("Transfer-Retention: Commit zu gross".to_string());
    }
    let producer_build = retention_json_string(&commit, "producer-build")?;
    let source_identity = retention_json_string(&commit, "etuschlopf-source-identity")?;
    let corpus_run_id = retention_json_string(&commit, "corpus-run-id")?;
    let created_at_utc = retention_json_string(&commit, "created-at-utc")?;
    let lines = commit
        .lines()
        .filter(|line| line.contains("\"project-id\":"))
        .collect::<Vec<_>>();
    if lines.len() != REQUIRED_CORPUS_PROJECTS.len() {
        return Err("Transfer-Retention: Commit ist nicht 10/10".to_string());
    }
    let mut projects = BTreeSet::new();
    let mut files = BTreeSet::new();
    let mut members = Vec::with_capacity(lines.len());
    for line in lines {
        let project_id = retention_json_string(line, "project-id")?;
        let file_name = retention_json_string(line, "evidence-archive")?;
        let archive_sha256 = retention_json_string(line, "archive-sha-256")?;
        if !is_managed_evidence_name(&file_name)
            || archive_sha256.len() != 64
            || !archive_sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
            || !projects.insert(project_id.clone())
            || !files.insert(file_name.clone())
        {
            return Err("Transfer-Retention: Commit-Mitglied ungueltig".to_string());
        }
        members.push(PreparedCorpusArchive {
            project_id,
            file_name: file_name.clone(),
            staged_path: output_dir.join(file_name),
            archive_sha256: archive_sha256.to_ascii_lowercase(),
            qualified_decision_count: retention_json_usize(line, "qualified-decision-count")?,
            manifest: String::new(),
        });
    }
    let required = REQUIRED_CORPUS_PROJECTS
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if projects.iter().map(String::as_str).collect::<BTreeSet<_>>() != required {
        return Err("Transfer-Retention: Commit-Projektmenge ist nicht 10/10".to_string());
    }
    validate_corpus_commit_bytes(
        &commit,
        &members,
        &producer_build,
        &source_identity,
        &corpus_run_id,
    )?;
    for member in &members {
        if !member.staged_path.is_file()
            || file_sha256(&member.staged_path)? != member.archive_sha256
        {
            return Err(format!(
                "Transfer-Retention: Commit-Hashbindung verletzt {}",
                member.file_name
            ));
        }
        let identity = read_retention_evidence_identity(&member.staged_path)?;
        if identity.project_id != member.project_id
            || identity.corpus_run_id != corpus_run_id
            || identity.producer_build != producer_build
            || identity.source_identity != source_identity
        {
            return Err(format!(
                "Transfer-Retention: Kohortenidentitaet inkohärent {}",
                member.file_name
            ));
        }
    }
    Ok(RetentionCorpusCohort {
        commit_path: commit_path.to_path_buf(),
        created_at_utc,
        members,
    })
}

fn read_retention_evidence_identity(path: &Path) -> Result<RetentionEvidenceIdentity, String> {
    if fs::metadata(path)
        .map_err(|error| format!("Transfer-Retention: Evidence-Metadaten fehlen: {error}"))?
        .len()
        > ZIP_MAX_BYTES
    {
        return Err("Transfer-Retention: Evidence-ZIP zu gross".to_string());
    }
    let file = File::open(path)
        .map_err(|error| format!("Transfer-Retention: Evidence-ZIP unlesbar: {error}"))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|error| format!("Transfer-Retention: Evidence-ZIP ungueltig: {error}"))?;
    if archive.len() != 2 {
        return Err("Transfer-Retention: Evidence-ZIP ist nicht geschlossen".to_string());
    }
    let mut names = (0..archive.len())
        .map(|index| {
            archive
                .by_index(index)
                .map(|entry| entry.name().to_string())
                .map_err(|error| format!("Transfer-Retention: ZIP-Eintrag unlesbar: {error}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    names.sort();
    if names != [DECISIONS_PAYLOAD.to_string(), MANIFEST.to_string()] {
        return Err("Transfer-Retention: Evidence-ZIP-Allowlist verletzt".to_string());
    }
    let mut manifest = String::new();
    archive
        .by_name(MANIFEST)
        .map_err(|error| format!("Transfer-Retention: manifest.json fehlt: {error}"))?
        .read_to_string(&mut manifest)
        .map_err(|error| format!("Transfer-Retention: manifest.json unlesbar: {error}"))?;
    let identity = RetentionEvidenceIdentity {
        project_id: retention_json_string(&manifest, "project-id")?,
        corpus_run_id: retention_json_string(&manifest, "corpus-run-id")?,
        producer_build: retention_json_string(&manifest, "producer-build")?,
        source_identity: retention_json_string(&manifest, "etuschlopf-source-identity")?,
        created_at_utc: retention_json_string(&manifest, "created-at-utc")?,
    };
    validate_manifest_bytes(
        &manifest,
        &identity.project_id,
        &identity.producer_build,
        &identity.source_identity,
        &identity.corpus_run_id,
        retention_json_usize(&manifest, "qualified-decision-count")?,
    )?;
    Ok(identity)
}

fn retention_json_string(text: &str, key: &str) -> Result<String, String> {
    let marker = format!("\"{key}\"");
    let tail = text
        .get(text.find(&marker).ok_or_else(|| format!("Transfer-Retention: JSON-Feld fehlt {key}"))? + marker.len()..)
        .ok_or_else(|| format!("Transfer-Retention: JSON-Feld ungueltig {key}"))?
        .trim_start()
        .strip_prefix(':')
        .ok_or_else(|| format!("Transfer-Retention: JSON-Doppelpunkt fehlt {key}"))?
        .trim_start()
        .strip_prefix('"')
        .ok_or_else(|| format!("Transfer-Retention: JSON-String erwartet {key}"))?;
    let end = tail
        .find('"')
        .ok_or_else(|| format!("Transfer-Retention: JSON-String offen {key}"))?;
    let value = &tail[..end];
    if value.contains('\\') {
        return Err(format!("Transfer-Retention: JSON-Escape nicht erlaubt {key}"));
    }
    Ok(value.to_string())
}

fn retention_json_usize(text: &str, key: &str) -> Result<usize, String> {
    let marker = format!("\"{key}\"");
    let tail = text
        .get(text.find(&marker).ok_or_else(|| format!("Transfer-Retention: JSON-Feld fehlt {key}"))? + marker.len()..)
        .ok_or_else(|| format!("Transfer-Retention: JSON-Feld ungueltig {key}"))?
        .trim_start()
        .strip_prefix(':')
        .ok_or_else(|| format!("Transfer-Retention: JSON-Doppelpunkt fehlt {key}"))?
        .trim_start();
    let digits = tail
        .chars()
        .take_while(|value| value.is_ascii_digit())
        .collect::<String>();
    digits
        .parse::<usize>()
        .map_err(|error| format!("Transfer-Retention: JSON-Zahl ungueltig {key}: {error}"))
}

fn file_sha256(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| {
        format!(
            "Neuro-Evidence: Datei fuer SHA-256 nicht lesbar {}: {error}",
            path.display()
        )
    })?;
    Ok(sha256::hex(&bytes))
}

fn parse_tonal_audit_summary(report: &str) -> Result<TonalAuditSummary, String> {
    let mut matches = report
        .lines()
        .filter(|line| line.starts_with("Kurz: TonalSegmentationEvidenceAudit:"));
    let line = matches.next().ok_or_else(|| {
        "Neuro-Evidence: TonalSegmentationEvidenceAudit-Summary fehlt; kein Evidence-Artefakt"
            .to_string()
    })?;
    if matches.next().is_some() {
        return Err(
            concat!(
                "Neuro-Evidence: TonalSegmentationEvidenceAudit-Summary ist mehrdeutig; ",
                "kein Evidence-Artefakt"
            )
            .to_string(),
        );
    }
    let summary = TonalAuditSummary {
        boundaries_checked: summary_usize(line, "BoundariesChecked")?,
        key_change_candidates: summary_usize(line, "KeyChangeCandidates")?,
        full_evidence_candidates: summary_usize(line, "FullEvidenceCandidates")?,
        promising_candidates: summary_usize(line, "PromisingCandidates")?,
        held_candidates: summary_usize(line, "HeldCandidates")?,
    };
    if summary.full_evidence_candidates
        + summary.promising_candidates
        + summary.held_candidates
        != summary.key_change_candidates
    {
        return Err(
            "Neuro-Evidence: Tonal-Audit-Summary verletzt Candidate-Closure".to_string(),
        );
    }
    if summary.key_change_candidates > summary.boundaries_checked {
        return Err(
            "Neuro-Evidence: mehr KeyChangeCandidates als gepruefte Boundaries".to_string(),
        );
    }
    Ok(summary)
}

fn summary_usize(line: &str, label: &str) -> Result<usize, String> {
    let token = format!("{label} ");
    let tail = line
        .split_once(&token)
        .map(|(_, tail)| tail)
        .ok_or_else(|| format!("Neuro-Evidence: Summary-Feld {label} fehlt"))?;
    let value = tail
        .split(|ch: char| !ch.is_ascii_digit())
        .next()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("Neuro-Evidence: Summary-Feld {label} ist nicht numerisch"))?;
    value
        .parse::<usize>()
        .map_err(|error| format!("Neuro-Evidence: Summary-Feld {label} ungueltig: {error}"))
}

fn parse_tonal_audit_rows(
    report: &str,
    summary: TonalAuditSummary,
) -> Result<Vec<TonalAuditRow>, String> {
    let lines = report.lines().collect::<Vec<_>>();
    let headers = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| **line == TONAL_LEDGER_HEADER)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    if headers.len() != 1 {
        return Err(format!(
            "Neuro-Evidence: TonalBoundaryLedger-Header {} mal gefunden statt genau einmal",
            headers.len()
        ));
    }
    let mut rows = Vec::new();
    let mut locators = BTreeSet::new();
    for line in lines.iter().skip(headers[0] + 1) {
        if line.starts_with("Decision:") {
            break;
        }
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with("none |") {
            if summary.key_change_candidates != 0 {
                return Err(
                    "Neuro-Evidence: TonalBoundaryLedger meldet none trotz Candidates".to_string(),
                );
            }
            continue;
        }
        let row = parse_tonal_audit_row(line)?;
        let locator = (
            row.boundary_measure,
            row.pre_start_measure,
            row.post_end_measure,
        );
        if !locators.insert(locator) {
            return Err(format!(
                "Neuro-Evidence: doppelter Boundary-Locator {}:{}-{}",
                row.boundary_measure, row.pre_start_measure, row.post_end_measure
            ));
        }
        rows.push(row);
    }
    if rows.len() != summary.key_change_candidates {
        return Err(format!(
            "Neuro-Evidence: DecisionEvidence-Closure verletzt: LedgerRows {} != KeyChangeCandidates {}",
            rows.len(), summary.key_change_candidates
        ));
    }
    let full = rows
        .iter()
        .filter(|row| row.verdict == FULL_EVIDENCE_CANDIDATE)
        .count();
    let promising = rows
        .iter()
        .filter(|row| row.verdict == PROMISING_CANDIDATE)
        .count();
    let held = rows
        .iter()
        .filter(|row| row.verdict == HELD_CANDIDATE)
        .count();
    if full != summary.full_evidence_candidates
        || promising != summary.promising_candidates
        || held != summary.held_candidates
    {
        return Err(format!(
            "Neuro-Evidence: Verdict-Closure verletzt: rows {full}/{promising}/{held} != summary {}/{}/{}",
            summary.full_evidence_candidates,
            summary.promising_candidates,
            summary.held_candidates
        ));
    }
    Ok(rows)
}

fn parse_tonal_audit_row(line: &str) -> Result<TonalAuditRow, String> {
    let fields = line.split(" | ").collect::<Vec<_>>();
    if fields.len() != 14 {
        return Err(format!(
            "Neuro-Evidence: TonalBoundaryLedger-Zeile hat {} statt 14 Felder",
            fields.len()
        ));
    }
    let boundary_measure = parse_usize_field(fields[0], "boundary_measure")?;
    let (pre_start_measure, post_end_measure) = parse_window(fields[1])?;
    let source_fifths = parse_i32_field(fields[2], "source_fifths")?;
    let candidate_fifths = parse_candidate_fifths(fields[3])?;
    let lbdm = parse_i32_field(fields[4], "lbdm")?;
    let bee = parse_i32_field(fields[5], "bee")?;
    let margin = parse_i64_field(fields[6], "margin")?;
    let stable = parse_usize_field(fields[7], "stable")?;
    let bass_cadence = parse_i32_field(fields[8], "bass_cadence")?;
    let (accidental_cost_before, accidental_cost_after) = parse_cost_pair(fields[9])?;
    let hmm = parse_i64_field(fields[10], "hmm")?;
    let green_groups = parse_usize_field(fields[11], "green_groups")?;
    let counters = parse_counterevidence(fields[12])?;
    let verdict = parse_verdict(fields[13])?;
    Ok(TonalAuditRow {
        boundary_measure,
        pre_start_measure,
        post_end_measure,
        source_fifths,
        candidate_fifths,
        lbdm,
        bee,
        margin,
        stable,
        bass_cadence,
        accidental_cost_before,
        accidental_cost_after,
        hmm,
        green_groups,
        counter_boundary_weak: counters.contains("boundary_weak"),
        counter_profile_margin_low: counters.contains("profile_margin_low"),
        counter_short_tonicization_risk: counters.contains("short_tonicization_risk"),
        counter_bass_cadence_unconfirmed: counters.contains("bass_cadence_unconfirmed"),
        counter_notation_saving_small: counters.contains("notation_saving_small"),
        counter_transition_prior_rejects: counters.contains("transition_prior_rejects"),
        counter_none_but_corpus_calibration_open: counters
            .contains("none_but_corpus_calibration_open"),
        verdict,
    })
}

fn parse_usize_field(value: &str, label: &str) -> Result<usize, String> {
    value.trim().parse::<usize>().map_err(|error| {
        format!("Neuro-Evidence: {label} ungueltig {value:?}: {error}")
    })
}

fn parse_i32_field(value: &str, label: &str) -> Result<i32, String> {
    value.trim().parse::<i32>().map_err(|error| {
        format!("Neuro-Evidence: {label} ungueltig {value:?}: {error}")
    })
}

fn parse_i64_field(value: &str, label: &str) -> Result<i64, String> {
    value.trim().parse::<i64>().map_err(|error| {
        format!("Neuro-Evidence: {label} ungueltig {value:?}: {error}")
    })
}

fn parse_window(value: &str) -> Result<(usize, usize), String> {
    let (start, end) = value
        .trim()
        .split_once('-')
        .ok_or_else(|| format!("Neuro-Evidence: Window ungueltig {value:?}"))?;
    let start = parse_usize_field(start, "window_start")?;
    let end = parse_usize_field(end, "window_end")?;
    if start > end {
        return Err(format!("Neuro-Evidence: Window invertiert {start}-{end}"));
    }
    Ok((start, end))
}

fn parse_candidate_fifths(value: &str) -> Result<i32, String> {
    let (_, fifths) = value
        .trim()
        .rsplit_once('(')
        .ok_or_else(|| format!("Neuro-Evidence: CandidateFifths fehlen {value:?}"))?;
    let fifths = fifths
        .strip_suffix(')')
        .ok_or_else(|| format!("Neuro-Evidence: CandidateFifths-Klammer fehlt {value:?}"))?;
    parse_i32_field(fifths, "candidate_fifths")
}

fn parse_cost_pair(value: &str) -> Result<(i64, i64), String> {
    let (before, after) = value
        .trim()
        .split_once('/')
        .ok_or_else(|| format!("Neuro-Evidence: AccidentalCost-Paar ungueltig {value:?}"))?;
    Ok((
        parse_i64_field(before, "accidental_cost_before")?,
        parse_i64_field(after, "accidental_cost_after")?,
    ))
}

fn parse_counterevidence(value: &str) -> Result<BTreeSet<&str>, String> {
    const ALLOWED: [&str; 7] = [
        "boundary_weak",
        "profile_margin_low",
        "short_tonicization_risk",
        "bass_cadence_unconfirmed",
        "notation_saving_small",
        "transition_prior_rejects",
        "none_but_corpus_calibration_open",
    ];
    let parts = value
        .trim()
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if parts.is_empty() || parts.iter().any(|part| !ALLOWED.contains(part)) {
        return Err(format!(
            "Neuro-Evidence: unbekannte/fehlende Counterevidence {value:?}"
        ));
    }
    let set = parts.iter().copied().collect::<BTreeSet<_>>();
    if set.len() != parts.len() {
        return Err(format!("Neuro-Evidence: doppelte Counterevidence {value:?}"));
    }
    if set.contains("none_but_corpus_calibration_open") && set.len() != 1 {
        return Err(
            "Neuro-Evidence: none_but_corpus_calibration_open darf nicht kombiniert werden"
                .to_string(),
        );
    }
    Ok(set)
}

fn parse_verdict(value: &str) -> Result<&'static str, String> {
    match value.trim() {
        PROMISING_CANDIDATE => Ok(PROMISING_CANDIDATE),
        HELD_CANDIDATE => Ok(HELD_CANDIDATE),
        FULL_EVIDENCE_CANDIDATE => Ok(FULL_EVIDENCE_CANDIDATE),
        other => Err(format!("Neuro-Evidence: unbekannter Audit-Verdict {other:?}")),
    }
}

fn decisions_jsonl(
    rows: &[TonalAuditRow],
    project_id: &str,
    producer_build: &str,
    source_identity: &str,
    corpus_run_id: &str,
) -> Result<String, String> {
    let mut payload = String::new();
    for row in rows {
        payload.push_str(&decision_json(
            row,
            project_id,
            producer_build,
            source_identity,
            corpus_run_id,
        )?);
        payload.push('\n');
    }
    Ok(payload)
}

fn decision_json(
    row: &TonalAuditRow,
    project_id: &str,
    producer_build: &str,
    source_identity: &str,
    corpus_run_id: &str,
) -> Result<String, String> {
    let stable = i64::try_from(row.stable)
        .map_err(|_| "Neuro-Evidence: stable ausserhalb i64".to_string())?;
    let green_groups = i64::try_from(row.green_groups)
        .map_err(|_| "Neuro-Evidence: green_groups ausserhalb i64".to_string())?;
    let observations = [
        number_observation("source_fifths", i64::from(row.source_fifths)),
        number_observation("candidate_fifths", i64::from(row.candidate_fifths)),
        number_observation("lbdm", i64::from(row.lbdm)),
        number_observation("bee", i64::from(row.bee)),
        number_observation("margin", row.margin),
        number_observation("stable", stable),
        number_observation("bass_cadence", i64::from(row.bass_cadence)),
        number_observation("accidental_cost_before", row.accidental_cost_before),
        number_observation("accidental_cost_after", row.accidental_cost_after),
        number_observation("hmm", row.hmm),
        number_observation("green_groups", green_groups),
        bool_observation("counter_boundary_weak", row.counter_boundary_weak),
        bool_observation("counter_profile_margin_low", row.counter_profile_margin_low),
        bool_observation(
            "counter_short_tonicization_risk",
            row.counter_short_tonicization_risk,
        ),
        bool_observation(
            "counter_bass_cadence_unconfirmed",
            row.counter_bass_cadence_unconfirmed,
        ),
        bool_observation(
            "counter_notation_saving_small",
            row.counter_notation_saving_small,
        ),
        bool_observation(
            "counter_transition_prior_rejects",
            row.counter_transition_prior_rejects,
        ),
        bool_observation(
            "counter_none_but_corpus_calibration_open",
            row.counter_none_but_corpus_calibration_open,
        ),
        text_observation("source_audit_verdict", "post_decision", row.verdict),
    ]
    .join(",");
    let evidence_id = stable_evidence_id(project_id, row);
    let locator = format!(
        "boundary_measure={};window={}-{}",
        row.boundary_measure, row.pre_start_measure, row.post_end_measure
    );
    let legal_candidates = [
        legal_candidate_json(PROMISING_CANDIDATE, true, false, false),
        legal_candidate_json(HELD_CANDIDATE, false, true, false),
        legal_candidate_json(FULL_EVIDENCE_CANDIDATE, false, false, true),
    ]
    .join(",");
    Ok(format!(
        concat!(
            "{{\"schema_version\":2,",
            "\"evidence_id\":\"{}\",",
            "\"contract\":{{\"owner\":\"{}\",\"decision_class\":\"{}\",",
            "\"epistemic_kind\":\"unresolved\",\"contract_fingerprint\":\"{}\"}},",
            "\"decision_observations\":{{{}}},",
            "\"legal_candidates\":[{}],",
            "\"label\":null,\"proof\":null,",
            "\"provenance\":{{\"etuschlopf_version\":\"{}\",",
            "\"etuschlopf_source_identity\":\"{}\",",
            "\"corpus_run_id\":\"{}\",\"project_id\":\"{}\",",
            "\"locator\":\"{}\"}}}}"
        ),
        json_escape(&evidence_id),
        OWNER,
        DECISION_CLASS,
        DECISION_CONTRACT_FINGERPRINT,
        observations,
        legal_candidates,
        json_escape(producer_build),
        json_escape(source_identity),
        json_escape(corpus_run_id),
        json_escape(project_id),
        json_escape(&locator),
    ))
}

fn stable_evidence_id(project_id: &str, row: &TonalAuditRow) -> String {
    let canonical = format!(
        "decision_class={DECISION_CLASS}\nproject_id={project_id}\nboundary_measure={}\nwindow={}-{}\n",
        row.boundary_measure, row.pre_start_measure, row.post_end_measure
    );
    format!("sha256:{}", sha256::hex(canonical.as_bytes()))
}

fn number_observation(name: &str, value: i64) -> String {
    format!(
        "\"{}\":{{\"phase\":\"pre_decision\",\"value\":{{\"kind\":\"number\",\"value\":{}.0}}}}",
        json_escape(name),
        value
    )
}

fn bool_observation(name: &str, value: bool) -> String {
    format!(
        "\"{}\":{{\"phase\":\"pre_decision\",\"value\":{{\"kind\":\"bool\",\"value\":{}}}}}",
        json_escape(name),
        if value { "true" } else { "false" }
    )
}

fn text_observation(name: &str, phase: &str, value: &str) -> String {
    format!(
        "\"{}\":{{\"phase\":\"{}\",\"value\":{{\"kind\":\"text\",\"value\":\"{}\"}}}}",
        json_escape(name),
        json_escape(phase),
        json_escape(value)
    )
}

fn legal_candidate_json(id: &str, promising: bool, held: bool, full_evidence: bool) -> String {
    format!(
        "{{\"id\":\"{}\",\"observations\":{{{},{},{}}}}}",
        json_escape(id),
        bool_observation("choice_promising", promising),
        bool_observation("choice_held", held),
        bool_observation("choice_full_evidence", full_evidence),
    )
}

fn json_escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch <= '\u{1f}' => {
                let _ = write!(&mut out, "\\u{:04x}", ch as u32);
            }
            ch => out.push(ch),
        }
    }
    out
}

fn manifest_json(
    project_id: &str,
    provenance: EvidenceProvenance<'_>,
    qualified_decision_count: usize,
    payload_bytes: usize,
    payload_sha256: &str,
) -> String {
    format!(
        concat!(
            "{{\n",
            "  \"artifact-kind\": \"etuschlopf_neuro_evidence\",\n",
            "  \"schema-version\": 2,\n",
            "  \"producer\": \"EtuSchlopf\",\n",
            "  \"producer-build\": \"{}\",\n",
            "  \"created-at-utc\": \"{}\",\n",
            "  \"project-id\": \"{}\",\n",
            "  \"corpus-run-id\": \"{}\",\n",
            "  \"etuschlopf-source-identity\": \"{}\",\n",
            "  \"corpus-id\": \"etuschlopf-tonal-audit\",\n",
            "  \"decision-class\": \"tonal_segmentation_boundary_audit_v1\",\n",
            "  \"decision-contract-fingerprint\": \"fnv64:4f2c765d00c527e6\",\n",
            "  \"decision-population\": {{\n",
            "    \"scope\": \"qualified_key_change_candidates_v1\",\n",
            "    \"qualified-decision-count\": {}\n",
            "  }},\n",
            "  \"payload-manifest\": [\n",
            "    {{\n",
            "      \"path\": \"decisions.jsonl\",\n",
            "      \"bytes\": {},\n",
            "      \"sha-256\": \"{}\"\n",
            "    }}\n",
            "  ]\n",
            "}}\n"
        ),
        json_escape(provenance.producer_build),
        json_escape(provenance.created_at_utc),
        json_escape(project_id),
        json_escape(provenance.corpus_run_id),
        json_escape(provenance.source_identity),
        qualified_decision_count,
        payload_bytes,
        payload_sha256,
    )
}

fn validate_decisions_payload(
    payload: &str,
    expected_rows: usize,
    project_id: &str,
    producer_build: &str,
    source_identity: &str,
    corpus_run_id: &str,
) -> Result<(), String> {
    if payload.len() > PAYLOAD_MAX_BYTES {
        return Err("Neuro-Evidence: decisions.jsonl Groesse ausserhalb Vertrag".to_string());
    }
    if expected_rows == 0 {
        if !payload.is_empty() {
            return Err(
                "Neuro-Evidence: Zero-Case muss exakt 0 Bytes decisions.jsonl besitzen".to_string(),
            );
        }
        return Ok(());
    }
    if payload.is_empty() {
        return Err(
            "Neuro-Evidence: nichtleere Population besitzt leere decisions.jsonl".to_string(),
        );
    }
    let lower = payload.to_ascii_lowercase();
    for forbidden in [
        "assistant_export",
        "assistant_context",
        "handover",
        "uebergabe",
        "readme",
        "diagnosefreitext",
        "human hypothesis",
        "ground_truth",
        "learningrow",
        "prediction",
        "humanpreference",
    ] {
        if lower.contains(forbidden) {
            return Err(format!(
                "Neuro-Evidence: Trust-Domain-Verletzung in decisions.jsonl: {forbidden}"
            ));
        }
    }
    let lines = payload.lines().collect::<Vec<_>>();
    if lines.len() != expected_rows {
        return Err(format!(
            "Neuro-Evidence: decisions.jsonl Zeilen {} != erwartete DecisionEvidence {}",
            lines.len(), expected_rows
        ));
    }
    let escaped_project = json_escape(project_id);
    let escaped_build = json_escape(producer_build);
    let escaped_source = json_escape(source_identity);
    let escaped_run = json_escape(corpus_run_id);
    for line in lines {
        for required in [
            "{\"schema_version\":2,".to_string(),
            "\"owner\":\"TonalSegmentationEvidenceAudit\"".to_string(),
            "\"decision_class\":\"tonal_segmentation_boundary_audit_v1\"".to_string(),
            "\"epistemic_kind\":\"unresolved\"".to_string(),
            "\"contract_fingerprint\":\"fnv64:4f2c765d00c527e6\"".to_string(),
            "\"source_audit_verdict\":{\"phase\":\"post_decision\"".to_string(),
            DECISION_NULL_FIELDS_JSON_FRAGMENT.to_string(),
            PROMISING_CANDIDATE.to_string(),
            HELD_CANDIDATE.to_string(),
            FULL_EVIDENCE_CANDIDATE.to_string(),
            format!("\"project_id\":\"{escaped_project}\""),
            format!("\"corpus_run_id\":\"{escaped_run}\""),
            format!("\"etuschlopf_source_identity\":\"{escaped_source}\""),
            format!("\"etuschlopf_version\":\"{escaped_build}\""),
        ] {
            if !line.contains(&required) {
                return Err(format!(
                    "Neuro-Evidence: DecisionEvidence-Pflicht fehlt: {required}"
                ));
            }
        }
        for retired_summary_field in [
            "\"boundaries-checked\"",
            "\"key-change-candidates\"",
            "\"full-evidence-candidates\"",
            "\"promising-candidates\"",
            "\"held-candidates\"",
        ] {
            if line.contains(retired_summary_field) {
                return Err(format!(
                    "Neuro-Evidence: aggregiertes Summary-Feld verboten: {retired_summary_field}"
                ));
            }
        }
    }
    Ok(())
}

fn validate_manifest_bytes(
    manifest: &str,
    project_id: &str,
    producer_build: &str,
    source_identity: &str,
    corpus_run_id: &str,
    qualified_decision_count: usize,
) -> Result<(), String> {
    if manifest.is_empty() || manifest.len() > MANIFEST_MAX_BYTES {
        return Err("Neuro-Evidence: manifest.json Groesse ausserhalb Vertrag".to_string());
    }
    for required in [
        format!("\"artifact-kind\": \"{ARTIFACT_KIND}\""),
        format!("\"schema-version\": {SCHEMA_VERSION}"),
        format!("\"producer\": \"{PRODUCER}\""),
        format!("\"producer-build\": \"{}\"", json_escape(producer_build)),
        format!("\"project-id\": \"{}\"", json_escape(project_id)),
        format!("\"corpus-run-id\": \"{}\"", json_escape(corpus_run_id)),
        format!(
            "\"etuschlopf-source-identity\": \"{}\"",
            json_escape(source_identity)
        ),
        format!("\"corpus-id\": \"{CORPUS_ID}\""),
        format!("\"decision-class\": \"{DECISION_CLASS}\""),
        format!(
            "\"decision-contract-fingerprint\": \"{DECISION_CONTRACT_FINGERPRINT}\""
        ),
        format!("\"scope\": \"{POPULATION_SCOPE}\""),
        format!(
            "\"qualified-decision-count\": {qualified_decision_count}"
        ),
        format!("\"path\": \"{DECISIONS_PAYLOAD}\""),
    ] {
        if !manifest.contains(&required) {
            return Err(format!("Neuro-Evidence: Manifestpflicht fehlt: {required}"));
        }
    }
    if manifest.matches("\"project-id\":").count() != 1
        || manifest.matches("\"corpus-run-id\":").count() != 1
        || manifest
            .matches("\"etuschlopf-source-identity\":")
            .count()
            != 1
        || manifest
            .matches("\"qualified-decision-count\":")
            .count()
            != 1
    {
        return Err(
            "Neuro-Evidence: Manifest-Identitaet/Population ist mehrdeutig".to_string(),
        );
    }
    Ok(())
}

fn write_closed_archive(path: &Path, manifest: &str, decisions: &[u8]) -> Result<(), String> {
    let file = File::create(path)
        .map_err(|error| format!("Neuro-Evidence: temporaeres ZIP nicht anlegbar: {error}"))?;
    let mut zip = zip::ZipWriter::new(file);
    zip.start_file(
        MANIFEST,
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated),
    )
        .map_err(|error| format!("Neuro-Evidence: manifest ZIP-Eintrag fehlgeschlagen: {error}"))?;
    zip.write_all(manifest.as_bytes())
        .map_err(|error| format!("Neuro-Evidence: manifest ZIP-Write fehlgeschlagen: {error}"))?;
    zip.start_file(
        DECISIONS_PAYLOAD,
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated),
    )
        .map_err(|error| format!("Neuro-Evidence: decisions ZIP-Eintrag fehlgeschlagen: {error}"))?;
    zip.write_all(decisions)
        .map_err(|error| format!("Neuro-Evidence: decisions ZIP-Write fehlgeschlagen: {error}"))?;
    zip.finish()
        .map_err(|error| format!("Neuro-Evidence: ZIP-Finalisierung fehlgeschlagen: {error}"))?;
    Ok(())
}

fn validate_closed_archive(
    path: &Path,
    expected_manifest: &[u8],
    expected_decisions: &[u8],
    expected_sha256: &str,
) -> Result<(), String> {
    let file = File::open(path)
        .map_err(|error| format!("Neuro-Evidence: ZIP-Readback nicht moeglich: {error}"))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|error| format!("Neuro-Evidence: ZIP-Readback ungueltig: {error}"))?;
    if archive.len() != 2 {
        return Err(format!(
            "Neuro-Evidence: Manifest Closure verletzt, ZIP enthaelt {} statt 2 Dateien",
            archive.len()
        ));
    }
    let mut names = Vec::with_capacity(2);
    for index in 0..archive.len() {
        let entry = archive
            .by_index(index)
            .map_err(|error| format!("Neuro-Evidence: ZIP-Eintrag {index} unlesbar: {error}"))?;
        if entry.is_dir() || entry.name().contains('/') || entry.name().contains('\\') {
            return Err(format!(
                "Neuro-Evidence: Unterordner/Directory-Eintrag verboten: {}",
                entry.name()
            ));
        }
        names.push(entry.name().to_string());
    }
    names.sort();
    if names != [DECISIONS_PAYLOAD.to_string(), MANIFEST.to_string()] {
        return Err(format!(
            "Neuro-Evidence: positive ZIP-Allowlist verletzt: {names:?}"
        ));
    }
    let mut manifest = Vec::new();
    archive
        .by_name(MANIFEST)
        .map_err(|error| format!("Neuro-Evidence: manifest.json fehlt: {error}"))?
        .read_to_end(&mut manifest)
        .map_err(|error| {
            format!("Neuro-Evidence: manifest.json Readback fehlgeschlagen: {error}")
        })?;
    if manifest != expected_manifest {
        return Err(
            "Neuro-Evidence: manifest.json Readback weicht von geschriebenen Bytes ab"
                .to_string(),
        );
    }
    let mut decisions = Vec::new();
    archive
        .by_name(DECISIONS_PAYLOAD)
        .map_err(|error| format!("Neuro-Evidence: decisions.jsonl fehlt: {error}"))?
        .read_to_end(&mut decisions)
        .map_err(|error| {
            format!("Neuro-Evidence: decisions.jsonl Readback fehlgeschlagen: {error}")
        })?;
    if decisions != expected_decisions {
        return Err(
            "Neuro-Evidence: decisions.jsonl Readback weicht von geschriebenen Bytes ab"
                .to_string(),
        );
    }
    if sha256::hex(&decisions) != expected_sha256 {
        return Err("Neuro-Evidence: SHA-256 Readback inkonsistent".to_string());
    }
    Ok(())
}

fn read_take_revision(project_root: &Path) -> Result<String, String> {
    let text = fs::read_to_string(project_root.join("ZIP_KIND.txt"))
        .map_err(|error| format!("Neuro-Evidence: ZIP_KIND.txt nicht lesbar: {error}"))?;
    let revision = text
        .lines()
        .find_map(|line| line.strip_prefix("revision="))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "Neuro-Evidence: revision fehlt in ZIP_KIND.txt".to_string())?;
    if revision.contains(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')) {
        return Err("Neuro-Evidence: revision enthaelt unzulaessige Zeichen".to_string());
    }
    Ok(revision.to_string())
}

fn utc_stamp(now: SystemTime) -> Result<UtcStamp, String> {
    let duration = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|error| format!("Neuro-Evidence: Uhr vor Unix-Epoch: {error}"))?;
    let total_seconds = duration.as_secs();
    let days = i64::try_from(total_seconds / 86_400)
        .map_err(|_| "Neuro-Evidence: UTC-Datum ausserhalb Bereich".to_string())?;
    let seconds_of_day = total_seconds % 86_400;
    let hour = seconds_of_day / 3_600;
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;
    let (year, month, day) = utc_civil_from_unix_days(days)?;
    let nanos = duration.subsec_nanos();
    Ok(UtcStamp {
        filename: format!(
            "{year:04}{month:02}{day:02}T{hour:02}{minute:02}{second:02}_{nanos:09}Z"
        ),
        rfc3339: format!(
            "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{nanos:09}Z"
        ),
    })
}

fn utc_civil_from_unix_days(days: i64) -> Result<(i64, u32, u32), String> {
    let shifted = days + 719_468;
    let era = if shifted >= 0 {
        shifted
    } else {
        shifted - 146_096
    } / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let provisional_year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    let year = provisional_year + if month <= 2 { 1 } else { 0 };
    let month = u32::try_from(month)
        .map_err(|_| "Neuro-Evidence: UTC-Monat ausserhalb Bereich".to_string())?;
    let day = u32::try_from(day)
        .map_err(|_| "Neuro-Evidence: UTC-Tag ausserhalb Bereich".to_string())?;
    Ok((year, month, day))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_REPORT: &str = concat!(
        "Kurz: TonalSegmentationEvidenceAudit: BoundariesChecked 43; ",
        "KeyChangeCandidates 2; FullEvidenceCandidates 0; PromisingCandidates 1; ",
        "HeldCandidates 1; WriterOpen nein; RenderXmlWrite nein; LegacyMusicMutation nein; ",
        "MasterApply nein.\n",
        "TonalBoundaryLedger: BoundaryM | Window | SourceFifths | Candidate | LBDM | Bee | ",
        "Margin | Stable | BassCadence | AccidentalCostBeforeAfter | HMM | GreenGroups | ",
        "Counterevidence | Verdict\n",
        "31 | 27-34 | 2 | H-Dur/B-major(5) | 2 | 2 | 104 | 4 | 4 | 21/2 | 68 | 5 | notation_saving_small | promising_candidate_more_corpus_calibration\n",
        "30 | 26-33 | 2 | H-Dur/B-major(5) | 1 | 1 | 82 | 0 | 4 | 17/0 | 46 | 3 | boundary_weak,short_tonicization_risk,notation_saving_small | held_as_tonicization_or_chromatic_colour\n",
        "Decision: Audit-only.\n"
    );

    fn sample_rows() -> Vec<TonalAuditRow> {
        let summary = parse_tonal_audit_summary(SAMPLE_REPORT).expect("summary");
        parse_tonal_audit_rows(SAMPLE_REPORT, summary).expect("rows")
    }

    fn sample_provenance() -> EvidenceProvenance<'static> {
        EvidenceProvenance {
            producer_build: "0.10.1821-r02",
            source_identity: "v0.10.1821:0123456789abcdef",
            corpus_run_id: "obs-0.10.1821-1-2-0",
            created_at_utc: "2026-08-15T19:30:00Z",
        }
    }

    #[test]
    fn normal_publication_is_isolated_between_project_roots() {
        let root = test_zip_path("project_roots").with_extension("");
        let first = root.join("Project Alpha");
        let second = root.join("Project Beta");
        let mut published = Vec::new();
        for project_root in [&first, &second] {
            fs::create_dir_all(project_root).expect("project root");
            fs::write(project_root.join("ZIP_KIND.txt"), "revision=r9000\n")
                .expect("revision");
            let report = project_root.join("Synthetic Project.result.txt");
            fs::write(&report, SAMPLE_REPORT).expect("synthetic report");
            let path = publish_neuro_evidence_for_successful_normal_run(
                project_root,
                &report,
                "Synthetic Project",
                "0.10.1839",
                "synthetic-local-run",
                0,
            )
            .expect("publish")
            .expect("successful run has evidence");
            assert_eq!(path.parent(), Some(project_root.join("target/etuschlopf_neuro_evidence").as_path()));
            assert_eq!(read_retention_evidence_identity(&path).expect("closed evidence").project_id,
                "Synthetic Project");
            published.push(path);
        }
        fs::remove_dir_all(&first).expect("remove first project");
        assert!(published[1].is_file(), "the other project's evidence remains independent");
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn corpus_publication_uses_its_project_root_and_preserves_foreign_files() {
        let root = test_zip_path("local_corpus").with_extension("");
        let project_root = root.join("Project Alpha");
        let stage = project_root.join("target/staging");
        fs::create_dir_all(&stage).expect("staging");
        let report = stage.join("synthetic.result.txt");
        fs::write(&report, SAMPLE_REPORT).expect("synthetic report");
        let stamp = "20260815T193000_000000000Z";
        let name = format!("etuschlopf_neuro_evidence_{stamp}_01.zip");
        let archive = prepare_project_archive(
            &report,
            "Synthetic Project",
            sample_provenance(),
            name.clone(),
            stage.join(&name),
        ).expect("prepare closed archive");
        let commit = corpus_commit_json(
            std::slice::from_ref(&archive),
            "0.10.1821-r02",
            "v0.10.1821:0123456789abcdef",
            "synthetic-local-run",
            "2026-08-15T19:30:00Z",
        );
        let staged_commit = stage.join(format!("etuschlopf_neuro_corpus_commit_{stamp}.json"));
        fs::write(&staged_commit, &commit).expect("staged commit");
        let output = project_root.join("target/etuschlopf_neuro_evidence");
        fs::create_dir_all(&output).expect("output");
        let foreign = output.join("user-notes.txt");
        fs::write(&foreign, "keep").expect("foreign file");
        let published = publish_prepared_corpus(&project_root, &[archive], &staged_commit)
            .expect("publish corpus artifacts");
        assert_eq!(published.parent(), Some(output.as_path()));
        assert_eq!(fs::read_to_string(&published).expect("commit readback"), commit);
        assert_eq!(read_retention_evidence_identity(&output.join(name)).expect("closed evidence").project_id,
            "Synthetic Project");
        assert_eq!(fs::read_to_string(foreign).expect("foreign file remains"), "keep");
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn payload_is_machine_only_and_nonempty() {
        let rows = sample_rows();
        let payload = decisions_jsonl(
            &rows,
            "Synthetic Project Alpha",
            "0.10.1821-r02",
            "v0.10.1821:0123456789abcdef",
            "obs-0.10.1821-1-2-0",
        )
        .expect("payload");
        validate_decisions_payload(
            &payload,
            2,
            "Synthetic Project Alpha",
            "0.10.1821-r02",
            "v0.10.1821:0123456789abcdef",
            "obs-0.10.1821-1-2-0",
        )
        .expect("machine-only payload");
        assert_eq!(payload.lines().count(), 2);
        assert!(payload.contains("\"source_fifths\":{\"phase\":\"pre_decision\""));
        assert!(payload.contains("\"candidate_fifths\":{\"phase\":\"pre_decision\""));
        assert!(payload.contains("\"source_audit_verdict\":{\"phase\":\"post_decision\""));
        assert!(payload.contains("\"label\":null,\"proof\":null"));
        assert!(!payload.contains("boundaries-checked"));
        assert!(!payload.contains("assistant"));
    }

    #[test]
    fn legal_candidates_are_exact_and_one_hot() {
        let rows = sample_rows();
        let line = decision_json(
            &rows[0],
            "Synthetic Project Alpha",
            "0.10.1821-r02",
            "v0.10.1821:0123456789abcdef",
            "obs-0.10.1821-1-2-0",
        )
        .expect("decision");
        assert_eq!(line.matches("\"id\":").count(), 3);
        assert_eq!(line.matches(PROMISING_CANDIDATE).count(), 2);
        assert_eq!(line.matches(HELD_CANDIDATE).count(), 1);
        assert_eq!(line.matches(FULL_EVIDENCE_CANDIDATE).count(), 1);
        assert!(line.contains(
            "\"choice_promising\":{\"phase\":\"pre_decision\",\"value\":{\"kind\":\"bool\",\"value\":true}}"
        ));
    }

    #[test]
    fn evidence_id_is_stable_across_run_provenance() {
        let rows = sample_rows();
        let left = stable_evidence_id("Synthetic Project Alpha", &rows[0]);
        let right = stable_evidence_id("Synthetic Project Alpha", &rows[0]);
        assert_eq!(left, right);
        assert!(left.starts_with("sha256:"));
        assert_ne!(left, stable_evidence_id("Synthetic Project Beta", &rows[0]));
    }

    #[test]
    fn report_summary_parser_requires_machine_tonal_summary() {
        let summary = parse_tonal_audit_summary(SAMPLE_REPORT).expect("summary");
        let rows = parse_tonal_audit_rows(SAMPLE_REPORT, summary).expect("rows");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].source_fifths, 2);
        assert_eq!(rows[0].candidate_fifths, 5);
        assert_eq!(rows[0].margin, 104);
        assert!(rows[0].counter_notation_saving_small);
        assert!(!rows[0].counter_boundary_weak);
        assert!(rows[1].counter_boundary_weak);
        assert!(rows[1].counter_short_tonicization_risk);
        assert!(parse_tonal_audit_summary("no tonal evidence").is_err());
        let truncated = SAMPLE_REPORT.replace(
            "30 | 26-33 | 2 | H-Dur/B-major(5) | 1 | 1 | 82 | 0 | 4 | 17/0 | 46 | 3 | boundary_weak,short_tonicization_risk,notation_saving_small | held_as_tonicization_or_chromatic_colour\n",
            "",
        );
        let summary = parse_tonal_audit_summary(&truncated).expect("summary");
        assert!(parse_tonal_audit_rows(&truncated, summary).is_err());
    }

    #[test]
    fn manifest_contract_is_closed_to_one_payload() {
        let payload = "{\"schema_version\":2}\n";
        let hash = sha256::hex(payload.as_bytes());
        let manifest = manifest_json(
            "Synthetic Project Alpha",
            sample_provenance(),
            2,
            payload.len(),
            &hash,
        );
        validate_manifest_bytes(
            &manifest,
            "Synthetic Project Alpha",
            "0.10.1821-r02",
            "v0.10.1821:0123456789abcdef",
            "obs-0.10.1821-1-2-0",
            2,
        )
        .expect("manifest contract");
        assert_eq!(manifest.matches("\"path\":").count(), 1);
        assert_eq!(manifest.matches("decisions.jsonl").count(), 1);
        assert!(!manifest.contains("features.jsonl"));
        assert!(!manifest.contains("outcomes.jsonl"));
    }

    #[test]
    fn archive_readback_proves_exact_two_file_manifest_closure() {
        let rows = sample_rows();
        let payload = decisions_jsonl(
            &rows,
            "Synthetic Project Alpha",
            "0.10.1821-r02",
            "v0.10.1821:0123456789abcdef",
            "obs-0.10.1821-1-2-0",
        )
        .expect("payload");
        let hash = sha256::hex(payload.as_bytes());
        let manifest = manifest_json(
            "Synthetic Project Alpha",
            sample_provenance(),
            2,
            payload.len(),
            &hash,
        );
        let path = test_zip_path("closed");
        write_closed_archive(&path, &manifest, payload.as_bytes()).expect("write");
        validate_closed_archive(&path, manifest.as_bytes(), payload.as_bytes(), &hash)
            .expect("closed archive");
        let _ = fs::remove_file(path);
    }

    #[test]
    fn archive_readback_rejects_unmanifested_extra_file() {
        let rows = sample_rows();
        let payload = decisions_jsonl(
            &rows,
            "Synthetic Project Alpha",
            "0.10.1821-r02",
            "v0.10.1821:0123456789abcdef",
            "obs-0.10.1821-1-2-0",
        )
        .expect("payload");
        let hash = sha256::hex(payload.as_bytes());
        let manifest = manifest_json(
            "Synthetic Project Alpha",
            sample_provenance(),
            2,
            payload.len(),
            &hash,
        );
        let path = test_zip_path("extra");
        {
            let file = File::create(&path).expect("file");
            let mut zip = zip::ZipWriter::new(file);
            for (name, bytes) in [
                (MANIFEST, manifest.as_bytes()),
                (DECISIONS_PAYLOAD, payload.as_bytes()),
                ("notes.txt", b"forbidden".as_slice()),
            ] {
                zip.start_file(
                    name,
                    FileOptions::default()
                        .compression_method(zip::CompressionMethod::Deflated),
                )
                .expect("entry");
                zip.write_all(bytes).expect("bytes");
            }
            zip.finish().expect("finish");
        }
        assert!(
            validate_closed_archive(&path, manifest.as_bytes(), payload.as_bytes(), &hash)
                .is_err()
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn zero_case_is_attested_with_exact_empty_payload() {
        let payload = decisions_jsonl(
            &[],
            "Synthetic Project Alpha",
            "0.10.1839-r425",
            "v0.10.1839:0123456789abcdef",
            "corpus-test",
        )
        .expect("empty payload");
        assert!(payload.is_empty());
        validate_decisions_payload(
            &payload,
            0,
            "Synthetic Project Alpha",
            "0.10.1839-r425",
            "v0.10.1839:0123456789abcdef",
            "corpus-test",
        )
        .expect("zero-case closure");
        assert_eq!(sha256::hex(payload.as_bytes()), EMPTY_PAYLOAD_SHA256);
    }

    #[test]
    fn evidence_id_does_not_depend_on_corpus_run_provenance() {
        let row = &sample_rows()[0];
        let first = decision_json(
            row,
            "Synthetic Project Alpha",
            "0.10.1839-r425",
            "source-a",
            "corpus-a",
        )
        .expect("first");
        let second = decision_json(
            row,
            "Synthetic Project Alpha",
            "0.10.1839-r999",
            "source-b",
            "corpus-b",
        )
        .expect("second");
        let expected = stable_evidence_id("Synthetic Project Alpha", row);
        assert!(first.contains(&format!("\"evidence_id\":\"{expected}\"")));
        assert!(second.contains(&format!("\"evidence_id\":\"{expected}\"")));
    }

    #[test]
    fn corpus_commit_binds_exact_ten_project_archives() {
        let prepared = REQUIRED_CORPUS_PROJECTS
            .iter()
            .enumerate()
            .map(|(index, project_id)| PreparedCorpusArchive {
                project_id: (*project_id).to_string(),
                file_name: format!("etuschlopf_neuro_evidence_test_{:02}.zip", index + 1),
                staged_path: PathBuf::from(format!("staged_{:02}.zip", index + 1)),
                archive_sha256: format!("{:064x}", index + 1),
                qualified_decision_count: index,
                manifest: String::new(),
            })
            .collect::<Vec<_>>();
        let commit = corpus_commit_json(
            &prepared,
            "0.10.1839-r425",
            "v0.10.1839:0123456789abcdef",
            "corpus-test",
            "2026-08-29T18:00:00.000000000Z",
        );
        validate_corpus_commit_bytes(
            &commit,
            &prepared,
            "0.10.1839-r425",
            "v0.10.1839:0123456789abcdef",
            "corpus-test",
        )
        .expect("commit closure");
        assert_eq!(commit.matches("\"project-id\":").count(), 10);
        assert_eq!(commit.matches("\"archive-sha-256\":").count(), 10);
        assert!(commit.contains("\"population-scope\": \"qualified_key_change_candidates_v1\""));
    }


    #[test]
    fn retention_plan_is_bounded_and_manifest_identity_driven() {
        let root = PathBuf::from("transfer");
        let old = test_retention_cohort(&root, "a", "2026-08-28T12:00:00Z");
        let previous = test_retention_cohort(&root, "b", "2026-08-29T12:00:00Z");
        let newest = test_retention_cohort(&root, "c", "2026-08-30T12:00:00Z");
        let standalone_old = RetentionStandalone {
            path: root.join("etuschlopf_neuro_evidence_alpha.zip"),
            project_id: "Synthetic Project Alpha".to_string(),
            created_at_utc: "2026-08-29T13:00:00Z".to_string(),
        };
        let standalone_new = RetentionStandalone {
            path: root.join("etuschlopf_neuro_evidence_beta.zip"),
            project_id: "Synthetic Project Alpha".to_string(),
            created_at_utc: "2026-08-30T13:00:00Z".to_string(),
        };
        let mut managed = Vec::new();
        for cohort in [&old, &previous, &newest] {
            managed.push(cohort.commit_path.clone());
            managed.extend(
                cohort
                    .members
                    .iter()
                    .map(|member| root.join(&member.file_name)),
            );
        }
        managed.push(standalone_old.path.clone());
        managed.push(standalone_new.path.clone());
        let plan = select_transfer_retention(
            managed,
            vec![old.clone(), previous.clone(), newest.clone()],
            vec![standalone_old.clone(), standalone_new.clone()],
            &BTreeSet::new(),
        );
        assert_eq!(plan.cohort_count, 2);
        assert_eq!(plan.standalone_count, 1);
        assert!(plan.stale.contains(&old.commit_path));
        assert!(plan.stale.contains(&standalone_old.path));
        assert!(!plan.stale.contains(&previous.commit_path));
        assert!(!plan.stale.contains(&newest.commit_path));
        assert!(!plan.stale.contains(&standalone_new.path));
    }

    #[test]
    fn retention_legacy_cleanup_waits_for_two_valid_cohorts() {
        let root = PathBuf::from("transfer");
        let cohort = test_retention_cohort(&root, "a", "2026-08-30T12:00:00Z");
        let legacy = root.join("etuschlopf_neuro_evidence_legacy_v1.zip");
        let mut managed = vec![cohort.commit_path.clone(), legacy.clone()];
        managed.extend(
            cohort
                .members
                .iter()
                .map(|member| root.join(&member.file_name)),
        );
        let plan = select_transfer_retention(
            managed,
            vec![cohort],
            Vec::new(),
            &BTreeSet::new(),
        );
        assert!(!plan.stale.contains(&legacy));
    }

    #[test]
    fn retention_never_deletes_current_successful_publication() {
        let root = PathBuf::from("transfer");
        let first = test_retention_cohort(&root, "a", "2026-08-29T12:00:00Z");
        let second = test_retention_cohort(&root, "b", "2026-08-30T12:00:00Z");
        let current = root.join("etuschlopf_neuro_evidence_current.zip");
        let mut managed = vec![
            first.commit_path.clone(),
            second.commit_path.clone(),
            current.clone(),
        ];
        for cohort in [&first, &second] {
            managed.extend(
                cohort
                    .members
                    .iter()
                    .map(|member| root.join(&member.file_name)),
            );
        }
        let protected = BTreeSet::from([current.clone()]);
        let plan = select_transfer_retention(
            managed,
            vec![first, second],
            Vec::new(),
            &protected,
        );
        assert!(!plan.stale.contains(&current));
    }

    #[test]
    fn retention_managed_name_allowlist_is_closed() {
        assert!(is_managed_evidence_name(
            "etuschlopf_neuro_evidence_20260830T120000Z.zip"
        ));
        assert!(is_managed_commit_name(
            "etuschlopf_neuro_corpus_commit_20260830T120000Z.json"
        ));
        assert!(!is_managed_evidence_name("foreign_export_latest.zip"));
        assert!(!is_managed_commit_name("foreign.json"));
        assert!(!is_managed_evidence_name(
            "folder/etuschlopf_neuro_evidence_x.zip"
        ));
    }

    fn test_retention_cohort(
        root: &Path,
        token: &str,
        created_at_utc: &str,
    ) -> RetentionCorpusCohort {
        let members = REQUIRED_CORPUS_PROJECTS
            .iter()
            .enumerate()
            .map(|(index, project_id)| PreparedCorpusArchive {
                project_id: (*project_id).to_string(),
                file_name: format!(
                    "etuschlopf_neuro_evidence_{token}_{:02}.zip",
                    index + 1
                ),
                staged_path: PathBuf::new(),
                archive_sha256: format!("{:064x}", index + 1),
                qualified_decision_count: index,
                manifest: String::new(),
            })
            .collect();
        RetentionCorpusCohort {
            commit_path: root.join(format!(
                "etuschlopf_neuro_corpus_commit_{token}.json"
            )),
            created_at_utc: created_at_utc.to_string(),
            members,
        }
    }

    fn test_zip_path(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "etuschlopf_neuro_evidence_test_{}_{}_{}.zip",
            label,
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ))
    }
}
