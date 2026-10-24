// r1265: coordinate launcher host build-script optimization; complete gates and target profiles unchanged.
// r860 KodexDuplicateFullBytePinRetirementAndProofDependencyEssenceClosure: retires only exact whole-byte FNV checks duplicated by identity_gate_trust_anchor; local semantic dependency checks and returned inventory fingerprints remain unchanged.
// r783 FeedbackStructuralizationLegacyPrivateProjectionRetirementProofDependencyClosure: binds the retired legacy feedback structuralization surface and exact append-only KodexNegativeCheck retirements while preserving the live minimal non-linkable feedback projection and all unrelated policy pins.
// r781 OutcomeProjectionKodexNegativeCheckRetirementAdmissionTestPinClosure: preserves the rejected r780 admission semantics exactly and closes its stale identity-trust unit-test pins; exact append-only KodexNegativeCheck retirement evidence is admitted in OutcomeProjection while fingerprint, anti-stub, anti-relocation, writer, music, repair and release defenses remain unchanged.
// r777 LauncherPhaseCompatibilityRetirementCompileClosure: preserves the r776 Phase-retirement intent, closes its rejected E0716 inline-message lifetime defect, and rebinds final launcher control-plane bytes without changing recovery, export, writer, music, release or process-lifecycle authority.
// r722 PayloadStableManagedTreeApply: preserves byteidentical TakeManaged payloads physically during the already verified transactional apply, while changed/new/deleted payload semantics and full installed-tree readback remain fail-closed; Take-bootstrap pin rebound from final bytes.
// r596 MusicXml40ProductContractCutoverTestCompileRecoveryAdmission: rebinds TakeChangeControl after recording rejected r595 test-only E0515 and moving the exact one-shot MusicXML-4.0 cutover target to r597; no policy widening or productive authority change.
// r575 SchemaSchlopfIntegrationManifestWarningFreeClosure: rebinds TakeChangeControl after replacing the rejected upstream warnings=warn component-manifest expectation with the exact warnings=deny integration-manifest contract; no productive runtime or Warning-Free policy weakening.
// r592 MusicXml40ProductContractAdmissionBoundary: rebinds TakeChangeControl and SemanticCapabilityDiff after introducing the exact one-shot SchemaSchlopf-r020 -> r021 / MusicXML-3.1 -> 4.0 admission and closing steady-state qualified-component mutation; productive runtime and Cargo.toml remain unchanged in this take.
// r574 SchemaSchlopfDependencyAdmissionBoundary: rebinds TakeChangeControl and SemanticCapabilityDiff after introducing the exact ValidationOnly SchemaSchlopf-r020 dependency admission; productive runtime and Cargo.toml remain unchanged in this take.
// r528 ChangeClassPathRoleChangeKindColdStartReadbackClosure: rebinds TakeChangeControl after aligning explicit Audit-owner PathRole precedence with SemanticCapabilityDiff while preserving all writer and mutation defenses.
// r501 AssistantHandoffInsideAssistantExportClosure: rebinds the pinned Assistant-Export projection after staging the already repository-first assistant_handoff_latest.txt inside assistant_export_latest.zip before the final ZIP freeze; the handoff remains projection-only and grants no architecture, music, writer, repair, proof, release, identity or feedback authority.
// r469 EarlyPreflightCandidateRejectionPersistenceClosure: binds the Take bootstrap after moving exact candidate SHA-256 identity ahead of candidate-specific fail-closed preflights and persisting early Tree-Integrity/dependency/dead-code/watch preflight rejections into the existing MultiRevision ledger; discovery fail-closed semantics are unchanged and no package is deleted.
// r453 FinalConsoleOutcomeClarityNegativeCheckIdentityClosure: preserves the existing steady-state Cargo manifest KodexNegativeCheck rejection fingerprint exactly while making its return structurally necessary, closing needless_return without retirement, relocation, suppression, release/audit change, or new authority.
// r452 FinalConsoleOutcomeClarityClosure: closes the remaining steady-state needless_return warning debt and rebinds launcher/assistant-export presentation bytes after separating human console wording from unchanged machine AuditBlocked evidence; no release, audit, music, writer, repair or recovery authority changes.
// r451 CompletedCargoTransitionTestCompileClosure: rebinds TakeChangeControl after restoring the local DeclaredChange test fixture needed by the surviving steady-state Watch Cargo rejection proof; no transition capability or productive authority is reintroduced.
// r449 CompletedCargoTransitionRetirementSuperCut: binds the steady-state Take/Cargo policy after retiring completed one-shot Cargo and build-owner transition capabilities; reusable relocation and warning-debt closure contracts remain unchanged.
// r448 CompletedTransitionRetirementSuperCut: rebinds steady-state TakeChangeControl after retiring the completed one-shot namespace transition machinery; historical retirement-ledger compatibility remains unchanged.
// r437 KodexRenameChangeClassNegativeCheckOwnerClosure: rebinds TakeChangeControl after preserving the build-owner negative-check identity across the one-shot namespace cutover; no policy weakening.
// r434 KodexRenameHistoricalRetirementReadAnchorClosure: preserves the append-only historical KodexNegativeCheck retirement bytes across the later exact Kodex->Kodex namespace cutover by constructing that one historical ledger token from non-renameable fragments; no retirement, trust-domain, launcher, music, writer, repair or recovery semantics are weakened.
// r429 KodexRenameTransitionFoundation: TakeChangeControl gains the one-shot exact namespace cutover proof and SemanticCapabilityDiff gains only the legacy retirement-ledger alias needed to preserve append-only history across the later Kodex->Kodex rename; no existing policy is weakened and no music/writer authority changes.
// r414 DirectLauncherTakeSurface retirement pin coordination: the retired direct-launcher Take request/helpers are removed while run.cmd plus etuschlopf_take_bootstrap remain the sole transactional Take path; five historical launcher negative checks are retired append-only and pins are rebound from final bytes. The historical version-bound post-take bridge remains unchanged because its writer-reachable Kodex capabilities are not covered by the existing retirement contract.
// r405 CrossFileInterfaceClosure retirement closure: r404 rolled back before commit; pins follow the final retirement bytes after removing the obsolete SourceIntegrity mask_non_code consumer binding, with transactional compiler/test rollback remaining the Rust correctness authority.
use super::rust_source_scan_primitives::fnv1a64;

