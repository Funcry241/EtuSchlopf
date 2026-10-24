# r1211 MasterIdentityFoundationAndCandidateLineageClosure

Parent: accepted/released **v0.10.1839 r1210**. Change class: **RefactorOnly**. No Kodex file changed.

## Phase A: Identity Foundation

### A1 Opaque minted identity
- `MintedIdentity` comes from a process-wide `AtomicU64` counter and is minted once per creation or transformation event.
- It is never derived from XML, fingerprints, locators, profiles or musical coordinates. `Debug` prints only `MintedIdentity(opaque)`, so the raw value never becomes output.
- Clone and move carry the same identity. Two independently created, byte-identical documents carry different identities (unit test).
- `validate_*` compares bindings only (descriptor contract, parameter digest, input binding, pre/post fingerprints) and never recomputes an id.

### A2 Minting only at the owner
Every mint function takes an owner capability value:
- `mint_genesis<O: GenesisMintOwner>`
- `mint_block_receipt<O: BlockMintOwner>`
- `mint_exact_receipt<O: ExactMintOwner>`

The capability traits are sealed, so only `master_state_identity` can bind an owner type to a descriptor. Each owner type has a private field and can be constructed only in its owner module:

| Owner type | Module | Row |
|---|---|---|
| `MidiSeedGenesisMint` | `midi_seed` | GENESIS #68 |
| `ArrangementGenesisMint` | `arrangement` | GENESIS #62 |
| `VisibleRhythmBlockMint` | `arrangement` | BLOCK #52 |
| `ClefMeasureBlockMint` | `structural_write_authority::render_measure_articulation_authority` | BLOCK #49 |
| `SafeTransferExactMint` | `structural_write_authority::remaining_atomic_patch_authority` | EXACT #3 |
| `StaffVoiceMoveExactMint` | `structural_write_authority::remaining_atomic_patch_authority` | EXACT #4 |
| `BeatScopedBeamingBlockMint` | `pipeline` | BLOCK #53 |
| `RestifyPatchExactMint` | `pipeline` | EXACT #63 |

Foreign modules can transport and validate an identity, but cannot mint one.

Granularity limit: Rust privacy is module-based, and all pipeline shards are `include!`d into the one `pipeline` module. The #53 and #63 capabilities can therefore be constructed anywhere in `pipeline`, not only in their own shard file. For #63 the EXACT mint is still byte-verified, so a forged receipt would have to be true about its bytes.

MIDI-seed GENESIS #68 is now minted inside `midi_to_musicxml_seed_with_profile_and_hotspots` itself. It travels out through `MidiSeedStats.master_genesis`, which keeps the frozen signature, and the solver takes it out immediately. Before r1211 it was bound by the caller.

### A3 Generic types
- `MasterGenesisIdentity`, `MasterBlockReceipt`, `MasterExactReceipt` and `MasterLineage` replace `ArrangementGenesisIdentity` and the #52-specific `MasterBlockIdentity`.
- The semantic anchor is `owner + function signature + effect contract + identity contract`.
- The population row number is reporting metadata only.
- A unit test checks every production descriptor against `src/build_gate/destructive_transformation_population_v1.tsv`.

### A4 BLOCK #52 generic
`normalize_visible_rhythm_musicxml` mints a `MasterBlockReceipt` through `VisibleRhythmBlockMint`. `build_atomic_candidate` validates it fail-closed, as in r1210, and then appends it to the candidate lineage.

### A5–A7 Lineage, PendingGap, continuity
- Structure: `MasterLineage { genesis, elements: Vec<Block | Exact | PendingGap> }`.
- `LineagePendingGap` carries only `pre_fp`, `post_fp` and a reason. It has no identity, no effect class, no receipt, no proof and no permit.
- `lineage_complete()` is false as soon as one gap exists.
- `lineage_continuity_holds(current)` checks:
  - GENESIS post equals the first pre;
  - element N post equals element N+1 pre;
  - the last post equals the current state;
  - minted identities are unique within the lineage.
- Appending a receipt whose pre is not the head, or whose identity is already present, is refused. The refusal is counted as a continuity violation and the stretch is bridged by an explicit gap (`continuity_refused_receipt`). There is never a silent jump.

### Phase-A gate (internal, checked before Phase B)
- No content-derived identity.
- Minting only through sealed owner capabilities.
- #52 generic.
- Lineage closed by construction.
- PendingGap untrusted.
- No new writer root: the harness reports no new writer-reachable function.
- No selector or ranking change.

## Phase B: Candidate Lineage Closure

