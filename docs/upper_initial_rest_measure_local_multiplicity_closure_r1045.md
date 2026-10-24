# r1045 - UpperInitialRestMeasureLocalMultiplicityClosure

## Parent and purpose

Parent: r1044 `UpperInitialRestMeasureLocalCausalClassAndDebtSync`.

r1044 froze the witness-blind MI-A04 causal class before any selector-sensitive implementation change. r1045 implements that accepted class by removing scorewide ranking and the global initial-rest cap as write authority for already-eligible, physically independent measures.

## Candidate recovery

The first r1045 candidate was rejected at the Proxy-Identity Selector gate because its contract declared a selector change while the Rust delta only removed old scorewide selection mechanics and did not expose the replacement conflict-group decision as a new selector-sensitive capability. r1045 r02 corrected that by making physical-measure conflict grouping explicit, but was rejected by `NO-NEW-REPAIR-AUTHORITY` solely because two newly added unit-test functions called the productive writer-reaching owner directly and therefore appeared as new noncanonical writer callers. Neither rejected candidate is a parent.

r1045 r03 keeps the r02 productive implementation but changes regression topology only: candidate duplication and byte-stability checks are folded into the already-existing writer-reaching test `inserted_initial_rest_uses_metric_authority_on_canonical_basis`; the only newly registered test is the pure same-measure conflict-group selector test, which invokes no writer.

## Productive change

`apply_upper_staff_initial_rest_and_beat_local_coalescing` keeps the accepted local eligibility definition for the initial-rest branch:

- positive first visible upper-staff onset no greater than one canonical quarter;
- no visible upper rest already covering the initial silence;
- current metric-rest authority proves the entire silence as exactly one visible rest;
- existing local lower/cross-staff, same-voice staff-switch or octave-context evidence;
- the same upper voice and the same physical-measure insertion location.

After those facts are established, r1045 no longer assigns an initial-rest score and no longer performs scorewide ranking. In particular, `first_onset==24`, lower/cross-staff score bonuses, staff-switch score bonuses, octave-context score bonuses and source-offset tie-break position no longer affect which already-eligible measure receives a write.

The `max_initial_rests` parameter and the productive caller's `max_initial_rests=1` cap are removed. The replacement authority is now explicit: candidates are grouped by physical-measure scope, exactly one already-eligible candidate in a measure-local group may enter the existing replacement/overlap guard, and a same-measure group with multiplicity other than one fails closed. Different physical-measure groups remain independent.

The separate beat-local rest-coalescing branch remains out of scope: its candidate scoring, `max_coalesces`, conflict handling and writer boundary are not redefined by this take.

## Multiplicity proof

Regression coverage pins the multiplicity boundary without creating a new writer caller outside `StructuralWriteAuthority`:

1. one new pure selector regression feeds two synthetic candidates into the same physical-measure conflict group and requires fail-closed;
2. the pre-existing writer-reaching regression `inserted_initial_rest_uses_metric_authority_on_canonical_basis` is extended in-place with a second independent eligible measure carrying the retired `first_onset==24` preference; both measures must materialize;
3. the same pre-existing regression also proves that adding the second measure leaves the first measure's repaired bytes identical to the singleton decision.

No new `#[test]` function calls the productive writer path. This closes MI-A04 without introducing global ranking, a new winner selector, a new threshold, a replacement writer or a new test-level writer-reachability root.

## Expected validation

ChangeClass: `MusicMutation`.

The accepted r1044 strict baseline is 1,351 tests. r1045 adds one new pure selector regression and extends one pre-existing writer-reaching regression in-place, so 1,352 strict tests are expected if no unrelated test topology changes occur. A normal `--take --all` corpus run is required.
