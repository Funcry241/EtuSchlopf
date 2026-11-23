# Windows-Abnahme r1994

Der abgenommene Stand **EtuSchlopf 0.10.1839 r1994** verarbeitete unter Windows
alle 15 Stücke frisch aus MIDI und WAV. Alle 15 Projekte endeten mit Exitcode 0
und vollständigen Ausgaben. Der PDF-Renderer lief erfolgreich. Die Strict-Prüfung
meldete 3.168 bestandene Tests, vier übersprungene Tests und null Clippy-Warnungen.

Die PDF-Prüfung verwendet die im PDF deklarierte Streamlänge. So berücksichtigt
sie auch gültige komprimierte Nutzdaten am Stream-Ende.

Die [Galerie](../../examples/README.md#notenbilder) enthält die Original-PDFs
aus dem r1994-Lauf. Die [SHA-256-Prüfsummen](score-sha256.txt) beziehen sich
auf diese veröffentlichten Dateien.

| Abnahmenachweis | Wert |
| --- | --- |
| Windows-Lauf | `corpus-0.10.1839-1795383853-24064` |
| SHA-256 des abgenommenen Takes | `aac240fd96b468b1679bba68ff6d0a55f0366fed3592c5d6acbe4ee1d248c662` |
| SHA-256 des nativen Quellarchivs | `6c6ba408002f1cb6ad777bbf6b3803900936c16abe82e2bae173022e1055e10d` |
| SHA-256 der abgenommenen `Cargo.lock` | `a0178cc691bf26f99e83af20498711117ccb557bccc6d0827e6d5b1c204a6884` |
