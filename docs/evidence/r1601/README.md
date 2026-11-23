# Native Abnahme r1601

Am 3. November 2026 lief r1601 über alle zehn Etüden. Der akzeptierte Take
hat den SHA-256
`17f98efe4e6304bb385833c1088ee2b40afa41b64872c4e7b45d49d110b11726`.
Der native Quellsnapshot stimmt in allen 2.002 Take-Dateien bytegenau mit
dem Take überein und enthält zusätzlich die Root-Cargo.lock.

| Prüfung | Ergebnis |
|---|---|
| Build, Clippy, Renderer und Lauf | erfolgreich, finaler Exit 0 |
| Native Tests | 2.919 bestanden, 0 fehlgeschlagen, 3 ignoriert |
| Master-MusicXML gegen r1599 | 10 von 10 bytegleich |
| Render-MusicXML | Änderungen in 9 Etüden, in den geänderten Takten ausschließlich Stimmenzuordnungen |
| Etüde 6, T32 | H4 wechselt von Stimme 7 auf 2. Beide Gruppen sind im PDF verbunden. |
| Etüde 6, T34 und Etüde 5, T37 | Balkenbrücken erhalten |
| PDF-Seitenvergleich | Gleiche Seitenzahlen. 9 von 39 Seiten unterscheiden sich bei 150 dpi. Die übrigen sichtbaren Änderungen betreffen vor allem die Haltebogenrichtung. |

Der Importtrace für H4 meldet die ursprüngliche Eingabe, die bereits
vorliegenden Änderungen und den Versuch mit Zielstimme 2 als zulässig.
Die hörzeitbasierte Kanalprüfung bleibt bestehen.

Der [native Laufstatus](native_run_outcome.txt) führt separat
`audit_blocked_expected_audit_block`. Die Windows-Paketprüfung lief getrennt im GitHub-Workflow.

## Windows-Testversion

Der [Windows-Workflow](https://github.com/Funcry241/EtuSchlopf/actions/runs/41727617295)
hat Build, Pakettests und den isolierten Starttest bestanden.
Die [Windows-Testversion r1601](https://github.com/Funcry241/EtuSchlopf/releases/tag/v0.10.1839-r1601-windows-preview)
enthält die EXE, das zum Build gehörende Quellarchiv und die SHA-256-Prüfsummen.
Die EXE hat SHA-256
`b01d0d88547cf7e26375414477e82cbdc5d9a027460ed4a9fd01f5ebfd9277e8`.

[Annahmebeleg](native_acceptance.txt) ·
[Quellprüfsummen](accepted_runtime_sha256.tsv) ·
[Prüfsummen der GitHub-Fassung](github_projection_sha256.tsv)
