# r962 – CanonicalMeasurementSnapshotV1

## Parent und Scope

r962 baut direkt auf dem real installierten und `dark_green`/`released` bestätigten r961 `RuntimeProducerBindingClosure` auf.

Der Take schließt ausschließlich Measurement v1 für den einzelnen Produktlauf. Assistant-Export-Bindung, Corpus-Aggregation und AIC bleiben nachgeordnet.

## Genau sechs Native Metrics

1. `final_findings.blocker.count` – Owner `ReportSummary.blockers`, CountClass `FINDING`.
2. `final_findings.warning.count` – Owner `ReportSummary.warnings`, CountClass `FINDING`.
3. `final_findings.hint.count` – Owner `ReportSummary.hints`, CountClass `FINDING`.
4. `source.canonical_event.count` – Owner `CanonicalSourceEventProjection.event_count`, CountClass `POPULATION`.
5. `source.selection.retained.count` – Owner `SourceSelectionFullCausalProjection.productive_plan_retained_event_count`, CountClass `OUTCOME`.
6. `source.selection.removed.count` – Owner `SourceSelectionFullCausalProjection.removed_event_count`, CountClass `OUTCOME`.

Keine Prozentwerte, Scores, Worker-/Solverzahlen oder zusätzliche Registry.

## Integritätsgates

Vor Materialisierung gilt fail-closed:

- `productive_plan_authority == true`,
- `productive_plan_equivalence_verified == true`,
- `productive_plan_retained_event_count == retained_event_count`,
- `CanonicalSourceEventProjection.event_count == SourceSelectionFullCausalProjection.canonical_source_event_count`,
- `CanonicalSourceEvents == ProductivePlanRetainedEvents + RemovedSourceEvents`.

Die letzte Gleichung ist kein zusätzlicher Messwert.

## Producer Binding

Der Snapshot konsumiert nur die r961-Umgebungsprojektion:

- `bound`: Version + Source-Signatur + Binary-Fingerprint müssen vollständig vorhanden sein.
- `partially_bound`: nur Binary-Fingerprint darf vorhanden sein; Version/Source-Signatur wären ein Fehler.
- `unbound`: keine Producer-Identitätsfelder dürfen behauptet werden.

Unbekannte Binding-State-Texte fallen fail-closed auf `unbound`; gleichzeitig vorhandene Identitätsfelder machen die Projektion ungültig.

`ProducerRevision` bleibt absent, weil r961 dafür keinen revisionssicheren Build-Bindungsbeweis besitzt.

## Snapshot-Grenze

Pro Projekt entsteht neben den normalen Projektartefakten:

`<Projekt>.canonical_measurement_snapshot_v1.json`

Struktur:

- `schema_version = 1`,
- `measurement_contract_version = CMS-1`,
- `producer`,
- `run`,
- `metrics`.

Nicht enthalten sind `physical_source`, Assistant-Export-Identität, Architekturinterpretation, Historie oder Renderprofil.

Der Snapshot wird erst am finalen Report-Projektionscheckpoint geschrieben. Deshalb sind alle in einem vorhandenen Snapshot enthaltenen v1-Metrics `measured`; ein früher abgebrochener Lauf besitzt keinen aktuellen v1-Snapshot. Ein späterer Export-Consumer muss zusätzlich `run_id` binden und darf einen alten Snapshot niemals als aktuellen Lauf behandeln.

## Kein Corpus in r962

Ein Produktprozess kennt nur sein Projekt. `scope=project`, `full_corpus=false`, `expected_projects=[aktuelles Projekt]`, `measured_projects=[aktuelles Projekt]`.

Die 10/10-Aggregation gehört nicht in diese Runtime-Projektion und wird erst an der Export-/Manifest-Grenze geschlossen.

## Nicht enthalten

- keine Assistant-Export-Erweiterung,
- kein `physical_source` im Measurement Snapshot,
- kein AIC/EIC,
- keine Excel-Erzeugung,
- keine Historien-/Comparator-Logik,
- keine Metric Registry oder Datenbank,
- keine MusicMutation,
- keine Writer-/Repair-/Recovery-/Release-Authority.
