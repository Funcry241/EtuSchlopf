# r344 HiddenChordToneRecoveryDecisionOwnershipEvidence

Parent: r343 `CrossStaffHandProjectionReadabilityEvidenceProjectionClosure`

ChangeClass: `AuditOnly`

## Anlass

Der r343-UnifiedDomainOwnership-Bestand zeigt fuer `HiddenMaterial` 17 vereinheitlichte
Oberflaechen, 15 erreichbare Oberflaechen, drei serielle Same-Domain-Signale und bislang
kein explizites OwnerPattern. Fuenf starke erreichbare Oberflaechen werden als
`StateAdoption` klassifiziert.

Die Quellpfadpruefung grenzt daraus genau eine belastbare musikalische Insel ab:
`HiddenChordToneRecovery`.

Mehrere produktive Pfade lesen denselben Kandidatenraum aus
`collect_hidden_chord_tone_recovery_candidates` und duerfen danach einen neuen
`best.layout_xml`-Zustand uebernehmen.

Die zwei produktiven Akzeptanzgrenzen sind:

- `ComposerFaithfulRecovery`
- `ConsensusGestureProbe`

Sie verwenden unterschiedliche Materialisierungsstrategien, entscheiden aber beide
erneut, welches bereits vorhandene verborgene Akkordtonmaterial wieder Teil der
sichtbaren musikalischen Gestalt werden darf.

## Abgrenzung

Dieser Take behauptet ausdruecklich nicht, dass jede `HiddenMaterial`-Oberflaeche eine
zweite musikalische Authority ist. Insbesondere reine String-Helfer, Render-Pruning und
Timeline-Forward-Projektionen werden nicht als Beweis fuer Mehrfach-Ownership gewertet.

Ebenso wird `StaffVoice` bewusst nicht als naechster Schnitt gewaehlt: Die dortigen
StateAdoptions betreffen unterschiedliche Entscheidungen (Voice-Rollenbereinigung versus
konfliktbedingte Staff-Verteilung) und sind deshalb kein hinreichender Same-Decision-Beweis.

## Neue Evidence

Der bestehende `MUTATION-OWNERSHIP-EVIDENCE-BOUNDARY`-Unterblock projiziert zusaetzlich:

- Pattern: `HiddenChordToneRecovery`
- CandidateEvidence: gemeinsamer Hidden-Chord-Tone-Kandidatenraum
- ProductiveAcceptanceBoundaries: `ComposerFaithfulRecovery`, `ConsensusGestureProbe`
- Composer-Materialisierungen: HarmonicPitchSwap, RhythmicTextureRebuild, PackageUnhide, SingleUnhide
- Consensus-Materialisierungen: DirectUnhide, GaussCandidate, GaussChain, BrokenGesture, ConstrainedTail
- SharedDecisionOwner: `none`
- OwnershipState: `multiple_productive_acceptance_owners_same_hidden_material_decision_open`

## Was r344 nicht tut

Keine produktive Rust-Datei wird geaendert. Keine Note, kein Pitch, kein Onset, keine Dauer,
kein Staff, keine Voice, kein Hidden-Attribut, keine Passorder, kein Threshold, kein Writer,
kein Repair-/Recovery-Pfad und kein Security-Gate werden veraendert.

r344 ist nur der enge Beweis-Take vor dem produktiven Owner-Cut.

## Naechster Take bei GREEN

Der folgende produktive Take soll keinen neuen Recovery-Pass einfuehren. Er soll einen
gemeinsamen `HiddenChordToneRecovery`-DecisionScope herstellen, den die vorhandenen
Materialisierungsstrategien konsumieren. Erst danach darf entschieden werden, welche der
heutigen lokalen Akzeptanzentscheidungen reine Materialisierungsguards bleiben und welche
als doppelte musikalische Authority entfallen muessen.
