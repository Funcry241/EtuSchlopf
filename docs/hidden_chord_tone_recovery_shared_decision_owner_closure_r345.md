# r345 HiddenChordToneRecoverySharedDecisionOwnerClosure

Parent: r344 `HiddenChordToneRecoveryDecisionOwnershipEvidence`

ChangeClass: `RefactorOnly`

## Ausgangspunkt

Der reale r344-Lauf ist GREEN und released: `FinalExit 0`, Build 0, Clippy 0,
Launcher 3/3 und 1084/1084 integrierte Tests. r344 bestaetigt fuer
`HiddenChordToneRecovery` denselben produktiven Kandidatenraum an zwei getrennten
Akzeptanzgrenzen: `ComposerFaithfulRecovery` und `ConsensusGestureProbe`.

Die Materialisierungsstrategien sind verschieden, die fachliche Commit-Frage ist aber
gleich: Darf ein aus bereits vorhandenem verborgenem Akkordtonmaterial erzeugter
Recovery-Vorschlag den aktuellen sichtbaren HiddenMaterial-Zustand uebernehmen?

## Gemeinsamer DecisionOwner

r345 zieht diese Commit-Frage in genau einen writer-freien Owner:

`decide_hidden_chord_tone_recovery_adoption`

Der Owner arbeitet ausschliesslich auf typisierten Entscheidungsfakten. Er schreibt weder
MusicXML noch `best.layout_xml` und baut keine Kandidaten. Die bisherigen Proposal-Builder
bleiben erhalten.

Explizite DecisionProjections:

- `HarmonicPitchSwap`
- `RhythmicTextureRebuild`
- `PackageUnhide`
- `SingleUnhide`
- `ConsensusGesture`

## Materialisierungsprojektionen bleiben erhalten

ComposerFaithfulRecovery behaelt:

- HarmonicPitchSwap
- RhythmicTextureRebuild
- PackageUnhide
- SingleUnhide

ConsensusGestureProbe behaelt:

- DirectUnhide
- GaussCandidate
- GaussChain
- BrokenGesture
- ConstrainedTail
- die vorhandene HAN02-Staff-Verteilungsprobe

Diese Pfade duerfen weiterhin Kandidaten bauen, ordnen und lokal pruefen. Sie duerfen aber
die gemeinsame finale HiddenMaterial-Adoption nicht mehr mit einem eigenen musikalischen
Praedikat autorisieren.

## Verhaltensinvarianz

Die vorhandenen Akzeptanzbedingungen werden 1:1 in den gemeinsamen Owner ueberfuehrt.

- HarmonicPitchSwap, PackageUnhide und SingleUnhide: Blocker, Warnungen, kritische Befunde
  und Hinweise duerfen sich gegenueber der aktuellen Baseline nicht verschlechtern.
- RhythmicTextureRebuild: die bereits vorhandenen `delta_scope_clean`- und `delta_clean`-
  Beweise bleiben erforderlich; Warnungen, kritische Befunde und Hinweise duerfen nicht
  schlechter werden. Baseline-alte/out-of-scope Blocker behalten wie bisher den alten
  `final_result`-Readback statt den Kandidatenreport zu uebernehmen.
- ConsensusGesture: Kandidat muss weiterhin 0 Blocker, 0 Warnungen, 0 kritische Befunde,
  keine Hint-Verschlechterung und positiven Sichtgewinn besitzen.

Gauss-, Chain-, Broken-Gesture-, Direct- und HAN02-Proposal-Auswahl konsumieren dieselbe
Consensus-Projektion. Der finale `best`-Commit wird zusaetzlich nochmals ueber denselben
Owner gebunden.

Vier neue allgemeine Regressionstests pinnen die unterschiedlichen Projektionen:

- PitchSwap darf ohne Sichtzahlgewinn adoptieren.
- DirectUnhide verwirft eine Diagnoseverschlechterung.
- TextureRebuild kann bei delta-cleanen baseline-alten Blockern den Baseline-Report behalten.
- ConsensusGesture verlangt positiven Sichtgewinn.

## Ownership-Evidence

Da r345 ein verhaltensinvarianter RefactorOnly-Cut ist, wird die r344-Evidence im selben
Take auf den Zielzustand nachgezogen:

`HiddenChordToneRecovery -> decide_hidden_chord_tone_recovery_adoption -> mehrere Proposal-/Materialisierungsprojektionen`

OwnershipState:

`single_shared_decision_owner_multiple_materialization_projections`

Damit ist kein separater r346-AuditOnly-Nachtake vorgesehen.

## Nicht geaendert

- keine neue Recovery-/Repair-Phase
- kein neuer Writer
- keine Pass-Reihenfolge
- keine Schwelle oder Gewichtung
- keine Pitch-/Onset-/Duration-/Staff-/Voice-Regel
- keine Etueden-/Takt-Sonderlogik
- keine Security-/Gate-Lockerung
- Beam, RestSpanConsolidation, CrossStaff und RestExistence bleiben unberuehrt

Der beabsichtigte MusicXML-/Render-/PDF-Delta ist null. Jede reale musikalische Abweichung
gegen r344 waere deshalb ein Regressionsbefund und kein erwartetes Ergebnis dieses Takes.
