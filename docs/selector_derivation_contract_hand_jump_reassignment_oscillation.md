# Gegensätzliche Handwechsel in der Sprungglättung

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
ParentErrorClassContract: none
ErrorClassDefinitionIndependentOfKnownWitness: yes
ErrorClassName: hand_jump_reassignment_oscillation
CausalFeature: the same retained attack is reassigned between hands by successive decisions of one owner on one source and profile
CausalFeature: one hand removes a local transition demand by transferring the attack and the receiving hand returns it under an opposing transition demand
CausalFeature: the combined assignment and the affected transition context recur so the repeated decisions make no net progress
NonCausalContextExcluded: work identity, printed bar number, absolute pitch, source ordinal value, chosen profile name, corpus frequency, iteration cap and final engraving appearance

## Ursache am MIDI-Seed-Owner

Die Sprungglättung betrachtet die Noten einer Hand in zeitlicher Reihenfolge.
Die Zielwahl begründet den Wechsel mit dem lokalen Sprung der abgebenden Hand.
Dadurch kann der Wechsel in der aufnehmenden Hand einen Gegengrund erzeugen.
Wenn dieselbe Zuordnung und derselbe betroffene Kontext wiederkehren, belegt
die Schleife eine fehlende gemeinsame Entscheidung über beide Hände.

Ein Wechsel mit einer späteren Rückkehr kann sinnvoll sein, wenn sich der
betroffene Kontext durch andere Entscheidungen geändert hat. Die Fehlerklasse
setzt die Wiederkehr desselben Kontexts voraus. Gleichzeitige Anschläge liefern
Spannweiten- und Belegungsinformationen. Ein Zeitintervall zwischen zwei
verschiedenen Anschlägen liefert die Grundlage für eine Sprungbeurteilung.

## Gegenproben für die Umsetzung

- Drei aufeinanderfolgende Anschläge bilden zwei gleich starke gegenläufige
  Übergänge um den mittleren Anschlag. Der Owner vergleicht beide Zuordnungen
  zusammen und behält bei Gleichstand die vorhandene Hand.
- Eine Übergabe entlastet die abgebende Hand und bleibt in der aufnehmenden Hand
  zulässig. Der Owner übernimmt sie genau einmal. Er berücksichtigt auch die
  neue Nachbarschaft, die durch das Entfernen des Anschlags entsteht.
- Wenn die Übergabe in der anderen Hand eine stärkere Belastung erzeugt oder
  eine vorhandene Belegungs- oder Spannweitengrenze verletzt, bleibt die
  Zuordnung bestehen. Gleichzeitige Anschläge gelten dabei als Belegung.
- Eine zeitlich getrennte Kopie desselben Musters ergibt dieselbe lokale
  Entscheidung. Jede übernommene Entscheidung bindet ihre Quellanschläge
  und erhält Tonhöhe, Einsatz und Dauer.

Die Umsetzung muss einen gemeinsamen Vergleich für beide Hände formulieren,
der den betroffenen Kontext vor und nach einer Übergabe abdeckt. Der
Schleifenabbruch allein belegt keine Verbesserung. Die Bewertung einer
Übergabe und die Prüfung des erzeugten Notensatzes bleiben eigene Nachweise.
