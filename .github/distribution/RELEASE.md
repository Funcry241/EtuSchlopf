# EtuSchlopf für Windows

Lade **EtuSchlopf-Windows-x64.exe** herunter. Rust, Cargo, Python und zusätzliche Framework-Downloads sind für diesen Startweg nicht erforderlich. Für PDFs brauchst du MuseScore.

Ziehe deine MIDI-Datei auf die EXE oder starte die EXE und gib den Dateipfad ein. Neben der MIDI muss eine gleichnamige echte WAV-Datei liegen. Jeder Aufruf verwendet Kopien in einem neuen Unterordner von `EtuSchlopf-Ergebnisse` neben deiner MIDI. Originaldateien und frühere Laufordner werden nicht überschrieben.

Öffne nach erfolgreichem Lauf die PDF. Zum Bearbeiten in MuseScore dient die Datei mit der Endung `.render.musicxml`.

Der Build-Workflow dieses Entwicklungsstands bindet die Engine an den nativ angenommenen Quellstand r1577 mit den benötigten Notationsdaten. Seine Prüfbelege stehen unter `docs/evidence/r1577`. Der Workflow prüft das Paket ohne Entwickler-PATH und mit dem vorhandenen isolierten Starttest; der erfolgreiche Windows-Paketlauf #14 ist unter https://github.com/Funcry241/EtuSchlopf/actions/runs/41326070573 belegt. Die native Korpusabnahme ersetzt keinen vollständigen Anwender-Test auf einem sauberen Windows-10/11-Rechner mit MuseScore und fremden Stücken. MIDI ohne WAV ist noch offen. Die veröffentlichte Windows-Testversion enthält r1577. Die EXE stammt unverändert aus Lauf #14; Quellen und Prüfsummen liegen beim Release v0.10.1839-r1577-windows-preview.

Die EXE ist nicht codesigniert. Den SHA-256 findest du in `SHA256SUMS.txt`. Die enthaltenen Fremdlizenzen liegen im eingebetteten Laufzeitpaket; der Quellcode einschließlich Distribution ist im zugehörigen Git-Commit einsehbar.
