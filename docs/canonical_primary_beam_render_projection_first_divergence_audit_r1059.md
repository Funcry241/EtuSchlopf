# r1059 CanonicalPrimaryBeamRenderProjectionFirstDivergenceAudit

## Classification

- Parent: r1058 `CanonicalPrimaryBeamAuthorityBindingClosure` **only if r1058 is canonically accepted before execution**
- Change class: `AuditOnly`
- MusicMutation: no
- Productive Beam logic: unchanged
- Writer / repair authority: none
- Rule change: no
- Project/measure selector: none

## Purpose

r1058 left one exact current-canonical owner/persisted-render primary-Beam state mismatch. r1059 does not encode that witness as a selector. It extends the already existing real Render-MusicXML writer trace boundary with a compact automatic primary-Beam Level-1 delta trace for every authorized Render-MusicXML write in every project.

## Observation contract

A delta row may be emitted only when a note remains exact across one successful writer boundary by ordinal, onset, duration, staff, voice, pitch/rest identity and chord flag. Only its primary MusicXML Beam Level-1 state is compared. Note-count or structural-identity changes are not heuristically rebound.

The result report receives the always-present section:

```text
CANONICAL-PRIMARY-BEAM-RENDER-WRITER-DELTA-TRACE
```

with ordered rows such as:

```text
PrimaryBeamWriterDeltaRow: writer_ordinal=... writer=... measure=...
PrimaryBeamWriterDeltaChange: note_ordinal=... onset=... duration=... staff=... voice=... pitch=... primary_beam=continue=>begin
```

This makes the first persisted Beam divergence visible in the ordinary Assistant Export without `--writer-trace` and without a second manual diagnostic command.

## Non-authority boundary

The trace is evidence only. It cannot veto, repair, select, rank, release, mutate, or create Beam authority. It does not claim renderer geometry. The existing optional full writer trace remains optional and unchanged in purpose.

## Regression coverage

Two new audit tests require that the automatic trace records an exact primary-Beam state change without CLI activation and ignores a non-Beam XML delta when the structurally identical note keeps the same Beam state. Parent strict count is expected to be 1381; expected r1059 strict count is 1383.

## Explicit non-goals

r1059 does not change `.musicxml`, `.render.musicxml`, `.pdf`, `polish_musicxml`, BeatScopedBeaming, ScorewideMetricBeamContinuity, ExternalFeedback, any Beam writer, or any repair selector.

## r01 rejection / r02 closure

The first r1059 candidate was rejected before compilation by `PROXY-IDENTITY SELECTOR KODEX` because its implementation contract omitted the mandatory causal-feature necessity review and did not encode the complete Stage-B implementation contract. r02 is rebuilt independently from accepted r1058. It keeps the Rust trace and report projection semantically unchanged and binds them to the already accepted `final_primary_beam_serialization_voice_partition_fragmentation` error-class contract. No new error-class definition is born in r1059.
