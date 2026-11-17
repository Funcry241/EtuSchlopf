# Wiederholungsakkorde in der Handzuordnung erhalten

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
ParentErrorClassContract: docs/selector_derivation_contract_split_repeated_accompaniment.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: an inferred staff boundary divides simultaneous distinct attacks with the same bound release
CausalFeature: an intact pitch cohort immediately adjacent by bound attack and release supports one hand, independent of its duration
CausalFeature: independently fixed adjacent assignments agree before any change in this pass supplies new evidence
CausalFeature: held and entering notes fit the existing hand span and key count limits in both hands without crossing or a competing attack on the same key
NonCausalContextExcluded: work, recording, file name, measure label, fixed pitch, numeric voice label, candidate count and renderer output

Die Zuordnung läuft im MIDI-Seed nach der allgemeinen Handverteilung und vor
der Bildung der rhythmischen Stimmen. Ein gemeinsam freigegebener Akkord
braucht eine vollständige Wiederholung. Als Zeugen gelten die bereits
zugeordneten Gruppen, deren Ende unmittelbar an den aktuellen Einsatz
anschließt oder deren Einsatz unmittelbar auf dessen Ende folgt. Die
Tonhöhen müssen die vollständige Gruppe der bezeugenden Hand bilden.
Separate Anschläge der anderen Hand dürfen dieselbe Einsatz- und
Freigabezeit haben. Bei widersprüchlichen Zeugen bleibt die bestehende
Zuordnung maßgeblich.

Die Übernahme prüft beide Hände am Einsatz und bei allen weiteren Anschlägen
bis zur Freigabe des Akkords. Die Quellanschläge behalten Identität, Einsatz,
Dauer und Tonhöhe. Die Spur der MIDI-Entscheidung nennt die gebundene
Wiederholung. Deklarierte Quellstimmen folgen ihrer eigenen Zuordnung.
