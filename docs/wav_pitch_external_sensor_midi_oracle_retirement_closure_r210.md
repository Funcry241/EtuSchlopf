# r210 – WAV Pitch External Sensor MIDI Oracle Retirement Closure

## Ziel

Der externe WAV-Pitch-Sensor darf nach dem FLP→MIDI-Cutover keine `.flp`-Datei mehr benoetigen.

## Schnitt

- Stage 1 bleibt strikt WAV-only und friert die Sensorprojektion vor jedem Oracle-Zugriff ein.
- Stage 2 liest danach die gleichnamige `.mid`-Datei als reine Evaluationsevidenz.
- Der Sensor verwendet keinen separaten MIDI-Parser. `src/midi.rs` wird beim isolierten Werkzeug-Build als `src/shared_midi.rs` synchronisiert.
- Der Werkzeug-Cache wird bei jeder Aenderung des gemeinsamen MIDI-Decoders invalidiert.
- Der PackageFingerprint des Werkzeugs bindet die Bytes von `src/midi.rs`.
- `midi_authority=none_evaluation_only_post_freeze` beschreibt den realen Vertrag: MIDI wird transportiert, besitzt aber keine Sensor- oder Writer-Autoritaet.
- Kein Source-Event wird aus dem Sensor in den produktiven Pfad promoviert.

## Nicht Teil dieses Takes

- keine Aenderung an `src/flp.rs`
- keine Launcher-/Security-Hygiene
- keine Solver-/Writer-Aenderung
- keine musikalische Mutation
