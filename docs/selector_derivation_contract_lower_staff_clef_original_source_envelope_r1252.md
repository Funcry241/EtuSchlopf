# Lower-staff clef original-source envelope error class

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
ErrorClassName: source_bound_clef_moves_consumed_outside_their_original_independent_state_envelope
CausalFeature: original byte coordinates are consumed after an earlier batch edit changes their referenced bytes
CausalFeature: overlapping move envelopes cannot be independently realized from the same source segments
CausalFeature: moving a clef across an attributes, clef, temporal-cursor or measure boundary changes a state domain outside the intended local preceding event interval
NonCausalContextExcluded: project identity, etude, measure number, source ordinal, byte offset value, discovery order and corpus frequency
NonCausalContextExcluded: candidate count as a global ranking rule, pitch spelling, exact MIDI threshold and a known witness identity

## Independent definition and necessity

A batch relocation is defective when consumption no longer refers to its original bound bytes or independent envelopes overlap. A single relocation is also outside the local construction class when it crosses a clef/attributes, backup/forward or measure boundary. Coordinates locate evidence; their numeric values do not select membership. A parser-success or unchanged sounding signature cannot prove effective clef scope preservation.

Original source binding is necessary to distinguish stale-coordinate consumption from ordinary edits. Envelope independence is necessary for original-segment construction; touching envelopes are independent. State barriers are necessary because the same byte relocation may affect a different clef or temporal domain even without a second move.

## Later implementation obligations

A later MusicMutation must reference this accepted contract unchanged via its own repair-scope implementation contract. It must realize the bound V2 batch construction, limit planner candidates to the preceding local clef and cursor domain, and transport authorization failures without panic or silent success. Rejects leave original bytes untouched and cannot produce an adopted receipt. Independent candidate duplication and relocation must preserve the local decision. A local move may intentionally change the effective clef for the authorized preceding low/rest interval; unrelated event domains and end-of-measure carry must retain their state. Synthetic tests must cover the actual planner, trial, receipt, adoption and released artifact. This document grants no new productive selection, SSC proof or enforcement authority.
