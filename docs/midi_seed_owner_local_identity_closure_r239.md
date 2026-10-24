# r239 MidiSeedOwnerLocalIdentityClosure

ChangeClass: RefactorOnly.

## Elternstand

r239 basiert direkt auf dem real dunkelgruenen r238 `MidiToMusicXmlSeedApiIdentityRecovery`. Der r238-Assistant-Export weist FinalExit 0, BuildExit 0, ClippyExit 0 und gruenen Launcher-Teststatus aus. Master- und Render-MusicXML sind gegenueber dem real dunkelgruenen r234 byteidentisch; das r238-PDF ist gegenueber r237 auf allen 6 Seiten pixelidentisch.

## Scope

- `_flp_seed_optional_api_keepalive` wird zu `_midi_seed_optional_api_keepalive`.
- `redistribute_flp_hands_for_piano` wird zu `redistribute_midi_seed_hands_for_piano`.
- Betroffen sind genau drei lebende Identifier-Vorkommen in genau `src/midi_seed.rs`.
- Die Keepalive-Bindung zeigt weiterhin auf `midi_to_musicxml_seed`.
- Die Handverteilungsfunktion arbeitet unveraendert auf `&mut [MidiSeedNote]`, `CandidateProfile` und optionalem `HotspotPlan`.
- Der Funktionskoerper, die Hand-/Staff-Zuordnung, Schwellenwerte, Sortierung, Solver-Aufrufreihenfolge und MusicXML-Erzeugung bleiben unveraendert.
- Die Security-Foundation aus r237 bleibt byteidentisch und wird nicht erweitert.

## Refactor-Beweis

`src/midi_seed.rs` unterscheidet sich vom real dunkelgruenen r238 ausschliesslich durch die token-genauen Substitutionen

```text
_flp_seed_optional_api_keepalive      -> _midi_seed_optional_api_keepalive
redistribute_flp_hands_for_piano      -> redistribute_midi_seed_hands_for_piano
```

Werden diese beiden Substitutionen rueckgaengig gemacht, ist `src/midi_seed.rs` byteidentisch zum r238-Elternstand.

Nach der Umstellung existiert in produktivem Rust kein lebender Rust-Identifier mehr, dessen Name `flp` enthaelt.

## Bewusst nicht Teil dieses Takes

Verbleibende Texttreffer `FLP`/`flp` werden nicht mechanisch umgeschrieben. Sie gehoeren zu unterschiedlichen Klassen:

- historische Kommentare und Dokumentation ueber den frueheren FLP-Owner,
- Security-/Retirement-Historie,
- aktive Report-/Hinweisvokabeln wie `WAV/FLP`.

Insbesondere aktive Report- oder Audittexte duerfen erst nach eigener semantischer Bewertung umbenannt werden. r239 schliesst nur die produktive Rust-Identifier-Identitaet.

Keine Musik-, MIDI/WAV-, MusicXML-, Solver-, Writer-, Renderer-, Layout-, Report-, PDF-, Repair-, Recovery-, Selection- oder Dependency-Semantik wird veraendert.
