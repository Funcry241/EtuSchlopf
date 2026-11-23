# Windows-Abnahme r1964

EtuSchlopf 0.10.1839 r1964 wurde am 20. November 2026 unter Windows angenommen. Der anschließende Einzellauf verarbeitete Mozarts Klaviersonate A-Dur KV 331, Andante grazioso, Einspielung p01. Als Eingaben dienten die Performance-MIDI und ihre WAV. Der Lauf endete mit Exitcode 0 und erzeugte zwei PDF-Seiten. Der Windows-Testlauf meldete **3.148 bestandene Tests**, vier ignorierte und keine Fehler. Strict Clippy meldete keine Warnung.

## Ergebnis für Mozart

| Messung | r1963 | r1964 |
| --- | ---: | ---: |
| Reguläre Noten mit übereinstimmendem Einsatz und Tonhöhe | 474 von 474 | 474 von 474 |
| Übereinstimmende geschriebene Dauern | 438 von 474 | 460 von 474 |
| Sichtbare Pauseneinträge im MusicXML | 14 | 54 |

Der Vergleich verwendet die unabhängige Notationszuordnung zur Aufnahme nach dem Lauf. Die Systemzuordnung bleibt bei allen 474 regulären Tönen gleich. Von den 36 früheren Dauerabweichungen sind 24 korrigiert. In Takt 18 und 28 kommt je eine Abweichung hinzu. Das A erscheint dort als punktierte Viertelnote, die Referenz schreibt eine Achtel. Master- und Render-MusicXML stimmen bytegenau überein. · [Notengalerie](../../examples/README.md)

Die Galerie enthält diesen neuen Mozart-Lauf und 13 PDFs aus dem letzten belegten [Windows-Vollkorpuslauf r1961](../r1961/README.md). Der Vollkorpus umfasste 14 Stücke mit insgesamt 57 Seiten.

## Prüfsummen

| Gegenstand | SHA-256 |
| --- | --- |
| Angenommenes Take-Paket | `407fed052c5fc33f49f3cab319fa519b62c085668bd69db7cbdb75ac07a02cf9` |
| Im Windows-Export eingebettetes Quellarchiv | `11cf97dd4d9db4f8e8c6b30de9ac4f3ea67729e6af9c4a669d1b4dfb38f01fd3` |
| Mozart-PDF aus dem Windows-Lauf | `880e6621529b52cd042ac891b1cf06e8382c3bcc67e057df17fd770bfc2f06c5` |
| Master- und Render-MusicXML | `1adf3a648c4cb86af8a97abd86f1ca184e454a8505d8eba642bc3b43fb4939c2` |

Der [Annahmebeleg](native_acceptance.txt) bindet das ursprüngliche Take-Paket. Der [Windows-Build](../../../.github/distribution/verify-accepted-source.py) prüft alle 2.217 Dateien anhand der gemeinsamen SHA-256-Prüfsumme `467f51863e55d9a2b6cfb11fd98a9cd7c6fe60c897f5adf56d67339522b6f15c`. Er rekonstruiert sie aus dem veröffentlichten r1961-Verzeichnis, den vier hinzugekommenen Pfaden und den ausgecheckten Dateiinhalten. Die [Dokumentationsprüfsummen](github_projection_sha256.tsv) erfassen die für GitHub angepassten Fassungen. Die [Prüfsummen der Notenbilder](score_sha256.tsv) und die [Quellenangaben](score_credits.md) gehören zum Download der Galerie.
