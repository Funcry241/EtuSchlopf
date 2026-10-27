# Dokumentierte Abnahme r1533

Auswertung vom 27.10.2026, Paketversion 0.10.1839. Angenommener Parent und
Ergebnisvergleich: r1532. Vorheriger öffentlicher Git-Quellstand: r1526.

| Prüfung | Ergebnis |
| --- | --- |
| Native Annahme | r1533 angenommen; Paket- und Acceptance-Hashes geprüft |
| Tests | 2.564 bestanden, 0 fehlgeschlagen, 1 ignoriert; 15 Testprogramme |
| Build / Clippy / finaler Exit | jeweils 0 |
| Privater Zehnerkorpus | 10/10 vollständig; Renderer bestanden; dunkelgrün |
| Quell-Readback | alle 1.868 Take-Dateien bytegleich im nativen Snapshot; zusätzlich Root-Cargo.lock |
| XML gegen r1532 | zehn Master- und zehn Render-Dateien bytegleich |
| PDF gegen r1532 | alle zehn Dateibytes verschieden; alle 50 Seiten mit PDFium bei 144 dpi pixelgleich |

## Native Auszüge und Auswertung

- [Acceptance](native_acceptance.txt)
- [Finaler Lifecycle-Handover](native_lifecycle_handover.txt)
- [RunOutcome](native_run_outcome.txt)
- [Zusammenfassung mit Identitäten und Prüfgrenzen](summary.json)
- [Runtime-/Tool-/Test-Hashinventar](accepted_runtime_sha256.tsv)

Die nativen Auszüge bleiben unverändert. Zusammenfassung und Hashinventar sind
redaktionelle read-only Auswertungen, keine zusätzliche Runtime-Autorität.

## Veröffentlichung und Grenzen

Der öffentliche Quellstand wird von r1526 auf den angenommenen r1533-Stand
aktualisiert. Die dazwischen angenommenen Quellen enthalten unter anderem frühe
Eingabeprüfungen, Renderer-Startabfrage, Prüfungen benötigter Renderprodukte und
die Begrenzung optionaler PDF-Nachweise. r1533 selbst ergänzt nur Community-
Dokumentation und Vorlagen. Die jeweiligen Take-Beschreibungen bleiben historische
Beschreibungen ihres damaligen Prüfstands.

Alle aus dem Snapshot übernommenen Runtime-, Test- und Tooldateien sind bytegleich
zum angenommenen Stand. Die öffentliche Download-README bleibt erhalten; die
technische Take-README wird in die technische Referenz projiziert. Community-
Links werden dafür angepasst, Issue-Vorlagen nach `.github/ISSUE_TEMPLATE/`
projiziert und der bestehende Windows-Workflow auf dieses Hashinventar gebunden.
Diese Git-spezifischen Anpassungen sind keine nachträgliche Änderung des Takes.

Der Vergleich zu r1532 belegt keine unveränderte Notation seit r1526 und keine
neue unabhängige musikalische Bewertung. Private Korpuseingaben und vollständige
musikalische Ausgaben werden nicht veröffentlicht. Die bereits veröffentlichte
Windows-EXE bleibt die Testversion r1526; hier wird keine neue EXE freigegeben.
Eine eigenständige Anwenderabnahme auf sauberem Windows und die Bewertung fremder
Stücke werden durch den erfolgreichen Entwicklungskorpus nicht ersetzt.
