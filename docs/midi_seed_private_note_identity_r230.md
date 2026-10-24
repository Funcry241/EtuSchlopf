# MidiSeedPrivateNoteIdentity r230

## Zweck

r229 hat den physischen Owner `src/flp.rs -> src/midi_seed.rs` real gruen abgeschlossen.
r230 entfernt nun genau eine verbleibende historische interne Typidentitaet innerhalb dieses Owners:

```text
FlpNote -> MidiSeedNote
```

`FlpNote` ist ein privater Struct-Typ in `src/midi_seed.rs`. Er besitzt keine Cross-File-API.
Alle Verwendungen liegen innerhalb desselben Owners und werden in diesem Take geschlossen umbenannt.

## Invarianten

- ChangeClass: `RefactorOnly`
- nur die private Typidentitaet `FlpNote` wird in `MidiSeedNote` umbenannt
- Struct-Felder, Feldtypen und Feldreihenfolge bleiben unveraendert
- keine Funktionsnamen werden umbenannt
- keine Funktionssignatur wird ausser durch den privaten Typnamen semantisch veraendert
- keine Cross-File-Schnittstelle wird bewegt
- keine MusicXML-, MIDI-, WAV-, Solver-, Writer-, Renderer-, Layout-, Report- oder PDF-Semantik wird veraendert
- keine neue Repair-, Recovery-, Selection- oder Writer-Authority entsteht

## Bewusst nicht Teil von r230

Die weiterhin historischen Namen `FlpImportProfile`, `FlpSeedStats`, `flp_seed_stats`,
`flp_solver_profiles` und `flp_to_musicxml_seed*` bleiben unangetastet. Sie werden erst nach
einem real gruenen r230 separat bewertet.
