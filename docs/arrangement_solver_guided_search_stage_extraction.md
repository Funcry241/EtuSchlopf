# ArrangementSolverGuidedSearchStageExtraction

## Ziel

Der zweite Take des letzten Architektur-/Schönheitssuperwurfs trennt ausschließlich die erste Hälfte der musikalischen Arrangement-Solver-Steuerung in zwei benannte private Grenzen: Initialauswahl und Guided Search. Die Passreihenfolge, Kandidatenbewertung, Profile, Hotspot-Budgets und Writer bleiben unverändert.

## Neuer privater Arbeitsvertrag

`ArrangementSolverSearchState` besitzt exakt drei Felder:

- `best`
- `candidates`
- `next_iteration`

Damit wird der bisher lose über die lange Stage-Funktion geführte Solver-Zustand lokal und typisiert. Es entsteht kein öffentliches API und kein generisches Pass-Framework.

## Initialauswahl

`select_initial_arrangement_solver_search_state` übernimmt ausschließlich den bisherigen Block von Basiskandidatenbildung bis Best/Guard-Auswahl und initialer Iterationsnummer. Die bestehende Trajectory-Guard-Regel, `candidate_selection_key` und `candidate_strictly_beats_guard` bleiben in identischer Reihenfolge und mit identischen Argumenten erhalten.

## Guided Search

`run_arrangement_solver_guided_search_stage` übernimmt ausschließlich die bisher unmittelbar anschließende Reihenfolge:

1. `run_corpus_driven_hand_rhythm_primary_repair`
2. `LocalBestRefinement`
3. `ResidualCriticalPass`
4. `TargetedCriticalPass`
5. `CriticalMicroRepair`

Alle vorhandenen Bedingungen, Profilquellen, Hotspot-Kappen, Kandidatenbatches und Guard-Vergleiche bleiben unverändert. Die lokalen `next_iteration`-Berechnungen der vier Batch-Pässe bleiben weiterhin lokal und verändern den äußeren Iterationsvertrag nicht.

## Harte Beweiskante

Der Code ab dem Kommentar `v0.1.53: SafeAtomicRepair` bis zum Ende von `run_arrangement_solver_stage` wird gegenüber v0.10.1677 byteidentisch übernommen. AtomicCriticalRepair, WarningElimination, FinalCriticalTaktRepair, PlayabilityCleaner, PlayabilityPolish, GlobalHandTrajectory, SubstanceGuard, ComposerSubstance, NearMiss, HandTrajectoryV2 und ConsensusGesture werden in diesem Take nicht zerlegt oder verändert.

## Verhaltenstreue

Keine Musikregel, kein Schwellenwert, keine Profilgewichtung, keine Kandidatenreihenfolge, keine Writer-/Passreihenfolge, kein Report, keine Releaseentscheidung und kein Artefaktpfad wird geändert. Die Änderung ist ausschließlich eine private Kontrollfluss- und Zustandsgrenze.

## Erfolgskriterium

- Initial Selection und Guided Search sind im Quelltext als getrennte private Stufen sichtbar.
- Der Arbeitsvertrag enthält exakt `best`, `candidates`, `next_iteration`.
- Die Reihenfolge Hand/Rhythmus -> LocalBest -> Residual -> Targeted -> CriticalMicro bleibt unverändert.
- Der gesamte Solver-Tail ab SafeAtomicRepair bleibt byteidentisch zu v0.10.1677.
- Der Strict-Take für Etüde Nr. 4 baut und läuft grün.
- Normalisierte Master-/Render-MusicXML und das PDF bleiben gegenüber dem v0.10.1676-Beweisstand identisch.

## Stopkriterium

Nach grünem Beweis wird in diesem Take keine Atomic-/Playability-/Substance-Grenze angefasst. Die zweite Hälfte der Solver-Steuerung ist ausschließlich Scope von Take 3.
