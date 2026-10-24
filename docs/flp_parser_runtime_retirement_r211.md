# r211 – FLP Parser Runtime Retirement

## Ausgangslage

r210 hat den letzten externen FLP-Leser ausserhalb des Launchers pensioniert. Produktiver Quellpfad, WAV-Oracle und externes WAV-Pitch-Sensorwerkzeug verwenden MIDI.

`src/flp.rs` enthielt danach weiterhin einen historischen FLP-Decoder, der seit r203 nur durch lokale Keepalive-Funktionszeiger und zwei Parser-Tests kompiliert wurde. Der produktive Seed-Owner las ihn nicht mehr.

## Schnitt

r211 entfernt ausschliesslich diese tote Decoderoberflaeche:

- `NOTE_EVENT_ID`, `FLHD`, `FLDT`
- `read_flp_ppq`
- `read_flp_notes`
- `parse_note_payload`
- `scale_ticks` des alten FLP-Decoders
- `take_payload`
- `read_var_len`
- parserexklusive Byte-Helfer
- r203-Keepalive-Referenzen
- `truncated_fldt_tests`

Es wird kein Ersatz, Fallback oder Kompatibilitaetsparser eingefuehrt.

## Unveraendert

- produktiver MIDI-Decoder `src/midi.rs`
- bestehender writer-erreichbarer Seed-Owner in `src/flp.rs`
- Solver-/Hand-/Notensatzverhalten
- Profile und Kandidatenordnung
- WAV-Audits und Sensorwerkzeug
- Launcher und Security
- Cargo-Vertrag

Die Datei `src/flp.rs` und historische `Flp*`-Typ-/Funktionsnamen bleiben vorerst bestehen, weil dort weiterhin lebende Solver-/Seed-Verantwortung liegt. Deren spaetere fachliche Umbenennung ist nicht Teil dieses Runtime-Retirements.
