# SSC v4 all-commit measurement (Take 5a, r1285)

Stage B of causal_class_first_v1. The existing integrity, invariant and
containment classes also measure committed events absent from their domain's
released scope. The same classify_record -> classify_bound_origin ->
observe_ssc_event_v2 path classifies both sets. No class, envelope or owner
purpose is widened. Measurement chooses, ranks, repairs and writes nothing.
PermitEnforcement stays none; necessity remains measurement only.

```text
SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: error_classification
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: measurement_only
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: docs/selector_derivation_contract_ssc_containment_error_class.md
ErrorClassDefinitionIndependentOfKnownWitness: yes
CausalFeature: containment violation = a difference between the exact captured pre and post bytes lies outside the declared element kinds, fields or operations of the descriptor's frozen envelope row, or outside its bound span
CausalFeature: invariant violation = the envelope's named invariant (conserved projection, single insertion geometry, edit-script locality, bound-span locality, lane fields) does not hold on the exact pre and post bytes
CausalFeature: containment unproven = the descriptor has no envelope row, or the captured bytes cannot be read, so containment cannot be decided constructively
NonCausalContextExcluded: project, etude, absolute measure number, part id, pitch values, owner run order, candidate profile, corpus frequency, WAV-derived values and whether the change was musically necessary
```

Released scopes remain authoritative for the released denominator. Publishing
that scope no longer deletes candidate commit evidence. Reports append a
separate V4UnreleasedCommits projection without changing existing V2/V4 lines.
Each retained record counts once as checked. Repeated scope publication does
not add records. Duplicate identities among unreleased records produce an
integrity failure for each affected record, rather than a silent collapse.
An identity found only in the opposite released domain is a DomainMismatch.

V4AllCommitsPermitted is yes only when both released scopes exist, the released
set has no integrity failures and every released and unreleased record has a
v4 Permit. Missing scope, evidence, duplicate, invalid, OutOfScope, Denied or
Unproven prevents yes. Empty known scopes with no records are vacuously yes.
Records are cleared by the existing project reset. Their compressed captures
remain unchanged; classification temporarily decodes one event at a time.

Synthetic tests use actual sealed owner receipts and candidate adoptions.
A scalar edit to octave deliberately escapes the voice/staff envelope; it is
an unreleased Denied witness, never a new musical permission. The corpus is
used only to measure the distribution, not to redefine a class.
