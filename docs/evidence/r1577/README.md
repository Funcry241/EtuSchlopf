# Dokumentierte Abnahme r1577

Stand 1. November 2026, Paketversion 0.10.1839. Der öffentliche Entwicklungsbaum
wird mit dem nativ angenommenen r1577 synchronisiert. Beim ursprünglichen
Quellabgleich blieb der Download r1537. Nach erfolgreicher Paketprüfung wurde
[r1577 als Windows-Testversion veröffentlicht](https://github.com/Funcry241/EtuSchlopf/releases/tag/v0.10.1839-r1577-windows-preview).
Die historische `summary.json` beschreibt den Stand vor dieser Veröffentlichung.

| Prüfung | Ergebnis |
|---|---|
| Native Annahme | r1577 angenommen; Take-SHA-256 und ausgeführter Quellstand verglichen |
| Quell-Readback | alle 1952 Take-Dateien bytegleich; zusätzlich lokale Root-Cargo.lock |
| Build / Clippy / finaler Exit | jeweils 0; Clippy ohne Warnungen |
| Tests | 2875 bestanden, 0 fehlgeschlagen, 3 ignoriert |
| Privater Zehnerkorpus | 10/10 erfolgreich und vollständig |
| MusicXML gegen r1576 | alle zehn Master- und zehn Render-Dateien bytegleich |
| C3 in E7/E8 | beide Systeme bis zum Abschluss gebunden; Schutzprüfung nach der Balken-Stimmenteilung grün |
| Windows-Paket des geprüften Quellbaums | Workflow #14 erfolgreich; interaktiver Anwenderlauf weiter offen |

## Belege und Prüfgrenzen

Die veröffentlichte EXE stammt unverändert aus
[Windows download #14](https://github.com/Funcry241/EtuSchlopf/actions/runs/41326070573),
Commit `55be4dc022739fed83c9d13a5e73d0cce03dfcb4`.
Quellprüfung, Windows-Engine-Build, drei Distributionstests, Version/Hilfe ohne
Entwickler-PATH und isolierter `--no-pdf`-Starttest sind bestanden. Die EXE hat
SHA-256 `fc413fbcefbfc80026f34ddaa591c7ce49e4f2b5daee3943d2085aa5350aac29`.
Das Quellarchiv im Release gehört zu diesem geprüften Commit. Die spätere
Aktualisierung der Download-Dokumentation verändert keine Runtime-Datei.

- [Native Acceptance-Metadaten](native_acceptance.txt), unverändert aus dem Export.
- [Nativer RunOutcome](native_run_outcome.txt), unverändert aus dem Export. Sein
  `ReleaseDecision` gehört zur lokalen Pipeline und veröffentlicht keine GitHub-EXE.
- [Zusammenfassung](summary.json) und [XML-Prüfsummenvergleich](xml_equality.json).
- [Inventar der nativen Runtime, Tests, Tools und benötigten Daten](accepted_runtime_sha256.tsv).
- [Native Take-Deklaration](native_take_change_class.txt), unveränderte Archivkopie.
- [Prüfsummen der Git-Dokumentation und Workflow-Projektion](github_projection_sha256.tsv).

Die Zusammenfassung und Inventare sind lesende Auswertungen, keine zusätzliche
Runtime-Autorität. Die vollständigen privaten Exporte, Eingaben und musikalischen
Ausgaben werden nicht veröffentlicht. Eine neue PDF-Bildabnahme wurde bei diesem
Performance-Vergleich nicht durchgeführt. Die Bytegleichheit der XML ersetzt
keine unabhängige musikalische Bewertung.

## Was r1577 beschleunigt

Der konkrete unveränderliche Register-Preview verwendet eine Bewertung nur bei
vollständig gleichem Vorschlag innerhalb derselben Eingabe erneut. Die Auswahl,
Suchreihenfolge, Kostenordnung, UNKNOWN-Behandlung und Schreiber bleiben gleich.

Nativ: 566 Anfragen, 89 exakte Treffer und 477 tatsächliche Berechnungen; kein
Speicherlimit erreicht. Die Master-Stufe summiert sich über zehn Projekte von
594,391 auf 532,224 Sekunden (−10,46 %). Die Korpusdauer laut Manifest steigt
gleichzeitig von 863 auf 874 Sekunden (14:23 auf 14:34). Beide Läufe verwenden
drei parallele Projekte, sechs Solver-Worker je Projekt und drei Renderer-Slots.
Stufensummen sind keine Gesamtlaufzeit. Ein Gewinn des Gesamtlaufs ist nicht belegt.

## C3 und Mathematik-Seite

r1576 übernimmt erstmals im hier untersuchten Integrationsstand beide Systeme
von E7/E8 und hält die Verläufe bis zum Abschluss gebunden. r1577 bewahrt deren
Musik bytegleich. Die anderen acht Etüden behalten bisherige Verläufe; C3 ist
noch keine vollständige Lösung des Korpus. Die eingefrorene lexikografische
Policy ist unverändert; das gesonderte Schlüsselziel bleibt verfehlt.

Die [Mathematik-Seite](../../MATHEMATIK.md) baut auf Dachs' über PR #6
übernommener r1562-Fassung auf. Die Formeln bleiben GitHub-tauglich. Neue Aussagen
unterscheiden produktive C3-Integration, begrenzte Layout-Teilumsetzungen und
reine Rechenwiederverwendung. Historische Zahlen behalten ihre Revision.

## Git-Projektion und Reproduktion

Alle nativen Rust-, Test- und Tooldateien sowie Laufzeitdaten bleiben bytegleich.
Die aktuelle Take-Deklaration wird nicht umgeschrieben. Der Windows-Workflow
prüft das native Inventar und das separate Git-Projektionsinventar, bevor er baut.
Die dritte Spalte des nativen Inventars nennt den zu prüfenden Repository-Pfad.

Öffentliche README, Beitragsregeln, Wegweiser, Dachs' Mathematik-Seite und bereits
anonymisierte Dokumentationspfade bleiben als Git-Dokumentation erhalten. Die
technische Referenz und Quellstandverweise werden auf r1577 aktualisiert.
GitHub-eigene Dateien, historische Belege, Lizenz und Abbildungen bleiben erhalten.
Ein Git-Quellarchiv ist deshalb nicht mit dem ursprünglichen Take-ZIP identisch.

Aus der Projektwurzel mit der Toolchain aus `rust-toolchain.toml`:

```text
cargo test --locked --profile strict --all-targets
cargo clippy --locked --profile strict -- -D clippy::correctness -D clippy::suspicious -D clippy::perf
```

Für einen vollständigen nativen Korpuslauf werden eigene geeignete Eingaben und
MuseScore 4 benötigt. Die Windows-Paketprüfung bleibt eine gesonderte Prüfung.
