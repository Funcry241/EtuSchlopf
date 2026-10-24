use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

pub const CORPUS_RUN_MANIFEST_FILE: &str = "corpus_run_manifest.json";
pub const CORPUS_RUN_MANIFEST_SCHEMA_VERSION: u32 = 6;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorpusRunFileEvidence {
    pub file: String,
    pub bytes: u64,
    pub fnv64: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorpusRunProjectEvidence {
    pub project: String,
    pub result_file: String,
    pub result_version: String,
    pub result_bytes: u64,
    pub result_fnv64: String,
    pub result_status: String,
    pub pipeline_exit_code: Option<i32>,
    pub outputs_complete: bool,
    pub evidence_mode: String,
    pub renderer_outcome_observed: bool,
    pub writer_outcome_observed: bool,
    pub selected_render_source: bool,
    pub write_enabled: bool,
    pub write_candidates: usize,
    pub routes_written: usize,
    pub forward_nodes_replaced: usize,
    pub forward_duration_replaced: usize,
    pub candidate_rows_after: usize,
    pub beam_lost_begin_candidates: usize,
    pub beam_lost_begin_notes_written: usize,
    pub beam_lost_begin_candidate_rows_after: usize,
    pub cross_staff_rest_suppressions: usize,
    pub source_higher_beam_notes_restored: usize,
    pub rest_vertical_anchors_written: usize,
    pub ledger_bounded_clef_boundaries_written: usize,
    pub primary_beam_voice_elements_written: usize,
    pub clef_write_cause: bool,
    pub render_xml_write: bool,
    pub notation_mutation: bool,
    pub sounding_event_mutation: bool,
    pub music_mutation: bool,
    pub master_apply: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorpusRunManifest {
    pub schema_version: u32,
    pub version: String,
    pub run_id: String,
    pub started_unix_seconds: u64,
    pub completed_unix_seconds: Option<u64>,
    pub completed: bool,
    pub full_corpus: bool,
    pub expected_project_count: usize,
    pub expected_projects: Vec<String>,
    pub projects: Vec<CorpusRunProjectEvidence>,
    pub report: Option<CorpusRunFileEvidence>,
    pub renderer_outcome_observed: bool,
    pub writer_outcome_observed: bool,
    pub write_projects: usize,
    pub routes_written: usize,
    pub forward_nodes_replaced: usize,
    pub forward_duration_replaced: usize,
    pub candidate_rows_after: usize,
    pub beam_lost_begin_candidates: usize,
    pub beam_lost_begin_notes_written: usize,
    pub beam_lost_begin_candidate_rows_after: usize,
    pub cross_staff_rest_suppressions: usize,
    pub source_higher_beam_notes_restored: usize,
    pub rest_vertical_anchors_written: usize,
    pub ledger_bounded_clef_boundaries_written: usize,
    pub primary_beam_voice_elements_written: usize,
    pub render_xml_write: bool,
    pub notation_mutation: bool,
    pub sounding_event_mutation: bool,
    pub music_mutation: bool,
    pub master_apply: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorpusRunValidation {
    pub manifest: CorpusRunManifest,
    pub manifest_path: PathBuf,
    pub project_files_valid: usize,
    pub project_files_expected: usize,
    pub report_file_valid: bool,
    pub valid: bool,
    pub reason: String,
}

impl CorpusRunManifest {
    pub fn started(version: &str, run_id: String, started_unix_seconds: u64, expected_projects: Vec<String>) -> Self {
        Self {
            schema_version: CORPUS_RUN_MANIFEST_SCHEMA_VERSION,
            version: version.to_string(),
            run_id,
            started_unix_seconds,
            completed_unix_seconds: None,
            completed: false,
            full_corpus: false,
            expected_project_count: expected_projects.len(),
            expected_projects,
            projects: Vec::new(),
            report: None,
            renderer_outcome_observed: false,
            writer_outcome_observed: false,
            write_projects: 0,
            routes_written: 0,
            forward_nodes_replaced: 0,
            forward_duration_replaced: 0,
            candidate_rows_after: 0,
            beam_lost_begin_candidates: 0,
            beam_lost_begin_notes_written: 0,
            beam_lost_begin_candidate_rows_after: 0,
            cross_staff_rest_suppressions: 0,
            source_higher_beam_notes_restored: 0,
            rest_vertical_anchors_written: 0,
            ledger_bounded_clef_boundaries_written: 0,
            primary_beam_voice_elements_written: 0,
            render_xml_write: false,
            notation_mutation: false,
            sounding_event_mutation: false,
            music_mutation: false,
            master_apply: false,
        }
    }

    pub fn upsert_project(&mut self, evidence: CorpusRunProjectEvidence) {
        if let Some(existing) = self.projects.iter_mut().find(|row| row.project == evidence.project) {
            *existing = evidence;
        } else {
            self.projects.push(evidence);
        }
        self.projects.sort_by_key(|left| natural_project_key(&left.project));
        self.refresh_outcome_projection();
    }

    fn refresh_outcome_projection(&mut self) {
        self.renderer_outcome_observed = self.expected_project_count > 0
            && self.projects.len() == self.expected_project_count
            && self.projects.iter().all(|project| project.renderer_outcome_observed);
        self.writer_outcome_observed = self.expected_project_count > 0
            && self.projects.len() == self.expected_project_count
            && self.projects.iter().all(|project| project.writer_outcome_observed);
        self.write_projects = self
            .projects
            .iter()
            .filter(|project| project.render_xml_write)
            .count();
        self.routes_written = self.projects.iter().map(|project| project.routes_written).sum();
        self.forward_nodes_replaced = self
            .projects
            .iter()
            .map(|project| project.forward_nodes_replaced)
            .sum();
        self.forward_duration_replaced = self
            .projects
            .iter()
            .map(|project| project.forward_duration_replaced)
            .sum();
        self.candidate_rows_after = self
            .projects
            .iter()
            .map(|project| project.candidate_rows_after)
            .sum();
        self.beam_lost_begin_candidates = self
            .projects
            .iter()
            .map(|project| project.beam_lost_begin_candidates)
            .sum();
        self.beam_lost_begin_notes_written = self
            .projects
            .iter()
            .map(|project| project.beam_lost_begin_notes_written)
            .sum();
        self.beam_lost_begin_candidate_rows_after = self
            .projects
            .iter()
            .map(|project| project.beam_lost_begin_candidate_rows_after)
            .sum();
        self.cross_staff_rest_suppressions = self
            .projects
            .iter()
            .map(|project| project.cross_staff_rest_suppressions)
            .sum();
        self.source_higher_beam_notes_restored = self
            .projects
            .iter()
            .map(|project| project.source_higher_beam_notes_restored)
            .sum();
        self.rest_vertical_anchors_written = self
            .projects
            .iter()
            .map(|project| project.rest_vertical_anchors_written)
            .sum();
        self.ledger_bounded_clef_boundaries_written = self
            .projects
            .iter()
            .map(|project| project.ledger_bounded_clef_boundaries_written)
            .sum();
        self.primary_beam_voice_elements_written = self
            .projects
            .iter()
            .map(|project| project.primary_beam_voice_elements_written)
            .sum();
        self.render_xml_write = self.projects.iter().any(|project| project.render_xml_write);
        self.notation_mutation = self.projects.iter().any(|project| project.notation_mutation);
        self.sounding_event_mutation = self
            .projects
            .iter()
            .any(|project| project.sounding_event_mutation);
        self.music_mutation = self.projects.iter().any(|project| project.music_mutation);
        self.master_apply = self.projects.iter().any(|project| project.master_apply);
    }

    pub fn finalize(&mut self, completed_unix_seconds: u64, report: CorpusRunFileEvidence) {
        self.completed_unix_seconds = Some(completed_unix_seconds);
        self.report = Some(report);
        self.completed = true;
        self.refresh_outcome_projection();
        let expected_project_set = self.expected_projects.iter().collect::<BTreeSet<_>>();
        let evidenced_project_set = self.projects.iter().map(|project| &project.project).collect::<BTreeSet<_>>();
        self.full_corpus = self.expected_project_count > 0
            && self.expected_projects.len() == self.expected_project_count
            && self.projects.len() == self.expected_project_count
            && expected_project_set.len() == self.expected_project_count
            && evidenced_project_set.len() == self.expected_project_count
            && expected_project_set == evidenced_project_set
            && self.projects.iter().all(|project| {
                project.result_version == self.version
                    && project.outputs_complete
                    && project.evidence_mode == "fresh_generated_after_delete"
                    && project.result_bytes > 0
                    && project.result_fnv64.starts_with("fnv64:")
                    && project.result_status == "OK"
                    && project.pipeline_exit_code == Some(0)
            });
    }

    pub fn write_atomic(&self, path: &Path) -> std::io::Result<()> {
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, self.to_json())?;
        match fs::rename(&tmp, path) {
            Ok(()) => Ok(()),
            Err(first_error) if path.exists() => {
                fs::remove_file(path)?;
                fs::rename(&tmp, path).map_err(|_| first_error)
            }
            Err(error) => Err(error),
        }
    }

    pub fn to_json(&self) -> String {
        let mut out = String::new();
        out.push_str("{\n");
        push_number(&mut out, 1, "schema_version", self.schema_version as u64, true);
        push_string(&mut out, 1, "version", &self.version, true);
        push_string(&mut out, 1, "run_id", &self.run_id, true);
        push_number(&mut out, 1, "started_unix_seconds", self.started_unix_seconds, true);
        match self.completed_unix_seconds {
            Some(value) => push_number(&mut out, 1, "completed_unix_seconds", value, true),
            None => push_null(&mut out, 1, "completed_unix_seconds", true),
        }
        push_bool(&mut out, 1, "completed", self.completed, true);
        push_bool(&mut out, 1, "full_corpus", self.full_corpus, true);
        push_number(&mut out, 1, "expected_project_count", self.expected_project_count as u64, true);
        out.push_str("  \"expected_projects\": [\n");
        for (index, project) in self.expected_projects.iter().enumerate() {
            out.push_str("    \"");
            out.push_str(&json_escape(project));
            out.push('"');
            if index + 1 != self.expected_projects.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ],\n");
        out.push_str("  \"projects\": [\n");
        for (index, project) in self.projects.iter().enumerate() {
            out.push_str("    {");
            inline_string(&mut out, "project", &project.project, true);
            inline_string(&mut out, "result_file", &project.result_file, true);
            inline_string(&mut out, "result_version", &project.result_version, true);
            inline_number(&mut out, "result_bytes", project.result_bytes, true);
            inline_string(&mut out, "result_fnv64", &project.result_fnv64, true);
            inline_string(&mut out, "result_status", &project.result_status, true);
            match project.pipeline_exit_code {
                Some(value) => inline_signed_number(&mut out, "pipeline_exit_code", value, true),
                None => inline_null(&mut out, "pipeline_exit_code", true),
            }
            inline_bool(&mut out, "outputs_complete", project.outputs_complete, true);
            inline_string(&mut out, "evidence_mode", &project.evidence_mode, true);
            inline_bool(&mut out, "project_renderer_outcome_observed", project.renderer_outcome_observed, true);
            inline_bool(&mut out, "project_writer_outcome_observed", project.writer_outcome_observed, true);
            inline_bool(&mut out, "writer_selected_render_source", project.selected_render_source, true);
            inline_bool(&mut out, "writer_write_enabled", project.write_enabled, true);
            inline_number(&mut out, "writer_candidates", project.write_candidates as u64, true);
            inline_number(&mut out, "writer_routes_written", project.routes_written as u64, true);
            inline_number(&mut out, "writer_forward_nodes_replaced", project.forward_nodes_replaced as u64, true);
            inline_number(&mut out, "writer_forward_duration_replaced", project.forward_duration_replaced as u64, true);
            inline_number(&mut out, "writer_candidate_rows_after", project.candidate_rows_after as u64, true);
            inline_number(
                &mut out,
                "writer_beam_lost_begin_candidates",
                project.beam_lost_begin_candidates as u64,
                true,
            );
            inline_number(
                &mut out,
                "writer_beam_lost_begin_notes_written",
                project.beam_lost_begin_notes_written as u64,
                true,
            );
            inline_number(
                &mut out,
                "writer_beam_lost_begin_candidate_rows_after",
                project.beam_lost_begin_candidate_rows_after as u64,
                true,
            );
            inline_number(
                &mut out,
                "writer_cross_staff_rest_suppressions",
                project.cross_staff_rest_suppressions as u64,
                true,
            );
            inline_number(
                &mut out,
                "writer_source_higher_beam_notes_restored",
                project.source_higher_beam_notes_restored as u64,
                true,
            );
            inline_number(
                &mut out,
                "writer_rest_vertical_anchors_written",
                project.rest_vertical_anchors_written as u64,
                true,
            );
            inline_number(
                &mut out,
                "writer_ledger_bounded_clef_boundaries_written",
                project.ledger_bounded_clef_boundaries_written as u64,
                true,
            );
            inline_number(
                &mut out,
                "writer_primary_beam_voice_elements_written",
                project.primary_beam_voice_elements_written as u64,
                true,
            );
            inline_bool(&mut out, "writer_clef_write_cause", project.clef_write_cause, true);
            inline_bool(&mut out, "writer_render_xml_write_committed", project.render_xml_write, true);
            inline_bool(&mut out, "writer_render_notation_mutation", project.notation_mutation, true);
            inline_bool(&mut out, "writer_sounding_event_mutation", project.sounding_event_mutation, true);
            inline_bool(&mut out, "writer_declared_music_mutation", project.music_mutation, true);
            inline_bool(&mut out, "writer_master_musicxml_mutation", project.master_apply, false);
            out.push('}');
            if index + 1 != self.projects.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ],\n");
        match &self.report {
            Some(report) => {
                out.push_str("  \"report\": {");
                inline_string(&mut out, "file", &report.file, true);
                inline_number(&mut out, "bytes", report.bytes, true);
                inline_string(&mut out, "fnv64", &report.fnv64, false);
                out.push_str("},\n");
            }
            None => out.push_str("  \"report\": null,\n"),
        }
        push_bool(&mut out, 1, "renderer_outcome_observed", self.renderer_outcome_observed, true);
        push_bool(&mut out, 1, "writer_outcome_observed", self.writer_outcome_observed, true);
        push_number(&mut out, 1, "write_projects", self.write_projects as u64, true);
        push_number(&mut out, 1, "routes_written", self.routes_written as u64, true);
        push_number(&mut out, 1, "forward_nodes_replaced", self.forward_nodes_replaced as u64, true);
        push_number(&mut out, 1, "forward_duration_replaced", self.forward_duration_replaced as u64, true);
        push_number(&mut out, 1, "candidate_rows_after", self.candidate_rows_after as u64, true);
        push_number(
            &mut out,
            1,
            "beam_lost_begin_candidates",
            self.beam_lost_begin_candidates as u64,
            true,
        );
        push_number(
            &mut out,
            1,
            "beam_lost_begin_notes_written",
            self.beam_lost_begin_notes_written as u64,
            true,
        );
        push_number(
            &mut out,
            1,
            "beam_lost_begin_candidate_rows_after",
            self.beam_lost_begin_candidate_rows_after as u64,
            true,
        );
        push_number(
            &mut out,
            1,
            "cross_staff_rest_suppressions",
            self.cross_staff_rest_suppressions as u64,
            true,
        );
        push_number(
            &mut out,
            1,
            "source_higher_beam_notes_restored",
            self.source_higher_beam_notes_restored as u64,
            true,
        );
        push_number(
            &mut out,
            1,
            "rest_vertical_anchors_written",
            self.rest_vertical_anchors_written as u64,
            true,
        );
        push_number(
            &mut out,
            1,
            "ledger_bounded_clef_boundaries_written",
            self.ledger_bounded_clef_boundaries_written as u64,
            true,
        );
        push_number(
            &mut out,
            1,
            "primary_beam_voice_elements_written",
            self.primary_beam_voice_elements_written as u64,
            true,
        );
        push_bool(&mut out, 1, "render_xml_write_committed", self.render_xml_write, true);
        push_bool(&mut out, 1, "render_notation_mutation", self.notation_mutation, true);
        push_bool(&mut out, 1, "sounding_event_mutation", self.sounding_event_mutation, true);
        push_bool(&mut out, 1, "declared_music_mutation", self.music_mutation, true);
        push_bool(&mut out, 1, "master_musicxml_mutation", self.master_apply, false);
        out.push_str("}\n");
        out
    }
}



pub fn file_evidence(path: &Path) -> std::io::Result<CorpusRunFileEvidence> {
    let bytes = fs::read(path)?;
    Ok(CorpusRunFileEvidence {
        file: path.file_name().and_then(|value| value.to_str()).unwrap_or_default().to_string(),
        bytes: bytes.len() as u64,
        fnv64: format!("fnv64:{:016x}", fnv1a64(&bytes)),
    })
}

pub fn project_evidence(
    project: &str,
    result_path: &Path,
    result_text: &str,
    pipeline_exit_code: Option<i32>,
    outputs_complete: bool,
) -> CorpusRunProjectEvidence {
    let bytes = result_text.as_bytes();
    let write_outcome = final_writer_outcome(result_text);
    CorpusRunProjectEvidence {
        project: project.to_string(),
        result_file: result_path.file_name().and_then(|value| value.to_str()).unwrap_or_default().to_string(),
        result_version: report_field(result_text, "Werkzeug: etuschlopf ").unwrap_or_else(|| "unknown".to_string()),
        result_bytes: bytes.len() as u64,
        result_fnv64: format!("fnv64:{:016x}", fnv1a64(bytes)),
        result_status: report_field(result_text, "Ergebnis: ").unwrap_or_else(|| "unknown".to_string()),
        pipeline_exit_code,
        outputs_complete,
        evidence_mode: "fresh_generated_after_delete".to_string(),
        renderer_outcome_observed: renderer_outcome_observed(result_text),
        writer_outcome_observed: write_outcome.is_some(),
        selected_render_source: write_outcome.as_ref().is_some_and(|row| row.selected_render_source),
        write_enabled: write_outcome.as_ref().is_some_and(|row| row.write_enabled),
        write_candidates: write_outcome.as_ref().map_or(0, |row| row.write_candidates),
        routes_written: write_outcome.as_ref().map_or(0, |row| row.routes_written),
        forward_nodes_replaced: write_outcome.as_ref().map_or(0, |row| row.forward_nodes_replaced),
        forward_duration_replaced: write_outcome.as_ref().map_or(0, |row| row.forward_duration_replaced),
        candidate_rows_after: write_outcome.as_ref().map_or(0, |row| row.candidate_rows_after),
        beam_lost_begin_candidates: write_outcome
            .as_ref()
            .map_or(0, |row| row.beam_lost_begin_candidates),
        beam_lost_begin_notes_written: write_outcome
            .as_ref()
            .map_or(0, |row| row.beam_lost_begin_notes_written),
        beam_lost_begin_candidate_rows_after: write_outcome
            .as_ref()
            .map_or(0, |row| row.beam_lost_begin_candidate_rows_after),
        cross_staff_rest_suppressions: write_outcome
            .as_ref()
            .map_or(0, |row| row.cross_staff_rest_suppressions),
        source_higher_beam_notes_restored: write_outcome
            .as_ref()
            .map_or(0, |row| row.source_higher_beam_notes_restored),
        rest_vertical_anchors_written: write_outcome
            .as_ref()
            .map_or(0, |row| row.rest_vertical_anchors_written),
        ledger_bounded_clef_boundaries_written: write_outcome
            .as_ref()
            .map_or(0, |row| row.ledger_bounded_clef_boundaries_written),
        primary_beam_voice_elements_written: write_outcome
            .as_ref()
            .map_or(0, |row| row.primary_beam_voice_elements_written),
        clef_write_cause: write_outcome.as_ref().is_some_and(|row| row.clef_write_cause),
        render_xml_write: write_outcome.as_ref().is_some_and(|row| row.render_xml_write),
        notation_mutation: write_outcome.as_ref().is_some_and(|row| row.notation_mutation),
        sounding_event_mutation: write_outcome
            .as_ref()
            .is_some_and(|row| row.sounding_event_mutation),
        music_mutation: write_outcome.as_ref().is_some_and(|row| row.music_mutation),
        master_apply: write_outcome.as_ref().is_some_and(|row| row.master_apply),
    }
}

pub fn read_manifest(path: &Path) -> Result<CorpusRunManifest, String> {
    let text = fs::read_to_string(path).map_err(|error| format!("manifest_read_failed:{error}"))?;
    parse_manifest(&text)
}

pub fn validate_manifest(project_root: &Path, expected_version: &str) -> CorpusRunValidation {
    let manifest_path = project_root.join(CORPUS_RUN_MANIFEST_FILE);
    let manifest = match read_manifest(&manifest_path) {
        Ok(manifest) => manifest,
        Err(reason) => {
            return CorpusRunValidation {
                manifest: CorpusRunManifest::started(expected_version, "none".to_string(), 0, Vec::new()),
                manifest_path,
                project_files_valid: 0,
                project_files_expected: 0,
                report_file_valid: false,
                valid: false,
                reason,
            };
        }
    };
    let expected = manifest.projects.len();
    let mut valid_projects = 0usize;
    for project in &manifest.projects {
        let path = project_root.join(&project.result_file);
        let Ok(bytes) = fs::read(&path) else {
            continue;
        };
        if bytes.len() as u64 == project.result_bytes
            && format!("fnv64:{:016x}", fnv1a64(&bytes)) == project.result_fnv64
            && project.result_version == expected_version
            && project.outputs_complete
            && project.evidence_mode == "fresh_generated_after_delete"
        {
            valid_projects += 1;
        }
    }
    let report_file_valid = manifest.report.as_ref().is_some_and(|report| {
        let path = project_root.join(&report.file);
        let Ok(bytes) = fs::read(path) else {
            return false;
        };
        bytes.len() as u64 == report.bytes
            && format!("fnv64:{:016x}", fnv1a64(&bytes)) == report.fnv64
    });
    let expected_project_set = manifest.expected_projects.iter().collect::<BTreeSet<_>>();
    let evidenced_project_set = manifest.projects.iter().map(|project| &project.project).collect::<BTreeSet<_>>();
    let project_sets_match = manifest.expected_project_count > 0
        && manifest.expected_projects.len() == manifest.expected_project_count
        && manifest.projects.len() == manifest.expected_project_count
        && expected_project_set.len() == manifest.expected_project_count
        && evidenced_project_set.len() == manifest.expected_project_count
        && expected_project_set == evidenced_project_set;
    let project_run_statuses_green = manifest.projects.iter().all(|project| {
        project.result_status == "OK" && project.pipeline_exit_code == Some(0)
    });
    let renderer_rows_complete = manifest.projects.len() == manifest.expected_project_count
        && manifest
            .projects
            .iter()
            .all(|project| project.renderer_outcome_observed);
    let writer_rows_complete = manifest.projects.len() == manifest.expected_project_count
        && manifest
            .projects
            .iter()
            .all(|project| project.writer_outcome_observed);
    let projected_write_projects = manifest
        .projects
        .iter()
        .filter(|project| project.render_xml_write)
        .count();
    let projected_routes_written: usize = manifest
        .projects
        .iter()
        .map(|project| project.routes_written)
        .sum();
    let projected_forward_nodes_replaced: usize = manifest
        .projects
        .iter()
        .map(|project| project.forward_nodes_replaced)
        .sum();
    let projected_forward_duration_replaced: usize = manifest
        .projects
        .iter()
        .map(|project| project.forward_duration_replaced)
        .sum();
    let projected_candidate_rows_after: usize = manifest
        .projects
        .iter()
        .map(|project| project.candidate_rows_after)
        .sum();
    let projected_beam_lost_begin_candidates: usize = manifest
        .projects
        .iter()
        .map(|project| project.beam_lost_begin_candidates)
        .sum();
    let projected_beam_lost_begin_notes_written: usize = manifest
        .projects
        .iter()
        .map(|project| project.beam_lost_begin_notes_written)
        .sum();
    let projected_beam_lost_begin_candidate_rows_after: usize = manifest
        .projects
        .iter()
        .map(|project| project.beam_lost_begin_candidate_rows_after)
        .sum();
    let projected_cross_staff_rest_suppressions: usize = manifest
        .projects
        .iter()
        .map(|project| project.cross_staff_rest_suppressions)
        .sum();
    let projected_source_higher_beam_notes_restored: usize = manifest
        .projects
        .iter()
        .map(|project| project.source_higher_beam_notes_restored)
        .sum();
    let projected_rest_vertical_anchors_written: usize = manifest
        .projects
        .iter()
        .map(|project| project.rest_vertical_anchors_written)
        .sum();
    let projected_ledger_bounded_clef_boundaries_written: usize = manifest
        .projects
        .iter()
        .map(|project| project.ledger_bounded_clef_boundaries_written)
        .sum();
    let projected_primary_beam_voice_elements_written: usize = manifest
        .projects
        .iter()
        .map(|project| project.primary_beam_voice_elements_written)
        .sum();
    let project_outcomes_consistent = manifest.projects.iter().all(|project| {
        let write_cause_count = project
            .routes_written
            .saturating_add(project.cross_staff_rest_suppressions)
            .saturating_add(project.beam_lost_begin_notes_written)
            .saturating_add(project.source_higher_beam_notes_restored)
            .saturating_add(project.rest_vertical_anchors_written)
            .saturating_add(project.primary_beam_voice_elements_written)
            .saturating_add(usize::from(project.clef_write_cause));
        let forward_projection_consistent = if project.forward_nodes_replaced > 0 {
            project.forward_duration_replaced > 0
        } else {
            project.forward_duration_replaced == 0
        };
        let beam_lost_begin_projection_consistent = if project.beam_lost_begin_notes_written > 0 {
            project.beam_lost_begin_candidates == project.beam_lost_begin_notes_written
                && project.beam_lost_begin_candidate_rows_after == 0
        } else {
            project.beam_lost_begin_candidates == 0
                && project.beam_lost_begin_candidate_rows_after == 0
        };
        project.renderer_outcome_observed
            && project.writer_outcome_observed
            && project.write_enabled
            && forward_projection_consistent
            && beam_lost_begin_projection_consistent
            && project.candidate_rows_after == 0
            && project.render_xml_write == (write_cause_count > 0)
            && project.notation_mutation == project.render_xml_write
            && (!project.music_mutation || project.render_xml_write)
            && !project.sounding_event_mutation
            && !project.master_apply
            && (!project.render_xml_write || project.selected_render_source)
    });
    let outcome_projection_consistent = manifest.renderer_outcome_observed
        && renderer_rows_complete
        && manifest.writer_outcome_observed
        && writer_rows_complete
        && manifest.write_projects == projected_write_projects
        && manifest.routes_written == projected_routes_written
        && manifest.forward_nodes_replaced == projected_forward_nodes_replaced
        && manifest.forward_duration_replaced == projected_forward_duration_replaced
        && manifest.candidate_rows_after == projected_candidate_rows_after
        && manifest.beam_lost_begin_candidates == projected_beam_lost_begin_candidates
        && manifest.beam_lost_begin_notes_written == projected_beam_lost_begin_notes_written
        && manifest.beam_lost_begin_candidate_rows_after
            == projected_beam_lost_begin_candidate_rows_after
        && manifest.cross_staff_rest_suppressions
            == projected_cross_staff_rest_suppressions
        && manifest.source_higher_beam_notes_restored
            == projected_source_higher_beam_notes_restored
        && manifest.rest_vertical_anchors_written
            == projected_rest_vertical_anchors_written
        && manifest.ledger_bounded_clef_boundaries_written
            == projected_ledger_bounded_clef_boundaries_written
        && manifest.primary_beam_voice_elements_written
            == projected_primary_beam_voice_elements_written
        && manifest.render_xml_write == (projected_write_projects > 0)
        && manifest.notation_mutation
            == manifest.projects.iter().any(|project| project.notation_mutation)
        && manifest.sounding_event_mutation
            == manifest
                .projects
                .iter()
                .any(|project| project.sounding_event_mutation)
        && manifest.music_mutation
            == manifest.projects.iter().any(|project| project.music_mutation)
        && manifest.master_apply == manifest.projects.iter().any(|project| project.master_apply)
        && manifest.notation_mutation == manifest.render_xml_write
        && !manifest.sounding_event_mutation
        && (!manifest.music_mutation || manifest.notation_mutation)
        && !manifest.master_apply
        && project_outcomes_consistent;
    let valid = manifest.schema_version == CORPUS_RUN_MANIFEST_SCHEMA_VERSION
        && manifest.version == expected_version
        && manifest.completed
        && manifest.completed_unix_seconds.is_some()
        && manifest.run_id != "none"
        && manifest.full_corpus
        && manifest.expected_project_count > 0
        && manifest.expected_projects.len() == manifest.expected_project_count
        && manifest.projects.len() == manifest.expected_project_count
        && project_sets_match
        && project_run_statuses_green
        && valid_projects == manifest.expected_project_count
        && report_file_valid
        && outcome_projection_consistent;
    let reason = if manifest.schema_version != CORPUS_RUN_MANIFEST_SCHEMA_VERSION {
        "manifest_schema_mismatch"
    } else if manifest.version != expected_version {
        "manifest_version_mismatch"
    } else if !manifest.completed {
        "manifest_not_completed"
    } else if !manifest.full_corpus
        || manifest.expected_project_count == 0
        || manifest.expected_projects.len() != manifest.expected_project_count
        || manifest.projects.len() != manifest.expected_project_count
    {
        "manifest_not_full_expected_project_run"
    } else if !project_sets_match {
        "manifest_expected_and_evidenced_project_sets_differ"
    } else if !project_run_statuses_green {
        "manifest_one_or_more_project_runs_not_green"
    } else if valid_projects != manifest.expected_project_count {
        "manifest_project_hash_evidence_invalid_or_missing"
    } else if !report_file_valid {
        "manifest_report_hash_evidence_invalid_or_missing"
    } else if !renderer_rows_complete || !manifest.renderer_outcome_observed {
        "manifest_renderer_outcome_not_observed_for_all_projects"
    } else if !writer_rows_complete || !manifest.writer_outcome_observed {
        "manifest_writer_outcome_not_observed_for_all_projects"
    } else if !project_outcomes_consistent {
        return CorpusRunValidation {
            manifest_path,
            project_files_valid: valid_projects,
            project_files_expected: expected,
            report_file_valid,
            valid,
            reason: format!(
                "manifest_project_write_outcome_inconsistent: {}",
                project_write_outcome_failure_evidence(&manifest.projects)
            ),
            manifest,
        };
    } else if !outcome_projection_consistent {
        "manifest_corpus_write_outcome_projection_inconsistent"
    } else {
        "manifest_hash_bound_full_corpus_valid_with_outcome_projection"
    }
    .to_string();
    CorpusRunValidation {
        manifest,
        manifest_path,
        project_files_valid: valid_projects,
        project_files_expected: expected,
        report_file_valid,
        valid,
        reason,
    }
}


fn project_write_outcome_failure_evidence(projects: &[CorpusRunProjectEvidence]) -> String {
    projects
        .iter()
        .map(|project| {
            format!(
                "project={};renderer={};writer={};selected={};enabled={};routes={};forward_nodes={};forward_duration={};candidate_after={};beam_candidates={};beam_written={};beam_after={};cross_staff_suppressions={};higher_beams={};rest_anchors={};clef_boundaries={};primary_beam_voice={};clef_cause={};render_write={};notation_mutation={};sounding_mutation={};music_mutation={};master_apply={}",
                project.project,
                project.renderer_outcome_observed,
                project.writer_outcome_observed,
                project.selected_render_source,
                project.write_enabled,
                project.routes_written,
                project.forward_nodes_replaced,
                project.forward_duration_replaced,
                project.candidate_rows_after,
                project.beam_lost_begin_candidates,
                project.beam_lost_begin_notes_written,
                project.beam_lost_begin_candidate_rows_after,
                project.cross_staff_rest_suppressions,
                project.source_higher_beam_notes_restored,
                project.rest_vertical_anchors_written,
                project.ledger_bounded_clef_boundaries_written,
                project.primary_beam_voice_elements_written,
                project.clef_write_cause,
                project.render_xml_write,
                project.notation_mutation,
                project.sounding_event_mutation,
                project.music_mutation,
                project.master_apply,
            )
        })
        .collect::<Vec<_>>()
        .join(" || ")
}

fn parse_manifest(text: &str) -> Result<CorpusRunManifest, String> {
    let schema_version = json_u32_field(text, "schema_version")?;
    let version = json_string_field(text, "version")?;
    let run_id = json_string_field(text, "run_id")?;
    let started_unix_seconds = json_u64_field(text, "started_unix_seconds")?;
    let completed_unix_seconds = json_optional_u64_field(text, "completed_unix_seconds")?;
    let completed = json_bool_field(text, "completed")?;
    let full_corpus = json_bool_field(text, "full_corpus")?;
    let expected_project_count = json_usize_field(text, "expected_project_count")?;
    let expected_projects = json_string_array(text, "expected_projects")?;
    let projects = json_object_array(text, "projects")?
        .into_iter()
        .map(|object| {
            Ok(CorpusRunProjectEvidence {
                project: json_string_field(object, "project")?,
                result_file: json_string_field(object, "result_file")?,
                result_version: json_string_field(object, "result_version")?,
                result_bytes: json_u64_field(object, "result_bytes")?,
                result_fnv64: json_string_field(object, "result_fnv64")?,
                result_status: json_string_field(object, "result_status")?,
                pipeline_exit_code: json_optional_i32_field(object, "pipeline_exit_code")?,
                outputs_complete: json_bool_field(object, "outputs_complete")?,
                evidence_mode: json_string_field(object, "evidence_mode")?,
                renderer_outcome_observed: json_bool_field(object, "project_renderer_outcome_observed")?,
                writer_outcome_observed: json_bool_field(object, "project_writer_outcome_observed")?,
                selected_render_source: json_bool_field(object, "writer_selected_render_source")?,
                write_enabled: json_bool_field(object, "writer_write_enabled")?,
                write_candidates: json_usize_field(object, "writer_candidates")?,
                routes_written: json_usize_field(object, "writer_routes_written")?,
                forward_nodes_replaced: json_usize_field(object, "writer_forward_nodes_replaced")?,
                forward_duration_replaced: json_usize_field(object, "writer_forward_duration_replaced")?,
                candidate_rows_after: json_usize_field(object, "writer_candidate_rows_after")?,
                beam_lost_begin_candidates: json_usize_field(
                    object,
                    "writer_beam_lost_begin_candidates",
                )?,
                beam_lost_begin_notes_written: json_usize_field(
                    object,
                    "writer_beam_lost_begin_notes_written",
                )?,
                beam_lost_begin_candidate_rows_after: json_usize_field(
                    object,
                    "writer_beam_lost_begin_candidate_rows_after",
                )?,
                cross_staff_rest_suppressions: json_usize_field(
                    object,
                    "writer_cross_staff_rest_suppressions",
                )?,
                source_higher_beam_notes_restored: json_usize_field(
                    object,
                    "writer_source_higher_beam_notes_restored",
                )?,
                rest_vertical_anchors_written: json_usize_field(
                    object,
                    "writer_rest_vertical_anchors_written",
                )?,
                ledger_bounded_clef_boundaries_written: json_usize_field(
                    object,
                    "writer_ledger_bounded_clef_boundaries_written",
                )?,
                primary_beam_voice_elements_written: if schema_version >= 6 {
                    json_usize_field(object, "writer_primary_beam_voice_elements_written")?
                } else {
                    0
                },
                clef_write_cause: if schema_version >= 5 {
                    json_bool_field(object, "writer_clef_write_cause")?
                } else {
                    false
                },
                render_xml_write: json_bool_field(
                    object,
                    "writer_render_xml_write_committed",
                )?,
                notation_mutation: json_bool_field(
                    object,
                    "writer_render_notation_mutation",
                )?,
                sounding_event_mutation: json_bool_field(
                    object,
                    "writer_sounding_event_mutation",
                )?,
                music_mutation: json_bool_field(
                    object,
                    "writer_declared_music_mutation",
                )?,
                master_apply: json_bool_field(
                    object,
                    "writer_master_musicxml_mutation",
                )?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let report = json_optional_object(text, "report")?
        .map(|object| -> Result<CorpusRunFileEvidence, String> {
            Ok(CorpusRunFileEvidence {
                file: json_string_field(object, "file")?,
                bytes: json_u64_field(object, "bytes")?,
                fnv64: json_string_field(object, "fnv64")?,
            })
        })
        .transpose()?;
    let renderer_outcome_observed = json_bool_field(text, "renderer_outcome_observed")?;
    let writer_outcome_observed = json_bool_field(text, "writer_outcome_observed")?;
    let write_projects = json_usize_field(text, "write_projects")?;
    let routes_written = json_usize_field(text, "routes_written")?;
    let forward_nodes_replaced = json_usize_field(text, "forward_nodes_replaced")?;
    let forward_duration_replaced = json_usize_field(text, "forward_duration_replaced")?;
    let candidate_rows_after = json_usize_field(text, "candidate_rows_after")?;
    let beam_lost_begin_candidates = json_usize_field(text, "beam_lost_begin_candidates")?;
    let beam_lost_begin_notes_written =
        json_usize_field(text, "beam_lost_begin_notes_written")?;
    let beam_lost_begin_candidate_rows_after =
        json_usize_field(text, "beam_lost_begin_candidate_rows_after")?;
    let cross_staff_rest_suppressions =
        json_usize_field(text, "cross_staff_rest_suppressions")?;
    let source_higher_beam_notes_restored =
        json_usize_field(text, "source_higher_beam_notes_restored")?;
    let rest_vertical_anchors_written =
        json_usize_field(text, "rest_vertical_anchors_written")?;
    let ledger_bounded_clef_boundaries_written =
        json_usize_field(text, "ledger_bounded_clef_boundaries_written")?;
    let primary_beam_voice_elements_written = if schema_version >= 6 {
        json_usize_field(text, "primary_beam_voice_elements_written")?
    } else {
        0
    };
    let render_xml_write = json_bool_field(text, "render_xml_write_committed")?;
    let notation_mutation = json_bool_field(text, "render_notation_mutation")?;
    let sounding_event_mutation = json_bool_field(text, "sounding_event_mutation")?;
    let music_mutation = json_bool_field(text, "declared_music_mutation")?;
    let master_apply = json_bool_field(text, "master_musicxml_mutation")?;
    Ok(CorpusRunManifest {
        schema_version,
        version,
        run_id,
        started_unix_seconds,
        completed_unix_seconds,
        completed,
        full_corpus,
        expected_project_count,
        expected_projects,
        projects,
        report,
        renderer_outcome_observed,
        writer_outcome_observed,
        write_projects,
        routes_written,
        forward_nodes_replaced,
        forward_duration_replaced,
        candidate_rows_after,
        beam_lost_begin_candidates,
        beam_lost_begin_notes_written,
        beam_lost_begin_candidate_rows_after,
        cross_staff_rest_suppressions,
        source_higher_beam_notes_restored,
        rest_vertical_anchors_written,
        ledger_bounded_clef_boundaries_written,
        primary_beam_voice_elements_written,
        render_xml_write,
        notation_mutation,
        sounding_event_mutation,
        music_mutation,
        master_apply,
    })
}

const FINAL_REST_VERTICAL_WRITE_PIPELINE_PREFIX: &str =
    "FinalRestVerticalPlacementRenderWriterStage ";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct FinalWriterOutcome {
    selected_render_source: bool,
    write_enabled: bool,
    write_candidates: usize,
    routes_written: usize,
    forward_nodes_replaced: usize,
    forward_duration_replaced: usize,
    candidate_rows_after: usize,
    beam_lost_begin_candidates: usize,
    beam_lost_begin_notes_written: usize,
    beam_lost_begin_candidate_rows_after: usize,
    cross_staff_rest_suppressions: usize,
    source_higher_beam_notes_restored: usize,
    rest_vertical_anchors_written: usize,
    ledger_bounded_clef_boundaries_written: usize,
    primary_beam_voice_elements_written: usize,
    clef_write_cause: bool,
    render_xml_write: bool,
    notation_mutation: bool,
    sounding_event_mutation: bool,
    music_mutation: bool,
    master_apply: bool,
}

fn renderer_outcome_observed(text: &str) -> bool {
    report_field(text, "RendererOutcome: ")
        .is_some_and(|value| value.trim_end_matches('.').starts_with("observed_"))
}

fn final_writer_outcome(text: &str) -> Option<FinalWriterOutcome> {
    const SILENT_PREFIX: &str = "SilentVoiceVisibleMeasureCoverageWriterStage ";
    const BEAM_PREFIX: &str = "FinalBeamLostBeginRenderWriterStage ";
    let silent_line = text
        .lines()
        .find(|line| line.starts_with(SILENT_PREFIX))?;
    let beam_line = text.lines().find(|line| line.starts_with(BEAM_PREFIX))?;
    let rest_line = text
        .lines()
        .find(|line| line.starts_with(FINAL_REST_VERTICAL_WRITE_PIPELINE_PREFIX))?;

    let selected_render_source = report_outcome_bool(silent_line, "SelectedRenderSource")?;
    let write_enabled = report_outcome_bool(silent_line, "WriteEnabled")?;
    let write_candidates = report_outcome_usize(silent_line, "Candidates")?;
    let routes_written = report_outcome_usize(silent_line, "MeasuresWritten")?;
    let forward_nodes_replaced = report_outcome_usize(silent_line, "ForwardNodesReplaced")?;
    let forward_duration_replaced =
        report_outcome_usize(silent_line, "ForwardDurationReplaced")?;
    let silent_candidate_rows_after_raw =
        report_outcome_usize(silent_line, "CandidateRowsAfter")?;
    let silent_exact_candidate_delta =
        report_outcome_bool(silent_line, "ExactCandidateDelta")?;
    // Since r1121, ExactCandidateDelta remains green with residual SilentVoice rows
    // only when every residual concrete materialization is positively proven to
    // reproduce the accepted canonical shadow-rest conflict. Those rows are
    // authority-withheld, not unresolved writer candidates.
    let silent_candidate_rows_after = if silent_exact_candidate_delta {
        0
    } else {
        silent_candidate_rows_after_raw
    };
    let source_higher_beam_notes_restored =
        report_outcome_usize(silent_line, "SourceHigherBeamNotesRestored")?;
    let ledger_bounded_clef_boundaries_accepted =
        report_outcome_usize(silent_line, "LedgerBoundedClefBoundariesAccepted")?;
    let ledger_bounded_clef_boundaries_written =
        report_outcome_usize(silent_line, "LedgerBoundedClefBoundariesWritten")?;
    let primary_clef_projection_changed =
        report_outcome_bool(silent_line, "PrimaryClefProjectionChanged")?;
    let clef_write_cause =
        ledger_bounded_clef_boundaries_accepted > 0 || primary_clef_projection_changed;
    let primary_beam_voice_elements_written =
        report_outcome_optional_usize(silent_line, "PrimaryBeamVoiceElementsWritten")?;
    let (visible_rests_before, visible_rests_after) =
        report_outcome_usize_pair(silent_line, "VisibleRestsBeforeAfter")?;
    let (note_count_before, note_count_after) =
        report_outcome_usize_pair(silent_line, "NoteCountBeforeAfter")?;
    let pitched_event_multiset_stable =
        report_outcome_bool(silent_line, "PitchedEventMultisetStable")?;

    let visible_rest_balance = visible_rests_before
        .checked_add(forward_nodes_replaced)?
        .checked_sub(visible_rests_after)?;
    let note_count_balance = note_count_before
        .checked_add(forward_nodes_replaced)?
        .checked_sub(note_count_after)?;
    if visible_rest_balance != note_count_balance {
        return None;
    }
    let cross_staff_rest_suppressions = visible_rest_balance;

    let beam_lost_begin_candidates = report_outcome_usize(beam_line, "Candidates")?;
    let beam_lost_begin_notes_written = report_outcome_usize(beam_line, "NotesWritten")?;
    let beam_lost_begin_candidate_rows_after =
        report_outcome_usize(beam_line, "CandidateRowsAfter")?;
    let rest_vertical_candidates = report_outcome_usize(rest_line, "Candidates")?;
    let rest_vertical_anchors_written = report_outcome_usize(rest_line, "AnchorsWritten")?;
    let rest_vertical_candidate_rows_after =
        report_outcome_usize(rest_line, "CandidateRowsAfter")?;

    let render_xml_write = report_outcome_bool(silent_line, "RenderXmlWrite")?;
    let notation_mutation = report_outcome_bool(silent_line, "NotationMutation")?;
    let music_mutation = report_outcome_bool(silent_line, "LegacyMusicMutation")?;
    let master_apply = report_outcome_bool(silent_line, "MasterApply")?;
    let beam_render_xml_write = report_outcome_bool(beam_line, "RenderXmlWrite")?;
    let beam_notation_mutation = report_outcome_bool(beam_line, "NotationMutation")?;
    let beam_music_mutation = report_outcome_bool(beam_line, "LegacyMusicMutation")?;
    let beam_master_apply = report_outcome_bool(beam_line, "MasterApply")?;
    let rest_render_xml_write = report_outcome_bool(rest_line, "RenderXmlWrite")?;
    let rest_notation_mutation = report_outcome_bool(rest_line, "NotationMutation")?;
    let rest_music_mutation = report_outcome_bool(rest_line, "LegacyMusicMutation")?;
    let rest_master_apply = report_outcome_bool(rest_line, "MasterApply")?;

    let aggregate_write = routes_written > 0
        || cross_staff_rest_suppressions > 0
        || beam_lost_begin_notes_written > 0
        || source_higher_beam_notes_restored > 0
        || rest_vertical_anchors_written > 0
        || primary_beam_voice_elements_written > 0
        || clef_write_cause;
    let beam_write = beam_lost_begin_notes_written > 0;
    let rest_write = rest_vertical_anchors_written > 0;
    let rest_projection_consistent = selected_render_source
        == report_outcome_bool(rest_line, "SelectedRenderSource")?
        && write_enabled == report_outcome_bool(rest_line, "WriteEnabled")?
        && rest_vertical_candidates
            == report_outcome_usize(silent_line, "RestVerticalPlacementCandidates")?
        && rest_vertical_anchors_written
            == report_outcome_usize(silent_line, "RestVerticalPlacementAnchorsWritten")?
        && rest_vertical_candidate_rows_after
            == report_outcome_usize(silent_line, "RestVerticalPlacementCandidateRowsAfter")?
        && rest_render_xml_write == rest_write
        && rest_notation_mutation == rest_write
        && (!rest_music_mutation || rest_write)
        && !rest_master_apply;

    if selected_render_source != report_outcome_bool(beam_line, "SelectedRenderSource")?
        || write_enabled != report_outcome_bool(beam_line, "WriteEnabled")?
        || beam_lost_begin_notes_written
            != report_outcome_usize(silent_line, "BeamLostBeginNotesWritten")?
        || !rest_projection_consistent
        || render_xml_write != aggregate_write
        || notation_mutation != aggregate_write
        || (music_mutation && !aggregate_write)
        || ((beam_music_mutation || rest_music_mutation) && !music_mutation)
        || master_apply
        || beam_render_xml_write != beam_write
        || beam_notation_mutation != beam_write
        || beam_music_mutation != beam_write
        || beam_master_apply
    {
        return None;
    }

    Some(FinalWriterOutcome {
        selected_render_source,
        write_enabled,
        write_candidates: write_candidates.saturating_add(rest_vertical_candidates),
        routes_written,
        forward_nodes_replaced,
        forward_duration_replaced,
        candidate_rows_after: silent_candidate_rows_after
            .saturating_add(rest_vertical_candidate_rows_after),
        beam_lost_begin_candidates,
        beam_lost_begin_notes_written,
        beam_lost_begin_candidate_rows_after,
        cross_staff_rest_suppressions,
        source_higher_beam_notes_restored,
        rest_vertical_anchors_written,
        ledger_bounded_clef_boundaries_written,
        primary_beam_voice_elements_written,
        clef_write_cause,
        render_xml_write,
        notation_mutation,
        sounding_event_mutation: aggregate_write && !pitched_event_multiset_stable,
        music_mutation,
        master_apply,
    })
}

fn report_outcome_value<'a>(line: &'a str, label: &str) -> Option<&'a str> {
    let marker = format!("{label} ");
    line.split(';')
        .map(str::trim)
        .find_map(|field| field.strip_prefix(&marker))
        .map(str::trim)
        .map(|value| value.trim_end_matches('.'))
        .filter(|value| !value.is_empty())
}

fn report_outcome_usize(line: &str, label: &str) -> Option<usize> {
    report_outcome_value(line, label)?.parse::<usize>().ok()
}

fn report_outcome_optional_usize(line: &str, label: &str) -> Option<usize> {
    report_outcome_value(line, label)
        .map_or(Some(0), |value| value.parse::<usize>().ok())
}

fn report_outcome_usize_pair(line: &str, label: &str) -> Option<(usize, usize)> {
    let value = report_outcome_value(line, label)?;
    let (left, right) = value.split_once('/')?;
    Some((left.trim().parse().ok()?, right.trim().parse().ok()?))
}

fn report_outcome_bool(line: &str, label: &str) -> Option<bool> {
    match report_outcome_value(line, label)? {
        "ja" => Some(true),
        "nein" => Some(false),
        _ => None,
    }
}

fn report_field(text: &str, prefix: &str) -> Option<String> {
    text.lines()
        .find_map(|line| line.trim().strip_prefix(prefix))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

fn natural_project_key(value: &str) -> (u64, String) {
    let lower = value.to_lowercase();
    let number = lower
        .split(|ch: char| !ch.is_ascii_digit())
        .find(|part| !part.is_empty())
        .and_then(|part| part.parse::<u64>().ok())
        .unwrap_or(u64::MAX);
    (number, lower)
}

fn push_string(out: &mut String, indent: usize, key: &str, value: &str, comma: bool) {
    out.push_str(&"  ".repeat(indent));
    out.push('"');
    out.push_str(key);
    out.push_str("\": \"");
    out.push_str(&json_escape(value));
    out.push('"');
    if comma { out.push(','); }
    out.push('\n');
}

fn push_number(out: &mut String, indent: usize, key: &str, value: u64, comma: bool) {
    out.push_str(&"  ".repeat(indent));
    out.push('"');
    out.push_str(key);
    out.push_str("\": ");
    out.push_str(&value.to_string());
    if comma { out.push(','); }
    out.push('\n');
}

fn push_bool(out: &mut String, indent: usize, key: &str, value: bool, comma: bool) {
    out.push_str(&"  ".repeat(indent));
    out.push('"');
    out.push_str(key);
    out.push_str("\": ");
    out.push_str(if value { "true" } else { "false" });
    if comma { out.push(','); }
    out.push('\n');
}

fn push_null(out: &mut String, indent: usize, key: &str, comma: bool) {
    out.push_str(&"  ".repeat(indent));
    out.push('"');
    out.push_str(key);
    out.push_str("\": null");
    if comma { out.push(','); }
    out.push('\n');
}

fn inline_string(out: &mut String, key: &str, value: &str, comma: bool) {
    out.push('"');
    out.push_str(key);
    out.push_str("\":\"");
    out.push_str(&json_escape(value));
    out.push('"');
    if comma { out.push(','); }
}

fn inline_number(out: &mut String, key: &str, value: u64, comma: bool) {
    out.push('"');
    out.push_str(key);
    out.push_str("\":");
    out.push_str(&value.to_string());
    if comma { out.push(','); }
}

fn inline_signed_number(out: &mut String, key: &str, value: i32, comma: bool) {
    out.push('"');
    out.push_str(key);
    out.push_str("\":");
    out.push_str(&value.to_string());
    if comma { out.push(','); }
}

fn inline_bool(out: &mut String, key: &str, value: bool, comma: bool) {
    out.push('"');
    out.push_str(key);
    out.push_str("\":");
    out.push_str(if value { "true" } else { "false" });
    if comma { out.push(','); }
}

fn inline_null(out: &mut String, key: &str, comma: bool) {
    out.push('"');
    out.push_str(key);
    out.push_str("\":null");
    if comma { out.push(','); }
}

pub fn json_escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch.is_control() => out.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => out.push(ch),
        }
    }
    out
}

fn json_string_field(text: &str, key: &str) -> Result<String, String> {
    let value = json_raw_field(text, key)?;
    parse_json_string(value)
}

fn json_u64_field(text: &str, key: &str) -> Result<u64, String> {
    json_raw_field(text, key)?
        .split(|ch: char| ch == ',' || ch == '}' || ch.is_whitespace())
        .next()
        .unwrap_or_default()
        .parse::<u64>()
        .map_err(|_| format!("invalid_u64_field:{key}"))
}

fn json_u32_field(text: &str, key: &str) -> Result<u32, String> {
    let value = json_u64_field(text, key)?;
    u32::try_from(value).map_err(|_| {
        format!("integer_field_out_of_range:{key}:target=u32:value={value}")
    })
}

fn json_usize_field(text: &str, key: &str) -> Result<usize, String> {
    let value = json_u64_field(text, key)?;
    usize::try_from(value).map_err(|_| {
        format!("integer_field_out_of_range:{key}:target=usize:value={value}")
    })
}

fn json_optional_u64_field(text: &str, key: &str) -> Result<Option<u64>, String> {
    let raw = json_raw_field(text, key)?;
    if raw.starts_with("null") {
        Ok(None)
    } else {
        json_u64_field(text, key).map(Some)
    }
}

fn json_optional_i32_field(text: &str, key: &str) -> Result<Option<i32>, String> {
    let raw = json_raw_field(text, key)?;
    if raw.starts_with("null") {
        return Ok(None);
    }
    raw.split(|ch: char| ch == ',' || ch == '}' || ch.is_whitespace())
        .next()
        .unwrap_or_default()
        .parse::<i32>()
        .map(Some)
        .map_err(|_| format!("invalid_i32_field:{key}"))
}

fn json_bool_field(text: &str, key: &str) -> Result<bool, String> {
    let raw = json_raw_field(text, key)?;
    if raw.starts_with("true") {
        Ok(true)
    } else if raw.starts_with("false") {
        Ok(false)
    } else {
        Err(format!("invalid_bool_field:{key}"))
    }
}

fn json_raw_field<'a>(text: &'a str, key: &str) -> Result<&'a str, String> {
    let needle = format!("\"{key}\"");
    let start = text.find(&needle).ok_or_else(|| format!("missing_field:{key}"))? + needle.len();
    let colon = text[start..].find(':').ok_or_else(|| format!("missing_colon:{key}"))? + start;
    Ok(text[colon + 1..].trim_start())
}

