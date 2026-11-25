# Windows-Abnahme r2005

EtuSchlopf **0.10.1839 r2005** verarbeitete unter Windows alle 15 MIDI/WAV-Paare des Vollkorpus. Alle Projekte meldeten vollständige Ausgaben und Exitcode 0. Der Lauf erzeugte 59 PDF-Seiten. Clippy Strict meldete null Warnungen. Die Strict-Tests meldeten 3.175 bestanden, null fehlgeschlagen und vier übersprungen.

Der Vergleich mit r2002 zeigt zwölf geänderte grafische Pausenpositionen im Render-MusicXML. Sieben betreffen Chopin op. 10 Nr. 3 p01, vier p02 und eine Etüde Nr. 7. Alle 15 Master-MusicXML-Dateien sind bytegleich. Bei 144 dpi sind 55 der 59 PDF-Seiten pixelgleich. Die vier visuell abweichenden Seiten wurden gesichtet. [Mathematische Herleitung](../../MATHEMATIK.md#kapitel-10-5) · [Notenbilder](../../examples/README.md#notenbilder) · [PDF-Prüfsummen](score-sha256.txt)

| Abnahmenachweis | Wert |
| --- | --- |
| Windows-Lauf | `corpus-0.10.1839-1795617318-17260` |
| SHA-256 des abgenommenen Takes | `135bd9cd4a51874732ecca8d7056afe54d3579f202b373393e476ad765437e7f` |
| SHA-256 des nativen Quellarchivs | `1d0e73a8943573af71f26d7168143f33a19627bee7a7a310a8b71f098cdf53d2` |
| SHA-256 der abgenommenen `Cargo.lock` | `a0178cc691bf26f99e83af20498711117ccb557bccc6d0827e6d5b1c204a6884` |
| SHA-256 der [Dateiliste des nativen Quellstands](accepted-source.tsv) | `5253067a58f2984af4a28b5210c41298270dd69d5468bfe90fd5985bd1820ea8` |
