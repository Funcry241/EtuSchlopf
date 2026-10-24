# r1202 EtudeSpecificAccentReserveAuthorityRetirement — stage 1

Parent: accepted/released r1201. Change class: `MusicMutation`. Changed productive source: `src/pipeline_shards/noether_register_articulation_readback_tail.rs` only.

## Evidence from accepted r1201

- `NominalGateInventory` flagged `LITERAL_COORDINATE_TABLE` on the AccentReserve candidate table and `CONSTANT_OPEN_GATE_PARAMETER` on its caller (the stage passes `user_gate_open = true`).
- The linked identity-bearing path applied 4 edits in exactly one etude, all with final receipts.
- Direct Render-MusicXML readback: those 4 table coordinates carry an accent in the final Render file; the Master file contains no accents. The remaining two table coordinates are tie-stop notes and carry no accent (blocked by the existing tied-tail rule).
- No table coordinate is the tie-start attack of a WAV-accented tail, so removing the table from the WAV attack-ownership planning cannot change any WAV accent.

## Change

1. `accent_reserve_top_six_micro_write_candidates` (the literal coordinate table) is deleted.
2. `accent_attack_ownership_plan` plans only the WAV rows; the former reserve rows are no longer planned.
3. `apply_accent_reserve_top_six_micro_write_candidate` keeps its signature (its stage call site cannot be edited in a MusicMutation take, see below) but its body returns the unchanged input with `written = false` and verdict `accent_reserve_retired_no_write`. The stage writes through the `accent_reserve` checkpoint sink only when `written` is true, so the sink is statically reachable but can no longer fire.

## Why the structural retirement cannot happen in this take (verified with the take gates)

| Attempt | Result |
|---|---|
| Full removal with `authority_retirement_v1` | rejected: removed lines of the owner file are selector-sensitive (the table itself is selector logic), which retirement forbids |
| Removing call and sink in the render stage | rejected: `run_rest_and_articulation_render_stage` is a `ProofReentryReader`; MusicMutation may not modify it |
| Full removal as RefactorOnly or plain MusicMutation | rejected: removing the sink is a pure writer-authority loss and requires `authority_retirement_v1` |
| Removing the measure-literal probe | orphans AuditOnly probe functions; MusicMutation may not remove AuditLogic |

The remaining structural retirement (stage call, `accent_reserve` checkpoint sink with enum/mapping/re-export, measure-literal probe and orphaned audit symbols) is deferred to the next take.

## Expected corpus readback

- One Render-MusicXML loses exactly the four table-derived accents; nothing else changes.
- All Master-MusicXML files and all other Render-MusicXML files byte-identical to r1201; other PDFs pixel-identical.
- `DestructiveEditAppliedDisposition`: AppliedEdits 9451 → 9447, FINAL_RECEIPT 1614 → 1610, NO_SELECTION_WITNESS 7837 unchanged.
- WAV accent path: still 9–12 applied edits per etude, 105 in total.
- r1201 static inventory: `LITERAL_COORDINATE_TABLE` 0 rows; `LITERAL_MEASURE_BINDING` still 1 (probe, removed in the next take). Minting-site lines in the owner file shift; runtime origins remain matched.
- RenderEpoch: no changed-bytes boundary of kind `accent_reserve`.

## Inertness

The retired function body contains no call at all: no table lookup, no articulation derivation, no write authority and no file access; it constructs a default audit and returns its input. `written` is never set, so the stage's `if audit.written` branch that calls the `accent_reserve` checkpoint sink cannot execute. A direct unit test of the retired function is intentionally not added: the Take gate forbids new test callers of a productive writer root, and the function keeps its writer-rooted name until the structural retirement. The corpus readback (no changed `accent_reserve` epoch boundary, exactly four accents removed) is the runtime proof.
