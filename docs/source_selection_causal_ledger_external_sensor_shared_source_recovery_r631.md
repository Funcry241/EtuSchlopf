# r631 – SourceSelectionCausalLedgerExternalSensorSharedSourceRecovery

## Ausgangspunkt

r631 basiert direkt auf dem installierten r630 `SourceSelectionCausalLedgerFoundation`. Der r630-Hauptbuild und Strict-Clippy waren GREEN, der normale Lauf von Etuede 6 zeigte jedoch beim separat gebauten audit-only WAV-Pitch-Sensor einen Integrationsfehler: Der Sensor uebernahm `src/midi.rs` als `shared_midi.rs`, nicht aber dessen seit r630 erforderlichen `src/source_event.rs`-Owner. Dadurch fehlten `normalize_tick_exact`, `project_source_event_causality`, `SourceEventCausalEntry`, `SourceEventCausalLedger` und `SourceEventLocator` im isolierten Audit-Crate.

Der Fehler betrifft keine MusicXML-/PDF- oder Musikmutation und veraendert die r630-SourceEvent-Kausalitaet nicht. Er ist eine unvollstaendige Shared-Source-Grenze des optionalen, evidence-only Sensors.

## Abschluss

r631 synchronisiert fuer den isolierten Rust-Sensor nun immer beide zusammengehoerigen Quellen:

- `src/midi.rs` -> `src/shared_midi.rs`,
- `src/source_event.rs` -> `src/shared_source_event.rs`.

Die lokale, seit r157 ausreichende Minimal-Fassade fuer `CanonicalSourceEvent` wird entfernt. Das Sensor-Crate kompiliert stattdessen exakt den aktuellen `source_event.rs`-Owner. Eine kleine lokale `sha256::hex`-Fassade stellt nur die bereits vom Owner benoetigte Hashfunktion bereit; sie besitzt keine Source-, Selection-, Ranking-, Writer- oder Musikautoritaet.

Der portable Sensor-Package-Fingerprint bindet nun ebenfalls beide gemeinsam kompilierten Source-Dateien. Aendert sich entweder der MIDI-Decoder oder der SourceEvent-Owner, wird der gecachte Audit-Sensor neu gebaut und seine Paketidentitaet aendert sich deterministisch.

## Regression

`tests/wav_pitch_external_sensor_shared_source_audit_contract.rs` prueft statisch fail-closed, dass

- der produktive MIDI-Decoder den r630-Kausalvertrag verwendet,
- der aktuelle SourceEvent-Owner die benoetigte API bereitstellt,
- der Audit-Stager beide Source-Dateien synchronisiert,
- das isolierte Tool beide Shared-Dateien einbindet,
- keine alte lokale SourceEvent-Minimal-Fassade zurueckkehrt,
- der Package-Fingerprint beide Shared-Source-Dateien bindet.

Erwarteter integrierter Testbestand: 1238/1238.

## Nicht-Scope

`ChangeClass=AuditOnly`. Unveraendert bleiben insbesondere:

- alle Bytes von `src/midi.rs` und `src/source_event.rs`,
- r630 CanonicalSourceEvent-Identitaet und beide Ledger-Fingerprints,
- MIDI-Seed und Solver-Input,
- Selection, Ranking, Weighting und Candidate-Reihenfolge,
- Hand-, Staff-, Voice-, Rhythmus-, Register- und Layoutentscheidungen,
- Writer, MusicXML, Render und PDF,
- Musikmutation und Pass-Reihenfolge.

r631 ist damit ein enger Integrations-Recovery-Take vor dem geplanten zweiten fachlichen SourceSelection-Take.
