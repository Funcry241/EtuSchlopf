# Beam Continuity Mutation / Verification Boundary

## Scope

`v0.10.1686 r01` ist ausschliesslich ein verhaltensneutraler `RefactorOnly`-Take
innerhalb von
`src/pipeline_shards/lower_dual_edge_beat_beam_continuity_repair.rs`.

Der reale Zielsystembeweis fuer Take 2 (`v0.10.1685`) ist gruen:

- BuildExit 0
- ClippyExit 0
- FinalExit 0
- 0 Blocker
- 0 Warnungen
- normalisierte Master-MusicXML gegen v0.10.1684 identisch
- normalisierte Render-MusicXML gegen v0.10.1684 identisch
- PDF 6/6 Seiten pixelidentisch
- 0 veraenderte Pixel

Damit ist die Observation-/Decision-Seite abgeschlossen. Take 3 schneidet nur
die bereits vorhandene Mutation und deren Readback aus dem verbleibenden
Batch-Orchestrator.

## Bestehender RepairPlan wird wiederverwendet

Es wird bewusst kein neuer `BeamContinuityRepairPlan` eingefuehrt.

`LocalVoiceBatchPlan` beschreibt bereits vollstaendig, was geschrieben werden
soll:

- Cell-Key,
- optionaler kanonischer `BeamPlan`,
- `LocalVoiceBatchNodePatch`-Liste,
- Beam-Patch-Zaehler,
- Role-Transfer-Fakten,
- erwartetes Note-Delta,
- Dauer-/Rest-/Hidden-Invarianten,
- Orthogonal-Span-Gate,
- Decision-Ergebnis ueber `green`.

Ein zweiter Planvertrag waere reine Doppelstruktur. Take 3 macht deshalb den
bestehenden Plan sichtbar zum Eingang der Mutation.

## Mutation Boundary

Die bisher als lokale Closure in
`patch_lower_dual_edge_beat_beam_continuity_in_xml()` enthaltene
Materialisierung liegt nun in:

`local_voice_batch_materialize_plan(source, plan)`

Der Funktionskoerper uebernimmt ohne neue Semantik dieselben Schritte:

1. Node-Bereiche und UTF-8-Grenzen pruefen,
2. Source-Fingerprint pruefen,
3. Remove-/Forward-/Chord-/Hidden-Patches anwenden,
4. Beam-Zustaende aus dem bestehenden `BeamPlan` projizieren,
5. ueberlappende Replacements ablehnen,
6. XML in derselben Reihenfolge neu zusammensetzen,
7. Parse-Readback ausfuehren,
8. ChangedNodes und ChangedBeamTags unveraendert zaehlen.

Fehlerfaelle bleiben fail-closed und liefern wie bisher das unveraenderte
Source-XML, `false`, 0 ChangedNodes und 0 ChangedBeamTags. Der bisherige
Tuple-Vertrag ist nur in `LocalVoiceBatchMaterialization` typisiert.

## Candidate Verification Boundary

Die nach jeder Materialisierung bereits vorhandenen Invarianten liegen nun in:

`local_voice_batch_verify_candidate(...)`

Unveraendert geprueft werden:

- Parse nach Mutation,
- Rebuild des Plan-Ledgers,
- selektierter Plan ist verschwunden,
- erwartetes Note-Delta,
- sichtbares klingendes Multiset,
- geordnete klingende Ereignisse,
- Zahl gruener Plaene sinkt,
- Zahl blockierter Plaene steigt nicht,
- bestehender Role-Transfer-Vertrag.

`green` ist exakt die bisherige Konjunktion dieser Bedingungen.

## Final Verification Boundary

Der abschliessende Batch-Readback liegt nun in:

`local_voice_batch_verify_final(...)`

Unveraendert geprueft werden:

- finaler Plan-Rebuild,
- finaler Snapshot,
- Parse,
- 0 verbleibende gruene Plaene,
- blockierte Plaene nicht ueber Initialstand,
- sichtbare klingende Invariante,
- geordnete klingende Invariante,
- bestehender Role-Delta-Vertrag,
- kein vorheriger Batch-Fehler.

`role_delta_authorized` und `exact_final_readback` entstehen aus denselben
Bedingungen wie zuvor.

## Explizit unveraendert

Byteidentisch gegen v0.10.1685 bleiben unter anderem:

- `apply_lower_dual_edge_beat_beam_continuity_repair()` – 652 Zeilen,
- `observe_local_voice_batch()` – 43 Zeilen,
- `build_local_voice_batch_plans()` – 50 Zeilen,
- `local_voice_batch_triggered_beats()` – 119 Zeilen,
- `local_voice_batch_parse_scope_events()` – 80 Zeilen,
- `local_voice_batch_decide()` – 95 Zeilen,
- `local_voice_batch_build_plan()` – 169 Zeilen,
- `local_voice_batch_onset_groups()` – 35 Zeilen,
- `local_voice_batch_canonical_beam_plan()` – 21 Zeilen,
- `local_voice_batch_snapshot()` – 63 Zeilen,
- `patch_lower_dual_edge_beat_beam_continuity_in_measure()` – 98 Zeilen.

Damit bleiben Observation, Qualification, Decision und RepairPlan selbst
unangetastet.

## Resultierende Kette

Nach Take 3 ist der Local-Voice-Batch-Pfad explizit lesbar als:

`Observation`
-> `Trigger Qualification`
-> `Decision`
-> `LocalVoiceBatchPlan`
-> `local_voice_batch_materialize_plan`
-> `local_voice_batch_verify_candidate`
-> Commit oder Rollback
-> `local_voice_batch_verify_final`

Der bisherige XML-Batch-Orchestrator
`patch_lower_dual_edge_beat_beam_continuity_in_xml()` sinkt durch diese
Trennung von 337 auf 199 Zeilen. Die ausgelagerte Mutation umfasst 106 Zeilen,
Candidate Verification 50 Zeilen und Final Verification 40 Zeilen.

## Keine musikalische Aenderung

Dieser Take aendert nicht:

- Beam-Regeln,
- Beam-Zustaende,
- Scores oder Gewichte,
- Schwellenwerte,
- Staff-/Voice-/Pitch-/Onset-/Duration-Entscheidungen,
- Role-Transfer-Regeln,
- Handspannen-Gates,
- Writer-Reihenfolge,
- Commit-/Rollback-Reihenfolge,
- MusicXML-/Render-Auswahl,
- Projekt-, Etueden- oder Taktselektion.

Es gibt keine Etueden- oder Takt-spezifische Logik.

## Erwarteter Zielsystembeweis

Genau ein Lauf:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Erwartung:

- BuildExit 0
- ClippyExit 0
- FinalExit 0
- normalisierte Master-MusicXML identisch zu v0.10.1685
- normalisierte Render-MusicXML identisch zu v0.10.1685
- PDF 6/6 Seiten pixelidentisch
- 0 veraenderte Pixel

Mit diesem Beweis endet der vereinbarte Beam-Continuity-Superwurf.

**Kein Take 4.**
