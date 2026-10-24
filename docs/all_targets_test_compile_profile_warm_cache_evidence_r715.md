# r715 AllTargetsTestCompileProfileWarmCacheEvidence

## Zweck

r715 ist der bewusst funktionslose zweite Performance-Take auf dem real dunkelgruenen r714.
Er erzeugt einen neuen Take-/Reexec-Zyklus, ohne die in r714 eingefuehrte Testmaterialisierung erneut zu aendern.

## Was bewiesen werden soll

r714 hat den neuen Bootstrap-Targetroot fuer den Strict-All-Targets-Test im ersten Lauf aufgewaermt.
Der normale Successful-Take-Cleanup entfernt dort EtuSchlopfs Package-Artefakte, behaelt aber Dependency-Artefakte.
Damit soll r715 zeigen, wie teuer derselbe r714-Testvertrag im warmen Steady State ist.

Unveraendert bleibt der Testprozess aus r714:

```text
cargo --config profile.strict.package.etuschlopf.opt-level=0 test --locked --profile strict --all-targets --no-fail-fast --jobs <N> --target-dir target/etuschlopf_bootstrap_cache
```

## Keine Funktionsaenderung

Byteidentisch zu r714 bleiben insbesondere:

- `src/bin/etuschlopf_launcher.rs`
- `src/build_gate/pinned_policy_kodex.rs`
- `src/identity_gate_trust_anchor.rs`
- Produkt-/Pipeline-/Writer-Code
- Testquellen und Testauswahl
- Recovery- und Cache-Cleanup-Code
- Cargo- und Produktprofile

Es gibt keine Musikmutation, keine neue Writer-/Repair-/Selection-Authority und keine neue Performancevariable.

## Auswertung

Performance ist weiterhin Observation und kein Release-Gate. Nach dem realen Windows-Lauf werden mindestens verglichen:

- Cargo `Finished strict profile ...` der integrierten All-Targets-Tests
- `IntegratedLauncherTestMs`
- 1285/1285 Tests bzw. die dann aktuelle unveraenderte Testbreite
- `FinalExit`, `RunSeverity`, `ReleaseDecision`

Wenn r715 deutlich unter dem kalten r714-Testcompile liegt, ist der r714-Cut als Steady-State-Hebel belegt. Wenn nicht, folgt keine weitere Vermutung: Dann wird mit Cargo-Fingerprint-Evidence die verbleibende Invalidierungsursache bestimmt.
