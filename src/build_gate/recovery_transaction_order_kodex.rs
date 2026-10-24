// r1076 BootstrapCacheShortPathClosure: synchronizes the owned Cargo cache-tag contract to target\\bc while recovery ordering and package-only rollback clean remain unchanged.
// r1054 TakeEvidenceFoundationPrewire: binds Incoming candidate-intent preservation, canonical Acceptance before advisory Evidence, non-authoritative Evidence I/O failure, and final transaction cleanup without changing reject/accept/music authority.
// r926 IncrementalValidationCacheRetentionClosure: retires destructive post-Take package cleaning while preserving recovery-first ordering and binds retained derived Cargo cache reuse to source-signature + Cargo fingerprint + executable-FNV verification; changed source still requires compilation.\n// r860 KodexDuplicateFullBytePinRetirementAndProofDependencyEssenceClosure: retires Recovery's three exact whole-file pins duplicated by identity_gate_trust_anchor while preserving all recovery-order and ownership semantics.
// r417 RunCmd-Pin-Closure: r416 was fully rolled back because this recovery owner still pinned the pre-retirement run.cmd bytes. r417 keeps the cache-retention threshold retirement unchanged and rebinds only the existing run.cmd recovery-boundary fingerprint to the final bytes; recovery-first ordering, exact two-root Cargo-cache ownership, CACHEDIR.TAG validation and package-only cargo clean remain fail-closed.
// r378 Pin-Koordination: mechanischer Kodex-Clippy-Cleanup bindet die bereits gepinnten PublicArtifactRecovery- und FormalGreenContract-Projektionen aus finalen Bytes neu; Recovery- und Commit-Semantik bleiben unveraendert.

pub(super) struct RecoveryTransactionOrderInputs<'a> {
    pub(super) public_artifact_recovery_text: &'a str,
    pub(super) formal_green_contract_text: &'a str,
    pub(super) run_cmd_text: &'a str,
    pub(super) take_bootstrap_text: &'a str,
    pub(super) take_change_control_text: &'a str,
    pub(super) semantic_capability_text: &'a str,
    pub(super) launcher_text: &'a str,
    pub(super) launcher_build_control_text: &'a str,
    pub(super) launcher_assistant_export_text: &'a str,
    pub(super) cargo_target_cache_tag_kodex_contract: &'a str,
}

