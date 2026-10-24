# SixteenthRestVerticalClearanceDebtRetirement r525

## Ausgangspunkt

Parent ist der committed GreenBase `v0.10.1839 r524`
`TakeBootstrapCandidateProvenanceTypeComplexityWarningDebtClosure`.

Der r524-Vollkorpus ist 10/10, `FinalExit 0`, `Warnings 0`, `dark_green` und
`released`. Der Infrastrukturblock r521-r524 bleibt unangetastet.

Der fachliche Vorzeichenabstand von Etuede 1 / Takt 3 wurde in r514 bereits an
der eigentlichen horizontalen Ursache geschlossen. Der writerfreie
`final_notation_timeline_facts`-Owner erkennt die strukturelle
`following_accidental_clearance_edge`; `GlobalHorizontalSystemReflow` addiert
dafuer aus den bestehenden Modellgroessen die horizontale Reserve und besitzt
weiterhin die Systempartition. Dieser Pfad ist der produktive Root-Cause-Fix.

Aus r508 ist daneben noch eine aeltere vertikale Sonderfaehigkeit aktiv: Bei
einer 16tel-Pause zwischen unmittelbar benachbarten Noten derselben
diatonischen Staffposition mit folgendem neuem Vorzeichen wird die Pause vom
kanonischen Mittelanker weg verschoben. Im Bassschluessel ergibt das fuer den
primaeren Fall `D3 -> B2`.

Nach r514 ist diese zweite vertikale Clearance-Strategie redundant und bildet
unnuetige Mutation-Debt.

## Retirement

r525 entfernt ausschliesslich die r508-Vertical-Clearance-Faehigkeit aus der
bestehenden Rest-Placement-/Final-Rest-Writer-Kette:

- `fixed_staff_rest_placement_plan_with_following_accidental_clearance()` wird
  entfernt;
- die ausschliesslich dafuer benoetigte diatonische Anchor-Hilfslogik wird
  entfernt;
- `RestStaffSlot::Lower` wird entfernt, weil diese Variante nach dem Retirement
  nicht mehr konstruiert wird;
- die lokale r508-Nachbarschaftserkennung im Final-Rest-Consumer wird entfernt;
- der Final-Rest-Writer verwendet fuer sichtbare Pausen wieder ausschliesslich
  den kanonischen typed Rest-Placement-Plan;
- r508-spezifische Auditzaehler und Placement-Class-Vokabeln werden entfernt;
- Regressionen fuer die beiden zuvor verschobenen 16tel-Kontexte binden nun die
  kanonische Basslage `D3`.

Es wird kein Ersatz-Workaround eingefuehrt.

## r514 bleibt autoritativ

Nicht entfernt oder veraendert werden insbesondere:

- `final_notation_timeline_facts`;
- `following_accidental_clearance_edges` als writerfreie Final-Notation-Fakt;
- `HorizontalLayoutFactSnapshot::capture_final_notation()`;
- `global_reflow_measure_demand()`;
- die modellskalierte Clearance-Reserve aus
  `GLOBAL_REFLOW_NOMINAL_MEASURE_DEMAND / GLOBAL_REFLOW_TYPICAL_SYSTEM_MEASURES`;
- Global-Reflow-Systempartition und bestehender HorizontalLayout-Materializer.

Damit bleibt die Abhaengigkeit weiterhin streng vorwaerts:

```text
finale Notationsquelle
-> writerfreie Timeline-/Clearance-Fakten
-> bestehender Global-Reflow-Demand
-> bestehende Systempartition
-> bestehender HorizontalLayout-Materializer
-> Renderer
```

## Invarianten

- keine klingende Pitch-Mutation;
- Onset unveraendert;
- Duration unveraendert;
- Staff/Voice unveraendert;
- Beam/Tie unveraendert;
- Restdauer und Restanzahl unveraendert;
- Master-MusicXML unveraendert;
- kein neuer Writer oder Writer-Caller;
- keine neue Rest-, Layout-, Repair- oder Recovery-Authority;
- keine Etueden-, Takt- oder Pitchidentitaet in produktiver Logik;
- r509 Measure-Rest-Closure bleibt unveraendert;
- r511 Register-Closure bleibt unveraendert;
- r523 CorpusRunManifest-ClefWriteCause bleibt unveraendert;
- r524 Bootstrap-/Warning-Debt-Closure bleibt unveraendert.

Die beabsichtigte sichtbare Render-Mutation ist das Retirement der r508-
Sonderkoordinate: Ein betroffener Bass-16tel-Rest kehrt von `B2`/`F3` auf die
kanonische typed Mittelposition `D3` zurueck. Die notwendige horizontale
Clearance bleibt durch r514 erhalten.

## Regression und Abnahme

Source-seitig muss nach dem Take gelten:

```text
RestStaffSlot::Lower                                      absent
fixed_staff_rest_placement_plan_with_following_accidental_clearance absent
following_printed_accidental_pitch_for_rest              absent
r508 FollowingAccidentalClearance Writer-Zaehler         absent
following_accidental_clearance_edges (r514)              present
```

Die r525-Regressionsfaelle pruefen fuer die frueheren r508-Kontexte die
kanonische Basslage `D3`. Die bestehenden r514-Tests pruefen weiterhin, dass
eine Final-Notation-Clearance-Kante den modellskalierten Horizontal-Demand
erhoeht.

Erste reale Abnahme:

```powershell
./run.cmd --take "Etüde Nr. 1"
```

Danach visuell und im Render-MusicXML:

- Etuede 1 / Takt 3: 16tel-Pause wieder kanonisch; kein erneuter Kontakt zum
  folgenden Vorzeichen;
- Etuede 3 / Takt 7: kanonische Restlage; horizontale Clearance bleibt;
- Etuede 5 / Takt 14: kanonische Restlage; horizontale Clearance bleibt.

Erst danach:

```powershell
./run.cmd --all
```

Ziel: 10/10, `FinalExit 0`, `Warnings 0`, Manifest gueltig,
`writer_outcome_observed true`, `dark_green`, `released`.

## ChangeClass

`MusicMutation`.

Der Take pensioniert eine bestehende sichtbare Render-Restkoordinaten-Mutation.
Er fuehrt keine neue Musikregel ein und mischt keine Infrastruktur- oder
Kodexarbeit bei.
