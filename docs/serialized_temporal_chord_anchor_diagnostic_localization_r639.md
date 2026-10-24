# r639 – SerializedTemporalChordAnchorDiagnosticLocalization

## Ausgangslage

Der installierte Parent ist r638 `SourceSelectionCandidateGuardHistoricalManifestDecouplingRecovery`.

Zwei spaetere MusicMutation-Kandidaten wurden im realen Etuede-6-Reexec fail-closed verworfen. Build, Strict-Clippy und integrierte Rust-Tests waren jeweils gruen; der produktive Lauf scheiterte erst beim kanonischen Master-MusicXML-Schreiben an der bereits bestehenden Temporal-Invariante:

`serialized temporal materialization contains a chord member longer than its cursor-carrying anchor`

Die im Assistant-Export vorhandenen Master-/Render-MusicXML- und PDF-Artefakte blieben dabei die vorherigen r638-Dateien und duerfen nicht als Ergebnis des verworfenen Kandidaten interpretiert werden.

## Schnitt

Dieser Take ist `RefactorOnly`.

`validate_serialized_temporal_materialization` behaelt dieselbe fail-closed Entscheidung:

`chord_duration > anchor_duration => Err(InvalidData)`

Es wird ausschliesslich die Diagnose erweitert. Bei genau diesem Fehler werden aus dem bereits vorliegenden serialisierten XML ausgegeben:

- Part-ID
- Measure-Nummer
- Anchor-Onset
- Anchor-Dauer
- Anchor-Pitch
- Anchor-Voice
- Anchor-Staff
- Chord-Dauer
- Chord-Pitch
- Chord-Voice
- Chord-Staff

Die vorhandene Unit-Regression beweist zusaetzlich, dass ein laengeres Chord-Mitglied weiterhin abgelehnt wird und die Diagnose die konkreten Fixture-Identitaeten nennt.

## Nicht enthalten

Keine SourceSelection-Aenderung.
Keine Staff-/Voice-Aenderung.
Keine Span-/Jump-/Register-Schwelle.
Keine Delete-Authority-Aenderung.
Keine Candidate-/Score-/Guard-Aenderung.
Keine Retention-Kopplung.
Keine Writer-/MusicXML-Reparatur.
Keine Chord-Normalisierung.
Kein Fallback.
Keine Etuede-spezifische Sonderlogik.

## Ziel

Der reale Reexec soll den bislang anonymen Temporal-Fehler auf die konkrete materialisierte Beziehung lokalisieren. Erst danach wird entschieden, ob die Ursache in SourceSelection, Voice-/Chord-Gruppierung oder einer bereits vorhandenen Materialisierungsannahme liegt.
