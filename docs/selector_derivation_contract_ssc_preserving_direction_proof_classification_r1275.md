# Selector derivation contract: SSC PRESERVING/E036 proof classification (implementation)

Stage B of causal_class_first_v1 for r1275. It implements exactly the classes frozen in the accepted r1274 parent contract, observation-only.

```text
SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: error_classification
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
ParentErrorClassContract: docs/selector_derivation_contract_ssc_preserving_direction_proof_classes.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: beam_chain_repair_note implements the frozen malformed beam sequence class and its full-attribution repair rule on exact captured bytes
CausalFeature: preserving_assessment recomputes the owner-bound conserved projection; a difference is a constructive PreservedProjectionChanged witness
CausalFeature: direction_insertion_geometry implements the frozen single-insertion class for E036
NonCausalContextExcluded: project, etude, absolute measure number, part id, pitch values, owner run order, candidate profile, corpus frequency and WAV-derived tempo values
```

Each released owner event is classified independently from its own sealed origin; duplicated or foreign records are integrity failures, never extra credit. No productive write, candidate choice or veto reads these classes. Corpus numbers are measured after acceptance and may not narrow the classes.
