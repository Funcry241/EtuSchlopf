# r736 – UnreachableArticulationWriterRootRetirementClippyClosure

## Ausgangspunkt

r734 ist der letzte dunkelgruene/released Parent. r733 wurde bereits vor
Installation vom Semantic-Capability-Diff abgewiesen. r735 wurde danach auf
r734 fachlich korrekt neu aufgebaut und erreichte den Strict-Clippy, scheiterte
dort aber ausschliesslich an zwei durch das Retirement freigelegten
Dead-Code-Resten. r735 ist deshalb ebenfalls kein Parent.

## Unveraenderte Retirement-Absicht

r736 pensioniert dieselben zwei statisch unerreichbaren Writer-Roots wie der
r735-Kandidat:

1. `LegacyLowerStaccatoProxySuppressRenderWrite`: Die ausschliesslich
   selektierte Klasse `legacy_lower_staccato_proxy` kann vom aktuellen Producer
   nicht mehr erzeugt werden; der historische Proxy-Producer ist bereits
   pensioniert.
2. `AssertiveVisibleArticulationRenderWrite`: Die einzige produktive
   Aufrufstelle war dauerhaft mit `user_gate_open=false` gebunden.

Ihre exklusiven Sinks, Checkpoints, Compatibility-/Pilot-Counter und
Reporttransporte werden gemeinsam pensioniert.

## Clippy-Closure

Der Strict-Clippy von r735 meldete exakt zwei Restklassen:

- `src/wav_truth_articulation.rs`: Import `yes_no` ist nach Entfernung der
  alten Textprojektion unbenutzt.
- `AssertiveVisibleArticulationRenderWriteAudit::candidate_count`: Das Feld
  wird von keinem lebenden Consumer mehr gelesen. Die zwei noch lebenden
  Nachfolger `WavTruthBindingClearAccentRenderWrite` und
  `AccentReserveTopSixMicroWriteCandidate` initialisierten es lediglich noch.

r736 entfernt daher nur den Import, dieses Feld und seine beiden
Initialisierungen. Die Nachfolger selbst, ihre Auswahl, Gates, Counts, Writes
und Readbacks bleiben unveraendert.

## Proof-Reentry-Grenze

Der in r734 isolierte Reader
`readback_primary_staccato_boundary_artifact` bleibt gegen r734 textidentisch.
Damit veraendert r736 keine Proof-Reentry-Capability.

## Bilanz

- Parent: r734, dark_green/released.
- r735: Strict-Clippy failed, kein Parent.
- Rust r734: 199.321.
- Rust r736: 198.679.
- Netto dieses vollstaendigen Retirement-Cuts gegen r734: -642.
- Feste 60k-Baseline: 203.261.
- Korrektes kumuliertes physisches Retirement nach Erfolg: -4.582 / -60.000.
