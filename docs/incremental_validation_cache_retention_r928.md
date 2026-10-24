# r928 IncrementalValidationCacheRetentionClosure

## Zweck

r928 reduziert ausschliesslich die belegte Cold-Run-Kompilationslast nach einem Take.
Der reale r925-Etüde-3-Lauf zeigte:

- produktiver Strict-Build: ca. 117 s,
- Strict-Clippy: ca. 44.9 s,
- integrierter All-Targets-Testcompile: ca. 55.7 s,
- Basic-Pitch-Cold-Evidence: ca. 2.8 s.

Damit liegen mindestens 217.6 s in drei getrennten Cargo-Kompilationsphasen; der
evidence-only ONNX-Sensor ist nicht der Minutenverursacher.

## Aenderung

Der historische Post-Take-Package-Clean wird pensioniert und `CARGO_INCREMENTAL=1`
aktiviert. EtuSchlopf-Package-Artefakte in den beiden bereits autorisierten Cargo-Targetroots
bleiben als rein abgeleiteter Cargo-Cache erhalten.

Das veraendert keine Build-/Test-/Clippy-Breite:

- Strict-Clippy bleibt blockierend und unveraendert,
- der produktive strict/opt3-Build bleibt unveraendert,
- alle Cargo-Targets werden weiterhin mit no-fail-fast getestet,
- keine Testquelle und kein Test wird uebersprungen.

## Fail-Closed Reuse-Authority

Ein vorhandenes Produktbinary wird weiterhin nur dann direkt wiederverwendet, wenn alle
bestehenden Verifikationen passen:

1. Paketversion,
2. aktuelle Source-Build-Signatur,
3. gespeicherte Build-Signatur,
4. FNV64 der tatsaechlichen Executable.

Bei geaenderter Source-Signatur bleibt `CompileReason::MissingVerifiedArtifact` autoritativ:
Cargo muss bauen. Inkrementelle Units koennen nur dieses verpflichtende Bauen beschleunigen;
sie koennen es niemals ersetzen oder einen alten Binary-Stand legitimieren.

Cargo-Fingerprints bleiben die alleinige technische Entscheidung darueber, welche
Zwischenartefakte innerhalb eines angeforderten Builds wiederverwendbar sind.

## Recovery

Public-Artifact- und Take-Recovery bleiben vor jeder Kompilation unveraendert.
Transaktionsbackup, Installed-Tree-Readback, Rollback, Root-Cargo.lock-Resolution und
Acceptance-Provenienz bleiben unveraendert.

Der Cache ist kein Recovery-Traeger und keine Projektwahrheit.

## Nicht Bestandteil

Keine Aenderung an Musik, Solver, HAN03/SchlopfNarf, Writer, MusicXML, PDF, Renderer,
Basic Pitch, Schwellen, Warning-Policy oder Release-Entscheidung.


## r928 negative-check identity closure

The historical launcher test symbol `run_cmd_disables_cargo_incremental_compilation` is intentionally retained so the existing KodexNegativeCheck capability is modified in place rather than removed/relocated. Its assertions now enforce the current fail-closed cache-retention contract: Cargo incremental reuse is enabled, destructive package-only clean is retired, and binary reuse authority remains bound to source signature, Cargo fingerprint and executable FNV.
