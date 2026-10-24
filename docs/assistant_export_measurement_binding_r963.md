# r963 AssistantExportMeasurementBinding

## Ziel

r963 bindet den bereits in r962 erzeugten `CanonicalMeasurementSnapshotV1` an den normalen Einzelprojekt-Assistant-Export. Es werden keine Metrics neu berechnet und keine Corpus-Werte aggregiert.

## Current-run-Bindung

Der Launcher übergibt seine aktuelle Observation-Session-ID an `AssistantExportRequest::Normal`.

Der Export akzeptiert `<Projekt>.canonical_measurement_snapshot_v1.json` nur, wenn gleichzeitig gilt:

- `schema_version = 1`,
- `measurement_contract_version = CMS-1`,
- `expected_projects = [aktuelles Projekt]`,
- `measured_projects = [aktuelles Projekt]`,
- `run.run_id = aktuelle Observation-Session-ID`.

Ein erfolgreicher Lauf (`FinalExit = 0`) ohne passenden Snapshot bricht nur die Assistant-Export-Erzeugung fail-closed ab. Ein kontrollierter Nicht-Null-Lauf darf den Diagnose-Export weiterhin ohne Measurement Snapshot erzeugen; ein stale Snapshot wird dabei niemals übernommen.

## Transport

Vor dem Staging werden ein vorhandenes `target/etuschlopf_assistant_export/latest.zip` und das Root-`assistant_export_latest.zip` entfernt. Damit kann ein aktueller fail-closed Exportfehler keinen alten Export als scheinbar aktuellen Bundle zurücklassen. Ein akzeptierter Snapshot wird danach byteidentisch in `assistant_export_latest.zip` übernommen.

`assistant_context.json` referenziert:

- `status`,
- Bindung `current_observation_session_run_id`,
- Dateiname,
- SHA-256,
- Run-ID,
- Measurement Contract Version.

`manifest.txt` enthält dieselben Bindungsdaten als Textprojektion.

Der physische Source-Snapshot bleibt ein separates Exportartefakt. Der Measurement Snapshot wird nicht um `physical_source`, Revision, AIC oder Renderdaten erweitert.

## Grenzen

Nicht Bestandteil von r963:

- keine neue Metric,
- keine Corpus-Aggregation,
- kein AIC/EIC,
- keine Excel-Erzeugung,
- keine Historie/Comparator-Logik,
- keine MusicMutation,
- keine Writer-/Repair-/Recovery-/Release-Authority.

ChangeClass: `KodexOnly`.
