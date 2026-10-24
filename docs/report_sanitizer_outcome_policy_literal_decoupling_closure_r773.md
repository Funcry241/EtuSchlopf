# r773 ReportSanitizerOutcomePolicyLiteralDecouplingClosure

## Parent

Ausschliesslicher Parent ist der real angenommene Stand
`0.10.1839 r772 ReportSanitizerRewriteScopeAuditOwnerBindingWriterCallerNeutralClosure`.

r768-r771 bleiben abgelehnte Failure-Evidence und besitzen keine Parent-Authority.

## Anlass

r772 hat die Rewrite-Scope-Ownership erfolgreich in einen bereits produktiv gelesenen
Audit-owned Zustand gelegt:

```text
ReportRewriteScope.external_metadata_opaque = false
```

Der bestehende externe Contract-Test pinnt jedoch noch den transienten Literalwert `false`.
Ein spaeterer `OutcomeProjection`-Cut `false -> true` koennte deshalb die korrekte Audit-Policy
nicht alleine umschalten, ohne gleichzeitig einen `tests/`-Pfad zu aendern. `OutcomeProjection`
darf diesen OtherSource-Pfad absichtlich nicht veraendern.

## r773-Schluss

r773 trennt deshalb Policy-Ownership von Policy-Wert:

```text
EXTERNAL_METADATA_OPAQUE: bool = false
ACTIVE_REPORT_REWRITE_SCOPE.external_metadata_opaque = EXTERNAL_METADATA_OPAQUE
```

Der aktive Wert bleibt `false`; die laufende Whole-Document-Sanitizer-Semantik bleibt damit
unveraendert.

Der externe Contract prueft jetzt nur noch:

- der Audit-Owner besitzt `EXTERNAL_METADATA_OPAQUE`,
- `ACTIVE_REPORT_REWRITE_SCOPE` bindet genau diesen Owner-Wert,
- `final_report.rs` besitzt die typisierte ExternalMetadata-Grenze.

Der Test pinnt bewusst nicht mehr den transienten Wahrheitswert der Outcome-Policy.

## Nicht geaendert

Keine aktuelle Reportausgabe, kein MusicXML, kein PDF, keine Musik-, Writer-, Reader-, Repair-,
Recovery-, Voice-, Tie-, Duration-, Beam-, Layout- oder Release-Semantik wird geaendert.

Nach realer Annahme von r773 darf ein spaeterer `OutcomeProjection` ausschliesslich den
Audit-owned Wert `EXTERNAL_METADATA_OPAQUE: false -> true` umschalten. `final_report.rs`,
Orchestrator, Kodex und externe Contract-Tests muessen dafuer unveraendert bleiben.
