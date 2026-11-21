# EtuSchlopf für Windows

Die portable x64-EXE enthält EtuSchlopf 0.10.1839 r1985. Für die PDF-Ausgabe brauchst du MuseScore. Die EXE läuft ohne Rust, Cargo oder Python. [Windows-EXE und Prüfsummen](https://github.com/Funcry241/EtuSchlopf/releases/tag/v0.10.1839-r1985-windows-preview)

Lege deine Performance-MIDI und die gleichnamige WAV-Aufnahme in denselben Ordner. Ziehe die MIDI auf die EXE oder starte die EXE und gib den MIDI-Pfad ein. Jeder Lauf erhält unter `EtuSchlopf-Ergebnisse` neben der MIDI einen eigenen Ordner. Dort findest du das PDF und die MusicXML-Datei für MuseScore.

Das Startbeispiel enthält ein MIDI/WAV-Paar und das PDF, das die r1985-EXE im Windows-Pakettest frisch daraus setzt. Der Windows-Vollkorpus r1985 setzte alle 15 Stücke. Gegenüber r1973 sind alle 30 MusicXML-Dateien bytegleich und alle 59 PDF-Seiten bei 150 dpi pixelgleich. Die Galerie und das Notenarchiv zeigen die veröffentlichten PDFs aus r1965. [Startbeispiel r1985](https://github.com/Funcry241/EtuSchlopf/releases/download/v0.10.1839-r1985-windows-preview/EtuSchlopf-Beispiel-r1985.zip) · [Vergleich aller 15 Stücke](../../docs/evidence/r1985/README.md) · [Korpusnoten r1965](https://github.com/Funcry241/EtuSchlopf/releases/download/v0.10.1839-r1965-windows-preview/EtuSchlopf-Notenbilder-r1965.zip)

Für einen eigenen Engine-Build entpackst du `EtuSchlopf-r1985-source.zip`. Es enthält den angenommenen nativen Quellstand samt geprüftem `Cargo.lock`. Im entpackten Ordner baut `cargo +1.98.0 build --locked --profile strict --bin etuschlopf` die Engine. Der Windows-Starter und seine Paketierung liegen im öffentlichen Commit, den `runtime-provenance.txt` nennt. [Annahme von r1985](../../docs/evidence/r1985/README.md)

Die EXE ist unsigniert. Mit `Get-FileHash .\EtuSchlopf-Windows-x64.exe` kannst du ihren SHA-256-Wert mit `SHA256SUMS.txt` vergleichen. Die Fremdlizenzen liegen im Laufzeitpaket.
