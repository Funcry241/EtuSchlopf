# Source selection for an irreducible hand span

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
ParentErrorClassContract: docs/selector_derivation_contract_irreducible_hand_span.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: the selected base candidate supplies a hand span warning and the source-selection owner permits reduction
CausalFeature: the complete source attack and held keys admit no pitch-ordered, non-crossing two-hand assignment within the existing span and key count limits
CausalFeature: one unprotected interior attack can be excluded while a simultaneous note on the same hand retains the full attack duration, the outer attacks remain and the assigned hands satisfy the limits
CausalFeature: a protected grace host, a repetition of a held key or a note continuing past the metric boundary cannot supply the omission
NonCausalContextExcluded: piece, absolute measure, specific onset, absolute pitch, file name, project or voice label, solver order and scorewide frequency

The solver evaluates a parallel candidate with the selected base profile
before the guided search. It passes the warning locations solely to the inner
tone selector. Both candidates use the same profile parameters for initial hand
assignment, register limits, jump handling and arrangement. The usual
refinement profiles retain their existing selection rules.

The proposal uses the existing permission to reduce source notes and
the existing per-focus budget. Ambiguous local omissions yield no proposal.
The existing candidate evaluation compares the complete resulting scores.

The proposal uses the existing source selection decision and removal evidence
for ergonomic span reduction. It leaves the source event data and the time
and pitch of retained attacks intact. It does not create an XML rewrite.
