# Verbindliche Notenzeit im MIDI-Seed ausschreiben

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
ParentErrorClassContract: docs/selector_derivation_contract_bound_written_duration.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: A validated note-specific written time map supplies an exact written onset and release to the existing MIDI seed for each regular retained source event.
CausalFeature: The existing piano seed without independently declared voice assignments can serialize overlapping written obligations using separate technical cursor runs while retaining the assigned staff of each source event.
CausalFeature: The plan and written output must realize the complete interval of each source identity with matching pitch and a unique attack, including the ties of metric fragments.
NonCausalContextExcluded: Work and project identity, filenames, measure labels, pitch as an identity proxy, a fixed rhythmic figure, candidate count and renderer result.

Die Umsetzung setzt dort an, wo eine notenbezogene Zeitkarte verbindliche
Dauern trägt und die beiden vorhandenen generierten Klaviersysteme die
Darstellung übernehmen. Bereits anderweitig gebundene Quellstimmen
benutzen ihren bestehenden Autoritätspfad. Die Ausweitung auf weitere
Zeitquellen bedarf ihres jeweiligen Nachweises. Der MIDI-Release der
Einspielung ändert sich durch die Notationsentscheidung nicht.

Pro System bleiben die bisherigen Primärkurse bestehen. Die technische
Belegung reserviert einen Cursorlauf bis zum Ende des beanspruchten
Intervalls. Nacheinander liegende Intervalle können den Lauf wiederverwenden.
Gleichzeitige verschiedene Töne mit genau gleichem Intervall können als
Akkord erscheinen. Gleichnamige Töne bleiben getrennte Quellereignisse.
Ein Vorschlag bleibt beim gebundenen Hauptton. Es gibt keine neue
musikalische Stimm- oder Handentscheidung. Die technische XML-Kanalnummer
begründet keine originale Stimmidentität.

Teilungen an Taktgrenzen und bei nicht mit einem einzelnen Notenwert
darstellbaren Dauern bleiben Halteketten derselben Quellnote. Die
Teilungsgrenzen stammen aus dem vorhandenen metrischen Raster und den
schreibbaren Notenwerten. Jeder in einem Takt benutzte Cursorlauf deckt den Takt ab. Der Seed
schreibt Ruheintervalle für die Lücken des jeweiligen Laufs. Der vorhandene Seed-Writer schreibt den Plan. Vor der Emission
prüft der Owner die Intervallpartition pro Quellenidentität. Ein separater
Readback prüft den tatsächlich ausgeschriebenen Cursor, Ton, Notenwert
und die Haltekennzeichnung gegen den geplanten Satz.

Eine disjunkte Wiederholung erhält dieselben lokalen Zeitintervalle,
Pausen und Halteketten. Technische Kanalnummern dienen der Darstellung.
Aufführungsschätzung, Quell- und Staff-Auswahl und nachgelagerte
Korrekturregeln beurteilen weiterhin ihre eigenen Eingangsgrößen.
