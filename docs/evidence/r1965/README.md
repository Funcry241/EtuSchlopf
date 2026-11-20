# Windows-Abnahme r1965

**EtuSchlopf 0.10.1839 r1965** verarbeitete am 20. November 2026 unter Windows alle 14 Stücke des Vollkorpus. Der Lauf erzeugte **14 PDFs mit 57 Seiten** und endete mit Exitcode 0. Alle 14 Projektberichte melden null Blocker, Warnungen und Hinweise. [14 Notenbilder mit PDFs](../../examples/README.md#notenbilder) · [Vergleich mit den bisherigen Ausgaben](vergleich.md)

## Umfang und Laufzeit

Für jedes Stück entfernte EtuSchlopf die alten Ausgaben und erzeugte Master-MusicXML, Render-MusicXML, PDF und Ergebnisbericht neu. Für alle 14 Stücke ist der Renderer-Lauf belegt. Größe und Prüfsumme aller 14 Projektberichte sowie des Korpusberichts stimmen mit dem Laufmanifest überein. [Prüfsummen der PDFs und Ausschnitte](score_sha256.tsv)

| Messung | Ergebnis |
| --- | ---: |
| Gesamtlauf des Launchers bis zur Diagnose vor dem Export | 9 min 26,660 s |
| Davon Programmlauf | 9 min 24,967 s |
| Vollständige Projekte mit Exitcode 0 | 14 von 14 |
| Seiten der vier Referenzstücke Bach, Chopin, Mozart und Schubert | 3 + 1 + 2 + 2 |
| Seiten der zehn Etüden | 49 |

Der Launcher verwendete die bereits geprüfte Programmdatei und die passenden Windows-Prüfnachweise für alle Strict-Testziele und für Clippy mit Rust 1.98.0. Der Clippy-Nachweis weist null Warnungen aus. Der Testnachweis bestätigt die Wiederverwendung für diesen Quellstand und dieses Programm. Der Korpusexport enthält keine Einzelzählung bestandener oder ignorierter Tests.

## Erhaltene Quellnoten

Die Quellauswahl übernahm insgesamt **12.143 von 12.181 MIDI-Quellereignissen** (99,69 %). Bei Bach BWV 847, Chopin op. 10 Nr. 3 und den Etüden 1 bis 3 übernahm sie alle Ereignisse. Bei Mozart übernahm sie 478 von 479 und bei Schubert 323 von 330. Diese Zählung erfolgt vor dem Arrangement. Der [Notenvergleich](vergleich.md) untersucht die tatsächlich ausgegebenen Noten.

## Angenommener Quellstand

Der letzte Eintrag im exportierten Annahmeverlauf weist r1965 als angenommen aus und bindet das Take-Paket an die unten genannte Prüfsumme. Das im Korpus-Export enthaltene Quellarchiv stimmt in allen 2.218 Dateien mit dem Take überein und enthält zusätzlich die beim Windows-Build verwendete `Cargo.lock`. [Annahmenachweis](native_acceptance.txt)

| Gegenstand | SHA-256 |
| --- | --- |
| Angenommenes Take-Paket | `568cd3fca2fd302c5c66daf797f2c1e8b670a713241f1daee45098917b436386` |
| Quellarchiv aus dem Windows-Export | `f26df127c4a4b17e3631e816ef70d4b7a88c207eb2766c465dce650d2b3cb36e` |
