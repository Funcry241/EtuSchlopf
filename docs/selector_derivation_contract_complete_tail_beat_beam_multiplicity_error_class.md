# CompleteTailBeatBeam multiplicity error class

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

ErrorClassName: local_tail_beat_beam_serialization_incompleteness

CausalFeature: one identity-free local structural window contains a metrically closed tail-beat event topology whose sounding, hidden-carrier, rest and chord roles are internally sufficient to classify the tail-beam state without project, measure, pitch, voice-label or staff-number identity
CausalFeature: the local tail-beam state is incomplete or cross-beat-mis-scoped while the same local musical event multiset, sounding identity and cursor accounting can remain unchanged under a beam-only normalization
CausalFeature: class membership is local to the structural window and does not depend on how many other independent windows in the score exhibit the same causal topology
CausalFeature: two occurrences whose serialized mutation domains are disjoint are independent class members; the existence of one cannot create or remove class membership of the other
CausalFeature: ambiguity inside one local structural window remains fail-closed unless a later accepted contract proves a smaller disjoint conflict partition

NonCausalContextExcluded: project identity, concrete etude identity, absolute measure number, concrete pitch spelling, absolute register, current corpus frequency, scorewide candidate count, first-or-last occurrence, global ranking, rarity and any feature whose purpose is to leave one known witness
NonCausalContextExcluded: a second independent occurrence of the same local class is not evidence against the first occurrence
NonCausalContextExcluded: renderer success, PDF geometry, historical regression origin and prior revision identity are not class-membership features

## Definition

A `local_tail_beat_beam_serialization_incompleteness` exists when an identity-free
local structural view already contains the complete event-role topology needed to
describe a metrically closed tail-beat beam group, but its beam state is incomplete
or serialized across the beat boundary in a way that can be normalized without
changing musical events, sounding identity, cursor accounting, project identity or
any other independent window.

This document freezes only the causal class. It grants no selector, repair scope,
authorization token or writer authority. The installed r1096 implementation remains
byte-identical in this take.

## Independence boundary

The class is occurrence-local. Adding another structurally equivalent occurrence in a
different disjoint serialization window must not change whether the first occurrence
belongs to the class. A later repair-scope take must prove the concrete mutation-domain
disjointness and candidate-duplication invariant before productive write authority may
be widened.

Multiple interpretations that compete for the same local serialized events are not
declared independent here and remain fail-closed.
