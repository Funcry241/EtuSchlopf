# LegacyCandidateScoreDecisionAuthorityDependencyMapClippyRecovery r697

## Zweck

r697 ist der compile-recovered dritte und letzte Vorbereitungsschnitt vor der produktiven Entfernung der
Legacy-Retention-Policy.

r696 wurde nicht angenommen. Der Windows-Strict-Clippy-Lauf brach vor Build und Tests ausschliesslich wegen eines im Nicht-Test-Build unbenutzten Imports von `SourceSelectionCandidateEvaluation` im Audit-Modul ab. r697 basiert deshalb weiterhin auf dem letzten akzeptierten Parent r695, uebernimmt den fachlichen r696-Inhalt und verschiebt genau diesen nur von Tests benoetigten Import hinter `#[cfg(test)]`. Keine fachliche oder produktive Semantik wird dadurch geaendert.

r694 hat die SourceCandidate-Ebene beobachtbar gemacht. r695 hat zusaetzlich den finalen
Solverzustand an die vorhandene SolverFrontier gebunden. Der reale r695-Lauf zeigte dabei
eine wichtige Beobachtbarkeitsluecke:

```text
FinalSolverFrontierScore = 611
FinalSolverScoreTotal aus CandidateEvaluation = 4319226
FinalSolverFrontierMatchesEvaluation = nein
```

Der Grund ist nicht eine inkonsistente SolverFrontier. Die bisher an `MidiSeedStats`
gebundene `SourceSelectionCandidateEvaluation` beschreibt den unveraenderlichen
SourceSelection-Ursprung. Spaetere Solverstufen veraendern denselben Kandidaten weiter,
waehrend die SourceCandidate-ID absichtlich erhalten bleibt. Candidate-ID-Gleichheit
beweist deshalb keine Score-State-Gleichheit.

## Schnitt

Jede bereits read-only projizierte `SourceSelectionSolverFrontierRow` transportiert in r697
zusaetzlich die fuer genau diesen live Solverzustand erneut aus seinen bereits vorhandenen
Fakten berechnete `SourceSelectionCandidateEvaluation`.

Die Berechnung benutzt ausschliesslich die bereits produktive Funktion
`candidate_score_breakdown` ueber:

- `candidate.final_result`,
- `candidate.fidelity.retained_note_percent`,
- `candidate.arrangement_stats.human_potential_notes_moved`.

Es wird keine Schwelle, kein Gewicht und keine Policy dupliziert.

Der produktive `candidate.score` bleibt unveraendert die beobachtete Autoritaetsrealitaet.
Die Frontier-Evaluation ist nur Evidence. Gerade deshalb kann der Audit jetzt auch sichtbar
machen, ob ein Score ausserhalb von `candidate_score_breakdown` noch einen zusaetzlichen
Aufschlag traegt.

## Neue read-only Fragen

r697 beantwortet auf der echten SolverFrontier:

1. Wie viele live Solverzustaende tragen eine Retention-Strafe?
2. Wie gross sind Summe und Maximum dieser Strafe?
3. Wie lautet der finale Solver-Score?
4. Wie lautet dessen zentraler Breakdown?
5. Wie gross ist die Retention-Komponente des finalen Zustands?
6. Wie lautet derselbe beobachtete Solver-Score, wenn ausschliesslich diese bereits
   beobachtete Retention-Komponente read-only subtrahiert wird?
7. Gibt es zusaetzliche Score-Aufschlaege ausserhalb des zentralen Breakdowns?
8. Aendert sich der beste beobachtete SolverFrontier-Zustand, wenn nur die
   Retention-Komponente kontrafaktisch entfernt wird?

Die SourceCandidate-Gegenrechnung aus r694/r695 bleibt separat bestehen. Dadurch sind jetzt
drei Ebenen explizit getrennt:

```text
immutable SourceSelection origin
        !=
live solver frontier state
        !=
produktive Auswahl-/Mutationsautoritaet
```


