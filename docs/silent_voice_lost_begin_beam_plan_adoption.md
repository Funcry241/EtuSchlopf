# SilentVoiceLostBeginBeamPlanAdoption

## Ausgangspunkt

Der `FinalBeamLostBeginRenderWriterStage` repariert genau den Fall, in dem eine
sichtbare, masteridentische Anfangsnote im Render-MusicXML ihre Balkenanfangs-
Tags verloren hat, waehrend alle folgenden Gruppenmitglieder und die komplette
Lane-Timeline bereits korrekt sind.

Der Pfad war schon stark abgesichert: exakte Master-/Render-Identitaet,
vollstaendige Timeline, geschlossene Folgegruppe, keine bebeamte Pause,
autorisierter Bound-Note-Batch, Parse-, Cursor-, Eventmultiset- und Beam-only-
Readback. Die noch lokale Doppelmodellierung lag im Zieltyp: Er transportierte
die erwarteten `begin`-Zustaende als `BTreeMap<u32, String>` und der Writer
baute daraus erneut XML.

## Aenderung

`FinalBeamTarget` traegt jetzt einen `BeamPlan`.

`final_beam_begin_plan()` projiziert die bereits bewiesenen Masterzustände in
einen typisierten Ein-Ereignis-Plan. `BeamPlan::from_explicit_states()` prueft
dabei fail-closed:

- mindestens ein Ereignis und mindestens ein Edit,
- jedes Ereignisordinal liegt im Plan,
- Balkenebene ist groesser als 0,
- kein doppeltes `(Ereignisordinal, Balkenebene)`.

Der XML-Adapter serialisiert ausschliesslich die Edits dieses Plans. Die
abschliessende Beam-only-Pruefung vergleicht dasselbe Planobjekt mit den
zurueckgelesenen `<beam>`-Tags. Writer und Readback koennen dadurch nicht mehr
unterschiedliche lokale Zielmaps besitzen.

## Nicht geaendert

- keine Master-/Render-Kandidatenregel,
- keine Timeline-, Staff-, Voice-, Onset-, Dauer- oder Gruppengrenze,
- kein MusicXML-Parser fuer beobachtete Beam-Zustaende,
- kein autorisierter Batch-Writer oder produktiver Sink,
- keine Pitch-, Onset-, Dauer-, Staff-, Voice-, Pausen- oder Notenzahlaenderung,
- keine Pipeline-Reihenfolge,
- keine neue Lintunterdrueckung, Audit- oder History-Schicht.

## Statische Aequivalenz

- 200.000 zufaellige beobachtete Beam-Level-/State-Konstellationen mit Level 1
  bis 8 lieferten dieselbe Annahme oder Ablehnung wie die alte Begin-Map.
- 200.000 akzeptierte Levelmengen mit korrekten, fehlenden, veraenderten und
  zusaetzlichen Readback-Tags lieferten dieselbe Serialisierung und dieselbe
  Beam-only-Annahme oder -Ablehnung.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Bis zur realen gruenen Abnahme bleibt v0.10.1603 autoritativ.
