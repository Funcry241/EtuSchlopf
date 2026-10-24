# r714 AllTargetsTestCompileProfileCutover

## Zweck

r714 ist ein enger `KodexOnly`-Performance-Cut auf dem real gruenen Parent r713.
Er reduziert ausschliesslich die Codegen-Kosten der integrierten EtuSchlopf-Testtargets.

Die Testbreite bleibt unveraendert:

- `cargo test`
- `--locked`
- `--profile strict`
- `--all-targets`
- `--no-fail-fast`
- gleicher blockierender Exit
- gleiche Testquellen und Testlogik

Nur fuer das Package `etuschlopf` wird innerhalb dieses Testprozesses `opt-level=0`
ueber Cargo-Konfiguration gesetzt. Der produktive EtuSchlopf-Build bleibt `strict` / releaseartig / opt3.

Die Testartefakte werden physisch getrennt vom produktiven `target/strict` in der bereits
recovery-autorisierten Root `target/etuschlopf_bootstrap_cache` materialisiert. Es wird keine dritte
Cache-Authority eingefuehrt.

## Exakter Testprozess

```text
cargo --config profile.strict.package.etuschlopf.opt-level=0 test --locked --profile strict --all-targets --no-fail-fast --jobs <N> --target-dir target/etuschlopf_bootstrap_cache
```

## Toolchain

Die technische Projektbaseline wird durch `rust-toolchain.toml` auf Rust/Cargo 1.98.0 gepinnt.
`Cargo.toml` `rust-version = "1.88"` ist die MSRV-Aussage und nicht die ausgefuehrte Toolchainbaseline.

## Nicht Bestandteil

Keine Aenderung an:

- `Cargo.toml`
- `Cargo.lock`
- `run.cmd`
- Take-Bootstrap
- Build-Orchestrator
- Produktprofil
- Clippy-Profil oder Clippy-Breite
- Testauswahl oder Testquellen
- Pipeline / Musiklogik / Writer
- Renderer / PDF / MusicXML
- Recovery- oder Successful-Take-Cleanup
- `CARGO_INCREMENTAL`
- Linker / LTO / codegen-units
- Assistant-Export

## Beweisgrenze

Die Testbreite ist gleich, aber die Maschinenrepraesentation der EtuSchlopf-Testtargets ist nicht
bitidentisch zu opt3. Ein ausschliesslich optimierungssensitiver Test-Harness-Fehler bleibt theoretisch
moeglich. Das reale Produkt wird weiterhin strict/opt3 gebaut und ausgefuehrt.

Performance ist Observation, keine Release-Authority. Der erste Lauf im Bootstrap-Targetroot kann
noch Cold-Cache-Kosten enthalten; ein warmer Folge-Take ist der relevante Steady-State-Vergleich.

## Erwartete Sourceflaeche

Produktiver/Kodex-Rust-Diff nur:

- `src/bin/etuschlopf_launcher.rs`
- `src/build_gate/pinned_policy_kodex.rs`
- `src/identity_gate_trust_anchor.rs`

plus Take-Metadaten und diese Dokumentation.
