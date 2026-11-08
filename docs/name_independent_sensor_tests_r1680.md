# r1680 – Namensunabhängige Sensorprüfung und synthetische Testnamen

ChangeClass: RefactorOnly. Parent ist der bereitgestellte r1679 / 0.10.1839.
r1676 ist im Export `assistant_export_latest(20261108-030840).zip` als nativ
angenommen geprüft. r1679 muss vor r1680 nativ angenommen werden.
Dieser Take ersetzt r1678 unter neuer eindeutiger Revision. Produktivcode
und Tests bleiben gegenüber dem lokal qualifizierten r1678 unverändert;
Identität, Basisdokumentation und Änderungsmanifest sind aktualisiert.

Dieser zweite Teil ergänzt den Kodex-Take r1679. Die Trennung folgt den
unveränderten Paketrollen: Launcher-/Exportcode ist Kodex, der separate
Sensor und allgemeine Tests gehören zu anderen Quellrollen.

Der separate WAV-Sensor wiederholte die eingefrorene Inferenz nur für einen
bestimmten Projektnamen. Die Wiederholungsprüfung läuft jetzt für jede frisch
ausgeführte Sensorqualifikation nach demselben Verfahren. Ihr Ergebnis bleibt
`yes` oder `no`; der Name entscheidet nicht mehr über `not_probed`. Sensor-
Freeze, MIDI-Zugriff erst nach Freeze, Modellpin und Ereignisvergleich bleiben
erhalten. Bei bisher ausgelassenen Projekten kostet eine frische Qualifikation
dadurch einen zusätzlichen Inferenzdurchlauf. Es entsteht kein Musikschreibrecht.

Allgemeine Projektstamm-, JSON-, Unicode-, PDF-Routen- und PDF-Frischetests
verwenden synthetische Namen statt konkreter Etüdennamen. Akzente, Unicode,
Anführungszeichen, Pfadtraversal, Frische und vollständige PDF-Bytes behalten
ihre bisherigen Prüfungen. Scanner-Negativtests bleiben unberührt: Sie müssen
verbotene Etüden-/Takt-Sonderregeln weiterhin erkennen können.

Die Watch-Tests und bestehenden Browserproben werden lokal geprüft.
Die Sensoränderung erhält einen Rust-Syntaxcheck; ein vollständiger separater
Sensorbuild und ein echter ONNX-/WAV-Wiederholungslauf sind hier offen: Im
Offline-Cache fehlt bereits `minreq`. Die beiden Shared-Quelldateien stellt
der vorhandene Sensor-Bootstrap in seiner Runtime bereit. Die Modell-/Audio-
qualifikation und die native Korpusabnahme bleiben separate Nachweise.
Die musikalischen Owner und Auswahlregeln sind unverändert.
