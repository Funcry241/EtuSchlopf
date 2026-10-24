# r1121 SilentVoiceCanonicalShadowRestConflictOwnerClosure

Parent: accepted v0.10.1839 r1120 `SilentVoiceCanonicalShadowRestConflictErrorClassDefinition`. ChangeClass: `MusicMutation`.

## Purpose

r1121 closes the accepted r1120 causal class at the existing `SilentVoiceVisibleMeasureCoverage` owner.

The defect is a contradiction between two existing truths: SilentVoice may propose visible rests from cursor carriers while the canonical FinalVisibleRestRole predicate, under the same frozen serialized voice relation, classifies those newly visible rests as auxiliary shadow bookkeeping.

## Productive delta

Changed files:
- `src/render/silent_voice_visible_measure_coverage_render_writer.rs`
- `src/pipeline_shards/staff_voice_rest_glyph_topology_tail.rs`

The canonical shadow-rest module gains only a read-only comparison helper. It computes how many canonical shadow-rest candidate rows are newly introduced between two XML states using the exact existing bound predicate.

The existing SilentVoice writer now receives the already available `FrozenSerializedVoicePlan` from the FinalNotation bundle. Before granting a concrete forward-to-rest replacement write authority, it probes that replacement through the existing structural materialization authority and asks the canonical predicate whether the concrete result introduces new shadow-bookkeeping rests.

Positive conflict means that replacement is withheld. Missing or unbound canonical evidence does not create positive repair authority. Non-conflicting replacements remain eligible under the existing parse, cursor, sounding-event and exact-delta guards.

When a mixed plan writes only non-conflicting replacements, residual SilentVoice candidates are permitted only if all residual concrete materializations are positively proven to reproduce the same canonical conflict. This prevents the old `candidate_rows_after == 0` invariant from forcing an all-or-nothing rewrite while keeping the exception tightly bound to the accepted r1120 class.

## Preserved boundaries

- no new writer or filesystem sink;
- no new overlap heuristic;
- no change to the canonical FinalVisibleRestRole selector;
- no change to the committed `CommittedShadowRestCandidates == 0` veto;
- no pitch, onset, duration, chord, tie or cursor mutation beyond the pre-existing SilentVoice replacement domain;
- no corpus locator is executable authority;
- no global disablement of legitimate silent-voice coverage.

## Expected product effect

Previously blocked projects whose only FinalPdfSource residual is a SilentVoice-created canonical shadow rest should no longer materialize those conflicting visible rests. Legitimate non-conflicting SilentVoice coverage remains available. The full corpus and persisted PDF-source readback remain the acceptance evidence.
