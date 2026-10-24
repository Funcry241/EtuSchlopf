# BeatScopedRestCompletionPlanBoundary

## Ausgangslage

v0.10.1756 ist der autoritative gruene Rollback nach dem musikalisch falschen
v0.10.1755-Cutover. Der historische Kombipass
`enforce_beat_scoped_beaming_and_rest_completion_in_xml()` entscheidet dort
weiterhin gleichzeitig ueber Beam-Zustaende, Rest-Completion, Rest-Splitting
und XML-Materialisierung.

Die forensische Rekonstruktion des bekannten Regressionsankers zeigt eine
allgemeine Strukturklasse: dieselbe Voice kann in einem seriellen MusicXML-
Cursorlauf zwischen Staff 1 und Staff 2 wechseln. Die bisherige
Rest-Completion gruppiert sichtbare gepitchte Ereignisse dagegen nach
`(staff, voice)`. Damit kann sie einen Cross-Staff-Abschnitt derselben Voice als
lokale Staff-Luecke interpretieren.

Im Regressionsmuster von Etuede 1/Takt 13 liegen vor den spaeter zu frueh
erscheinenden gepitchten Ereignissen vier serielle 24-Division-Resttraeger.
Der v0.10.1755-Fehler wuchs entsprechend 24, 48, 72 und 96 divisions an. Das
ist ein Cursortraeger-/Timelinefehler, kein Pitch-Reordering.

## Dieser Take

v0.10.1757 trennt ausschliesslich die bestehende Rest-Completion-Entscheidung
von ihrer XML-Materialisierung.

Neu ist der schreibfreie Planvertrag:

`BeatScopedRestCompletionGapPlan`

mit:

- exaktem `insert_before`-Source-Offset,
- Staff,
- Voice,
- Gap-Start,
- Gap-Ende.

`plan_visible_voice_rest_completion_gaps()` reproduziert die bisherige
`(staff, voice)`-Entscheidung unveraendert. Der bestehende
`add_visible_voice_rest_completion()` konsumiert danach nur noch diese
Planzeilen, zerlegt die bereits entschiedenen Gaps weiterhin mit der bisherigen
`beat_bounded_rest_segments()`-Logik und materialisiert dieselben XML-Rests wie
zuvor.

Die bisher nur von Rest-Completion verwendete
`existing_visible_rests_cover_range()`-Logik verlaesst dabei den
Metric-Beam-Planning-Core und gehoert jetzt zum Rest-Completion-Planning-Core.
Die eigentliche Beam-Planung bleibt unveraendert.

## Shadow-Evidence ohne Produkt-Cutover

Der Take aendert bewusst noch nicht die Voice-/Timeline-Wahrheit.

Statt `PartialBeatNoteInfo` zu einem neuen Universalmodell aufzublaehen, nutzen
Tests die bereits vorhandene `DistinctLaneTxnMeasure`-Observation als
Shadow-Truth. Dort ist `lane_epoch` schon definiert und wechselt an `<backup>`.
Die Tests beweisen allgemein:

- Staff-Wechsel innerhalb derselben seriellen Lane erzeugen keinen neuen
  `lane_epoch`;
- ein `<backup>` trennt gleiche Voice-Nummern in verschiedene Lane-Epochen;
- die heutige `(staff, voice)`-Rest-Completion kann einen Gap planen, obwohl im
  gleichen `lane_epoch` und in derselben Voice auf dem anderen Staff gepitchte
  Aktivitaet liegt.

Damit ist die naechste Ownership-Grenze bewiesen, ohne bereits eine neue
produktive Silence-Regel einzufuehren.

## Unveraendert

Keine Aenderung an:

- Musikregeln,
- Rest-Gap-Selektion,
- Rest-Dauersegmentierung,
- BeamPlan oder Beam-Regeln,
- Writer-Reihenfolge,
- MusicXML-/PDF-Semantik,
- Structural Write Authority,
- Gates oder Schwellenwerten.

Keine Etueden-, Takt-, Staff-, Pitch- oder Onset-Sonderregel.

## Naechster moeglicher Schritt

Erst nach gruener Abnahme dieses Boundary-Takes kann die gemeinsame
Timeline-Observation neutral aus der bereits vorhandenen Lane-Wahrheit
herausgeloest bzw. fuer Rest-Completion konsumierbar gemacht werden. Erst dann
darf der Rest-Completion-Plan von `(staff, voice)` auf bewiesene Voice-/Lane-
Stille umgestellt werden. `MetricRestNotation` bleibt bis dahin ausserhalb
dieses historischen Kombipasses.
