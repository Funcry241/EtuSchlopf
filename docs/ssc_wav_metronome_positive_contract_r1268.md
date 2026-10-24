# Positive WAV-metronome contract: preparation only

Status: specification_frozen_pending_runtime_and_governance
ProductDecision: keep_permanently
SSCObligation: wav_metronome
CurrentClosure: open
ProofAuthority: none
RuntimeChange: none

## Defined input and unchanged arithmetic

WAV duration is an input to a fixed dimensional conversion, not a candidate preference or proof. In build_reference_signature, score_quarter_signature uses the first score part and sums, for each measure:

`(measure.expected.numerator / measure.expected.denominator) / max(measure.divisions, 1)`.

Call this existing sum Q (quarter notes), and WAV duration T (seconds). The current estimate is `Some(Q * 60.0 / T)` exactly when T is present, T > 0 and Q > 0; otherwise None. The existing floating-point expression, order of operations and missing/invalid-input behavior remain unchanged. Do not change duration computation, part selection, meter interpretation or input validation as part of representational closure.

apply_expression_polish maps the estimate with `bpm.round().clamp(40.0, 220.0) as u32`, then retains a value only when it is > 0. The current Rust rounding and clamp order remain unchanged. The caller continues to use `best.reference_signature.estimated_bpm`. No second WAV-derived tempo source, salience input, candidate comparison or ranking is admitted.

## Target representation and local scope

When an estimate is present and the score has a first structural measure, the WAV initial tempo must have exactly one coherent direction container in that measure, combining the visible quarter-note value and matching playback tempo. A suitable construction carries the visible words `quarter-note = BPM` and sound tempo BPM in the same direction. It uses the existing additive direction receipt owner and preserves every unrelated note, duration, voice, staff, attribute, clef, tie and non-tempo direction.

This means one XML direction, not two containers counted as one logical source. No estimate or no structural measure yields no new WAV tempo direction. Reapplication to an already complete equivalent representation adds nothing. A complete equivalent quarter-note metronome already supplies visible and playback semantics and must remain unchanged. Conflicting/incomplete legacy tempo representations require an explicit local construction/ownership policy and tests; do not delete or overwrite independent tempo events to satisfy the single-direction invariant. This specification does not grant a writer mutation or bypass existing ownership, additive, semantic-diff or selector gates.

## SSC positive closure and separation from proof

The current K2 predicate requires estimated_bpm absent and therefore cannot close the permanent Keep path. A later named governance revision must add a constructive positive closure alternative, bind it to the real installed input, caller, writer and SSC reference firewall, and reject missing or changed construction. Merely finding this document, removing OPEN or changing a candidate-controlled checksum is insufficient.

The positive check must reject a modified Q*60/T formula, changed rounding or clamp, another tempo source, duplicate initial WAV tempo construction, restored Accent/Salience writers and WAV-derived SSC proof certificates. WAV may remain decoding/feature/audit input outside proof. Existing SSC evidence-domain restrictions remain in force; the contract creates no certificate, enforcement, ranking or veto authority.

Only after runtime, functional tests, installed-tree checks and adversarial probes establish the positive path may wav_metronome leave OPEN. #57/#67, population statuses, the shadow-only enforcement state and unresolved musical observations are outside this closure.

## Required corpus-independent validation

Synthetic scores and WAV reports exercise the real signature builder, caller/writer and additive adoption path. Use an independent parsed XML oracle, not the same formatter as the writer. Positive cases include ordinary tempo, lower/upper clamp, rounding boundaries, absent duration and absent measure, equivalent preexisting tempo and repeat application. Prove the unchanged first-part quarter calculation for different divisions and meters.

Negative source/tree probes change each arithmetic stage, introduce a second source or a duplicate direction and inject WAV into a proof path; each must fail the constructive closure. Restored retired writers and OPEN removal without positive evidence must fail. Metamorphic cases rename parts, renumber opening measures, move the synthetic score envelope, perturb irrelevant directions and duplicate independent musical material without changing the existing local tempo decision. A newly doubled score duration intentionally changes Q; distinguish this legitimate arithmetic change from candidate-duplication invariance.

These are requirements for subsequent implementation, not tests claimed to exist in r1268. Real corpus files remain evaluation only. Final Windows acceptance and a representation-aware MusicXML/PDF comparison remain necessary for the productive take; do not claim byte-neutrality for consolidation of direction containers.

## Reviewed accepted-parent source bindings

The following hashes identify the inspected r1267 files. They are provenance, not executable checks or a future waiver of semantic admission:

- `src/reference.rs`: SHA256 `cf3a01648d2c292404f0d015217d9506958346c0c93a473f006398bc46001cd5`.
- `src/pipeline_shards/pipeline_orchestrator_project_setup_stages.rs`: SHA256 `d6b7683c5c8532f66330bbf7a1d3c1f9144494e5223287d6fe2d0bcb88e6478e`.
- `src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs`: SHA256 `9fd3baa24a4ee41b2ac54268c7f0e4343a843919eea929cf65702d9d602e3923`.
- `src/ssc_evidence_proof.rs`: SHA256 `34932a54b24744814966bbcf67d056e6fc3844a295de6a84a9bcf6b9917c54bf`.
- `src/ssc_shadow_permit.rs`: SHA256 `b44818ead7176314c397e3c4fed6a4d1d078c43c3c8f14c67fd995f4a54cbdb6`.
- `src/build_gate/ssc_k2_retirement_governance.rs`: SHA256 `b38fdb13658e52f758acbb38565fcaa78370f0e50b26154b607f477cdc4a9ff8`.
