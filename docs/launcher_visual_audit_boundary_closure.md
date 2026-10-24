# LauncherVisualAuditBoundaryClosure

## Ziel

Dieser `SecurityOnly`-Take ist Schritt 3 und Abschluss des vereinbarten
Launcher-/Build-/Audit-Superwurfs. Er trennt den historischen internen
Visual-Audit-Kern physisch aus `src/bin/etuschlopf_launcher.rs` in den privaten
Include-Shard `src/bin/etuschlopf_launcher/launcher_visual_audit.rs`.

## Grenze

Im Launcher verbleiben bewusst die zwei sibling-uebergreifend verwendeten
Datenvertraege `VisualProjectRunLedger` und `VisualPreflightPlan`. Sie werden von
`assistant_export.rs` beziehungsweise dem typisierten `visual_preflight`-Modul
direkt verwendet und bleiben deshalb am bisherigen Parent-Ownership-Ort.

Ausgelagert werden ausschliesslich bereits vorhandene Visual-Preflight-Mechaniken:

- Projektbatch-Ausfuehrung und Freshness-Ledger
- allgemeine und Clef-bezogene Visual-Audits
- Finalisierung und Assistant-Export-Anbindung der Visual-Ausfuehrung
- PDF-Datei-/Content-Fingerprints und Auto-Visual-Proxys
- Clef-/Register-Parsing, Findings und Aggregatprojektion
- Corpus-Emergence-Auswertung und Visual-Hotspot-Planung
- zugehoerige lokale Parsing-/Format-Hilfen

Die ausgelagerten Items bleiben textlich unveraendert. `include!` behaelt denselben
Rust-Modul- und Privacy-Namensraum. Dispatch, Argumentmodell, `visual_preflight.rs`
als reale typisierte Orchestrierungsgrenze, Build-/Reuse-Boundaries, Reports,
Release und Musik bleiben unveraendert.

## Cross-File-Vertraege

`VisualProjectRunLedger` und `VisualPreflightPlan` bleiben mit exakt denselben
Feldern im Launcher. Dadurch entstehen keine entfernten Struct-Felder fuer
`assistant_export.rs` oder `src/visual_preflight.rs`.

Ein bestehender Quelltext-Test in `src/visual_preflight.rs` betrachtet nach dem
physischen Schnitt Launcher plus Visual-Audit-Shard; seine fachlichen Assertions
bleiben unveraendert. Der historische Accepted-Closure-Test liest direkt den
neuen Shard.

## Nicht geaendert

- keine Visual-Preflight-Entscheidung
- keine Freshness-, Renderer- oder Clef/Register-Regel
- keine PDF-Fingerprint- oder Emergence-Logik
- keine Run-/Corpus-/Take-Reihenfolge
- keine Build-, Cleanup- oder Assistant-Export-Semantik
- keine Report-/Release-Semantik
- keine Musiklogik, Writer, Schwellenwerte oder Artefaktpfade

## Erfolg

- der produktive Visual-Audit-Kern ist physisch aus dem Launcher entfernt
- die zwei Cross-File-Datenvertraege bleiben unveraendert am Parent
- alle ausgelagerten Items sind gegen 0.10.1682 textidentisch
- Security-Gate-Launcher-Marker und Pin-Kette bleiben geschlossen
- Etuede 4 bleibt in normalisierter Master-/Render-MusicXML und PDF pixelidentisch

## Stop

Mit einem gruenen Etuede-4-Artefaktbeweis endet der dreiteilige
Launcher-/Build-/Audit-Superwurf. Kein Take 4 nur wegen weiterer Launcher- oder
Clippy-Kosmetik.
