# SessionSnapshotV1 Projection Boundary

## Zweck

Take 3A projiziert den bereits akzeptierten `SessionAggregator` in einen kleinen,
transportneutralen `SessionSnapshotV1`. Es entsteht keine neue Runtime-Wahrheit.

## Vertrag

`SessionSnapshotV1` enthaelt:

- `protocol_version`,
- `observation_session_id`,
- die aktuell beobachteten `SourceSnapshotV1` in deterministischer SourceId-Reihenfolge,
- pro Source `last_seen_age_millis`,
- pro Source `last_state_change_age_millis`.

Die Alterswerte sind reine Watch-Projektion. Sie veraendern weder Source-Revision noch
SourceState. Reannouncement kann `last_seen` auffrischen, ohne `last_state_change` zu
veraendern.

## Verlustsemantik

Eine Session-Projektion erfindet keine fehlenden Sources. Startet die Watch spaet oder
gehen Datagramme verloren, enthaelt der Snapshot nur bereits beobachtete Sources und
konvergiert mit spaeteren Reannouncements.

## Nicht enthalten

Kein MQTT, Broker, GUI, JSON-Wire-Format, Eventfeed, Persistenz, Rueckkanal oder
EtuSchlopf-Schreibrecht. SourceSnapshotV1-Datagrammformat und Musikpipeline bleiben
unveraendert.
