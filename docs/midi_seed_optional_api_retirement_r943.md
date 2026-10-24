# r943 MidiSeedOptionalApiRetirement

## Parent

Autoritativer Parent ist der real dunkelgruene r942 `GlobalSystemReflowReleaseShadowRefactorClosure`.

## Ziel

Die historisch als optionale oeffentliche Seed-API vorbereitete Wrapper-Kette in `src/midi_seed.rs` wird physisch entfernt, nachdem der aktuelle Repository-Stand beweist, dass sie weder extern exportiert noch intern produktiv aufgerufen wird.

## Reachability- und API-Beweis

`src/lib.rs` exportiert `midi_seed` nicht. Das Modul wird nur im Binary-Entrypoint `src/main.rs` als privates Modul eingebunden. Ein `pub fn` innerhalb dieses privaten Moduls bildet daher keine von externen Crates erreichbare Library-API.

Der aktuelle Rust-Call-Graph zeigt:

- `midi_to_musicxml_seed` besitzt ausser seiner Definition nur den kuenstlichen Function-Item-Keepalive `_midi_seed_optional_api_keepalive`;
- `midi_to_musicxml_seed_with_profile` wird ausschliesslich vom vorgenannten toten Basis-Wrapper aufgerufen;
- `midi_to_musicxml_seed_with_profile_and_hotspots` besitzt reale produktive Caller und bleibt unveraendert bestehen;
- `MidiSeedStats`, `CandidateProfile`, `HotspotPlan`, Solver-, Selection-, Writer- und MusicXML-Pfade bleiben unveraendert.

Die historischen r238/r239-Dokumente bleiben absichtlich als Historie erhalten. Ihre damalige Bezeichnung als oeffentliche Seed-API wird nicht rueckwirkend umgeschrieben; der aktuelle Exportvertrag wird durch `src/lib.rs` bestimmt.

## Schnitt

Physisch entfernt werden ausschliesslich:

- `midi_to_musicxml_seed`;
- `midi_to_musicxml_seed_with_profile`;
- `_midi_seed_optional_api_keepalive`.

Nicht entfernt oder veraendert werden:

- `midi_to_musicxml_seed_with_profile_and_hotspots`;
- `midi_seed_solver_profiles`;
- Standard-MIDI-Reading;
- CandidateProfile-/Hotspot-Semantik;
- Source Selection;
- Hand-/Trajectory-Solver;
- Writer-Roots;
- MusicXML/PDF/Renderer/Release.

## ChangeClass

`RefactorOnly`.

Der finale Diff entfernt tote Music-domain Function-/WriterReachability-Oberflaechen aus einer bestehenden produktiven Source-Datei, ohne neue Capability, neue Authority oder neue Musiksemantik einzufuehren. Die Klasse folgt damit dem realen finalen Semantic-Capability-Diff und wird nicht als `AuditOnly` vorweggenommen.

## Erwarteter Beweis

Tree Integrity, Change-Control, Semantic Capability Diff, Strict Clippy, Build/All-Targets und ein realer Musiklauf muessen gruen bleiben. Im finalen Rust-Baum duerfen `midi_to_musicxml_seed`, `midi_to_musicxml_seed_with_profile` und `_midi_seed_optional_api_keepalive` nicht mehr existieren; der produktive `midi_to_musicxml_seed_with_profile_and_hotspots` muss weiterhin erreichbar und im realen Lauf aktiv sein.