fn parse_json_string(raw: &str) -> Result<String, String> {
    let mut chars = raw.chars();
    if chars.next() != Some('"') {
        return Err("expected_json_string".to_string());
    }
    let mut out = String::new();
    let mut escaped = false;
    let mut unicode = String::new();
    let mut unicode_left = 0usize;
    for ch in chars {
        if unicode_left > 0 {
            unicode.push(ch);
            unicode_left -= 1;
            if unicode_left == 0 {
                let value = u32::from_str_radix(&unicode, 16).map_err(|_| "invalid_unicode_escape".to_string())?;
                out.push(char::from_u32(value).ok_or_else(|| "invalid_unicode_scalar".to_string())?);
                unicode.clear();
            }
            continue;
        }
        if escaped {
            match ch {
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'u' => unicode_left = 4,
                _ => return Err("unsupported_json_escape".to_string()),
            }
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            return Ok(out);
        } else {
            out.push(ch);
        }
    }
    Err("unterminated_json_string".to_string())
}

fn json_string_array(text: &str, key: &str) -> Result<Vec<String>, String> {
    let raw = json_raw_field(text, key)?;
    let start = raw.find('[').ok_or_else(|| format!("missing_array_start:{key}"))?;
    let end = matching_delimiter(raw, start, '[', ']').ok_or_else(|| format!("missing_array_end:{key}"))?;
    let mut values = Vec::new();
    let mut rest = raw[start + 1..end].trim();
    while !rest.is_empty() {
        if rest.starts_with(',') {
            rest = rest[1..].trim_start();
            continue;
        }
        if !rest.starts_with('"') {
            break;
        }
        let (value, consumed) = parse_json_string_with_len(rest)?;
        values.push(value);
        rest = rest[consumed..].trim_start();
    }
    Ok(values)
}

