# r1118 VisibleRestElisionRhythmicObligationRestOnlyOwnerClosure

Parent: accepted v0.10.1839 r1117 `VisibleRestElisionRhythmicObligationErrorClassDefinition`. ChangeClass: `MusicMutation`.

## Purpose

r1118 is the first deliberately local enforcement of the causal class frozen in r1117. It changes the insufficient RestOnly deletion rule and preserves the already-existing positive technical-shadow decision downstream without adding a new proof system.

The old RestOnly branch treated a full rest-only serialization segment as an auxiliary lane whenever the same staff contained any visible pitched event elsewhere in the measure. With exactly one visible rest in that segment, the owner converted the rest to `<forward>` even though staff activity outside the affected interval did not prove the rest rhythmically dispensable.

r1118 removes that positive deletion authority. Same-staff activity remains an observed candidate condition only. The already visible rest is preserved unless another already-authorized path independently proves and performs a permissible transformation.

The full product-path replay then exposed a second half of the same local obligation. The existing canonical FinalVisibleRestRole owner can later positively classify such a preserved carrier as `AUXILIARY_SHADOW_BOOKKEEPING_REST_LANE`. That positive C2 classification is allowed to remove the rest. However, serializing the technical purge as a voice/staff-labelled forward can cause the later SilentVoice coverage planner to reconstruct the retired technical lane as visible rests. r1118 therefore changes only the representation of an already positively selected C2 purge to identity-neutral cursor time.

## Productive delta

### 1. RestOnly owner

Changed function: `apply_rest_only_staff_empty_measure_readability` in `src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs`.

- `staff_has_visible_pitched` now increments only `auxiliary_lane_candidates_seen` and returns `Keep` for that rest-only segment.
- It no longer increments `auxiliary_lane_attempted` and no longer emits a rest-to-forward replacement from that condition.
- The old bounded-scan action slot and overlap reservation are retained internally for candidates that would previously have attempted the unsafe deletion, so this suppression does not unlock unrelated later writes.
- The existing no-visible-pitch canonical measure-rest path is unchanged.
- The existing same-voice cross-staff guard for that measure-rest path is unchanged.
- Structural writer authority is unchanged.
- No hidden rest is made visible and no arbitrary forward is converted into a rest.

### 2. Existing canonical C2 shadow-rest purge

Changed materialization only in `apply_final_visible_rest_role_purge_and_vertical_placement_write` in `src/pipeline_shards/staff_voice_rest_glyph_topology_tail.rs`.

- The existing selector and `atomic_visible_rest_is_shadow_bookkeeping_candidate` decision are unchanged.
- Voice-1 protection, overlap requirements, mixed-review protection, and all candidate counts are unchanged.
- A carrier already positively classified as `AUXILIARY_SHADOW_BOOKKEEPING_REST_LANE` is converted to an ordinary `<forward><duration>...</duration></forward>` rather than a forward carrying the retired rest's voice/staff labels.
- This prevents a downstream silent-coverage planner from treating the technical cursor carrier as evidence for a real silent voice lane.
- No new rest is deleted by this change; only the cursor representation of an already-authorized deletion changes.

## Synthetic regression

No new writer-reaching test symbol is added.

The pre-existing `quarter_rest_plus_forward_coverage_becomes_one_measure_rest` regression remains extended with:

1. a partially active 4/4 staff whose separate rest-only segment contains one visible rest: candidate observed, no deletion attempted, bytes unchanged;
2. the proportional case in 3/4 with different divisions and a different technical voice label: same `Keep` result;
3. bounded-action parity: suppressing an unsafe deletion does not free a slot for a later unrelated measure-rest mutation;
4. an independent two-measure duplication: both observations remain local `Keep` decisions and scorewide multiplicity creates no write authority.

The pre-existing `canonical_role_owner_detects_majority_covered_auxiliary_shadow_rest` classifier regression remains unchanged. No test outside `StructuralWriteAuthority` calls the writer merely to prove this representation-only change. The real take execution and final persisted readback must prove that a positively selected technical shadow rest is still purged without recreating a voice/staff-labelled silent lane.

Existing positive measure-rest and canonical shadow-role boundaries remain the neighboring authorized behavior.

## Product-path evidence motivating the second local serialization correction

The rejected diagnostic r1118 attempt reached all tests and Clippy successfully, then failed closed in the real Etude-6 execution at `FinalPdfSourceCommit`. The committed readback was green for clef, rest placement, higher beam, lost begin, partial-beat lane split, lower dual edge, tied-tail accent, cursor, and pitched-event multiset. The sole residual was three `CommittedShadowRestCandidates` in one full technical lane.

Those three rows exactly reconstructed a full voice/staff-labelled silent lane after the canonical shadow-rest purge. This evidence is used only to identify the owner interaction; no project, measure, pitch, or corpus identity appears in executable selection logic or normative tests.

## Scope boundary

This take does not claim that all rest-elision paths are closed, does not assert that every visually incomplete measure is repaired, and does not make any corpus locator an acceptance condition. It closes only the existing RestOnly rule whose deletion authority was based solely on staff activity elsewhere in the measure, plus the downstream re-materialization loop of carriers already positively proven technical by the established canonical role owner.

Expected musical effect: fewer unjustified visible-rest-to-forward rewrites; positively proven technical shadow rests remain suppressible without being reconstructed as a visible silent lane; sounding notes, pitch, duration, chord/tie structure, cursor duration, hidden-rest policy, and unrelated voices remain unchanged.
