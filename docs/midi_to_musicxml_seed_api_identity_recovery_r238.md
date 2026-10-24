# r238 MidiToMusicXmlSeedApiIdentityRecovery

ChangeClass: RefactorOnly.

## Elternstand

r238 basiert direkt auf dem real dunkelgruenen r237 `ExactRefactorFunctionSymbolRenameContractTestFixtureRecovery`. r237 ist der erste installierte und vollstaendig gruene Stand der engen Function-Symbol-Rename-Foundation. Der abgewiesene r235-Kandidat und der nach Testfehler vollstaendig zurueckgerollte r236 sind keine Elternstaende.

## Scope

- Die bestehende oeffentliche Seed-API-Familie `flp_to_musicxml_seed*` wird geschlossen auf `midi_to_musicxml_seed*` umbenannt.
- Betroffen sind ausschliesslich die drei bestehenden Funktionsidentitaeten `flp_to_musicxml_seed`, `flp_to_musicxml_seed_with_profile` und `flp_to_musicxml_seed_with_profile_and_hotspots` samt ihren bestehenden internen und Cross-File-Verwendungen.
- Insgesamt werden acht lebende Identifier-Vorkommen in genau drei bestehenden Rust-Dateien umgestellt.
- Der neue Name beschreibt die produktive Quellautoritaet korrekt: Standard-MIDI wird in denselben MusicXML-Seed ueberfuehrt.
- Funktionskoerper, Parameter, Rueckgabetypen, `CandidateProfile`, `HotspotPlan`, `MidiSeedStats`, Sortierung, Solver-/Handlogik und `build_musicxml` bleiben unveraendert.
- Die Writer-Roots bleiben unveraendert `apply_hand_trajectory_solver`, `apply_hotspot_reductions` und `apply_residual_critical_pass`.
- Die r237-Security-Foundation wird in diesem Take nicht veraendert.
- Die owner-lokalen historischen Identifier `_flp_seed_optional_api_keepalive` und `redistribute_flp_hands_for_piano` bleiben absichtlich fuer den naechsten getrennten Restidentitaets-Take bestehen.

## Refactor-Beweis

Die drei geaenderten Rust-Dateien unterscheiden sich vom real dunkelgruenen r237 ausschliesslich durch die token-genauen Substitutionen

```text
flp_to_musicxml_seed_with_profile_and_hotspots -> midi_to_musicxml_seed_with_profile_and_hotspots
flp_to_musicxml_seed_with_profile              -> midi_to_musicxml_seed_with_profile
flp_to_musicxml_seed                           -> midi_to_musicxml_seed
```

Werden diese Substitutionen rueckgaengig gemacht, sind alle drei Rust-Dateien byteidentisch zum r237-Elternstand.

Keine Musik-, MIDI/WAV-, MusicXML-, Solver-, Writer-, Renderer-, Layout-, Report-, PDF-, Repair-, Recovery-, Selection- oder Dependency-Semantik wird veraendert.
