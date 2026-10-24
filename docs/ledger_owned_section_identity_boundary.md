# LedgerOwnedSectionIdentityBoundary

## Zweck

v0.10.1694 beseitigt die zweite manuell gepflegte formale Section-Identitaet fuer die zwei realen Problemgrenzen des Superwurfs.

`CompactedReport::project_owned_section` besitzt Section-ID und Underline und materialisiert die formale Grenze selbst. Der jeweilige Producer liefert nur noch den Body.

Pilotiert werden bewusst nur:

- `REPAIR-LOOP-INVOCATION-EVIDENCE`,
- `MUTATION-SEMANTICS-AUTHORITY`.

Der bestehende fail-closed Projection-Ledger bleibt erhalten: exakt eine erwartete Sektion, keine undeplizierte Zusatzsektion, kein partieller Vertragszustand. Zwei Regressionstests pruefen ledger-owned Materialisierung und das Verbot einer vom Body geoeffneten Zusatzsektion.

Nicht veraendert werden Bundles, ExactLine-Projektionen, `REQUIRED_FINAL_REPORT_SECTIONS`, Version-/Hash-/Diff-Verifikation, Writer oder Musikregeln.

Abnahme:

```powershell
.\run.cmd --take --strict --all
```
