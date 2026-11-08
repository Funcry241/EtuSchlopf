# r1665 – Schlüsselentscheidung vor der Balkenpolitur erhalten

Parent: akzeptierter r1664, Version 0.10.1839. ChangeClass: RefactorOnly.

## Ursache und Änderung

`notation::polish_musicxml` rief E054 vor und nach `polish_measure` auf.
Der zweite Aufruf war als Reparatur eines entfernten Rückkehrschlüssels
kommentiert. Der aktuelle Zwischenpfad entfernt jedoch keine Schlüssel:
`polish_measure` kopiert alle Zwischenräume und verändert innerhalb der
Noten ausschließlich Balkenangaben.

Der zweite E054-Aufruf einschließlich seines lokalen Capture-/Receipt-
Transports entfällt. Der erste E054-Aufruf, seine Entscheidung und sein
Transport bleiben unverändert. E051 bindet seinen Erhaltungsbeleg direkt
an das Ergebnis der Balkenpolitur. Die Transportkomposition enthält den
frühen Schlüsseltransport und die tatsächlichen Balkenedits; der entfallene
Identitätsschritt wird nicht durch einen Ersatzschreiber ersetzt.

## Erhaltungsargument

E054 im BeamSafety-Modus liest die serielle Folge aus Schlüsselzuständen,
Oktavierungen und sichtbaren unteren Noten-/Akkordgruppen. Er ergänzt einen
Schlüssel nur bei mehr als drei Hilfslinien im aktuellen Schlüssel und
höchstens drei im alternativen Schlüssel. Nach diesem Pass liegt jeder
solche Wechsel bereits vor seiner auslösenden Gruppe. Alle anderen Gruppen
bleiben unverändert. Ein erneuter Pass ist daher auf derselben Eingabe
idempotent. `polish_measure` erhält sämtliche hierfür gelesenen Eigenschaften
und alle Schlüssel-/Oktavierungsblöcke. Balkenangaben sind keine Eingabe dieser
Entscheidung. Der erneute Schlüsselpass nach der Balkenpolitur ist folglich
ein Identitätsschritt. Im aktiven gemeinsamen Lesekurs ist E054 bereits ein
Identitätsschritt; diese Verzweigung bleibt unverändert.

Die synthetische Gegenprobe variiert beide Anfangsschlüssel, sechs Oktaven,
drei Tonstufen, normale Noten, Akkorde, unsichtbare Noten, Pausen, Vorschläge,
Schlüssel-/Oktavierungsgrenzen sowie versetzte unabhängige Takte. Sie prüft
über 1.080 Eingaben die Gleichheit mit dem bisherigen Zwei-Pass-Ablauf und
den Erhalt der Nicht-Balken-Inhalte durch den Balken-Owner, abgesehen von
Formatierungswhitespace innerhalb der Noten. Die Zwischenräume mit Schlüsseln
und Oktavierungen kopiert der Owner unverändert. Positive Zähler
verhindern eine ausschließlich wirkungslose Probe. Bestehende Receipt- und
Notationstests prüfen weiterhin die gebundenen Transport-/Erhaltungsgrenzen.

## Nachweisgrenze

Erwartete native Wirkung: Master-/Render-XML und PDF bleiben unverändert.
Der lokale Build verwendet die Root-Cargo.lock des bestätigten r1664-Exports;
diese lokale Lockdatei gehört nicht in das eingehende Take-Paket. Gates,
Abhängigkeiten, Schwellwerte und die frühe Schlüsselentscheidung ändern sich
nicht. Die native Abnahme mit authentischen WAV-Eingängen bleibt ein eigener
Nachweis und erfolgt mit `run.cmd --take --all`.
