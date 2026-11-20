# EtuSchlopf für Windows

Lade **EtuSchlopf-Windows-x64.exe** herunter. Rust, Cargo, Python und zusätzliche Framework-Downloads sind für diesen Startweg nicht erforderlich. Für PDFs brauchst du MuseScore.

Ziehe deine MIDI-Datei auf die EXE oder starte die EXE und gib den Dateipfad ein. Neben der MIDI muss eine gleichnamige echte WAV-Datei liegen. Jeder Aufruf verwendet Kopien in einem neuen Unterordner von `EtuSchlopf-Ergebnisse` neben deiner MIDI. Originaldateien und frühere Laufordner werden nicht überschrieben.

Öffne nach erfolgreichem Lauf die PDF. Zum Bearbeiten in MuseScore dient die Datei mit der Endung `.render.musicxml`.

Die EXE enthält die Engine r1964 (0.10.1839). Der [Windows-Nachweis für Mozart](../../docs/evidence/r1964/README.md) dokumentiert den angenommenen Quellstand. GitHub baut daraus die EXE. Er startet die paketierte EXE in einem eigenen Ordner und erzeugt aus dem MIDI/WAV-Startbeispiel mit MuseScore ein PDF.

Die EXE ist nicht codesigniert. Den SHA-256 findest du in `SHA256SUMS.txt`. Die Fremdlizenzen stehen im eingebetteten Laufzeitpaket. Der zugehörige Git-Commit enthält den Quellcode einschließlich des Windows-Starters.

[Startbeispiel r1964](https://github.com/Funcry241/EtuSchlopf/releases/download/v0.10.1839-r1964-windows-preview/EtuSchlopf-Beispiel-r1964.zip) enthält MIDI, WAV und die mit der paketierten EXE erzeugte PDF.
