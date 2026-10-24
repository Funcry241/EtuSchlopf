# r194 ArrangementSolverCandidateMovePerformanceClosure

## Ziel

Der Arrangement-Solver konsumiert seinen `ArrangementSolverSearchState` am Ende der Suchphase. Bis r193 wurden dennoch alle verworfenen `SolverCandidate`-Werte mit `candidates.iter().cloned()` tief kopiert, bevor sie in `all_for_lines` fuer nachgelagerte Auswertung und Berichtsfunktionen uebernommen wurden.

Ein `SolverCandidate` enthaelt unter anderem das vollstaendige Layout-MusicXML sowie umfangreiche Check-, Arrangement-, Polish- und Layoutdaten. Bei grossen Kandidatenmengen erzeugt die tiefe Kopie daher vermeidbare Speicher- und Kopierarbeit.

## Aenderung

`finish_arrangement_solver_stage` reserviert die Zielkapazitaet einmal, kopiert weiterhin genau den separat benoetigten `best`-Kandidaten und verschiebt anschliessend die bereits besessene `candidates`-Sammlung direkt in `all_for_lines`.

Vor dem unveraenderten `sort_by_key(|c| c.iteration)` bleibt die Reihenfolge identisch zu r193: zuerst `best`, danach die bisherigen Kandidaten. Kandidateninhalt, Iterationen, Auswahl, Scores, Pruefberichte, MusicXML und alle nachgelagerten Verbraucher bleiben unveraendert.

## Sicherheits- und Musikgrenze

- keine Schwelle, Gewichtung oder Auswahlregel geaendert
- keine Kandidaten entfernt oder hinzugefuegt
- keine Parallelisierung oder Reihenfolge der Solverbewertung geaendert
- keine Writer-, Renderer- oder Security-Authority geaendert
- keine architektur-gepinnte Datei betroffen
- erwartete Master- und Render-MusicXML-Ausgabe byte-identisch zu r193

Die Aenderung ist reine Ownership-/Allokationsbereinigung und soll nur die vermeidbare Deep-Clone-Spitze am Ende der Solver-Suchphase beseitigen.
