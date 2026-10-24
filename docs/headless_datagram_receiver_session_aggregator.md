# Headless Datagram Receiver and Session Aggregator

## Scope

Take 2C der Observability Session Architecture. Dieser Take implementiert die passive
Empfangsseite fuer den bereits akzeptierten `SourceSnapshotV1`-Datagrammvertrag, aber noch
keine Watch-Anwendung, keinen MQTT-Broker und keine GUI.

## Receiver

`HeadlessSnapshotDatagramReceiver` bindet ausschliesslich den bestehenden Loopback-Port
`127.0.0.1:47621`, arbeitet nonblocking und dekodiert nur vollstaendige
`SourceSnapshotV1`-Datagramme. Unbekannte oder ungueltige Datagramme werden verworfen und
koennen keinen EtuSchlopf-Lauf beeinflussen. Der Receiver besitzt keinen Rueckkanal.

## Session Aggregator

`SessionAggregator` haelt pro `ObservationSessionId` und `SourceId` genau den letzten
gueltigen Snapshot. Die Annahmeregel ist der beschlossene latest-wins-Vertrag:

- hoehere Revision: State uebernehmen und `last_seen` sowie `last_state_change` erneuern,
- gleiche Revision + identischer Snapshot: nur `last_seen` erneuern,
- niedrigere Revision: ignorieren,
- gleiche Revision + anderer Payload: als semantischen Konflikt ablehnen,
- Role/Parent-Wechsel derselben Session-/Source-Adresse: als Source-Identity-Konflikt ablehnen.

Damit bleiben `last_seen` und `last_state_change` getrennte Beobachtungen. Es gibt keine
Eventhistorie, keine wachsende Telemetriequeue und keine Rekonstruktion verlorener
Zwischenrevisionen.

## Failure Envelope dieses Takes

Unit-Tests beweisen insbesondere:

- Reordering: Revision 3 vor Revision 2 konvergiert auf Revision 3,
- Verlust: 1 -> 3 ohne Revision 2 konvergiert auf Revision 3,
- Reannouncement: gleicher Snapshot aktualisiert nur `last_seen`,
- Same-Revision-Konflikt wird nicht uebernommen,
- Source-Identity bleibt innerhalb Session+Source stabil,
- mehrere Sessions bleiben getrennt,
- malformed Datagramme vergiften den Aggregator nicht.

Der bereits in v0.10.1726 bewiesene periodische Sender-Reannouncement liefert die
Gegenwarts-Konvergenz fuer einen spaet gestarteten Receiver; dieser Take fuegt auf der
Empfangsseite keine Historien- oder Gap-Recovery-Semantik hinzu.

## Nicht enthalten

Kein MQTT, kein Broker, keine GUI, kein SessionSnapshotV1, keine Persistenz, kein
Command-Channel, keine Musik-/Writer-/Render-/Release-Aenderung.
