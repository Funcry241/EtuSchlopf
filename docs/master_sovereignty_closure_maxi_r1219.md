# r1219 MasterSovereigntyClosureMaxi

Parent: accepted/released **v0.10.1839 r1218** `NestedFrozenOwnerReceiptTransportKodex`
(`8fd6056f56318fe7334a9f5d390ec0290ad0af6d6e9335516d01840142c1d8ec`). Change class: **RefactorOnly**.
No Kodex file, population row, exclusion row, classification, domain or physical write budget changes.
Expected product effect: none (Master/Render MusicXML byte-identical, PDF pixel-identical).

## Architecture after r1219

```text
MasterAdoptedState (GENESIS #68 / #62)
  -> early Master owners, each minting its own neutral receipt at origin
  -> candidate / request adoption, fully covered (bind_with_owner_receipts only)
  -> #56 BLOCK_DOCUMENT  (cursor-lane-end closure, whole call)
  -> #65 EXACT           (two-phase script composed into pre coordinates)
  -> #59 EXACT           (opening-key mirror)
  -> #60 EXACT           (functional respelling, nested 59 -> 60)
  -> E007 PRESERVING     (final layout/phrase owner, empty phrase intents; E011 stays a sink)
  -> FinalNotationLayoutSnapshotSet.master = sovereign MasterAdoptedState + full lineage
  -> artifact write -> byte-exact readback witness (fatal on mismatch)
```

No Master state is created from the Master file. The file is only an artifact and integrity witness.

## 1. Adoption contract (Gate C)
- `LineageGapReason`, `LineagePendingGap`, the gap bridge and every external `lineage_extend_owner_receipts` are removed.
  - The gap reasons `UnmigratedMasterStage`, `MasterRequestReplacement`, `HintPolicyVariant`, `UnreceiptedPatchVariant`, `ArrangementPolishAndLayout` and `RenderDerivedMasterReload` no longer exist.
- `MasterLineage::commit_owner_receipts` requires a `LineageCommitBoundary` capability. Only `MasterAdoptionCommitBoundary` implements it, and only the adoption module constructs it.
- Every adoption must cover its transition completely and continuously. A violation is fatal (`MASTER SOVEREIGNTY VIOLATION`):
  - a receipt pre-state that is not the lineage head;
  - a duplicate identity;
  - an uncovered remainder;
  - a changed document without a receipt.
- There is no candidate fallback and no alternative selection after a failure. Trials, workers, clones and candidates all use the same `MasterAdoptedState` API.
- Receipts are move-only and are converted into `CommittedTransformationEvent`s on adoption.
  - Variants carry their own events: patch variants and micro deltas re-execute their bound owner, and a byte mismatch is fatal.
  - Only committed history is clonable.
- `MasterXmlReplacementRequest::bind` is removed; requests use `bind_with_owner_receipts` only.

## 2. Receipt kinds
- **EXACT**: the geometry is known at origin; `exact_edits_reproduce` must hold.
  - `exact_script_from_descending_application` canonicalizes the start-descending application of the owner's own bound edits, merging same-start edits in application order.
  - `compose_exact_edit_scripts` folds a second bound script into the coordinates of the original pre state (used by #65 and the #60 final-section case). No post diff is used.
- **BLOCK**: binds pre and post of the frozen owner's whole call.
- **PRESERVING**: unchanged contract, the conserved content projection must be identical.
  - `mint_file_bound_preserving_receipt` serves path-bound owners (E007). The owner captures its real pre bytes before its sink runs. Its own verified post bytes, read back inside the same transaction, travel move-only in the receipt and are bound to the post fingerprint.
- **ADDITIVE**: insertions only (`start == end`), each exactly one element of the sealed `AdditiveInsertionClass` (E004 `barline`, E036 `direction`); the class is tested against the frozen reason text.

## 3. Owner coverage

| Owner | Kind | Notes |
|---|---|---|
| E004 / E036 | ADDITIVE | |
| #50 / #49 | BLOCK_MEASURE | |
| #5 / #6 | EXACT | |
| E055 / E044 / E013 / E051 / E056 / E053 / E024 | PRESERVING | |
| #53 / #52 | BLOCK_DOCUMENT | |
| #8 / #11 / #13 / #26 / #27 | EXACT | |
| #3 / #4 / #63 / #25 / #39 / #40 / #1 | EXACT | existing |
| #56 | BLOCK_DOCUMENT | |
| #65 | EXACT | composed two-phase |
| #59 / #60 | EXACT | nested r1218 return `(io::Result<()>, Option<Exact>, Option<Exact>)`, descriptors checked by #59 |
| E007 | PRESERVING | |

- **Wrapper rule:** wrappers forward inner owners' receipts and mint none.
  - #56 owns its whole call; the inner #24/#36/#54 attach no events to that invocation.
  - The fragment rewriters #7/#12/#14 only produce bound replacement fragments; the enclosing EXACT owners (#26/#27/#13) carry the edits.

## 4. TonalFrameFact (#59/#60)
- The metric stage binds a sealed `TonalFrameFact` to the Render snapshot it reads. The fact holds:
  - opening fifths and the fifths vector;
  - the explicit key vector;
  - the score-wide final bass;
  - the tail bass at the key boundary;
  - the bound snapshot bytes and their fingerprint.
- The retired thread-local Render->Master view is evaluated once for comparison; any divergence is fatal.
- The carrier `FunctionalMasterRespellingOperation` re-derives the fact from its bound bytes before the frozen `&str` crosses into #60. #60 derives the same fact from that `&str` and consumes only its fields.
- #60 mints its EXACT receipt at origin from the planner's own geometry: note edits plus one of the three key-signature edit cases (replace key, insert into attributes, insert new attributes). It compares fail-closed against the legacy algorithm's bytes.

## 5. Released Master (Gate D)
- The #56 receipt travels with `InitialRenderTransaction`. The #65/#59/#60 receipts and the export document travel in `CommittedMasterExport`; the document is realized from the EXACT scripts.
- The written export file must equal that document byte for byte.
- The release support adopts #56 -> #65 -> #59 -> #60, then the E007 transition, into `best.master_state` and witnesses the artifact bytes.
- `FinalNotationLayoutSnapshotSet.master` carries that sovereign state, including GENESIS and the full lineage.
- The reload in the release stage is removed.

## 6. Report
`MasterSovereigntyClosure` reports these closure metrics:
- MasterSovereigntyClosed
- MasterLineagePending
- AdoptionWithoutGenesis
- ContentChangingMasterAdoptionWithoutIdentity
- MasterContinuityViolations
- MasterReceiptCoverageFailures
- MasterOwnerWriteWithoutReceipt
- MasterCandidateFallbackAfterReceiptFailure
- MasterArtifactReadbackMismatch
- LegacyMasterReloadCount

`MASTER SOVEREIGNTY CLOSED` is printed only when continuity holds and the artifact witness matched.

## 7. Deliberately remaining
- E011 remains an `IMPLEMENTATION_TRANSPORT_SINK` without a receipt.
- The thread-local Render checkpoint stays for Render-side checkpoint validation. Its only Master use is the one fail-closed comparison in `bind_tonal_frame_fact_from_render_snapshot`.
- Render-side owners discard their receipts; they never enter a Master lineage.
