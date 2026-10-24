# PartialBeatPostRestBeamPlanAdoption – v0.10.1599

## Ausgangspunkt

v0.10.1598 ist nach realem Zielsystemlauf hellgruen: Strict Clippy 0,
FinalExit 0, Master- und Render-MusicXML gegen v0.10.1597 nur mit geaenderter
Softwareversionszeile, alle sechs PDF-Seiten pixelidentisch.

Die Beat-Scoped- und Partial-Beat/Post-Rest-Pfade verwendeten fuer ihre bereits
strukturell ausgewaehlten Notengruppen noch zwei lokale Funktionen, die
Balkenebenen sowie `begin/continue/end` eigenstaendig aufbauten. Damit blieb
neben `BeamPlan` eine zweite produktive Zustandsplanung bestehen.

## Aenderung

- `BeamPlan::from_level_groups` bildet explizite, geordnete Gruppen je
  Balkenebene fail-closed ab. Dadurch duerfen sekundaere, tertiaere und
  quartaere Balken kuerzere zusammenhaengende Laeufe innerhalb eines
  primaeren Balkens besitzen.
- `BeamPlan::from_groups` delegiert auf dieselbe explizite Ebenenprimitive;
  Gould und ScorewideMetricBeamContinuity behalten ihr Verhalten.
- Beat-Scoped-Beaming, Rest-Bridge-Zweiergruppen und Partial-Beat/Post-Rest
  erzeugen ihre Zustandszeilen nun aus `BeamPlan`.
- `partial_beat_add_beam_states` und `partial_beat_add_level_states` werden
  vollstaendig entfernt.
- Der bestehende XML-Adapter, das Entfernen und Einsetzen von `<beam>`-Tags,
  die Rest-Completion und alle Kandidatenregeln bleiben unveraendert.

## Bewusst unveraendert

- Staff-/Voice-/Beat-Gruppierung und Kontinuitaetsregeln
- sichtbare Restabdeckung und Post-Rest-Zulassung
- Balkenfaehigkeit nach Notentyp
- bestehende Beam-Tags-Guards
- XML-Writer, Render-Sink und Pipeline-Reihenfolge
- Pitch, Onset, Dauer, Staff, Voice, Pausen und Notenanzahl

## Statische Aequivalenz

Die alte lokale Ebenenbildung und die neue `BeamPlan`-Abbildung wurden ueber
200.000 zufaellige Notengruppen mit gemischten Achtel-, 16tel-, 32tel- und
64tel-Ebenen verglichen. Alle pro Note erzeugten Level- und
`begin/continue/end`-Zeilen waren identisch. Zusaetzlich wurde die Delegation
von `BeamPlan::from_groups` fuer 200.000 gueltige und ungueltige
Gruppenkonstellationen gegen die vorherige Implementierung verglichen.
