# Continuity of independently declared source voices in MIDI notation

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
ErrorClassName: declared_source_voice_continuity_lost_in_notation
CausalFeature: authoritative source information binds positive-duration occurrences to independent voice identities before notation allocation
CausalFeature: the written occurrence binding establishes a loss of independent continuity or multiplicity across those voices in the resulting notation
CausalFeature: simultaneous occurrences without a positive joint-notation decision must retain the information needed to distinguish their source voices
NonCausalContextExcluded: project name, title, measure address, track or channel number, track-name text alone, pitch register, numeric voice label, staff number, renderer output and corpus frequency

A voice relation must come from a positive source declaration or a separately
qualified authority. Track and channel fields identify recorded events.
They may implement a declared mapping, but neither field alone declares a
voice. A track can carry more than one voice, and channels may change for
playback reasons. The mapping has to join to the existing causal note-on
and note-off evidence by source occurrence, including distinct occurrences
that share pitch and onset.

The error is a lost written voice obligation, not simply the reuse of a
label or a change of staff. The same voice can legitimately cross staves
or change technical channels. An explicitly authorized shared engraving
can preserve two independent obligations through one glyph. Conversely, a
visible notehead for each pitch is insufficient when occurrences from
separate voices become one indistinguishable written course.

The MIDI source ledger supplies the occurrence provenance. The seed and
serialization channel owners are responsible for representing a qualified
voice relation. This definition establishes neither a declaration format nor
a staff, stem, beam or channel choice. Missing, overlapping or contradictory
declarations leave those decisions to existing authorities.

Independent synthetic evidence must include two interleaving lines, a
same-pitch simultaneous pair, one legitimately shared engraving, one
cross-staff line, and tracks or channels with no voice declaration. Renaming
opaque identities, permuting tracks and duplicating disjoint musical
components must preserve the associated local decision. Compare against the
original pitch, attack and release multiset, including multiplicity.
