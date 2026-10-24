# TonalBoundaryLedgerDecisionEvidenceClosure

Stand: v0.10.1839 r312  
Parent: r311 `TransactionalRestCoalesceChordAnchorExistingOwnerClosure`  
ChangeClass: `RefactorOnly`

## Befund

Der nach r311 erfolgreiche Lauf von Etuede 5 erzeugte 36 reale `KeyChangeCandidates`, aber der maschinenautoritative `TonalBoundaryLedger` publizierte nur 32 Zeilen. Der Neuro-Evidence-Exporter blockierte deshalb korrekt fail-closed mit:

```text
DecisionEvidence-Closure verletzt: LedgerRows 32 != KeyChangeCandidates 36
```

Die Ursache lag nicht in der Kandidatenbildung und nicht im Neuro-Evidence-Exporter. Die bereits vollstaendige `audit.rows`-Wahrheit wurde bei der Ergebnisprojektion durch `audit.rows.iter().take(32)` gekuerzt. Diese historische Praesentationsgrenze ist mit dem spaeter eingefuehrten Decision-Level-Machine-Evidence-Vertrag unvereinbar.

## Closure

r312 entfernt ausschliesslich die feste 32-Zeilen-Kappung aus der bestehenden `TonalBoundaryLedger`-Projektion. Alle bereits berechneten Audit-Zeilen werden in bestehender Sortierung und bestehendem Format ausgegeben. Es wird keine Kandidatenlogik, keine Schwelle, keine Profilgewichtung, kein Verdict und kein MusicXML-Writer geaendert.

Damit gilt konstruktiv fuer die Projektion:

```text
LedgerRows == audit.rows.len() == KeyChangeCandidates
```

sofern der vorhandene Result-Report unverfaelscht geschrieben und gelesen wird. Der Neuro-Evidence-Exporter behaelt seinen strikten Closure-Check unveraendert und bleibt fail-closed.

## Regression

Ein synthetischer 40-Takt-Tonal-Audit erzeugt bewusst mehr als 32 KeyChangeCandidates. Die Regression prueft, dass die Zahl der im Report materialisierten Ledger-Zeilen exakt `audit.key_change_candidates` entspricht. Damit ist eine erneute stillschweigende Praesentationskappung oberhalb 32 direkt gebunden.

## Nicht geaendert

- keine Musikmutation
- keine Render-/Master-MusicXML-Aenderung durch diesen Owner
- keine KeyChangeCandidate-Selektion
- keine Tonal-Schwelle oder Gewichtung
- kein neuer Writer oder Repair-/Recovery-Owner
- kein Neuro-Evidence-Sicherheitsvertrag
- keine Lockerung des `LedgerRows == KeyChangeCandidates`-Gates
- keine Etueden-, Takt- oder Kandidatenanzahl-Sonderregel

`RefactorOnly` wird verwendet, weil der bestehende gemischte Tonal-Owner nur in seiner vollstaendigen Read-only-Outcome-Projektion korrigiert wird; die fachliche Tonalentscheidung und jede Musikmutation bleiben unveraendert.
