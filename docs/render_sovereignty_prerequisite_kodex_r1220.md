# r1220 RenderSovereigntyPrerequisiteKodex — K-R

Parent: accepted/released r1219 `MasterSovereigntyClosureMaxi`.
Canonical Take SHA-256: `20a013d90140488d71e50e01a30060127f6bcb63d5722dee00dec80bfe802dcd`.
Class: **KodexOnly**. Version remains `0.10.1839`.

The current Lifecycle handover records accepted r1219, no rejection suffix after it, and the supplied history contains no higher Take revision. This K-R therefore uses the next free revision r1220. No revision is reserved for the subsequent Render refactor; its free revision must be determined when that candidate is actually built.

## Product boundary

K-R changes governance, pinned contracts and Take metadata only. Every productive Rust transformation remains byte-identical to the accepted r1219 source snapshot. There is no physical #67 extraction, RenderAdoptedState, Render receipt migration, thread-local retirement, #57/#67 repair, Shadow Permit, WAV work, new selector/ranking logic or MusicMutation.

Expected against r1219: 20 Master/Render MusicXML files byte-identical and 51 PDF pages pixel-identical. A source-neutrality proof is distinct from regenerating those outputs: the provided export supplies the reference outputs, but not the corpus's MIDI/WAV inputs or its Windows renderer installation. A new complete corpus/pixel comparison must be reported only when actually executed.

## Population and the two existing cursor/direction events

Population IDs are exactly 1..=69. Existing rows other than the explicitly authorized Row 2 precision remain unchanged; Row 67's stored original identity/signature remains unchanged.

Row 69 registers the already existing `render_measure_articulation_authority.rs::apply_bound_missing_rest_batch_authorized` as DESTRUCTIVE/EXACT/R with its original `Option<String>` return and pre-bound `(forward_start, forward_end, rest_note)` geometry. Its normal future receipt-only return is `(Option<String>, Option<ExactTransformationReceipt>)`; the existing classification-matched receipt policy remains authoritative, without parameter or write-budget expansion.

Row 2 is bound to the already existing `writer_entry_boundary.rs::apply_authorized_serialization_plan`, whose own event is the cursor-neutral `Vec<TextEdit>` transition. The previous `execute_authorized_cursor_neutral_writer_candidate` entry is an orchestrator. This is the authorized physical-boundary precision, not a change to the #2 edit identity or algorithm.

`render_event_boundaries_v1.tsv` registers exactly two event boundaries:

- CursorNeutral: population #2, EXACT/R, `source_xml` before its `Vec<TextEdit>`.
- DirectionRelocation: separate named EXACT/R event, existing `compose_authorized_direction_delta_onto_sink`, **last #58 committed render state**. It is not a numbered Population 70 or a preserving exclusion.

Both signatures are bound to real existing functions. Only their original return payload plus one classification-matched optional EXACT receipt is admitted. No OwnerReceipts/multi-receipt return is admitted at either event owner. Their ordinary orchestrator can later transport distinct owner receipts; it does not acquire a Population identity for the combined state.

The historical #58 snapshot also performs a file readback check today. K-R preserves those productive bytes. The future state implementation must preserve the historical delta basis and make missing state/readback fail-closed; it must not substitute the latest trial or arbitrary previous state.

## One future joint #67 relocation

The accepted source has one inline Raw-Splice inside `materialize_silent_voice_visible_measure_coverage_for_pdf_source`; no dedicated helper exists. K-R deliberately retains that physical owner.

The single row in `render_owner_relocation_v1.tsv` authorizes only the immediately following, named `RenderSovereigntyClosureMaxi` **RefactorOnly** transition whose accepted parent is this exact K-R revision/name. Rejected attempts do not create a different accepted parent; a different intervening accepted Take expires this authorization. Acceptance remains owned by the existing bootstrap, not by this table.

The later transition must atomically remove the original guarded splice from the wrapper, introduce exactly one helper in the **same frozen source file**, and replace that splice with exactly one guarded call. Population #67 is then resolved through the immutable K-R overlay to the helper. No RefactorOnly edit to a Kodex table is needed or permitted; the stored original semantic Population identity is retained.

Frozen target signature:

```rust
fn same_lane_strict_interior_raw_splice(
    final_candidate: &str,
    edit_start: usize,
    edit_end: usize,
    replacement: &str,
) -> String
```

Frozen body, preserving the original four statements:

```rust
let mut candidate = String::with_capacity(final_candidate.len() + replacement.len());
candidate.push_str(&final_candidate[..edit_start]);
candidate.push_str(&replacement);
candidate.push_str(&final_candidate[edit_end..]);
candidate
```

The guarded wrapper replacement is exactly:

```rust
if let Some((edit_start, edit_end, replacement)) = same_lane_strict_interior_edit {
    let mut candidate = same_lane_strict_interior_raw_splice(
        &final_candidate, edit_start, edit_end, &replacement,
    );
    // Existing validation/adoption logic follows unchanged in meaning.
}
```

Whitespace and comments do not change the code binding; comments or strings cannot stand in for executable code. The helper permits only the frozen `String` return or `(String, Option<ExactTransformationReceipt>)`. For that optional receipt form, the four-statement splice remains exact, followed only by one typed mint call with `(final_candidate, edit_start, edit_end, replacement, &candidate)` and the `(candidate, receipt)` return. The corresponding wrapper binding is `let (mut candidate, same_lane_splice_receipt) = ...`. Runtime descriptor sealing, complete receipt coverage and owner-write invariants still belong to the subsequent sovereign Render adoption owner.

