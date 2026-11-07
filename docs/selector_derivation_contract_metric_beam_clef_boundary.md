# Metric primary beaming across an effective clef change

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
CausalFeature: the metric primary-beam owner receives a notation surface with an already effective visible clef transition on the staff whose notes it groups
CausalFeature: the owner creates or extends a primary beam with consecutive group members on opposite sides of that transition in musical time on the same staff
CausalFeature: the owner's input did not connect those members across that boundary and its output introduces the connection while the clef boundary remains effective
NonCausalContextExcluded: piece identity, file name, measure label, page, absolute position, numeric staff or voice label, pitches, clef direction, exact durations, group size, discovery order, corpus frequency and uniqueness

## Meaning

The class concerns an existing clef transition that a later metric-beam decision
newly spans. At an attack coinciding with the transition, the note belongs to
the new clef state if that state governs the attack in the owner input. Time
and staff attachment must be resolved from that input, including cursor
changes. A repeated equivalent clef indication, a transition on another staff,
or an already connected pair carried unchanged does not establish the class.
An unresolved effective state or an unbound pre/post comparison yields unknown.
A PDF collision can corroborate impact but cannot establish the causal sequence.

This class leaves open how the existing owner should compare admissible groups
and how ownership and continuity are preserved when it emits positive or
negative beam decisions. Clef placement, retained foreign beam truth,
cross-staff groups and pre-existing collisions require their own evidence.
The class does not authorize moving the clef, editing notes, or adding a
postprocessing pass.

## Stage A

Parent: accepted r1602. This take freezes the class only. It defines no
classifier, repair selector, writer, threshold, or new authority. There is no
corpus distribution measurement and no intended MusicXML or PDF change.
Any later implementation must refer to this accepted definition unchanged.
The synthetic proof must remain valid after relabeling, relocation and
independent duplication. The real corpus serves as evaluation evidence.
