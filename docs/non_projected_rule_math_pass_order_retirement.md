# Non-Projected Rule-Math and Pass-Order Retirement

Stand: v0.10.1589 r02

## Revision r02

`r01` wurde im Strict-Clippy-Gate vor jeder Installation verworfen. Die
Post-Render-Auditstufe nahm `materialized_rewrite_portfolio` noch als Input
entgegen und band es beim Destrukturieren, obwohl der letzte Consumer mit dem
Pass-Order-Rückbau entfallen war. r02 entfernt nur diese tote Weitergabe.

Das Portfolio selbst bleibt produktiv: Es wird weiterhin in der
Master-/Salience-Stufe erzeugt, darf weiterhin einen Rewrite adoptieren,
kann dadurch `best` verändern und projiziert `adopted_count` weiterhin in
den Schlopf-Kernel. Die fachliche Wirkung von r01 bleibt unverändert; r01 ist
kein gültiger Ausgangsstand.

## Ausgangsbefund

Der reale Lauf von v0.10.1588 / Export 528 bestätigt den vorherigen
Post-Render-Delta-Rückbau:

- Strict-Build und Strict-Clippy erfolgreich,
- `FinalExit 0`,
- Master- und Render-MusicXML gegenüber v0.10.1587 ausschließlich mit
  geändertem `<software>`-Versionsstring,
- alle sechs PDF-Seiten pixelidentisch.

Der Einzelwert der Laufzeit war in Export 528 insgesamt höher als im
Vorlauf. Clippy, Build und Programmlauf waren gemeinsam langsamer; daraus wird
kein Rückschluss auf den entfernten Delta-Audit gezogen. Der folgende Take
stützt sich deshalb auf garantiert entfallende Arbeit, nicht auf einen
ungeklärten Sekundenvergleich.

## Befund

Nach dem aktiven Voice-Fiber-/Rest-/Releaseblock wurde weiterhin eine
umfangreiche, ausschließlich in den nicht mehr konsumierten
`pipeline_lines`-Vektor projizierte Rule-Math-/Pass-Order-Familie berechnet.

Sie umfasste:

- `MathematicalRuleApplicationAudit`,
- Kopien aller Solver-Kandidatenberichte als `RuleMathCandidateInput`,
- `RuleMathCandidateStressTest`,
- `RuleMathParetoLocalRewriteBuilder`,
- `RuleMathLocalRewriteExecutionTrial`,
- `PostRewriteMusicalDeltaAudit`,
- `MathematicalPassOrderKernel`,
- `RuleDependencySchedulerPreview`.

Keines dieser Ergebnisse wurde in den typisierten Finalreport projiziert.
Keines veränderte `best`, `best.final_result`, MusicXML, Render-MusicXML,
PDF-Quellenauswahl, Blocker, Releaseentscheidung oder Exitcode.

Die vermeintliche lokale Rewrite-Ausführung war ausdrücklich ein
nicht materialisierender Beweis-/Berichtspfad. Der Pass-Order-Kernel ordnete
einen historischen Modellgraphen und schrieb ausschließlich eine nicht
ausgegebene Pipelinezeile.

## Entfernte Laufzeitarbeit

Pro Projekt entfallen garantiert:

1. vollständige Klone aller Kandidaten-`CheckReport`s für den
   Rule-Math-Stresstest,
2. zweimaliger Aufbau und Vergleich der Rule-State-Vektoren aller Kandidaten,
3. Pareto-Markierung und Sortierung der Kandidatenzeilen,
4. Aufbau, Pareto-Markierung und Sortierung lokaler Rewrite-Seeds,
5. Erzeugung eines nicht materialisierenden Rewrite-Trials,
6. Post-Rewrite-Delta-Projektion ohne tatsächliches Delta,
7. Aufbau und Prüfung des historischen Pass-/Artefaktgraphen,
8. Scheduler-Preview und sämtliche dazugehörigen Stringprojektionen,
9. die nach dem letzten produktiven Consumer verbliebene Sortierung von
   `all_for_lines`.

## Erhaltene produktive Regelmathematik

`rule_math.rs` wird nicht vollständig gelöscht.

Der Solver verwendet weiterhin unverändert:

```text
rule_math::score_penalty(CheckReport) -> i64
```

Dafür bleiben exakt erhalten:

- RuleId-Auflösung,
- zentrale RuleBook-Gewichte,
- Severity-Faktoren,
- Divisions-Metrikanteil,
- Gruppierung nach identischem Rule-/Scope-Zustand,
- Summierung zur identischen `total_energy`.

Nur die nicht projizierten Stresstest-, Rewrite- und Pass-Order-Oberflächen
werden entfernt.

## Struktureller Rückbau

- `src/pass_order.rs` vollständig gelöscht,
- `mod pass_order` und Pipeline-Import gelöscht,
- `rule_math.rs` auf den aktiven Score-Kern reduziert,
- nicht projizierter Post-Render-Aufrufblock gelöscht,
- `all_for_lines` verlässt die Master-/Salience-Stufe nicht mehr,
- kein Ersatztyp,
- kein Adapter,
- kein Shadowpfad,
- keine neue Telemetrie.

## Unveränderte Autorität

Unverändert bleiben:

- produktive Solverbewertung einschließlich `score_penalty`,
- Arrangement- und Kandidatenauswahl,
- SalienceWeightedCandidateGate,
- DiscreteActionSalienceTranslator,
- MaterializedRewritePortfolio,
- VoiceFiberBehindBarsDecisionLedger und dessen reale Reportmutation,
- sämtliche Writer,
- MusicXML-/Render-/PDF-Auswahl,
- RawCursor- und Rest-Gates,
- typisierter Finalreport,
- RunOutcome- und Releaseprojektion.

## Beweisgrenze

Der Take ist `RefactorOnly`.

Die Abnahme muss zeigen:

- Strict-Build/Clippy grün,
- `FinalExit 0`,
- gleiche Blocker-, Warnungs- und Releaseentscheidung,
- Master-/Render-MusicXML gegenüber v0.10.1588 nur im Versionsstring
  verschieden,
- PDF pixelidentisch.

Ein Korpuslauf ist für diesen nicht projizierten, nicht schreibenden
Rückbau nicht erforderlich.
