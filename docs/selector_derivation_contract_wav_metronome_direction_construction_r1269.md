# WAV metronome coherent direction construction r1269

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
ParentErrorClassContract: docs/selector_derivation_contract_wav_metronome_single_direction_r1268.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
ErrorClassDefinitionIndependentOfKnownWitness: yes
CausalFeature: visible quarter-note tempo and playback tempo describe the same initial tempo event; separate insertion of their XML direction containers can split or duplicate that one event
CausalFeature: a tempo container already supplying both the same visible quarter-note value and playback value needs no additional WAV-derived direction; an incomplete representation cannot be certified as the complete single-direction construction
CausalFeature: a competing value or another tempo input is a distinct source, not evidence for the duration-derived tempo and not permission to rank or overwrite independent musical directions
NonCausalContextExcluded: project identity, file name, absolute measure number, pitch, octave, register, corpus frequency, report counters and historical output hashes


The frozen class requires a complete equivalent visible/playback direction to remain unchanged. The old literal extraction matched only an unadorned words tag, while the generated visible tempo has positioning and typography attributes. The repair reads the first words element's decoded text in the local direction through the existing roxmltree parser. It keeps the existing quarter-note marker, numeric equality tolerance, staff domain, hidden-state behavior, metronome/dotted-quarter identity and playback check. This is local recognition of the same tempo event independent of its presentation attributes, not another tempo source or candidate ranking.

The two independent additive insertions become one insertion of the missing representations. If neither is present, visible words and playback sound share one direction and E036 receipt. If both exist, nothing is inserted. If exactly one exists, its bytes remain unchanged and one additive carrier supplies the missing half. Legacy partial/split inputs are preserved and are not certified as positive single-container SSC closure evidence. Independent or conflicting tempo values are never overwritten to meet a global count.

Synthetic tests exercise renamed and renumbered structural openings, unrelated and independently duplicated musical material, repeat application, fixed arithmetic and the actual Master adoption boundary with one receipt. No formula, threshold, ranking, proof source, authority owner or enforcement is changed. NAOpenGroups stays 3; positive K2 closure remains a later named governance take.
