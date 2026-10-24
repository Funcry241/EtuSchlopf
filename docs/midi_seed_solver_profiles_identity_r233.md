# r233 MidiSeedSolverProfilesIdentity

ChangeClass: RefactorOnly.

Scope:
- `flp_solver_profiles` wird zu `midi_seed_solver_profiles`.
- Genau drei lebende Rust-Verwendungen werden geschlossen in drei bestehenden Rust-Dateien umgestellt: Definition, Import und Aufruf.
- Die Funktion bleibt ein unveraenderter Thin-Forwarder auf `base_solver_profiles()`.
- `FlpImportProfile` und `flp_to_musicxml_seed*` bleiben absichtlich fuer spaetere, getrennte Takes bestehen.
- Keine Musik-, MIDI/WAV-, MusicXML-, Solver-, Writer-, Renderer-, Layout-, Report-, PDF-, Repair-, Recovery-, Selection- oder Dependency-Semantik wird veraendert.

Der Schritt ist absichtlich klein, damit die historische FLP-Identitaet ohne Grossrename und ohne Security-Evasion stufenweise geschlossen werden kann.
