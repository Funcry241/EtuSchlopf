# r1115 NormalRunAssistantExportLeanProjectionClosure

## Parent und Change Class

Parent ist der kanonisch akzeptierte `v0.10.1839 r1114 TakeFailureEvidenceContinuityEndToEndClosure`.

ChangeClass: `OutcomeProjection`.

Der Take aendert ausschliesslich die normale Einzelprojekt-Assistant-Projektion. Er erzeugt keine MusicMutation und keine Writer-, Repair-, Selection-, Renderer-, Release-, Take-Acceptance-, Rollback- oder Failure-Evidence-Authority.

## Ziel

Der normale `assistant_export_latest.zip` soll den aktuellen repository-first rekonstruierbaren Zustand transportieren, nicht bei jedem Lauf die vollstaendige lokale Diagnose- und Lifecycle-Historie replizieren.

Der vollstaendige aktuelle Source-Snapshot bleibt unveraendert Bestandteil des Exports. Master-MusicXML, Render-MusicXML, PDF, current-run Measurement Snapshot, aktuelle Logs, RunOutcome und die uebrigen aktuellen Bindungen bleiben erhalten.

## Normal-Run-Projektion

Der bisher vollstaendig exportierte `<Projekt>.result.txt` bleibt lokal und wird im Assistant-Export durch die bereits vorhandene kompakte `<Projekt>.assistant-summary.txt`-Projektion ersetzt. Die Summary erzeugt keine neue Wahrheit; sie liest ausschliesslich das lokale aktuelle Resultat.

`transition_history.json` bleibt lokal. Fuer einen normalen Assistant-Cold-Start ist diese Datei nicht kanonische Authority; die Post-Acceptance-Lifecycle-Bindung bleibt beim bestehenden Acceptance-/Lifecycle-Vertrag. Corpus-/Visual-Preflight-Exporter behalten ihre bisherige Transition-History-Projektion unveraendert.

Der bounded Normal-Run-Projektkatalog wird nur noch als `project_file_catalog.tsv` exportiert. Die semantisch redundante Markdown-Darstellung wird im Normal-Run nicht erzeugt. Andere Exportmodi behalten beide Darstellungen.

`assistant_handoff_latest.txt` bettet beim Normal-Run weder das volle Resultat noch den Markdown-Katalog ein. Er verweist auf die kompakte Result-Summary und den TSV-Katalog. Repository-Cold-Start, aktueller Take-Vertrag, Assistant Context, RunOutcome, Manifest, Console und Current-Take-Changed-Source bleiben erhalten.

## Kodex-Retirement

Der bisherige synthetische Negative-Check `normal_run_assistant_export_contains_all_four_project_outputs#1` kodifizierte exakt den alten Vier-Output-Vertrag inklusive vollstaendig exportiertem `.result.txt`. Da dieser Vertrag durch diesen OutcomeProjection-Take bewusst endet, wird die alte Negative-Check-Capability ueber den bestehenden append-only `KodexCapabilityRetirement`-Vertrag fingerprint-gebunden pensioniert. Die neue Regression ist kein verschobener Stub: sie prueft den neuen Drei-Score-Output-Vertrag plus kompakte Result-Summary und die weiteren Lean-Projection-Grenzen.

## Failure-Evidence-Grenze

Dieser Take veraendert keine r1108-r1114 Failure-Evidence-Logik. Der bestehende Post-Acceptance-Projektor nimmt weiterhin exakt den Rejection-Suffix seit der vorherigen Acceptance. Wenn zwischen r1114 und r1115 keine neue Rejection liegt, ist dieser Suffix leer und `take_attempt_evidence/` wird im neuen accepted Assistant-Export nicht mitgefuehrt. Persistente lokale Failure-Evidence bleibt davon unberuehrt.

## Regression

Der bestehende synthetische Normal-Run-Exporttest wird ohne neue Korpusidentitaet erweitert. Er beweist:

- Master-MusicXML, Render-MusicXML und PDF bleiben direkte Projektoutputs des Normal-Run-Exports;
- das volle Resultat ist kein direktes Exportpayload mehr;
- die bestehende Assistant-Summary wird verwendet;
- der Normal-Run exportiert nur den TSV-Katalog;
- `transition_history.json` wird im Normal-Run nicht mehr erzeugt.

Der bestehende synthetische Projektkatalogtest beweist zusaetzlich, dass der bounded Normal-Run-Katalog keinen geplanten Markdown-Katalog mehr behauptet und eine gegebenenfalls vorhandene alte Markdown-Projektion entfernt.

## Erwarteter Effekt

Der r1114-Sonderexport war wegen der bewusst projizierten r1113-Rejection-Capsule rund 10,8 MB gross. Bei einem erfolgreichen r1115 ohne neue Rejection entfallen die beiden historischen Voll-Snapshots bereits durch den bestehenden Lifecycle-Vertrag. Dieser Take reduziert anschliessend die verbleibende Normal-Run-Projektion weiter, ohne den vollstaendigen aktuellen Source-Snapshot anzutasten.

Der genaue physische Exportumfang ist Laufresultat und keine Acceptance-Vorgabe. Entscheidend ist die semantische Grenze: aktueller rekonstruierbarer Zustand im Normalexport, historische/advisory Volltexte lokal bzw. bei ihrem spezialisierten Beweisweg.
