# r354 KeySignatureEnharmonicSpanWriteAuthorityRetirementStrictClippyRecovery

Parent: r351 `KeySignatureEnharmonicSpanLegacyAuthorityEvidence` (real GREEN/released).

r352 wurde vom `NO-NEW-REPAIR-AUTHORITY SECURITY GATE` abgelehnt und nie installiert.
r353 behob diese Security-Reachability, wurde jedoch im anschliessenden Strict-Clippy-Lauf
nicht freigegeben. `#![deny(dead_code)]` meldete ausschliesslich tote Datenfelder in den nach
der Authority-Pensionierung zu breit gebliebenen KeySignatureEnharmonicSpan-Audit-Strukturen.
Die 132 uebrigen Clippy-Meldungen blieben diagnostisch (`architectural_budget: 0`).

r354 basiert deshalb erneut direkt auf r351 und enthaelt die Security-saubere Retirement-
Grenze aus r353 plus die mechanische Dead-Code-Closure:

- die dedizierte `KeySignatureSpan`-StructuralWriteAuthority-Capability bleibt entfernt;
- der Orchestrator besitzt weiterhin keinen spaeten Span-Sink;
- es gibt weiterhin keinen hypothetischen Key-Signature-Change-/Restore-Plan;
- `KeySignatureEnharmonicSpanRow` behaelt nur noch die fuer die read-only Klassifikation
  tatsaechlich konsumierte `decision`-Information;
- `KeySignatureEnharmonicSpanAudit` behaelt nur noch die drei aggregierten Zaehler;
- `KeySignatureEnharmonicSpanRenderWriteAudit` verliert die nach Retirement unbenutzten
  Writer-Kontextfelder `source_label`, `output_path`, `target_bound`, `done_fragment` und
  `pipeline_line`;
- die observation-only Audit-Funktion braucht deshalb nur noch die XML-Beobachtung;
- die vier Regressionstests pruefen unveraendert: kein Materialisierungsplan, keine
  Materialisierung, Invarianten stabil, Owner-State pensioniert;
- keine `#[allow(dead_code)]`-Ausnahme wird eingefuehrt.

Architektur nach r354:

```text
KeySignatureEnharmonicSpelling
        -> aktive tonale DecisionAuthority
        -> bestehende Render-/Master-Materialisierung

KeySignatureEnharmonicSpan
        -> Span-/Tonal-Beobachtung
        -> MaterializationPlan none
        -> StructuralWriteAuthority none
        -> RenderXmlWrite nein
        -> keine tote Writer-Datenstruktur mehr
```

Der Take ist `RefactorOnly`. Keine Passorder-, Tonalregel-, Threshold-, Pitch-, Staff-, Voice-,
Onset- oder Dauerentscheidung wird geaendert. Erwartetes Master-/Render-/PDF-Delta: 0. Der
integrierte Testbestand soll 1092/1092 bleiben.
