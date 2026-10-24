# r712 – FinalChordTieStopSameOnsetTemporalClosureRegressionExpectationRecovery

## Zweck

r711 wurde vom PRE-TAKE-DIFF-GATE akzeptiert, Strict-Clippy und Build waren grün. Der Reexec endete jedoch bereits in den integrierten Rust-Tests, bevor Etüde Nr. 6 ausgeführt wurde. Ursache war ausschließlich eine falsche Zählerwartung im neu hinzugefügten Regressionstest.

## Produktive Änderung

Unverändert zu r711: Der Same-Onset-Ownership-Guard von FinalChordEncoding ignoriert `tie_stop`-Fortsetzungen bei der Suche nach einem konkurrierenden neuen Attack-Owner. Ein Tie-Stop ist eine Fortsetzung und keine neue Attacke. Dadurch kann der bestehende Auxiliary-/Retiming-Pfad unabhängigen temporalen Umfang kanonisch materialisieren.

Keine Sonderregel für Projekt, Takt, Pitch oder Position. Keine neue Writer- oder Repair-Schicht. Keine Änderung an CandidateScore, Retention, HumanPlayability oder Auswahlheuristiken.

## Regressionstest-Korrektur

Die erzeugte Auxiliary-Lane enthält die technische Voice-Markierung dreimal: einmal im Gap-`forward`, einmal in der rematerialisierten Note und einmal im Tail-`forward`. Der r711-Test erwartete fälschlich zwei Vorkommen. r712 korrigiert ausschließlich diese Erwartung von 2 auf 3; der produktive Funktionskörper bleibt gegenüber r711 byteidentisch.