## Statische Dependency-Inventur vor dem Lauf

Der r695-Source besitzt 15 produktive Aufrufstellen von `score_candidate` ausserhalb der
Funktionsdefinition:

```text
audio_salience_discrete_action_conflict_gate_tail.rs   1
audio_salience_discrete_translator_core.rs             2
corpus_fastjump_human_potential_core.rs                 1
corpus_fastjump_human_potential_repair_tail.rs          4
cross_staff_editorial_voice_math_gate_tail.rs           1
hidden_chord_reduction_texture_rebuild_tail.rs          5
sheaf_noether_projection_core.rs                        1
```

Zusaetzlich existieren mindestens zwei explizite Score-Aufschlaege ausserhalb des zentralen
`candidate_score_breakdown`:

```text
audio_salience_discrete_translator_core.rs:
    + variant.action_cost.max(0) / 20

corpus_fastjump_human_potential_repair_tail.rs:
    candidate.score += 10_000
```

Diese Aufschlaege werden in r697 **nicht** entfernt oder neu bewertet. Die neue Frontier-
Evidence macht lediglich sichtbar, ob und in welcher Hoehe ein live Solverzustand von seinem
zentralen Breakdown abweicht. Das ist fuer die spaetere Pensionierung des gesamten additiven
Score-Modells wichtiger als eine blosse Liste von Call-Sites.

## ChangeClass

`RefactorOnly`.

Begruendung: Die Instrumentierung erweitert eine bestehende musiknahe
SolverFrontier-Struktur und einen produktiv eingebundenen Orchestrator ausschliesslich um
bereits berechenbare read-only Evidence. `AuditOnly` ist fuer diese gemischten
Music-/Evidence-Pfade gemaess dem bestehenden Cross-Role-Vertrag zu eng. r697 veraendert
keine Musiksemantik.

## Harte Nicht-Ziele

Keine Aenderung an:

- `candidate_score_breakdown`,
- `score_candidate`,
- Retention-Schwellen 68/82,
- Retention-Gewichten,
- Kandidatenauswahl,
- Sortierung oder Tie-Break,
- Solver-Pass-Reihenfolge,
- Writern,
- MusicXML,
- Renderer/PDF,
- menschlicher Spielbarkeitsautoritaet,
- Source-/Attack-Identitaet.

Insbesondere ist der r697-Counterfactual weiterhin **keine Auswahlautoritaet**.

## Erwarteter Erkenntniswert fuer Etuede 5

Der r695-Lauf hat den finalen Frontier-Score `611` bereits autoritativ beobachtet.
r697 soll fuer genau diesen Frontier-Zustand erstmals den dazugehoerigen live
Breakdown sichtbar machen.

Wenn die bislang aus dem Lauf abgeleitete Arbeitshypothese stimmt, wird gelten:

```text
FinalSolverScoreTotal                  = 611
FinalSolverBreakdownScoreTotal         = 611
FinalSolverRetentionPenalty            = 611
FinalSolverScoreWithoutRetentionPenalty= 0
FinalSolverScoreAdjustmentOutsideBreakdown = 0
```

r697 behauptet diese Werte nicht vorab. Der Windows-Lauf entscheidet.

## Danach

Wenn r697 gruen ist und die Live-Frontier-Evidence konsistent ist, ist die
Vorbereitungsphase abgeschlossen.

Der naechste fachliche Take darf dann die Legacy-Retention-Policy produktiv entfernen:

- `ArrangementVisibleRetention` bleibt Mess- und Reportgroesse,
- `CanonicalSourceRetention` bleibt getrennte Provenienzmetrik,
- eine hohe sichtbare Retention wird nicht mehr bestraft,
- musikalischer Verlust muss durch einen anderen fachlichen Konflikt begruendet sein.

Erst danach wird das restliche additive Score-Modell als Entscheidungsautoritaet separat
pensioniert.
