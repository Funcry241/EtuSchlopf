# r774 – ExternalMetadataOpaqueTransportClosure

## Authority

r774 basiert ausschliesslich auf dem real akzeptierten r773 `ReportSanitizerOutcomePolicyLiteralDecouplingClosure`. r768-r771 bleiben verworfene Kandidaten; r772 und r773 sind die akzeptierte Vorbereitungslinie.

## Semantik-Cut

Der Audit-Owner

```text
src/audit_only/report_sanitizer_rewrite_scope_projection.rs
```

schaltet genau

```text
EXTERNAL_METADATA_OPAQUE: false -> true
```

um. `ACTIVE_REPORT_REWRITE_SCOPE` bleibt unveraendert an diesen Owner gebunden.

Damit transportieren die bereits in r772/r773 vorhandenen typisierten Final-Report-Grenzen Fragmente mit `ReportFragmentOrigin::ExternalMetadata` opaque durch Report-Compaction und spaetere Whole-Document-Rewrites. Interne Fragmente mit `InternalReportVocabulary` bleiben unveraendert unter den bestehenden Report-Sanitizern.

## Was unveraendert bleibt

- kein neuer Writer-Caller, Writer-Root oder Repair-/Recovery-Owner,
- keine Aenderung an `src/final_report.rs`, Orchestrator oder Kodex,
- keine Aenderung an externen Contract-Tests,
- keine MusicXML-, PDF-, Musik-, Voice-, Tie-, Duration-, Beam- oder Layout-Semantik,
- keine neue Sanitizer-Regel und keine entfernte interne Sanitizer-Regel.

Der aktuelle Titel `Partitur: Etüde Nr. 6` enthaelt keinen der bestehenden Sanitizer-Tokens; fuer diesen konkreten Lauf wird deshalb kein Reportbyte-Unterschied erwartet. Der semantische Abschluss besteht darin, dass kuenftige bzw. andere extern gelieferte Score-Titel nicht mehr von interner Legacy-Berichtssprache umgeschrieben werden duerfen.

## ChangeClass

`OutcomeProjection`: Die einzige produktive Source-Aenderung liegt im dedizierten Audit-Owner. Der vorhandene typisierte Transport und seine Writer-/Reportgrenzen bleiben unveraendert.
