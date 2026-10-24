# Beam Continuity Observation Boundary

## Scope

`v0.10.1684 r01` ist ausschliesslich ein verhaltensneutraler `RefactorOnly`-Take
innerhalb von
`src/pipeline_shards/lower_dual_edge_beat_beam_continuity_repair.rs`.

Der Local-Voice-Batch-Pfad hatte vor diesem Take drei unmittelbar
aufeinanderfolgende Aufgaben in `build_local_voice_batch_plans()`:

1. MusicXML lesen und den aktuellen metrischen Kontext fortschreiben,
2. daraus `LocalVoiceBatchEvent`-Beobachtungen erzeugen,
3. diese Beobachtungen nach Scope/Voice gruppieren, problematische Beats
   qualifizieren und daraus bestehende `LocalVoiceBatchPlan`s bauen.

Take 1 schneidet exakt zwischen 2 und 3.

## Bestehender Vertrag bleibt autoritativ

Es wird kein zweites Ereignismodell eingefuehrt. `LocalVoiceBatchEvent` bleibt
unveraendert und traegt weiterhin unter anderem:

- Scope- und Note-Ordinal,
- XML-Node-Grenzen und Source-Fingerprint,
- Onset und Dauer,
- Staff und Voice,
- Notentyp und Beat-Kontext,
- Rest-/Visibility-/Chord-Zustand,
- Pitch-MIDI,
- vorhandene Beam-Zustaende.

Neu ist nur der private Transportcontainer `LocalVoiceBatchObservation` mit:

- `parse_ok`,
- `events: Vec<LocalVoiceBatchEvent>`.

`observe_local_voice_batch()` enthaelt exakt den zuvor inline ausgefuehrten
XML-/Metrik-/Event-Ermittlungspfad. `build_local_voice_batch_plans()` konsumiert
danach diese Observation und fuehrt die bereits vorhandene Qualifikation und
Planbildung in derselben Reihenfolge fort.

## Explizit unveraendert

Dieser Take veraendert nicht:

- `local_voice_batch_triggered_beats()`,
- `local_voice_batch_build_plan()`,
- `local_voice_batch_canonical_beam_plan()`,
- Beam-Zustaende oder Beam-Level,
- Bedingungen, Guard-Reihenfolge oder Blockertexte,
- Handspannen-Gate,
- Role-Transfer-Entscheidungen,
- Node-Patches oder XML-Materialisierung,
- Snapshots, Readback oder Rollback,
- Outer-Writer-/Commit-Grenzen,
- Schwellenwerte, Projekt-/Etueden-/Taktselektion oder musikalische Regeln.

Take 2 und Take 3 bleiben ausserhalb des Scopes.

## Statische Aequivalenz

Die aus `build_local_voice_batch_plans()` verschobene Observation besteht aus
demselben `roxmltree::Document::parse`, derselben Part-/Measure-Reihenfolge,
denselben geerbten `divisions`, `beats` und `beat-type`, denselben
Fail-closed-Pruefungen und demselben Aufruf von
`local_voice_batch_parse_scope_events()` mit identischen Argumenten.

Auf Parse-Fehler liefert die neue Observation `parse_ok=false`;
`build_local_voice_batch_plans()` liefert dann wie zuvor den Default-Build. Auf
erfolgreichem Parse wird `parse_ok=true` gesetzt und die Eventfolge unveraendert
in dieselbe `(scope_ordinal, voice)`-Gruppierung gegeben.

Die Decision- und Mutation-Funktionen werden in diesem Take nicht editiert.
Damit entsteht erstmals eine lesbare Grenze:

`MusicXML -> LocalVoiceBatchObservation -> bestehende Qualification/Planbildung`

und noch bewusst **nicht** die spaetere Decision- oder Writer-Zerlegung.

## Erwarteter Beweis auf dem Zielsystem

Der verbindliche Lauf ist:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Da der Take verhaltensneutral ist, muessen normalisierte Master-MusicXML,
normalisierte Render-MusicXML und alle PDF-Seiten gegen den zuvor bestaetigten
Arbeitsstand identisch bleiben. Erst nach diesem Beweis ist Take 1 abgeschlossen.
