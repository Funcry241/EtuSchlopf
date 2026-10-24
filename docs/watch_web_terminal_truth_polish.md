# Watch Web Terminal Truth Polish

Version: 0.10.1737 r01

## Anlass

Der erste reale Browserlauf von v0.10.1736 zeigte einen bereits terminalen
`SessionSnapshotV1` korrekt als `COMPLETED`, waehrend einzelne Karten weiterhin nur
die letzte fachliche Stage (`Finalizing`, `Release`) darstellten. Gleichzeitig wurde
das planmaessige Ende der Web-/SSE-Verbindung oben als `FINISHED` angezeigt. Dadurch
standen fachlicher Laufzustand und Transportzustand optisch zu nah beieinander.

Die Runtime-Wahrheit selbst war dabei nicht falsch: der Sessionstatus wird weiterhin
ausschliesslich aus den Source-Statuswerten abgeleitet. Dieser Take aendert nur die
Browserprojektion.

## Projektion

Die Weboberflaeche trennt nun zwei Achsen:

```text
fachlicher Runtime-Zustand
WAITING | RUNNING | COMPLETED | FAILED | CANCELLED

Transportzustand
MQTT · lokal · read-only · CONNECTING | LIVE | RECONNECTING | CLOSED
```

Das Ende der SSE-/Watch-Verbindung darf damit keinen fachlichen Zustand mehr erfinden.
Ein terminaler Runtime-Zustand bleibt sichtbar; nur die dezente Transportzeile wechselt
nach dem planmaessigen Ende auf `CLOSED`.

## Source-Karten

Solange eine Source `running` ist, zeigt die Karte weiterhin Stage und optionale
Activity. Sobald die Source terminal ist, zeigt die Karte stattdessen ihren echten
Status:

```text
Completed
Failed
Cancelled
```

Damit wird z. B. ein bereits abgeschlossener Launcher nicht mehr als scheinbar noch
`Finalizing` dargestellt. Die letzte Stage bleibt im unveraenderten Snapshot erhalten,
wird aber nicht mit dem terminalen Status verwechselt.

## Zeitdarstellung

Bei laufenden Sources bleiben `gesehen` und `Zustand seit` sichtbar. Bei terminalen
Sources wird daraus eine ruhigere Aussage wie `abgeschlossen vor`. Die Browserseite
zaehlt das Alter ab dem letzten empfangenen Snapshot lokal weiter; dadurch bleibt die
Zeit auch nach dem planmaessigen Watch-Ende sinnvoll lesbar. Dies ist nur eine
Zeitprojektion und keine neue Runtime-Wahrheit.

## Visuelle Ruhe

Die Session-ID ist staerker zurueckgenommen und gekuerzt. MQTT-/Loopback-Details stehen
nur noch dezent im Footer; die Portnummern bleiben dort lediglich als Tooltip erhalten.
Der obere Statusbereich gehoert ausschliesslich dem fachlichen Laufzustand.

## Nicht geaendert

- kein MQTT-Protokoll- oder Topic-Change
- keine Aenderung an retained State oder Subscription-Lifecycle
- keine Aenderung an UDP oder SessionAggregator
- keine neue Runtime- oder Health-Wahrheit
- kein Command Channel
- keine Persistenz
- keine externe Cargo-Abhaengigkeit
- keine Launcher-, Musik-, Writer-, Render- oder Regelmutation

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 6"
```

Danach:

```powershell
.\run.cmd --watch --strict "Etüde Nr. 6"
```
