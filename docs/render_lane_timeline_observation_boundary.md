# RenderLaneTimelineObservationInterfaceClosure

## Revisionsbefund r02

Der erste r01-Versuch wurde vor jeder Installation vom
`Take-CrossFile-Interface-Closure` korrekt abgelehnt. `beams` war im neuen
`RenderLaneTimelineEvent` nicht entfernt; der kanonische Typ enthaelt das Feld
unveraendert. Der Gate erkannte jedoch die nur ueber einen lokalen Typalias
ausgedrueckte historische `DistinctLaneTxnEvent`-Schnittstelle nicht als
explizit geschlossenen Cross-File-Vertrag.

r02 lockert keinen Gate. Stattdessen werden saemtliche verbleibenden
`DistinctLaneTxnEvent`-/`DistinctLaneTxnMeasure`-Consumer in den beiden
betroffenen Shards direkt auf `RenderLaneTimelineEvent` und
`RenderLaneTimelineMeasure` umgestellt. Damit existiert nur noch der kanonische
Event-/Measure-Typ; insbesondere werden keine Beam-, Cursor-, Staff-, Voice-,
Hidden-, Chord- oder Pitchdaten kopiert oder parallel gehalten.

## Ausgangslage

v0.10.1757 hat die historische Beat-Scoped-Rest-Completion erstmals als
schreibfreien `BeatScopedRestCompletionGapPlan` von ihrer XML-Materialisierung
getrennt. Der produktive Gap-Entscheid bleibt dort absichtlich identisch zur
Legacy-Semantik und gruppiert sichtbare gepitchte Ereignisse nach
`(staff, voice)`.

Die Shadow-Evidence zeigte zugleich, dass EtuSchlopf bereits eine staerkere
MusicXML-Timeline-Wahrheit besitzt: die bisher fuer DistinctLane verwendete
Lane-Observation mit einem `lane_epoch`, der an `<backup>` wechselt.

Die reale v0.10.1757-Abnahme ist dunkelgruen. Damit darf die naechste
Ownership-Grenze ohne gleichzeitigen Musik-Cutover gezogen werden.

## Forensische Praezisierung

Der Cross-Staff-Fall darf nicht auf die Aussage reduziert werden:

`Aktivitaet derselben Voice im Gap -> Gap verwerfen`.

Ein serieller Staff-2-Gap kann zum Beispiel `24..72` umfassen, waehrend
dieselbe Lane/Voice nur `24..48` auf Staff 1 belegt. `48..72` bleibt echte
serielle Stille. Die spaetere korrekte Rest-Completion muss deshalb die
Belegung aller pitched Events derselben `(lane_epoch, voice)` als Intervalle
betrachten und deren Komplement innerhalb des Projektionsgaps bilden.

Damit ist auch klar, warum ein einfacher Schluesselwechsel von
`(staff, voice)` auf `(lane_epoch, voice)` noch keine ausreichende
Produktregel waere.

## Dieser Take

v0.10.1758 zieht ausschliesslich die bereits vorhandene MusicXML-Cursor- und
Lane-Beobachtung aus
`lower_dual_edge_lane_timeline_planning_core.rs` in den neutralen,
schreibfreien Owner:

`render_lane_timeline_observation_core.rs`

Der neue Owner enthaelt:

- `RenderTimelineNoteObservation`,
- `RenderLaneTimelineEvent`,
- `RenderLaneTimelineForward`,
- `RenderLaneTimelineMeasure`,
- `render_lane_timeline_measures()`.

Die Semantik bleibt identisch:

- Nicht-Chord-Noten treiben den seriellen Cursor voran.
- Chords verwenden den letzten Non-Chord-Onset und treiben den Cursor nicht.
- `<forward>` treibt den Cursor voran.
- `<backup>` setzt den Cursor zurueck und erhoeht `lane_epoch`.
- Staff-Wechsel allein erzeugen keine neue Lane-Epoche.
- Hidden pitched Events bleiben Timeline-Ereignisse.

DistinctLane und der Cross-Staff/LaneFold-Consumer verwenden diese kanonische
Observation direkt. Die historischen Event-/Measure-Typaliases entfallen in r02;
es gibt damit keinen zweiten Typnamen fuer dieselbe Timeline-Wahrheit.

Die Rest-Completion-Tests verwenden den kanonischen Timeline-Owner direkt.
Zusaetzliche generische Tests binden Staff-Wechsel, Backup-Epoch und
Chord-Ankersemantik an genau diesen Owner.

## Unveraendert

Keine Aenderung an:

- `plan_visible_voice_rest_completion_gaps()`,
- `(staff, voice)`-Gap-Selektion,
- `beat_bounded_rest_segments()`,
- `MetricRestNotation`,
- BeamPlan oder Beam-Regeln,
- XML-Materialisierung,
- Writer-Reihenfolge,
- MusicXML-/PDF-Semantik,
- Gates, Schwellenwerten oder Selektoren.

Keine Etueden-, Takt-, Staff-, Pitch- oder Onset-Sonderregel.

## Naechster moeglicher Schritt

Nach gruener Abnahme kann ein schreibfreier Silence-Projection-Plan aus der
kanonischen Lane-Timeline entstehen. Dieser Plan muss fuer jeden bestehenden
Staff-Projektionsgap die pitched Belegungsintervalle derselben
`(lane_epoch, voice)` abziehen und nur die verbleibenden echten
Stille-Teilintervalle liefern.

Erst wenn dieser Plan gegen die Legacy-Ausgabe und die all-pitched Timeline
bewiesen ist, darf eine produktive Rest-Completion-Semantik geaendert werden.
`MetricRestNotation` bleibt weiterhin nachgeordnet.
