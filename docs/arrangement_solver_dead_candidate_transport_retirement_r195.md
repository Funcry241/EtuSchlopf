# r195 – ArrangementSolverDeadCandidateTransportRetirement

## Befund

Nach r194 blieb `all_for_lines` als historischer Transportpfad bestehen. Eine repository-weite Verwendungsinventur ergab:

- Erzeugung ausschliesslich am Ende des Arrangement-Solvers.
- Transport in `MasterNotationAndSalienceStageInputs`.
- Dort keine einzige lesende Auswertung.
- Der lokale Materializing-Rewrite-Pfad fuegte lediglich weitere verworfene bzw. ersetzte `SolverCandidate`s hinzu.
- Nach Rueckkehr aus dieser Funktion wurde der gesamte Vektor fallengelassen.

Damit existierte kein lebender Konsument fuer Kandidatenprofil, Score, `CheckReport`, `layout_xml`, Arrangement-/Polish-/Layout-Statistiken oder Layoutplaene der verworfenen Kandidaten.

## Aenderung

- `ArrangementSolverStageOutcome::all_for_lines` entfernt.
- Aufbau, Kapazitaetsreservierung, Best-Clone und Sortierung des toten Vektors entfernt.
- `MasterNotationAndSalienceStageInputs::all_for_lines` entfernt.
- `run_certified_materializing_local_rewrite_portfolio` erhaelt keinen Kandidaten-Sink mehr.
- Verworfene Mikrokandidaten werden nach vollstaendiger Bewertung direkt freigegeben.
- Ein akzeptierter Mikrokandidat ersetzt `best` direkt; der vorherige Best wurde zuvor nur in den ungelesenen Sink verschoben.

## Semantiknachweis

Die lebenden Entscheidungswerte bleiben unveraendert:

- `best`
- `next_iteration`
- Kandidatenvalidierung
- Micro-Delta-Budget
- `final_clean`
- Pareto-Rang
- Materialized-Rewrite-Score
- Proof-Cost
- Winner-Sortierung

Es gibt keinen `Drop`-Implementierer fuer `SolverCandidate`; frueheres Freigeben besitzt keinen fachlichen Seiteneffekt.

## Erwarteter Nutzen

r194 beseitigte die zweite Vollkopie der verworfenen Solverpopulation. r195 beendet nun auch deren nutzlose Lebenszeit ueber die komplette Master-Notation-/Salience-Stufe und vermeidet dort zusaetzlichen Working-Set-, Cache- und Speicherallocator-Druck. Zusaetzlich entfallen ungenutzte Best-/Mikrokandidaten-Kopien in diesem historischen Sink.

Die Verbesserung ist absichtlich als Performance-/Ownership- und Legacy-Retirement eingeordnet, nicht als musikalische Optimierung.
