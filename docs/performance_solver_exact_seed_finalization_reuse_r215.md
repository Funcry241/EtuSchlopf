# PerformanceSolverExactSeedFinalizationReuse r215

## Ausgangsevidenz r214

Der autoritativ gruene r214-Vollkorpus hat den Solver erstmals pro Pass und innerhalb des Kandidatenbaus vermessen.

Korpusweit:

- ArrangementSolver: 253224 ms
- LocalBestRefinement: 165364 ms = rund 65.3 % des gesamten Solvers
- LocalBestRefinement: 120 Kandidaten pro Etuede, 1200 Kandidaten ueber den Korpus
- SeedPrepareWorkerAccumulated: 5572317 us
- CandidateFinishWorkerAccumulated: 1714192422 us
- MusicXmlParseWorkerAccumulated: 215973593 us
- RunChecksWorkerAccumulated: 42125632 us

Damit ist nicht die Seed-Erzeugung der dominante Kostenblock, sondern die vollstaendige Finalisierung eines bereits erzeugten Seeds.

## Ziel

Byteidentische Solver-Seeds duerfen innerhalb desselben Kandidaten-Batches nicht mehrfach durch dieselbe teure Finalisierung laufen.

Nicht Ziel sind:

- weniger Profile,
- weniger Kandidaten,
- frueheres Abbrechen eines Passes,
- Veraendern der Pass-Reihenfolge,
- Entfernen aktuell erfolgloser Repair-Passes,
- neue Heuristiken,
- neue Scores oder Schwellenwerte.

## Exakter Wiederverwendungsvertrag

1. Jeder angeforderte Profilkandidat erzeugt weiterhin seinen eigenen `PreparedSolverSeed` mit urspruenglicher Iteration, Profil und `FlpSeedStats`.
2. Die komplette Seed-MusicXML wird mit FNV64 nur in einen Such-Bucket eingeordnet.
3. Innerhalb eines Buckets muss die komplette Seed-MusicXML byteidentisch sein. Nur diese exakte Gleichheit erlaubt Wiederverwendung.
4. Pro byteeindeutigem Seed wird `finish_solver_candidate_from_seed` genau einmal ausgefuehrt.
5. Fuer jedes urspruengliche Profil wird danach wieder ein eigener `SolverCandidate` erzeugt. Zurueckgebunden werden exakt dessen:
   - `iteration`,
   - `profile_name`,
   - `profile_contract`,
   - `profile_lineage`,
   - `flp_seed_stats`.
6. Score, CheckReport, Layout-MusicXML, Fidelity, Arrangement-/Polish-/Layout-Statistik und HorizontalLayoutSeed stammen von der einmal ausgefuehrten Finalisierung derselben byteidentischen Seed-MusicXML.
7. Kandidatenreihenfolge und Kandidatenanzahl bleiben identisch zum bisherigen Vertrag.

## Warum die Finalisierung bei byteidentischem Seed wiederverwendbar ist

Nach `prepare_solver_seed` konsumiert die Finalisierung die Seed-MusicXML zusammen mit den projektweiten unveraenderten Eingaben `ParallelPlan`, `ReferenceReport`, `CheckOptions`, `layout_pages` und `no_polish`.

Das Profil beeinflusst in dieser Phase keine weitere musikalische Transformation mehr. Sein Anzeigename wird fuer Fehlermeldungskontext verwendet; Profilvertrag und Lineage werden als Kandidatenmetadaten getragen. Diese Metadaten werden bei wiederverwendeten Kandidaten wieder auf das urspruengliche Profil zurueckgebunden.

Damit wird keine Ergebnisberechnung angenaehert oder ausgelassen: nur eine nachweislich identische Berechnung fuer identische Musikbytes innerhalb desselben Batches wird nicht wiederholt.

## Messhistorie und heutige Beobachtung

Die temporaeren r213/r214-Zaehler (`SeedPrepareCalls`, `CandidateFinishCalls`,
`CandidateDelta`, `BestChanged` und Passzeiten) wurden mit r219 entfernt.
Die oben genannten Messwerte gehoeren zur historischen r214/r215-Abnahme,
nicht zum aktuellen Lauf. Die Ergebnisse und Rueckbauentscheidung stehen im
[Abschlussbericht](performance_closure_instrumentation_retirement_r219.md).

Der erhaltene Mechanismus liegt in
`src/pipeline_shards/corpus_fastjump_human_potential_core.rs::build_candidate_batch`.
Der aktuelle Lauf meldet `ExactSeedReuse` bei Treffern; Phasenzeiten beschreibt
[die aktuelle Messdokumentation](phase_timing_console_audit_r1291.md).

## Sicherheitsgrenzen

- ChangeClass `RefactorOnly`
- keine neue Writer-Funktion
- keine neue Repair-Funktion
- keine neue Selection-Autoritaet
- keine neue Cache-Persistenz
- keine Wiederverwendung ueber Projekte oder Runs hinweg
- kein Dateisystemcache
- FNV ist nur Bucket, niemals Gleichheitsbeweis
- exakte Seed-Bytes sind der Gleichheitsbeweis
