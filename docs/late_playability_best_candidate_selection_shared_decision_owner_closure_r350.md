# r350 LatePlayabilityBestCandidateSelectionSharedDecisionOwnerClosure

Parent: r349 `LatePlayabilityBestCandidateSelectionOwnershipEvidenceTreeIntegrityRecovery`

ChangeClass: `RefactorOnly`

## Ausgangspunkt

Der reale r349-Lauf ist GREEN und released:

- `FinalExit 0`
- Build/Clippy/Launcher GREEN
- 1088/1088 integrierte Tests
- Etuede Nr. 6 bleibt 0/0/0

r349 bestaetigt fuer `LatePlayabilityBestCandidateSelection` dieselbe globale
Adoptionsfrage an mehreren seriellen Solver-Grenzen:

> Darf dieser gepruefte `SolverCandidate` den aktuellen globalen `best`-Zustand
> ersetzen?

Die Kandidatengeneratoren und Suchphasen sind verschieden und duerfen verschieden
bleiben. Offen war die Authority der finalen Adoption: vier lokale Comparatoren
implementierten diese Entscheidung separat; `GlobalHandTrajectory` delegierte bereits
an die Polish-Regel.

## Gemeinsamer DecisionOwner

r350 zieht die gemeinsame Adoptionsfrage in genau einen writer-freien Owner:

`decide_late_playability_best_candidate_adoption`

Der Owner arbeitet ausschliesslich auf typisierten
`LatePlayabilityBestCandidateDecisionFacts`. Er schreibt kein MusicXML, ersetzt keinen
`best`-Zustand und erzeugt keine Kandidaten.

Die bisherigen lokalen Comparator-Funktionen werden pensioniert:

- `warning_candidate_improves`
- `final_takt_candidate_improves`
- `final_playability_candidate_improves`
- `final_playability_polish_candidate_improves`
- der reine Delegationshelfer `global_hand_trajectory_candidate_improves`

## Explizite Policy-Projektionen

Der gemeinsame Owner behaelt die bisherigen stufenspezifischen Regeln als explizite
DecisionProjections:

- `StrictWarningCleaner`
- `FinalCriticalTaktRepair`
- `FinalPlayabilityCleaner`
- `FinalPlayabilityPolish`

`GlobalHandTrajectory` konsumiert wie bisher die `FinalPlayabilityPolish`-Projektion.

Damit wird in r350 bewusst **nicht** behauptet, dass die vier Policies musikalisch
identisch sein muessen. Geschlossen wird nur die mehrfach verteilte Authority fuer die
globale `best`-Adoption.

## Verhaltensinvarianz

Die vorhandenen Comparator-Regeln werden 1:1 uebernommen:

- `StrictWarningCleaner`: kein Warning-Slack; kritische Verschlechterung verboten;
  kritische Haerte bleibt letzter Tiebreak.
- `FinalCriticalTaktRepair`: bei sinkender Critical-Zahl bleiben bis zu
  `current warnings + 4` erlaubt.
- `FinalPlayabilityCleaner`: bei sinkender Critical-Zahl bleiben bis zu
  `current warnings + 6` erlaubt; bei Gleichstand bleiben kritische Haerte und Score
  die nachgeordneten Tiebreaks.
- `FinalPlayabilityPolish`: lexikographisch Critical, Warnings,
  PlayabilityWarningLoad, maximale Playability-Haerte und Score.
- Blocker-Semantik bleibt fuer alle Projektionen unveraendert.

Vier neue Regressionstests pinnen die entscheidenden Unterschiede:

- Warning-Slack 0 fuer `StrictWarningCleaner`.
- Warning-Slack 4 fuer `FinalCriticalTaktRepair`.
- Warning-Slack 6 fuer `FinalPlayabilityCleaner`.
- bestehende Polish-Reihenfolge Critical -> Warning -> Load -> Hardness -> Score.

Zusaetzlich wurde die neue Owner-Logik gegen die vorherigen Comparator-Formeln ueber
200000 deterministische Faktpaar-Proben gegengeprueft; es gab keine Abweichung.

## Materialisierungen bleiben erhalten

Unveraendert bleiben:

- Kandidatengeneratoren
- Kandidatenbudgets
- Rundenzahlen
- Pass-Reihenfolge
- `round_best`-Auswahl
- Austausch des globalen `best`
- Kandidatenhistorie / `candidates`
- Scoring
- Checks
- alle MusicXML-/Render-/PDF-Writer

Die Solver-Stufen entscheiden nicht mehr mit lokalem Comparator-Code, sondern konsumieren
den gemeinsamen Owner mit ihrer jeweiligen Projektion.

## Ownership-Evidence

Die r349-Evidence wird im selben Take auf den Zielzustand nachgezogen:

```text
LatePlayabilityBestCandidateSelection
    -> decide_late_playability_best_candidate_adoption
    -> StrictWarningCleaner
    -> FinalCriticalTaktRepair
    -> FinalPlayabilityCleaner
    -> FinalPlayabilityPolish
    -> GlobalHandTrajectory uses FinalPlayabilityPolish
```

OwnershipState:

`single_shared_decision_owner_multiple_policy_projections`

Damit ist kein separater AuditOnly-Nachtake vorgesehen, wenn r350 real GREEN und
verhaltensinvariant laeuft.

## Nicht geaendert

- keine neue Repair-/Recovery-Phase
- kein neuer Writer
- keine Candidate-Generation
- keine Schwelle oder Gewichtung
- keine Budget-/Runden-Aenderung
- keine Passorder-Aenderung
- keine Etueden-/Takt-Sonderlogik
- keine Security-/Gate-Lockerung
- Beam, RestSpanConsolidation, CrossStaff, HiddenChordToneRecovery und RawCursor bleiben
  geschlossen

Der beabsichtigte MusicXML-/Render-/PDF-Delta ist null. Jede reale musikalische
Abweichung gegen r349 waere deshalb ein Regressionsbefund und kein erwartetes Ergebnis
dieses Takes.
