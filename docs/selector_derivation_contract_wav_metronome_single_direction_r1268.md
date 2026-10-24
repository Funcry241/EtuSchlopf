# WAV metronome: one coherent initial tempo direction

SelectorDerivationContract: causal_class_first_v1
SelectorChange: no
SelectorRole: error_class_definition
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: not_yet_measured
CorpusDistributionMayRefineClassToKnownWitness: no
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
ParentErrorClassContract: none
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
ErrorClassDefinitionIndependentOfKnownWitness: yes
CausalFeature: visible quarter-note tempo and playback tempo describe the same initial tempo event; separate insertion of their XML direction containers can split or duplicate that one event
CausalFeature: a tempo container already supplying both the same visible quarter-note value and playback value needs no additional WAV-derived direction; an incomplete representation cannot be certified as the complete single-direction construction
CausalFeature: a competing value or another tempo input is a distinct source, not evidence for the duration-derived tempo and not permission to rank or overwrite independent musical directions
NonCausalContextExcluded: project identity, file name, absolute measure number, pitch, octave, register, corpus frequency, report counters and historical output hashes

The causal class is representational coherence of the single duration-derived initial tempo event, not uniqueness of candidate notes or global musical event cardinality. The opening measure is found structurally, even when its number is not 1. Independent parts, measures and non-tempo directions cannot be selected by scorewide ranking or removed to make a count pass.

This preparation changes no selector or writer. Later implementation must reference this unchanged accepted-parent file. Corpus evaluation cannot refine the class to observed files. The positive product specification is docs/ssc_wav_metronome_positive_contract_r1268.md; neither document substitutes for a constructive installed-tree check.
