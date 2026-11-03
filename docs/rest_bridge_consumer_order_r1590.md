# r1590: belegte Importreihenfolge und getrennte Fehlerdefinition

KodexOnly auf dem nativ angenommenen r1589. Alle Rust-Quellen, Tests,
Produktionsaufrufe, Writer, Quittungen und musikalischen Regeln bleiben
bytegleich. Dieser Take friert ausschließlich die zusätzliche kausale
Fehlerklasse ein. Er enthält keine produktive Balkenkorrektur.

## Befund und Gegenprobe

Im endgültigen Render-MusicXML von 1T13 stehen die unteren beidseitigen
Balkenmarkierungen auf den Einsätzen 0 und 48. Die Pause 24..48 folgt physisch
erst in einem späteren vollständigen Backup-Durchlauf. Der MuseScore-Importer
verarbeitet eine Pause innerhalb eines aktiven Balkens anders als nach dessen
Ende. Erhaltene Endpoint-Tags genügen deshalb hier allein nicht.

Eine isolierte Gegenprobe tauscht die zwei vorhandenen Tonköpfe mit ihren
vorhandenen gleich langen Forward-Platzhaltern im Pausendurchlauf. Sie ändert
keinen Ton-, Pausen- oder Balkenknoten. Die vollständigen Notenereignis-Multisets
einschließlich Einsatzzeiten bleiben gleich; beide Dateien bestehen das
unveränderte MusicXML-4.0-XSD. MuseScore Studio 4.7.6 unter Linux importiert
danach einen Balken mit eingebundener Pause und zeigt die Verbindung im PDF.
Das ist eine lokale Gegenprobe, keine native Annahme einer Rust-Reparatur.
Die genaue Windows-MuseScore-Version des angenommenen Exports ist nicht belegt.

Synthetische Rendererproben bestätigen: getrennte Reihenfolge ergibt keinen
Balken; zusätzliche Rest-Beamtags allein ebenfalls keinen; zusammenhängende
Reihenfolge ergibt einen Balken. Stimmenumbenennung, proportionale
Zeitbasisskalierung und unabhängige Verdopplung behalten das Ergebnis. Diese
Proben erweitern nicht die Zeitbasiszulassung des produktiven E050-Owners.

## Warum eine eigene Definition?

Die angenommene Precanonical-Klasse betrifft noch nicht geschriebene
Verbindungen. Die frühere Cursor-Lane-Klasse betrifft einen zusammenhängenden
Tonlauf und eine andere bestehende Reparatur. Keine deckt diese nachträgliche
Rest-Verarbeitungsgrenze. `src/take_change_control.rs` verlangt die Definition
in einem bereits angenommenen Parent vor neuem selektorsensitivem Rust-Code.
Die Regel bleibt unverändert.

Der spätere Korrekturweg muss die vorhandene Serialisierungszuständigkeit und
den tatsächlichen Occurrence-/Quittungstransport binden. Der ausdrücklich auf
Balkentags beschränkte E050-Companion darf kein stiller Umsortierer werden.
Die isolierte Python-Gegenprobe ist keine produktive Reparatur.

## Prüfung und native Anwendung

Die lokale Parent-Zulassung wird gegen den vollständigen r1589-Snapshot
ausgeführt. Sämtliche Rust-Dateien werden byteweise mit dem Parent verglichen.
Neue Compiler-, Clippy- oder Testläufe werden deshalb nicht behauptet; der
native r1589-Nachweis bleibt unverändert bestehen.

Native Anwendung: `run.cmd --take --all` ohne Zusatzflag. Der angenommene Export
dieses Definitions-Takes schafft den erforderlichen Parent für die produktive
Umsetzung. Das ausgelieferte Notenbild bleibt das von r1589; die sichtbar
korrigierte Gegenprobe ist ausdrücklich separat.
