# LatePlayabilityBestCandidateSelectionOwnershipEvidence

## Revision

`EtuSchlopf v0.10.1839 r349`

Parent: real GREEN r347 `FinalRenderRawCursorFallbackAuthorityRetirement`.

Recovery: r348 wurde nie installiert. Take-Tree-Integrity blockierte den Take fail-closed wegen des unbeabsichtigt enthaltenen Top-Level-Pfads `Cargo.lock`. r349 traegt dieselbe AuditOnly-Evidence ohne diesen unzulaessigen Paketpfad weiter.

## Ausgangspunkt

r347 ist im realen Etuede-6-Kontrolllauf GREEN und released:

- `FinalExit 0`
- Build/Clippy/Launcher GREEN
- 1088/1088 integrierte Tests
- Master-MusicXML gegen r346 byteidentisch
- Render-MusicXML gegen r346 byteidentisch
- alle sechs PDF-Seiten pixelidentisch

Damit ist `FinalRenderRawCursorClosure` geschlossen und wird nicht erneut geoeffnet.

## Verworfenes Tonal-Signal

Der r347-UnifiedDomainOwnership-Report zeigt fuer `Tonal` mehrere serielle
String-Reconstruction-Flaechen. Der konkrete Quellbefund reicht jedoch nicht
fuer einen Mehrfach-Authority-Beweis:

`KeySignatureEnharmonicSpanRenderWrite` besitzt aktuell keinen offenen
produktiven Tonal-Entscheidungsweg. `behind_bars_gate_open` bleibt `false`;
die Span-Evidence ist damit candidate-only/fail-closed und kein lebender
zweiter Tonal-Owner.

Tonal wird deshalb in r349 nicht geschnitten.

## Naechste belastbare Insel

Im spaeten Arrangement-Solver schreiben mehrere serielle Stufen in denselben
globalen Zustand:

```text
SolverCandidate best
```

Jede Stufe erzeugt eigene Kandidaten. Das ist fuer sich genommen legitim.
Die fachlich relevante Ueberlagerung liegt in der Adoption: mehrere Stufen
entscheiden erneut, ob ein Kandidat `best` ersetzen darf.

Die produktiven Grenzen sind:

```text
StrictWarningCleaner
  -> warning_candidate_improves

FinalCriticalTaktRepair
  -> final_takt_candidate_improves

FinalPlayabilityCleaner
  -> final_playability_candidate_improves

FinalPlayabilityPolish
  -> final_playability_polish_candidate_improves

GlobalHandTrajectory
  -> delegiert bereits an final_playability_polish_candidate_improves
```

Alle ersetzen bei positivem Urteil denselben `best`-Zustand.

## Warum dies ein echtes Ownership-Signal ist

Die Kandidatengeneratoren duerfen verschieden bleiben. Die Adoption beantwortet
jedoch stets dieselbe fachliche Frage:

> Darf dieser gepruefte SolverCandidate den aktuellen globalen Bestzustand
> ersetzen?

Diese Entscheidung besitzt heute mehrere lokale Policy-Implementierungen.
Die Regeln sind nicht identisch:

- StrictWarningCleaner: keine Warnungsverschlechterung; kritische Haerte als
  spaeter Tiebreak.
- FinalCriticalTaktRepair: bei weniger kritischen Befunden sind bis zu
  `current warnings + 4` erlaubt.
- FinalPlayabilityCleaner: bei weniger kritischen Befunden sind bis zu
  `current warnings + 6` erlaubt; Score wird als weiterer Tiebreak benutzt.
- FinalPlayabilityPolish: strikt lexikographisch ueber Critical, Warnings,
  PlayabilityWarningLoad, maximale Playability-Haerte und Score.
- GlobalHandTrajectory besitzt bereits keine eigene Policy, sondern delegiert
  an die Polish-Policy.

Damit ist die Ueberlagerung nicht nur ein Scanner-Treffer. Mehrere serielle
produktive Grenzen treffen dieselbe globale Adoption mit unterschiedlichen
lokalen Wahrheiten.

## r349

r349 ist ausschliesslich `AuditOnly`.

Die bestehende Mutation-Ownership-Evidence erhaelt das Muster:

```text
LatePlayabilityBestCandidateSelection
```

mit:

```text
SharedState         SolverCandidate_best
SameDecision        candidate_replaces_global_best
SharedDecisionOwner none
OwnershipState      multiple_late_solver_candidate_adoption_policies_open
```

Die unterschiedlichen Kandidatengeneratoren, Suchbudgets, Rundenzahlen und
Passgrenzen werden nicht umgedeutet und nicht veraendert.

## Nicht Bestandteil von r349

- keine Aenderung an `best`
- kein neuer DecisionOwner
- keine Comparator-Aenderung
- keine Candidate-Aenderung
- keine Budget-/Runden-Aenderung
- keine Passorder-Aenderung
- keine Musikmutation
- keine Security-/Gate-Lockerung
- keine neue Repair-/Recovery-Authority

## Ziel fuer den Folgetake

Nur wenn r349 real GREEN ist, darf der Folgetake die Adoption konvergieren.

Das Ziel ist nicht, alle Generatoren zu vereinheitlichen. Ziel ist ein
gemeinsamer writer-freier DecisionOwner fuer die globale Candidate-Adoption,
der die heutigen Stufen zunaechst verhaltensinvariant als explizite
DecisionProjections konsumiert.

Erst ein spaeterer, separat bewiesener Schritt duerfte die unterschiedlichen
Policy-Projektionen selbst fachlich vereinheitlichen.

## ChangeClass

`AuditOnly`.
