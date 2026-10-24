# LauncherHygieneBoundaryClosure

## Ziel

Dieser SecurityOnly-Take trennt ausschließlich den bereits zusammenhängenden Launcher-Hygiene-Funktionsblock physisch aus `src/bin/etuschlopf_launcher.rs` aus. Der neue Shard `src/bin/etuschlopf_launcher/launcher_hygiene.rs` wird per `include!` im selben Rust-Modul- und Privacy-Namensraum eingebunden.

## Grenze

Verschoben werden vollständig und in unveränderter Reihenfolge alle Funktionen von `cleanup_program_temp_leftovers()` bis einschließlich `remove_obsolete_powershell_files()`. Die bestehenden Hygiene-Datenverträge (`TempCleanupReport`, `RootCleanupReport`, `BuildCacheHygieneReport`, `CachePathOutcome` usw.) bleiben bewusst im Launcher. Damit entstehen keine neuen Cross-File-Struct-/Visibility-Verträge.

Der einzige Test-Quelltextbezug wird so erweitert, dass er für seinen statischen Vertrag Launcher und Hygiene-Shard gemeinsam betrachtet. Produktionsausdrücke des Tests und sämtliche Runtime-Aufrufstellen bleiben unverändert.

## Nicht geändert

- keine Cleanup-Regel oder Pfadklassifikation
- keine Build-/Clippy-/Compile-Logik
- keine Run-/Corpus-/Visual-Preflight-Reihenfolge
- keine Take-/Handover-Logik
- keine Console-/Report-/Release-Semantik
- keine Musiklogik oder Artefaktpfade

## Statischer Beweis vor Paketierung

- der ausgelagerte Hygiene-Block ist byteidentisch zum entsprechenden v0.10.1680-Block
- beim Ersetzen des `include!` durch den Shard-Inhalt entsteht der ursprüngliche Launcher bis auf den bewusst erweiterten statischen Test wieder
- alle verschobenen Hygiene-Funktionsnamen existieren genau einmal als Definition
- die Aufrufreihenfolge in `run()` bleibt unverändert
- Change-Manifest und FNV64 werden vor Paketierung vollständig geprüft

## Stop

Mit einem grünen Etüde-4-Artefaktbeweis endet Take 1. Build-Control-Plane wird in diesem Take nicht begonnen.


## r02 – synchronisierte Architektur-Pins

Der erste r01-Versuch wurde vor Apply abgelehnt: `src/bin/etuschlopf_launcher.rs` ist ein gepinnter Architekturpfad und darf nur gemeinsam mit `src/identity_debt_build.rs` und `src/identity_gate_trust_anchor.rs` geaendert werden. r02 synchronisiert deshalb ausschliesslich diese Pin-Eigentuemer auf den bereits unveraenderten Hygiene-Schnitt. Keine Cleanup-, Build-, Run-, Visual-, Take-, Report-, Release- oder Musiksemantik wurde gegenueber r01 geaendert.
