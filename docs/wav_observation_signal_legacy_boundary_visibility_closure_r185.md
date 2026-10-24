# r185 – WavObservationSignalLegacyBoundaryVisibilityClosure

## Ausgangslage

r184 hat die neutrale RIFF/WAVE-Signaldecodierung erfolgreich aus
`audit_only/wav_observation_legacy_oracle.rs` in
`audit_only/wav_signal_decode.rs` herausgelöst. Der erste reale Lauf war
funktional vollständig grün und erzeugte gegenüber r182 byte-identische
Master- und Render-MusicXML-Ausgaben.

Der Rust-Compiler meldete jedoch eine neue `private_interfaces`-Warnung:

`build_audio_only_pitch_timing_snapshot` war `pub(crate)`, obwohl sein
Parameter `DecodedWavSignalAudit` bewusst nur `pub(super)` innerhalb des
`audit_only`-Bereichs sichtbar ist.

## Änderung

r185 verengt ausschließlich die Sichtbarkeit von

`build_audio_only_pitch_timing_snapshot`

von `pub(crate)` auf `pub(super)`.

Damit entspricht die Funktionsoberfläche wieder der Sichtbarkeit des
zugehörigen neutralen WAV-Signaltyps. Die bestehenden Verbraucher liegen
allesamt innerhalb von `audit_only` und bleiben über die gemeinsame
Elternmodul-Grenze erreichbar.

Die bestehende WAV-Retirement-Regression bindet zusätzlich diese
Sichtbarkeitsgrenze, damit der neutrale Decoder nicht erneut unbeabsichtigt
eine crate-weite Oberfläche erzeugt.

## Nicht geändert

- keine Decoderberechnung,
- keine WAV-Schwelle,
- keine Gewichtung,
- keine FLP-/Oracle-Semantik,
- keine PPQ-Skalierung,
- keine Musikmutation,
- kein Writer,
- keine MusicXML-/Render-/PDF-Entscheidung,
- keine Release- oder Exit-Semantik.

`ChangeClass: RefactorOnly`.
