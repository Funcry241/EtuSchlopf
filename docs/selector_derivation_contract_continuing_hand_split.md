# Fortlaufende Handaufteilung beim MIDI-Entwurf

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
ParentErrorClassContract: docs/selector_derivation_contract_hand_jump_reassignment_oscillation.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: an initial wide split binds two adjacent hand lines and both notes release at the following paired attack
CausalFeature: the new attack admits a stepwise continuation of the upper line and the lower continuation remains within the existing span limit
CausalFeature: the receiving hand has room for the transferred attack over its entire interval
CausalFeature: a single advancing note with a representable release on a beat leaves the following gap to the rhythmic cursor instead of sustaining into the gap
NonCausalContextExcluded: work identity, measure labels, absolute pitch values, source ordinal values and number of independent groups

Der MIDI-Seed entscheidet Aufteilung und metrische Abdeckung vor der
MusicXML-Ausgabe. Vorrang haben bereits erforderliche Spannweiten- und
Belegungsentscheidungen. Eine fortgesetzte Aufteilung bindet die
vorausgehenden Anschläge als Beleg. Ein einzelner Anschlag am Hauptcursor
behält sein darstellbares Ende auf der Zählzeitgrenze. Der Cursor deckt
den restlichen Abstand zum Folgeanschlag mit einer Pause ab.
