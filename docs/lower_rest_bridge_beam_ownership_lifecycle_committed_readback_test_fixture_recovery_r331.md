# LowerRestBridgeBeamOwnershipLifecycleCommittedReadbackTestFixtureRecovery – r331

## Autoritative Basis

Parent ist der real installierte und zielsystem-gruene Stand `v0.10.1839 r325 SharedMetricBeamHierarchyDecisionOwnerConvergenceCompileRecovery`.

Verworfene Zwischenrevisionen:
- r326: vor Installation durch Take-Tree-Integrity wegen Top-Level `Cargo.lock` abgewiesen.
- r327: vor Installation durch NO-NEW-REPAIR-AUTHORITY abgewiesen.
- r328: nicht ausgefuehrt.
- r329: statische Gates und alle Tests GREEN, realer Etuede-6-Lauf `FinalExit 1`, automatischer Rollback auf r325.
- r330: PRE-TAKE, Clippy und Build GREEN; `--bin etuschlopf` Testtarget rot, automatischer Rollback auf r325.

## r329 Runtime-Beweis

Der r329-Export zeigte als einzigen relevanten Runtime-Blocker `CommittedLowerDualEdgeCandidates 3` bei gleichzeitig:
- `CommittedBeamLostBeginCandidates 0`,
- `VisibleRestsBeforeAfter 82/82`,
- `NoteCountBeforeAfter 1206/1206`,
- stabilem RawCursor und PitchedEventMultiset.

Der pensionierte LowerDualEdge-Readback hielt also noch die historische Rest-Carrier-Norm fuer kanonisch.

## r330 Testtarget-Beweis

r330 konvergierte den read-only Committed-State-Detector auf den lebenden Shared L1/L2 BeamHierarchy DecisionOwner und machte den Restslot beamfrei. Der Produktivcode kompilierte; erst das interne Bin-Testtarget wurde rot.

Die Ursache lag in einer aelteren Test-Fixture `exact_structural_measure`: deren Helper `note(...)` erzeugte auch fuer den Restslot weiterhin `<beam number="1">...` und `<beam number="2">...`. Die vermeintlich gruene Referenz enthielt damit exakt den historischen Failure-Mode A, den r330 nun korrekt als Regression klassifiziert.

## r331 Recovery

r331 uebernimmt den gesamten r330-Lifecycle-Cut unveraendert und korrigiert ausschliesslich diese bestehende Test-Fixture-Semantik:
- klingende 16tel erhalten die bisherigen L1/L2-Teststates,
- der strukturelle Restslot wird beamfrei erzeugt.

Dadurch bleiben die bestehenden Tests semantisch aussagekraeftig:
- `continue` auf der zweiten klingenden Note ist gruen,
- doppeltes `begin` bleibt rot,
- verlorenes primaeres `begin` bleibt rot,
- historische Beam-Tags auf dem Rest bleiben rot.

Es wird kein neuer TestOwner, Writer, Repair-/Recovery-Root oder produktiver Selector eingefuehrt.

## Produktiver Ownership-Cut

Unveraendert gegenueber r330:
- Early und Late Lower-RestBridge konsumieren denselben `structural_metric_beam_hierarchy_plan(..., 2)`.
- Nur klingende beam-faehige Anker sind BeamPlan-Ereignisse.
- Restknoten bleiben beamfrei.
- L2 folgt dem tatsaechlichen Notenwert.
- L3/L4 bleiben Read-through.
- Der bestehende pensionierte LowerDualEdge-Detector bleibt read-only und fail-closed und projiziert dieselbe kanonische Semantik.

## Harte Abnahme

Etüde 6:
- alle Cargo-Targets GREEN,
- FinalExit `0`,
- `CommittedLowerDualEdgeCandidates 0`,
- `CommittedBeamLostBeginCandidates 0`,
- `VisibleRestsBeforeAfter` stabil,
- Render-Achtel mit Beam-Level 2 im Lower-RestBridge-Scope `5 -> 0`.

Vollkorpus danach:
- Master Achtel + Beam2 `0`,
- Render Achtel + Beam2 `12 -> 0`,
- 10/10 GREEN,
- keine neue Repair-/Recovery-Authority.
