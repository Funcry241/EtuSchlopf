# Vergleich der Windows-Notenbilder r1961 und r1965

Beide Windows-Vollkorpusläufe erzeugten für dieselben 14 Stücke jeweils 57 PDF-Seiten. Bei zwölf Stücken stimmen Master-MusicXML und Render-MusicXML bytegenau überein. Im Rastervergleich stimmen 53 der 57 Seiten pixelgenau überein. Sichtbare Änderungen liegen auf den beiden Mozart-Seiten und den beiden Schubert-Seiten.

| Stück | Takte | PDF-Seiten | Master-MusicXML | Render-MusicXML | Sichtbar geänderte Seiten |
| --- | ---: | ---: | --- | --- | --- |
| Bach · Fuge c-Moll BWV 847 | 31 | 3 | bytegleich | bytegleich | – |
| Chopin · Etüde op. 10 Nr. 3, p01 | 22 | 1 | bytegleich | bytegleich | – |
| Etüde Nr. 1 | 18 | 3 | bytegleich | bytegleich | – |
| Etüde Nr. 2 | 31 | 5 | bytegleich | bytegleich | – |
| Etüde Nr. 3 | 29 | 4 | bytegleich | bytegleich | – |
| Etüde Nr. 4 | 31 | 3 | bytegleich | bytegleich | – |
| Etüde Nr. 5 | 51 | 6 | bytegleich | bytegleich | – |
| Etüde Nr. 6 | 44 | 6 | bytegleich | bytegleich | – |
| Etüde Nr. 7 | 48 | 6 | bytegleich | bytegleich | – |
| Etüde Nr. 8 | 47 | 6 | bytegleich | bytegleich | – |
| Etüde Nr. 9 | 38 | 4 | bytegleich | bytegleich | – |
| Etüde Nr. 10 | 46 | 6 | bytegleich | bytegleich | – |
| Mozart · Sonate KV 331, p01 | 36 | 2 | geändert | geändert | 1–2 |
| Schubert · Deutscher Tanz D. 783 Nr. 15, p18 (Import) | 33 | 2 | geändert | geändert | 1–2 |

## Notierte Einsätze, Tonhöhen und Dauern

Jede reguläre Referenznote wird über ihren geschriebenen Einsatz und ihre klingende Tonhöhe identifiziert. Diese Kombination ist für die drei Vergleichsstücke eindeutig. Der Abgleich verbindet unmittelbar anschließende Haltefragmente derselben Stimme und zählt die vollständige Dauer. Vorschläge werden getrennt behandelt. Die Werte gelten jeweils für Master und Render.

| Stück | Passende Einsätze und Tonhöhen, beide Revisionen | Passende Dauern r1961 | Passende Dauern r1965 | Korrigierte Dauern | Zuvor passende, jetzt abweichende Dauern |
| --- | ---: | ---: | ---: | ---: | ---: |
| Mozart KV 331 | 474/474 | 438 | 460 | 24 | 2 |
| Schubert D. 783 Nr. 15, Import | 315/315 | 303 | 315 | 12 | 0 |
| Chopin op. 10 Nr. 3 | 448/448 | 380 | 380 | 0 | 0 |

Bei Mozart bleiben 14 Dauern offen. Zwei davon betreffen das A3 (MIDI 57) auf der vierten Achtelposition in Takt 18 und Takt 28. r1965 schreibt dort eine punktierte Viertelnote, die Referenz und r1961 schreiben eine Achtelnote. Dieselben beiden Stellen sind auch im Windows-Einzellauf r1964 enthalten. Bei Chopin weichen weiterhin 68 Dauern ab. Der Schubert-Import verwendet die bereits im historischen Projekt-MIDI notenspezifisch eingetragenen Entscheidungen. Die 315 verglichenen regulären Anschläge ergänzen dort acht Vorschläge. Fünf in der Aufführung ausgelassene Partiturnoten und sieben zusätzliche Anschläge sind im Referenzkorpus gesondert markiert.

Die vergleichende Auswertung verwendet für Mozart die vorhandene, an das Original-MIDI gebundene Referenzzuordnung aus der Projektabnahme, für Schubert die Originaldatei `Schubert_D783_no15_p18.match` und für Chopin die an die Aufführungs-MIDI gebundene Projektzuordnung aus dem aktualisierten Korpus der JKU. Der Vergleich liest die fertigen MusicXML-Ausgaben. [Herkunft der Einspielungen und Zuordnungen](../../examples/README.md#quellen-und-rechte) · [Nachweis je Referenz und Takt](compare-reference.tsv)

## Welche Takte und Seiten sich geändert haben

Die Taktzahlen zählen die MusicXML-Takte ab dem ersten, einschließlich Auftakt. Die zweite Spalte erfasst jede Byteänderung innerhalb eines Taktelements, auch Pausen, Stimmenbelegung und Notensatz. Die dritte Spalte erfasst nur Änderungen der vollständigen Dauer eines regulären Anschlags. Änderungen einer gebundenen Note sind dem Takt ihres Anschlags zugeordnet.

| Stück | Geänderte Takte in Master und Render | Anschlagtakte mit geänderten Dauern |
| --- | --- | --- |
| Mozart | 1–2, 4–6, 8–10, 12–14, 16–22, 24, 26–32, 34, 36 | 8, 16–20, 26–30, 36 |
| Schubert | 1–6, 8–14, 17–21, 23, 25–29, 31, 33 | 8–9, 17, 23, 25, 31, 33 |

| Werk und Seite | Veränderte Pixel | Anteil der Seite |
| --- | ---: | ---: |
| Mozart, Seite 1 | 73.203 | 3,65 % |
| Mozart, Seite 2 | 223.815 | 11,17 % |
| Schubert, Seite 1 | 126.563 | 6,32 % |
| Schubert, Seite 2 | 87.783 | 4,38 % |

**Gegenprobe mit dem Windows-Einzellauf r1964.** Bei Mozart stimmen Master und Render von r1964 und r1965 jeweils als vollständige Datei überein. Beide PDF-Seiten sind im selben Raster pixelgleich. [Seitenvergleich r1964 zu r1965](compare-mozart-r1964-pages.tsv)

## Messverfahren und Herkunft

Jede PDF-Seite wurde mit PyMuPDF 1.26.6 in RGB bei 144 dpi ohne Alphakanal gerastert. Seitengröße und Rasterabmessungen wurden vor dem Vergleich geprüft. Ein Bildpunkt gilt als verändert, sobald mindestens einer seiner drei Farbkanäle abweicht. Die vier veränderten Seiten messen jeweils 1.190 × 1.684 Pixel. Alle 14 PDF-Dateien unterscheiden sich auf Byteebene zwischen den Vollkorpusläufen. Der RGB-Vergleich trennt sichtbare Änderungen von Unterschieden im PDF-Dateiformat. [SHA-256 je Ausgabedatei und geänderte Takte](compare-scores.tsv) · [SHA-256 der RGB-Raster und Zahl abweichender Pixel je Seite](compare-pages.tsv)

| Windows-Export | SHA-256 |
| --- | --- |
| r1961, Vollkorpus | `6e5bcc8977e5cbe48cde1723989eb52e79ccc460bf636e92c88bb622422972a4` |
| r1964, Mozart-Einzellauf | `328ebab979b53afb5d180c4c114b03e14b118b92009ffc6118cffbb013af3b65` |
| r1965, Vollkorpus | `938c8c947f81bff5c86222ebe322961be635714a0616ddd967014d6051710ed6` |
