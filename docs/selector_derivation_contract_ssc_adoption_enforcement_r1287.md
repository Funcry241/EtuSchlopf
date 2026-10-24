# Selector derivation contract: SSC containment v4 Take 5b (adoption enforcement, r1287)

Stage B of causal_class_first_v1. This take implements no new class. The frozen V4 containment classes are now enforced at the two wired adoption boundaries of contract v4 A7 (Master `commit_owner_receipts`, Render lineage `commit`): every receipt is classified by the report's own classifier before any commit, and only Permit admits it. Enforcement only refuses; it selects, ranks, repairs and writes nothing, and it never falls back to another candidate.

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
A refusal is a sovereignty violation with the constructive witness; there is no partial commit. Corpus measurement (r1285) found every released and every unreleased commit Permit, so enforcement changes no adoption and no music byte.
