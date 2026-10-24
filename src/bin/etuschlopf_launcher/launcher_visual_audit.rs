fn execute_visual_preflight_project_batch(
    launcher: &mut Launcher,
    state: &mut RunState,
    session: &VisualPreflightSession,
) -> AppResult<VisualPreflightProjectBatch> {
    let plan = &session.plan;
    let projects = session.projects.as_slice();
    let aggregate_path = session.aggregate_path.as_path();
    launcher.line(
        PresentationRole::Program,
        None,
        Attention::Neutral,
        Emphasis::Primary,
        "",
    );
    launcher.line(
        PresentationRole::Program,
        None,
        Attention::Neutral,
        Emphasis::Primary,
        &format!(
            "[EtuSchlopf] Visual-Preflight: {}; Quelle {}; Log {CONSOLE_LOG}",
            projekt_count(projects.len()),
            plan.selector_source
        ),
    );

    let mut final_exit = 0i32;
    let mut total_program_ms = 0u128;
    let mut ledgers = Vec::new();
    let mut renderer_reuse_rows = Vec::new();
    for (idx, project) in projects.iter().enumerate() {
        let result_file = resolve_result_file(project);
        let pdf_file = resolve_pdf_file(project);
        let previous_pdf_fingerprint = parse_pdf_fingerprint_from_result(&result_file);
        let previous_pdf_visual_fingerprint = compute_pdf_visual_fingerprint_snapshot(&pdf_file);
        let project_run_started_at = SystemTime::now();
        let header = format!("PROGRAM-RUN-VISUAL-PREFLIGHT-{}", idx + 1);
        let reuse_status = if state.force_mode {
            "force_rendered".to_string()
        } else {
            visual_preflight_project_renderer_reuse_status(project, &launcher.version)?
        };
        let reuse_action = visual_preflight_renderer_reuse_action(&reuse_status);
        let reusable = reuse_action == "reused";
        let guard_blocked = reuse_action == "blocked";
        let result = if reusable {
            renderer_reuse_rows.push(format!("{project} | reused | unchanged | skipped"));
            launcher.line(
                PresentationRole::Program,
                None,
                Attention::Neutral,
                Emphasis::Primary,
                &format!(
                    "[EtuSchlopf] Visual-Preflight {}/{}: {} reuse",
                    idx + 1,
                    projects.len(),
                    project
                ),
            );
            append_command_header(Path::new(CONSOLE_LOG), &header, &state.target_exe, &[OsString::from(project.as_str())])?;
            {
                let mut file = OpenOptions::new().create(true).append(true).open(CONSOLE_LOG)?;
                writeln!(file, "Skipped: renderer_reuse_for_unchanged_project_inputs")?;
                writeln!(file, "Project: {project}")?;
                writeln!(file, "Version: {}", launcher.version)?;
                writeln!(file, "RendererReuseStatus: reused")?;
                writeln!(file)?;
            }
            RunResult { exit_code: 0, elapsed_ms: 0 }
        } else if guard_blocked {
            state.audit_signal = AuditSignal::ExpectedBlocked;
            state.renderer_outcome = state
                .renderer_outcome
                .merge(RendererOutcome::GuardBlocked);
            renderer_reuse_rows.push(format!("{project} | blocked | {reuse_status} | skipped"));
            launcher.line(
                PresentationRole::Program,
                None,
                Attention::Neutral,
                Emphasis::Primary,
                &format!(
                    "[EtuSchlopf] Visual-Preflight {}/{}: {} guard-blocked",
                    idx + 1,
                    projects.len(),
                    project
                ),
            );
            append_command_header(Path::new(CONSOLE_LOG), &header, &state.target_exe, &[OsString::from(project.as_str())])?;
            {
                let mut file = OpenOptions::new().create(true).append(true).open(CONSOLE_LOG)?;
                writeln!(file, "Skipped: renderer_guard_blocked_project_reuse")?;
                writeln!(file, "Project: {project}")?;
                writeln!(file, "Version: {}", launcher.version)?;
                writeln!(file, "RendererReuseStatus: {reuse_status}")?;
                writeln!(file)?;
            }
            RunResult { exit_code: 0, elapsed_ms: 0 }
        } else {
            renderer_reuse_rows.push(format!("{project} | {reuse_action} | {reuse_status} | rendered"));
            launcher.line(
                PresentationRole::Program,
                None,
                Attention::Neutral,
                Emphasis::Primary,
                &format!(
                    "[EtuSchlopf] Visual-Preflight {}/{}: {}",
                    idx + 1,
                    projects.len(),
                    project
                ),
            );
            let project_args = vec![OsString::from(project.as_str())];
            invoke_logged_program(
                launcher,
                &state.target_exe,
                &project_args,
                Path::new(CONSOLE_LOG),
                &header,
                PresentationRole::Program,
                "Programm/PDF",
            )?
        };
        if !reusable && !guard_blocked {
            let renderer_observation = if result.exit_code != 0 {
                RendererOutcome::ProcessFailed(result.exit_code)
            } else {
                RendererOutcome::Passed
            };
            state.renderer_outcome = state
                .renderer_outcome
                .merge(renderer_observation);
        }
        total_program_ms = total_program_ms.saturating_add(result.elapsed_ms);
        if result.exit_code == 0 && !reusable {
            write_visual_preflight_project_reuse_stamp(project, &launcher.version)?;
        }
        let mut ledger = visual_project_run_ledger(
            project,
            &result_file,
            result.exit_code,
            &launcher.version,
            previous_pdf_fingerprint,
            previous_pdf_visual_fingerprint,
        );
        if !reusable && !guard_blocked {
            ledger.shadow_run_started_at = Some(project_run_started_at);
        }
        state.input_arg = Some(project.clone());
        state.result_file = Some(result_file.clone());
        state.run_exit = result.exit_code.to_string();
        state.program_ms = if reusable {
            "skipped_renderer_reuse".to_string()
        } else {
            result.elapsed_ms.to_string()
        };
        append_run_diagnostic(state, "visual_preflight_project")?;

        {
            let mut file = OpenOptions::new().append(true).open(aggregate_path)?;
            writeln!(file, "{}", ledger.project_row())?;
        }

        if result.exit_code != 0 {
            final_exit = result.exit_code;
        } else if final_exit == 0 && (guard_blocked || !ledger.is_fresh()) {
            final_exit = 2;
        }
        ledgers.push(ledger);
        if result.exit_code != 0 {
            break;
        }
    }

    Ok(VisualPreflightProjectBatch {
        final_exit,
        total_program_ms,
        ledgers,
        renderer_reuse_rows,
    })

}

fn execute_visual_preflight_general_audits(
    launcher: &mut Launcher,
    _state: &mut RunState,
    session: &VisualPreflightSession,
    batch: VisualPreflightProjectBatch,
) -> AppResult<VisualPreflightExecution> {
    let project_root = session.project_root.as_path();
    let plan = &session.plan;
    let projects = session.projects.as_slice();
    let aggregate_path = session.aggregate_path.as_path();
    let VisualPreflightProjectBatch {
        final_exit,
        total_program_ms,
        ledgers,
        renderer_reuse_rows,
    } = batch;
    append_visual_preflight_renderer_reuse_section(
        aggregate_path,
        &launcher.version,
        &renderer_reuse_rows,
        projects.len(),
    )?;

    append_visual_preflight_freshness_gate(
        aggregate_path,
        &launcher.version,
        projects,
        &ledgers,
        final_exit,
    )?;
    append_visual_preflight_rule_candidate_delta_audit(
        aggregate_path,
        &launcher.version,
        plan,
        projects,
        &ledgers,
        final_exit,
    )?;
    let clef_register_artifacts = append_clef_register_stability_audit_section(
        aggregate_path,
        &launcher.version,
        projects,
        project_root,
    )?;
    launcher.line(
        PresentationRole::Program,
        None,
        Attention::Neutral,
        Emphasis::Primary,
        &format!(
            "[EtuSchlopf] ClefRegisterStabilitySSOT: {}, Schluessel/Register-Thematik scoreweit regelbasiert auditiert",
            artefakt_count(clef_register_artifacts.len())
        ),
    );

    Ok(VisualPreflightExecution {
        final_exit,
        total_program_ms,
        ledgers,
    })

}

