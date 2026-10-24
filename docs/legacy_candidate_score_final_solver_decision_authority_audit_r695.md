# LegacyCandidateScoreFinalSolverDecisionAuthorityAudit r695

## Zweck

Dieser Take ist der zweite und letzte Vorbereitungsschritt vor der geplanten Entfernung der alten Retention-Policy aus der produktiven Kandidatenbewertung.

Parent ist der angenommene Stand r694 `LegacyCandidateScoreDecisionAuthorityPreparationAudit`.

r694 hat für Etüde Nr. 5 gezeigt:

- 124 beobachtete SourceSelection-Kandidaten;
- 76 davon mit einem produktiv berechneten Retention-Strafanteil;
- Summe dieser Strafanteile 32026 Punkte;
- Maximum 878 Punkte;
- bester immutable SourceCandidate nach Legacy-Gesamtscore: Candidate 190;
- bester immutable SourceCandidate ohne Retention-Strafanteil: ebenfalls Candidate 190;
- finaler Source-Ursprung des Solvergewinners: Candidate 4.

Damit ist bewiesen, dass die reine SourceCandidate-Gegenrechnung aus r694 nicht die gesamte spätere Solverentscheidung beschreibt. Ein SourceCandidate kann nach seiner Erzeugung durch weitere Reparatur- und Auswahlstufen verändert werden, während seine SourceSelectionCandidateID als Herkunft erhalten bleibt.

r695 verändert weiterhin keine produktive Auswahl.

## Neue Trennung

Der Audit unterscheidet jetzt ausdrücklich zwei Ebenen:

1. `immutable_source_origin_row`
   - unveränderliche Bewertung des ursprünglich erzeugten SourceSelection-Kandidaten;
   - genau diese Ebene wurde bereits in r694 kontrafaktisch ausgewertet.

2. `final_solver_state_evaluation`
   - Bewertung des tatsächlich finalen, nachgelagert weiterverarbeiteten Solverzustands;
   - diese Bewertung stammt aus dem bereits vorhandenen `MidiSeedStats.source_selection_candidate_evaluation` des finalen Gewinnerzustands.

Damit wird ein irreführender Schluss verhindert: Der Score des Source-Ursprungs ist nicht automatisch der Score des finalen Solverzustands.

## Read-only Frontier-Audit

r695 ergänzt ausschließlich Audit-Ausgaben. Gemessen werden:

- Gesamtscore des finalen Solverzustands;
- darin bereits vorhandener Retention-Strafanteil;
- rein kontrafaktischer finaler Solver-Score `score_total - retention_penalty`;
- Score desselben finalen Zustands in der vorhandenen SolverFrontier;
- Konsistenz zwischen finaler Evaluation und SolverFrontier;
- minimaler Legacy-Score innerhalb der vorhandenen SolverFrontier;
- zugehörige Iteration, Source-Herkunft und Profilname;
- ob der finale Solverzustand zugleich der minimale Legacy-Score der Frontier ist;
- ob der finale Source-Ursprung mit dem besten immutable SourceCandidate übereinstimmt;
- ob eine mehrstufige Entscheidungsautorität zwischen Source-Erzeugung und finalem Solverzustand beobachtet wird.

Die Gegenrechnung besitzt weiterhin keinerlei Selection-, Solver-, Candidate-, Writer- oder MusicMutation-Authority.

## Keine neue Retention-Regel

r695 dupliziert weder die bestehenden Retention-Schwellen noch deren Gewichte. Es übernimmt ausschließlich den bereits produktiv berechneten `retention_penalty` und subtrahiert diesen read-only vom bereits produktiv berechneten Gesamtscore.

Damit wird die zu entfernende Altmechanik nicht als neuer Test- oder Auditvertrag konserviert.

## Tests

Zusätzlich zu den r694-Tests wird geprüft:

1. Der Audit verwendet für den finalen Solverzustand dessen aktuelle Evaluation und nicht irrtümlich die immutable Bewertung seines Source-Ursprungs.
2. Eine Abweichung zwischen finaler Evaluation und dem gespeicherten SolverFrontier-Score wird sichtbar gemacht, aber nicht repariert oder als neue Authority interpretiert.

Die Tests kodieren keine gewünschte Retentionquote.

## Erwarteter Befund für Etüde Nr. 5

Der r694-Lauf hat bereits folgende bestehende Werte geliefert:

- finaler Solverzustand: Iteration 475;
- finaler Source-Ursprung: Candidate 4;
- finaler Solver-Score: 611;
- finaler ArrangementVisibleRetention-Strafanteil nach aktueller produktiver Formel: 611.

Wenn r695 diese bereits vorhandenen Daten konsistent bindet, muss die read-only Gegenrechnung für den finalen Solverzustand `0` ergeben. Dieser Wert ist keine neue Auswahlentscheidung, sondern nur die explizite Sicht auf den bereits beobachteten Legacy-Score ohne dessen Retention-Komponente.

## Abschlussbeweis

Der reale Windows-Take muss zeigen:

- Admission GREEN;
- Strict Clippy GREEN / 0 Warnungen;
- Build GREEN;
- alle Tests GREEN;
- ProgramExit 0;
- Renderer GREEN;
- keine MusicXML-/Render-MusicXML-Mutation gegenüber r694;
- `LegacyCandidateScoreDecisionAuthorityChanged: nein`;
- `LegacyCandidateScoreWeightingChanged: nein`;
- `LegacyCandidateScoreRetentionPolicyChanged: nein`;
- `LegacyCandidateScoreMusicMutation: nein`;
- finale SolverEvaluation und SolverFrontier sind konsistent gebunden.

Danach ist die Vorbereitung abgeschlossen. Der folgende fachliche Take darf die alte Retention-Policy gezielt aus der produktiven Kandidatenbewertung entfernen.
