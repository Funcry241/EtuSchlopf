# r237 ExactRefactorFunctionSymbolRenameContractTestFixtureRecovery

## Ausgangslage

r235 `MidiToMusicXmlSeedApiIdentity` wurde vom bestehenden `NO-NEW-REPAIR-AUTHORITY SECURITY GATE` vor der Installation korrekt fail-closed abgewiesen, weil drei rein umbenannte, bereits writer-erreichbare Funktionen als neue `WriterReachability`-Symbole erschienen.

r236 sollte dafuer einen engen identitaetserhaltenden RefactorOnly-Vertrag etablieren. Der Take wurde installiert, Build und Clippy waren gruen, danach scheiterte jedoch der integrierte All-Targets-Test des `etuschlopf_take_bootstrap` (Exit 101). Die Take-Transaktion rollte den Quellstand vollstaendig auf den vorherigen real dunkelgruenen r234 zurueck. r236 ist deshalb kein Elternstand.

## Ursache der r236-Testregression

Der Vertrag selbst verlangt unveraenderte Writer-Roots. Die positive r236-Testfixture rief jedoch aus der umzubenennenden Zwischenfunktion direkt `write_musicxml(...)` auf. Nach der bereits bestehenden Writer-Inventur macht genau dieses Primitive die Zwischenfunktion selbst zu einem direkten Writer-Root. Ein Symbolrename dieser Funktion aendert damit den Writer-Root-Key und erfuellt den engen Vertrag absichtlich nicht. Der Test erwartete trotzdem `Ok` und war damit widerspruechlich zum Vertrag.

Der reale r235-Pre-Take-Befund zeigte dagegen fuer alle drei umbenannten Seed-APIs ausschliesslich die unveraenderten Writer-Roots:

- `src/midi_seed.rs::apply_hand_trajectory_solver#1`
- `src/midi_seed.rs::apply_hotspot_reductions#1`
- `src/midi_seed.rs::apply_residual_critical_pass#1`

Die produktive Rename-Familie ist damit eine writer-erreichbare Caller-Familie, kein Rename eines direkten Writer-Roots.

## Recovery

r237 basiert direkt auf r234 und uebernimmt den r236-Vertrag mit korrigierter Regression:

- Der positive Test verwendet eine zweistufige umzubenennende Caller-Familie, die einen separat benannten, unveraenderten `apply_hand_trajectory_solver`-Writer-Root erreicht.
- Keine umzubenennende Testfunktion ist selbst ein direkter Writer-Root.
- Der Negativtest verwendet dieselbe unveraenderte Root-Struktur, veraendert aber zusaetzlich den Funktionskoerper und muss weiterhin mit `NO-NEW-REPAIR-AUTHORITY SECURITY GATE` scheitern.

Nicht erweitert wird der Vertrag: Direkte Writer-Root-Renames werden durch r237 nicht autorisiert.

## Scope

`SecurityOnly`. Keine Produktdatei und insbesondere `src/midi_seed.rs` wird geaendert. Keine Musik-, MIDI/WAV-, MusicXML-, Solver-, Writer-, Renderer-, Layout-, Report-, PDF-, Repair-, Recovery-, Selection- oder Dependency-Semantik wird geaendert. Die notwendigen Compile-Time-Trust-Anchor-FNVs werden gegen die tatsaechlich enthaltenen Bytes koordiniert synchronisiert.