fn execute_visual_preflight_clef_audits(
    _launcher: &mut Launcher,
    state: &mut RunState,
    session: &VisualPreflightSession,
    execution: &mut VisualPreflightExecution,
) -> AppResult<()> {
    let fresh_rows = execution.ledgers.iter().filter(|row| row.is_fresh()).count();
    let ledger_green = !execution.ledgers.is_empty()
        && fresh_rows == execution.ledgers.len()
        && execution.ledgers.iter().all(|row| row.exit_code == 0);
    let renderer_green = matches!(
        state.renderer_outcome,
        RendererOutcome::NotObserved | RendererOutcome::Passed
    );
    let gate_green = execution.final_exit == 0 && ledger_green && renderer_green;

    let mut file = OpenOptions::new()
        .append(true)
        .open(session.aggregate_path.as_path())?;
    writeln!(file)?;
    writeln!(file, "VISUAL-PREFLIGHT-TYPED-PROJECT-RENDERER-GATE")?;
    writeln!(file, "------------------------------------------------")?;
    writeln!(file, "Version: {}", env!("CARGO_PKG_VERSION"))?;
    writeln!(file, "ProjectRows: {}", execution.ledgers.len())?;
    writeln!(file, "FreshProjectRows: {fresh_rows}")?;
    writeln!(file, "RendererOutcome: {}", state.renderer_outcome.label())?;
    writeln!(file, "Gate: {}", if gate_green { "green" } else { "blocked" })?;
    writeln!(file, "RetiredDetailFamilies: ClefRegister/PdfSurface accepted-closure carry and no-write preview recomputation")?;
    writeln!(file, "Boundary: typed project freshness plus renderer process truth; audit_only; music_mutation_no; master_apply_no")?;

    if !gate_green && execution.final_exit == 0 {
        state.audit_signal = AuditSignal::ExpectedBlocked;
        execution.final_exit = 2;
    }
    Ok(())
}

fn finalize_visual_preflight_execution(
    launcher: &mut Launcher,
    state: &mut RunState,
    session: &VisualPreflightSession,
    execution: VisualPreflightExecution,
) -> AppResult<i32> {
    let project_root = session.project_root.as_path();
    let plan = &session.plan;
    let projects = session.projects.as_slice();
    let aggregate_path = session.aggregate_path.as_path();
    let root_cleanup = cleanup_project_root_residue(project_root, Some("--visual-preflight"), false);
    append_root_cleanup_to_console_log("NachRun", &root_cleanup)?;

    state.input_arg = Some("--visual-preflight".to_string());
    state.result_file = Some(session.aggregate_path.clone());
    state.run_exit = execution.final_exit.to_string();
    state.program_ms = execution.total_program_ms.to_string();
    let strict_log_for_export = state.strict_log.clone();
    let clippy_exit_for_export = state.strict_clippy_exit.clone();
    let build_exit_for_export = state.build_exit.clone();
    append_run_diagnostic(state, "visual_preflight")?;
    let reference_guard_project = assistant_context_reference_guard_project(plan);
    let current_renderer_context = assistant_context_renderer_state_from_runtime(
        project_root,
        &reference_guard_project,
        &launcher.version,
    );
    state.renderer_outcome = reconcile_renderer_outcome_with_current_evidence(
        state.renderer_outcome,
        &execution.ledgers,
        &current_renderer_context,
        &launcher.version,
    );
    let export_context = AssistantExportBundleContext {
        project_root,
        aggregate_path,
        plan,
        projects,
        ledgers: &execution.ledgers,
        strict_log: strict_log_for_export.as_deref(),
        strict_test_log: state.strict_test_log.as_deref(),
        version: &launcher.version,
    };
    let export_status = AssistantExportRunStatus {
        final_exit: execution.final_exit,
        clippy_exit: &clippy_exit_for_export,
        build_exit: &build_exit_for_export,
        strict_test_exit: &state.strict_test_exit,
        strict_test_summary: &state.strict_test_summary,
        audit_signal: state.audit_signal,
        renderer_outcome: state.renderer_outcome,
        emergence_outcome: state.emergence_outcome,
        export_policy: state.export_policy,
    };
    create_assistant_export(AssistantExportRequest::VisualPreflight {
        context: export_context,
        status: export_status,
    })?;
    Ok(execution.final_exit)
}

impl VisualProjectRunLedger {
    fn is_fresh(&self) -> bool {
        self.fresh
    }

    fn project_row(&self) -> String {
        format!(
            "{} | {} | {} | {} | {} | {}",
            self.project,
            self.exit_code,
            self.result_file.display(),
            self.result_status,
            self.result_version,
            if self.fresh { "ja" } else { "nein" }
        )
    }
}

fn visual_project_run_ledger(
    project: &str,
    result_file: &Path,
    exit_code: i32,
    expected_version: &str,
    previous_pdf_fingerprint: Option<PdfVisualFingerprintSnapshot>,
    previous_pdf_visual_fingerprint: Option<PdfAutoVisualFingerprintSnapshot>,
) -> VisualProjectRunLedger {
    let mut result_exists = false;
    let mut result_status = "missing_result_file".to_string();
    let mut result_version = "missing".to_string();
    if let Ok(text) = fs::read_to_string(result_file) {
        result_exists = true;
        if let Some(status) = parse_result_status(&text) {
            result_status = status;
        } else {
            result_status = "missing_ergebnis_line".to_string();
        }
        if let Some(version) = parse_result_tool_version(&text) {
            result_version = version;
        } else {
            result_version = "missing_werkzeug_line".to_string();
        }
    }

    let fresh = exit_code == 0 && result_exists && result_version == expected_version;
    let verdict = if exit_code != 0 {
        "program_exit_red"
    } else if !result_exists {
        "stale_missing_result_file"
    } else if result_version == "missing_werkzeug_line" {
        "stale_missing_result_version"
    } else if result_version != expected_version {
        "stale_result_version"
    } else {
        "fresh_result_version_match"
    }
    .to_string();

    let current_pdf_fingerprint = parse_pdf_fingerprint_from_result(result_file);
    let current_pdf_visual_fingerprint = compute_pdf_visual_fingerprint_snapshot(&resolve_pdf_file(project));

    VisualProjectRunLedger {
        project: project.to_string(),
        exit_code,
        result_file: result_file.to_path_buf(),
        result_exists,
        result_status,
        result_version,
        fresh,
        verdict,
        shadow_run_started_at: None,
        previous_pdf_fingerprint,
        current_pdf_fingerprint,
        previous_pdf_visual_fingerprint,
        current_pdf_visual_fingerprint,
    }
}

fn parse_result_tool_version(text: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("Werkzeug:") else {
            continue;
        };
        let rest = rest.trim();
        return rest
            .strip_prefix("etuschlopf")
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);
    }
    None
}

fn parse_result_status(text: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("Ergebnis:") else {
            continue;
        };
        let value = rest.trim();
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }
    None
}

fn visual_preflight_freshness_gate_green(
    planned_project_rows: usize,
    project_rows: usize,
    stale_or_missing_rows: usize,
) -> bool {
    planned_project_rows > 0
        && project_rows == planned_project_rows
        && stale_or_missing_rows == 0
}

