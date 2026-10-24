# Structure -> System Break — retired experiment history

**Project:** EtuSchlopf  
**Version:** `0.10.1839`  
**Retirement revision:** `r179`  
**Capability status:** `RETIRED`  
**Productive authority:** `NONE`

## Purpose of this record

This document is the compact historical record for the completed musical-structure -> system-break research branch. It is documentation only. No structure-derived layout preference, score, threshold, voice vote, search window, alternate layout planner, writer authority or dormant feature switch is authorized by this record.

The existing phrase/structure analysis remains an independent capability. Its rhythm, meter, melody, continuation, parallelism, novelty, tonal, local-group and closed-span evidence is not retired. Existing phrase-notation consumers are likewise outside this retirement unless independently changed elsewhere.

## Research sequence

- `r176 StructureLayoutCapabilityGapAudit` established that broad lane-local musical structure evidence already existed and that the open question was an integration/projection question rather than a missing phrase detector.
- `r177 StructureToLayoutCounterfactualAudit` implemented a bounded, read-only final-plan information experiment. Formal page-local `2+3 <-> 3+2` break alternatives were screened by already-existing capacity and productive run-local renderer no-goods before any musical structure evidence was evaluated.
- `r178 StructureToLayoutEvaluationStateClosure` separated `NotEvaluatedConstraintRejected` from genuinely evaluated `InsufficientEvidence`, closing the only report-semantic ambiguity found in the r177 acceptance run.

No revision in this branch granted productive layout authority to musical structure.

## Predeclared interpretation contract

The experiment was not allowed to ask which layout was prettier, more readable or musically correct. It asked whether existing lane-preserving structure evidence provided stable, explainable, non-redundant directional information between otherwise technically admissible final-plan local break alternatives.

The route had to stop if the technically admissible cases produced only unordered/polyphonic differences without a defensible direction, or if direction required post-hoc weights, majority voting, dominant-voice rules, proximity windows, thresholds, a second planner or writer workarounds.

## Accepted full-corpus result

The accepted r178 ten-etude corpus run was technically dark green and produced:

- final-plan formal shape opportunities: **30**
- capacity-eligible alternatives: **15**
- constraint-eligible alternatives after productive run-local renderer no-goods: **12**
- technically rejected / not structurally evaluated: **18**

Among the **12** structure-evaluated cases:

- `DifferentUnordered`: **11**
- `InsufficientEvidence`: **1**
- `DirectionalBaseline`: **0**
- `DirectionalAlternative`: **0**
- `PolyphonicOpposition`: **0**
- `Equivalent`: **0**

All twelve constraint-eligible cases were geometrically `BaselineFavored` by the existing technical layout relation. Structure therefore differentiated many candidate boundaries qualitatively, but supplied no permitted directional preference to add to the existing system-break decision.

The confound sensitivity check did not rescue the hypothesis: after excluding phrase-notation-break-dependent cases, the remaining unconfounded cases still produced no directional structure relation.

## Scientific conclusion

Three statements must remain separate:

1. Existing musical structure evidence is real and broad. **Confirmed.**
2. That evidence often distinguishes two possible break locations qualitatively. **Confirmed.**
3. Under the predeclared lane-preserving, non-scalar rules, it provides a directional preference for system-break selection. **Not confirmed.**

The negative third result triggers the predeclared stop condition.

## Retirement decision

**Structure -> SystemBreak preference research route: RETIRED.**

Do not continue this branch through:

- musical weights or scalar boundary scores;
- voice majority voting or a preferred/dominant voice heuristic;
- widened nearest-boundary windows;
- threshold calibration;
- etude- or measure-specific rules;
- a second layout planner;
- writer/slur workarounds;
- post-hoc candidate-domain expansion whose purpose is to manufacture direction.

Reopening requires genuinely new external evidence that changes the scientific premise, not a more elaborate version of the same inference machinery.

## r179 source retirement

r179 removes the active r177/r178 experiment instrumentation:

- the final-plan counterfactual audit module;
- its page-count projection hook and renderer-oracle capture;
- the complete exact-measure-boundary lane projection retained only for that experiment;
- the final renderer-selected plan helper exposed only to that experiment.

The r176 capability inventory remains temporarily as the branch-status record for the retirement verification run, but now reports the completed negative result instead of an open/pending experiment. A later structural-closure revision may remove that residual audit inventory after a green corpus closure, while preserving this documentation-only history.