fn json_object_array<'a>(text: &'a str, key: &str) -> Result<Vec<&'a str>, String> {
    let raw = json_raw_field(text, key)?;
    let start = raw.find('[').ok_or_else(|| format!("missing_array_start:{key}"))?;
    let end = matching_delimiter(raw, start, '[', ']').ok_or_else(|| format!("missing_array_end:{key}"))?;
    let body = &raw[start + 1..end];
    let mut objects = Vec::new();
    let mut index = 0usize;
    while index < body.len() {
        let Some(relative) = body[index..].find('{') else { break; };
        let object_start = index + relative;
        let object_end = matching_delimiter(body, object_start, '{', '}').ok_or_else(|| format!("missing_object_end:{key}"))?;
        objects.push(&body[object_start..=object_end]);
        index = object_end + 1;
    }
    Ok(objects)
}

fn json_optional_object<'a>(text: &'a str, key: &str) -> Result<Option<&'a str>, String> {
    let raw = json_raw_field(text, key)?;
    if raw.starts_with("null") {
        return Ok(None);
    }
    let start = raw.find('{').ok_or_else(|| format!("missing_object_start:{key}"))?;
    let end = matching_delimiter(raw, start, '{', '}').ok_or_else(|| format!("missing_object_end:{key}"))?;
    Ok(Some(&raw[start..=end]))
}

