# EtuSchlopf für Windows

Lade **EtuSchlopf-Windows-x64.exe** herunter. Rust, Cargo, Python und zusätzliche Framework-Downloads sind für diesen Startweg nicht erforderlich. Für PDFs brauchst du MuseScore.

Ziehe deine MIDI-Datei auf die EXE oder starte die EXE und gib den Dateipfad ein. Neben der MIDI muss eine gleichnamige echte WAV-Datei liegen. Jeder Aufruf verwendet Kopien in einem neuen Unterordner von `EtuSchlopf-Ergebnisse` neben deiner MIDI. Originaldateien und frühere Laufordner werden nicht überschrieben.

Öffne nach erfolgreichem Lauf die PDF. Zum Bearbeiten in MuseScore dient die Datei mit der Endung `.render.musicxml`.

Diese Vorschau enthält die nativ angenommene r1526-Engine mit den benötigten Notationsdaten. Der Windows-Build prüft das Paket ohne Entwickler-PATH und bis zur synthetischen MIDI/WAV-Verarbeitung ohne PDF. Das einfache synthetische Beispiel wird derzeit durch eine bestehende Berichtprüfung abgelehnt; ein erfolgreicher musikalischer Paketlauf wird damit nicht behauptet. Das ersetzt noch keinen vollständigen Test auf einem sauberen Windows-10/11-Rechner mit MuseScore und fremden Stücken. MIDI ohne WAV ist noch offen.

Die EXE ist nicht codesigniert. Den SHA-256 findest du in `SHA256SUMS.txt`. Die enthaltenen Fremdlizenzen liegen im eingebetteten Laufzeitpaket; der Quellcode einschließlich Distribution ist im zugehörigen Git-Commit einsehbar.
