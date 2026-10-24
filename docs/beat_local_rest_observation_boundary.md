# BeatLocalRestObservationBoundary

## Zweck

v0.10.1699 ist Take 1 des auf drei Takes begrenzten
`StrictStaffRestTruthOwnershipArchitecture`-Superwurfs.

Die drei Phasen des `BeatLocalVoiceRestRewrite` haben denselben Takt bisher
mehrfach rekonstruiert: Cross-Staff-Staff/Voice-Beobachtung wurde separat
geparst, `MissingStaffEdgeRestRewrite` las dieselben Timeline-Fragmente ein
zweites Mal, und Backup-/Final-Cursor wurden zusätzlich mit eigenen
`<note>/<backup>/<forward>`-Scans aufgebaut.

Take 1 führt dafür genau eine typisierte
`BeatLocalVoiceRestMeasureObservation` ein. Pro Phase bleibt der Quellstand
bewusst frisch, weil Backup-, Edge- und Hidden-Rest-Phase nacheinander auf dem
jeweils vorherigen Kandidaten laufen. Innerhalb einer Phase besitzt aber nur
noch die Observation Taktgrenzen, Expected-Duration, Beat, Fragmente,
Cross-Staff-Erkennung, Backup-Cursor und Final-Cursor.

Die drei bestehenden Structural-Write-Authority-Capabilities und ihre
Reihenfolge bleiben unverändert. Keine musikalische Regel, kein Threshold,
kein Candidate-Limit und kein produktiver Writer-Root wird ergänzt.

Abnahme:

```powershell
.\run.cmd --take --strict --all
```

Erwartet: 10/10 artefaktidentische Projekte gegen v0.10.1698.
