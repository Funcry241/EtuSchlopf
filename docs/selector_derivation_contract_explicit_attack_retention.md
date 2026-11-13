# Explicit note alignment and attack retention

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
ParentErrorClassContract: docs/selector_derivation_contract_source_time_notation_projection.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: a validated written note alignment binds an explicit occurrence to each ordinal of the exact source MIDI
CausalFeature: the MIDI selection, arrangement, and playability repair owners can withdraw a bound occurrence through deletion, restification or hidden-note disposition even though the declared input contains the attack
CausalFeature: a simultaneous group exceeds the shared hand span limit under its current assignment and admits a pitch-ordered split within both hand limits
NonCausalContextExcluded: project identity, title, filenames, measure identifiers, absolute pitch identity, absolute staff identity, voice, scorewide density, candidate count and corpus membership

The MIDI seed keeps declared occurrences in its selection and transports
the alignment authority with its bound rhythmic courses. The arrangement
owner retains their visible pitched disposition. Playability candidates retain
the full multiset of visible attacks at their written positions and pitches.
The same authority applies
to each bound occurrence independently. A performance anchor map supplies
time coordinates and follows the existing selection and arrangement policy.

After seed hand selection, an explicitly aligned group with an excessive
same-hand span can change hands if a pitch-ordered split satisfies the
validator's span and key-count limits on both hands. The split minimizes
changed staff assignments, then the widest hand span. Groups without such a
split retain their current assignments. Bound source voices retain their
declared assignments. The split writes a source-selection decision for each
staff change, before score emission and grace-note host binding.
