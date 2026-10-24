# Selector Derivation Contract - Partial-Beat Cursor-Lane Serialization Split Error Class

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
ErrorClassName: partial_beat_cursor_lane_serialization_split
CausalFeature: one measure contains a rhythmically contiguous visible pitched cohort owned by one semantic staff and voice whose event onsets and durations require uninterrupted local continuation
CausalFeature: the serialized cursor lane places an unrelated forward after an earlier cohort member and at least one backup before a later cohort member so the later event is physically split from its rhythmically contiguous predecessors
CausalFeature: the split is a serialization-order defect only when the complete note-event multiset and sounding event identity can remain unchanged by local cursor-lane restitching
CausalFeature: a local restitch is admissible only when XML parse validity and backup-forward cursor accounting remain valid and exact post-write readback proves the intended contiguous cohort
CausalFeature: independent candidates in different measures have disjoint XML mutation domains and therefore are not alternatives of one scorewide transaction
NonCausalContextExcluded: project identity, etude identity, concrete measure number, page number, system number, source offset, or corpus rarity
NonCausalContextExcluded: scorewide candidate count, discovery order, first or last candidate position, or uniqueness in the current corpus
NonCausalContextExcluded: exact pitch names, pitch contour, register, hand label, or numeric voice label when not required by the existing semantic owner
NonCausalContextExcluded: an existing known witness, its exact beat number, its exact XML byte offsets, or any feature added only to preserve a single historical location

## Causal definition

A `partial_beat_cursor_lane_serialization_split` exists when events that are rhythmically and semantically contiguous inside one measure are serialized into separate cursor-lane regions by an intervening forward/backup traversal even though their complete musical event multiset can be preserved by a local reordering. The defect is local to the measure that owns the cursor traversal. A second occurrence in another measure is therefore an independent candidate, not evidence that the first occurrence became ambiguous.

This definition deliberately does not grant repair authority. It freezes only the causal class needed to remove the existing scorewide multiplicity veto in a later take. The current `human_bound_cursor_lane_owner_transfer_xml` selector and writer remain byte-identical in r1042.

## Conflict-group boundary for a later repair scope

The smallest presently justified conflict group is one measure. Two candidate plans in different measures cannot overlap in XML node ownership because the existing transformation rewrites only the selected measure. Multiple candidate plans inside the same measure are not yet proven independent and must remain fail-closed until a separate proof establishes disjoint cursor ownership.

A later repair-scope implementation may therefore make scorewide duplication invariant by evaluating one explicit measure-local conflict group at a time. It may not use scorewide `len()==1`, measure order, source offsets, corpus rarity, or a global winner as positive write authority.

## Necessity review

1. Rhythmic and semantic contiguity is necessary; without it, separation in serialization may be intentional.
2. An intervening cursor traversal is necessary; without forward/backup separation there is no cursor-lane split to repair.
3. Event-multiset and sounding identity preservation is necessary so a serialization repair cannot silently become a musical rewrite.
4. Parse and cursor-accounting validity is necessary because a locally plausible reorder is invalid if it corrupts MusicXML timeline semantics.
5. Exact local readback is necessary so the repair proves the intended contiguous serialization rather than merely changing bytes.
6. Measure-local disjointness is necessary for multiplicity invariance; candidates sharing one measure remain one unresolved conflict group until independently proven otherwise.

## Corpus boundary

r1042 intentionally performs no new selector-sensitive Rust change and no corpus-distribution-based narrowing. The accepted r1041 corpus is evidence only that the current productive FragmentedRestChain path adopted and wrote zero transactions in all ten projects; that observation does not define the class and does not authorize retirement of the living owner.
