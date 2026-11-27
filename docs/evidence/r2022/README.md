# Windows-Abnahme r2022

Der Windows-Vollkorpus von **EtuSchlopf 0.10.1839 r2022** endete mit Exitcode 0. Alle 15 Projekte liefen erfolgreich und erzeugten zusammen 58 PDF-Seiten. Im vollständigen Testlauf bestanden 3.236 Tests bei zehn übersprungenen Tests. Clippy Strict meldete null Warnungen. [Notenbilder](../../examples/README.md#notenbilder) · [PDF-Quellen](score-source.tsv) · [Prüfsummen](score-sha256.txt)

Alle 30 MusicXML-Dateien (Master und Render) sind bytegleich zu r2021. Alle 58 PDF-Seiten stimmen nach Rasterung mit 144 dpi in jedem RGB-Pixel überein. Die Galerie enthält die PDFs aus dem r2022-Lauf.

Die Oktavbewertung verwendet die vorhandene Zeitanalyse wieder. Beide Vollkorpusläufe verwendeten drei parallele Projekte mit sechs Workern je Projekt. Die Phasenzeit ist die Summe der Einzelmessungen über den Korpus.

| Messgröße | r2021 | r2022 |
| --- | ---: | ---: |
| Gesamtlaufzeit | 11:58 min | 10:41 min |
| Oktavbewertung, Summe über den Korpus | 313,076 s | 171,731 s |
| Oktavbewertung, Etüde 5 | 148,352 s | 78,611 s |
| Oktavbewertung, Etüde 6 | 55,475 s | 24,476 s |

Der installierte Quellsnapshot enthält 2.288 Dateien. Die [Bestandsliste](accepted-source.tsv) enthält für jede Datei die SHA-256-Prüfsumme. Der Windows-Build rekonstruiert anhand der Liste die abgenommenen Quellbytes einschließlich `Cargo.lock`.

| Geprüftes Paket | SHA-256 |
| --- | --- |
| Angenommener Take | `750a5ad375a810faea56cf6a0fa6798309625d90e1d89ff49db1209c64834b8a` |
| Installierter Windows-Quellsnapshot | `8b0d85bc4cadc953b2f224d088cafefd9fe4f896409de2f03f56c9cdb5416c58` |
