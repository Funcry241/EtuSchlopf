# Watch MQTT Terminal UI Surface

Version: 0.10.1732

## Scope

Dieser Take fuegt ausschliesslich die erste sichtbare Human Surface der separaten
EtuSchlopf Watch hinzu. Musikpipeline, Launcher, Observation-Domain, SourceSnapshotV1,
UDP-Sender, Receiver, SessionAggregator und eingebetteter Broker bleiben unveraendert.

## Contract path

```text
EtuSchlopf Sources
    -> loopback UDP SourceSnapshotV1
    -> Watch SessionAggregator
    -> retained MQTT SessionSnapshotV1 JSON
    -> Watch MQTT client
    -> Terminal UI
```

Die UI besitzt keinen Direktzugriff auf den Aggregator. Sie verbindet sich mit
`127.0.0.1:47622`, abonniert `etuschlopf/v1/session/+/state` und dekodiert ausschliesslich
den oeffentlichen JSON-Vertrag. Damit beweist die eigene Watch praktisch dieselbe
Consumer-Grenze, die spaeter auch weitere lokale MQTT-Consumer verwenden koennen.

## Human surface

Die Anzeige bleibt klein und ruhig. Pro Session zeigt sie:

- Sessionstatus ohne erfundene Health-Aussage
- Corpus `completed / total` und aktive Projektzahl, falls beobachtet
- Launcher-Stage, falls beobachtet
- alle tatsaechlich beobachteten Project-Sources
- Activity vor Stage, wenn eine Activity vorliegt
- `last_seen` getrennt von `last_state_change`

Fehlende Sources werden nicht als Pending-Zeilen erfunden. Eine spaet gestartete Watch
darf deshalb zunaechst eine partielle Session anzeigen und konvergiert mit den naechsten
Reannouncements.

## MQTT client

Der Client implementiert nur die fuer die read-only Watch benoetigte MQTT-3.1.1-Teilmenge:
CONNECT, SUBSCRIBE QoS 0 und eingehende PUBLISH-Pakete. Er akzeptiert nur einen
Loopback-Broker. Bei lokaler Trennung versucht die UI spaeter erneut zu verbinden; dies
hat keinen Einfluss auf Receiver, Broker oder EtuSchlopf.

## JSON decoding

Der bereits deterministische `SessionSnapshotV1`-Encoder erhaelt einen passenden
read-only Decoder. Der Decoder validiert Protokollversionen und Sessionidentitaet und
ist tolerant gegen JSON-Feldreihenfolge und Whitespace. Er erzeugt keine neue
Runtime-Wahrheit, sondern rekonstruiert nur den veroeffentlichten Vertrag fuer die
Darstellung.

## Dependency firewall

Die Watch bleibt ein eigenes Cargo-Projekt und hat weiterhin keine externe
Cargo-Abhaengigkeit. Der EtuSchlopf-Musikkern wird in diesem Take nicht geaendert.

## Start

```powershell
.\tools\etuschlopf_watch\run.cmd
```

Die Watch startet Receiver, eingebetteten Broker und den eigenen MQTT-UI-Client in
diesem separaten Prozess.