The gate proves together:

- one legacy splice before transition;
- one exact target body afterward;
- no retained/second splice in the wrapper;
- one target definition and one guarded caller, with no external uses;
- the same #67 identity, EXACT classification, R domain and pre-coordinate geometry;
- unchanged `DEFERRED_RENDER_PREREQUISITE`;
- one raw buffer splice, zero file writes, and an unchanged physical-surface budget table;
- no aggregate receipt, nested-owner rule, different target, broadened signature or arbitrary relocation.

The K-R parent may lack the target. The named Render refactor may not. Reversal of an already relocated owner is fatal. The old wrapper becomes an ordinary orchestrator after extraction; inner owners keep their independent events in their real execution order. The existing nested-owner table remains byte-identical and exclusive to #59→#60.

## Closed Render origins and legacy selection

`render_origins_v1.tsv` has exactly these seven bound tuples:

| ID | Role | Fork | Source | Reason |
| --- | --- | --- | --- | --- |
| G1 | first Genesis | PrunedPdfTrial | `best.layout_xml()` | RenderGenesisPrunedPdf |
| G2 | first Genesis | first writing Initial Trial 2–8 | `best.layout_xml()` | RenderGenesisFirstTrial |
| G3 | first Genesis | FinalNotation B1 | `final_export_xml` | RenderGenesisLateFinalNotation |
| G4 | first Genesis | FinalNotation B2 | `final_export_xml` | RenderGenesisLateFinalNotation |
| G5 | first Genesis | FinalNotation B4 | `final_export_xml` | RenderGenesisLateFinalNotation |
| S3 | history-preserving FromMaster | FinalNotation B2, `!avail` | `final_export_xml` | FinalCompleteBeatWithoutRenderSource |
| S4 | history-preserving FromMaster | FinalNotation B4, `!avail` | `final_export_xml` | CursorNeutralWriterWithoutRenderSource |

G3/G4/G5 and S3/S4 bind the Master epoch **after #56 and before #65**, not the earlier `best.layout_xml()` and not an arbitrary latest Master. Their owner, fork, reason, source, epoch, legacy selection and history requirement are part of the closed tuple.

The governance request validator rejects an unknown ID/function/reason/source/epoch/selection, a Genesis on existing history, a reset outside S3/S4, or a substitution that loses history. The subsequent Render adoption implementation must consume this exact contract and bind its request to its actual typed source/master event, fork and history; strings alone are not runtime provenance.

`render_legacy_selection_v1.tsv` explicitly separates:

- legacy availability flags: historical selection only, never Render existence;
- actually written trials, including V1/V2 and IR 2–5: preserved Render history, readable by B1's historical `path.exists()` branch;
- class-F missing state/readback under existing Render state or availability: a future fatal sovereignty error, no legal substitution;
- checkpoint/causal-lane/E059 audit reads: read-only existing Render or explicit absence, no automatic Master reset.

K-R verifies the real legacy source bindings while the legacy architecture remains installed. Once the authorized relocation occurs, those historical reader spellings need not survive the state refactor; the frozen tuples and their meaning do. K-R does not implement the sovereign runtime or claim that the legacy runtime already enforces the new state model.

## Gate integration, trust and packaging

The existing destructive population freeze owner invokes the new Render contract gate. TakeChangeControl invokes the joint transition check both before installation and during controlled reexec against the transaction backup. The accepted-parent bootstrap remains the owner of canonical Acceptance/Recovery.

All four new tables and both new Rust gate/test files are exact-byte pinned by the existing Trust Anchor. The population/freeze/Take-control/build-policy pins are rebound from final bytes. Exclusions, nested-owner transport, physical write budgets, Cargo dependencies and Root Cargo.lock remain unchanged.

The ZIP is a complete managed tree, without a containing folder, Root Cargo.lock, target/cache files or a child Take. Every actual changed file is declared by kind and final FNV in `take_change_class.txt`; unchanged managed files remain present because the current Take-tree contract requires a complete tree.

Synthetic regression coverage is source/contract-driven and independent of corpus/project/measure identities. Positive cases are paired with negative owner, signature, body, caller, budget, phase, origin, history and source variants.

## Executed validation and limits

- Accepted r1219 TakeChangeControl preflight: 14 declared/14 actual changes, 0 music-sensitive paths, 0 semantic music changes and 0 writer-family changes.
- A synthetic complete-tree future probe extracts only the four original #67 statements, preserves their bytes, introduces the single frozen helper and replaces the one guarded caller. The installed K-R TakeChangeControl admits that joint transition as RefactorOnly. This probe is not a packaged Take and reserves no future revision.
- Rust 1.98.0, `cargo check --locked --profile strict --all-targets --target x86_64-pc-windows-msvc`: exit 0, no warnings. The real build owner runs the complete architecture/population gates, reports 69 frozen rows and all 21 existing adversarial checks blocked.
- The canonical Strict Clippy correctness/suspicious/perf contract, with the same Windows target: exit 0, no warnings.
- 171 standard-library scanner/TakeChangeControl regression tests executed successfully, including all 10 new K-R tests. The tests use the actual installed governance modules, not a reimplementation.

A complete linked Windows launcher/Strict-tests/corpus Acceptance was not executed here. The unchanged r1219 normal binary already fails the Linux-only branch at two WAV-audit dead-code lints; a local linked unit-test attempt also failed while archiving an unchanged qualified component. No productive source or lint policy was changed to work around either local limitation. Windows target compilation and the executed governance tests are the proofs reported above; complete Windows Acceptance remains owned by the normal Take reexec.
