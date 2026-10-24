# TakeBootstrapArchitectureGateMandatoryRetirementClosure r253

## Ausgangspunkt

r253 basiert direkt auf dem real dunkelgruenen Vollkorpusstand r251 `WholeRestCandidateRewriteAuthorityVocabularyClosure`. Der r252-Kandidat wurde vor jeder Installation durch den bestehenden Semantic-Capability-Diff abgelehnt und ist kein Elternstand.

## ε-nativer Security-Befund

Der bisherige Vertrag erlaubte ausschliesslich der Take-Bootstrap-Kompilation, das produktive Architecture-Gate ueber `ETUSCHLOPF_ARCHITECTURE_GATE_CONTEXT=take_bootstrap_compile` zu ueberspringen. Die Enforcement-Funktion `take_bootstrap_compile_isolated` pruefte jedoch nur Context, Debug-Profil und einen Bootstrap-Target-Pfadmarker; die Identitaet des tatsaechlich kompilierten Binaries war nicht Bestandteil der strukturellen Autorisierung. Eine speziell vorbereitete lokale Cargo-Kompilation konnte dadurch denselben Ausnahmezustand beanspruchen.

## Closure

- `run.cmd` setzt keinen `ETUSCHLOPF_ARCHITECTURE_GATE_CONTEXT` mehr.
- `src/security_build_orchestrator.rs` besitzt weder `take_bootstrap_compile_isolated` noch einen Bootstrap-spezifischen Architecture-Gate-Early-Return.
- Bootstrap- und produktive Kompilation durchlaufen dasselbe verpflichtende Architecture-Gate.
- `tests/bootstrap_gate_isolation.rs` bindet den neuen Negativvertrag: Sondercontext und isolierter Marker muessen fehlen; die normale Architecture-Gate-Validierung muss vorhanden bleiben.
- Assistant-Export und Recovery-Security projizieren den neuen Vertrag wahrheitsgemaess.

## Exakte SecurityCapabilityRetirements

Der bestehende Semantic-Capability-Diff inventarisiert sowohl die entfernte Enforcement-Funktion als auch den alten Negativtest als `SecurityNegativeCheck`. Beide werden deshalb mit ihren exakten r251-Rejection-Surface-Fingerprints pensioniert:

```text
SecurityNegativeCheck|src/security_build_orchestrator.rs|take_bootstrap_compile_isolated#1|fnv64:92d9ea0d11828812
SecurityNegativeCheck|tests/bootstrap_gate_isolation.rs|take_bootstrap_gate_isolation_is_narrow_and_recovery_free#1|fnv64:d1661b3895dd59db
```

Der zweite Eintrag ist die Recovery-Ergaenzung gegen den r252-Pre-Take-Gate-Befund. Er schwaecht den Gate-Vertrag nicht, sondern benutzt exakt dessen vorhandene append-only Retirement-Authority.

## Nicht-Ziele

Keine Musik-, MusicXML-, MXL-, Writer-, Renderer-, Solver-, Layout-, Calibration-, Dependency-, Cargo.lock-, Toolchain-, Repair- oder Selection-Authority-Aenderung. Kein neuer Security-Owner und keine neue Ausnahme.
