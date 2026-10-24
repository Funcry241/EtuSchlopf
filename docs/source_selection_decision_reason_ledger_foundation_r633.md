# r633 SourceSelectionDecisionReasonLedgerFoundation

## Zweck

r633 baut direkt auf dem real gruenen r632 `SourceSelectionCanonicalIdentityTransportBoundary` auf.
Der r632-Nachlauf ist fuer Etuede 6 geschlossen: FinalExit 0, Build 0, Strict-Clippy 0,
externer Rust/ONNX-Sensor erfolgreich und 1241/1241 integrierte Tests GREEN.

r633 entwickelt ausdruecklich keine neue Selection-Semantik. Der Take macht nur sichtbar, welche
bereits vorhandene MIDI-Seed-Removal-Verzweigung ein spaeter fehlendes CanonicalSourceEvent
tatsaechlich entfernt hat.

## Beweisobjekt

Jeder bestehende Removal-Pfad bindet beim bereits vorhandenen Entfernen die unveraenderte
`source_ordinal` an genau einen stabilen Reason-Code:

- `hotspot_span_reduction`
- `hotspot_jump_reduction`
- `hotspot_register_outlier_reduction`
- `trajectory_zigzag_reduction`
- `residual_critical_span_reduction`
- `residual_critical_jump_reduction`

Staff-Reassignments sind bewusst nicht Teil dieses Ledgers, weil r632 an der Selection-Grenze
ausschliesslich retained/removed SourceEvents beweist.

Nach allen unveraenderten Seed-Paessen prueft
`project_source_selection_decision_reasons()` fail-closed:

- CanonicalSourceEvent-Ordinalitaeten sind eindeutig und lueckenlos,
- retained SourceEvents bleiben in Position, Dauer und Pitch kanonisch,
- kein retained Event besitzt einen Removal-Reason,
- kein entferntes Event besitzt mehr als einen Removal-Reason,
- jeder Removal-Reason referenziert ein vorhandenes CanonicalSourceEvent,
- die reason-gebundenen Ordinalitaeten sind exakt das r632-entfernte Komplement,
- `UnexplainedRemovedEvents` ist damit definitionsgemaess 0.

Ein domain-separierter SHA-256-Fingerprint bindet jede entfernte kanonische Identitaet zusammen
mit dem bereits bestehenden Removal-Branch. Der Fingerprint ist Evidenz und kein Selection-Key.

## Nicht-Scope

Unveraendert bleiben:

- SourceEvent-Decoding, FIFO-Paarung und r630-Kausalledger,
- r632-Identitaetstransport und dessen drei Fingerprints,
- MIDI-Seed-Passreihenfolge,
- CandidateProfile-Werte, Schwellen und Gewichte,
- alle bestehenden Removal-Bedingungen und Cap-Regeln,
- Staff-/Hand-/Voice-Heuristiken und Reassignments,
- Candidate-Ranking und Solverentscheidungen,
- Writer, MusicXML, Render, PDF und Musik.

`ChangeClass=RefactorOnly`, weil bestehende produktive Removal-Stellen ausschliesslich um
read-only Provenienztransport ergaenzt werden. Die Selection selbst wird nicht veraendert.
Erwarteter integrierter Testbestand: 1244/1244.
