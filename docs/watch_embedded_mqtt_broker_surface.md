# Watch Embedded MQTT Broker Surface

## Ziel

Dieser Take erzeugt erstmals die separate `EtuSchlopf Watch` als eigenes Cargo-Projekt
unter `tools/etuschlopf_watch/`. Die Musikseite bleibt unveraendert und kennt weiterhin
weder MQTT-Topics noch Broker noch GUI.

Die Watch bindet den bereits akzeptierten UDP-Receiver auf `127.0.0.1:47621`, aggregiert
`SourceSnapshotV1` zu `SessionSnapshotV1` und projiziert jede beobachtete Session retained
auf:

```text
etuschlopf/v1/session/<ObservationSessionId>/state
```

Der eingebettete MQTT-v1-Broker bindet ausschliesslich an Loopback auf TCP-Port 47622.

## Dependency-Firewall

Das Watch-Projekt besitzt ein eigenes `Cargo.toml` und `Cargo.lock`. In diesem ersten
Broker-Take besitzt es absichtlich **keine externe Cargo-Abhaengigkeit**. Die bereits
existierenden transportneutralen Observation-/Protocol-Module werden als dieselben
physischen Rust-Quellen aus dem Repository eingebunden.

Der Broker ist fuer den beschlossenen lokalen Read-only-Observability-Vertrag klein
geschnitten:

- MQTT 3.1.1 CONNECT
- SUBSCRIBE
- retained QoS-0 PUBLISH von Watch zu Consumer
- PINGREQ/PINGRESP
- DISCONNECT
- `+`- und finales `#`-Topic-Matching
- kein eingehender PUBLISH-/Command-Kanal
- kein LAN-Bind
- keine Persistenz

Damit gibt es weiterhin keinen Rueckkanal von Watch nach EtuSchlopf.

## SessionSnapshotV1 MQTT Payload

Die Payload ist deterministisches UTF-8-JSON der bereits vorhandenen
`SessionSnapshotV1`-Projektion. `last_seen_age_millis` und
`last_state_change_age_millis` bleiben getrennt.

Unbeobachtete Sources werden nicht erfunden. Reannouncements aktualisieren den
Sessionzustand erneut retained, ohne die Source-Revision zu erhoehen.

## Noch nicht enthalten

Dieser Take enthaelt noch keine grafische Watch-Oberflaeche. Die UI wird im naechsten
Schritt als MQTT-Consumer an exakt denselben Broker-/Topic-Vertrag angebunden.
