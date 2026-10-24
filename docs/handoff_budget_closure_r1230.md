# r1230 HandoffBudgetClosure auf akzeptiertem r1229

Der vorherige Render-Kandidat wurde nach erfolgreichen Windows-All-Targets-Tests und erfolgreichem Programm/PDF-Lauf mit Exit 1 zurückgerollt: assistant handoff hard limit exceeded: 786413 + 254 > 786432 bytes. Die feste Reserve von 8 KiB deckte die Auslassungsvermerke vieler geänderter Quelldateien nicht ab.

Die Export-Projektion reserviert jetzt vor jeder optionalen vollständigen Datei die tatsächlichen Bytes aller noch nötigen Auslassungsvermerke und Abschlussabschnitte. Pfad, Bytezahl und SHA-256 bleiben vorhanden. Pflichtdateien werden weiter vollständig verlangt; ihr Überlauf bleibt ein Fehler. Das Limit bleibt 768 KiB. Die Änderung betrifft ausschließlich AssistantExport und dessen exakten Trust-Anchor-Pin; Gates, Musik, Writer, Release-Entscheidungen und Rollback bleiben unverändert.

Regression: 128 lange Quellpfade und große optionale Dateien bei fast gefülltem Pflichtteil. Der alte Code scheitert, der neue publiziert deterministisch eine vollständige Projektion innerhalb des Limits. Zusätzlicher Negativtest: Pflichtüberlauf publiziert keinen Kandidaten und erhält die vorherige Projektion.

Installationsfolge: erst dieses KodexOnly-Paket auf r1229 mit ./run.cmd --take --all; danach r1231 RenderSovereigntyClosureMaxi mit ./run.cmd --take --all. Die Render-Änderungen gehören weiterhin zu RefactorOnly. Kein Force und keine Aufweichung der Klassenprüfung.