fn append_visual_preflight_freshness_gate(
    aggregate_path: &Path,
    expected_version: &str,
    planned_projects: &[String],
    ledgers: &[VisualProjectRunLedger],
    final_exit: i32,
) -> AppResult<()> {
    let fresh_rows = ledgers.iter().filter(|row| row.is_fresh()).count();
    let missing_or_stale_rows = ledgers.len().saturating_sub(fresh_rows);
    let incomplete_rows = planned_projects.len().saturating_sub(ledgers.len());
    let run_ledger_complete = ledgers.len() == planned_projects.len();
    let gate_green = visual_preflight_freshness_gate_green(
        planned_projects.len(),
        ledgers.len(),
        missing_or_stale_rows,
    );
    let mut file = OpenOptions::new().append(true).open(aggregate_path)?;
    writeln!(file)?;
    writeln!(file, "VISUAL-PREFLIGHT-FRESH-RUN-GATE")?;
    writeln!(file, "-------------------------------")?;
    writeln!(file, "ExpectedRunVersion: etuschlopf {expected_version}")?;
    writeln!(file, "PlannedProjectRows: {}", planned_projects.len())?;
    writeln!(file, "ProjectRows: {}", ledgers.len())?;
    writeln!(file, "FreshRows: {fresh_rows}")?;
    writeln!(file, "StaleOrMissingRows: {missing_or_stale_rows}")?;
    writeln!(file, "IncompleteRows: {incomplete_rows}")?;
    writeln!(file, "RunLedgerComplete: {}", if run_ledger_complete { "ja" } else { "nein" })?;
    writeln!(file, "Gate: {}", if gate_green { "green_fresh_project_results" } else { "red_stale_or_incomplete_project_results" })?;
    writeln!(file, "OverallRunExitAtFreshnessGate: {final_exit}")?;
    writeln!(file, "GateOwnership: project_result_freshness_only; renderer/audit exits are classified by their own gates.")?;
    writeln!(file, "ExitPolicy: stale/missing result => Exit 2; unrelated audit/renderer blockers keep their own exit without recoloring FreshGate.")?;
    writeln!(file, "Scope: freshness-only; no corpus, no music change, no etude special case.")?;
    writeln!(file)?;
    writeln!(file, "FRESH-RUN-DETAILS")?;
    writeln!(file, "-----------------")?;
    writeln!(file, "Projekt | Exit | Exists | ResultWerkzeug | ExpectedWerkzeug | Fresh | Verdict")?;
    for ledger in ledgers {
        writeln!(
            file,
            "{} | {} | {} | {} | {} | {} | {}",
            ledger.project,
            ledger.exit_code,
            if ledger.result_exists { "ja" } else { "nein" },
            ledger.result_version,
            expected_version,
            if ledger.fresh { "ja" } else { "nein" },
            ledger.verdict
        )?;
    }
    if incomplete_rows > 0 {
        for project in planned_projects.iter().skip(ledgers.len()) {
            writeln!(file, "{} | not_run | nein | missing | {} | nein | incomplete_after_previous_failure", project, expected_version)?;
        }
    }
    Ok(())
}

fn append_visual_preflight_rule_candidate_delta_audit(
    aggregate_path: &Path,
    expected_version: &str,
    plan: &VisualPreflightPlan,
    planned_projects: &[String],
    ledgers: &[VisualProjectRunLedger],
    final_exit: i32,
) -> AppResult<()> {
    let fresh_rows = ledgers.iter().filter(|row| row.is_fresh()).count();
    let ledger_complete = ledgers.len() == planned_projects.len();
    let target = plan.target_project.as_deref().unwrap_or("none");
    let locator = plan.target_locator.as_deref().unwrap_or("none");
    let score = plan.target_score.map(|value| value.to_string()).unwrap_or_else(|| "n/a".to_string());
    let candidate_bound = final_exit == 0
        && ledger_complete
        && fresh_rows == ledgers.len()
        && plan.target_score.unwrap_or(0) >= 58
        && plan.target_project.is_some();
    let mut file = OpenOptions::new().append(true).open(aggregate_path)?;
    writeln!(file)?;
    writeln!(file, "VISUAL-PREFLIGHT-RULE-CANDIDATE-DELTA-AUDIT-NO-MUSIC-CHANGE")?;
    writeln!(file, "----------------------------------------------------------------")?;
    writeln!(file, "Version: {expected_version}")?;
    writeln!(file, "Target: {target}; Locator: {locator}; Score: {score}")?;
    writeln!(file, "FreshLedger: {fresh_rows}/{}; Complete: {}", ledgers.len(), yes_no_bool(ledger_complete))?;
    writeln!(file, "CandidateBoundNoWrite: {}", yes_no_bool(candidate_bound))?;
    writeln!(file, "WriteAuthority: closed_no_runtime_apply_path")?;
    writeln!(file, "ReferenceProject: {}; project_neutral_context_only", plan.reference_project)?;
    writeln!(file, "Boundary: audit_only; apply_closed; render_xml_write_no; music_mutation_no; master_apply_no")?;
    Ok(())
}

#[derive(Clone)]
struct PdfVisualFingerprintSnapshot {
    sha256: String,
    bytes: String,
    pages: String,
}

fn parse_pdf_fingerprint_from_result(result_file: &Path) -> Option<PdfVisualFingerprintSnapshot> {
    let text = fs::read_to_string(result_file).ok()?;
    let mut sha256: Option<String> = None;
    let mut bytes: Option<String> = None;
    for line in text.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("PDF-Datei-Fingerprint:") else {
            continue;
        };
        for part in rest.split(';') {
            let part = part.trim();
            if let Some(value) = part.strip_prefix("PdfFileSha256") {
                let value = value.trim();
                if !value.is_empty() {
                    sha256 = Some(value.to_string());
                }
            } else if let Some(value) = part.strip_prefix("PdfBytes") {
                let value = value.trim();
                if !value.is_empty() {
                    bytes = Some(value.to_string());
                }
            }
        }
    }
    let sha256 = sha256?;
    let bytes = bytes.unwrap_or_else(|| "unknown".to_string());
    let pages = parse_pdf_pages_from_result_text(&text).unwrap_or_else(|| "unknown".to_string());
    Some(PdfVisualFingerprintSnapshot { sha256, bytes, pages })
}

fn parse_pdf_pages_from_result_text(text: &str) -> Option<String> {
    for line in text.lines() {
        let Some(pos) = line.find("PDF-Seiten") else {
            continue;
        };
        let suffix = &line[pos + "PDF-Seiten".len()..];
        if let Some(value) = first_unsigned_number(suffix) {
            return Some(value);
        }
    }
    None
}

fn first_unsigned_number(text: &str) -> Option<String> {
    let mut started = false;
    let mut value = String::new();
    for ch in text.chars() {
        if ch.is_ascii_digit() {
            started = true;
            value.push(ch);
        } else if started {
            break;
        }
    }
    if value.is_empty() { None } else { Some(value) }
}


#[derive(Clone)]
struct PdfAutoVisualFingerprintSnapshot {
    fingerprint: String,
    content_streams: usize,
    normalized_bytes: usize,
}

fn compute_pdf_visual_fingerprint_snapshot(pdf_file: &Path) -> Option<PdfAutoVisualFingerprintSnapshot> {
    let bytes = fs::read(pdf_file).ok()?;
    let streams = extract_pdf_content_like_streams(&bytes);
    if streams.is_empty() {
        return None;
    }
    let mut normalized = Vec::new();
    for stream in &streams {
        normalized.extend_from_slice(stream);
        normalized.push(b'\n');
    }
    let fingerprint = format!("fnv64:{:016x}", stable_fnv1a64(&normalized));
    Some(PdfAutoVisualFingerprintSnapshot {
        fingerprint,
        content_streams: streams.len(),
        normalized_bytes: normalized.len(),
    })
}

