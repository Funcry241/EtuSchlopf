# r1214 MasterLineageCompletionPreservingAndVariantReceipts

Parent: accepted/released **v0.10.1839 r1213**. Change class: **RefactorOnly**. No Kodex file changed. This is SSC-2 phase C, part 1.

## Starting point (r1213 released corpus, 10 projects)
160 PendingGaps:

| Gap reason | Count |
|---|---|
| `unmigrated_master_stage` | 98 |
| `unreceipted_patch_variant` | 29 |
| `master_request_replacement` | 14 |
| `arrangement_polish_and_layout` | 10 |
| `render_derived_master_reload` | 10 |

Continuity violations: 0. Master and Render 10/10 byte-identical.

## C3: `MasterPreservingReceipt`

### Contract
A receipt may be minted only by a frozen **PRESERVING exclusion owner**. The anchor is exclusion id + owner + owner signature + `PRESERVING`, and a unit test checks it against `destructive_transformation_exclusions_v1.tsv`. Minting is sealed through owner capability types, as for BLOCK and EXACT.

### Verifier
A receipt exists **only** if `conserved_content_projection(pre) == conserved_content_projection(post)`.

**Conserved** (in document order):
- parts and measures;
- per note: pitch or rest, duration, chord, grace, cue, voice, staff, tie types and visibility (`print-object`);
- backup and forward durations;
- divisions, key and time;
- barlines.

**Free:** beams, stems, clefs, notations other than ties, directions, print and layout.

If anything conserved differs, or either document does not parse, no receipt is minted and the lineage keeps an untrusted gap. The receipt is not a BLOCK substitute and grants no authority.

### Connected owners
Each receipt travels in the owner's existing return struct, so the frozen signatures stay unchanged.

| Exclusion | Owner | Carried in | Lineage effect |
|---|---|---|---|
| E051 | `polish_musicxml` | `PolishResult` | arrangement variant: GENESIS #62 → PRESERVING E051; Master: final beam polish in the initial-render stage |
| E056 | `apply_pianist_layout` | `LayoutResult` | arrangement variant: → PRESERVING E056 |
| E053 | `enforce_partial_beat_post_rest_beams_in_xml` | `PartialBeatPostRestBeamPatchStats` (no longer `Copy`) | Master initial-render stage |

`evaluate_arrangement_variant` validates the E051 and E056 receipts against the exact bytes (fail-closed, like #52) before attaching them.

## C1: patch-variant receipts

- `PatchedXml { xml, lineage_receipts }` and `PatchVariant { name, patched }` replace `(String, String)` in every Master patch-variant generator:
  - final critical takt, final playability (cleaner and polish), global hand trajectory (compound moves, mixed move+restify);
  - corpus overfull crop and corpus hand-rhythm.
- Migrated under the r1212 receipt-only rule (return type only):
  - #39 `apply_atomic_restification_batch_authorized` → `(Option<String>, Option<MasterExactReceipt>)`
  - #40 `apply_atomic_staff_voice_scalar_batch_authorized` → same
  - #25 `apply_voice_overfull_crop_edits_authorized` → same
- Receipt chains:
  - Overfull crop: EXACT #25 → BLOCK #52.
  - Mixed variants: EXACT #40 → EXACT #39.
  - Atomic-patch variants: EXACT #3, #4 or #63.
- `final_variants_to_candidate_inputs` passes the receipts to `build_atomic_candidate`. The lineage then accepts them only if they are continuous from the base state; otherwise an explicit gap is recorded.
- Receipts minted but deliberately not attached:
  - Render-side #40 in the silent-voice writer.
  - Editorial integration #40, which reaches the Master only through a `MasterXmlReplacementRequest`.
  - The inner #39 inside #63, which is the same edit.

## What remains for Gate C
- **Master stage wrappers:** barline+expression (ADDITIVE), octave policy, final-notation guard, export normalization, visual layout pressure, pianistic readability, Rustig clef (E024, whose frozen return type `Result<(), &'static str>` cannot carry a receipt).
- **Candidate-level writes:** hidden-chord reduction, audio salience, sheaf-noether.
- **`master_request_replacement`:** editorial and micro requests.
- **Next steps:** an ADDITIVE receipt type, and per-wrapper mapping of the inner surfaces to their population rows or exclusions.

## Metrics
The report gains:
- `preserving_receipts` and `preserving_refused_to_gap` in `MasterLineageMints`;
- `preserving_receipts (Exxx×n)` in `MasterLineageReleased`.

## Expected product effect
None.
- Master MusicXML, Render MusicXML and PDF identical to r1213.
- No selection or ranking change.
- Only the `MasterLineage*` report lines change.
