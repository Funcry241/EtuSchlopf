# LegacyCandidateScoreDecisionAuthorityPreparationAudit r694

## Zweck

Dieser Take bereitet die geplante Ablösung der alten additiven Kandidatengewichtung vor, ohne die produktive Auswahl zu verändern.

Parent ist der angenommene Stand r693 `MusicalConservationMetricSemanticsEssenceClosure`.

Die neue Essenz verlangt insbesondere:

- musikalische Veränderung braucht einen Beweis, musikalischer Erhalt nicht;
- `CanonicalSourceRetention`, `ArrangementVisibleRetention` und menschliche Spielbarkeit sind getrennte Aussagen;
- Unsicherheit ist kein Löschgrund;
- Kennzahlen erhalten erst mit eindeutiger Bedeutung, Bezugsgröße und Berechnung Beweischarakter.

r694 ist ausschließlich Beobachtung und Charakterisierung. Es ändert keine MusicXML-, Renderer-, Solver-, Candidate-, Ranking-, Weighting-, Threshold-, Retention- oder Spielbarkeitsentscheidung.

## Produktiver Ist-Zustand

Der gemeinsame produktive Score wird in

`src/pipeline_shards/staff_voice_move_restify_atomic_patch_tail.rs`

über `candidate_score_breakdown()` gebildet. Die Funktion addiert aktuell voneinander verschiedene Größen:

- Blocker
- Critical
- Critical-Hardness
- Fast-Jump
- Hand-Span
- Simultaneous-Notes
- Warnungen
- Hinweise
- RuleMath
- `ArrangementVisibleRetention` als `retention_penalty`

Die Retention-Komponente ist im produktiven Score derzeit nicht monoton. Der aktuelle Source-Stand enthält eine strafpunktfreie Zone von 68 bis 82 Prozent; unterhalb und oberhalb dieser Zone entstehen Strafpunkte. r694 dupliziert diese Schwellen bewusst nicht in Audit-Code oder Tests. Die produktive Funktion bleibt einzige Quelle dieser Altmechanik.

## Inventur der produktiven Score-Erzeugung

Neben der Definition existieren im r693-Stand 15 produktive Aufrufe von `score_candidate(...)` in diesen Bereichen:

- `hidden_chord_reduction_texture_rebuild_tail.rs` – 5
- `audio_salience_discrete_translator_core.rs` – 2
- `audio_salience_discrete_action_conflict_gate_tail.rs` – 1
- `corpus_fastjump_human_potential_repair_tail.rs` – 4
- `cross_staff_editorial_voice_math_gate_tail.rs` – 1
- `corpus_fastjump_human_potential_core.rs` – 1
- `sheaf_noether_projection_core.rs` – 1

Zusätzlich erzeugt `source_selection_candidate_evaluation()` aus demselben `candidate_score_breakdown()` den bereits vorhandenen Audit-Breakdown je SourceSelection-Kandidat.

Der Score wird danach an mehreren Stellen direkt oder als Tie-Breaker verwendet. Dazu gehören unter anderem Candidate-Key/Guard, SubstanceGuard, Human-Potential-Repair, Hidden-Chord-Recovery und Audio-Salience-Varianten. Diese Verwendung wird in r694 nicht verändert.

## Neue read-only Beobachtung

Der vorhandene `SOURCE-SELECTION-FULL-CAUSAL-AUDIT-SHADOW-PLAN` erhält eine zusätzliche Sektion `LegacyCandidateScorePreparationAudit`.

Sie verwendet ausschließlich bereits vorhandene `SourceSelectionCandidateEvaluation`-Werte und berechnet für den SourceCandidate-Satz zusätzlich:

- Zahl der Kandidaten mit Retention-Strafanteil;
- Summe und Maximum des vorhandenen Retention-Strafanteils;
- bisherigen besten SourceCandidate nach Legacy-Gesamtscore;
- rein kontrafaktischen Score `score_total - retention_penalty`;
- rein kontrafaktisch besten SourceCandidate ohne Retention-Strafanteil;
- ob sich innerhalb dieses SourceCandidate-Satzes dadurch der Gewinner ändern würde;
- Retention-Strafanteil und kontrafaktischen Score des finalen SourceCandidate.

Diese Gegenrechnung besitzt ausdrücklich keine Selection-, Candidate-, Solver- oder Writer-Authority. Sie verändert keinen Kandidaten und wird nicht zurück in den Solver gespeist.

Für jede vorhandene `SourceCandidate`-Zeile wird außerdem eine separate `LegacyCandidateScoreCounterfactual`-Zeile ausgegeben. Dadurch bleiben produktiver Ist-Score und read-only Gegenrechnung gleichzeitig sichtbar.

## Tests

Die neuen Tests prüfen nur die Beobachtungslogik:

1. Die Gegenrechnung entfernt ausschließlich den bereits beobachteten `retention_penalty` aus `score_total`.
2. Die Audit-Zusammenfassung kann erkennen, wenn sich der beste SourceCandidate in einer synthetischen Gegenrechnung ändern würde.
3. Ein leerer Kandidatensatz erzeugt keinen erfundenen Gewinner.

Die Tests kodieren bewusst weder 68 noch 82 Prozent als zukünftigen Vertrag und behaupten keine gewünschte Retentionquote.

## Abschlussbeweis für r694

Der reale Windows-Take muss zeigen:

- Admission GREEN;
- Strict Clippy GREEN / 0 Warnungen;
- Build GREEN;
- alle Tests GREEN;
- ProgramExit 0;
- Renderer GREEN;
- MusicXML und Render-MusicXML gegenüber r693 musikalisch unverändert;
- `LegacyCandidateScoreDecisionAuthorityChanged: nein`;
- `LegacyCandidateScoreWeightingChanged: nein`;
- `LegacyCandidateScoreRetentionPolicyChanged: nein`;
- `LegacyCandidateScoreMusicMutation: nein`.

Danach liefert der Audit die notwendige Datenbasis für den nächsten Take, in dem ausschließlich die alte Retention-Policy aus der produktiven Bewertung entfernt werden soll.