fn extract_pdf_content_like_streams(pdf_bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut search_from = 0usize;
    while let Some(stream_pos_rel) = find_bytes(&pdf_bytes[search_from..], b"stream") {
        let stream_pos = search_from + stream_pos_rel;
        let mut data_start = stream_pos + b"stream".len();
        if pdf_bytes.get(data_start) == Some(&b'\r') && pdf_bytes.get(data_start + 1) == Some(&b'\n') {
            data_start += 2;
        } else if pdf_bytes.get(data_start) == Some(&b'\n') || pdf_bytes.get(data_start) == Some(&b'\r') {
            data_start += 1;
        }
        let Some(end_rel) = find_bytes(&pdf_bytes[data_start..], b"endstream") else {
            break;
        };
        let data_end = data_start + end_rel;
        let payload = trim_pdf_stream_edges(&pdf_bytes[data_start..data_end]);
        if let Some(decoded) = decode_pdf_stream_payload(payload)
            && let Some(normalized) = normalize_visual_pdf_stream(&decoded)
        {
            out.push(normalized);
        }
        search_from = data_end + b"endstream".len();
    }
    out
}

fn trim_pdf_stream_edges(payload: &[u8]) -> &[u8] {
    let mut start = 0usize;
    let mut end = payload.len();
    while start < end && matches!(payload[start], b'\r' | b'\n') {
        start += 1;
    }
    while end > start && matches!(payload[end - 1], b'\r' | b'\n') {
        end -= 1;
    }
    &payload[start..end]
}

fn decode_pdf_stream_payload(payload: &[u8]) -> Option<Vec<u8>> {
    let mut decoded = Vec::new();
    let mut zlib = ZlibDecoder::new(payload);
    if zlib.read_to_end(&mut decoded).is_ok() && !decoded.is_empty() {
        return Some(decoded);
    }
    if payload.iter().any(|b| b.is_ascii_graphic() || matches!(b, b' ' | b'\n' | b'\r' | b'\t')) {
        Some(payload.to_vec())
    } else {
        None
    }
}

fn normalize_visual_pdf_stream(decoded: &[u8]) -> Option<Vec<u8>> {
    if !looks_like_pdf_visual_content_stream(decoded) {
        return None;
    }
    let mut out = Vec::with_capacity(decoded.len());
    let mut last_space = true;
    for &byte in decoded {
        let normalized = match byte {
            b'\r' | b'\n' | b'\t' | 0x0c | b' ' => b' ',
            0x20..=0x7e => byte,
            _ => continue,
        };
        if normalized == b' ' {
            if !last_space {
                out.push(b' ');
                last_space = true;
            }
        } else {
            out.push(normalized);
            last_space = false;
        }
    }
    while out.last() == Some(&b' ') {
        out.pop();
    }
    if out.len() < 16 { None } else { Some(out) }
}

fn looks_like_pdf_visual_content_stream(decoded: &[u8]) -> bool {
    let sample = String::from_utf8_lossy(decoded);
    let visual_tokens = [" BT", "ET", " Tf", " TJ", " Tj", " re", " m", " l", " c", " f", " S", " cm", " q", " Q"];
    let hits = visual_tokens.iter().filter(|token| sample.contains(*token)).count();
    hits >= 2 && (sample.contains("BT") || sample.contains(" re") || sample.contains(" m"))
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn stable_fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for &byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

fn auto_visual_shape_proxy_stable(ledger: &VisualProjectRunLedger) -> bool {
    let (Some(before_visual), Some(after_visual)) = (
        ledger.previous_pdf_visual_fingerprint.as_ref(),
        ledger.current_pdf_visual_fingerprint.as_ref(),
    ) else {
        return false;
    };
    let (Some(before_pdf), Some(after_pdf)) = (
        ledger.previous_pdf_fingerprint.as_ref(),
        ledger.current_pdf_fingerprint.as_ref(),
    ) else {
        return false;
    };
    before_visual.content_streams == after_visual.content_streams
        && before_visual.normalized_bytes == after_visual.normalized_bytes
        && before_pdf.bytes == after_pdf.bytes
        && before_pdf.pages == after_pdf.pages
}

fn auto_visual_changed_label(ledger: &VisualProjectRunLedger) -> &'static str {
    match (&ledger.previous_pdf_visual_fingerprint, &ledger.current_pdf_visual_fingerprint) {
        (Some(before), Some(after)) if before.fingerprint == after.fingerprint => "nein_auto_visual_fingerprint_identical",
        (Some(_), Some(_)) if auto_visual_shape_proxy_stable(ledger) => {
            "nein_shape_proxy_stable_strict_fingerprint_changed"
        }
        (Some(_), Some(_)) => "ja_auto_visual_fingerprint_changed",
        (None, Some(_)) => "unbekannt_no_prior_visual_fingerprint",
        (_, None) => "unbekannt_missing_current_visual_fingerprint",
    }
}


#[derive(Clone)]
struct PianisticCellNote {
    pitch_midi: Option<i32>,
    staff: usize,
    is_rest: bool,
}





#[derive(Clone)]
struct ClefRegisterPolicy {
    lower_staff: usize,
    min_lower_treble_span_notes: usize,
    max_clef_changes_per_measure_staff: usize,
    high_register_midi_floor: i32,
    accidental_pressure_per_measure_per_mille: usize,
}

impl ClefRegisterPolicy {
    fn summary(&self) -> String {
        format!(
            "SSOT lower_staff={} min_lower_treble_span_notes={} max_clef_changes_per_measure_staff={} high_register_midi_floor={} accidental_pressure_per_measure_per_mille={}",
            self.lower_staff,
            self.min_lower_treble_span_notes,
            self.max_clef_changes_per_measure_staff,
            self.high_register_midi_floor,
            self.accidental_pressure_per_measure_per_mille
        )
    }
}

fn clef_register_policy() -> ClefRegisterPolicy {
    ClefRegisterPolicy {
        lower_staff: 2,
        min_lower_treble_span_notes: 3,
        max_clef_changes_per_measure_staff: 2,
        high_register_midi_floor: 72,
        accidental_pressure_per_measure_per_mille: 330,
    }
}

#[derive(Clone)]
struct ClefRegisterEvent {
    sequence: usize,
    measure: usize,
    staff: usize,
    sign: String,
    line: String,
    octave_change: Option<i32>,
}

#[derive(Clone)]
struct ClefRegisterNote {
    sequence: usize,
    measure: usize,
    staff: usize,
    pitch_midi: Option<i32>,
    is_rest: bool,
    has_accidental: bool,
}

#[derive(Clone)]
struct ClefRegisterKeyEvent {
    measure: usize,
    fifths: i32,
}

struct ClefRegisterScoreAudit {
    project: String,
    source_path: PathBuf,
    source_exists: bool,
    clefs: Vec<ClefRegisterEvent>,
    notes: Vec<ClefRegisterNote>,
    key_events: Vec<ClefRegisterKeyEvent>,
    findings: Vec<ClefRegisterFinding>,
}

#[derive(Clone)]
struct ClefRegisterFinding {
    project: String,
    measure: usize,
    staff: usize,
    severity: &'static str,
    rule_id: &'static str,
    evidence: String,
    recommendation: String,
}

