# EtuSchlopf für Windows

Lade **EtuSchlopf-Windows-x64.exe** herunter. Rust, Cargo, Python und zusätzliche Framework-Downloads sind für diesen Startweg nicht erforderlich. Für PDFs brauchst du MuseScore.

Ziehe deine MIDI-Datei auf die EXE oder starte die EXE und gib den Dateipfad ein. Neben der MIDI muss eine gleichnamige echte WAV-Datei liegen. Jeder Aufruf verwendet Kopien in einem neuen Unterordner von `EtuSchlopf-Ergebnisse` neben deiner MIDI. Originaldateien und frühere Laufordner werden nicht überschrieben.

Öffne nach erfolgreichem Lauf die PDF. Zum Bearbeiten in MuseScore dient die Datei mit der Endung `.render.musicxml`.

Der Windows-Workflow prüft den nativ abgenommenen Quellstand r1602 und baut daraus die Engine. Der Build prüft die EXE ohne Entwickler-PATH und führt den isolierten Starttest aus. Der Download enthält die geprüfte Engine r1601.

Die EXE ist nicht codesigniert. Den SHA-256 findest du in `SHA256SUMS.txt`. Die enthaltenen Fremdlizenzen liegen im eingebetteten Laufzeitpaket; der Quellcode einschließlich Distribution ist im zugehörigen Git-Commit einsehbar.
