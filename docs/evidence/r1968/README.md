# Windows-Abnahme und Notenvergleich r1968

EtuSchlopf **0.10.1839 r1968** erzeugte am 21. November 2026 unter Windows alle **14 Partituren mit 57 PDF-Seiten** neu. Gegenüber dem Windows-Vollkorpus r1965 sind alle 14 Master-MusicXML und alle 14 Render-MusicXML jeweils bytegleich. Im direkten Vergleich der Seitenraster bei 144 dpi unterscheiden sich **0 von 114.225.720 Pixeln**. Die PDF-Dateien haben jeweils einen anderen SHA-256. Bei allen 14 änderten sich die PDF-Metadaten für Erstellung und letzte Änderung. [Vergleich je Stück](compare-scores.tsv) · [Vergleich je PDF-Seite](compare-pages.tsv) · [Windows-Vollkorpus r1965](../r1965/README.md)

## Windows-Vollkorpus vom 21. November

Der Launcher löschte die bisherigen Ausgaben und erzeugte für jedes Stück Master-MusicXML, Render-MusicXML, PDF und Ergebnisbericht. Alle 14 Projektläufe endeten mit Exitcode 0 und bestätigten den Renderer-Aufruf. Jeder Projektbericht meldet **0 Blocker, 0 Warnungen und 0 Hinweise**. Dateigröße und FNV-1a-Prüfsumme aller Projektberichte und des Korpusberichts stimmen mit dem Laufmanifest überein. Für jedes PDF stimmen außerdem Größe und SHA-256 mit seinem Projektbericht überein. Die Quellauswahl übernahm 12.143 von 12.181 MIDI-Quellereignissen (99,69 %). [Prüfwerte je Stück](corpus-certificates.tsv)

| Messung | Ergebnis |
| --- | --- |
| Korpuslauf-Kennung | `corpus-0.10.1839-1795251037-21216` |
| Manifestbeginn und -abschluss | 21. November 2026, 08:50:37 bis 09:00:22 UTC |
| Programmlauf für alle 14 Projekte | 9 min 47,582 s |
| Launcher einschließlich Strict-Prüfung bis zur Diagnose vor dem Export | 15 min 42,283 s |
| Frisch ausgeführte Tests | 3.142 bestanden, 0 fehlgeschlagen, 4 ignoriert, verteilt auf 15 Testprogramme |
| Build und Clippy | Beide Exitcode 0, Clippy mit 0 Warnungen |
| SHA-256 des Windows-Exports | `afaefae2a650cba5b8a3d80b33509551c8fdaeac2c0e62bf56772ef335c112a3` |
| SHA-256 des exportierten Laufmanifests | `c45009473de833b7daece79e2f97fa9d3221277a322dc36dc2fc9f29af942bd8` |
| SHA-256 des Quellarchivs aus dem Lauf | `66834b8b441a53a31d9e69305445338605188e2cebf273edb459d5dffd747b25` |

Der Annahmeverlauf im Export reicht bis zum angenommenen r1967-Quellpaket. Der anschließend geschriebene Annahmebeleg bestätigt r1968 mit derselben Paket-Prüfsumme wie die Windows-Prüfung vom 20. November. Er ist über Größe und SHA-256 an das Übergabeprotokoll gebunden. Das Quellarchiv aus dem Korpuslauf enthält alle 2.218 Dateien des angenommenen r1968-Pakets bytegenau und zusätzlich die unveränderte `Cargo.lock` des geprüften Vorgängers.

## Windows-Prüfung der drei Quellpakete

Der Windows-Launcher baute jedes Paket unter Windows Server 2022 mit Rust 1.98.0 für x64 und MSVC. Jeder Lauf bestand Clippy, meldete null Warnungen und verarbeitete das öffentliche Beispiel mit Exitcode 0. Die Prüfsumme identifiziert jeweils das angenommene Quell-ZIP. [Protokollauszüge und Prüfsummen der Testnachweise](windows-native-acceptances.json)

