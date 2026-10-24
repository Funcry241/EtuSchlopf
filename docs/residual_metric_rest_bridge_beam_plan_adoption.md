# ResidualMetricRestBridgeBeamPlanAdoption – v0.10.1602

## Ausgangspunkt

v0.10.1601 ist nach realem Zielsystemlauf hellgruen und autoritativ:
Strict Clippy 0, Build 0, FinalExit 0, Master- und Render-MusicXML gegen
v0.10.1600 nur mit geaenderter Softwareversionszeile und alle sechs PDF-Seiten
pixelidentisch. Der bekannte externe MuseScore-Prozessabgang erfolgte erst nach
gueltiger PDF-Erzeugung.

Die Balkeninventur zeigte im bereits migrierten Composer-Renderpfad noch zwei
feste Gruppen, die ihre Zustandszeilen lokal als rohe
`begin/continue/end`-Strings erzeugten:

- metrische terminale 16tel–8tel–16tel-Gruppe, Beam-Level 1,
- dynamische untere Note–Pause–Note-Rest-Bridge, Beam-Level 1+2.

## Aenderung

- Der bisherige lokale Structural-Helfer wird zum allgemeinen
  `complete_group_beam_plan(group_len, highest_level)` verallgemeinert.
- `extend_complete_group_beam_states` bindet typisierte Plan-Edits an die
  bereits ausgewaehlten `PartialBeatNoteInfo`-Zeilen, lehnt fremde Ordinals und
  widerspruechliche Zustandszeilen fail-closed ab und dupliziert identische
  Zeilen nicht.
- Die terminale 16tel–8tel–16tel-Gruppe erzeugt ihren Primaerbalken ueber
  denselben `BeamPlan`.
- Die dynamische untere Rest-Bridge erzeugt ihre Primaer- und Sekundaerbalken
  ueber denselben `BeamPlan`.
- Der bereits migrierte Structural-Closure-Merge verwendet nun ebenfalls den
  gemeinsamen Mapper statt einer zweiten lokalen Plan-zu-Zeilen-Abbildung.

## Bewusst unveraendert

- Kandidatenauswahl und metrische 16tel–8tel–16tel-Bedingung
- Note–Pause–Note-Erkennung, Staff 2 / Voice 2 und Beat-Grenzen
- `existing_beams`-Entscheidung und disjunkte Gruppenbindung
- MusicXML-Adapter, Writer, Checkpoint-Sinks und Pipeline-Reihenfolge
- Pitch, Onset, Dauer, Staff, Voice, Pausen, Notenanzahl und musikalischer Klang
- Report- und Releaseprojektion

## Statische Aequivalenz

Die alte lokale Zustandsbildung und die neue `BeamPlan`-Abbildung wurden ueber
200.000 zufaellige feste Gruppen mit ein bis acht Balkenebenen sowie ueber
200.000 zufaellige Mergefaelle mit leeren, identischen und widerspruechlichen
Vorbelegungen verglichen. Fuer die produktiven Dreiereigruppen waren
Zustandsmenge, Reihenfolge, Annahme und Ablehnung identisch.
