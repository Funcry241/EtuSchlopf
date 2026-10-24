# r1027 r02 — MusicXML 4.0 child-order validation closure

The first r1027 candidate reached the real Etüde-7 product run with PRE-TAKE, Clippy, Build and all 1332 tests green, but the persisted-byte MusicXML-4.0 XSD gate correctly rejected the newly materialized rest note in T20. The generated note child sequence was `rest, duration, type, voice, staff`; MusicXML 4.0 requires `rest, duration, voice, type, ..., staff`. The public-artifact transaction rolled back completely and the candidate remains rejected failure evidence.

r02 keeps the exact r1027 authority, selector, rest/pitch split, terminal FinalPdfSourceCommit location and committed residual readback. It changes only the serialization of the two generated edge-rest notes inside the same existing local decision block by reusing the already-existing schema-ordered `render_visible_rest_note_fragment` serializer (`rest -> duration -> voice -> type -> dot* -> staff`). The shared historical `beat_scoped_visible_rest_note` helper is intentionally not changed, avoiding unrelated call-site expansion. No new writer, writer caller, helper function, authority, selector, threshold or test is added. Expected strict test cardinality remains 1332.

---

# Same-Lane Rest/Pitch Final-PDF-Source Commit Closure — r1027

## Parent and rejected evidence

`r1027 SameLaneRestPitchFinalPdfSourceCommitClosure` builds logically on accepted freeze parent `r1023 ShadowRestInvocationEvidenceReportCompatibilityClosure`.

`r1024`, `r1025` and `r1026` remain rejected failure evidence and are not parents.

- r1024 proved that the early `LowerVisibleRhythmRestLane` planner can construct the intended split, but the mutation does not survive to the persisted final Render-MusicXML.
- r1025 proved that `FinalVisibleRestRole` is still too early.
- r1026 proved that even the pre-checkpoint FinalNotation projection is not the terminal ownership epoch for this rest shape: its Etüde-7 run was technically green, but the persisted final product still contained `Rest [0,96)` overlapping `Pitch [48,72)` and `SameLaneVisibleRestPitchOverlapPairs` remained `1`.

## Newly bound terminal owner

The selected Render source is finally committed through the already-existing structural authority boundary:

```text
FinalPdfSourceCommit
  -> materialize_silent_voice_visible_measure_coverage_for_pdf_source
  -> transport_final_pdf_source_occurrence_continuation_through_final_notation_bundle
  -> write_final_notation_checkpoint_authorized
  -> exact persisted readback
```

This materializer already owns the terminal candidate composition for silent-voice coverage, orphan-chord host normalization, clef continuation and final rest placement. It is therefore the first location where the strict-interior Rest/Pitch closure can be both applied and verified against the exact committed bytes without adding a new writer, sink or authority.

## Product mutation

Inside the existing `materialize_silent_voice_visible_measure_coverage_for_pdf_source` function, r1027 adds one identity-free, score-wide, fail-closed candidate decision. No new named productive helper function is introduced.

A candidate must satisfy all of the following:

- visible positive-duration rest;
- Staff 2, non-primary voice;
- no chord syntax;
- exactly one visible non-chord pitched note in the same staff/voice lane overlapping the rest;
- that pitch is strictly interior to the rest;
- no overlapping visible pitch from another voice on the same staff;
- no overlapping other visible rest on the same staff;
- both remaining edge silences are each representable by exactly one canonical metric rest;
- score-wide candidate cardinality is exactly one.

For the accepted r1023 corpus the intended witness remains:

```text
Etüde 7 / T20 / Staff 2 / Voice 2
Rest  [0,96)
Pitch [48,72)
```

The edge-touching comparator cases remain read-only:

```text
Etüde 3 / T26: Rest [0,48),    Pitch [24,48)
Etüde 6 / T13: Rest [144,192), Pitch [168,192)
```

The unique strict-interior replacement is:

```text
Rest    [0,48)
Forward [48,72)
Rest    [72,96)
```

The existing pitched event is not moved, copied, deleted or respelled. Existing rest display coordinates are copied to the two visible edge rests.

## Invariants and terminal readback

Before the existing FinalPdfSourceCommit sink accepts the candidate, r1027 requires:

- XML parse before/after green;
- backup/forward parser state before/after green;
- raw cursor signature unchanged;
- pitched timeline signature unchanged;
- pitched payload signature unchanged;
- visible-rest cardinality exactly `+1`;
- note cardinality exactly `+1`;
- no strict-interior candidate remains in the in-memory candidate.

The existing exact persisted readback is then strengthened by one additional condition: the same identity-free scanner must report `0` strict-interior candidates on the committed Render bytes. Any residual candidate makes `CommittedStateReadbackGreen` false and therefore produces the existing FinalPdfSourceCommit release veto.

A diagnostic line is appended to the existing writer report:

```text
FinalPdfSourceSameLaneStrictInteriorClosure:
  Candidates <n>;
  SplitAccepted <ja/nein>;
  CommittedResidual <n>;
  FinalPdfSourceCommitBound ja;
  Selector identity_free_scorewide_exactly_one;
  ReleaseVeto <ja/nein>.
```

## Authority contract

No new writer, writer caller, filesystem sink, StructuralWriteAuthority function, capability enum, threshold, project selector, measure selector or pitch selector is added.

The only physical sink remains the existing `write_final_notation_checkpoint_authorized` reached from the already-existing `FinalPdfSourceCommit` structural authority.

No new source-level test directly calls the productive writer. Expected strict test cardinality therefore returns to the accepted r1023 baseline: `1332/1332`. Product acceptance is the real Etüde-7 run plus committed readback.

## Physical rejected-tree recovery

The locally installed tree after the technically green but musically rejected r1026 run is physically r1026. r1027 does not promote it to a parent.

The incoming package explicitly restores both r1026 projection files byte-for-byte to accepted r1023 and deletes the r1026 documentation:

- delete `docs/same_lane_rest_pitch_final_notation_projection_closure_r1026.md`;
- restore `src/pipeline_shards/composer_beat_scoped_rest_completion_planning_core.rs` exactly to r1023;
- restore `src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs` exactly to r1023.

Relative to logical parent r1023, the only productive r1027 source change is `src/render/silent_voice_visible_measure_coverage_render_writer.rs`.

## Acceptance

Early gate: `Etüde Nr. 7`.

Required:

- PRE-TAKE gates green;
- Clippy 0;
- `1332/1332` tests green;
- Master MusicXML byte-identical to r1023;
- all pitched event tuples unchanged;
- `FinalPdfSourceSameLaneStrictInteriorClosure: Candidates 1; SplitAccepted ja; CommittedResidual 0`;
- final E7 Render-MusicXML T20 contains Rest `[0,48)`, Forward `[48,72)`, Rest `[72,96)`;
- final `SameLaneVisibleRestPitchOverlapPairs` becomes `0`;
- no E7/T20 `SAME_LANE_REST_PITCH_OVERLAP` row remains;
- PDF delta, if any, is local to E7/T20.

If the persisted final audit still reports the E7/T20 row, r1027 is rejected regardless of otherwise green run status.
