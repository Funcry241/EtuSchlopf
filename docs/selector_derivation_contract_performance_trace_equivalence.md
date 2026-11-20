# Erhaltung der metrischen Entscheidung bei freier Einspielung

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
ParentErrorClassContract: docs/selector_derivation_contract_ambiguous_performance_meter_class.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: Competing meters must explain the same aligned attacks, with local tempo, subdivision, phase and repeated transitions evaluated on the existing common scale.
CausalFeature: A proposed shared written release must stop before a subsequent onset of the same pitch on the same track and channel.
NonCausalContextExcluded: Work, performer, corpus membership, absolute pitch and measure, candidate count, runtime measurements and reference notation.

Der Umsetzungsbereich ist der bereits zugelassene Vergleich konkurrierender
Lesarten derselben Einspielung. r1965 ändert den Zugriff auf seine Kosten und
Einsatzdaten. Zulassung, Gewichte und Schreibumfang folgen r1964.

- Die Phrasensuche liest pro Zelle links, oben und diagonal oben links. Zwei
  wechselnde Zeilen enthalten diese drei bisherigen Werte. Vor der ersten
  ausgewerteten Spalte jeder Zeile steht wieder die Null des unbesuchten
  Randes. Jede Rückrichtung (0 bis 3) bleibt bis zur Rückverfolgung erhalten.
  Reihenfolge, strikter Kostenvergleich, Sortierung der Endpunkte und Markierung
  belegter Einsätze bleiben identisch, auch bei Gleichstand.
- Für jeden Übergang vom früheren zum neuen Tempo berechnet der Owner weiterhin
  denselben Quotienten und dieselbe Strafe. Nur die Speicherachsen wechseln.
  Die Vorgängertempi laufen weiterhin aufsteigend. Pro Phase laufen dieselben
  Sprünge in derselben Reihenfolge. Bei gesperrtem Nullsprung entfällt genau
  das erste Element. Vorgängerphase, Phasenfaktor, Sprunggewicht und Strafe
  hängen allein vom aktuellen Metrummodell ab und dürfen vor der Einsatzschleife
  berechnet werden. Subtraktionen und Additionen behalten ihre Reihenfolge.
  Striktes Minimum und Rückzeiger behalten den ersten bisherigen Gewinner.
  Das stabile Sortieren der vier Meterkandidaten erhält bei gleichen Kosten
  ihre ursprüngliche Reihenfolge. Der Phasenanker wird je Einsatz einmal gelesen.
- Bei einem möglichen gemeinsamen Notenende war ein Wiederanschlag genau dann
  ein Veto, wenn Spur, Kanal und Tonhöhe übereinstimmten und der Einsatz strikt
  zwischen Anfang und Ende lag. Die bestehende Zeitleiste enthält genau die
  Einsatzgruppen dieser Spur und dieses Kanals. Das offene Intervall befragt
  alle Töne dieser Gruppen. Ein Einsatz auf einer Intervallgrenze, in einer
  unabhängigen Spur oder auf einem anderen Kanal bleibt außerhalb des Vetos.

Die isolierte Gegenprobe verglich die Wiederholungszuordnung beider Fassungen
an 80 synthetischen Eingaben mit bis zu 1024 Gruppen. Für die metrischen
Vergleiche wurden alle vier Taktarten bei freiem und vorgegebenem Puls geprüft,
jeweils einschließlich der Scorebits, Anfangsphase und vollständigen Folge der
notierten Einsätze. Die bestehende lokale Gegenprobe zum Akkordende deckt
Wiederanschlag und unabhängigen Kanal ab. Die Quellereignisse behalten ihre
Identität. Eine weitere unabhängige Kopie liefert dem lokalen Veto keine Evidenz.
