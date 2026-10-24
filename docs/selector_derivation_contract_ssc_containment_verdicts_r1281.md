# Selector derivation contract: SSC containment verdicts v4 (classification, r1281)

Stage B of causal_class_first_v1. This take implements the containment classes frozen in r1277 (`docs/selector_derivation_contract_ssc_containment_error_class.md`) for the PROJECTION, INSERTION and LANE envelopes. The classes decide no candidate, repair, write, ranking or release; they classify already adopted owner events against the frozen envelope row of their own descriptor, observation only (PermitEnforcement: none).

```text
SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: error_classification
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: measurement_only
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: docs/selector_derivation_contract_ssc_containment_error_class.md
ErrorClassDefinitionIndependentOfKnownWitness: yes
CausalFeature: containment violation = a difference between the exact captured pre and post bytes lies outside the declared element kinds, fields or operations of the descriptor's frozen envelope row, or outside its bound span
CausalFeature: invariant violation = the envelope's named invariant (conserved projection, single insertion geometry, edit-script locality, lane fields) does not hold on the exact pre and post bytes
CausalFeature: containment unproven = the descriptor has no envelope row, or the captured bytes cannot be read, so containment cannot be decided constructively
NonCausalContextExcluded: project, etude, absolute measure number, part id, pitch values, owner run order, candidate profile, corpus frequency, WAV-derived values and whether the change was musically necessary
```

A difference is contained only if every changed element, attribute or child lies in the descriptor's envelope row, and the kind's bound invariant holds (owner conserved projection, single contiguous insertion, lane fields only). A missing row or unreadable bytes give ContainmentUnproven. Necessity is still measured by the V2 classes and never decides the V4 verdict.
