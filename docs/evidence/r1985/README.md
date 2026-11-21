# Windows-Abnahme r1985

EtuSchlopf **0.10.1839 r1985** verarbeitete am 21. November 2026 unter Windows alle 15 MIDI/WAV-Paare. Der Lauf endete mit Exitcode 0. Build und Strict Clippy waren erfolgreich, Clippy meldete null Warnungen. Von den Strict-Tests bestanden 3.156, vier wurden übersprungen. [Prüfdaten](windows-checks.json)

## Angenommene Änderungen

| Revision | Änderung |
| --- | --- |
| r1983 | Die Pausenverarbeitung spart zwei XML-Kopien und zwei Protokollzweige. Eine synthetische Gegenprobe prüft, dass nach einem ungültigen Rücksprung kein vorbereiteter Umbau in die Ausgabe gelangt. Das historische Balken-Diagnosewerkzeug wurde entfernt. |
| r1984 | Die Oktavierungsvorschau verwendet den Taktbereich und ermittelte Tonanker erneut. Drei WAV-Berichtsauswertungen teilen ein einmal dekodiertes Signal. Der Harmonikbericht verwendet nur Sensordaten, die zur geprüften Aufnahme gehören. Alle 15 Projektberichte enthalten einen beobachteten Sensorbefund. |
| r1985 | Die Kodex-Prüfung liest die Rust-Quellen für vier Marker gemeinsam und ermittelt die Referenzpfade pro Prüflauf einmal. Beim Strict-Start entfällt die zusätzliche Prüfung des Release-Builds. |

Die Windows-Annahme belegt die Folge r1973 → r1983 → r1984 → r1985. [Prüfsummen der angenommenen ZIPs](take-sha256.tsv) · [Dateiprüfsummen des nativen Quellstands](accepted-source.tsv)

## Vergleich mit r1973

Alle 15 Master-MusicXML und alle 15 Render-MusicXML sind paarweise bytegleich. Die 59 PDF-Seiten sind beim Rastervergleich mit 150 dpi paarweise pixelgleich. Die SHA-256-Werte der PDFs unterscheiden sich. Die PDF-Metadaten weisen unterschiedliche Zeitstempel für Erstellung und letzte Änderung aus. [Prüfsummen und Projektzeiten](compare-scores.tsv) · [Rasterprüfsummen pro Seite](compare-pages.tsv)

Der Korpus brauchte 10:09 Minuten gegenüber 9:55 Minuten bei r1973. Der Launcher erreichte das Ende des Korpuslaufs nach rund 14:50 Minuten gegenüber 15:51 Minuten. Die Differenz dieser beiden Läufe erlaubt keine getrennte Aussage über den Laufzeitbeitrag der einzelnen Änderungen.

Der vollständige Ausbau des alten Pausen-Schreibpfads und des Staccato-Pfads steht aus.
