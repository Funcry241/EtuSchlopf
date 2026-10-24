# r234 CandidateProfileTypeIdentity

ChangeClass: RefactorOnly.

## Scope

- `FlpImportProfile` wird zu `CandidateProfile`.
- Alle 62 lebenden Rust-Verwendungen werden geschlossen in neun bestehenden Rust-Dateien umgestellt.
- Der Typ bleibt derselbe opaque, typisierte Kandidaten-/Solververtrag aus `crate::pipeline_profile`.
- `CandidateProfileKind`, `CandidateProfileFamily`, `ProfileParameters`, alle Profilwerte und alle Methoden bleiben unveraendert.
- Der neue Name ist bewusst nicht `MidiSeedProfile`, weil derselbe Vertrag auch Katalog, Lineage, Korpusdiagnostik und Pipeline-Orchestrierung traegt.
- `flp_to_musicxml_seed*` sowie weitere historische `flp_*`-Funktions-/Variablenidentitaeten bleiben fuer spaetere getrennte Takes bestehen.

## Refactor-Beweis

Die neun geaenderten Rust-Dateien unterscheiden sich vom real gruenen r233 ausschliesslich durch die token-genaue Substitution

```text
FlpImportProfile -> CandidateProfile
```

Wird diese Substitution rueckgaengig gemacht, sind alle neun Rust-Dateien byteidentisch zum r233-Elternstand.

Keine Musik-, MIDI/WAV-, MusicXML-, Solver-, Writer-, Renderer-, Layout-, Report-, PDF-, Repair-, Recovery-, Selection- oder Dependency-Semantik wird veraendert.
