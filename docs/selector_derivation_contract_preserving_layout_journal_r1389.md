# Received course continuation through recorded layout writes — r1389

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
ParentErrorClassContract: docs/selector_derivation_contract_unbound_rest_context.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: the frozen E044 or E013 writer records the operations of its own authorized token and attaches them to the receipt it has just minted
CausalFeature: the journal must replay the authorized replacement and the actual write and bind the original receipt fingerprints
CausalFeature: no recorded fragment may remove or insert a rhythmic element and every complete note stays byte-identical
NonCausalContextExcluded: piece identity measure label pitch register voice label value staff distance value corpus distribution converter outcome and independent candidate count

Parent: accepted r1388. Implementation scope only; no new error class.
The branch conditions in `local_staff_distance_operations` mirror the unchanged
legacy planner (existing lower staff distance, self-closing print, print element,
no print) to record its operations; they select no candidate, write, role or
channel. The planner's `target_distance` is the owner's own requested value, not a
selection target. Details: docs/preserving_layout_journal_custody_r1389.md.
