# Clef boundaries in the existing beat-scoped beam planner

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
ParentErrorClassContract: docs/selector_derivation_contract_metric_beam_clef_boundary.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: the existing beat-scoped planner must not propose a connection between successive candidate attacks on opposite sides of a visible clef boundary on their own staff
CausalFeature: the same boundary constrains contiguous runs and the existing visible-rest fallback before either can emit a positive decision
NonCausalContextExcluded: project identity, measure labels, page, pitch, absolute location, voice label value and corpus distribution

The owner reads clef indications from its current measure, with backup and
forward applied to the same time cursor used for attacks. Equivalent local
repetitions do not open a boundary. Without earlier local clef evidence, a
visible mid-measure indication is treated conservatively as a boundary for new
connections. A parse failure leaves the input measure unchanged. Decisions
remain independent across existing staff, voice and beat scopes. Negative
emissions retain their existing authority and decision scope.

The existing owner and receipt path remain responsible for materialization.
No clef, pitch, onset, duration, staff, voice or rest is changed by the beam
planner. Native acceptance must compare the real corpus and inspect changed
rendered pages. The contract in r1603 remains byte-identical.

## Continuation after a changed register course

CausalFeature: a subsequent authorized register transaction can change the clef
cuts that the original E053 decision used. The same E053 planner compares the
two cut sets on a shared complete note course. Its difference is closed over
all members and levels of the affected decisions. It includes explicit empty
decisions when a formerly owned beam is split into isolated attacks.
CausalFeature: a revised group is admissible only on the actual prepared
register course and cannot cross a retained clef or octave boundary.
CausalFeature: E053 carries the unaffected original occurrences by its
complete-note emitter map. Changed occurrences must agree with the original
owned beam levels on input. Partial replacement of an existing batch and
changes to foreign beam levels refuse the continuation.
CausalFeature: Master and Render each bind to their own actual original E053
capture through the register and clef journals. Both continuations are prepared
before either revision is adopted by the existing joint transaction.

The two decisions are compared only where the original and prepared clef
maps differ. The same owner still determines beam hierarchy and all underlying
musical conditions. Independent groups in another time bucket or staff retain
their original decisions. Clef and octave authority remains with their original
owners. The continuation emits beam changes only.
