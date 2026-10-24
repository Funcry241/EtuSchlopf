# DeadRenderProbeRetirementSuperCut

## Version

`v0.10.1763 r02`

## Revision r02

Der reale r01-Strict-Clippy-Lauf stoppte nach dem erfolgreichen Retirement an drei nun
toten StructuralWriteAuthority-Resten: `CrossStaffLedger`,
`write_cross_staff_ledger_candidate_authorized()` und dessen Re-Export. r02 entfernt
genau diese unbenutzte Sink-Schnittkante. Zwei `let _ =`-Bindungen an der stabilen
Beam/Rest-Fassade werden rein mechanisch entfernt, weil die Funktion inzwischen `()`
liefert. Keine produktive Musik- oder Writerentscheidung aendert sich.

## Ziel

Reine Laufzeit-Sedimente entfernen, deren Ergebnis im autoritativen v0.10.1762-Stand
keine produktive Entscheidung mehr erreichen kann. Keine neue Musiklogik, kein neuer
Owner, keine Gate-Ausnahme.

## Entfernte Runtime-Pfade

### CurrentRenderOperatorNoop

Der Builder analysierte die aktuelle Render-XML erneut, erzeugte Signaturen und
Parse-/BackupForward-Evidenz, gab aber konstruktiv `attempted=false`, `written=false`
und null Kandidaten zurueck. Im Orchestrator wurde danach nur sein immer falscher
`written`-Wert abgefragt. Der Pfad ist vollstaendig entfernt; die nachfolgende
Missing-Rest-Source-Beschriftung entspricht direkt dem zuvor erreichbaren `else`-Pfad.

### CrossStaffVoiceLedgerPlanner / RewriteTrial

Der Planner war `audit-only` und besass keinen produktiven Consumer ausser dem direkt
nachfolgenden RewriteTrial. Der Trial setzte `voice_ledger_proof_complete=false` und
`hand_ledger_proof_complete=false`, materialisierte als Kandidat nur `xml.to_string()`,
pruefte diese identische Kopie erneut und konnte deshalb den Writer-Gate konstruktiv
nicht erreichen. Sein `Result` wurde im Orchestrator mit `let _ = ...` verworfen.

Entfernt werden daher:

- `build_render_xml_pianistic_cross_staff_voice_ledger_planner()`
- `build_render_xml_pianistic_cross_staff_voice_ledger_rewrite_trial()`
- `render_cross_staff_gap_profile()`
- `forward_coverage_for_gaps()`
- `RenderXmlPianisticCrossStaffVoiceLedgerPlanner`
- `RenderXmlCrossStaffVoiceLedgerRow`
- `RenderXmlPianisticCrossStaffVoiceLedgerRewriteTrial`
- `RenderXmlCrossStaffVoiceLedgerRewriteAction`
- der komplette Runtime-Aufrufblock im Initial-Render-Orchestrator

## Nicht betroffen

- produktive Cross-Staff-Hand-/Voice-Materialisierung
- `RenderLaneTimelineMeasure` / `RenderLaneTimelineEvent`
- Lower-Dual-Edge-/LaneFold-/Beam-Logik
- `BeatScopedRestCompletionGapPlan`
- `MetricRestNotation`
- Ganztaktpausen-Ownership
- Beam-/Rest-Trennung aus v0.10.1761
- Writer-Reihenfolge und StructuralWriteAuthority

## Abnahmevertrag

Der Cut ist `RefactorOnly`. Fuer Etuede 1 gilt:

- all-pitched Timeline 1762 == 1763
- Reststruktur 1762 == 1763
- Beamstruktur 1762 == 1763
- Master-/Render-MusicXML bis auf Versionsmetadaten identisch
- PDF pixelweise 0,000 % Abweichung

Ein Unterschied ausserhalb Versionsmetadaten ist ein Regressionsfund und blockiert
die Autorisierung.
