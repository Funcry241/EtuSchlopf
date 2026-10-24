# r334 BeamDomainDecisionMaterializationOwnershipClosureDeadApiRecovery

## Autoritative Basis

Parent ist die real installierte und 10/10 corpus-green r331
`LowerRestBridgeBeamOwnershipLifecycleCommittedReadbackTestFixtureRecovery`.

r332 wurde vor Installation wegen ChangeClass-Mischung abgelehnt.
r333 trennte die AuditOnly-Aenderung korrekt ab, scheiterte jedoch im Strict-Clippy-
Build an `#![deny(dead_code)]`, weil `BeamPlan::from_complete_levels()` nach der
FinalCompleteBeat-Umstellung keinen produktiven Aufrufer mehr besass.

r333 ist damit keine GreenAuthority und wird nicht wiederverwendet.

## Recovery

r334 uebernimmt den produktiven r333-Beam-Domain-Cut und schliesst die dadurch
offengelegte tote Alt-API sauber:

1. `FinalCompleteBeatPrimaryBeamContinuity` konsumiert den gemeinsamen
   `structural_metric_beam_hierarchy_plan(..., 1)` statt einer eigenen
   Complete-Level-Planung.

2. Der gemeinsame BeamHierarchy-Owner besitzt einen writer-freien L1-
   Projektionstest.

3. `BeamPlan::from_complete_levels()` wird entfernt, weil nach (1) kein
   produktiver Aufrufer mehr existiert. Es wird weder mit `allow(dead_code)`
   kaschiert noch kuenstlich produktiv referenziert.

4. Die bisherigen Sparse-Level- und Fail-Closed-Assertions der entfernten
   Convenience-API werden direkt auf `BeamPlan::from_level_groups()` projiziert,
   sodass die zugrunde liegenden Invarianten weiterhin getestet werden.

Damit bleibt nur eine produktive BeamHierarchy-Entscheidungsabstraktion fuer
explizite Level-Gruppen bestehen.

## ChangeClass

`MusicMutation`.

Die entfernte Funktion ist Music-Domain-Logik und nach der r333-Entkopplung
detached. Der spezielle `SecurityCapabilityRetirement`-Vertrag ist nicht
einschlaegig; er gilt fuer Security-Negativchecks.

## Bewusste Nicht-Aenderung

`src/audit_only/mutation_ownership_evidence.rs` bleibt byteidentisch zu r331.

Keine Aenderung an:

- Writer-Roots
- Pass-Reihenfolge
- Lower RestBridge Materialisierung
- Upper Materialisierung
- Lost-Begin-/Rest-Carrier-Gates
- Pitch, Onset, Duration, Staff, Voice oder Restbestand

## Erwartete Abnahme

Gegen r331 wird MusicXML-Invarianz erwartet.

Corpusweit bleiben insbesondere:

```text
Master Achtel + Beam2  = 0
Render Achtel + Beam2  = 0
Restknoten mit Beam    = 0
CommittedLowerDualEdgeCandidates = 0
CommittedBeamLostBeginCandidates = 0
FinalBeamLostBegin Candidates    = 0
```

Die spaeten Materialisierungszaehler sind Evidence und keine Zielzahlen.
