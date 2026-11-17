# Local time and voice evidence for notation decisions

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
ParentErrorClassContract: docs/selector_derivation_contract_redundant_metric_note_fragmentation.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: adjacent compatible rests in one voice consume the same silent interval, with the metric owner checking whether it has a single representable rest value
CausalFeature: independent rest edits are retained and only intersecting source ranges compete
CausalFeature: whole and half-bar values follow actual metric extent and relative offsets, and written note values follow exact conversion of the local divisions
CausalFeature: visual rest proximity uses actual interval overlap, half-beat regions and quarter-beat separation measured in the local metric
CausalFeature: visible-rest priority uses local evidence and source order, with eligibility and score invariant under measure relabeling
CausalFeature: an overlapping lower-staff rest candidate requires local rest collision or overlapping notes in another lower-staff voice
CausalFeature: lower-staff alto placement depends on continuous source notes, ledger-line gain, and separation from all simultaneous companion voices
NonCausalContextExcluded: project, measure label, fixed tick widths and addresses, remote same-measure events, scorewide candidate count, corpus distribution, and voice roles outside the local sounding interval

The existing rest and rhythm writers use their exact source authorization.
The MIDI source-voice decision changes placement by source occurrence while
keeping its attacks and durations. Each area is tested with synthetic
equivalent inputs and an independent conflict or rejection case.

MIDI staff selection binds the complete occupied measures of each source
note. Measures connected by one sustained note share one decision. If
any note in such a group lacks a lower-staff proposal, the group uses
the original staff. Unrelated groups retain independent admission.

The beat-relative rest proximity applies to both observation and selection.
Measure labels identify evidence rows and carry no score bonus. The
lower-staff overlap selector reports candidates and requires distinct
written voices in the local evidence. The bound rhythmic eligibility
class is documented in the existing
`selector_derivation_contract_visible_rest_elision_rhythmic_obligation.md`.

Synthetic checks vary time resolution, measure labels, and placement or
presence of distant events. A sustained-note example verifies staff
assignment across the actual boundaries used by the seed.
