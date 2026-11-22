# Windows-Prüfstand r1992

EtuSchlopf **0.10.1839 r1992** setzte am 22. November 2026 unter Windows alle
15 Stücke frisch aus MIDI und WAV. Alle 15 Projekte meldeten Exitcode 0 und
vollständige Ausgaben. Der Build und der PDF-Renderer waren erfolgreich. Der
Launcher verwendete die an Quelle, Toolchain und Binary gebundenen Nachweise
für die Strict-Tests und den passenden Clippy-Nachweis mit null Warnungen erneut.

| Vergleich r1989 → r1992 | Befund |
| --- | --- |
| Master-MusicXML | Alle 15 Dateien sind bytegleich. |
| Render-MusicXML | Etüde 8 erhält am Bogen der Takte 25–29 `placement="above"`. Die übrigen 14 Dateien sind bytegleich. |
| PDF | Beide Läufe ergeben 59 Seiten. Im Rastervergleich bei 144 dpi unterscheidet sich nur Seite 4 von Etüde 8. |
| Etüde 8 | Die Gegenlage beseitigt alle sechs Kontakte über drei Systeme. |
| Chopin op. 10 Nr. 3, p02 | Ein Balkenkontakt blockiert die Gegenlage. Beide PDF-Seiten sind pixelgleich. |

SHA-256 des nativen Quellarchivs aus dem geprüften Windows-Export
`eed9c01ec0f21af1d0a03a1ae10bdf69e8af79c12f46d2fd39fdca3ef843d8c7`.

Der öffentliche Quellabgleich übernimmt Engine, Tests und Revisionsmetadaten
aus diesem Archiv. Für die öffentliche Dokumentation gelten die GitHub-Fassungen.
Der veröffentlichte Windows-Download bleibt auf seinem eigenen Prüfstand r1985.
