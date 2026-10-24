# r1065 PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplay

## Change class

`AuditOnly`

## Parent

This take requires canonically accepted `v0.10.1839 r1064 RhythmicSpineRetiredRuntimeGraphPhysicalRetirement` as its parent.

Canonical acceptance identity supplied for the parent:

`6d699c0b2de4684197d6dd67d6740eb69c45cb2135e4825b6b6a8942135136b5`

No rejected r1062 PrimaryBeam materialization candidate is a parent or source authority for this take.

## Why this take exists

The accepted r1061 audit freezes the repair scope for
`final_primary_beam_serialization_voice_partition_fragmentation` as the intersection of:

1. a renderer-visible frozen class member proven from current canonical PrimaryBeam authority; and
2. the independently proven pre-serialization MaterializationSafetyConjunction.

The current r1064 corpus still reports 121 proven frozen-class members, 18 unresolved members, 138 materialization-safe candidates and 94 repair-scope-eligible metric lines. The old r1062/r07 MusicMutation reached full-corpus execution but all ten children returned exit 1. At that time child stderr was still discarded. Accepted r1062 subsequently changed only batch-child stderr visibility; it did not rerun or prove the rejected mutation.

Therefore r1065 does not repeat r07 and does not guess its runtime failure. It closes the missing observation boundary immediately before any future productive voice-scalar mutation.

## Existing authority only

The new evidence replay consumes no new selector and creates no writer.

Selection is exactly the already accepted r1061 set:

`PROVEN_FROZEN_CLASS_MEMBER ∩ MATERIALIZATION_SAFETY_CONJUNCTION_PROVEN`

`metric_line_id` remains an opaque equality join key only.

The primary target is not inferred from a voice number, staff number, PDF geometry or historical witness. During the already-existing MaterializationSafetyConjunction, the frozen serialization channel plan already proves one existing same-staff primary channel for every safe current-split metric line. r1065 carries one source-event/attack member already bound to that channel as typed audit evidence. It does not change the channel plan.

At S1, the replay binds that source and every selected member through:

`SourceEvent/Attack -> canonical occurrence receipt -> persisted Render-MusicXML physical pitched occurrence -> current <voice> scalar`

The current `<voice>` value of the proven primary target source becomes the observed target scalar for the dry run. Thus no canonical voice literal such as `1` or `2` is guessed and no label-alpha assumption is introduced.

## Replay checks

For every r1061-eligible metric line the audit requires:

- exactly one current metric candidate;
- exactly one carried MaterializationSafety primary-target evidence row;
- the target source is a member of that exact metric line;
- every member binds to exactly one persisted physical pitched occurrence through current canonical occurrence identity plus S0 geometry;
- the target scalar already exists in that selected line;
- all selected source events are globally source-disjoint;
- all proposed physical edit footprints `(measure, pitched_ordinal)` are globally disjoint;
- each dry-run edit is only `current_voice -> existing_primary_target_voice`.

It reports the exact proposed scalar transitions but does not call `apply_atomic_staff_voice_scalar_batch_authorized`, does not mutate a String copy, does not create a checkpoint, and does not claim renderer improvement.

## Productive behavior

Unchanged by construction:

- Master `.musicxml`;
- final `.render.musicxml`;
- PDF;
- Pitch;
- Onset;
- Duration;
- Staff;
- Chord syntax;
- Ties;
- Beam tags;
- Voice/Channel inventory;
- candidate breadth and solver semantics;
- renderer behavior and release gates;
- all writer, repair, selection and release authorities.

Only `.result.txt` gains the read-only section
`PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplay*`.

## Expected corpus evidence

The accepted r1064 corpus has 94 repair-scope-eligible metric lines. r1065 does not hard-code that number as selection authority, but on the same corpus the evidence replay is expected to report:

- `EligibleMetricLines: 94` in aggregate;
- one target-evidence binding for every eligible metric line;
- zero unresolved member occurrences;
- zero source-overlap conflicts;
- zero physical-edit-footprint conflicts;
- `ReadyMetricLines` equal to `EligibleMetricLines`.

Any mismatch is evidence against immediately retrying the MusicMutation and must remain fail-closed at the analysis boundary.

## Test inventory

r1064 has 1,387 strict tests. r1065 adds exactly one read-only replay unit test and changes assertions in one existing MaterializationSafety test without removing tests.

Expected strict test inventory: **1,388**.

## Required run

```powershell
.\run.cmd --take --all
```

Accept only if:

- r1064 is the canonical accepted parent with the SHA above;
- PRE-TAKE / semantic-capability / change-control gates remain green;
- Strict Clippy and Build remain green;
- 1,388 tests pass with 0 failures;
- the corpus remains 10/10 dark-green;
- `.musicxml`, `.render.musicxml` and `.pdf` remain unchanged;
- the new replay remains read-only and reports enough exact evidence to decide the next PrimaryBeam MusicMutation without guessing.
