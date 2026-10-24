# MetricBeamPlanningCoreExtraction

## Zweck

v0.10.1703 ist Take 2 des `ComposerBeamTruthOwnershipArchitecture`-Superwurfs.

Nach der typisierten Observation aus Take 1 werden die reinen Beam-Entscheidungen aus `composer_recovery_octave_beam_export_tail.rs` nach `composer_metric_beam_planning_core.rs` verschoben.

Der neue Planning-Core besitzt die bereits vorhandenen, unveraenderten Regeln fuer Beat-Gruppierung, sichtbare Rest-Gap-Coverage, post-rest-Beam-Zulassung, Beam-Level-Runs, Scorewide-Metric-Beamability, Gould-Primary-State-Beobachtung, Structural-Staff-Gruppen und Rest-Bridge-BeamPlan-Konstruktion.

Damit liegt Beam-Planung nicht mehr innerhalb derselben Datei wie Candidate-Materialisierung, produktive Writer-Audits, Octave/Clef-Policy und finaler Export. Die vorhandene `structural_write_authority::BeamPlan` bleibt der einzige Planvertrag; es wird weder eine zweite Beam-Wahrheit noch eine neue Write-Capability eingefuehrt.

Nicht geaendert werden Candidate-Auswahlwerte, Sortierungen, Beam-Zustaende, Materialize-/Readback-Semantik, produktive Operatoren, Passreihenfolge oder Musikregeln.

Abnahme:

```powershell
.\run.cmd --take --strict --all
```

Erwartet: 10/10 artefaktidentische Projekte gegen v0.10.1702.
