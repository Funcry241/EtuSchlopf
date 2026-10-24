# r372 HiddenChordToneRecoveryAudioSalienceDecisionOwnerClosure

Parent: r371 `RootCargoLockTransactionalResolutionOwnerCompileClosure`

ChangeClass: `RefactorOnly`

## Anlass

Die StaffVoice-Untersuchung auf dem real GREEN r371 bestaetigt zunaechst die bereits in r344
dokumentierte Abgrenzung: `run_editorial_voice_decision_pass` bereinigt editoriale Voice-Rollen
innerhalb eines Staffs, `run_conflict_staff_split_proof_trial` verteilt dagegen in einem
Audio-/HAN02-Konflikt eine konkrete Note staffuebergreifend. `build_editorial_voice_trial_candidate`
ist lediglich isolierter CandidateBuilder. Daraus entsteht kein gemeinsamer StaffVoice-DecisionOwner.

Beim anschliessenden semantischen `Other`-Cluster-Audit wurde jedoch frische konkrete
Same-Decision-Evidenz fuer den bereits bestehenden `HiddenChordToneRecovery`-Owner sichtbar.
Drei Audio-Salienz-Pfade konsumieren denselben
`collect_hidden_chord_tone_recovery_candidates`-Kandidatenraum und duerfen daraus einen neuen
`best.layout_xml`-Zustand adoptieren, ohne den seit r345 vorhandenen gemeinsamen
`decide_hidden_chord_tone_recovery_adoption`-Owner zu konsumieren:

- `run_salience_weighted_candidate_gate`: direkte Sichtbarmachung role-/audio-bestaetigter
  verborgener Akkordtoene.
- `run_discrete_action_salience_translator`: sichtbare Pitch-Substitution aus einem verborgenen
  Akkordtonkandidaten.
- `run_conflict_staff_split_proof_trial`: dieselbe Pitch-Substitution plus eine getrennte
  Staff-1→Staff-2-Verteilungsentscheidung.

Das ist neue Quell-Evidenz und kein Wiederaufreissen aufgrund von Scannerzahlen.

## Closure

Die drei vorhandenen Pfade bleiben erhalten und behalten ihre lokalen, strengeren
Materialisierungs-/Scope-Guards. Ihre HiddenMaterial-Adoptionskomponente wird zusaetzlich an den
bestehenden writer-freien DecisionOwner gebunden:

- WeightedUnhide → `ConsensusGesture`
- AudioSalience PitchSubstitution → `HarmonicPitchSwap`
- ConflictStaffSplit PitchSubstitution → `HarmonicPitchSwap`

Der Staff-Split selbst bleibt bewusst eine getrennte StaffVoice-/Handverteilungsentscheidung.
Es entsteht kein UniversalOwner und kein neuer Pass.

## Verhaltensinvarianz

Die Bindung ist logisch redundant zu den bereits vorhandenen lokalen Freigaben und kann deshalb
keinen bislang abgelehnten Kandidaten neu freigeben:

- WeightedUnhide verlangte bereits exakt 0 Blocker, 0 Warnungen, 0 kritische Befunde, keine
  Hint-Verschlechterung und positiven Sichtgewinn. Das ist dieselbe Predicate wie die bestehende
  `ConsensusGesture`-Projektion.
- Beide Pitch-Substitutionspfade verlangen bereits 0 Blocker, 0 Warnungen, 0 kritische Befunde
  und keine Hint-Verschlechterung. Damit erfuellen sie die schwachere bestehende
  `HarmonicPitchSwap`-Projektion automatisch. Die bisherigen strengeren lokalen Guards bleiben
  zusaetzlich bestehen.

Erwarteter MusicXML-/Render-/PDF-Delta: null.

## Ownership-Evidence

Die bestehende `HiddenChordToneRecovery`-Projektion wird um die drei realen Audio-Salienz-
Materialisierungen ergaenzt. Zielzustand bleibt:

`HiddenChordToneRecovery -> decide_hidden_chord_tone_recovery_adoption -> mehrere legitime Proposal-/Materialisierungsprojektionen`

## Nicht geaendert

- keine neue Musikmutation oder neue Freigaberegel
- keine Pass-Reihenfolge
- keine Schwelle, Gewichtung oder Candidate-Rangfolge
- kein neuer Writer
- keine neue Repair-/Recovery-Authority
- keine Etueden-/Takt-/Pitch-Sonderlogik
- keine Security-/Gate-Lockerung
- keine Cargo-/Lockfile-Aenderung
