# FinalNotationTimelineClearanceSystemReflowCutover r514

## Ausgangspunkt

r513 `FinalNotationTimelineClearanceFactOwnerExtraction` ist fuer Etuede 1 installiert und dark green.
Der neutrale writerfreie `final_notation_timeline_facts`-Owner besitzt damit die bereits in r512
bewiesene Clearance-Beobachtung, waehrend der Audit nur noch Consumer ist.

Im finalen Etuede-1-Lauf bleibt die Trennung eindeutig:

- Takt 3 ist der einzige Slot mit `following_accidental_clearance_edges=1`;
- sein bestehender gebundener Measure-Demand ist 117;
- das finale System 3-5 besitzt Load 406 bei Capacity 465;
- alle anderen Etuede-1-Slots besitzen `clearanceEdges=0`.

Damit ist die fehlende Information nicht Renderer-Feedback, sondern bereits vor dem Renderer
verfuegbare Final-Notation-Evidenz.

## Produktiver Cutover

r514 fuehrt keine neue Width-, Renderer- oder Writer-Authority ein. Der bestehende
`GlobalHorizontalSystemReflow` bleibt Demand-/Systempartition-Owner und der bestehende
HorizontalLayout-Materializer bleibt alleiniger Writer.

Der bestehende `HorizontalLayoutFactSnapshot` behaelt seine generische, identitaetsfreie
`capture()`-Semantik unveraendert. Fuer die drei bereits final-notationsgebundenen
Global-Reflow-/PageCount-/Joint-Page-System-Consumer gibt es nun die schmale
`capture_final_notation()`-Bindung. Sie reichert dieselben Taktfakten slotgenau mit der neutralen
`following_accidental_clearance_edges`-Beobachtung an. Dadurch bleibt der generische Plan-/Testpfad
von strengeren finalen MusicXML-Annahmen unberuehrt.

Der bisherige `visual_layout_pressure_score()` bleibt unveraendert.

Erst `global_reflow_measure_demand()` addiert fuer jede solche Clearance-Kante eine lokale Reserve.
Die Reserve besitzt keinen neuen frei kalibrierten Zahlenwert, sondern wird ausschliesslich aus
bereits vorhandenen Modellgroessen abgeleitet:

```text
clearance_reserve_per_edge =
    ceil(GLOBAL_REFLOW_NOMINAL_MEASURE_DEMAND
         / GLOBAL_REFLOW_TYPICAL_SYSTEM_MEASURES)
```

Bei den bestehenden Modellwerten entspricht dies 34 Demand-Einheiten. Dieser Wert wird nicht aus
Takt 3, dessen Headroom oder einer gewuenschten Systemgrenze abgeleitet. Aendern sich die bereits
autoritativen Nominal-/Typical-Modellgroessen, skaliert die Reserve mit derselben Modellskala.

## Architekturrichtung

Die Abhaengigkeit bleibt streng vorwaerts:

```text
finale Notationsquelle
-> writerfreie Timeline-/Clearance-Fakten
-> bestehender Global-Reflow-Demand
-> bestehende Systempartition
-> bestehender HorizontalLayout-Materializer
-> Renderer
```

Es gibt keinen Pfeil vom Renderer oder Readback zur Notation oder zur Demandentscheidung.

## Nicht geaendert

- kein Rest-, Pitch-, Onset-, Duration-, Voice- oder Staff-Write;
- keine Aenderung von r508-Restpositionen;
- keine `measure/@width`- oder `default-x`-Writes;
- kein neuer HorizontalLayout-Writer oder Writer-Caller;
- keine Capacity-Aenderung;
- keine neue feste Calibration-Schwelle oder Takt-3-spezifische Demandzahl;
- keine Renderer-NoGood-/Feedback-Schleife;
- keine Etueden-, Takt- oder Pitch-Sonderregel;
- keine Audit-Quellaenderung;
- keine Wiederbelebung pensionierter Width-/Register-/Preview-Capabilities.

`ChangeClass=MusicMutation`, weil die bestehende produktive Systempartition durch eine neue
Final-Notation-Demanddimension beeinflusst werden darf. Die musikalischen Ereignisse bleiben
unveraendert.
