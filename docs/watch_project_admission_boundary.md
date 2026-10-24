# Watch Project Admission Boundary

## Ziel

Take 3 benoetigt gemaess der beschlossenen Observability-Architektur eine eigenstaendige
`EtuSchlopf Watch` mit eigenem Cargo-Manifest und eigenen externen Dependencies. Der
bisherige Take-Change-Control akzeptiert Source unter `tools/`, lehnt aber ein dort
verschachteltes `Cargo.toml`/`Cargo.lock` im Semantic-Capability-Diff fail-closed ab.

Dieser SecurityOnly-Take oeffnet **keine allgemeine Nested-Cargo-Ausnahme**. Er admittiert
ausschliesslich die beiden exakten Pfade:

```text
tools/etuschlopf_watch/Cargo.toml
tools/etuschlopf_watch/Cargo.lock
```

Beide bleiben `OtherSource` und sind damit fuer spaetere `RefactorOnly`-Watch-Takes
erlaubt, nicht fuer `SecurityOnly`. Andere verschachtelte Cargo-Manifeste bleiben
fail-closed.

## Ein-Befehl-Beweis fuer kuenftige Watch-Takes

Wenn sich der Watch-Unterbaum eines eingehenden Takes aendert, fuehrt der
Rust-Take-Bootstrap **vor dem Kopieren** aus:

```text
cargo test --locked --profile strict --manifest-path tools/etuschlopf_watch/Cargo.toml
```

Der Cargo-Target-Ordner liegt ausserhalb des eingehenden Vollbaums. Dadurch werden weder
Take-Payload noch installierter Projektbaum durch Build-Artefakte veraendert. Fehlen
Watch-`Cargo.toml` oder Watch-`Cargo.lock`, startet Cargo nicht oder schlagen Tests fehl,
wird der Kandidat vor dem Kopieren abgelehnt.

Unveraenderte Watch-Projekte werden nicht erneut gebaut. Damit bleibt der normale
EtuSchlopf-Lauf unabhaengig von Broker/GUI/Watch-Dependencies.

## Nicht enthalten

Dieser Take enthaelt noch keine Watch-Anwendung, keinen MQTT-Broker, keine GUI und keine
neue Abhaengigkeit. Er aendert ausschliesslich die Security-/Take-Control-Grenze, die
notwendig ist, um das bereits beschlossene separate Watch-Projekt spaeter ueberhaupt
sauber durch die Ein-Befehl-Regel liefern zu koennen.
