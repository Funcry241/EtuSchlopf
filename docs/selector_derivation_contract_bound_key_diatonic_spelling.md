# Diatonic spelling under an already bound key signature

SelectorDerivationContract: causal_class_first_v1
SelectorChange: no
SelectorRole: error_class_definition
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: not_yet_measured
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: none
ErrorClassDefinitionIndependentOfKnownWitness: yes
ErrorClassName: bound_key_diatonic_spelling_skipped_by_key_rewrite_gate
CausalFeature: a current committed key signature binds the tonal context at a written occurrence and supplies a unique diatonic spelling for the retained sounding pitch
CausalFeature: the retained occurrence uses an equivalent spelling inconsistent with that key and has no independently bound conflicting local spelling obligation
CausalFeature: the spelling decision is skipped because a key signature rewrite is not authorized even though the already bound key has not changed
NonCausalContextExcluded: project, etude, movement, title, page or measure address, concrete key, pitch class, octave, voice number, final bass note, accidental count and corpus frequency

## Scope

Key-signature selection and diatonic spelling under a bound signature are
separate decisions. An existing key signature can remain unchanged while
the existing tonal spelling owner selects the uniquely diatonic spelling.
The active key must be established by the actual notation course.
A neutral placeholder or an ambiguous key boundary supplies no additional
spelling authority. An independent local spelling obligation or a conflicting
spelling decision in an original tie chain leaves the incumbent in place.

This class concerns sounding pitches that belong to the active signature's
diatonic pitch set. A chromatic pitch outside that set does not follow from
this definition. A later chromatic spelling decision needs independent
evidence from its melodic or harmonic function. A minor-mode inference is
unnecessary for the diatonic pitch set shared by relative keys.

## Authority and verification

The existing tonal spelling owner must retain the exact key declarations.
Use the original occurrence identity and existing complete-chain obligations.
A spelling edit must preserve the computed MIDI pitch, onset, duration,
staff, voice, chord, rest and grace disposition, visibility and tie edges.
Do not spell a single sounding key differently inside a chord. Keep written
accidentals and their display state consistent with the new pitch spelling.
Verify the Render source and the actual PDF. A successful Master-only
respelling does not prove the Render result.

Use independent synthetic examples with flat and sharp signatures,
a chromatic pitch that needs no diatonic change, and a bound tie chain.
Verify that an unrelated duplicate candidate does not change either local
decision. A later selector-sensitive take must reference this unchanged
definition in its accepted parent.
