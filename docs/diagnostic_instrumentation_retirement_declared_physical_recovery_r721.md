# r721 DiagnosticInstrumentationRetirementDeclaredPhysicalRecovery

## Authority und physischer Ausgangszustand

Der letzte autoritative dunkelgruene Parent bleibt r716. Der Arbeitsbaum ist physisch r717, weil r717 zwar technisch FinalExit 0 erreichte, durch seine eigene `cargo:warning=ETUSCHLOPF_KODEX_PHASE_MS ...`-Messausgabe aber yellow / audit-blocked wurde. r718 wurde vor Installation semantisch abgelehnt. r719 und r720 wurden ebenfalls vor Installation abgelehnt und werden weder installiert noch als Parent verwendet.

r721 behandelt r717 deshalb nicht als neue fachliche Authority. Der r717-Baum ist nur der aktuelle physische Transaktionszustand, gegen den Change-Control den vollstaendigen eingehenden Diff korrekt sehen muss.

## Vollstaendig deklarierte r717-Recovery

Gegen den physischen r717-Baum sind deshalb zwei Ruecknahmen ausdruecklich Bestandteil dieses Takes:

- `src/kodex_build_orchestrator.rs`: reine r717-`Instant`-/`cargo:warning`-Phasenmessung entfernen und die r716-Orchestratorbytes wiederherstellen,
- `docs/kodex_build_gate_phase_timing_evidence_r717.md`: aktive r717-Diagnoseevidenz aus dem Managed Tree loeschen.

Diese Pfade werden nicht versteckt und nicht als implizite Parent-Rekonstruktion behandelt, sondern im Take-Change-Manifest als tatsaechliche `modified`-/`deleted`-Aenderungen deklariert.

## Retirement der temporaeren r716-Diagnostik

Daneben wird nur die in r716 eingefuehrte integrierte Cargo-Diagnostik pensioniert:

- `CARGO_LOG=cargo::core::compiler::fingerprint=trace` entfernen,
- `cargo --timings` entfernen,
- den Timing-HTML-Anhang im Strict-Testlog entfernen.

Der bereits gruene r714/r715-Vertrag bleibt bestehen:

- `--locked`,
- `--profile strict`,
- `--all-targets`,
- `--no-fail-fast`,
- EtuSchlopf-Testtargets mit `opt-level=0`,
- separater Bootstrap-Targetroot `target/etuschlopf_bootstrap_cache`.

Keine Musikmutation, keine Writer-/Renderer-/Recovery-Semantik, kein Proof-Reuse, keine NegativeCheck-/Trust-Capability-Pensionierung und keine Reduktion der Testbreite.

## Tree-Integritaet

Root-`Cargo.lock` ist `LocalTransactional` und ist im eingehenden Take-Payload nicht enthalten. Ein verwalteter Lockfile-Pfad innerhalb eines qualifizierten Unterprojekts bleibt davon unberuehrt.
