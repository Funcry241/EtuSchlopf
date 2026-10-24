# PostRejectedTakeCorpusRecovery r979

## Ausgangslage

r977 `SoundingFrontierAuditRetirementCompileClosure` ist der letzte real akzeptierte dark-green Source-Stand. Der danach angebotene r978 `ReportRewriteScopeIndirectionRetirement` wurde beim realen Windows-All-Targets-Testlauf abgelehnt. Der geschuetzte Contract-Test `report_sanitizer_external_ingress_contract` bewies, dass `src/audit_only/report_sanitizer_rewrite_scope_projection.rs` keine tote Einwert-Indirektion, sondern die explizite Audit-Ownership-Grenze fuer opaque externe Metadaten ist. Der Take-Mechanismus rollte den Kandidaten daraufhin auf r977 zurueck.

## Zweck dieses Takes

r979 fuehrt keine fachliche Reparatur an r978 durch und uebernimmt keinerlei r978-Produktdelta. Er baut direkt auf r977 auf und aendert nur Repository-Metadaten. Dadurch entsteht ein eindeutiger neuer Physical Source fuer einen erneuten Ein-Befehl-Lauf `run.cmd --take --all`.

Der Take ist `MechanicalOnly`. Geaendert werden ausschliesslich `README.md`, `ZIP_KIND.txt` und dieses Dokument. Alle Rust-, CMD-, Cargo-, Test-, Audit-, Musik-, Layout-, Renderer-, Lineage-, Freeze- und Readback-Dateien bleiben byteidentisch zu r977. Insbesondere bleibt `src/audit_only/report_sanitizer_rewrite_scope_projection.rs` unveraendert erhalten.

## Invarianten

- keine Musikmutation;
- keine Layout- oder Renderer-Aenderung;
- keine Report-Sanitizer-Semantikaenderung;
- keine Authority-, Gate-, Writer-, Ledger-, Registry- oder Trust-Boundary-Aenderung;
- keine Testaenderung; erwarteter Strict-Testbestand bleibt 1.268;
- r978 ist rejected Failure Evidence und niemals Parent;
- Root-`Cargo.lock` bleibt ausgeschlossen.

## Acceptance

Der Take behauptet vor dem realen Lauf keinen Erfolg. Acceptance verlangt den echten Windows-Folgelauf mit `--all` und mindestens:

- Strict Clippy gruen;
- Build gruen;
- 1.268/1.268 Tests gruen;
- 10/10 Korpusprojekte OK und final-clean;
- `RunSeverity: dark_green`;
- `ReleaseDecision: released`;
- `ActiveRepairTarget: none`.

Erst dieser reale Lauf macht r979 zu einer neuen stabilen M0.
