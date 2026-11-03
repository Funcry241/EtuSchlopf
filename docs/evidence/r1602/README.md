# Native Abnahme r1602

Der native Lauf über zehn Etüden war dunkelgrün und freigegeben. Build,
Clippy und Renderer liefen erfolgreich, der Exitcode war 0. Clippy meldete
keine Warnungen. 2.919 Tests bestanden, drei wurden ignoriert.

Der native Snapshot stimmt in allen 2.002 Take-Dateien bytegenau mit
dem akzeptierten Take überein. Die Root-Cargo.lock liegt zusätzlich im Snapshot.

Gegenüber r1601 blieben alle zehn Master- und zehn Render-MusicXML-Dateien
bytegleich. Die PDF-Dateien unterscheiden sich auf Byteebene. Bei 150 dpi
sind alle 39 Seiten pixelgleich. Damit bleibt auch die Balkenreparatur aus
r1601 im Notenbild erhalten.

[Native Annahme](native_acceptance.txt) · [Laufstatus](native_run_outcome.txt) ·
[Notenbeispiel aus r1602](../../examples/etude-1-r1602.pdf)


## Windows-Testversion

Der [Windows-Lauf](https://github.com/Funcry241/EtuSchlopf/actions/runs/41734180365)
hat Build, Pakettests, isolierten Starttest und PDF-Ausgabe mit MuseScore
Studio 4.7.5 bestanden. Das Startbeispiel enthält genau das MIDI/WAV-Paar
und die PDF aus diesem Paketlauf.

[Windows-Release r1602 mit EXE, Beispiel, Quellen und Prüfsummen](https://github.com/Funcry241/EtuSchlopf/releases/tag/v0.10.1839-r1602-windows-preview)
