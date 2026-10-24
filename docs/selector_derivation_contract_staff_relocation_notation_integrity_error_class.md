# Selector derivation contract: staff relocation notation integrity (definition)

Stage A of causal_class_first_v1, frozen without selector-sensitive code. The classes below define when relocating a single pitched note from one piano staff to the other, while it keeps its serialized position and voice label, produces a structurally invalid or unplayable notation. They decide no candidate, repair, write, ranking or release. A later implementation take references this file as ParentErrorClassContract and may only use these classes to withhold such a relocation from being offered.

The classes follow from MusicXML and keyboard semantics, not from any corpus distribution: a MusicXML voice is one sequence of chords in time, so two pitched events of the same voice label that overlap in time without sharing onset and duration cannot both belong to it; a chord is one event, so its members share one staff assignment; and a note handed to the lower staff is played by the left hand, which cannot sound above notes the right hand holds at the same time without crossing.

```text
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
CausalFeature: chord split = the relocated note is a chord member, or a chord head whose chord members would stay on the other staff, so one chord event would carry two staff assignments
CausalFeature: voice timeline collision = after relocation the destination staff holds another pitched non-chord event with the same voice label in the same measure whose time interval overlaps the relocated note without having the same onset and the same duration
CausalFeature: hand crossing = after relocation to the lower staff the relocated pitch lies above a pitch of the upper staff sounding at its onset, or after relocation to the upper staff it lies below a pitch of the lower staff sounding at its onset
NonCausalContextExcluded: project, etude, absolute measure number, part id, specific pitch values or registers, renderer behaviour, owner run order, candidate profile or rank, warning counts, corpus frequency and WAV-derived values
```

Rests, hidden rests and grace notes are no events for these classes. Relocations that move a complete chord, or that do not change the staff, are outside the classes. A renderer that rejects such notation is an observation of the consequence, never a feature of the class.
