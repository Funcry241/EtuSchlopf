# r240 MidiSourceEvidenceVocabularyClosure

ChangeClass: RefactorOnly.

## Elternstand

r240 basiert direkt auf dem real dunkelgruenen r239 `MidiSeedOwnerLocalIdentityClosure`. Der r239-Assistant-Export weist `FinalExit 0`, `BuildExit 0`, `ClippyExit 0` und gruene integrierte Launcher-Tests aus. Master- und Render-MusicXML sind gegenueber r238 byteidentisch; das PDF ist gegenueber r238 auf allen 6 Seiten pixelidentisch.

## Ziel

r239 hat die produktive Rust-Identifier-Identitaet geschlossen: In lebenden Rust-Identifiern existiert kein `flp` mehr. r240 bereinigt ausschliesslich heute noch aktive oder fuer aktuelle Wartung irrefuehrende FLP-Vokabeln in Rust-Kommentaren, Fehler-/Audittexten und einer internen Kategoriebezeichnung. Historische Retirement-, Security- und Migrationsaussagen bleiben unveraendert.

Die aktuelle Quellen-/Evidenztrennung wird sprachlich explizit gemacht:

- Standard-MIDI ist Quellmaterial bzw. Seed-Quelle.
- WAV bleibt Referenz-/Evidenzkanal.
- FLP ist historische Identitaet und wird nur dort beibehalten, wo gerade diese Historie beschrieben wird.

## Aenderungen

- `src/arrangement.rs`: `WAV/FLP` in der aktuellen `ArrangementProfile`-Dokumentation wird zu `MIDI-Quellmaterial bzw. WAV-Referenzevidenz`.
- `src/pipeline_shards/staff_voice_move_restify_atomic_patch_tail.rs`: die bestehende Kategorie `Motivwiedererkennung gegenueber WAV/FLP` wird in beiden exakt gekoppelten Vorkommen zu `Motivwiedererkennung gegenueber MIDI-Quelle und WAV-Evidenz`. Der Score-Zweig bleibt derselbe und behaelt unveraendert den Wert 120.
- `src/pipeline_shards/corpus_fastjump_human_potential_core.rs`: aktueller Seed-Kommentar und Parse-Kontext sprechen von `MIDI-Seed` bzw. `MIDI-Notationsbasis`.
- `src/pipeline_shards/human_pdf_pianistic_quality_core.rs`: die audit-only Reattack-Empfehlung benennt MIDI-Quelle und WAV-Evidenz statt WAV/FLP. Die Entscheidung bleibt audit-only und schreibt weiterhin nichts.
- `src/midi_seed.rs`: ein inzwischen sachlich falscher Kommentar ueber den angeblich erhaltenen Modulpfad `flp` wird zu einer historischen Herkunftsaussage; der aktuelle Rohmaterial-Kommentar benennt `MIDI-Seed-Material`.

## Invarianten

- keine Identifier-, Typ-, Funktions-, Modul- oder API-Namen werden geaendert;
- keine Schwellenwerte, Scores, Bedingungen oder Aufrufpfade werden geaendert;
- die Kategorie im Composer-Substance-Score wird an Erzeugung und Match exakt gleich umbenannt, der zugeordnete Score bleibt `120`;
- keine MusicXML-, MIDI-, WAV-, Writer-, Renderer-, Layout-, Repair-, Recovery- oder Selection-Semantik wird geaendert;
- historische FLP-Belege werden nicht umgeschrieben.

## Bewusst offen

`Cargo.toml` enthaelt weiterhin die historische package description `FLP/WAV-Projektpipeline: Klavier-PDF aus Projektname`. Dieser Take fasst `Cargo.toml` absichtlich nicht an: Der bestehende Take-Change-Control-Vertrag erlaubt dort ausser eng definierten Security-Migrationen nur Versionsaenderungen. Fuer eine reine Metadatenbezeichnung wird keine Security-Grenze aufgeweicht.
