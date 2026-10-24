# r632 – SourceSelectionCanonicalIdentityTransportBoundary

## Ausgangspunkt

r632 basiert direkt auf dem real gruenen r631. Der Etuede-6-Nachlauf bestaetigt `FinalExit 0`, `ReleaseDecision released`, Build/Strict-Clippy Exit 0, den beobachteten externen Rust/ONNX-WAV-Sensor und exakt 1238/1238 integrierte Tests. Damit ist die r630-SourceEvent-Kausalitaet einschliesslich der r631-Sensor-Integrationsgrenze geschlossen.

## Noch offene Identitaetsluecke

Vor r632 besitzt jedes `CanonicalSourceEvent` eine stabile `source_ordinal`, der produktive `MidiSeedNote` verwirft diese Identitaet aber beim Eintritt in die bestehende Seed-Selektion. Die folgenden unveraenderten Seed-Paesse duerfen Staff umverteilen oder Events entfernen. Ohne erhaltene Ordinalitaet waere danach insbesondere bei gleichen Pitch/Onset-Paaren nicht beweisbar, welches konkrete CanonicalSourceEvent erhalten bzw. entfernt wurde.

## Take-2-Schnitt

r632 transportiert deshalb ausschliesslich `source_ordinal` als opaques Identitaetsfeld in `MidiSeedNote`. Kein bestehender Sortier-, Staff-, Removal-, Hotspot-, Profil- oder Candidate-Ausdruck liest dieses Feld als Entscheidungseingang. Nach Abschluss der bereits vorhandenen Seed-Paesse erzeugt `project_source_selection_identity_transport()` eine read-only Projektion.

Sie prueft fail-closed:

- CanonicalSourceEvent-Ordinalitaeten sind eindeutig und lueckenlos `0..n-1`,
- jede erhaltene Seed-Ordinalitaet existiert exakt einmal im CanonicalSourceEvent-Satz,
- kein erhaltenes Event veraendert Position, Dauer oder Pitch,
- kein SourceEvent wird durch die Seed-Selektion dupliziert,
- entfernte Ordinalitaeten sind exakt das Komplement der erhaltenen Ordinalitaeten,
- Pitch/Onset-Mehrfachgruppen bleiben ueber ihre Ordinalitaeten unterscheidbar.

Drei domain-separierte SHA-256-Fingerprints binden den kanonischen Input, den erhaltenen Seed-Subset und das entfernte Komplement jeweils in Source-Ordinalitaetsreihenfolge. Diese Fingerprints sind Beweise, keine Selection-Keys.

## Sichtbarkeit

Der finale Projektbericht erhaelt genau eine Sektion:

`SOURCE-SELECTION-CANONICAL-IDENTITY-TRANSPORT`

Sie berichtet Input-, Retained- und Removed-Zahlen, Mehrfachgruppen/-instanzen und die drei Identitaetsfingerprints. Der Vertrag deklariert ausdruecklich `SelectionRulesChanged: nein`, `SelectionReasonLedger: none`, `RankingChanged: nein`, `WeightingChanged: nein`, `HandStaffVoiceAuthority: none`, `WriterAuthority: none` und `MusicMutation: nein`.

## Nicht-Scope

Unveraendert bleiben:

- SourceEvent-Paarung und r630-Ledger-Fingerprints,
- bestehende MIDI-Seed-Profile und deren Reihenfolge,
- alle Staff-/Hand-/Voice-Heuristiken,
- alle Removal-/Hotspot-/Trajectory-/Residual-Regeln,
- Candidate-Erzeugung, Candidate-Ranking, Scores, Schwellen und Gewichte,
- Arrangement-, Rhythmus-, Register-, Layout- und Notationsregeln,
- Writer, MusicXML, Render und PDF,
- Musikmutation und Pass-Reihenfolge.

`ChangeClass=RefactorOnly`, weil die Identitaet durch eine bestehende produktive Datenstruktur transportiert wird. Zwei Unit-Regressionen pruefen Mehrfachidentitaet und fail-closed Payloadabweichung; eine Integrationsregression bindet den produktiven Transport an die Reportprojektion. Erwarteter integrierter Testbestand: 1241/1241.
