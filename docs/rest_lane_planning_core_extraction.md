# RestLaneObservationPlanningCoreExtraction

## Zweck

v0.10.1701 ist Take 3 und Abschluss des auf drei Takes begrenzten
`StrictStaffRestTruthOwnershipArchitecture`-Superwurfs.

Nach der BeatLocal-Observation aus Take 1 und dem policy-bewussten
Same-Pitch-Planner aus Take 2 wird die verbleibende reine Lower-/Upper-Rest-
Lane-Planung aus der produktiven Writer-Datei herausgeloest.

Neu ist `strict_staff_rest_lane_planning_core.rs`. Dieser Shard besitzt keine
Structural-Write-Authority und keinen Filesystem-Sink. Er enthaelt nur
Interval-/Collision-/Ledger-Beobachtung, Candidate-Bildung und
Render-Fragment-Planung.

Sechs Lower-/Upper-Rest-Lane-Consumer besitzen ihre Measure-Grenzen und
Render-Timeline-Fragmente nicht mehr ueber sechs lokale XML-Scan-Schleifen.
`StrictStaffRestLaneMeasureObservation` ist dafuer der gemeinsame typisierte
Observation-Owner; jeder Consumer fordert bei seinem Aufruf weiterhin frisch
den aktuellen XML-Zustand an, sodass die bestehende Mutationsreihenfolge
unveraendert bleibt.

Zugleich verschwindet eine konkrete zweite Measure-Wahrheit:
`apply_lower_staff_visible_rhythm_rest_lane_trial()` bestimmt den Scope nicht
mehr nachtraeglich durch einen zweiten absoluten Position->Measure-Scan.
Der neue typisierte `LowerStaffVisibleRhythmRestLanePlan` traegt den Scope
direkt aus genau der Measure-Observation, die den Kandidaten erzeugt hat.
Der alte 15-Feld-Tuple-Alias wird entfernt.

Der bestehende `apply_lower_staff_visible_rhythm_rest_lane_trial` bleibt der
produktive Writer-Root und konsumiert nur noch source-bound Plandaten.
Alle bestehenden Lower-/Upper-Structural-Write-Authority-Capabilities,
Candidate-Limits, Sortierungen, Regeln und Passreihenfolgen bleiben
unveraendert.

Abnahme:

```powershell
.\run.cmd --take --strict --all
```

Erwartet: 10/10 artefaktidentische Projekte gegen v0.10.1700. Mit gruener
Abnahme ist `StrictStaffRestTruthOwnershipArchitecture` beendet; kein Take 4.
