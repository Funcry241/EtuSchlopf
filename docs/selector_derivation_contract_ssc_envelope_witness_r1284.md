# Selector derivation contract: SSC containment v4 Take 4b (envelope extensions and minimal witnesses, r1284)

Stage B of causal_class_first_v1. This take implements no new class. It widens seven envelope rows inside their frozen owner purposes (A5 analysis `docs/ssc_denied_purpose_analysis_r1283.md`) and makes the constructive ContainmentDenied witness minimal. The classes decide no candidate, repair, write, ranking or release; they classify already adopted owner events, observation only (PermitEnforcement: none).

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
CausalFeature: invariant violation = the envelope's named invariant (conserved projection, single insertion geometry, edit-script locality, bound-span locality, lane fields) does not hold on the exact pre and post bytes
CausalFeature: containment unproven = the descriptor has no envelope row, or the captured bytes cannot be read, so containment cannot be decided constructively
NonCausalContextExcluded: project, etude, absolute measure number, part id, pitch values, owner run order, candidate profile, corpus frequency, WAV-derived values and whether the change was musically necessary
```

Each widened row follows from the sealed writer code named in the A5 analysis, never from the corpus count. The witness names the first differing span of its basis (document, canonical form, whole-element sequence or owner projection); it does not change any verdict.
