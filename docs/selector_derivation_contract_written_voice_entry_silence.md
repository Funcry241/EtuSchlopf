# Metrische Pausen vor einem geschriebenen Stimmeinsatz

SelectorDerivationContract: causal_class_first_v1
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionMayRefineClassToKnownWitness: no
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
SelectorChange: no
SelectorRole: error_class_definition
CorpusDistributionPurpose: not_yet_measured
ParentErrorClassContract: none
ErrorClassDefinitionIndependentOfKnownWitness: yes
ErrorClassName: uncovered_metric_silence_before_bound_voice_entry
CausalFeature: a physically located measure contains a uniquely resolvable first written voice entry on a staff after the start of the measure
CausalFeature: the affected part's complete observed timeline proves this voice is silent over the entry prefix, including its sounding material on any staff
CausalFeature: the visible rests of that voice already represent the union of their covered intervals and only the uncovered metric silence needs notation
CausalFeature: the metric rest owner can represent the uncovered intervals exactly and the existing structural writer can preserve the physical cursor, pitched timeline and existing rest carriers
NonCausalContextExcluded: project, printed measure label, staff number, numeric voice label, pitch, clef, register, octave direction, other-voice accompaniment, specific duration, first-candidate preference and corpus rarity

## Regel und Geltungsbereich

Eine sichtbare Anfangspause gehört zur eindeutig einsetzenden geschriebenen
Stimme, wenn diese Stimme im vorausgehenden Taktintervall schweigt und dort
metrisch darstellbare Pausenzeit fehlt. EtuSchlopf prüft den Stimmverlauf über
alle Systeme des Parts und zieht die Zeitintervalle vorhandener sichtbarer
Pausen derselben Stimme ab. Der Pausenplaner legt die notierbaren Dauern fest.
Ein durchgehender Stimmenverlauf über zwei Systeme erzeugt keine zusätzliche
Anfangspause. Eine andere Stimme darf im fraglichen Intervall spielen.

Der bestehende strukturelle Owner setzt ausschließlich exakt darstellbare
Intervalle am gebundenen Einsatz um. Bei einer unklaren Einsetzstimme oder
fehlender Zeitbindung bleibt die Darstellung erhalten. Unabhängige Einsätze
in verschiedenen Systemen oder Takten können unabhängig entschieden werden.
Ein bereits aktives System mit später eintretenden Nebenstimmen benötigt eine
eigene Entscheidung über deren Rollen und gehört nicht zu diesem Vertrag.

Der erste Take definiert die Klasse und ihre Gegenproben vor der Umsetzung.
Die Gegenproben verwenden synthetische Einsätze mit vertauschten Systemen,
umbenannter Stimme, übergreifendem Klang derselben Stimme, einer unabhängigen
Begleitstimme und bereits teilweise oder vollständig abgedeckter Pausenzeit.
