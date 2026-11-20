# EtuSchlopf für Windows

Lade **EtuSchlopf-Windows-x64.exe** herunter. Rust, Cargo, Python und zusätzliche Framework-Downloads sind für diesen Startweg nicht erforderlich. Für PDFs brauchst du MuseScore.

Ziehe deine MIDI-Datei auf die EXE oder starte die EXE und gib den Dateipfad ein. Neben der MIDI muss eine gleichnamige echte WAV-Datei liegen. Jeder Aufruf verwendet Kopien in einem neuen Unterordner von `EtuSchlopf-Ergebnisse` neben deiner MIDI. Originaldateien und frühere Laufordner werden nicht überschrieben.

Öffne nach erfolgreichem Lauf die PDF. Zum Bearbeiten in MuseScore dient die Datei mit der Endung `.render.musicxml`.

Die EXE enthält die Engine r1961 (0.10.1839). Der Windows-Build prüft die Dateien gegen den angenommenen Quellstand. Er startet die paketierte EXE in einem eigenen Ordner und erzeugt aus dem MIDI/WAV-Startbeispiel mit MuseScore ein PDF.

Die EXE ist nicht codesigniert. Den SHA-256 findest du in `SHA256SUMS.txt`. Die enthaltenen Fremdlizenzen liegen im eingebetteten Laufzeitpaket; der Quellcode einschließlich Distribution ist im zugehörigen Git-Commit einsehbar.

[Startbeispiel r1961](https://github.com/Funcry241/EtuSchlopf/releases/download/v0.10.1839-r1961-windows-preview/EtuSchlopf-Beispiel-r1961.zip) enthält MIDI, WAV und die mit der paketierten EXE erzeugte PDF.
