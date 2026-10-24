# r1064 RhythmicSpineRetiredRuntimeGraphPhysicalRetirement

## Change class

`RefactorOnly`

## Parent

This candidate requires canonically accepted `v0.10.1839 r1063 RendererExplicitCliRejectImmediateFallbackClosure` as its parent. The current assistant export was created during the successful r1063 reexec and therefore still projects r1063 as `current_in_flight` / `not_yet_canonical`; if the local repository has not subsequently persisted the r1063 acceptance record, this candidate must fail closed and must not become a parent.

## Why this graph is dead

MI-A01 `RhythmicSpineOneTenutoRenderWrite` lost its positive productive Tenuto authority in accepted r1037. The surviving runtime island still parses and classifies the current Render-MusicXML, builds and score-wide sorts RhythmicSpine lane candidates, chooses one lane and a Hinge note, computes invariant evidence, transports a dedicated audit through FinalNotation -> release evidence -> final report, and then reaches `retired_rhythmic_spine_tenuto_noop()`, which always returns `None`.

The current r1063 corpus makes the dead-work property concrete:

- Etude 1: 0 candidates, 0 writes;
- Etudes 2..10: 71 candidates in aggregate, 0 writes;
- every candidate-bearing project terminates as `rhythmic_spine_writer_retired_noop`;
- `RhythmicNotationChange nein`, `RenderXmlWrite nein`, and Master-MusicXML unchanged in all ten projects.

The runtime graph therefore cannot alter Render-MusicXML, cannot select a PDF source, and cannot create a live release decision. Its remaining observable effect is only the retired diagnostic section in `.result.txt`.

## Physical retirement

r1064 removes the complete compile/runtime/report island:

1. delete `src/pipeline_shards/rhythmic_spine_one_tenuto_apply.rs`;
2. remove its `include!` from `pipeline.rs`;
3. remove the dead FinalRenderNotation invocation and its audit carrier;
4. remove the permanently-false RhythmicSpine row from `RenderPdfSourceEvidenceLedger`;
5. remove the dead release-evidence transport;
6. remove the final report section `RHYTHMIC-SPINE-ONE-TENUTO-RENDER-WRITE`;
7. retire the three unit tests that exclusively exercised the already-retired no-op graph.

The historical `thirty_surface_batch_manifest.tsv` row is intentionally retained. Its validator explicitly permits the legacy owner symbol occurrence count to fall to zero after productive cutover; changing that pinned governance manifest is neither necessary nor part of this runtime cleanup.

Strict Clippy on the first physical-retirement candidate proved that removing the last runtime caller also leaves three compile-time residues unreachable: the public re-export, `RenderCheckpointStage::RhythmicSpineTenuto` plus its tag arm, and the corresponding `render_checkpoint_entry!` invocation. These residues are removed in the same RefactorOnly physical-retirement closure because they are otherwise rejected by `#![deny(unused_imports)]` / `#![deny(dead_code)]`.

No `authority_retirement_v1` contract is declared. The repository's own authority-retirement detector rejected that contract for this surface because the macro-generated checkpoint residue does not constitute a detected pure monotone productive writer-authority loss. This take therefore follows the already-proven RefactorOnly path: remove the dead runtime/report island and the compiler-proven orphaned residue, without claiming a semantic writer-authority transition.

## Mechanical balance

Relative to r1063 before adding this contract document:

- deleted retired shard: 394 Rust lines;
- removed include/transport/invocation/report plumbing: 40 Rust lines net;
- total Rust reduction: 438 lines net;
- exactly three `#[test]` cases disappear, all owned exclusively by the deleted no-op graph.

The r1063 run contains 1,390 passing tests. Expected r1064 strict test inventory is therefore **1,387**, assuming no unrelated test inventory change. Any additional unexplained test loss is a blocker.

## Intended external behavior

Unchanged:

- Master `.musicxml` bytes/semantics;
- final `.render.musicxml` bytes/semantics;
- final PDF source selection and renderer behavior;
- pitch, onset, duration, staff, voice, chord, tie, beam, rest, clef and layout semantics;
- Candidate breadth, Solver behavior and ordering;
- all live Writer/Repair/Selection/Release authorities;
- fail-closed gates, receipts and provenance;
- r1063 renderer immediate-fallback closure.

Expected `.result.txt` delta only:

- the historical `RHYTHMIC-SPINE-ONE-TENUTO-RENDER-WRITE` section disappears.

This disappearance is intentional because the section is produced solely by the physically retired no-op graph. No live diagnostic or release veto is removed.

## Performance expectation

This is primarily a compile/runtime graph retirement, not a promised seconds-level optimization. It removes 438 Rust lines from Clippy/build/test compilation and avoids the per-project RhythmicSpine parse/candidate/ranking/invariant/report work. Because current Clippy/Build/Test timings are noisy, acceptance must not be inferred from a single small wall-clock delta.

The r1063 run already demonstrated why compile-side cleanup is worthwhile: `ProgramMs` improved strongly after r1063, while Clippy, Strict-Build and test compilation varied upward in that run. r1064 should be judged first by exact physical retirement and unchanged artifacts, then by timing evidence.

## Required run

```powershell
.\run.cmd --take --all
```

Accept only if:

- r1063 is the canonical accepted parent;
- PRE-TAKE and semantic-capability gates remain green;
- Strict Clippy and Build remain green;
- expected strict test inventory is 1,387 with 0 failures;
- full corpus remains 10/10 dark-green;
- Master MusicXML and Render MusicXML remain unchanged except for no intended XML delta at all;
- PDFs remain semantically stable;
- the only intended result-report removal is the retired RhythmicSpine section.
