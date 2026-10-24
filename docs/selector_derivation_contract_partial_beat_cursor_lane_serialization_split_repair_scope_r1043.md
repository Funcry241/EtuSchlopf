# Selector Derivation Contract - Partial-Beat Cursor-Lane Serialization Split Repair Scope r1043

SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: repair_scope
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: measurement_only
CorpusDistributionMayRefineClassToKnownWitness: no
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
ParentErrorClassContract: docs/selector_derivation_contract_partial_beat_cursor_lane_serialization_split.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: each frozen partial_beat_cursor_lane_serialization_split candidate owns exactly one physical measure-local cursor traversal and may be evaluated independently from candidates in other physical measures
CausalFeature: multiple candidate plans within the same physical measure remain one unresolved conflict group and receive no positive write authority
CausalFeature: every independently adopted measure-local transaction must preserve the complete musical event multiset, note count, parse validity, backup-forward accounting and exact local beam/readback invariants
NonCausalContextExcluded: project identity, etude identity, concrete measure number, page or system location, corpus rarity, discovery order and scorewide candidate count
NonCausalContextExcluded: global first/last/ranking position and any known witness location

## Repair-scope derivation

The accepted r1042 parent froze `partial_beat_cursor_lane_serialization_split` as a measure-local serialization defect and explicitly proved that distinct physical measures have disjoint mutation domains. r1043 changes only the multiplicity scope of the existing living owner: candidate discovery remains the same, but scorewide `candidates.len()!=1` no longer owns positive or negative write authority across independent measures.

Each physical measure is one explicit conflict group. A group with exactly one existing stitch plan may run through the unchanged local invariant proof. A group with more than one plan remains fail-closed and is left byte-identical. Failure or ambiguity in one measure does not veto a proven independent measure-local transaction.

The candidate-duplication probe is executable regression evidence: adding a second independent measure preserves the first measure's repaired bytes and allows both local transactions. A separate regression keeps multiple candidates inside one measure fail-closed.
