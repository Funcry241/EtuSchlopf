# Owner receipt failure diagnostics: observation implementation

This implementation consumes only the mechanical failure classes frozen by
accepted r1316. It projects error context from bound receipt data and documents.
It supplies no candidate selection, musical repair, writer or adoption authority.

The existing Exact owner verifier remains the refusal predicate. Its newly added
output distinguishes a non-EXACT descriptor from a non-reproducing origin-bound
edit script. The existing refusal counter, None result and successful mint path
retain their semantics. The bounded span list is presentation only; it neither
truncates verification nor changes class membership.

The coverage projection begins at the fingerprint of the actual bound pre-state,
walks supplied receipts in their original order, observes the first mismatching
pre-state and compares the resulting end with the actual proposed post-state.
It returns no text for a continuous covered byte transition. Its optional text
is used only for a best-effort stderr write immediately before the existing
SafeAtomic adoption. All existing fatal checks and SSC verdicts remain byte-identical.

```text
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
ParentErrorClassContract: docs/selector_derivation_contract_owner_receipt_failure_diagnostics.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: an existing exact owner attempts certification of a changed bound transition and its existing descriptor or origin-bound byte edit script fails the existing exact-receipt verifier
CausalFeature: a supplied receipt pre-state fingerprint differs from the existing transition head obtained from the bound pre-state and preceding supplied receipts
CausalFeature: the end fingerprint of the supplied receipt sequence differs from the actual document offered for adoption, with the bound pre-state serving as the end when the sequence is empty
NonCausalContextExcluded: project, etude, measure number, pitch, staff, voice label, caller label, patch name, iteration, owner identity as a membership selector, corpus distribution, candidate multiplicity, warning counts, worker count, execution timing and ambient inputs
```

Each transition is observed independently. Observing the same transition again,
or adding an unrelated transition, does not change its classification. A duplicate
observation can produce another line but cannot mint, consume, authorize or select
anything. Synthetic tests probe unchanged, covered, empty, partially covered and
discontinuous sequences; repeated and independent observations preserve their
results. The existing adoption tests remain the authority checks, including reused
identity refusal, and the real adoption follow-through proves no receipt consumption
by the projection.

This is diagnostic error_classification, not repair_scope. There is no new productive
write or decision controlled by the classification, so the stated duplication
boundary applies to the observation; no writer receives this result as permission.