fn append_clef_register_stability_audit_section(
    aggregate_path: &Path,
    expected_version: &str,
    projects: &[String],
    project_root: &Path,
) -> AppResult<Vec<PathBuf>> {
    let policy = clef_register_policy();
    let out_dir = project_root.join("target").join("etuschlopf_clef_register_stability");
    fs::create_dir_all(&out_dir)?;

    let mut audits = Vec::new();
    for project in projects {
        audits.push(build_clef_register_score_audit(project, &policy));
    }

    let aggregate_txt = out_dir.join("clef_register_stability_aggregate.txt");
    write_clef_register_aggregate_txt(&aggregate_txt, expected_version, &policy, &audits)?;

    let artifacts = vec![aggregate_txt.clone()];

    let total_findings: usize = audits.iter().map(|audit| audit.findings.len()).sum();
    let red_findings = audits
        .iter()
        .flat_map(|audit| audit.findings.iter())
        .filter(|finding| finding.severity == "red")
        .count();
    let mut file = OpenOptions::new().append(true).open(aggregate_path)?;
    writeln!(file)?;
    writeln!(file, "CLEF-REGISTER-STABILITY-SSOT-AUDIT")?;
    writeln!(file, "---------------------------------")?;
    writeln!(file, "Status: v{expected_version} auditiert die gesamte Schluessel-/Register-Stabilitaet scoreweit und regelbasiert.")?;
    writeln!(file, "Scope: alle Visual-Preflight-Projekte; Etuede 6 bleibt Regression/Referenz; keine MusicMutation; kein MasterApply; kein produktiver MusicXML-Write.")?;
    writeln!(file, "SingleSourceOfTruth: {}", policy.summary())?;
    writeln!(file, "NoMeasureSpecificLogic: ja; keine Takt-18/22/24/25/32/33/39-Ausnahme im Praedikat. Wenn diese Takte erscheinen, dann nur weil generische Regeln anschlagen.")?;
    writeln!(file, "BehindBarsTruthCleanup: rote Hotspot-PDFs werden nicht als Freigabeartefakte ausgegeben; sie bleiben hoechstens interne technische Zeugen.")?;
    writeln!(file, "Findings: total {}; red {}.", total_findings, red_findings)?;
    for audit in &audits {
        let project_red = audit.findings.iter().filter(|finding| finding.severity == "red").count();
        writeln!(file, "{} | source_exists {} | clefs {} | notes {} | key_events {} | findings {} | red {} | source {}", audit.project, yes_no(audit.source_exists), audit.clefs.len(), audit.notes.len(), audit.key_events.len(), audit.findings.len(), project_red, audit.source_path.display())?;
    }
    writeln!(file)?;
    writeln!(file, "TOP-CLEF-REGISTER-FINDINGS")?;
    writeln!(file, "--------------------------")?;
    for finding in audits.iter().flat_map(|audit| audit.findings.iter()).take(40) {
        writeln!(file, "{} | M{} staff {} | {} | {} | {} | {}", finding.project, finding.measure, finding.staff, finding.severity, finding.rule_id, finding.evidence, finding.recommendation)?;
    }
    Ok(artifacts)
}

fn build_clef_register_score_audit(project: &str, policy: &ClefRegisterPolicy) -> ClefRegisterScoreAudit {
    let render_path = resolve_render_musicxml_file(project);
    let fallback_path = PathBuf::from(format!("{}.musicxml", project));
    let source_path = if render_path.exists() { render_path } else { fallback_path };
    let source_exists = source_path.exists();
    let (clefs, notes, key_events) = if source_exists {
        parse_clef_register_score(&source_path).unwrap_or_default()
    } else {
        (Vec::new(), Vec::new(), Vec::new())
    };
    let findings = evaluate_clef_register_findings(project, &clefs, &notes, &key_events, policy);
    ClefRegisterScoreAudit {
        project: project.to_string(),
        source_path,
        source_exists,
        clefs,
        notes,
        key_events,
        findings,
    }
}

fn parse_clef_register_score(path: &Path) -> AppResult<(Vec<ClefRegisterEvent>, Vec<ClefRegisterNote>, Vec<ClefRegisterKeyEvent>)> {
    let text = fs::read_to_string(path)?;
    let mut clefs = Vec::new();
    let mut notes = Vec::new();
    let mut key_events = Vec::new();
    let mut current_measure = 0usize;
    let mut sequence = 0usize;
    let mut in_note = false;
    let mut note_block = String::new();
    let mut in_clef = false;
    let mut clef_block = String::new();
    let mut in_key = false;
    let mut key_block = String::new();

    for line in text.lines() {
        if let Some(number) = parse_measure_number_from_line(line) {
            current_measure = number;
        }

        if line.contains("<key") {
            in_key = true;
            key_block.clear();
        }
        if in_key {
            key_block.push_str(line);
            key_block.push('\n');
            if line.contains("</key>") {
                if let Some(fifths) = extract_tag_text(&key_block, "fifths").and_then(|value| value.parse::<i32>().ok()) {
                    key_events.push(ClefRegisterKeyEvent { measure: current_measure, fifths });
                }
                in_key = false;
                key_block.clear();
            }
        } else if line.contains("<fifths>")
            && let Some(fifths) =
                extract_tag_text(line, "fifths").and_then(|value| value.parse::<i32>().ok())
        {
            key_events.push(ClefRegisterKeyEvent {
                measure: current_measure,
                fifths,
            });
        }

        if line.contains("<clef ") || line.contains("<clef>") {
            in_clef = true;
            clef_block.clear();
        }
        if in_clef {
            clef_block.push_str(line);
            clef_block.push('\n');
            if line.contains("</clef>") {
                if !clef_register_block_print_object_no(&clef_block) {
                    sequence = sequence.saturating_add(1);
                    clefs.push(parse_clef_register_event(&clef_block, current_measure, sequence));
                }
                in_clef = false;
                clef_block.clear();
            }
        }

        if line.contains("<note") && !line.contains("</note>") {
            in_note = true;
            note_block.clear();
        }
        if in_note {
            note_block.push_str(line);
            note_block.push('\n');
            if line.contains("</note>") {
                sequence = sequence.saturating_add(1);
                notes.push(parse_clef_register_note(&note_block, current_measure, sequence));
                in_note = false;
                note_block.clear();
            }
        } else if line.contains("<note") && line.contains("</note>") {
            sequence = sequence.saturating_add(1);
            notes.push(parse_clef_register_note(line, current_measure, sequence));
        }
    }
    Ok((clefs, notes, key_events))
}





