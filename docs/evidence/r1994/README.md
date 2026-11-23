# Windows-Abnahme r1994

Der abgenommene Stand **EtuSchlopf 0.10.1839 r1994** verarbeitete unter Windows
alle 15 Stücke frisch aus MIDI und WAV. Alle 15 Projekte endeten mit Exitcode 0
und vollständigen Ausgaben. Der PDF-Renderer lief erfolgreich. Die Strict-Prüfung
meldete 3.168 bestandene Tests, vier übersprungene Tests und null Clippy-Warnungen.

Die PDF-Prüfung liest komprimierte Streams nun anhand der im PDF deklarierten
Länge. Dadurch bleiben auch gültige Nutzdaten am Stream-Ende erhalten. Etüde 3,
deren Prüfung nach der Cargo-Aktualisierung beim Dekomprimieren abbrach,
lieferte im abgenommenen Lauf alle vier Seiten.

| Vergleich r1992 → r1994 | Befund |
| --- | --- |
| Master-MusicXML | Alle 15 Dateien sind bytegleich. |
| Render-MusicXML | Alle 15 Dateien sind bytegleich. |
| PDF | Alle 59 Seiten sind im Rastervergleich bei 144 dpi pixelgleich. |

Die [Galerie](../../examples/README.md#notenbilder) enthält die Original-PDFs
aus dem r1994-Lauf. Die [SHA-256-Prüfsummen](score-sha256.txt) beziehen sich
auf diese veröffentlichten Dateien.

| Abnahmenachweis | Wert |
| --- | --- |
| Windows-Lauf | `corpus-0.10.1839-1795383853-24064` |
| SHA-256 des abgenommenen Takes | `aac240fd96b468b1679bba68ff6d0a55f0366fed3592c5d6acbe4ee1d248c662` |
| SHA-256 des nativen Quellarchivs | `6c6ba408002f1cb6ad777bbf6b3803900936c16abe82e2bae173022e1055e10d` |
| SHA-256 der abgenommenen `Cargo.lock` | `a0178cc691bf26f99e83af20498711117ccb557bccc6d0827e6d5b1c204a6884` |