### B1 GENESIS as start
- **MIDI-only candidate** (`no_polish` or pre-ledger blocker path): the lineage starts at the seed's own GENESIS #68, and its document is the seed output.
- **Arranged candidate:** the lineage starts at its own GENESIS #62, minted in `arrange_piano_musicxml_with_profile`. It is not a continuation of #68. `SolverCandidate.midi_seed_genesis` remains the separate seed-provenance field.

### B2 BLOCK #52 in the candidate lineage
`build_atomic_candidate` builds the lineage as:
1. base lineage, plus a gap if the base state moved without receipts;
2. the patch owner's receipts;
3. BLOCK #52 (normalization).

### B3 Transport
`master_lineage` is a non-optional field of `SolverCandidate` and of `ArrangementVariant`. It moves and clones with the object through:
- worker results (`parallel_indexed_results`, `evaluate_patched_candidates_parallel`);
- seed-equivalence reuse clones;
- `mem::replace` swaps and pool returns;
- `*best = …` drops;
- the hint-policy variant.

Nothing is recognised by fingerprint. Seed-equivalence reuse keeps the cloned lineage of the representative candidate whose execution produced the document. Only `midi_seed_genesis` is rebound to the concrete seed, as in r1207.

### Owner receipt transport
Owners whose frozen signature returns only XML mint inside their own body and deposit the receipt into a collection scope on the same thread. The immediate caller opens that scope with `collect_transformation_receipts(|| owner(..))`.
- The channel never crosses threads.
- A receipt minted without an open scope is counted and attached nowhere.
- A drop guard keeps scopes balanced on unwind.

This keeps every r1204-frozen owner signature and scanner surface unchanged.

### B4 BLOCK integrated
| Row | Owner | Where it enters a lineage |
|---|---|---|
| #52 | `normalize_visible_rhythm_musicxml` | every atomic candidate |
| #53 | `enforce_beat_scoped_beaming_and_rest_completion_in_xml` (BLOCK_DOCUMENT) | Master call in the initial-render stage |
| #49 | `replace_bound_clef_measure_authorized` (BLOCK_MEASURE, one receipt per measure, bound span kept) | Master lower-staff clef policy in project setup; HumanPotential hint-policy variant |

#49 stays BLOCK because the frozen contract says BLOCK, even though the owner has a single range edit. There is no upgrade to EXACT.

### B5/B6 EXACT integrated
| Row | Owner | Bound edits |
|---|---|---|
| #4 | `apply_bound_staff_voice_move_authorized` | voice/staff value ranges and the inserted `<staff>`, exactly the edits it applies in descending order |
| #63 | `apply_restify_patch` | AtomicPatch `note_start..note_end` and the restified block |
| #3 | `apply_bound_safe_transfer_authorized` | note replacement and insertion at `insert_after_original` in pre-state coordinates |

Each EXACT receipt is minted only if the edits reproduce the output byte-exactly and nothing outside the ranges changed. If an insertion falls inside the replaced range, no receipt is minted and the stretch becomes a PendingGap. Ranges come only from the owner's bound edits, never from XML geometry. The AtomicPatch ranges are kept.

### B7 Remaining PendingGap stretches
- Arrangement polish and layout (`polish_musicxml`, `apply_pianist_layout`; PRESERVING exclusions without receipts).
- Master stage mutations between candidate construction, the #49 call, the #53 call and release (all not-yet-migrated population rows).
- Patch variants without owner receipts (final, hand-trajectory and overfull-crop variants).
- Atomic patches whose base state moved without receipts.
- The Render-derived Master reload at release.

Lineage is never reported as complete while any of these exist.

## Metrics
Appended to the existing render-epoch section of the result report:
- `MasterLineageContract`
- `MasterLineageMints`: genesis, block, exact, exact refused to gap, attached, minted without collection.
- `MasterLineageCandidates`: lineages started with valid GENESIS, candidates without GENESIS (0, type-enforced), GENESIS loss on clone/move/worker/swap (0, type-enforced), continuity violations, gaps bridged.
- `MasterLineageReleased`: GENESIS row; Block, Exact and Gap counts per row and reason; `lineage_complete`; continuity of the released Master.

No identity value is reported. The metrics are projection only and never authority.

## Not in this take
- `layout_xml` encapsulation
- conversion of the ~30 Master write sites
- adoption API
- Render→Best reload removal
- Master export restructuring
- Render AdoptedState
- WAV
- AccentReserve
- Proof/Permit
- enforcement

Rows 57 and 67 remain `DEFERRED_RENDER_PREREQUISITE`. The r1205 boundary `Pipeline/Trial → immutable Request → writer-reachable Owner → StructuralWriteAuthority` is unchanged. No pipeline helper gains a writer root.

## Expected product effect
None.
- 10/10 projects.
- Master MusicXML, Render MusicXML and PDF identical to r1210.
- Only the result report gains the four `MasterLineage*` lines.