pub(super) struct PinnedPolicyKodexInputs<'a> {
    pub(super) scanner_text: &'a str,
    pub(super) baseline_text: &'a str,
    pub(super) feedback_gate_text: &'a str,
    pub(super) feedback_admission_text: &'a str,
    pub(super) kodex_capability_retirements_text: &'a str,
    pub(super) launcher_text: &'a str,
    pub(super) launcher_assistant_export_text: &'a str,
    pub(super) launcher_neuro_evidence_export_text: &'a str,
}

pub(super) struct PinnedPolicyFingerprints {
    pub(super) scanner: u64,
    pub(super) baseline: u64,
    pub(super) feedback_gate: u64,
    pub(super) feedback_admission: u64,
}

pub(super) fn validate_pinned_policy_kodex(
    inputs: PinnedPolicyKodexInputs<'_>,
) -> Result<PinnedPolicyFingerprints, String> {
    let PinnedPolicyKodexInputs {
        scanner_text,
        baseline_text,
        feedback_gate_text,
        feedback_admission_text,
        kodex_capability_retirements_text,
        launcher_text,
        launcher_assistant_export_text,
        launcher_neuro_evidence_export_text,
    } = inputs;
    // Whole-byte identity for these Kodex sources is already owned by
    // identity_gate_trust_anchor. This local gate keeps only the fingerprints
    // it actually exports to InventoryKodex plus its own semantic dependency checks.
    let scanner_fingerprint = fnv1a64(scanner_text.as_bytes());
    let baseline_fingerprint = fnv1a64(baseline_text.as_bytes());
    let feedback_gate_fingerprint = fnv1a64(feedback_gate_text.as_bytes());
    let feedback_admission_fingerprint = fnv1a64(feedback_admission_text.as_bytes());

    let retired_assistant_export_archive_retirement = [
        "retire=Sec",
        "urityNegativeCheck|src/bin/etuschlopf_launcher/assistant_export.rs|publish_assistant_export_archive#1|fnv64:3edf8bcc38f35327|reason:assistant_export_transfer_archive_surface_retired",
    ]
    .concat();
    if !launcher_neuro_evidence_export_text.contains(
        "const ARTIFACT_KIND: &str = \"etuschlopf_neuro_evidence\";",
    )
        || !launcher_neuro_evidence_export_text.contains("const SCHEMA_VERSION: u32 = 2;")
        || !launcher_neuro_evidence_export_text.contains(
            "const POPULATION_SCOPE: &str = \"qualified_key_change_candidates_v1\";",
        )
        || !launcher_neuro_evidence_export_text.contains(
            "const CORPUS_COMMIT_ARTIFACT_KIND: &str = \"etuschlopf_neuro_corpus_commit\";",
        )
        || !launcher_neuro_evidence_export_text.contains(
            "const DECISION_CLASS: &str = \"tonal_segmentation_boundary_audit_v1\";",
        )
        || !launcher_neuro_evidence_export_text.contains(
            "const DECISION_CONTRACT_FINGERPRINT: &str = \"fnv64:4f2c765d00c527e6\";",
        )
        || !launcher_neuro_evidence_export_text.contains(
            "const OWNER: &str = \"TonalSegmentationEvidenceAudit\";",
        )
        || !launcher_neuro_evidence_export_text.contains(
            "const PROMISING_CANDIDATE: &str = \"promising_candidate_more_corpus_calibration\";",
        )
        || !launcher_neuro_evidence_export_text.contains(
            "const HELD_CANDIDATE: &str = \"held_as_tonicization_or_chromatic_colour\";",
        )
        || !launcher_neuro_evidence_export_text.contains(
            "const FULL_EVIDENCE_CANDIDATE: &str = \"full_evidence_candidate_write_still_closed\";",
        )
        || !launcher_neuro_evidence_export_text.contains("TonalBoundaryLedger:")
        || !launcher_neuro_evidence_export_text.contains("source_audit_verdict")
        || !launcher_neuro_evidence_export_text.contains("\"label\":null,\"proof\":null")
        || !launcher_neuro_evidence_export_text.contains("DecisionEvidence-Closure verletzt")
        || !launcher_neuro_evidence_export_text.contains("archive.len() != 2")
        || !launcher_neuro_evidence_export_text.contains(
            "names != [DECISIONS_PAYLOAD.to_string(), MANIFEST.to_string()]",
        )
        || launcher_neuro_evidence_export_text.contains("assistant_context.json")
        || launcher_neuro_evidence_export_text.contains("assistant_export_latest.zip")
        || launcher_assistant_export_text.contains("neuro_evidence_export")
        || launcher_assistant_export_text.contains("EtuSchlopfTransfer")
        || launcher_assistant_export_text.contains("ETUSCHLOPF_ASSISTANT_EXPORT_DIR")
        || launcher_assistant_export_text.contains("ASSISTANT_EXPORT_ARCHIVE_DEFAULT_DIR")
        || launcher_assistant_export_text.contains("ASSISTANT_EXPORT_ARCHIVE_DIR_ENV")
        || launcher_assistant_export_text.contains("assistant_export_archive_dir")
        || launcher_assistant_export_text.contains("publish_assistant_export_archive_best_effort")
        || launcher_assistant_export_text.contains("retired_assistant_export_archive_tests")
        || launcher_assistant_export_text.contains("assistant_export_archive_timestamp_token")
        || launcher_assistant_export_text.contains("assistant_export_archive_revision")
        || launcher_assistant_export_text.contains("assistant_export_archive_name")
        || launcher_assistant_export_text.contains("is_managed_assistant_export_archive_name")
        || launcher_assistant_export_text.contains("publish_assistant_export_archive")
        || !kodex_capability_retirements_text
            .contains(retired_assistant_export_archive_retirement.as_str())
        || !launcher_text.contains("mod neuro_evidence_export;")
        || !launcher_text.contains("publish_neuro_evidence_for_successful_normal_run")
        || !launcher_text.contains("publish_neuro_evidence_for_successful_corpus_run")
        || launcher_text.contains("--neuro-corpus")
    {
        return Err(
            "NEURO EVIDENCE TRUST-DOMAIN BOUNDARY REGRESSION: schema-v2 population scope, decision contract, exact two-file project closure, corpus-commit publication and assistant-domain exclusion must remain explicit; assistant exports must not publish into EtuSchlopfTransfer; the retired archive surface must remain absent from Assistant Export source and its KodexCapabilityRetirement evidence must remain pinned."
                .to_string(),
        );
    }
    let renderer_binding_call =
        "bind_corpus_run_renderer_outcome(project_root, &launcher.version, state)?;";
    let all_run_diagnostic_call = "append_run_diagnostic(state, \"all_run\")?;";
    let final_report_rebind_call = "refresh_corpus_run_manifest_report_binding(";
    let renderer_binding_index = launcher_text.find(renderer_binding_call).ok_or_else(|| {
        "CORPUS REPORT FINALIZATION REGRESSION: renderer outcome binding call missing from --all launcher path."
            .to_string()
    })?;
    let all_run_diagnostic_index = launcher_text.find(all_run_diagnostic_call).ok_or_else(|| {
        "CORPUS REPORT FINALIZATION REGRESSION: all-run launcher diagnostic call missing."
            .to_string()
    })?;
    let final_report_rebind_index = launcher_text
        .find(final_report_rebind_call)
        .ok_or_else(|| {
            "CORPUS REPORT FINALIZATION REGRESSION: final corpus report rebind call missing."
                .to_string()
        })?;
    if !(renderer_binding_index < all_run_diagnostic_index
        && all_run_diagnostic_index < final_report_rebind_index
        && launcher_text.contains("program_result.exit_code == 0,")
        && launcher_text.contains("require_valid_manifest: bool")
        && launcher_text.contains("if require_valid_manifest && !validation.valid"))
    {
        return Err(
            "CORPUS REPORT FINALIZATION REGRESSION: renderer binding, launcher diagnostic and final report hash binding are not ordered or fail-closed as pinned."
                .to_string(),
        );
    }

    Ok(PinnedPolicyFingerprints {
        scanner: scanner_fingerprint,
        baseline: baseline_fingerprint,
        feedback_gate: feedback_gate_fingerprint,
        feedback_admission: feedback_admission_fingerprint,
    })
}
