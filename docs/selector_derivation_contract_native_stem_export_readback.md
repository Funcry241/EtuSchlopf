# Beleg zur nativen PDF-Exportquelle

SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: error_classification
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
ParentErrorClassContract: docs/selector_derivation_contract_visible_engraving_collision.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: stable native score and style inputs bind both exports before staff and stem strokes are compared in physical page coordinates
CausalFeature: a stroke needs geometric correspondence in both forms with a unique PDF owner; ambiguous or drifting evidence cannot certify the export
NonCausalContextExcluded: work identity, measure identity, pitch pattern, findings in other works and aggregate counts of unrelated strokes

r2008 erprobt die Bindung zwischen einer nativen Partitur, der SVG-Vorschau und einem getrennten PDF-Export. Das Produktions-PDF behält seine bestehende Quelle und seine Freigabeprüfung. Im normalen Kollisionsaudit durchläuft eine Kopie des unveränderten nativen Imports den Beleg. Eine synthetische Partitur prüft zusätzlich einen veränderten sichtbaren Notenhals.

Die geprüfte Vorschau und das Prüf-PDF verwenden dieselbe MSCX-Datei mit derselben vorhandenen Stil-Datei und dem bereits gebundenen Konverterprofil. Der Beleg vergleicht die Eingaben vor und nach beiden Konvertierungen bytegenau. Er bindet Prüfsummen der Partitur, des vorhandenen Stils, der Vorschauseiten und des erzeugten Prüf-PDFs. Der PDF-Zielpfad muss vor der Konvertierung frei sein.

Der Vergleich liest Notenlinien und Notenhälse als gezeichnete Striche. Er überführt die SVG-Koordinaten anhand der physischen Seitengröße in PDF-Punkte. Jeder Hals muss mit genau einem PDF-Strich übereinstimmen. Eine Notenlinie kann im PDF aus mehreren Teilstrichen bestehen. Deren äußere Endpunkte, Höhe und Strichbreite müssen zur SVG-Linie passen, und benachbarte Teilstriche müssen aneinander anschließen. Die Prüfreserve je Koordinate und Strichbreite beträgt 0,015 PDF-Punkte. Ein wiederverwendeter, mehrdeutiger oder abweichender Strich lässt den Beleg offen. Die Prüfreserve gilt für Exportkoordinaten. Die Mindestabstände der Kollisionsprüfung gelten separat.

Der Versuch erlaubt in einer Variante desselben nativen Imports endliche Änderungen der freien Halslänge. Der Strukturvergleich bindet alle übrigen Elemente, Werte, Attribute und deren Reihenfolge. Er erteilt keine Aussage über die Äquivalenz zweier eigenständiger MusicXML-Importe. Pausen, Fähnchen, Balken, Köpfe und Bögen gehören nicht zur formatübergreifenden Strichprobe.

Die synthetische Probe enthält Stimmen, Notenzeilen, eine Pause und einen Haltebogen. Sie prüft einen verkürzten oder verlängerten Notenhals im tatsächlich erzeugten Prüf-PDF und verwirft anschließend die Vorschau beim Vergleich mit dem PDF des anderen Stands. Synthetische Gegenproben weisen semantische Änderungen und mehrdeutige Striche zurück.

Das Ergebnis ist reine Diagnose. Fehlende oder unvollständige Evidenz erscheint im Ergebnisbericht. Die bestehende Kollisionsdiagnose läuft auch bei offenem Exportbeleg. Der Beleg wählt keine Korrektur aus.
