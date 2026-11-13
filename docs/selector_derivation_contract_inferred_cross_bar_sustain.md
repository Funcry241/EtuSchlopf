# Taktübergreifende Dauern bei der Stimmenbildung

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
ParentErrorClassContract: docs/selector_derivation_contract_cross_bar_source_sustain.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: bound metric time continues beyond the first barline and both adjacent fragments have exact notation values
CausalFeature: an intervening staff attack before the barline would truncate the held duration during seed creation
CausalFeature: crossing notes at the same staff onset have distinct pitches and one common release on an available independent cursor
CausalFeature: a conflicting same-key overlap invalidates an unambiguous tied continuation
NonCausalContextExcluded: work identity, absolute measure, system, pitch, numeric voice and scorewide candidate count

Der MIDI-Seed legt beide Fragmente vor der weiteren Stimmenzuordnung an. Das
zweite Fragment führt die Identität des ersten Anschlags weiter. Töne mit
gemeinsamem Einsatz und Ende teilen sich einen Cursor als Akkord. Die übrigen
Anschläge laufen über den Hauptcursor. Der Test verdoppelt die Konstellation
und prüft zusätzlich Wiederanschlag am Release und überlappenden Wiederanschlag.