| Revision | Tests bestanden | Fehlgeschlagen | Ignoriert | SHA-256 des Quell-ZIPs | Laufprotokoll |
| --- | ---: | ---: | ---: | --- | --- |
| r1966 | 3.145 | 0 | 4 | `4caabe3937e9f7b19a3c3179b4470e5226e1a49e8e8b002e4c465ee32212cb13` | [Windows r1966](https://github.com/Funcry241/EtuSchlopf/actions/runs/44829339866/job/136610005679) |
| r1967 | 3.145 | 0 | 4 | `7aae836c235097222ffda9fdedc3daa0db3d9c093d004189d60738138948a38c` | [Windows r1967](https://github.com/Funcry241/EtuSchlopf/actions/runs/44829639317/job/136611015674) |
| r1968 | 3.142 | 0 | 4 | `2138a8f2ad1746367c443f5c7ac80dada47b48f04414f2f21a773bf4f7a11880` | [Windows r1968](https://github.com/Funcry241/EtuSchlopf/actions/runs/44831896224/job/136618623298) |

Die vier regulär übersprungenen Tests bestanden für r1968 in einem gesonderten Lauf unter Ubuntu 22.04. Drei prüfen Rechenwege für Arrangementvarianten, Registervorschau und Schlüsselplanung mit synthetischen Daten. Der vierte prüft mit MuseScore 4.7.5, ob das vorgesehene Balkenziel nach dem Öffnen und Speichern der Partitur sichtbar ist. [Zusatzlauf mit den vier Einzeltests](https://github.com/Funcry241/EtuSchlopf/actions/runs/44835599703/job/136631087988)

## Öffentliches Beispiel und Chopin unter Linux

Die vier per SHA-256 geprüften Linux-Programmdateien verarbeiteten jeweils dieselben zwei MIDI/WAV-Paare unter Ubuntu 22.04 mit MuseScore 4.7.5. Alle acht Läufe endeten mit Exitcode 0. In jedem der drei aufeinanderfolgenden Versionswechsel stimmten für beide Stücke die vollständigen Master-MusicXML- und Render-MusicXML-Dateien jeweils bytegenau überein. Die PDF-Seiten waren bei 144 dpi pixelgleich. [Messwerte und Prüfsummen je Lauf (JSONL)](public-score-comparison.jsonl) · [Laufprotokoll](https://github.com/Funcry241/EtuSchlopf/actions/runs/44834621386/job/136627764315)

| Öffentliches Prüfstück | Taktart | Takte | Reguläre Anschläge | Vorschlagsnoten | PDF-Seiten |
| --- | --- | ---: | ---: | ---: | ---: |
| EtuSchlopf-Beispiel | 4/4 | 8 | 40 | 0 | 1 |
| Chopin, Etüde op. 10 Nr. 3, Aufnahme p01 | 2/4 | 22 | 448 | 4 | 1 |

Die Eingaben stammen aus dem veröffentlichten Beispielpaket und aus Werner Goebls *Vienna 4×22 Piano Corpus* (mdw, DOI 10.21939/4X22). Für Chopin verwendete der Lauf die unveränderte Performance-MIDI und WAV der Aufnahme p01. [Beispielpaket r1965](https://github.com/Funcry241/EtuSchlopf/releases/download/v0.10.1839-r1965-windows-preview/EtuSchlopf-Beispiel-r1965.zip) · [Datensatz der mdw](https://datasets.mdw.ac.at/datasets/dataset/98ea25fa-2468-43ff-929b-3c926e163583)

Bei Chopin stimmen die beiden MusicXML-Prüfsummen auch mit der r1965-Referenzauswertung überein. Deren Ergebnis gilt damit für dieselben Noten unter r1968. Bei allen **448 regulären Anschlägen** treffen Einsatz und Tonhöhe die Referenz, bei **380 von 448** auch die notierte Dauer. Die vier Vorschlagsnoten sind gesondert gezählt. [Notenvergleich r1965](../r1965/vergleich.md#notierte-einsätze-tonhöhen-und-dauern) · [Dauern je Takt](../r1965/compare-reference.tsv)
