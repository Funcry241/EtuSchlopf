# Launcher Watch One-Command Boundary

Version: 0.10.1734

## Ziel

`--watch` ist ein Launcher-Schalter und kein Argument der Musikpipeline.

Der Aufruf

```powershell
.\run.cmd --watch --strict "Etüde Nr. 6"
```

startet die lokale EtuSchlopf Watch best effort in einem eigenen Konsolenfenster
und fuehrt danach denselben musikalischen Lauf aus wie ohne `--watch`.

## Invarianten

- `--watch` wird nicht an das Musik-Binary weitergereicht.
- Die Watch wird aus `tools/etuschlopf_watch/Cargo.toml` mit `--locked --profile strict`
  in `target/etuschlopf_watch_runtime` gebaut.
- Watch-Build oder Watch-Start duerfen den musikalischen Lauf nicht fehlschlagen lassen.
- Die Watch bleibt ausserhalb der musikalischen Authority.
- Die Watch wird nach terminalem Launcher-Snapshot best effort beendet.
- `run.cmd`, SourceSnapshotV1, UDP, MQTT, Broker, Watch-Code und Musikcode bleiben unveraendert.
- `set_phase()` bleibt reine Praesentation.

## Bedienvertrag

Ein Benutzer benoetigt fuer einen beobachteten Lauf nur einen Befehl:

```powershell
.\run.cmd --watch --strict "Etüde Nr. 6"
```

Das bestehende `run.cmd` bleibt Owner des Gesamtstarts; die Interpretation von
`--watch` liegt ausschliesslich im Rust-Launcher.

## r02 Constructor Closure

Der nach dem r01-Take vom Reexec-Compile gefundene zweite `ExecutionRequest`-Konstruktor
`promote_post_take_args_to_strict_all()` setzt nun explizit `watch: WatchPlan::Disabled`.
Damit bleibt ein post-Take erzwungener Strict-Korpuslauf frei von einer implizit gestarteten Watch.
Der zugehoerige Test beweist zusaetzlich `!args.watch_requested()`.

Alle `ExecutionRequest { ... }`-Konstruktorstellen wurden geschlossen geprueft.
