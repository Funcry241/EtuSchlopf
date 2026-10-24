# r716 IntegratedTestCargoInvalidationTimingEvidence

## Zweck

r716 ist ein reiner `KodexOnly`-Evidenz-Take auf dem real dunkelgruenen r715. Er veraendert keine Performancearchitektur, sondern macht die verbleibende Cargo-Invalidierung des integrierten All-Targets-Testgates beobachtbar.

## Unveraenderter Testvertrag

Weiterhin laufen dieselben Tests mit:

```text
cargo --config profile.strict.package.etuschlopf.opt-level=0 test --locked --profile strict --all-targets --no-fail-fast --jobs <N> --target-dir target/etuschlopf_bootstrap_cache
```

Es gibt keinen Testskip, keine Testauswahl, keine Aenderung an Testquellen und keine Aenderung am produktiven strict/opt3-Build.

## Neue Diagnoseevidenz

Nur fuer diesen integrierten Cargo-Testprozess gilt zusaetzlich:

```text
CARGO_LOG=cargo::core::compiler::fingerprint=trace
--timings
```

Der Fingerprint-Trace wird bereits ueber stderr in `visual_preflight.strict-tests.txt` erfasst. Cargo schreibt den HTML-Timingreport in den Bootstrap-Targetroot. Nach der normalen Cargo-Testzusammenfassung haengt der Launcher diesen Report mit klaren BEGIN/END-Markern an dasselbe Strict-Testlog, damit der normale Assistant-Export die Evidenz ohne neue Export-Authority mitliefert.

Die Testzusammenfassung wird bewusst vor dem HTML-Anhang bestimmt. Der Diagnoseanhang kann daher die Zaehlung bzw. den Teststatus nicht veraendern.

## Beweisziel

Nach dem realen Windows-Lauf sollen insbesondere beantwortet werden:

- welche Cargo-Units dirty bzw. fresh sind,
- welche konkreten Fingerprint-Ursachen Cargo nennt,
- ob Take-spezifische Build-Script-/rustc-env-Eingaben den Produkt-/Testgraph invalidieren,
- welche Units die meiste Compile-/Linkzeit beanspruchen,
- welche Rolle `schemaschlopf` und der Full-Tree-Apply spielen,
- ob Successful-Take-Clean, Source-mtime oder Build-Script-Provenienz der groessere naechste Hebel ist.

## Nicht Bestandteil

Keine Aenderung an `run.cmd`, Take-Bootstrap, `Cargo.toml`, `Cargo.lock`, Build-Orchestrator, Successful-Take-Cleanup, Full-Tree-Apply, CARGO_INCREMENTAL, Linker, Produktprofil, Pipeline, Writer, Renderer oder Assistant-Export.

Die r716-Zeit ist wegen Trace-/Timing-I/O keine Performance-Baseline. r716 ist ausschliesslich ein Diagnosebeweis.
