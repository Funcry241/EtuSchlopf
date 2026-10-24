# Lower-staff chord attack projection: implementation classification

SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: error_classification
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
ParentErrorClassContract: docs/selector_derivation_contract_lower_staff_chord_attack_projection.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: the shared writer-free document reader retains the chord head before lower-staff visibility projection and distinguishes the first resolved lower observation, further resolved member and unresolved member
CausalFeature: the planner partitions unresolved members into independent anchorless entries but excludes them from passage attack count, while retaining pitch pressure; resolved members use the head for self-anchors and preserve independently known tie-chain origins
CausalFeature: E054 counts only the first resolved visible pitched lower observation and its own duration, retains all existing pitched pressure, and refuses a new clef insertion at a group that begins with an unresolved chord member
CausalFeature: backup, forward, attributes, measure and part boundaries invalidate current-document head attachment; an unresolved tie-start or beam-begin cannot establish a replacement origin for later continuation
NonCausalContextExcluded: project or etude identity, concrete measure or label values, pitch values or registers, absolute byte offset, corpus frequency, global candidate count, discovery order, renderer outcome and known witness location

The parent definition was accepted in r1500 and is byte-unchanged in accepted
r1501 and this candidate. Previously known exploratory corpus/synthetic results
are disclosed in that frozen parent; this take does not claim they were measured
after the original definition. No feature was narrowed to select a known witness.

Classification is local to each serialized attachment. Independently duplicated
parts retain their original materialized course byte-for-byte in synthetic tests.
No global uniqueness, ranking, action cap or new source identity supplies a write.

The existing E024 and E054 owners retain their signatures, thresholds and receipt
contracts. The E054 orphan restriction only removes an unsafe insertion location;
it does not create a new writer or general repair path. Global normalization,
primary-beam membership and tie identity observers stay unchanged. Missing local
attachment never grants an anchor through a surviving map entry.

All pinned Checkpoint58 files remain byte-identical. The integration test is in
an unpinned synthetic unit-test module reached from the planner and uses existing
crate-visible genesis/adoption APIs, real preparation, receipt verification, write,
witness and commit. This contract introduces no independent SSC authority.

The expected full corpus result is 20 byte-identical XMLs and unchanged Windows
PDF pixels, beams, ties and reentry. End-state probe neutrality is a measurement,
not proof for all pipeline intermediate states. Any unexpected corpus change must
be investigated at its writing owner before acceptance. E6/T22 and phantom remote
beam binding are not repaired here.
