# Source sustain truncated at a measure boundary

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
ErrorClassName: source_sustain_truncated_at_measure_boundary
CausalFeature: a retained source event has a verified attack and an exact release beyond the notated measure boundary that contains its attack
CausalFeature: the bound notation chain stops at that boundary while the source continues
NonCausalContextExcluded: project and etude identity, score name, measure and system addresses, absolute pitches, numeric voice labels, final cadence, simultaneous event count, and corpus frequency

## Scope

Measure the source interval and the represented interval on the same
established metric timeline. Check each bound source event separately,
including each member of a chord. An attack at the next boundary is a
separate event. A continuation ends at the original release and contains
only tied fragments of the same sounding attack. An existing complete
chain needs no change.

A first implementation may extend a truncated chain when the existing
bound staff and voice have exact time coverage through the release. Use
only time that can be occupied without changing another note or an
independently bound notation obligation. A rest in that voice may be
replaced or shortened to the exact extent that the continuation occupies.
Another voice on the same staff may continue independently. A competing
note in the same voice, an early reattack, or ambiguous source binding
leaves the incumbent in place for this first implementation. A note that
begins exactly at release can follow the continuation in the same voice.

## Authority and verification

Establish the interval at the source metric presentation stage. Keep the
existing attack and its source identity. Any continuation must have its own
fragment identity and retain the lineage of that one sounding attack. Reuse
the existing serialization authority for tied fragments where its contract
supports the write. Require exact representability, contiguous metric
coverage, valid measure totals, and a matching chain after serialization.
Confirm lineage through the committed notation and render input. A renderer
success alone does not establish any of those properties.

Compare the visible attack inventory, source release coverage and voice
timing before and after the change. Cover a multi-boundary sustain and a
mid-piece release followed immediately by another attack in that voice.
Include an independent active voice on the same staff, a conflicting note
in the same voice, an early reattack, a complete existing chain, and a
system break. Read back the MusicXML and inspect the freshly rendered PDF
at each representative boundary.

This take records the error class. A selector-sensitive implementation
requires this contract unchanged in an accepted parent.