fn evaluate_clef_register_findings(
    project: &str,
    clefs: &[ClefRegisterEvent],
    notes: &[ClefRegisterNote],
    key_events: &[ClefRegisterKeyEvent],
    policy: &ClefRegisterPolicy,
) -> Vec<ClefRegisterFinding> {
    let mut findings = Vec::new();
    let mut clefs_sorted = clefs.to_vec();
    clefs_sorted.sort_by_key(|clef| clef.sequence);

    for clef in &clefs_sorted {
        let next_same_staff_sequence = clefs_sorted
            .iter()
            .filter(|next| next.staff == clef.staff && next.sequence > clef.sequence)
            .map(|next| next.sequence)
            .min()
            .unwrap_or(usize::MAX);
        let span_notes: Vec<&ClefRegisterNote> = notes
            .iter()
            .filter(|note| note.staff == clef.staff && note.sequence > clef.sequence && note.sequence < next_same_staff_sequence)
            .collect();
        let non_rest: Vec<&ClefRegisterNote> = span_notes.iter().copied().filter(|note| !note.is_rest).collect();
        let pitch_values: Vec<i32> = non_rest.iter().filter_map(|note| note.pitch_midi).collect();
        let max_midi = pitch_values.iter().copied().max();
        let min_midi = pitch_values.iter().copied().min();
        let avg_midi = if pitch_values.is_empty() {
            None
        } else {
            Some(pitch_values.iter().sum::<i32>() / pitch_values.len() as i32)
        };
        if non_rest.is_empty() && clef.measure > 0 {
            findings.push(ClefRegisterFinding {
                project: project.to_string(),
                measure: clef.measure,
                staff: clef.staff,
                severity: "red",
                rule_id: "CLEF_EMPTY_SPAN",
                evidence: format!("clef {}{} staff {} has 0 non-rest notes before next clef", clef.sign, clef.line, clef.staff),
                recommendation: "remove or delay clef change unless it starts a real phrase".to_string(),
            });
        }
        if non_rest.len() == 1 && clef.measure > 0 {
            findings.push(ClefRegisterFinding {
                project: project.to_string(),
                measure: clef.measure,
                staff: clef.staff,
                severity: if clef.staff == policy.lower_staff && clef.sign == "G" { "red" } else { "yellow" },
                rule_id: "CLEF_SINGLE_NOTE_SPAN",
                evidence: format!("clef {}{} staff {} carries exactly 1 non-rest note; midi {:?}", clef.sign, clef.line, clef.staff, max_midi),
                recommendation: "prefer ledger/8va/register spelling over one-note clef twitch".to_string(),
            });
        }
        if clef.staff == policy.lower_staff && clef.sign == "G" && non_rest.len() < policy.min_lower_treble_span_notes && clef.measure > 0 {
            findings.push(ClefRegisterFinding {
                project: project.to_string(),
                measure: clef.measure,
                staff: clef.staff,
                severity: "red",
                rule_id: "LOWER_STAFF_TREBLE_TOO_SHORT",
                evidence: format!("lower staff treble span has {} notes; min {}; min_midi {:?}; max_midi {:?}; avg_midi {:?}", non_rest.len(), policy.min_lower_treble_span_notes, min_midi, max_midi, avg_midi),
                recommendation: "keep lower staff stable unless treble clef supports a real readable passage".to_string(),
            });
        }
        if clef.staff == policy.lower_staff && clef.sign == "G" && max_midi.unwrap_or(0) < policy.high_register_midi_floor && !non_rest.is_empty() {
            findings.push(ClefRegisterFinding {
                project: project.to_string(),
                measure: clef.measure,
                staff: clef.staff,
                severity: "yellow",
                rule_id: "LOWER_STAFF_TREBLE_REGISTER_WEAK",
                evidence: format!("lower staff treble span max_midi {:?} below high-register floor {}", max_midi, policy.high_register_midi_floor),
                recommendation: "do not switch clef just to avoid a small number of ledger lines".to_string(),
            });
        }
        if let Some(octave) = clef.octave_change {
            findings.push(ClefRegisterFinding {
                project: project.to_string(),
                measure: clef.measure,
                staff: clef.staff,
                severity: "yellow",
                rule_id: "CLEF_OCTAVE_CHANGE_REVIEW",
                evidence: format!("clef {}{} staff {} has clef-octave-change {}; span notes {}", clef.sign, clef.line, clef.staff, octave, non_rest.len()),
                recommendation: "allow only if octave clef is clearer than ordinary clef plus local 8va/8vb".to_string(),
            });
        }
    }

    let mut clef_count_by_measure_staff: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    for clef in &clefs_sorted {
        *clef_count_by_measure_staff.entry((clef.measure, clef.staff)).or_insert(0) += 1;
    }
    for ((measure, staff), count) in clef_count_by_measure_staff {
        if measure > 0 && count > policy.max_clef_changes_per_measure_staff {
            findings.push(ClefRegisterFinding {
                project: project.to_string(),
                measure,
                staff,
                severity: "red",
                rule_id: "CLEF_CHURN_MEASURE_STAFF",
                evidence: format!("{} clef declarations in same measure/staff; max {}", count, policy.max_clef_changes_per_measure_staff),
                recommendation: "choose a stable clef plan for the passage before any local engraving rewrite".to_string(),
            });
        }
    }

    let mut accidental_by_measure: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    for note in notes.iter().filter(|note| !note.is_rest) {
        let entry = accidental_by_measure.entry(note.measure).or_insert((0, 0));
        entry.0 = entry.0.saturating_add(1);
        if note.has_accidental {
            entry.1 = entry.1.saturating_add(1);
        }
    }
    for (measure, (note_count, accidental_count)) in accidental_by_measure {
        if note_count >= 6 {
            let per_mille = accidental_count.saturating_mul(1000) / note_count.max(1);
            if per_mille >= policy.accidental_pressure_per_measure_per_mille {
                findings.push(ClefRegisterFinding {
                    project: project.to_string(),
                    measure,
                    staff: 0,
                    severity: "yellow",
                    rule_id: "KEY_ACCIDENTAL_PRESSURE_REVIEW",
                    evidence: format!("{} accidentals/altered pitches over {} notes = {} per mille", accidental_count, note_count, per_mille),
                    recommendation: "review local key spelling/key signature before fixing clefs by hand".to_string(),
                });
            }
        }
    }

    let mut unique_key_events = Vec::<i32>::new();
    for event in key_events {
        if !unique_key_events.contains(&event.fifths) {
            unique_key_events.push(event.fifths);
        }
    }
    if unique_key_events.len() > 1 {
        let first_measure = key_events.first().map(|event| event.measure).unwrap_or(0);
        findings.push(ClefRegisterFinding {
            project: project.to_string(),
            measure: first_measure,
            staff: 0,
            severity: "yellow",
            rule_id: "KEY_SIGNATURE_CHURN_REVIEW",
            evidence: format!("multiple key signatures/fifths in score: {:?}", unique_key_events),
            recommendation: "key policy must be settled globally before local spelling repairs".to_string(),
        });
    }

    findings.sort_by(|a, b| severity_rank(a.severity).cmp(&severity_rank(b.severity)).then(a.project.cmp(&b.project)).then(a.measure.cmp(&b.measure)).then(a.rule_id.cmp(b.rule_id)));
    findings
}

fn severity_rank(severity: &str) -> usize {
    match severity {
        "red" => 0,
        "yellow" => 1,
        _ => 2,
    }
}

fn write_clef_register_aggregate_txt(
    path: &Path,
    expected_version: &str,
    policy: &ClefRegisterPolicy,
    audits: &[ClefRegisterScoreAudit],
) -> AppResult<()> {
    let mut file = File::create(path)?;
    writeln!(file, "ETUSCHLOPF CLEF-REGISTER-STABILITY-SSOT-AUDIT")?;
    writeln!(file, "Version: {expected_version}")?;
    writeln!(file, "Policy: {}", policy.summary())?;
    writeln!(file, "NoMeasureSpecificLogic: ja")?;
    writeln!(file, "MusicMutation: nein")?;
    writeln!(file, "MasterApply: nein")?;
    writeln!(file, "BehindBarsEvidenceExport: red_hotspot_pdfs_suppressed")?;
    writeln!(file)?;
    for audit in audits {
        let red = audit.findings.iter().filter(|finding| finding.severity == "red").count();
        let yellow = audit.findings.iter().filter(|finding| finding.severity == "yellow").count();
        writeln!(file, "PROJECT {}", audit.project)?;
        writeln!(file, "  Source: {}", audit.source_path.display())?;
        writeln!(file, "  SourceExists: {}", yes_no(audit.source_exists))?;
        writeln!(file, "  Clefs: {}", audit.clefs.len())?;
        writeln!(file, "  Notes: {}", audit.notes.len())?;
        writeln!(file, "  KeyEvents: {}", audit.key_events.len())?;
        writeln!(file, "  Findings: {} red {} yellow {}", audit.findings.len(), red, yellow)?;
        for finding in audit.findings.iter().take(80) {
            writeln!(file, "    M{} staff {} | {} | {} | {} | {}", finding.measure, finding.staff, finding.severity, finding.rule_id, finding.evidence, finding.recommendation)?;
        }
        writeln!(file)?;
    }
    Ok(())
}

























































































































































































fn yes_no(value: bool) -> &'static str {
    if value { "ja" } else { "nein" }
}