pub(super) fn validate_recovery_transaction_order_kodex(
    inputs: RecoveryTransactionOrderInputs<'_>,
) -> Result<(), String> {
    let RecoveryTransactionOrderInputs {
        public_artifact_recovery_text,
        formal_green_contract_text,
        run_cmd_text,
        take_bootstrap_text,
        take_change_control_text,
        semantic_capability_text,
        launcher_text,
        launcher_build_control_text,
        launcher_assistant_export_text,
        cargo_target_cache_tag_kodex_contract,
    } = inputs;

    const CARGO_TARGET_CACHE_TAG_KODEX_CONTRACT: &str =
        "OwnedCargoTargetCacheTagV3:target|target\\bc|CACHEDIR.TAG|existing=untouched|validation=cargo";

    if cargo_target_cache_tag_kodex_contract != CARGO_TARGET_CACHE_TAG_KODEX_CONTRACT {
        return Err(format!(
            "CARGO TARGET CACHE TAG OWNER CONTRACT CHANGED: expected {CARGO_TARGET_CACHE_TAG_KODEX_CONTRACT:?}, got {cargo_target_cache_tag_kodex_contract:?}. Explicit-target cleanup ownership must stay synchronized between build owner and recovery kodex."
        ));
    }

    let legacy_same_version_token_path_present = take_bootstrap_text
        .contains("const SAME_VERSION_TAKE_CONTINUATION_ENV")
        || take_bootstrap_text.contains(
            "fn same_version_build_continuation_token(\n    project_root: &Path,",
        )
        || take_bootstrap_text.contains(
            "same_version_build_continuation_token(&project_root, &current_version)",
        );

    if !take_change_control_text.contains("validate_complete_incoming_tree")
        || !take_change_control_text.contains("validate_incoming_change_dependencies")
        || !take_change_control_text.contains("verify_installed_tree_matches_source")
        || !take_change_control_text.contains("WATCH_PROJECT_DIR")
        || !take_change_control_text.contains("WATCH_PROJECT_MANIFEST_RELATIVE_PATH")
        || !take_change_control_text.contains("WATCH_PROJECT_LOCK_RELATIVE_PATH")
        || !take_change_control_text.contains("CalibrationOnly")
        || !take_change_control_text.contains("OutcomeProjection")
        || !take_change_control_text.contains("RefactorOnly")
        || !take_change_control_text.contains("validate_pending_transaction_semantics")
        || !semantic_capability_text.contains("validate_semantic_change")
        || !semantic_capability_text.contains("WATCH_PROJECT_MANIFEST_PATH")
        || !semantic_capability_text.contains("WATCH_PROJECT_LOCK_PATH")
        || !semantic_capability_text.contains("outcome_projection_allowed")
        || !semantic_capability_text.contains("executable_master_source_commit_surface")
        || !semantic_capability_text.contains("Writer-Faehigkeit und Schwellenwertaenderung")
        || !semantic_capability_text.contains("KodexNegativeCheck")
        || !semantic_capability_text.contains("ProofReentryReader")
        || !take_bootstrap_text.contains("struct TakeTransaction")
        || !take_bootstrap_text.contains("exact-replay hard")
        || !take_bootstrap_text.contains("prior-chain diagnostics")
        || !take_bootstrap_text.contains("TAKE-TRANSACTION ROLLBACK")
        || !take_bootstrap_text.contains("ChangeDependencyPreflight")
        || !take_bootstrap_text.contains("validate_changed_watch_project")
        || !take_bootstrap_text.contains("WATCH-PROJECT-PREFLIGHT GRUEN")
        || !take_bootstrap_text.contains("--manifest-path")
        || !take_bootstrap_text.contains("--locked")
        || !take_bootstrap_text.contains("fn resolve_root_cargo_lock(")
        || !take_bootstrap_text.contains("ROOT-CARGO-LOCK RESOLUTION GRUEN")
        || !take_bootstrap_text.contains(".arg(\"check\")")
        || !take_bootstrap_text.contains("LOCAL_TRANSACTIONAL_ROOT_FILES")
        || !take_bootstrap_text.contains("CARGO_TARGET_DIR")
        || !take_bootstrap_text.contains("recover_pending_transaction")
        || !take_bootstrap_text.contains("recover_pending_public_artifacts")
        || !take_bootstrap_text.contains("--recover-pending-public-artifacts")
        || !take_bootstrap_text.contains("identical_take_continuation_args")
        || !take_bootstrap_text.contains("Ein-Befehl-Fortsetzung")
        || !take_bootstrap_text.contains("run_cmd_command(&project_root, &clean_args)")
        || take_bootstrap_text.contains("ETUSCHLOPF_SAME_VERSION_TAKE_CONTINUATION")
        || legacy_same_version_token_path_present
        || take_bootstrap_text.contains("Naechster Befehl ohne --take")
        || !launcher_text.contains("enum BuildPlan")
        || !launcher_text.contains("enum ValidationPlan")
        || !launcher_text.contains("struct LauncherPlan")
        || !launcher_text.contains("fn plan_launcher")
        || !launcher_text.contains("RunRequest::DeepClippy")
        || !launcher_text.contains("execute_deep_clippy_diagnostic")
        || !launcher_text.contains("enum RunRequest")
        || !launcher_text.contains("struct ExecutionRequest")
        || !launcher_text.contains("enum RunTarget")
        || !launcher_text.contains("enum BuildPreference")
        || !launcher_text.contains("enum LauncherTestPlan")
        || launcher_text.contains("take_mode: bool")
        || launcher_text.contains("force: bool")
        || launcher_text.contains("visual_preflight_gate: bool")
        || launcher_text.contains("visual_preflight: bool")
        || launcher_text.contains("assistant_export_pdf: bool")
        || !launcher_text.contains("struct ArtifactAvailability")
        || !launcher_text.contains("enum BuildOutcome")
        || !launcher_text.contains("build_outcome: BuildOutcome")
        || !launcher_text.contains("fn initialize_run_state")
        || !launcher_text.contains("fn materialize_launcher_build")
        || !launcher_text.contains("fn run_integrated_launcher_tests")
        || !launcher_text.contains("fn execute_corpus_target")
        || !launcher_text.contains("fn execute_project_target")
        || !launcher_text.contains("enum CachePathOutcome")
        || !launcher_text.contains("struct CacheCleanupEntry")
        || launcher_text.contains("bootstrap_incremental_deleted: bool")
        || !launcher_text.contains("#[path = \"etuschlopf_launcher/assistant_export.rs\"]")
        || !launcher_assistant_export_text.contains("enum AssistantExportRequest")
        || !launcher_assistant_export_text.contains("fn create_assistant_export")
        || !launcher_assistant_export_text.contains("AssistantExportRequest::Normal")
        || !launcher_assistant_export_text.contains("AssistantExportRequest::Corpus")
        || !launcher_assistant_export_text.contains("AssistantExportRequest::VisualPreflight")
        || !launcher_assistant_export_text.contains("fn assistant_context_required_root_files_for_ledgers")
        || launcher_text.contains("fn assistant_context_required_root_files_for_ledgers")
        || launcher_text.contains("build_profile: String")
        || launcher_text.contains("build_decision: String")
        || launcher_text.contains("if args.deep_clippy")
        || !launcher_text.contains("build_plan_from_verification")
        || !launcher_text.contains("profile_build_artifact_is_verified")
        || !launcher_text.contains("write_successful_build_stamp")
        || launcher_text.contains("abort_if_same_version_already_built")
        || launcher_text.contains("ETUSCHLOPF_SAME_VERSION_TAKE_CONTINUATION")
        || !launcher_text.contains("recover_pending_public_artifacts")
        || !launcher_text.contains("is_public_artifact_transaction_eligible")
        || !launcher_text.contains("let RunTarget::Project(project_name) = &request.target else")
        || !launcher_text.contains("begin_public_artifact_transaction")
        || !launcher_text.contains("bind_corpus_run_renderer_outcome")
        || !launcher_text.contains("renderer_outcome_observed")
        || !launcher_text.contains("commit_public_artifact_transaction")
        || !launcher_text.contains("restore_public_artifact_transaction")
        || !public_artifact_recovery_text.contains("JOURNAL_SCHEMA_VERSION: u32 = 2")
        || !public_artifact_recovery_text.contains("begin_public_artifact_transaction")
        || !public_artifact_recovery_text.contains("commit_public_artifact_transaction")
        || !public_artifact_recovery_text.contains("public_artifact_paths")
        || !public_artifact_recovery_text.contains("validate_project_name")
        || !public_artifact_recovery_text.contains("validate_all_restore_inputs_before_mutation")
        || !public_artifact_recovery_text.contains("verify_restored_public_set")
        || !public_artifact_recovery_text.contains("JournalState::Restoring")
        || !public_artifact_recovery_text.contains("JournalState::Restored")
        || !public_artifact_recovery_text.contains("journal.project_name")
        || !public_artifact_recovery_text.contains("PdfFileSha256 {pdf_hash}")
        || !public_artifact_recovery_text.contains("report_header_is_formally_green")
        || !formal_green_contract_text.contains("FORMALLY_GREEN_RESULT_STATUS")
        || !formal_green_contract_text.contains("self.blockers == 0 && self.warnings == 0")
        || !formal_green_contract_text.contains("format_summary_line")
        || !formal_green_contract_text.contains("parse_summary_line")
        || public_artifact_recovery_text.contains("SUPPORTED_PROJECT_NAME")
        || public_artifact_recovery_text.contains("PUBLIC_ARTIFACT_NAMES")
        || public_artifact_recovery_text.contains("KEY_SIGNATURE_MUTATION_")
        || public_artifact_recovery_text.contains("Etüde Nr.")
        || public_artifact_recovery_text.contains("Etude Nr.")
        || launcher_text.contains("M18_CLEF_REWRITE_REFERENCE_PROJECT")
        || !run_cmd_text.contains("etuschlopf_take_transaction_pending.txt")
        || !run_cmd_text.contains("--recover-pending-take")
        || !run_cmd_text.contains("etuschlopf_public_artifact_transaction_pending.txt")
        || !run_cmd_text.contains("--recover-pending-public-artifacts")
        || run_cmd_text.contains("ETUSCHLOPF_ARCHITECTURE_GATE_CONTEXT")
        || launcher_assistant_export_text.contains("exact_context_plus_debug_target_bound")
        || launcher_assistant_export_text.contains("take_bootstrap_repair_compile_isolated_from_productive_architecture_gate")
        || !launcher_assistant_export_text.contains("BootstrapArchitectureGateIsolation: retired")
        || !launcher_assistant_export_text.contains("BootstrapAndProductiveArchitectureGate: mandatory")
        || !launcher_assistant_export_text.contains("Verdict: take_bootstrap_compile_uses_mandatory_architecture_gate")
    {
        return Err("SEMANTIC TRANSACTIONAL RECOVERY CONTRACT REGRESSION: complete-tree validation, result-gated RefactorOnly mode, diagnostic prior-revision history, exact-replay protection, pending-backup self-transition proof, post-copy verification, Public-Artifact-Recovery before Source-Recovery, generic project-stem-derived four-artifact snapshot/commit/rollback, strict shared journal parsing, path-safe project-name validation, verified idempotent restore, current-version/result/PDF-hash commit validation, identity-free recovery control plane, identical same-version one-command continuation, mandatory architecture gates for both take-bootstrap and productive builds, one shared ValidationPlan/BuildPlan including Deep Clippy, one typed RunRequest, linear execution stages, a dedicated typed Assistant-Export boundary, path-bound cleanup outcomes, both run.cmd recovery handoffs and the separate Watch-project pre-copy Cargo test boundary are mandatory.".to_string());
    }

    if !run_cmd_text.contains(
        r#"cargo build --quiet --locked --jobs "!ETUSCHLOPF_BOOTSTRAP_JOBS!" --target-dir "!ETUSCHLOPF_BOOTSTRAP_CACHE!" --bin etuschlopf_take_bootstrap"#,
    ) || !run_cmd_text.contains(
        r#"cargo build --quiet --locked --jobs "!ETUSCHLOPF_BOOTSTRAP_JOBS!" --target-dir "!ETUSCHLOPF_BOOTSTRAP_CACHE!" --bin etuschlopf_launcher"#,
    ) || !launcher_build_control_text.contains(
        "\"clippy\".to_string(),\n        \"--locked\".to_string()",
    ) || !launcher_text.contains(
        "\"test\".to_string(),\n        \"--locked\".to_string()",
    ) || !launcher_build_control_text.contains(
        "\"build\".to_string(),\n        \"--locked\".to_string()",
    ) {
        return Err("ROOT CARGO LOCK EXECUTION REGRESSION: all public/root Cargo build, Clippy and test paths after the transaction-owned resolution must remain --locked".to_string());
    }

    let run_public_recovery = run_cmd_text
        .find("etuschlopf_public_artifact_transaction_pending.txt")
        .ok_or_else(|| "RUN.CMD PUBLIC RECOVERY ORDER REGRESSION: public marker missing".to_string())?;
    let run_take_recovery = run_cmd_text
        .find("etuschlopf_take_transaction_pending.txt")
        .ok_or_else(|| "RUN.CMD PUBLIC RECOVERY ORDER REGRESSION: take marker missing".to_string())?;
    if run_public_recovery >= run_take_recovery {
        return Err("RUN.CMD PUBLIC RECOVERY ORDER REGRESSION: Public-Artifact-Recovery must precede Take/Source-Recovery.".to_string());
    }
    let run_take_cache_retention = run_cmd_text
        .find("if defined ETUSCHLOPF_TAKE_TRANSACTION_ACTIVE (")
        .ok_or_else(|| "RUN.CMD TAKE CACHE RETENTION REGRESSION: take-scoped retention guard missing".to_string())?;
    let run_launcher_build = run_cmd_text
        .find("--bin etuschlopf_launcher")
        .ok_or_else(|| "RUN.CMD TAKE CACHE RETENTION REGRESSION: launcher build missing".to_string())?;
    if !run_cmd_text.contains("set \"CARGO_INCREMENTAL=1\"")
        || !run_cmd_text.contains("StrictProfilePackageClean=retired_incremental_reuse")
        || !run_cmd_text.contains("BootstrapPackageClean=retired_incremental_reuse")
        || !run_cmd_text.contains("ReuseAuthority=source_signature+cargo_fingerprint+binary_fnv")
        || run_cmd_text.contains("cargo clean --quiet --package etuschlopf")
    {
        return Err("RUN.CMD TAKE CACHE RETENTION REGRESSION: incremental Cargo reuse must stay enabled, both historical package-clean paths must remain retired, and reuse authority must stay bound to source signature + Cargo fingerprint + binary FNV.".to_string());
    }
    if run_take_cache_retention <= run_take_recovery
        || run_take_cache_retention >= run_launcher_build
    {
        return Err("RUN.CMD TAKE CACHE RETENTION ORDER REGRESSION: recovery must finish first and the take-scoped retention declaration must precede the launcher rebuild.".to_string());
    }
    if !launcher_build_control_text.contains("fn source_build_signature(root: &Path)")
        || !launcher_build_control_text.contains("fn profile_build_artifact_is_verified(")
        || !launcher_build_control_text.contains("file_fnv1a64(&profile.target_exe())")
        || !launcher_build_control_text.contains("reason: CompileReason::MissingVerifiedArtifact")
    {
        return Err("RUN.CMD TAKE CACHE RETENTION AUTHORITY REGRESSION: retained derived Cargo artifacts are permitted only while launcher reuse remains fail-closed on current source signature and executable FNV, with missing verification forcing compilation.".to_string());
    }

    let launcher_recovery = launcher_text
        .find("recover_pending_public_artifacts(&project_root)")
        .ok_or_else(|| "LAUNCHER PUBLIC RECOVERY ORDER REGRESSION: recovery call missing".to_string())?;
    let launcher_cleanup = launcher_text
        .find("remove_obsolete_powershell_files(&project_root)")
        .ok_or_else(|| "LAUNCHER PUBLIC RECOVERY ORDER REGRESSION: cleanup call missing".to_string())?;
    if launcher_recovery >= launcher_cleanup {
        return Err("LAUNCHER PUBLIC RECOVERY ORDER REGRESSION: Public-Artifact-Recovery must precede every launcher cleanup.".to_string());
    }
    let rollback_start = take_bootstrap_text
        .find("fn rollback(&self, project_root: &Path)")
        .ok_or_else(|| "TAKE BOOTSTRAP PUBLIC RECOVERY ORDER REGRESSION: rollback function missing".to_string())?;
    let rollback_text = &take_bootstrap_text[rollback_start..];
    let rollback_public_recovery = rollback_text
        .find("recover_pending_public_artifacts(project_root)")
        .ok_or_else(|| "TAKE BOOTSTRAP PUBLIC RECOVERY ORDER REGRESSION: recovery call missing in rollback".to_string())?;
    let rollback_source_state = rollback_text
        .find("write_transaction_state(&self.state_path, \"rolling_back\")")
        .ok_or_else(|| "TAKE BOOTSTRAP PUBLIC RECOVERY ORDER REGRESSION: source rollback state missing".to_string())?;
    if rollback_public_recovery >= rollback_source_state {
        return Err("TAKE BOOTSTRAP PUBLIC RECOVERY ORDER REGRESSION: Public-Artifact-Recovery must precede the first Source-Rollback mutation.".to_string());
    }
    let rollback_end = rollback_text
        .find("fn rollback_preserving_provenance(&self, project_root: &Path)")
        .ok_or_else(|| "TAKE BOOTSTRAP REJECTION DURABILITY REGRESSION: provenance-preserving rollback function missing".to_string())?;
    let rollback_body = &rollback_text[..rollback_end];
    if rollback_body.contains("finish_rollback()") {
        return Err("TAKE BOOTSTRAP REJECTION DURABILITY REGRESSION: rollback() must stop in rolled_back state and preserve Pending provenance until the canonical Rejection record is durable.".to_string());
    }

    let failed_reexec_rollback = take_bootstrap_text
        .find("transaction.rollback(&project_root)?;")
        .ok_or_else(|| "TAKE BOOTSTRAP FAILED-REEXEC DURABILITY REGRESSION: bound rollback call site missing".to_string())?;
    let failed_reexec_tail = &take_bootstrap_text[failed_reexec_rollback..];
    let failed_reexec_rejection = failed_reexec_tail
        .find("multi_revision_evasion::record_rejection(")
        .ok_or_else(|| "TAKE BOOTSTRAP FAILED-REEXEC DURABILITY REGRESSION: canonical Rejection persistence missing after rollback".to_string())?;
    let failed_reexec_handover = failed_reexec_tail
        .find("finalize_rejection_assistant_handover(")
        .ok_or_else(|| "TAKE BOOTSTRAP FAILED-REEXEC HANDOVER REGRESSION: Failure-Assistant-Handover missing after canonical Rejection".to_string())?;
    let failed_reexec_finish = failed_reexec_tail
        .find("transaction.finish_rollback()?;")
        .ok_or_else(|| "TAKE BOOTSTRAP FAILED-REEXEC DURABILITY REGRESSION: final provenance cleanup missing after Rejection persistence".to_string())?;
    if !(failed_reexec_rejection < failed_reexec_handover
        && failed_reexec_handover < failed_reexec_finish)
    {
        return Err("TAKE BOOTSTRAP FAILED-REEXEC DURABILITY ORDER REGRESSION: required order is rollback < successful record_rejection < failure_handover < finish_rollback.".to_string());
    }
    if !take_bootstrap_text.contains("fn transactional_apply_can_rollback_exactly()")
        || !take_bootstrap_text.contains(
            "fn failed_reexec_rejection_io_failure_preserves_recovery_provenance()",
        )
        || !take_bootstrap_text.contains(
            "fn pending_rolled_back_transaction_recovers_rejection_before_cleanup()",
        )
        || !take_bootstrap_text.contains(
            "fn failure_handover_replaces_stale_export_and_binds_exact_rejection()",
        )
        || !take_bootstrap_text.contains(
            "fn failure_handover_io_failure_preserves_rejection_and_rolled_back_recovery_state()",
        )
    {
        return Err("TAKE BOOTSTRAP REJECTION DURABILITY WITNESS REGRESSION: rollback, canonical Rejection durability, Failure-Handover latest-attempt replacement and rolled_back recovery witnesses must remain registered tests.".to_string());
    }

    let failure_handover_helper = take_bootstrap_text
        .find("fn finalize_rejection_assistant_handover(")
        .map(|start| &take_bootstrap_text[start..])
        .ok_or_else(|| "FAILURE ASSISTANT HANDOVER REGRESSION: finalizer helper missing".to_string())?;
    if !failure_handover_helper.contains("multi_revision_evasion::read_rejection_record_exact(")
        || !failure_handover_helper
            .contains("multi_revision_evasion::read_unique_current_acceptance_record_exact(")
        || !failure_handover_helper.contains("CanonicalParentAcceptance: {}")
        || !failure_handover_helper.contains("canonical_verdict=rejected")
        || !failure_handover_helper.contains("LatestCompletedAttempt: this_failure")
        || !failure_handover_helper.contains("write_failure_source_snapshot(")
        || !failure_handover_helper.contains("kanonische Rejection bleibt unveraendert")
    {
        return Err("FAILURE ASSISTANT HANDOVER AUTHORITY REGRESSION: Failure handover must project exact canonical Parent-Acceptance and Rejection evidence, bind a self-contained Parent source snapshot, identify itself as latest completed attempt, carry no authority and preserve canonical Rejection on projection I/O failure.".to_string());
    }
    let candidate_attempt_start = take_bootstrap_text
        .find("let attempt_started_epoch_millis = current_epoch_millis()?")
        .ok_or_else(|| "FAILURE ASSISTANT HANDOVER STALE-PROJECTION REGRESSION: candidate attempt start missing".to_string())?;
    let candidate_export_invalidation = take_bootstrap_text[candidate_attempt_start..]
        .find("invalidate_published_assistant_export(&project_root)?")
        .ok_or_else(|| "FAILURE ASSISTANT HANDOVER STALE-PROJECTION REGRESSION: published previous Assistant export is not invalidated before candidate gates".to_string())?;
    let candidate_first_gate = take_bootstrap_text[candidate_attempt_start..]
        .find("validate_dead_code_detection_integrity_transition")
        .ok_or_else(|| "FAILURE ASSISTANT HANDOVER STALE-PROJECTION REGRESSION: first candidate gate missing".to_string())?;
    if candidate_export_invalidation >= candidate_first_gate {
        return Err("FAILURE ASSISTANT HANDOVER STALE-PROJECTION ORDER REGRESSION: stale previous Assistant export must be invalidated before the first rejectable candidate gate.".to_string());
    }

    let successful_reexec = take_bootstrap_text
        .find("if code == 0 {")
        .ok_or_else(|| "TAKE EVIDENCE FOUNDATION REGRESSION: successful reexec branch missing".to_string())?;
    let successful_tail = &take_bootstrap_text[successful_reexec..];
    let evidence_mark_committed = successful_tail
        .find("transaction.mark_committed()?")
        .ok_or_else(|| "TAKE EVIDENCE FOUNDATION REGRESSION: committed state missing".to_string())?;
    let canonical_acceptance = successful_tail
        .find("multi_revision_evasion::record_acceptance(")
        .ok_or_else(|| "TAKE EVIDENCE FOUNDATION REGRESSION: canonical acceptance persistence missing".to_string())?;
    let advisory_acceptance_evidence = successful_tail
        .find("record_acceptance_evidence_advisory(")
        .ok_or_else(|| "TAKE EVIDENCE FOUNDATION REGRESSION: advisory acceptance evidence hook missing".to_string())?;
    let post_acceptance_handover = successful_tail
        .find("finalize_post_acceptance_assistant_handover(")
        .ok_or_else(|| "TAKE EVIDENCE FOUNDATION REGRESSION: post-acceptance handover finalizer missing".to_string())?;
    let final_commit = successful_tail
        .find("transaction.commit()?")
        .ok_or_else(|| "TAKE EVIDENCE FOUNDATION REGRESSION: final transaction cleanup missing".to_string())?;
    if !(evidence_mark_committed < canonical_acceptance
        && canonical_acceptance < advisory_acceptance_evidence
        && advisory_acceptance_evidence < post_acceptance_handover
        && post_acceptance_handover < final_commit)
    {
        return Err("TAKE EVIDENCE FOUNDATION ORDER REGRESSION: required order is mark_committed < canonical record_acceptance < advisory acceptance evidence < post-acceptance handover < commit.".to_string());
    }
    let evidence_helper = take_bootstrap_text
        .find("fn record_acceptance_evidence_advisory(")
        .map(|start| &take_bootstrap_text[start..])
        .ok_or_else(|| "TAKE EVIDENCE FOUNDATION REGRESSION: advisory evidence helper missing".to_string())?;
    if !evidence_helper.contains("if let Err(error) = multi_revision_evasion::record_acceptance_evidence(")
        || !evidence_helper.contains("kanonische Acceptance bleibt gueltig")
    {
        return Err("TAKE EVIDENCE AUTHORITY REGRESSION: Acceptance evidence must remain advisory; controlled evidence I/O failure may warn but must not invalidate canonical Acceptance.".to_string());
    }
    if !take_bootstrap_text.contains("root.join(TAKE_TRANSACTION_CANDIDATE_INTENT)")
        || !take_bootstrap_text.contains("source_root.join(take_change_control::CHANGE_MANIFEST_RELATIVE_PATH)")
        || !take_bootstrap_text.contains("fs::copy(
            &intent_source,
            root.join(TAKE_TRANSACTION_CANDIDATE_INTENT),")
        || !take_bootstrap_text.contains(
            "fn pending_transaction_copies_candidate_intent_from_incoming_source_exactly()",
        )
        || !take_bootstrap_text.contains(
            "fn acceptance_evidence_io_failure_does_not_revoke_canonical_acceptance()",
        )
        || !take_bootstrap_text.contains(
            "fn post_acceptance_handover_never_claims_acceptance_before_canonical_record_exists()",
        )
        || !take_bootstrap_text.contains(
            "fn post_acceptance_handover_binds_exact_authority_and_preserves_existing_export_payload()",
        )
        || !take_bootstrap_text.contains(
            "fn post_acceptance_handover_io_failure_preserves_acceptance_and_committed_recovery_state()",
        )
        || !take_bootstrap_text.contains(
            "fn pending_committed_transaction_recovers_acceptance_without_rollback()",
        )
    {
        return Err("TAKE EVIDENCE INTENT REGRESSION: Candidate Intent must remain byte-bound; post-acceptance handover must require canonical Acceptance, preserve existing export payload, keep Handover I/O non-authoritative and recover from committed state before cleanup.".to_string());
    }

    let handover_helper = take_bootstrap_text
        .find("fn finalize_post_acceptance_assistant_handover(")
        .map(|start| &take_bootstrap_text[start..])
        .ok_or_else(|| "POST-ACCEPTANCE HANDOVER REGRESSION: finalizer helper missing".to_string())?;
    if !handover_helper.contains("multi_revision_evasion::read_acceptance_record_exact(")
        || !handover_helper.contains("multi_revision_evasion::read_acceptance_evidence_exact(")
        || !handover_helper.contains("canonical_verdict=accepted")
        || !handover_helper.contains("authority=none")
        || !handover_helper.contains("pre_acceptance_advisory_snapshot")
        || !handover_helper.contains("publizierte ZIP-Projektionen wurden invalidiert")
    {
        return Err("POST-ACCEPTANCE HANDOVER AUTHORITY REGRESSION: finalizer must project only already-persisted exact Acceptance/Evidence, retain transition history as pre-acceptance advisory context, carry no authority and invalidate incomplete published ZIP projections on I/O failure.".to_string());
    }

    let rolled_back_recovery = take_bootstrap_text
        .find("\"rolled_back\" => {")
        .map(|start| &take_bootstrap_text[start..])
        .ok_or_else(|| "FAILURE ASSISTANT HANDOVER RECOVERY REGRESSION: rolled_back recovery branch missing".to_string())?;
    let rolled_back_rejection = rolled_back_recovery
        .find("multi_revision_evasion::record_rejection(project_root, &rejection)")
        .ok_or_else(|| "FAILURE ASSISTANT HANDOVER RECOVERY REGRESSION: canonical Rejection persistence missing".to_string())?;
    let rolled_back_handover = rolled_back_recovery
        .find("finalize_rejection_assistant_handover(")
        .ok_or_else(|| "FAILURE ASSISTANT HANDOVER RECOVERY REGRESSION: Failure-Handover missing".to_string())?;
    let rolled_back_cleanup = rolled_back_recovery
        .find("transaction.finish_rollback()?")
        .ok_or_else(|| "FAILURE ASSISTANT HANDOVER RECOVERY REGRESSION: rolled_back cleanup missing".to_string())?;
    if !(rolled_back_rejection < rolled_back_handover
        && rolled_back_handover < rolled_back_cleanup)
    {
        return Err("FAILURE ASSISTANT HANDOVER RECOVERY ORDER REGRESSION: rolled_back recovery must persist/verify Rejection, finalize current failure handover, then cleanup provenance.".to_string());
    }

    let committed_recovery = take_bootstrap_text
        .find("\"committed\" => {")
        .map(|start| &take_bootstrap_text[start..])
        .ok_or_else(|| "POST-ACCEPTANCE HANDOVER RECOVERY REGRESSION: committed recovery branch missing".to_string())?;
    let committed_acceptance = committed_recovery
        .find("multi_revision_evasion::record_acceptance(")
        .ok_or_else(|| "POST-ACCEPTANCE HANDOVER RECOVERY REGRESSION: committed recovery Acceptance missing".to_string())?;
    let committed_evidence = committed_recovery
        .find("record_acceptance_evidence_advisory(")
        .ok_or_else(|| "POST-ACCEPTANCE HANDOVER RECOVERY REGRESSION: committed recovery evidence missing".to_string())?;
    let committed_handover = committed_recovery
        .find("finalize_post_acceptance_assistant_handover(")
        .ok_or_else(|| "POST-ACCEPTANCE HANDOVER RECOVERY REGRESSION: committed recovery handover missing".to_string())?;
    let committed_cleanup = committed_recovery
        .find("transaction.commit()?")
        .ok_or_else(|| "POST-ACCEPTANCE HANDOVER RECOVERY REGRESSION: committed recovery cleanup missing".to_string())?;
    if !(committed_acceptance < committed_evidence
        && committed_evidence < committed_handover
        && committed_handover < committed_cleanup)
    {
        return Err("POST-ACCEPTANCE HANDOVER RECOVERY ORDER REGRESSION: committed recovery must verify/persist Acceptance, project advisory Evidence, finalize Handover, then cleanup.".to_string());
    }

    let mark_reexec = take_bootstrap_text
        .find("transaction.mark_reexec()?")
        .ok_or_else(|| "ROOT CARGO LOCK TRANSACTION ORDER REGRESSION: mark_reexec missing".to_string())?;
    let lock_resolution = take_bootstrap_text
        .find("transaction.resolve_root_cargo_lock(")
        .ok_or_else(|| "ROOT CARGO LOCK TRANSACTION ORDER REGRESSION: transaction-owned Cargo resolution missing".to_string())?;
    let public_reexec = take_bootstrap_text
        .find("let mut cmd = run_cmd_command(&project_root, &clean_args);")
        .ok_or_else(|| "ROOT CARGO LOCK TRANSACTION ORDER REGRESSION: public reexec missing".to_string())?;
    if !(mark_reexec < lock_resolution && lock_resolution < public_reexec) {
        return Err("ROOT CARGO LOCK TRANSACTION ORDER REGRESSION: the authenticated transaction must enter reexec state, let Cargo reconcile the local lock, and only then start the --locked public reexec".to_string());
    }
    if !take_bootstrap_text.contains(
        ".chain(managed_source_tree_definition::LOCAL_TRANSACTIONAL_ROOT_FILES)",
    ) || !take_bootstrap_text.contains("rollback_restores_absent_local_cargo_lock_exactly")
        || !take_bootstrap_text.contains("cargo-owned-committed-lock")
    {
        return Err("ROOT CARGO LOCK TRANSACTION RESTORE REGRESSION: LocalTransactional root state must be backed up, exact absence must be restorable, and commit must preserve Cargo-owned transaction output".to_string());
    }

    Ok(())
}
