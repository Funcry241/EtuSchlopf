# TakeBootstrapFailClosedExitNormalizationPinCoordinationClosure – v0.10.1839 r16

## Anlass

Der Deep-Clippy-Hinweis in `src/bin/etuschlopf_take_bootstrap.rs` auf `i32 -> u8` ist kein bloßer Stilhinweis.
Der Take-Bootstrap übernimmt den Exitcode des nachgelagerten `run.cmd` als `i32`, entscheidet damit korrekt über Commit (`0`) oder Rollback (`!= 0`) und gibt den Code anschließend über `ExitCode::from(code as u8)` an den aufrufenden Ein-Befehl-Workflow zurück.

Dadurch konnte ein nichtnulliger Exitcode außerhalb `0..=255`, insbesondere `256` oder ein anderer nichtnulliger Wert mit niedrigem Byte `0`, beim äußeren Prozessstatus zu `0` werden. Die Source-Transaktion selbst blieb zwar korrekt zurückgerollt, aber `run.cmd` konnte den Bootstrap anschließend fälschlich als erfolgreich sehen.

## Änderung

Der äußere Bootstrap-Exit wird nun fail-closed normalisiert:

- `0` bleibt `0`.
- `1..=255` bleiben unverändert.
- negative Werte und Werte `>255` werden auf `1` abgebildet.
- ein nichtnulliger Reexec-/Child-Exit kann damit niemals zu äußerem Erfolg `0` werden.

Ein fokussierter Regressionstest deckt `0`, `2`, `255`, `256`, `-1` und `i32::MIN` ab.


## Pin-Koordination nach Pre-Copy-Ablehnung

Der vorherige r15-Paketversuch wurde vom bestehenden Take-Change-Control korrekt vor jeder Source-Mutation abgelehnt:
`src/bin/etuschlopf_take_bootstrap.rs` ist über `include_bytes!` als gepinnter Architekturpfad geschützt und darf nicht ohne die beiden synchronisierten Pin-Eigentümer `src/identity_debt_build.rs` und `src/identity_gate_trust_anchor.rs` geändert werden.

r16 erfüllt diese bestehende Schutzregel explizit:

- `src/identity_debt_build.rs` pinnt den neuen Bootstrap-FNV1A64 `827ed0523115b70d`.
- Dadurch ändert sich der Build-Policy-FNV1A64 auf `12be209951d00413`.
- `src/identity_gate_trust_anchor.rs` pinnt beide neuen Werte compile-time und in seinem Regressionstest.
- Die Pin-Koordination ändert weder Change-Control noch Security-Gates; sie aktualisiert ausschließlich die vorhandene Vertrauenskette für die bewusst deklarierte SecurityOnly-Änderung.

## Nicht geändert

- keine Musikmutation
- kein Layout / PageCount / Width
- keine Writer- oder Authority-Änderung
- keine Take-Commit-/Rollback-Entscheidungslogik
- keine Gate-Lockerung
- keine Renderer-/PDF-Änderung

ChangeClass: `SecurityOnly`. Parent: `0.10.1839 r14`; r15 wurde vor Copy abgelehnt und ist kein installierter Parent.