fn sanitize_checkpoint_component(value: &str) -> String {
    let mut out = String::new();
    let mut previous_separator = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            previous_separator = false;
        } else if !previous_separator {
            out.push('_');
            previous_separator = true;
        }
    }
    let cleaned = out.trim_matches('_').to_string();
    if cleaned.is_empty() {
        "target".to_string()
    } else {
        cleaned
    }
}


fn parse_measure_number_from_line(line: &str) -> Option<usize> {
    let marker = "<measure";
    let pos = line.find(marker)?;
    let slice = &line[pos..];
    let key = "number=\"";
    let start = slice.find(key)? + key.len();
    let rest = &slice[start..];
    let end = rest.find('"')?;
    rest[..end].trim().parse::<usize>().ok()
}


fn parse_pianistic_cell_note_block(block: &str, _measure: usize, _ordinal: usize) -> PianisticCellNote {
    let is_rest = block.contains("<rest") || block.contains("<rest/");
    let staff = extract_tag_text(block, "staff")
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(1);
    let step = extract_tag_text(block, "step").unwrap_or_else(|| "R".to_string());
    let alter = extract_tag_text(block, "alter")
        .and_then(|value| value.trim().parse::<i32>().ok())
        .unwrap_or(0);
    let octave = extract_tag_text(block, "octave")
        .and_then(|value| value.trim().parse::<i32>().ok())
        .unwrap_or(4);
    let pitch_midi = (!is_rest).then(|| midi_from_pitch(&step, alter, octave));
    PianisticCellNote { pitch_midi, staff, is_rest }
}


fn extract_tag_text(block: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = block.find(&open)? + open.len();
    let end = block[start..].find(&close)? + start;
    Some(block[start..end].trim().to_string())
}


fn midi_from_pitch(step: &str, alter: i32, octave: i32) -> i32 {
    let semitone = match step.chars().next().unwrap_or('C') {
        'C' | 'c' => 0,
        'D' | 'd' => 2,
        'E' | 'e' => 4,
        'F' | 'f' => 5,
        'G' | 'g' => 7,
        'A' | 'a' => 9,
        'B' | 'b' | 'H' | 'h' => 11,
        _ => 0,
    };
    (octave + 1) * 12 + semitone + alter
}


fn clef_register_block_print_object_no(block: &str) -> bool {
    block.contains("print-object=\"no\"") || block.contains("print-object='no'")
}


fn parse_clef_register_event(block: &str, measure: usize, sequence: usize) -> ClefRegisterEvent {
    let first_line = block.lines().find(|line| line.contains("<clef ") || line.contains("<clef>")).unwrap_or(block);
    let staff = extract_xml_attr_usize(first_line, "number").unwrap_or(1);
    let sign = extract_tag_text(block, "sign").unwrap_or_else(|| "unknown".to_string());
    let line = extract_tag_text(block, "line").unwrap_or_else(|| "?".to_string());
    let octave_change = extract_tag_text(block, "clef-octave-change").and_then(|value| value.parse::<i32>().ok());
    ClefRegisterEvent { sequence, measure, staff, sign, line, octave_change }
}


fn parse_clef_register_note(block: &str, measure: usize, sequence: usize) -> ClefRegisterNote {
    let cell_note = parse_pianistic_cell_note_block(block, measure, sequence);
    let has_accidental = block.contains("<accidental>")
        || extract_tag_text(block, "alter").and_then(|value| value.parse::<i32>().ok()).unwrap_or(0) != 0;
    ClefRegisterNote {
        sequence,
        measure,
        staff: cell_note.staff,
        pitch_midi: cell_note.pitch_midi,
        is_rest: cell_note.is_rest,
        has_accidental,
    }
}


fn extract_xml_attr_usize(line: &str, attr: &str) -> Option<usize> {
    let key = format!("{attr}=\"");
    let start = line.find(&key)? + key.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    rest[..end].trim().parse::<usize>().ok()
}


#[derive(Clone, Debug)]
struct CorpusEmergenceSelectionState {
    report_exists: bool,
    report_version: Option<String>,
    run_id: Option<String>,
    generated_unix_seconds: Option<u64>,
    full_corpus: bool,
    project_count: usize,
    reported_current_version_project_count: usize,
    current_version_project_count: usize,
    project_evidence_complete: bool,
    manifest_exists: bool,
    manifest_completed: bool,
    manifest_valid: bool,
    manifest_reason: String,
    manifest_project_files_valid: usize,
    manifest_project_files_expected: usize,
    report_hash_valid: bool,
    report_bound_to_manifest: bool,
    evidence_contract: String,
    projects: Vec<String>,
    selection_eligible: bool,
    fresh_for_current_version: bool,
    status: String,
    reason: String,
    selected_seed: Option<VisualHotspotSeed>,
}

