# r1173 LowerStaffBoundDispensabilityAssistantProjectionTrustAnchorRecovery

## Parent

Accepted r1171 `LowerStaffOverlappingRestBoundDispensabilityAuditProjectionRecovery`.

Rejected r1172 is evidence only and is not used as a parent.

## Established r1171 result

r1171 is accepted, dark-green and released. Its synthetic bound-dispensability audit tests pass, and the E6 evidence binds the selected complete lower-staff rest carrier to only partial final-Master pitched coverage, leaving the trailing interval uncovered. Coverage itself has no write authority; whole-carrier dispensability therefore remains unproven.

The bound-dispensability row is already produced in the full result section. The normal Assistant project-summary exporter, however, only whitelists the older `Summary`, `Witness`, `ReanchorTrialEvidence`, and `ReanchorAction` row families from that section, so the new row is dropped from `*.assistant-summary.txt`.

## r1172 rejection and recovery

r1172 changed only the Assistant-summary filter and reached Post-Apply-Reexec, proving the change-control accepted the intended OutcomeProjection boundary. Compilation then failed on the exact AssistantExport trust-anchor assertion because the expected FNV remained the r1171 value.

The transaction rolled back to r1171.

r1173 reapplies the same projection predicate directly to r1171 and synchronizes the pinned expected `LAUNCHER_ASSISTANT_EXPORT_FNV1A64` to the FNV-1a64 calculated from the actual changed `src/bin/etuschlopf_launcher/assistant_export.rs` bytes:

`0x73c83307d4f5e937`

No rejected source is treated as authority; r1172 only supplied failure evidence and the intended delta is reconstructed against r1171.

## Change

Extend the existing final-chord/rest-epoch Assistant-summary projection predicate to retain rows beginning with:

`LowerStaffOverlappingRestBoundDispensabilityAudit`

A corpus-independent unit test admits that row family and rejects an unrelated audit row.

Because `assistant_export.rs` is pinned Kodex control-plane, `identity_gate_trust_anchor.rs` is synchronously rebound to the exact changed bytes.

## Contract

`OutcomeProjection` only.

No MusicXML, Render-MusicXML or PDF change. No selector, candidate, score, threshold, writer, replacement, acceptance, release or music-rule change.
