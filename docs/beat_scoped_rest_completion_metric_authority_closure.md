# BeatScopedRestCompletionMetricAuthorityClosure

## Ziel

Nach v0.10.1754 lag die sichtbare Dauerentscheidung partieller Pausen fast
vollständig beim writer-freien `MetricRestNotation`-Owner. Ein historischer,
weiterhin produktiv auf Master und Render laufender Final-Patch
`enforce_beat_scoped_beaming_and_rest_completion_in_xml()` besaß jedoch noch
eine eigene Dauerzerlegung über `beat_bounded_rest_segments()`.

Der Patch hat zwei getrennte Aufgaben: Beaming und Rest-Completion. Die
Beaming-Entscheidung bleibt unverändert. Nur die sichtbare Dauerzerlegung
bereits bewiesener bzw. neu zu materialisierender partieller Stillespannen wird
an `MetricRestNotation` gebunden.

## Sicherheitsgrenze

Der Takt-17-Befund aus v0.10.1748 hat gezeigt, dass sichtbare Restanker
versteckte gepitchte `<chord/>`-Events tragen können. Deshalb darf der alte
Final-Patch einen Span nicht allein aus sichtbaren Nicht-Chord-Noten als Stille
klassifizieren.

Vor jeder Rest-Re-Segmentierung wird daher geprüft, ob im betreffenden
Voice-Span irgendein gepitchtes Ereignis liegt, unabhängig von Staff,
`print-object` oder Chord-Status. Ebenso wird eine zwischen zwei sichtbaren
Noten erkannte Rest-Completion verworfen, wenn dieselbe Voice im Gap hidden
oder cross-staff gepitchtes Material enthält.

Damit erhält `MetricRestNotation` nur einen tatsächlich bewiesenen
Voice-Stillespann.

## Metrischer Vertrag

Die lokale `beat_bounded_rest_segments()`-Policy entfällt vollständig.

Beispiele auf der kanonischen 96er-Zeitbasis:

- `0..288` -> `192 + 96`
- `96..288` -> `96 + 96`
- `0..192` -> `192`

Vollständige Taktruhe (`0..384`) bleibt ausdrücklich unangetastet und damit
beim bestehenden Ganztaktpausen-Owner.

Die lokale MusicXML-Repräsentation erhält zusätzlich die bereits an anderen
Stellen kanonischen `64th`-Abbildungen für 9 und 6 divisions; sie entscheidet
keine Dauer, sondern materialisiert nur einen vom zentralen Planner gelieferten
Wert.

## Nicht geändert

- keine Writerwurzel und kein Writername;
- keine Etüden-, Takt-, Staff- oder Onset-Sonderregel;
- keine Beam-Planung oder Beam-Gruppierung;
- keine Ganztaktpausen-Policy;
- keine Cross-Staff-Handklassifikation;
- keine klingende Pitch-/Onset-/Dauer-Mutation als Ziel dieses Takes;
- keine Gate-, Capability- oder Release-Lockerung.

## Regression

Neue semantische Tests binden vier Fälle:

1. ausgerichtete Drei-Beat-Pause wird `Halbe + Viertel`;
2. Zwei-Beat-Pause ab schwachem Beat bleibt `Viertel + Viertel`;
3. eine pitched Chord-Verankerung blockiert jede Rest-Re-Segmentierung;
4. hidden/cross-staff gepitchtes Material derselben Voice blockiert eine
   sichtbare Gap-Completion.

Erste Abnahme weiterhin nur an Etüde Nr. 1:

```powershell
.\run.cmd --take --strict "Etüde Nr. 1" --display
```

Erwartung: Takt 2 bleibt `Halbe + Viertel + Achtel`, Takt 17 bleibt mit seinen
hidden pitched Chord-Ankern sicher und die vollständige gepitchte Timeline
bleibt stabil.
