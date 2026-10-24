# r1215 MasterLineageContinuityAndEditorialRequestReceipts

Parent: accepted/released **v0.10.1839 r1214**. Change class: **RefactorOnly**. No Kodex file changed. This is SSC-2 phase C, part 2.

## Starting point (r1214 released corpus, 10 projects)
126 PendingGaps:

| Gap reason | Count |
|---|---|
| `unmigrated_master_stage` | 94 |
| `master_request_replacement` | 13 |
| `render_derived_master_reload` | 10 |
| `continuity_refused_receipt` | 9 |

`arrangement_polish_and_layout` and `unreceipted_patch_variant` were already 0. About half of the PRESERVING mints were refused at runtime.

## 1. PRESERVING projection: presentation-only attributes
**Cause of the r1214 refusals.** `polish_musicxml` inserts `<attributes>` blocks that carry only a clef. The projection emitted a line for every `<attributes>` element, so the conserved projection differed although no conserved content had changed.

**Fix.** The projection emits `D|`, `K|` and `T|` lines only when divisions, key or time are present. An `<attributes>` element that carries only clefs or staff details leaves no trace.
- Clefs were already declared free in r1214; the projection now matches that contract.
- Divisions, key and time remain conserved exactly as before.
- New synthetic test: `clef_only_attributes_are_conserved_by_the_projection`.

## 2. Lineage continuity: gap, then receipt
**Previously.** A receipt whose pre-state differed from the lineage head was refused and counted as `continuity_refused_receipt`. This happened, for example, after an earlier owner execution whose receipt had itself been refused. The receipt's evidence was then lost.

**Now.** `lineage_extend_receipt`:
- **Duplicate identity:** dropped and counted as a continuity violation. The remainder is bridged by an explicit gap.
- **Pre-state ≠ head:** an untrusted `PendingGap(head → receipt.pre)` is appended with the caller's reason, then the receipt is attached at its own pre-state.

**Consequences.**
- The gap reason `ContinuityRefusedReceipt` is removed.
- No receipt becomes trusted beyond its own pre/post binding, and every unmodelled stretch is still an explicit, untrusted gap.

## 3. Editorial request receipts (`master_request_replacement`)
Migrated under the r1212 receipt-only rule (return type only; parameters unchanged):

| Row | Owner | New return type |
|---|---|---|
| #1 | `apply_editorial_voice_complete_rewrite_authorized` | `(Option<String>, Option<MasterExactReceipt>)` |
| #5 | `hide_note_print_object_authorized` | `(Result<bool>, Option<MasterExactReceipt>)` |

**Bound edits.**
- **#1:** every bound note range is replaced by `""`, plus one insertion at `insert_position`.
- **#5:** the note's range is replaced by the hidden note.

Both are verified byte-exactly by `exact_edits_reproduce`.

**Data flow.**
- The editorial hide, integrate and complete-rewrite variants now return `PatchedXml` with receipts. The integrate variant carries #40.
- `build_editorial_voice_trial_candidate` binds them with `MasterXmlReplacementRequest::bind_with_owner_receipts`.
- The request carries `OwnerReceipts`, and `apply_block_master_xml_request` extends the lineage with them in application order before bridging any remainder.

Sealed owner tokens are `EditorialVoiceCompleteRewriteExactMint` and `NoteVisibilityExactMint`. Their descriptors are checked against `destructive_transformation_population_v1.tsv`; the descriptor test now covers 13 rows.

## What remains for Gate C
- **Master stage wrappers** (`unmigrated_master_stage`): barline+expression (ADDITIVE), octave policy, final-notation guard, export normalization, visual layout pressure, pianistic readability, Rustig clef (E024).
  - Most of their inner owners are **exclusion owners** with frozen scalar returns.
  - The r1204 freeze gate offers a receipt-only migration only for population owners.
  - So the next step is a **KodexOnly K2 take**: a receipt-only signature migration for PRESERVING/ADDITIVE exclusion owners, plus an ADDITIVE receipt contract.
- **Candidate-level micro requests:** hidden-chord, audio salience, sheaf-noether.

Gate C is not reached, so phases D and E are not started.

## Expected product effect
None.
- Master MusicXML, Render MusicXML and PDF identical to r1214.
- Only the writer-trace lineage metrics change.
