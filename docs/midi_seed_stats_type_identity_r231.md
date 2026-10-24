# r231 MidiSeedStatsTypeIdentity

ChangeClass: RefactorOnly.

Scope:
- `FlpSeedStats` wird zu `MidiSeedStats`.
- Alle 9 Typverwendungen werden in vier bestehenden Rust-Dateien geschlossen umgestellt.
- `flp_seed_stats` als Feld-/Variablenname bleibt bewusst unveraendert.
- Keine Funktions-, Modul-, Datei-, Feld-, Writer-, Solver-, Renderer-, MusicXML- oder Musiksemantik wird veraendert.

Der Schritt ist absichtlich kleiner als eine vollstaendige FLP-Symbolbereinigung, damit
Security- und Cross-File-Interface-Gates eine einzelne Identitaetsklasse beurteilen koennen.
