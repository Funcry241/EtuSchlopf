# FinalPartialBeatTripleBeamPlanAdoption

## Ausgangspunkt

Der spaete `PartialBeatPostRestTripleBeamFinalRepair`-Pfad ist mehr als ein
Balkenwriter: Er bindet genau eine strukturelle untere Post-Rest-Dreiergruppe,
verschiebt cursorneutral ein Forward gegen die dritte Note und beweist danach
vollstaendige Eventmultiset- und Cursorinvarianz. Trotz der bereits migrierten
allgemeinen Partial-Beat-Regeln erzeugte dieser Finalpfad seine sechs
Balkenzustaende noch lokal aus `begin`, `continue` und `end`.

## Aenderung

`human_bound_exact_double_beam_plan()` erzeugt einen gemeinsamen
`BeamPlan` fuer drei Ereignisse und zwei vollstaendige Balkenebenen.

Der Schreibpfad:

1. bindet weiterhin dieselben drei Zielnoten,
2. projiziert die typisierten Plan-Edits auf deren bestehende XML-Positionen,
3. verwendet unveraendert `partial_beat_strip_beam_tags` und
   `partial_beat_insert_beam_tags`.

Der Readback prueft dieselben Zielnoten gegen genau die Edits desselben Plans.
Damit koennen Writer und Verifier nicht mehr getrennte lokale
Balkenzustandsfolgen besitzen.

## Nicht geaendert

- keine Kandidaten- oder Singularitaetsregel,
- kein Staff-, Voice-, Beat-, Rest- oder Cursorkriterium,
- kein Lane-Stitch und kein Forward/Backup-Verhalten,
- kein XML-Writer oder Checkpoint-Sink,
- keine Pitch-, Onset-, Dauer-, Staff-, Voice-, Pausen- oder Notenzahlaenderung,
- keine Pipeline-Reihenfolge,
- keine neue Lintunterdrueckung, Audit- oder History-Schicht.

## Statische Aequivalenz

- 200.000 zufaellige Abbildungen der drei Planordinale auf XML-Notenpositionen
  lieferten dieselben sechs `(Note, Level, State)`-Zeilen.
- 200.000 korrekte, fehlende, vertauschte, duplizierte und zusaetzliche
  Readback-Tag-Konstellationen lieferten dieselbe Annahme oder Ablehnung.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Bis zur realen gruenen Abnahme bleibt v0.10.1602 autoritativ.
