# r212 – FlpFinalLauncherHygieneSecurityRetirement

## Ziel

Nach r211 existiert keine produktive, auditierende oder werkzeugseitige FLP-Dateilesung mehr. Dieser isolierte `SecurityOnly`-Schnitt entfernt die letzten aktiven `.flp`-Sonderbehandlungen aus Launcher-Hygiene und Assistentenexport.

## Aenderungen

- Root-Hygiene schuetzt nur noch `.wav` und `.mid` als Projekteingaben.
- Der Assistentenexport klassifiziert nur noch WAV/MIDI als rohe Projektinputs.
- Die pensionierte FLP-Zeile wird aus der Export-Policy entfernt.
- Die Policy-Texte nennen ausschliesslich den aktuellen WAV/MIDI-Vertrag.
- `pinned_policy_security.rs`, `security_build_orchestrator.rs` und `identity_gate_trust_anchor.rs` werden als koordinierte Pin-Eigentuemer synchronisiert.

## Nicht geaendert

- produktiver MIDI-Decoder und Source-Event-Vertrag
- Solver, Handverteilung und Kandidatenbewertung
- Writer-/StructuralWriteAuthority
- WAV/FrozenLegacySourceOracle
- externes WAV-Pitch-Sensorwerkzeug
- Renderer, PDF und MusicXML
- Repair-/Recovery-Autoritaet

## Abschlussbedingung

Nach gruenem r212 duerfen die alten `.flp`-Dateien aus dem Projektordner entfernt werden. Anschliessend muss `run.cmd --all` mit ausschliesslich WAV + MIDI den bekannten 10/10-Korpusstand reproduzieren.
