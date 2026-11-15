# Notenwert eines wechselnden Akkordtons

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
ParentErrorClassContract: docs/selector_derivation_contract_written_duration_from_performance.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: a chord member rounds to two subdivisions when its played release falls after the next attack and before the attack after that
CausalFeature: the member belongs to a repeated chord before the overlap and returns when the chord recurs
CausalFeature: two common pitched members articulate the chord across the intervening subdivision where the candidate is absent
NonCausalContextExcluded: work, recording, filename, measure address, absolute pitch, staff assignment and scorewide instance count

Der Owner der Notationszeit schreibt den kurzzeitig ausgesetzten
Akkordton bis zum nächsten Einsatz. Die wiederkehrenden Anschläge
stützen den Akkordwechsel. Die ursprünglichen MIDI-Zeiten bleiben
als Quellinformation gebunden.
