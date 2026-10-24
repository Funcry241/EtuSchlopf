# CanonicalMetricBeamObservationBoundary

## Zweck

v0.10.1702 ist Take 1 des auf drei Takes begrenzten `ComposerBeamTruthOwnershipArchitecture`-Superwurfs.

`composer_recovery_octave_beam_export_tail.rs` besass neben produktiven Beam-, Rest-, Octave- und Export-Operatoren auch die gemeinsame Rohbeobachtung, aus der mehrere Beam-Domaenen Measure-Grenzen, Note-Fragmente, Cursor-Onsets, Staff, Voice, Dauer, Rest-/Chord-/Hidden-Zustand und vorhandene Beam-Tags lesen.

Diese bestehende Wahrheit wird unveraendert nach `composer_metric_beam_observation_core.rs` verschoben. `PartialBeatMeasureSpan`, `PartialBeatRawNote`, `PartialBeatNoteInfo` und ihre Parser/Accessor-Funktionen bleiben im selben `pipeline`-Namensraum und behalten Signaturen sowie Verhalten exakt bei. Es entsteht kein zweiter Parser und kein neuer produktiver Writer-Root.

Nicht geaendert werden Beam-Gruppierung, BeamPlan-Erzeugung, Candidate-Sealing, Materialisierung, Readback, Structural-Write-Authority, Passreihenfolge, Regeln oder Schwellenwerte.

Der neue Core besitzt keine XML-Write-Authority, keinen Filesystem-Sink und keine Reportprojektion.

Abnahme:

```powershell
.\run.cmd --take --strict --all
```

Erwartet: 10/10 artefaktidentische Projekte gegen v0.10.1701.
