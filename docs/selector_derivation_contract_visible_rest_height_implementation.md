# Pausenhöhe im regulären PDF-Export

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
ParentErrorClassContract: docs/selector_derivation_contract_visible_rest_height.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: Je zwei zeitlich anschließende sichtbare Einzelnoten der geschriebenen Stimme geben die Referenzhöhe vor, wenn ihre gezeichneten Köpfe dieselbe Höhe besitzen.
CausalFeature: Ein vollständiger gemeinsamer Render bestätigt eine Annäherung bei mindestens 0,50 Notenlinienabständen konservativer Freistellung und stabiler übriger Geometrie.
CausalFeature: Der bestehende Pausenhöhen-Owner übernimmt ausschließlich belegte Änderungen der grafischen Positionsangabe der identifizierten Pause.
NonCausalContextExcluded: Werk, Dateiname, Takt- oder Systemnummer, absolute Tonhöhe, Stimmenbezeichnung, Anzahl und Auftretensreihenfolge im Korpus.

Die Umsetzung prüft kurze Pausen mit expliziter grafischer Positionsangabe im
unteren System einer zweisystemigen Klaviernotation mit Bassschlüssel.
Die Farbmarken dienen der eindeutigen Zuordnung und erscheinen ausschließlich
in den Vorschauen. Jede Markierung muss die gezeichnete Geometrie erhalten.

Ein diatonischer Schritt entspricht einem halben Linienabstand und liefert
eine erste Versuchslage. Der Export prüft diese Lage und bis zu zwei Schritte
darüber und darunter. Die Rangfolge beruht auf der gerenderten Entfernung zur
Referenzhöhe und bei Gleichstand auf dem kürzeren Verschiebeweg. Konservative
umhüllende Rechtecke prüfen die Abstände, einschließlich der Abstände zu anderen
Pausen. Notenlinien sind Führungsgeometrie.

Der gemeinsame Render prüft alle geplanten Änderungen und die vollständige
übrige Vektorgeometrie. Fällt eine Bewegung bei der Abstandsprüfung durch, prüft
der Export die verbleibenden Änderungen gemeinsam erneut. Unsichere Zuordnung
oder fehlender Beleg für die unveränderte Umgebung erhält die Ausgangslage.
