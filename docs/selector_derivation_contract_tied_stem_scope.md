# Halsrichtungen in belegten Haltebogenketten

SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: repair_scope
ParentErrorClassContract: docs/selector_derivation_contract_final_stem_error_class.md
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: measurement_only
CorpusDistributionMayRefineClassToKnownWitness: no
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: the established isolated rhythmic-unit direction disagrees with at least one explicit stem in a complete connected set of authenticated written tie occurrences
CausalFeature: each direction-bearing rhythmic unit of that set independently requires the same unambiguous direction under its printed clef and octave context
NonCausalContextExcluded: work, filename, measure label, system index, page index, corpus frequency, occurrence ID values, proxy note patterns, and discovery order

## Vorschlag

Der finale Notationsstand liefert den gedruckten Kontext. Gebundene
Quell- und Fragmentidentitäten verknüpfen die geschriebenen Haltebogenenden.
Der Planer fasst rhythmische Einheiten mit gemeinsamen Satzpflichten zusammen.
Eine verbundene Gruppe erhält genau dann einen Vorschlag, wenn jede Einheit
dieselbe eindeutige Halsrichtung verlangt und ihr mindestens ein expliziter
Hals widerspricht. Der Vorschlag beschreibt ausschließlich die nötigen
Halsänderungen. Takt-, System- und Seitengrenzen lösen keine Änderung aus.

## Inventar und Ausgabeprüfung

Ein sichtbares Segment erhält eine eigene Identität innerhalb seines
gebundenen Bogens. Die Seite gehört zur Geometriebeobachtung. Die Paarmenge
enthält die Vereinigung der betroffenen Paare auf derselben Seite je Fassung.
Ein Seitenwechsel erfasst damit sowohl Nachbarn der Ursprungsseite als auch
Nachbarn der Zielseite. Für jedes erwartete Segment braucht die Prüfung einen
eindeutigen Beleg. Bei nicht belegtem Bestand bleibt das Ergebnis unklar.

Die synthetische MuseScore-Probe prüft diesen Vertrag bei einem Systembruch
und einem Seitenbruch. Der Regelbetrieb beobachtet die Vorschläge am
nach dem Layout erneut gebundenen Notationsstand. Die synthetische PDF-Probe und der
Korpusbericht liefern Beobachtungen. Sie erteilen keine Schreibbefugnis.

Im geprüften SVG-Profil muss der Hals auf der vorgesehenen Seite am eigenen
Kopf in positiver Fläche ansetzen. Für ein verändertes fremdes Hals–Kopf- oder
Hals–Bogen-Paar fordert die Probe mindestens eine normale Halsstrichbreite
Abstand zuzüglich der Messreserve. Sie verwendet den belegten lokalen
Notenlinienabstand als Maßstab. Unklare geänderte Paare bestehen die Probe
nicht. Der PDF-Abgleich bestätigt Halsstriche und beide Bogenfragmente. Der
Abgleich der Notenkopfkonturen beruht hier auf der SVG-Ausgabe.

Die zweiseitige Probe verwendet den wirklichen Halsplaner und die bestehende
Erhaltungsprüfung für die MusicXML-Änderung. Sie bestätigt die vorher falschen
und anschließend richtigen Hälse und verwirft das alte PDF als Beleg für die
neue Fassung. Eine erneute Planung der geprüften Fassung ergibt keinen
weiteren Vorschlag. Zum Ausführen muss `ETUSCHLOPF_PDF_RENDERER` auf MuseScore
zeigen.

```powershell
cargo +1.98.0 test --locked --bin etuschlopf tied_stem_correction_renders_across_page_breaks -- --ignored --nocapture
```

Eine spätere produktive Entscheidung braucht eine durchgehende Bindung
aller relevanten Zeichen bis in das tatsächlich veröffentlichte PDF. Sie
übernimmt jede belegte unabhängige Gruppe für sich und jede belegte gekoppelte
Gruppe gemeinsam. Der Schlussbeleg bestätigt exakt die ausgewählte Fassung.
Bei einem abweichenden Schlussbeleg bleibt die für die Ausgabe belegte
Ausgangsfassung erhalten. Danach endet die Entscheidung.

## PDF-Konturprobe ab r2018

Die ermittelte Halsrichtung bleibt an die vorhandene rhythmische Einheit
gebunden. Die r2018-Probe schärft die Beobachtung der Ausgabe. Der Prüfer
erfasst die tatsächlich eingebettete Kontur des Notenkopfs und die
zugehörige Transformation in Seitenkoordinaten. Die Halsansatzprüfung
verlangt positive Überdeckung in der vorgesehenen Region des eigenen Kopfs.
Für fremde Köpfe verwendet sie den konservativen Abstand zwischen den
Außenhüllen. Der Prüfer behandelt eine ungeklärte Zuordnung oder
ein ungeprüftes PDF-Zeichenprofil als fehlenden Nachweis. Die [PDF-Konturprobe](final_pdf_head_contour_evidence_r2018.md)
läuft mit synthetischen Eingaben und erteilt keine produktive Freigabe.

## PDF-Textkonturen ab r2019

Der Prüfer erfasst im unterstützten Zeichenprofil auch Text, der in der
PDF-Datei innerhalb einer `ActualText`-Markierung gezeichnet wird. Die
Textmarkierung ändert die Geometrie des eingeschlossenen Zeichenbefehls
nicht. Ihre leere Textersetzung kann deshalb eine belegte Glyphenkontur
nicht aus der Kollisionsermittlung entfernen.

Für fremde Textzeichen verwendet der Prüfer die aus dem eingebetteten
Schriftprogramm ermittelte Außenhülle im Seitenkoordinatensystem. Er
prüft den Abstand zu dieser Hülle mit derselben Mindestreserve wie bei
fremden Notenköpfen. Der [PDF-Textkonturbeleg](final_pdf_foreground_text_evidence_r2019.md)
beschreibt die Beobachtung. Halsplanung und Freigabe der Ausgabe bleiben
an ihre bestehenden Voraussetzungen gebunden.

## Vorher-Nachher-Abgleich ab r2020

Der Audit überprüft eine einzeln und gemeinsam gerenderte Vorschlagsmenge.
Er vergleicht die Quellen bis auf die vorgesehenen expliziten Halsrichtungen
und bindet die gezeichneten Hälse, Köpfe und Haltebögen in farbneutralen
Paaren aus Normal- und Markierungsrender. Die Entscheidung behandelt eine
unvollständig beobachtete Kontur als unklar. Akkordköpfe, notierte Balken und
zulässige Fähnchenstufen gehören zum Quellinventar. Der erste Vorher-Nachher-
Nachweis erfasst Änderungen an einfachen Halsanschlüssen und schützt die
umliegenden Akkord-, Balken- und Fähnchenformen.

Die Abstandsmessung berücksichtigt die gezeichnete Tinte. Sie trennt aus
dem SVG gebundene Notenlinien als Führungsgeometrie ab. Sie lässt die
vorhandene Kreuzung eines Haltebogens mit demselben gebundenen Taktstrich
zwischen seinen Quellnoten zu, wenn beide Fassungen den Kontakt im Inneren
des Strichs belegen. Die gemeinsame Vorschau überprüft jeden Vorschlag
erneut gegen alle übrigen beobachteten Formen. Der
[Vorher-Nachher-Abgleich](final_pdf_tied_stem_transition_r2020.md) beschreibt
die Voraussetzungen für die externe Renderprobe.
