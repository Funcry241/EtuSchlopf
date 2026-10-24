# NotationMetricBeamPlanAdoption – v0.10.1600

## Ausgangspunkt

v0.10.1599 ist nach realem Zielsystemlauf hellgruen: Strict Clippy 0,
FinalExit 0, Master- und Render-MusicXML gegen v0.10.1598 nur mit geaenderter
Softwareversionszeile, alle sechs PDF-Seiten pixelidentisch.

Der generische `notation.rs`-Polish-Pfad war nach Gould, scoreweiter
metrischer Kontinuitaet und Partial-Beat/Post-Rest noch eine verbleibende
produktive Balkenzustandsautoritaet. Er erzeugte Balkenebenen und
`begin/continue/end` lokal als rohe Strings.

## Aenderung

- Die bereits ausgewaehlten Noten einer Staff-/Voice-/Beat-Gruppe werden wie
  bisher nach `(Onset, Notenposition)` geordnet.
- Die primaere Balkenebene umfasst weiterhin die komplette Gruppe.
- Balkenebenen 2 bis 4 umfassen weiterhin nur zusammenhaengende Laeufe von
  mindestens zwei Noten mit ausreichendem Notentyp.
- `BeamPlan::from_level_groups` erzeugt die typisierten Zustandsedits
  fail-closed.
- Die Eventordinals werden vor dem unveraenderten XML-Adapter wieder auf die
  bisherigen Notenpositionen abgebildet.
- `add_metric_beam_states` und `add_beam_level_states` werden entfernt.

## Bewusst unveraendert

- Staff-/Voice-/Beat-Gruppierung
- Partial-Beat/Post-Rest- und Sicherheitspruefungen
- Restabdeckung und Clef-Boundary-Splits
- Balkenfaehigkeit nach Notentyp
- Entfernen und Einsetzen bestehender MusicXML-`<beam>`-Elemente
- Writer, Sink und Pipeline-Reihenfolge
- Pitch, Onset, Dauer, Staff, Voice, Pausen und Notenanzahl

## Statische Aequivalenz

Die alte lokale Ebenenbildung und die neue `BeamPlan`-Abbildung wurden ueber
200.000 zufaellige Gruppen mit 2 bis 20 Noten verglichen. Eingeschlossen waren
gemischte Achtel-, 16tel-, 32tel- und 64tel-Ebenen, gleiche Onsets und
variierende Notenpositionen. Alle pro Note erzeugten Balkenlevel und
`begin/continue/end`-Zustaende waren identisch.
