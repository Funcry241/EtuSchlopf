# SilentVoiceCoveragePlanningCoreExtraction

## Zweck

v0.10.1706 ist Take 2 des `FinalNotationTruthOwnershipArchitecture`-Superwurfs.

Die Silent-Voice-Coverage-Stufe hat bisher reine Kandidatenplanung und produktive Mutation in derselben Writerfunktion vermischt. Dazu gehoerten Forward-Carrier-Analyse, Lane-Coverage, Rest-Darstellbarkeit sowie die strukturelle Cross-Staff-Unterdrueckung technisch vollstaendig abgedeckter Rest-Lanes.

Der neue `silent_voice_coverage_planning_core.rs` besitzt diese Planung writer-frei und liefert einen typisierten Plan aus Replacements und Audit-Zaehlwerten. Der bestehende Writer behaelt die einzige Structural-Write-Authority, die Readback-Invarianten, Candidate-Acceptance und den finalen Commit.

Keine Rest-, Staff-, Voice-, Timeline- oder Musikregel wird geaendert. Die Reihenfolge der Replacements und die bisherigen Audit-Zaehlungen bleiben erhalten.

Abnahme:

```powershell
.\run.cmd --take --strict --all
```

Erwartet: 10/10 artefaktidentische Projekte gegen v0.10.1705.
