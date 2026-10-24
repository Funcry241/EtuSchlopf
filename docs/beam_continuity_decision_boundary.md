# Beam Continuity Decision Boundary

## Scope

`v0.10.1685 r01` ist ausschliesslich ein verhaltensneutraler `RefactorOnly`-Take
innerhalb von
`src/pipeline_shards/lower_dual_edge_beat_beam_continuity_repair.rs`.

Take 1 (`v0.10.1684`) hat die XML-/Metrik-/Event-Beobachtung als
`LocalVoiceBatchObservation` vor die bestehende Qualifikation und Planbildung
geschnitten. Der reale Zielsystembeweis war gruen: BuildExit 0, ClippyExit 0,
FinalExit 0; Master- und Render-MusicXML unterschieden sich gegen v0.10.1683
nur im Software-Versionsstring; alle 6 PDF-Seiten waren pixelidentisch.

Take 2 schneidet nun ausschliesslich die bereits vorhandene Entscheidung aus
`local_voice_batch_build_plan()`.

## Tatsaechliche bestehende Entscheidungszustaende

Der vorhandene Kontrollfluss besitzt genau drei fachliche Ergebnisse:

1. `NoChange`: weder ein Beam-Problem noch ein Role-Transfer-Problem liegt vor;
   wie bisher wird kein `LocalVoiceBatchPlan` erzeugt.
2. `Reject`: ein Kandidat liegt vor, aber mindestens einer der bereits
   vorhandenen Blocker ist aktiv; wie bisher entsteht ein Plan mit
   `green=false`.
3. `Accept`: ein Kandidat liegt vor und alle bestehenden Guards sind gruen; wie
   bisher entsteht ein Plan mit `green=true`.

Diese drei Zustaende werden jetzt durch `LocalVoiceBatchDecision` explizit
benannt. Es werden keine zusaetzlichen musikalischen Zustaende erfunden.

## Decision-Vertrag

`local_voice_batch_decide()` konsumiert ausschliesslich bereits berechnete
Kandidatenfakten ueber den privaten read-only
`LocalVoiceBatchDecisionContext`. Der Kontext enthaelt unter anderem:

- Cell-Key und bestehende Events/Gruppen,
- bereits erzeugten kanonischen BeamPlan,
- bereits geplante Node-Patches,
- Sounding-Intervalle,
- Beam-/Role-Problemflags,
- den bestehenden Hidden-Anchor-Mehrdeutigkeitsbefund,
- `hidden_after`.

Die Funktion schreibt weder XML noch Patches und veraendert keine Events. Sie
entscheidet nur mit den schon vorhandenen Guards zwischen `NoChange`, `Reject`
und `Accept`.

## Guard-Reihenfolge bleibt erhalten

Die bisherige Reihenfolge bleibt semantisch unveraendert:

1. kein Beam-/Role-Problem -> `NoChange`,
2. nicht eindeutiger Hidden-Rest-Anchor-Transfer,
3. Role-Transfer bis zum aeusseren Eventvertrag gesperrt,
4. zu wenige klingende Onsets fuer ein Beam-Problem,
5. Voice-Scope nicht eindeutig,
6. Same-Voice-Overlap,
7. unvollstaendige sichtbare Restabdeckung,
8. verbleibende Hidden-Timeline,
9. bestehendes Handspannen-Gate mit derselben Beam-only-Ausnahme,
10. fehlender kanonischer BeamPlan.

Die bisherigen Blockertexte bleiben identisch. `Accept` entspricht exakt dem
bisherigen `blocking_reasons.is_empty()`, `Reject` dem Gegenfall.

## Explizit unveraendert

Dieser Take veraendert nicht:

- `observe_local_voice_batch()`,
- `build_local_voice_batch_plans()`,
- `local_voice_batch_triggered_beats()`,
- `local_voice_batch_parse_scope_events()`,
- `local_voice_batch_onset_groups()`,
- `local_voice_batch_canonical_beam_plan()`,
- BeamPlan-Zustaende oder Beam-Level,
- Bedingungen, Schwellenwerte oder Guard-Semantik,
- `LocalVoiceBatchPlan` und `LocalVoiceBatchNodePatch`,
- die Patch-/Repair-Inhalte,
- XML-Materialisierung und Writer-Reihenfolge,
- Snapshot, Readback, Invarianten oder Rollback,
- Projekt-/Etueden-/Taktselektion oder musikalische Regeln.

Take 3 bleibt ausserhalb des Scopes.

## Statische Aequivalenz

Die fruehere inline Variable `blocking_reasons` lebt jetzt innerhalb von
`local_voice_batch_decide()`. Der zuvor waehrend der Kandidatenbildung
erkannte nicht eindeutige Hidden-Rest-Anchor wird nur als Zaehler in den
Decision-Kontext transportiert und dort an derselben logischen Position wieder
in denselben Blocker uebersetzt. Dadurch bleibt auch der historische Sonderfall
erhalten, dass ein isolierter Mehrdeutigkeitsbefund ohne Beam-/Role-Problem
weiterhin `NoChange` ergibt.

Alle uebrigen Guards wurden ohne neue Gewichtung und ohne Zusammenlegung
semantisch unterschiedlicher Faelle in die Decision-Funktion verschoben.
`LocalVoiceBatchPlan` wird danach mit denselben Feldern gebaut; nur das bisher
inline berechnete `green` stammt nun explizit aus `Accept` oder `Reject`.

Damit ist die Kette nach Take 2 lesbar als:

`Observation -> Trigger Qualification -> Candidate Plan -> Decision -> bestehender LocalVoiceBatchPlan`

Die eigentliche Mutation/Verification-Grenze ist bewusst noch nicht Teil dieses
Takes.

## Erwarteter Beweis auf dem Zielsystem

Der verbindliche Lauf ist:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Da der Take verhaltensneutral ist, muessen normalisierte Master-MusicXML,
normalisierte Render-MusicXML und alle PDF-Seiten gegen den bestaetigten
v0.10.1684-Arbeitsstand identisch bleiben. Erst danach ist Take 2 abgeschlossen.