fn parse_json_string_with_len(raw: &str) -> Result<(String, usize), String> {
    let bytes = raw.as_bytes();
    if bytes.first() != Some(&b'"') {
        return Err("expected_json_string".to_string());
    }
    let mut escaped = false;
    for index in 1..bytes.len() {
        if escaped {
            escaped = false;
            continue;
        }
        match bytes[index] {
            b'\\' => escaped = true,
            b'"' => return Ok((parse_json_string(&raw[..=index])?, index + 1)),
            _ => {}
        }
    }
    Err("unterminated_json_string".to_string())
}

fn matching_delimiter(text: &str, start: usize, open: char, close: char) -> Option<usize> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, ch) in text[start..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        if ch == '"' {
            in_string = true;
        } else if ch == open {
            depth += 1;
        } else if ch == close {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(start + offset);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Clone, Copy)]
    struct WriterFixture {
        selected: bool,
        candidates: usize,
        routes: usize,
        forward_nodes: usize,
        forward_duration: usize,
        higher_beams: usize,
        rest_anchors: usize,
        clef_boundaries: usize,
        visible_rests_before: usize,
        visible_rests_after: usize,
        notes_before: usize,
        notes_after: usize,
        render_write: bool,
        declared_music_mutation: bool,
    }

    fn test_dir(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!(
            "etuschlopf_corpus_manifest_{label}_{}_{}",
            std::process::id(),
            nonce
        ))
    }

    fn yes_no(value: bool) -> &'static str {
        if value { "ja" } else { "nein" }
    }

    fn result_text(version: &str, project: &str, fixture: WriterFixture) -> String {
        let attempted = fixture.selected;
        let rest_write = fixture.rest_anchors > 0;
        format!(
            "ETUSCHLOPF-ERGEBNIS\nWerkzeug: etuschlopf {version}\nPartitur: {project}\nErgebnis: OK\n\
SilentVoiceVisibleMeasureCoverageWriterStage v{version}: Attempted {attempted}; SelectedRenderSource {selected}; WriteEnabled ja; Candidates {candidates}; MeasuresWritten {routes}; ForwardNodesPlanned {forward_nodes}; ForwardDurationPlanned {forward_duration}; ForwardNodesReplaced {forward_nodes}; ForwardDurationReplaced {forward_duration}; BeamLostBeginNotesWritten 0; SourceHigherBeamNotesRestored {higher_beams}; RestVerticalPlacementCandidates {rest_anchors}; RestVerticalPlacementAnchorsWritten {rest_anchors}; RestVerticalPlacementCandidateRowsAfter 0; LedgerBoundedClefBoundariesAccepted {clef_accepted}; LedgerBoundedClefBoundariesWritten {clef_boundaries}; PrimaryClefProjectionChanged nein; VisibleRestsBeforeAfter {visible_rests_before}/{visible_rests_after}; NoteCountBeforeAfter {notes_before}/{notes_after}; ParseBeforeAfter ja/ja; BackupForwardBeforeAfter ja/ja; RawCursorStable ja; PitchedEventMultisetStable ja; CandidateRowsAfter 0; ExactCandidateDelta ja; RenderXmlWrite {render_write}; NotationMutation {render_write}; LegacyMusicMutation {music_mutation}; MasterApply nein; Urteil test.\n\
FinalBeamLostBeginRenderWriterStage v{version}: Attempted {attempted}; SelectedRenderSource {selected}; MasterSourceBound {selected}; WriteEnabled ja; Candidates 0; NotesWritten 0; CandidateRowsAfter 0; RenderXmlWrite nein; NotationMutation nein; LegacyMusicMutation nein; MasterApply nein; Urteil test.\n\
FinalRestVerticalPlacementRenderWriterStage v{version}: Attempted {attempted}; SelectedRenderSource {selected}; FinalClefBound ja; WriteEnabled ja; Candidates {rest_anchors}; AnchorsWritten {rest_anchors}; CandidateRowsAfter 0; RenderXmlWrite {rest_write}; NotationMutation {rest_write}; LegacyMusicMutation nein; MasterApply nein; Urteil test.\n\
RendererOutcome: observed_from_existing_writer_audit.\n",
            attempted = yes_no(attempted),
            selected = yes_no(fixture.selected),
            candidates = fixture.candidates,
            routes = fixture.routes,
            forward_nodes = fixture.forward_nodes,
            forward_duration = fixture.forward_duration,
            higher_beams = fixture.higher_beams,
            rest_anchors = fixture.rest_anchors,
            clef_accepted = if fixture.render_write { fixture.clef_boundaries } else { 0 },
            clef_boundaries = fixture.clef_boundaries,
            visible_rests_before = fixture.visible_rests_before,
            visible_rests_after = fixture.visible_rests_after,
            notes_before = fixture.notes_before,
            notes_after = fixture.notes_after,
            render_write = yes_no(fixture.render_write),
            music_mutation = yes_no(fixture.declared_music_mutation),
            rest_write = yes_no(rest_write),
        )
    }

    #[test]
    fn renderer_and_writer_observation_are_independent() {
        let version = env!("CARGO_PKG_VERSION");
        let report = format!(
            "ETUSCHLOPF-ERGEBNIS\nWerkzeug: etuschlopf {version}\nPartitur: Demo\nErgebnis: OK\nRendererOutcome: observed_from_existing_writer_audit.\n"
        );
        let evidence = project_evidence(
            "Demo",
            Path::new("Demo.result.txt"),
            &report,
            Some(0),
            true,
        );
        assert!(evidence.renderer_outcome_observed);
        assert!(!evidence.writer_outcome_observed);
    }

    #[test]
    fn exact_delta_residual_rows_are_not_unresolved_candidate_rows() {
        let version = env!("CARGO_PKG_VERSION");
        let base = result_text(
            version,
            "ShadowResidual",
            WriterFixture {
                selected: true,
                candidates: 3,
                routes: 0,
                forward_nodes: 0,
                forward_duration: 0,
                higher_beams: 1,
                rest_anchors: 0,
                clef_boundaries: 0,
                visible_rests_before: 10,
                visible_rests_after: 10,
                notes_before: 20,
                notes_after: 20,
                render_write: true,
                declared_music_mutation: true,
            },
        );
        let report = base.replace(
            "CandidateRowsAfter 0; ExactCandidateDelta ja",
            "CandidateRowsAfter 3; ExactCandidateDelta ja",
        );
        let outcome = final_writer_outcome(&report).expect("exact-delta residual outcome");
        assert_eq!(outcome.candidate_rows_after, 0);
        assert!(outcome.render_xml_write);
    }

    #[test]
    fn non_exact_residual_rows_remain_unresolved() {
        let version = env!("CARGO_PKG_VERSION");
        let base = result_text(
            version,
            "UnresolvedResidual",
            WriterFixture {
                selected: true,
                candidates: 3,
                routes: 0,
                forward_nodes: 0,
                forward_duration: 0,
                higher_beams: 1,
                rest_anchors: 0,
                clef_boundaries: 0,
                visible_rests_before: 10,
                visible_rests_after: 10,
                notes_before: 20,
                notes_after: 20,
                render_write: true,
                declared_music_mutation: true,
            },
        );
        let report = base.replace(
            "CandidateRowsAfter 0; ExactCandidateDelta ja",
            "CandidateRowsAfter 3; ExactCandidateDelta nein",
        );
        let outcome = final_writer_outcome(&report).expect("non-exact residual outcome");
        assert_eq!(outcome.candidate_rows_after, 3);
    }

    #[test]
    fn higher_beam_only_and_cross_staff_only_writes_are_observed() {
        let version = env!("CARGO_PKG_VERSION");
        let higher = result_text(
            version,
            "Higher",
            WriterFixture {
                selected: true,
                candidates: 3,
                routes: 0,
                forward_nodes: 0,
                forward_duration: 0,
                higher_beams: 3,
                rest_anchors: 0,
                clef_boundaries: 0,
                visible_rests_before: 10,
                visible_rests_after: 10,
                notes_before: 20,
                notes_after: 20,
                render_write: true,
                declared_music_mutation: true,
            },
        );
        let higher_outcome = final_writer_outcome(&higher).expect("higher beam outcome");
        assert_eq!(higher_outcome.source_higher_beam_notes_restored, 3);
        assert_eq!(higher_outcome.cross_staff_rest_suppressions, 0);
        assert!(higher_outcome.render_xml_write);

        let cross_staff = result_text(
            version,
            "CrossStaff",
            WriterFixture {
                selected: true,
                candidates: 0,
                routes: 0,
                forward_nodes: 0,
                forward_duration: 0,
                higher_beams: 0,
                rest_anchors: 0,
                clef_boundaries: 0,
                visible_rests_before: 10,
                visible_rests_after: 8,
                notes_before: 20,
                notes_after: 18,
                render_write: true,
                declared_music_mutation: true,
            },
        );
        let cross_outcome = final_writer_outcome(&cross_staff).expect("cross staff outcome");
        assert_eq!(cross_outcome.cross_staff_rest_suppressions, 2);
        assert_eq!(cross_outcome.source_higher_beam_notes_restored, 0);
        assert!(cross_outcome.render_xml_write);
        assert!(!cross_outcome.sounding_event_mutation);
    }

    #[test]
    fn committed_clef_marker_without_current_clef_write_is_not_a_write_cause() {
        let version = env!("CARGO_PKG_VERSION");
        let report = result_text(
            version,
            "CommittedMarkerOnly",
            WriterFixture {
                selected: true,
                candidates: 0,
                routes: 0,
                forward_nodes: 0,
                forward_duration: 0,
                higher_beams: 0,
                rest_anchors: 0,
                clef_boundaries: 1,
                visible_rests_before: 10,
                visible_rests_after: 10,
                notes_before: 20,
                notes_after: 20,
                render_write: false,
                declared_music_mutation: false,
            },
        );
        let outcome = final_writer_outcome(&report).expect("committed marker readback outcome");
        assert_eq!(outcome.ledger_bounded_clef_boundaries_written, 1);
        assert!(!outcome.clef_write_cause);
        assert!(!outcome.render_xml_write);
    }

    #[test]
    fn current_ledger_clef_acceptance_is_a_write_cause_even_without_other_writer_counts() {
        let version = env!("CARGO_PKG_VERSION");
        let report = result_text(
            version,
            "ClefOnlyWrite",
            WriterFixture {
                selected: true,
                candidates: 1,
                routes: 0,
                forward_nodes: 0,
                forward_duration: 0,
                higher_beams: 0,
                rest_anchors: 0,
                clef_boundaries: 1,
                visible_rests_before: 10,
                visible_rests_after: 10,
                notes_before: 20,
                notes_after: 20,
                render_write: true,
                declared_music_mutation: false,
            },
        );
        let outcome = final_writer_outcome(&report).expect("current clef write outcome");
        assert!(outcome.clef_write_cause);
        assert!(outcome.render_xml_write);
    }

    #[test]
    fn primary_clef_projection_change_is_a_write_cause_without_ledger_acceptance() {
        let version = env!("CARGO_PKG_VERSION");
        let report = result_text(
            version,
            "PrimaryClefOnlyWrite",
            WriterFixture {
                selected: true,
                candidates: 0,
                routes: 0,
                forward_nodes: 0,
                forward_duration: 0,
                higher_beams: 0,
                rest_anchors: 0,
                clef_boundaries: 0,
                visible_rests_before: 10,
                visible_rests_after: 10,
                notes_before: 20,
                notes_after: 20,
                render_write: true,
                declared_music_mutation: true,
            },
        )
        .replace(
            "PrimaryClefProjectionChanged nein",
            "PrimaryClefProjectionChanged ja",
        );
        let outcome = final_writer_outcome(&report).expect("primary clef write outcome");
        assert!(outcome.clef_write_cause);
        assert!(outcome.render_xml_write);
    }

    #[test]
    fn typed_manifest_u32_field_rejects_out_of_range_value() {
        let text = r#"{"schema_version":4294967296}"#;
        let error = json_u32_field(text, "schema_version").expect_err("u32 overflow");
        assert_eq!(
            error,
            "integer_field_out_of_range:schema_version:target=u32:value=4294967296"
        );
    }

    #[test]
    fn typed_manifest_usize_field_accepts_platform_maximum() {
        let value = usize::MAX;
        let text = format!(r#"{{"expected_project_count":{value}}}"#);
        assert_eq!(
            json_usize_field(&text, "expected_project_count").expect("usize maximum"),
            value
        );
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn typed_manifest_usize_field_rejects_64_bit_only_value() {
        let text = r#"{"expected_project_count":4294967296}"#;
        let error =
            json_usize_field(text, "expected_project_count").expect_err("usize overflow");
        assert_eq!(
            error,
            "integer_field_out_of_range:expected_project_count:target=usize:value=4294967296"
        );
    }

    #[test]
    fn manifest_full_corpus_is_driven_by_declared_project_set_not_ten() {
        let root = test_dir("declared-project-set");
        fs::create_dir_all(&root).expect("create test dir");
        let version = env!("CARGO_PKG_VERSION");
        let projects = vec![
            "Alpha".to_string(),
            "Beta".to_string(),
            "Gamma".to_string(),
        ];
        let mut manifest = CorpusRunManifest::started(
            version,
            "corpus-declared-set-test".to_string(),
            100,
            projects.clone(),
        );
        for project in &projects {
            let result_path = root.join(format!("{project}.result.txt"));
            let text = result_text(
                version,
                project,
                WriterFixture {
                    selected: true,
                    candidates: 0,
                    routes: 0,
                    forward_nodes: 0,
                    forward_duration: 0,
                    higher_beams: 0,
                    rest_anchors: 0,
                    clef_boundaries: 0,
                    visible_rests_before: 0,
                    visible_rests_after: 0,
                    notes_before: 0,
                    notes_after: 0,
                    render_write: false,
                    declared_music_mutation: false,
                },
            );
            fs::write(&result_path, &text).expect("write result");
            manifest.upsert_project(project_evidence(
                project,
                &result_path,
                &text,
                Some(0),
                true,
            ));
        }
        let report_path = root.join("corpus_emergence.result.txt");
        fs::write(&report_path, "declared project set report").expect("write report");
        manifest.finalize(200, file_evidence(&report_path).expect("report evidence"));
        assert!(manifest.full_corpus);
        manifest
            .write_atomic(&root.join(CORPUS_RUN_MANIFEST_FILE))
            .expect("write manifest");
        let validation = validate_manifest(&root, version);
        assert!(validation.valid, "{}", validation.reason);
        assert_eq!(validation.project_files_expected, 3);
        assert_eq!(validation.project_files_valid, 3);
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn primary_beam_voice_writer_cause_is_optional_and_closes_render_write_projection() {
        let version = env!("CARGO_PKG_VERSION");
        let base = result_text(
            version,
            "Synthetic Project Nr. 1",
            WriterFixture {
                selected: true,
                candidates: 0,
                routes: 0,
                forward_nodes: 0,
                forward_duration: 0,
                higher_beams: 0,
                rest_anchors: 0,
                clef_boundaries: 0,
                visible_rests_before: 10,
                visible_rests_after: 10,
                notes_before: 20,
                notes_after: 20,
                render_write: false,
                declared_music_mutation: false,
            },
        );
        let future = base.replacen(
            "RenderXmlWrite nein; NotationMutation nein; LegacyMusicMutation nein;",
            "PrimaryBeamVoiceElementsWritten 2; RenderXmlWrite ja; NotationMutation ja; LegacyMusicMutation ja;",
            1,
        );
        let outcome = final_writer_outcome(&future).expect("primary-beam writer cause");
        assert_eq!(outcome.primary_beam_voice_elements_written, 2);
        assert!(outcome.render_xml_write);
        assert!(outcome.notation_mutation);
        assert!(outcome.music_mutation);

        let baseline = final_writer_outcome(&base).expect("legacy baseline remains readable");
        assert_eq!(baseline.primary_beam_voice_elements_written, 0);
        assert!(!baseline.render_xml_write);

        let malformed = base.replacen(
            "RenderXmlWrite nein; NotationMutation nein; LegacyMusicMutation nein;",
            "PrimaryBeamVoiceElementsWritten invalid; RenderXmlWrite ja; NotationMutation ja; LegacyMusicMutation ja;",
            1,
        );
        assert!(final_writer_outcome(&malformed).is_none());
    }

    #[test]
    fn hash_bound_manifest_roundtrip_and_tamper_detection() {
        let root = test_dir("roundtrip");
        fs::create_dir_all(&root).expect("create test dir");
        let version = env!("CARGO_PKG_VERSION");
        let projects = (1..=10)
            .map(|number| format!("Synthetic Project Nr. {number}"))
            .collect::<Vec<_>>();
        let mut manifest = CorpusRunManifest::started(
            version,
            "corpus-test-run".to_string(),
            100,
            projects.clone(),
        );
        for project in &projects {
            let number = natural_project_key(project).0;
            let fixture = match number {
                1 => WriterFixture {
                    selected: true,
                    candidates: 2,
                    routes: 1,
                    forward_nodes: 3,
                    forward_duration: 288,
                    higher_beams: 1,
                    rest_anchors: 0,
                    clef_boundaries: 0,
                    visible_rests_before: 51,
                    visible_rests_after: 52,
                    notes_before: 463,
                    notes_after: 464,
                    render_write: true,
                    declared_music_mutation: true,
                },
                2 => WriterFixture {
                    selected: true,
                    candidates: 3,
                    routes: 0,
                    forward_nodes: 0,
                    forward_duration: 0,
                    higher_beams: 3,
                    rest_anchors: 0,
                    clef_boundaries: 0,
                    visible_rests_before: 33,
                    visible_rests_after: 32,
                    notes_before: 875,
                    notes_after: 874,
                    render_write: true,
                    declared_music_mutation: true,
                },
                3 => WriterFixture {
                    selected: true,
                    candidates: 0,
                    routes: 0,
                    forward_nodes: 0,
                    forward_duration: 0,
                    higher_beams: 0,
                    rest_anchors: 0,
                    clef_boundaries: 0,
                    visible_rests_before: 63,
                    visible_rests_after: 61,
                    notes_before: 781,
                    notes_after: 779,
                    render_write: true,
                    declared_music_mutation: true,
                },
                4 => WriterFixture {
                    selected: true,
                    candidates: 6,
                    routes: 3,
                    forward_nodes: 0,
                    forward_duration: 0,
                    higher_beams: 3,
                    rest_anchors: 2,
                    clef_boundaries: 3,
                    visible_rests_before: 57,
                    visible_rests_after: 53,
                    notes_before: 795,
                    notes_after: 791,
                    render_write: true,
                    declared_music_mutation: true,
                },
                6 => WriterFixture {
                    selected: true,
                    candidates: 4,
                    routes: 4,
                    forward_nodes: 0,
                    forward_duration: 0,
                    higher_beams: 0,
                    rest_anchors: 2,
                    clef_boundaries: 4,
                    visible_rests_before: 80,
                    visible_rests_after: 80,
                    notes_before: 1206,
                    notes_after: 1206,
                    render_write: true,
                    declared_music_mutation: false,
                },
                9 => WriterFixture {
                    selected: true,
                    candidates: 2,
                    routes: 1,
                    forward_nodes: 3,
                    forward_duration: 288,
                    higher_beams: 1,
                    rest_anchors: 0,
                    clef_boundaries: 0,
                    visible_rests_before: 49,
                    visible_rests_after: 47,
                    notes_before: 1016,
                    notes_after: 1014,
                    render_write: true,
                    declared_music_mutation: true,
                },
                10 => WriterFixture {
                    selected: true,
                    candidates: 4,
                    routes: 3,
                    forward_nodes: 4,
                    forward_duration: 336,
                    higher_beams: 1,
                    rest_anchors: 2,
                    clef_boundaries: 1,
                    visible_rests_before: 69,
                    visible_rests_after: 72,
                    notes_before: 1064,
                    notes_after: 1067,
                    render_write: true,
                    declared_music_mutation: true,
                },
                _ => WriterFixture {
                    selected: false,
                    candidates: 0,
                    routes: 0,
                    forward_nodes: 0,
                    forward_duration: 0,
                    higher_beams: 0,
                    rest_anchors: 0,
                    clef_boundaries: 0,
                    visible_rests_before: 0,
                    visible_rests_after: 0,
                    notes_before: 0,
                    notes_after: 0,
                    render_write: false,
                    declared_music_mutation: false,
                },
            };
            let text = result_text(version, project, fixture);
            let result_path = root.join(format!("{project}.result.txt"));
            fs::write(&result_path, &text).expect("write result");
            manifest.upsert_project(project_evidence(
                project,
                &result_path,
                &text,
                Some(0),
                true,
            ));
        }
        let report_path = root.join("corpus_emergence.result.txt");
        fs::write(
            &report_path,
            format!(
                "ETUSCHLOPF-CORPUS-EMERGENCE\nWerkzeug: etuschlopf {version}\nRunId: corpus-test-run\nFullCorpus: ja\nProjectCount: 10\nCurrentVersionProjectCount: 10\nSelectionEligible: ja\n"
            ),
        )
        .expect("write report");
        manifest.finalize(200, file_evidence(&report_path).expect("report evidence"));
        let manifest_path = root.join(CORPUS_RUN_MANIFEST_FILE);
        manifest.write_atomic(&manifest_path).expect("write manifest");

        let roundtrip = read_manifest(&manifest_path).expect("read manifest");
        assert_eq!(roundtrip, manifest);
        assert!(roundtrip.renderer_outcome_observed);
        assert!(roundtrip.writer_outcome_observed);
        assert_eq!(roundtrip.write_projects, 7);
        assert_eq!(roundtrip.routes_written, 12);
        assert_eq!(roundtrip.forward_nodes_replaced, 10);
        assert_eq!(roundtrip.forward_duration_replaced, 912);
        assert_eq!(roundtrip.cross_staff_rest_suppressions, 15);
        assert_eq!(roundtrip.source_higher_beam_notes_restored, 9);
        assert_eq!(roundtrip.rest_vertical_anchors_written, 6);
        assert_eq!(roundtrip.ledger_bounded_clef_boundaries_written, 8);
        assert!(roundtrip.render_xml_write);
        assert!(roundtrip.notation_mutation);
        assert!(!roundtrip.sounding_event_mutation);
        assert!(roundtrip.music_mutation);
        assert!(!roundtrip.master_apply);
        let valid = validate_manifest(&root, version);
        assert!(valid.valid, "{}", valid.reason);

        fs::write(root.join("Synthetic Project Nr. 4.result.txt"), "tampered").expect("tamper result");
        let invalid = validate_manifest(&root, version);
        assert!(!invalid.valid);
        assert_eq!(invalid.reason, "manifest_project_hash_evidence_invalid_or_missing");

        fs::remove_dir_all(root).expect("remove test dir");
    }
}
