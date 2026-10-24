# Selector Derivation Contract - Upper-Staff Initial Visible Silence Repair Scope r1045

SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: repair_scope
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: measurement_only
CorpusDistributionMayRefineClassToKnownWitness: no
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
ParentErrorClassContract: docs/selector_derivation_contract_upper_staff_initial_visible_silence.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: each already-eligible upper_staff_initial_visible_silence_after_local_cross_staff_context candidate owns exactly one physical measure-local insertion domain immediately before that measure's first selected upper-staff event
CausalFeature: current discovery yields at most one initial-rest candidate per physical measure because it derives one upper_first event per measure
CausalFeature: implementation authority is explicitly grouped by physical-measure scope; exactly one already-eligible candidate in a measure-local conflict group may proceed, while a group with multiplicity other than one fails closed
CausalFeature: candidates in different physical measures have disjoint insertion domains and therefore are independent transactions rather than alternatives in one scorewide competition
CausalFeature: every adopted candidate retains the accepted r1044 local eligibility predicates, metric single-rest proof, existing overlap protection and the existing structural-write-authority boundary
NonCausalContextExcluded: first_onset equals 24, numeric score value, scorewide ranking position, source byte offset, discovery order, global candidate count, or a scorewide maximum action count
NonCausalContextExcluded: project identity, etude identity, concrete measure number, page or system location, corpus rarity, or known witness location

## Candidate-recovery boundary

r1045 r02 already expressed the intended physical-measure selector, but two newly registered regression-test functions called the writer-reaching productive owner and were correctly rejected as `new_writer_caller_outside_structural_write_authority`. r1045 r03 leaves the productive selector bytes semantically unchanged from r02 and moves those checks into one pre-existing writer-reaching test. The new same-measure ambiguity test remains pure and calls only `select_measure_local_upper_initial_rest_candidates`.

## Repair-scope derivation

The accepted r1044 parent already froze `upper_staff_initial_visible_silence_after_local_cross_staff_context` as a measure-local visibility/serialization obligation. r1045 changes only the multiplicity authority of the initial-rest branch. Local candidate discovery and all causal eligibility predicates remain unchanged.

The previous implementation collected already-eligible measure-local candidates into one scorewide vector, assigned non-causal scores, ranked the whole score, tie-broke by source position and then allowed only `max_initial_rests=1` at the productive caller. Those operations made the existence and score of a second independent measure capable of changing the first measure's write decision.

r1045 removes that scorewide competition and replaces it with an explicit physical-measure conflict-group boundary. Already-eligible initial-rest candidates are grouped by measure scope; a group with exactly one candidate proceeds, a same-measure ambiguous group fails closed, and different measure groups are evaluated independently. The separate beat-local coalescing branch retains its existing scoring, overlap checks and `max_coalesces` limit byte-semantically unchanged apart from the shared function signature adjustment required to remove the retired initial-rest cap parameter.

## Candidate-duplication proof

The already-existing writer-reaching regression `inserted_initial_rest_uses_metric_authority_on_canonical_basis` is extended in-place: it first materializes one eligible measure whose first upper onset is 48 and then repeats the decision after adding an independent second measure whose onset is 24. The latter deliberately carries the old `first_onset==24` preference. Both measures must materialize, and the first repaired measure must remain byte-identical to its singleton repair. A separate new pure selector regression feeds two synthetic candidates into the same physical-measure conflict group and requires the group to fail closed. No new test function calls the productive writer path. Therefore the old onset bonus, global ranking, source-offset tie-break and `max_initial_rests=1` can no longer own the first candidate's decision, while same-measure multiplicity remains explicitly guarded.
