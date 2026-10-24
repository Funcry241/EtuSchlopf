# Watch Local Web Surface

Version: 0.10.1736 r01

## Ziel

Der normale Ein-Befehl-`--watch`-Lauf oeffnet die bestehende read-only Runtime-
Observation nun als lokale Webseite. Die Weboberflaeche ersetzt keine Runtime-Wahrheit
und greift nicht direkt auf den SessionAggregator zu.

## Architektur

```text
EtuSchlopf Sources
      ↓ UDP 127.0.0.1:47621
SessionAggregator
      ↓ SessionSnapshotV1
Embedded MQTT Broker 127.0.0.1:47622
      ↓ etuschlopf/v1/session/+/state
Watch Web MQTT Consumer
      ↓ localhost HTTP + SSE
Browser 127.0.0.1:47623
```

Die Browser-Projektion konsumiert damit denselben oeffentlichen MQTT-Vertrag wie jeder
andere Consumer. `SessionSnapshotV1` bleibt die einzige Session-Projektion; die Web-
Bridge fuegt ausschliesslich ihren praesentativen MQTT-Verbindungszustand hinzu.

## Lokale Weboberflaeche

Der Server bindet ausschliesslich an Loopback:

```text
http://127.0.0.1:47623/
```

Beim erfolgreichen Start wird der lokale Standardbrowser best effort automatisch
goeffnet. Die Seite verwendet keine externen Assets, CDNs, Webfonts oder JavaScript-
Bibliotheken. HTML, CSS und JavaScript werden vom Watch-Binary selbst ausgeliefert.

Die Seite zeigt:

- Sessionstatus
- aktuelles Projekt beziehungsweise Korpus
- Stage und optionale Activity gemeinsam
- Launcher-, Corpus- und Project-Sources
- `last_seen` und `last_state_change`
- echten Corpus-Fortschritt, sofern vorhanden
- den letzten terminalen Sessionzustand auch nach dem planmaessigen Watch-Ende im offenen Browser
- MQTT-Verbindungszustand `CONNECTING`, `LIVE` oder `RECONNECTING`

Updates gelangen ueber Server-Sent Events zur Seite. SSE transportiert nur die aktuelle
Browserprojektion; es entsteht keine Eventhistorie und kein zweites Zustandsmodell.

## Failure Isolation

Web-Bind- oder Web-Startfehler duerfen weder Watch noch Musik zum Gate machen. Kann die
lokale Weboberflaeche nicht gestartet werden, verwendet die Watch die bereits
akzeptierte Terminal-MQTT-Oberflaeche als Fallback. Browser-Startfehler sind ebenfalls
best effort und besitzen kein Musikrecht.

## Nicht geaendert

- kein Command Channel
- keine Ruecksteuerung zu EtuSchlopf
- kein direkter Browserzugriff auf UDP oder Aggregator
- kein WebSocket-/MQTT-over-WebSocket-Parallelvertrag
- keine Persistenz oder History
- keine externe Cargo-Abhaengigkeit
- keine Musik-, Writer-, Render- oder Regelmutation
- Launcher-Spawn/Lifecycle bleibt in diesem Take unveraendert

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 6"
```

Danach:

```powershell
.\run.cmd --watch --strict "Etüde Nr. 6"
```
