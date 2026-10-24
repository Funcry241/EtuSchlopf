# MidiSeedPhysicalOwnerRelocation r229

## Zweck

Der logische Rust-Modulname wurde in r227 bereits auf `midi_seed` umgestellt und real gruen bestaetigt. r228 etablierte anschliessend real gruen den engen Security-Vertrag fuer eine byteidentische, identity-preserving Rust-Owner-Relocation unter `RefactorOnly`.

r229 fuehrt nun ausschliesslich den physischen Owner-Cutover aus:

```text
src/flp.rs -> src/midi_seed.rs
```

Der Dateiinhalt bleibt byteidentisch. In `src/main.rs` wird nur die explizite Pfadbindung von

```rust
#[path = "flp.rs"]
mod midi_seed;
```

auf

```rust
#[path = "midi_seed.rs"]
mod midi_seed;
```

umgestellt. Die Modulidentitaet `midi_seed` bleibt damit unveraendert.

## Invarianten

- ChangeClass: `RefactorOnly`
- genau ein nicht-sicherheitskritischer Rust-Owner wird geloescht und genau einer im selben Parent hinzugefuegt
- Alt- und Neuowner sind byteidentisch
- gleiche Domain
- gleiche explizite Modulidentitaet `midi_seed`
- kein Security-Pfad wird veraendert
- keine Musik-, MIDI-, WAV-, MusicXML-, Solver-, Writer-, Renderer-, Layout-, Report- oder PDF-Semantik wird veraendert
- keine neue Repair-, Recovery-, Selection- oder Writer-Authority
- historische `Flp*`-/`flp_*`-Symbolnamen bleiben in r229 unangetastet

## Abnahme

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Nach real gruenem Lauf werden Revision/Gates sowie MusicXML/PDF-Nebenwirkungsfreiheit erneut geprueft.
