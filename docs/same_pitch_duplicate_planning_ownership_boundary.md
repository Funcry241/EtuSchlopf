# SamePitchDuplicatePlanningOwnershipBoundary

## Zweck

v0.10.1700 ist Take 2 des auf drei Takes begrenzten
`StrictStaffRestTruthOwnershipArchitecture`-Superwurfs.

`DuplicateStemSamePitchRenderSuppression` und
`DuplicateSamePitchExactRenderWrite` haben bisher denselben sichtbaren
Pitch-/Onset-/Duration-Kandidatenmechanismus vollstaendig doppelt besessen.
Der Duplicate-Stem-Readback hat dieselbe sichtbare Pitch-Beobachtung ein
drittes Mal aus MusicXML rekonstruiert.

Take 2 fuehrt eine gemeinsame typisierte
`StrictStaffVisiblePitchObservation` und genau einen writer-freien
`plan_strict_staff_duplicate_same_pitch_suppression()` ein.

Die fachliche Differenz bleibt explizit:

- `StaffAcrossVoices` fuer den Behind-Bars-Suppressionspfad
- `ExactVoice` fuer den External-Feedback-Exact-Write

Damit wird nicht eine Regel vereinheitlicht, sondern nur die zuvor doppelte
Kandidatenmechanik. Beide produktiven Aufrufer behalten dieselbe
`apply_duplicate_same_pitch_suppression_authorized`-Authority, dieselben
Candidate-Limits, dieselbe Hide-Transformation und dieselben Action-/Verdict-
Semantiken. Der Readback konsumiert dieselbe Pitch-Observation, bleibt aber
writer-frei.

Abnahme:

```powershell
.\run.cmd --take --strict --all
```

Erwartet: 10/10 artefaktidentische Projekte gegen v0.10.1699.