impl CorpusEmergenceSelectionState {
    fn missing() -> Self {
        Self {
            report_exists: false,
            report_version: None,
            run_id: None,
            generated_unix_seconds: None,
            full_corpus: false,
            project_count: 0,
            reported_current_version_project_count: 0,
            current_version_project_count: 0,
            project_evidence_complete: false,
            manifest_exists: false,
            manifest_completed: false,
            manifest_valid: false,
            manifest_reason: "corpus_run_manifest_missing".to_string(),
            manifest_project_files_valid: 0,
            manifest_project_files_expected: 0,
            report_hash_valid: false,
            report_bound_to_manifest: false,
            evidence_contract: "hash_bound_corpus_run_manifest_v2".to_string(),
            projects: Vec::new(),
            selection_eligible: false,
            fresh_for_current_version: false,
            status: "no_current_target_awaiting_fresh_full_corpus_emergence".to_string(),
            reason: "corpus_emergence_report_missing".to_string(),
            selected_seed: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EmergenceTargetClass {
    NoCurrentTarget,
    BehindBarsRuleCheckHotspot,
}

impl EmergenceTargetClass {
    fn as_str(self) -> &'static str {
        match self {
            Self::NoCurrentTarget => "none",
            Self::BehindBarsRuleCheckHotspot => "behind_bars_rule_check_hotspot",
        }
    }

    fn behind_bars_rule_check_required(self) -> bool {
        matches!(self, Self::BehindBarsRuleCheckHotspot)
    }

    fn rule_candidate(self) -> bool {
        false
    }

    fn write_candidate(self) -> bool {
        false
    }

    fn permission_contract(self) -> &'static str {
        match self {
            Self::NoCurrentTarget => "no_current_target_no_rule_check_or_write_authority",
            Self::BehindBarsRuleCheckHotspot => "behind_bars_check_only_no_rule_or_write_authority",
        }
    }
}

#[derive(Clone, Debug)]
struct VisualHotspotSeed {
    project: String,
    locator: String,
    score: usize,
}

fn corpus_prefixed_value(text: &str, prefix: &str) -> Option<String> {
    text.lines()
        .find_map(|line| line.trim().strip_prefix(prefix))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn read_corpus_emergence_selection_state(
    project_root: &Path,
    expected_version: &str,
) -> CorpusEmergenceSelectionState {
    let path = project_root.join("corpus_emergence.result.txt");
    let Ok(text) = fs::read_to_string(&path) else {
        return CorpusEmergenceSelectionState::missing();
    };
    let validation = validate_corpus_run_manifest(project_root, expected_version);
    let manifest_exists = validation.manifest_path.exists();
    let manifest_completed = validation.manifest.completed;
    let report_version = corpus_prefixed_value(&text, "Werkzeug: etuschlopf ");
    let run_id = corpus_prefixed_value(&text, "RunId: ");
    let generated_unix_seconds = corpus_prefixed_value(&text, "GeneratedUnixSeconds: ")
        .and_then(|value| value.parse::<u64>().ok());
    let full_corpus = corpus_prefixed_value(&text, "FullCorpus: ")
        .map(|value| value.eq_ignore_ascii_case("ja") || value.eq_ignore_ascii_case("yes"))
        .unwrap_or(false);
    let project_count = corpus_prefixed_value(&text, "ProjectCount: ")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    let reported_current_version_project_count = corpus_prefixed_value(&text, "CurrentVersionProjectCount: ")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    let projects = corpus_prefixed_value(&text, "Projects: ")
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| validation.manifest.expected_projects.clone());
    let selection_eligible = corpus_prefixed_value(&text, "SelectionEligible: ")
        .map(|value| value.eq_ignore_ascii_case("ja") || value.eq_ignore_ascii_case("yes"))
        .unwrap_or(false);
    let version_matches = report_version.as_deref() == Some(expected_version);
    let report_bound_to_manifest = validation.report_file_valid
        && run_id.as_deref() == Some(validation.manifest.run_id.as_str())
        && report_version.as_deref() == Some(validation.manifest.version.as_str())
        && validation
            .manifest
            .report
            .as_ref()
            .map(|report| report.file.as_str() == "corpus_emergence.result.txt")
            .unwrap_or(false);
    let current_version_project_count = validation.project_files_valid;
    let expected_project_count = validation.manifest.expected_project_count;
    let project_evidence_complete = expected_project_count > 0
        && validation.project_files_valid == expected_project_count
        && validation.project_files_expected == expected_project_count;
    let manifest_valid = validation.valid && report_bound_to_manifest;
    let fresh_for_current_version = version_matches
        && full_corpus
        && selection_eligible
        && project_count == expected_project_count
        && reported_current_version_project_count == expected_project_count
        && project_evidence_complete
        && manifest_valid;
    let selected_seed = if fresh_for_current_version {
        visual_hotspot_seed_from_corpus_report_text(&text)
    } else {
        None
    };
    let (status, reason) = if !manifest_exists {
        (
            "no_current_target_awaiting_fresh_full_corpus_emergence".to_string(),
            "corpus_run_manifest_missing".to_string(),
        )
    } else if !manifest_completed {
        (
            "no_current_target_awaiting_fresh_full_corpus_emergence".to_string(),
            "corpus_run_manifest_incomplete".to_string(),
        )
    } else if !validation.valid {
        (
            "no_current_target_awaiting_fresh_full_corpus_emergence".to_string(),
            validation.reason.clone(),
        )
    } else if !report_bound_to_manifest {
        (
            "no_current_target_awaiting_fresh_full_corpus_emergence".to_string(),
            "corpus_report_not_bound_to_manifest_run_id_and_hash".to_string(),
        )
    } else if !version_matches {
        (
            "no_current_target_awaiting_fresh_full_corpus_emergence".to_string(),
            "corpus_report_version_mismatch".to_string(),
        )
    } else if !full_corpus || project_count != expected_project_count {
        (
            "no_current_target_awaiting_fresh_full_corpus_emergence".to_string(),
            "corpus_report_not_full_expected_project_run".to_string(),
        )
    } else if reported_current_version_project_count != expected_project_count
        || !project_evidence_complete
    {
        (
            "no_current_target_awaiting_fresh_full_corpus_emergence".to_string(),
            "corpus_manifest_project_evidence_not_complete".to_string(),
        )
    } else if !selection_eligible {
        (
            "no_current_target_awaiting_fresh_full_corpus_emergence".to_string(),
            "corpus_report_selection_not_eligible".to_string(),
        )
    } else if selected_seed.is_none() {
        (
            "fresh_full_corpus_without_selectable_visual_hotspot".to_string(),
            "fresh_full_corpus_has_no_visual_hotspot_seed".to_string(),
        )
    } else {
        (
            "fresh_full_corpus_emergence_target_selected".to_string(),
            "fresh_hash_bound_expected_project_corpus_current_version".to_string(),
        )
    };
    CorpusEmergenceSelectionState {
        report_exists: true,
        report_version,
        run_id,
        generated_unix_seconds,
        full_corpus,
        project_count,
        reported_current_version_project_count,
        current_version_project_count,
        project_evidence_complete,
        manifest_exists,
        manifest_completed,
        manifest_valid,
        manifest_reason: validation.reason,
        manifest_project_files_valid: validation.project_files_valid,
        manifest_project_files_expected: validation.project_files_expected,
        report_hash_valid: validation.report_file_valid,
        report_bound_to_manifest,
        evidence_contract: "hash_bound_corpus_run_manifest_v2".to_string(),
        projects,
        selection_eligible,
        fresh_for_current_version,
        status,
        reason,
        selected_seed,
    }
}

fn visual_preflight_projects(project_root: &Path, expected_version: &str) -> VisualPreflightPlan {
    let emergence = read_corpus_emergence_selection_state(project_root, expected_version);
    let Some(seed) = emergence.selected_seed.clone() else {
        return VisualPreflightPlan {
            projects: Vec::new(),
            target_project: None,
            target_locator: None,
            target_score: None,
            target_class: EmergenceTargetClass::NoCurrentTarget,
            selector_source: "none_fresh_full_corpus_required".to_string(),
            selector_status: emergence.status.clone(),
            reference_project: "none_project_neutral_renderer_evidence_selection".to_string(),
            emergence,
        };
    };

    let projects = vec![seed.project.clone()];

    VisualPreflightPlan {
        projects,
        target_project: Some(seed.project.clone()),
        target_locator: Some(seed.locator),
        target_score: Some(seed.score),
        target_class: EmergenceTargetClass::BehindBarsRuleCheckHotspot,
        selector_source:
            "fresh_hash_bound_corpus_run_manifest_v2:corpus_emergence.result.txt:CORPUS-VISUAL-PDF-HOTSPOT-RANKING-NO-APPLY"
                .to_string(),
        selector_status: emergence.status.clone(),
        reference_project: seed.project,
        emergence,
    }
}

fn yes_no_bool(value: bool) -> &'static str {
    if value { "ja" } else { "nein" }
}



fn assistant_context_reference_guard_project(plan: &VisualPreflightPlan) -> String {
    plan.reference_project.clone()
}













fn visual_hotspot_seed_from_corpus_report_text(text: &str) -> Option<VisualHotspotSeed> {
    let section_start = text.find("CORPUS-VISUAL-PDF-HOTSPOT-RANKING-NO-APPLY")?;
    let section = &text[section_start..];
    let mut in_rows = false;
    for line in section.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Project |") {
            in_rows = true;
            continue;
        }
        if !in_rows {
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with("NextConcreteBehindBarsCheck:") || trimmed.starts_with("Urteil:") {
            break;
        }
        let cells: Vec<&str> = trimmed.split('|').map(|cell| cell.trim()).collect();
        if cells.len() < 12 || cells[0].eq_ignore_ascii_case("Project") {
            continue;
        }
        let score = cells.get(3).and_then(|value| value.parse::<usize>().ok()).unwrap_or(0);
        let locator = canonical_visual_runtime_locator_from_cells(&cells)
            .unwrap_or_else(|| cells.get(11).copied().unwrap_or("n/a").to_string());
        return Some(VisualHotspotSeed {
            project: cells[0].to_string(),
            locator,
            score,
        });
    }
    None
}

fn canonical_visual_runtime_locator_from_cells(cells: &[&str]) -> Option<String> {
    cells
        .iter()
        .map(|cell| cell.trim())
        .find(|cell| is_runtime_measure_locator(cell))
        .map(|cell| cell.to_string())
}

fn is_runtime_measure_locator(value: &str) -> bool {
    let trimmed = value.trim();
    let Some(rest) = trimmed.strip_prefix('M').or_else(|| trimmed.strip_prefix('m')) else {
        return false;
    };
    !rest.is_empty() && rest.chars().all(|ch| ch.is_ascii_digit())
}

