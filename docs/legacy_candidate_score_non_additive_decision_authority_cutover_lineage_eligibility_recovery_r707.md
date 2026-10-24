# r707 – LegacyCandidateScoreNonAdditiveDecisionAuthorityCutoverLineageEligibilityRecovery

Parent: r705 `LegacyCandidateScoreGuardTransitionContractAudit`  
ChangeClass: `MusicMutation`

## Laufbefund r706

r706 passierte PRE-TAKE-DIFF-GATE, Strict-Clippy und Build. Alle integrierten Rust-Targets waren
gruen. Im produktiven Etuede-5-Reexec wurde die nicht-additive Authority erstmals live wirksam.
Dadurch entstand eine andere Suchbahn als unter dem Legacy-Score.

Der Lauf stoppte im `StrictWarningCleaner` an genau der bestehenden Material-Lineage-Invariante:

`AMBIGUOUS_FAIL_CLOSED representation transfer target already existed before owner SafeAtomicTransferCandidateProducer`

Der betroffene Vorschlag war damit kein zulaessiger RepresentationTransfer. Der Fehler ist
candidate-lokal: Die bestehende Lineage-Authority verbietet die Adoption dieses Vorschlags.

## Recovery

r707 behaelt den nicht-additiven CandidateDecision-Cutover aus r706 unveraendert bei.

Die bestehende RepresentationTransfer-Pruefung wird so faktorisert, dass dieselbe autoritative
Preflight-Logik bereits vor der parallelen Candidate-Auswertung verwendet werden kann:

- Source-Span muss eindeutig sein.
- Source muss an bestehende Attack-/Tie-Lineage gebunden sein.
- Target-Mass muss numerisch und eindeutig bestimmbar sein.
- Existiert das exakte Target bereits im gebundenen Base-Material vor dem Owner, ist der Vorschlag
  **nicht zulaessig** und wird nicht materialisiert.
- Alle anderen Preflight-/Lineage-Fehler bleiben harte `Err`-Zustaende.
- Die eigentliche Reconciliation wiederholt dieselbe Pruefung und bleibt fail-closed.

Das ist kein Error-Swallowing und kein Score-Fallback. Nur der bereits explizit definierte
`PreexistingTarget`-Fall wird von einem globalen Solver-Abbruch zu einer Candidate-Eligibility-
Entscheidung verschoben: der Kandidat nimmt nicht an der Auswahl teil.

Die urspruengliche Evaluationsposition bleibt als Solver-Iteration erhalten, auch wenn unzulaessige
Kandidaten herausgefiltert werden. Damit bleibt der deterministische Identitaets-Tie-Break stabil.

Audit- und Testpfade aus r705 bleiben unveraendert.
