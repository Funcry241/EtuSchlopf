# v0.10.1726 – Latest Snapshot Datagram Boundary (Project + Corpus)

## Scope

Take 2B der Observability Session Architecture bindet die in v0.10.1725
festgelegte `SourceSnapshotV1`-Projection erstmals an einen lokalen
best-effort-Datagrammtransport.

Dieser Take umfasst ausschließlich Project und Corpus. Der Launcher bleibt wegen
der bestehenden `SecurityOnly`-Grenze unverändert und folgt separat auf dem
akzeptierten Stand.

## Transport

- IPv4 Loopback: `127.0.0.1`
- UDP-Port: `47621`
- keine Verbindung, kein Connect-/Reconnect-Lifecycle
- keine ACKs, keine Queue historischer Events
- keine externe Abhängigkeit
- maximale Datagrammgröße: 8 KiB
- Transportfehler werden verworfen und ändern keinen RunStatus

`SnapshotDatagramPublisher` wird separat vom `SourceStateOwner` gehalten.
Semantische State-Änderungen sind zuerst vollständig abgeschlossen. Erst danach
wird `SourceSnapshotV1` projiziert, codiert und best effort gesendet.

## Latest state wins

Der Publisher hält genau den zuletzt codierten Snapshot einer Source. Neue
Snapshots ersetzen diesen Speicher; die Größe wächst nicht mit der Laufdauer.

Ein interner Reannouncement-Thread sendet den unveränderten neuesten Snapshot
periodisch erneut. Dabei wird keine neue Observation-Revision erzeugt. Der
Thread besitzt keine musikalische Autorität und beendet sich, sobald sein
Publisher nicht mehr existiert.

Beim Reannouncement wird der Snapshot unter einem kleinen lokalen Mutex nur
kopiert. `send_to()` läuft danach ohne gehaltenen Snapshot-Lock.

## Failure isolation

Ein fehlender Receiver, ein nicht nutzbarer Socket, ein fehlgeschlagener
Datagrammversand oder ein nicht startbarer Reannouncement-Thread bleiben
ausschließlich ein Beobachtungsverlust. Die Pipeline erhält daraus keinen
Fehlerwert.

Tests decken insbesondere ab:

- Receiver fehlt: Project-State kann weiter bis `Completed` laufen.
- Reannouncement: derselbe aktuelle Snapshot erscheint erneut mit derselben
  Revision.

## Nicht enthalten

- Launcher-Datagramm-Hook
- Datagramm-Receiver / Session-Aggregator
- MQTT
- Watch UI
- Eventstream / Event Store
- Command Channel
- MusicXML/PDF/Report-Transport
- musikalische Änderung

## Abnahme

Zuerst gezielt:

```powershell
.\run.cmd --take --strict "Etüde Nr. 6"
```

Ein Vollkorpus ist für diese senderseitige Compile-/Failure-Closure noch nicht
das erste Gate.
