# Windows-Abnahme r1961

EtuSchlopf **r1961 / 0.10.1839** hat am 20. November 2026 alle 14 Stücke des Vollkorpus verarbeitet. Der Lauf dauerte **9 Minuten 39 Sekunden**, endete mit Exitcode 0 und lieferte **57 PDF-Seiten**. Zu jedem Stück liegen das Master-MusicXML, das Render-MusicXML, das PDF und der Ergebnisbericht vor.

Die Windows-Prüfung mit Rust 1.98.0 meldete **3.145 bestandene und vier ignorierte Tests**. Der Vollkorpuslauf verwendete die zum Quellstand und zur ausführbaren Datei gehörenden Prüfnachweise für alle Testziele und für Strict Clippy. Der Clippy-Nachweis weist null Warnungen aus.

## Gegenprobe zur Stimmzuordnung

Die Diagnose liest jedes Paar benachbarter Noten einer ursprünglich belegten Primärbalkengruppe im erzeugten Master zurück. Die Tabelle zählt die dort verbundenen Paare, die auf verschiedene Stimmen verteilten Paare und die übrigen Rückleseergebnisse. Der Lauf erzeugte für jedes aufgeführte Stück sämtliche Ausgaben. [Mathematische Auswertung](../../MATHEMATIK-VOLLKORPUS.md#stimmfortsetzungen-im-vollkorpus-r1961) · [Notengalerie](../../examples/README.md#notenbilder)

| Stück | PDF-Seiten | Verbunden | Auf verschiedene Stimmen verteilt | Übrige |
| --- | ---: | ---: | ---: | ---: |
| Bach · Fuge c-Moll BWV 847 | 3 | 408 | 19 | 0 |
| Chopin · Etüde E-Dur op. 10 Nr. 3 | 1 | 202 | 7 | 1 |
| Mozart · Klaviersonate A-Dur KV 331, erster Satz | 2 | 90 | 0 | 0 |
| Schubert · Deutscher Tanz As-Dur D. 783 Nr. 15 | 2 | 12 | 0 | 0 |
| Etüde Nr. 1 | 3 | 216 | 1 | 0 |
| Etüde Nr. 2 | 5 | 426 | 23 | 0 |
| Etüde Nr. 3 | 4 | 344 | 12 | 0 |
| Etüde Nr. 4 | 3 | 360 | 34 | 0 |
| Etüde Nr. 5 | 6 | 653 | 37 | 0 |
| Etüde Nr. 6 | 6 | 616 | 46 | 1 |
| Etüde Nr. 7 | 6 | 363 | 226 | 0 |
| Etüde Nr. 8 | 6 | 524 | 38 | 0 |
| Etüde Nr. 9 | 4 | 505 | 34 | 0 |
| Etüde Nr. 10 | 6 | 510 | 43 | 0 |
| **Gesamt** | **57** | **5.229** | **520** | **2** |

Für vier der 520 getrennten Paare fand die Diagnose jeweils eine zulässige alternative Belegung. Alle vier liegen in Bachs Fuge. Bei jeder einzeln geprüften Alternative waren das jeweilige Zielpaar und alle 408 zuvor verbundenen Paare im Vergleichsmaster verbunden. Die ausgegebenen Notenbilder zeigen den regulären Stimmplan. Für 23 Zielpaare schloss die vollständige Suche einen gemeinsamen Kanal unter den geprüften Bedingungen aus. Bei 493 blieb die Frage offen.

## Quellstand und Prüfsummen

| Gegenstand | SHA-256 |
| --- | --- |
| Angenommenes Take-Paket | `fbe94e63f1d72b96de698069047e8649fbe7990dda5e38a0fc762c5946fbd691` |
| Im Windows-Export eingebettetes Quellarchiv | `a7e66447af5062461b66e914047bea700ce44dab204bdd7e5f410663accb70ef` |

Der [Annahmebeleg](native_acceptance.txt) identifiziert das geprüfte Take-Paket. Der Build gleicht die 2.213 Dateien des angenommenen Quellstands mit den [Quellprüfsummen](accepted_source_sha256.tsv) ab. Die [Dokumentationsprüfsummen](github_projection_sha256.tsv) erfassen die für GitHub angepassten Fassungen. Die [Prüfsummen der Notenbilder](score_sha256.tsv) binden alle 14 PDFs und ihre Vorschaubilder. [Quellen und Namensnennungen](score_credits.md)
