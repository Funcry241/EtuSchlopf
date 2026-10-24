# r232 MidiSeedStatsFieldIdentity

ChangeClass: RefactorOnly.

Scope:
- `flp_seed_stats` wird zu `midi_seed_stats`.
- Alle 13 lebenden Rust-Verwendungen werden in vier bestehenden Rust-Dateien geschlossen umgestellt.
- Der Typ `MidiSeedStats` aus r231 bleibt unveraendert.
- Keine Funktion, kein Modul, keine Datei und keine Writer-/Solver-/Renderer-/MusicXML-/Musiksemantik wird veraendert.

Der Schritt ist absichtlich auf genau diese Feld-/Variablenidentitaet begrenzt. Die groesseren
historischen FLP-Symbolfamilien bleiben fuer spaetere, separat pruefbare Schritte bestehen.
