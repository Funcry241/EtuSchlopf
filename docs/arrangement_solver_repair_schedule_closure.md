# ArrangementSolverRepairScheduleClosure

## Ziel

Der dritte und letzte Take des vereinbarten Architektur-/Schönheitssuperwurfs
schließt ausschließlich die zweite Hälfte der musikalischen Arrangement-Solver-
Steuerung. Nach Initial Selection und Guided Search werden die bereits
vorhandenen Atomic-/Playability-Pässe sowie die Substance-/Gesture-Pässe in
fachlich benannte private Stufen gefasst. Verhalten, Reihenfolge und
Kandidatenzustand bleiben unverändert.

## Atomic / Playability Repair

`run_arrangement_solver_atomic_playability_repair_stage` übernimmt exakt die
bisherige Reihenfolge:

1. `run_atomic_critical_repair`
2. `run_warning_elimination_repair`
3. `run_final_critical_takt_repair`
4. `run_final_playability_cleaner`
5. `run_final_playability_polish`
6. `run_global_hand_trajectory_solver`

Alle vorhandenen `no_polish`-, Blocker-, Warning- und Critical-Bedingungen
bleiben an derselben Stelle und mit derselben Bedeutung erhalten.

## Substance / Gesture

`run_arrangement_solver_substance_gesture_stage` übernimmt exakt die bisherige
anschließende Reihenfolge:

1. `run_substance_guard`
2. `run_composer_substance_pass`
3. `run_near_miss_repair_pass`
4. `run_hand_trajectory_v2_substance_bridge`
5. `run_consensus_gesture_probe`

Es entsteht keine neue Auswahlregel, kein generisches Pass-Framework und keine
neue Writer- oder Mutationsautorität.

## Finalisierung

`finish_arrangement_solver_stage` besitzt ausschließlich die bisherige
Abschlussprojektion des Solver-Zustands: `best` wird zusammen mit den übrigen
Kandidaten nach Iteration für `all_for_lines` sortiert und anschließend als
`ArrangementSolverStageOutcome` zurückgegeben.

`run_arrangement_solver_stage` ist danach nur noch der lesbare Ablaufbesitzer:

1. Initial Selection
2. Guided Search
3. Atomic / Playability Repair
4. Substance / Gesture
5. Finalisierung

## Verhaltenstreue

Unverändert bleiben insbesondere:

- Musikregeln und Schwellenwerte
- Profile und Hotspot-Budgets
- Kandidatenbewertung und Guard-Semantik
- Pass- und Writerreihenfolge
- `best`, `candidates`, `next_iteration`
- Reports und Releaseentscheidungen
- Master-/Render-MusicXML- und PDF-Erzeugung

## Erfolgskriterium

- `run_arrangement_solver_stage` liegt klar unter 100 physischen Zeilen.
- Die zweite Solverhälfte ist über zwei fachlich benannte private Stufen sichtbar.
- Atomic-/Playability- und Substance-/Gesture-Aufrufreihenfolge ist gegenüber
  v0.10.1678 identisch.
- Keine neue öffentliche API und kein generisches Pass-Framework entsteht.
- Strict-Take für Etüde Nr. 4 baut und läuft grün.
- Normalisierte Master-/Render-MusicXML und das PDF bleiben artefaktidentisch.

## Stopkriterium

Mit grünem Artefaktbeweis endet der vereinbarte letzte Schönheitssuperwurf.
Danach keine weitere Architektur- oder Clippy-Arbeit aus Selbstzweck; der
nächste Arbeitsblock darf wieder musikalisch priorisieren.
