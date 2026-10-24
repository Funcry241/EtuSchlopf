# r1213 MasterSovereigntyAdoptionAndFinalClosure

Parent: accepted/released **v0.10.1839 r1212**. Change class: **RefactorOnly**. No Kodex file changed.

The take ran strictly phase by phase. Gates A and B are green. Gate C cannot be reached in this take, so, as the take contract requires, phases C, D and E were **not started** rather than half-built.

## Phase A: receipt migration and back-channel retirement

### Owner signatures migrated (r1212 receipt-only rule)
Owner, function name, parameters, effect class and identity kind are unchanged. Only the return type changed.

| Row | Owner | Before | After |
|---|---|---|---|
| #3 | `apply_bound_safe_transfer_authorized` | `-> Option<String>` | `-> (Option<String>, Option<MasterExactReceipt>)` |
| #4 | `apply_bound_staff_voice_move_authorized` | `-> Option<String>` | `-> (Option<String>, Option<MasterExactReceipt>)` |
| #49 | `replace_bound_clef_measure_authorized` | `-> Option<String>` | `-> (Option<String>, Option<MasterBlockReceipt>)` |
| #53 | `enforce_beat_scoped_beaming_and_rest_completion_in_xml` | `(xml: &mut String)` | `-> Option<MasterBlockReceipt>` |
| #63 | `apply_restify_patch` | `-> String` | `-> (String, Option<MasterExactReceipt>)` |

#52 already returned its receipt inside `ArrangementResult` (r1211) and is unchanged.

The frozen surface signatures of the capability `apply` methods (E016, E020) are unchanged. Their edit lists are exposed through a private `receipt_edits()`, so the EXACT script is the same list `apply` applies. The non-population pipeline helpers `apply_atomic_patch`, `apply_staff_voice_move_patch` and `apply_safe_transfer_patch` forward the receipt. `LowerStaffClefPolish` carries the #49 receipts in application order.

### r1211 back-channels removed
- `collect_transformation_receipts`
- `deposit_transformation_receipt`
- `ReceiptCollectionScope` with its thread-local scope stack
- the `receipts_without_collection` metric

Receipts now travel only as typed values (`OwnerReceipts`). `lineage_extend_block` was also retired, because nothing else used it.

### Gate A
- Population unchanged (68 rows; the freeze gate accepted the migrated signatures).
- WriterReachability not extended (harness: no new writer-reachable function).
- Receipt continuity holds by construction.
- Product byte-neutral by construction: every returned XML string and every `expect` path is unchanged.

## Phase B: MasterAdoptedState

`structural_write_authority::master_adoption::MasterAdoptedState { document, lineage }`:
- `document` is private to the adoption module.
- Read access is `document()`, and on `SolverCandidate` `layout_xml()`.
- Changes are possible only through:
  - `adopt_genesis_state(document, lineage, reason)`: the first state of a candidate; the lineage must start at a minted GENESIS.
  - `adopt_materialized_document(document, owner_receipts, reason)`: bytes already materialized by an owner or by StructuralWriteAuthority.
  - `adopt_in_place_transformation(reason, |xml| (value, owner_receipts))`: owners with a frozen `&mut String` signature.
- Every adoption extends the lineage to the new document in the same call. XML and lineage can therefore no longer be adopted or overwritten independently, and lineage head = fingerprint(document) holds at all times.

### Integration
- `SolverCandidate.layout_xml` and `master_lineage` → one field `master_state`. 165 read sites now use `layout_xml()`; 144 of them were cleaned of a needless borrow.
- All 29 former direct Master writes go through adoption:
  - 8 project-setup stage writes
  - 4 initial-render stage writes
  - 4 in `build_atomic_candidate`
  - 5 hidden-chord, 3 audio-salience and 2 sheaf-noether candidate writes
  - 2 editorial-voice writes (request adoption and field adoption)
  - 1 release reload
- The r1205 request boundary `apply_block_master_xml_request` materializes into `&mut MasterAdoptedState` (gap reason `master_request_replacement`).
- Field adoption (`best.master_state = candidate.master_state`) transports document and lineage together.
- Swaps, drops, clones and worker transport move the whole state.

### Gate B
- `AdoptionWithoutGenesis = 0`: type-enforced, because no `MasterAdoptedState` exists without a lineage, and a lineage exists only from a minted GENESIS.
- `GenesisLoss = 0`: type-enforced.
- `ContinuityViolations = 0`: by construction, since the head always equals the document; receipt pre-states are taken from the adopted document. The released runtime metric reports it.
- No new writer root: the harness reports no new writer-reachable function, and the adoption functions carry no writer flag.
- Candidate order and musical selection unchanged.

**Transitional, named honestly:** `adopt_in_place_transformation` is crate-visible. A stage that returns no receipts produces an explicit `unmigrated_master_stage` gap. Phase C2 must narrow this to receipt-only use, and Phase E retires the gap-accepting form.

## Why Gate C cannot be reached in this take

Gate C requires that no Master gap other than `render_derived_master_reload` remains.

### C2, `unmigrated_master_stage`
Seven Master stage functions write the adopted document but are **neither population owners nor named exclusions**:
- `ensure_final_barline`
- `apply_expression_polish`
- `apply_octave_notation_policy`
- `apply_final_notation_polish_guard`
- `normalize_final_export_xml`
- `apply_visual_layout_pressure_pass`
- `apply_pianistic_readability_polish`

They are wrappers around inner physical write surfaces. Receipting them requires mapping every inner surface to its population row or exclusion, migrating those inner owners and aggregating their receipts per stage.

The candidate-level writes in hidden-chord reduction (rows 7–14, 26, 27), audio salience, sheaf-noether and editorial voice (row 1) need the same.

### C3, PRESERVING
The PRESERVING steps need a `MasterPreservingReceipt` with an executable conservation verifier per frozen exclusion contract:
- `polish_musicxml` (E051)
- `apply_pianist_layout` (E056)
- `enforce_partial_beat_post_rest_beams_in_xml` (E053)
- `apply_rustig_clef_course_to_buffer` (E024)

Each verifier must check something concrete, for example that only `<beam>` children or only `<print>` breaks changed. A receipt without a real verifier would be exactly the loophole the take forbids.

### C1, `unreceipted_patch_variant`
The variant generators (final-critical, playability cleaner and polish, hand trajectory, overfull crop) return `Vec<(String, String)>` without receipts. They use owners #39, #40 and #25 (EXACT, migratable) and mixed move+restify chains. Their variant type has to carry receipts.

Silently adding the seven wrappers to the population is forbidden, so Gate C would fail. The take stops here, as its contract requires.

## Phase D readiness (not executed)
The reload `best ← musicxml_out` reads exactly the bytes that `FinalNotationLayoutSnapshotSet.master` already holds after SnapshotClose. No writer touches the Master file afterwards.

Phase D can therefore pass these snapshot bytes as a typed released-master artifact to:
- the export validation;
- the post-render assembly (voice-fiber ledger → `final_result`);
- the 9 report reads.

There is then no XML re-import into `best`, and every report byte stays identical.

## Expected product effect
None.
- 10/10 projects.
- Master MusicXML, Render MusicXML and PDF identical to r1212.
- `result.txt` changes only in the `MasterLineageMints` line, where `receipts_without_collection` is gone.
