# Watch Consumer Compile Hygiene

## Ausgangspunkt

EtuSchlopf Watch v0.10.1732 bindet mehrere gemeinsame Observation-Dateien per
`#[path]` ein. Dadurch kompiliert das Watch-Binary auch Producer-only Typen und
Funktionen, die es nie verwendet. Der sichtbare Effekt sind zahlreiche
`dead_code`-Warnungen beim normalen Watch-Start.

## Schnitt

Die gemeinsame Boundary wird in drei Verantwortungen getrennt:

1. `observation_protocol.rs`
   - Wire-DTOs
   - SourceSnapshotV1-Decoder
   - Rollen-/Status-/Stage-Validierung

2. `observation_snapshot_projection.rs`
   - interne SourceState -> SourceSnapshotV1 Projektion
   - nur EtuSchlopf-Produzentenseite

3. `observation_protocol_encoder.rs`
   - deterministische SourceSnapshotV1-v1-Kodierung
   - nur EtuSchlopf-Produzentenseite

Die Watch bindet nur noch `observation_protocol.rs`,
`observation_receiver.rs` und `observation_session_aggregator.rs` ein.
`observation.rs` und `observation_transport.rs` verschwinden aus dem
Watch-Compilegraph.

Methoden, die ausschliesslich Tests dienen (`local_addr`, einzelne
Aggregator-Inspektionshelfer), werden mit `#[cfg(test)]` auf Test-Builds
begrenzt.

## Invarianten

- Wire-Bytes bleiben unveraendert.
- `SourceSnapshotV1` bleibt ein gemeinsamer physischer Vertrag.
- Keine Suppression mit `#[allow(dead_code)]`.
- Keine neue Cargo-Abhaengigkeit.
- Keine Musik-, Launcher-, MQTT- oder Broker-Semantik wird geaendert.
- Watch bleibt reiner Consumer der Observation-Welt.

## Standalone-Start

`tools/etuschlopf_watch/run.cmd` baut die Watch mit `--quiet` und startet die
fertige EXE direkt. Cargo-Laufzeitbanner werden dadurch nicht mehr mit der UI
vermischt.


## Generated-target isolation (r02)

Der Standalone-Start verwendet ein explizites Cargo-Target-Verzeichnis ausserhalb des
verwalteten `tools/`-Source-Baums:

`target/etuschlopf_watch_runtime`

Damit erzeugt ein normaler Watch-Start keine Dateien mehr unter
`tools/etuschlopf_watch/target`. Der r02-Take deklariert die von v0.10.1732 bereits
erzeugten lokalen Target-Dateien einmalig als geloeschte Buildartefakte, sodass die
Ein-Befehl-Regel erhalten bleibt und keine manuelle Bereinigung erforderlich ist.


## Receiver compile closure (r04)

Der Watch-Preflight von r03 zeigte zwei verbliebene Producer-Kanten im gemeinsam
eingebundenen Receiver: die Loopback-Port-/Datagrammgroessen-Konstanten lagen noch in
`observation_transport.rs`, und die Receiver-Tests importierten den produktiven Encoder.

Die beiden Transportvertragskonstanten liegen nun in `observation_protocol.rs` und werden
von Sender und Receiver gemeinsam gelesen. Die Receiver-Tests erzeugen ihre kleine
wohlgeformte Datagramm-Fixture lokal und ziehen dadurch weder
`observation_protocol_encoder.rs` noch `observation_transport.rs` in den Watch-Testgraph.

Damit bleibt der normale und der Test-Compilegraph der Watch auf der Consumer-Seite; es
gibt weiterhin keine `dead_code`-Suppression und keine neue Cargo-Abhaengigkeit.
