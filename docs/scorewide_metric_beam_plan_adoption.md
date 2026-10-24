# ScorewideMetricBeamPlanAdoption – v0.10.1598

## Ausgangspunkt

v0.10.1597 ist nach realem Zielsystemlauf hellgruen: Strict Clippy 0,
FinalExit 0, Master- und Render-MusicXML gegen v0.10.1596 nur mit geaenderter
Softwareversionszeile, alle sechs PDF-Seiten pixelidentisch.

Der scoreweite metrische Unterhandpfad besass bereits eine source-gebundene
Kandidatengrenze und einen kontrollierten nachgelagerten Dateisink. Seine
Beam-Zustaende wurden jedoch noch lokal als rohe `(level, "begin|continue|end")`
Zeilen durch `insert_metric_beam_group_states` aufgebaut. Damit existierte
neben dem gemeinsamen `BeamPlan` weiterhin eine zweite Zustandsbildung.

## Aenderung

- `BeamPlan::from_groups` bildet mehrere disjunkte, geordnete Ereignisgruppen
  fail-closed in typisierte `BeamEdit`s ab.
- Der Gould-Detector verwendet denselben Gruppenconstructor; seine
  1/4-1/2-1/4-Regel bleibt unveraendert.
- `ScorewideMetricBeamContinuityWrite` bindet seine bereits ausgewaehlten
  ersten drei und letzten zwei Noten an Ereignisordinale und erzeugt daraus
  genau einen gemeinsamen `BeamPlan`.
- Der kleine Adapter uebersetzt die typisierten Edits in die bereits bestehende
  Note-Fragment-Schreibprimitive.
- `insert_metric_beam_group_states` wird entfernt.
- Die MusicXML-Schreibweise eines `BeamState` liegt nur noch einmal im
  gemeinsamen Modell und wird auch vom bestehenden Gould-Writer verwendet.

## Bewusst unveraendert

- Auswahl der Staff-2-/Voice-Kandidaten
- Beat-, Kontinuitaets- und Dauerregeln
- erste Dreiergruppe und letzte Zweiergruppe
- Beam-Level-1-/Level-2-Entscheidung
- Entfernen und Einsetzen vorhandener `<beam>`-Tags
- source-gebundene Kandidatenversiegelung
- Pipeline-Reihenfolge und produktiver Dateisink
- Pitch, Onset, Dauer, Staff, Voice, Pausen und Notenanzahl

Dieser Take fuehrt noch keinen universellen XML-Writer ein. Er entfernt zuerst
die zweite Zustandsplanung, ohne die bestehende sichere Schreibgrenze zu
duplizieren.

## Statische Aequivalenz

Die alte lokale und die neue typisierte Zustandsbildung wurden fuer 200.000
zufaellige, produktionskonforme Kombinationen aus Dreier-/Zweiergruppen und
sekundaerer Balkenebene verglichen. Die pro Note erzeugten Level- und
`begin/continue/end`-Zeilen waren identisch.
