# Selector Derivation Contract - Render Epoch Excluded-Path Bridge Accounting r1185

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
ParentErrorClassContract: docs/selector_derivation_contract_semantic_obligation_accounting_failure.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: r1184 already proves same-path render-epoch drift only when the previous accepted after-fingerprint differs from the next accepted before-fingerprint; r1185 does not change that drift decision and adds only read-only accounting over recorder calls that were already observed but excluded from accepted render-path continuity
CausalFeature: every recorder call receives a monotonically local observation serial inside one project session; an excluded-path state can bridge one later same-stream drift only when its complete-byte after-fingerprint exactly equals that drift boundary's complete-byte before-fingerprint and the excluded observation occurred strictly after the previous accepted observation of that same stream and strictly before the current boundary
CausalFeature: exactly one temporally eligible excluded-path state with the required complete-byte fingerprint yields EXPLAINED_BY_EXCLUDED_PATH_STATE; more than one yields AMBIGUOUS_EXCLUDED_PATH_STATE; none yields UNEXPLAINED
CausalFeature: an excluded-path match explains only that the byte state seen before the accepted boundary was already observed at an excluded recorder path in the eligible temporal window; it does not prove which persistence mechanism moved those bytes, does not identify an owner, does not prove semantic equivalence, and does not authorize a repair or write
CausalFeature: accepted render-stream continuity remains independently keyed by the normalized persisted path from r1184; excluded-path observations never become accepted streams, never replace previous accepted fingerprints, and never change whether a boundary is FIRST CONTINUOUS or DRIFT_OR_EPOCH_RESET
CausalFeature: observations outside accepted paths that do not fingerprint-match the current same-stream before-state remain accounting evidence only and cannot explain the drift; adding an unrelated excluded observation with a different fingerprint cannot change an existing explanation
NonCausalContextExcluded: project identity etude identity measure number page system staff voice onset duration pitch register hand label clef spelling known witness location and corpus rarity
NonCausalContextExcluded: semantic kind name physical path text excluded-path filename suffix and known L1 or L2 labels as explanation selectors
NonCausalContextExcluded: carrier counts silence metrics sounding-signature outcome effect-family name writer name authority name and report ordinal as transition-accounting selectors; only temporal eligibility plus exact complete-byte fingerprint equality selects a bridge candidate
NonCausalContextExcluded: expected corpus count expected ratio of explained to unexplained drift and any rule designed to force the r1184 corpus distribution into a predetermined result

## Derivation from the accepted parent class

The accepted `semantic_obligation_accounting_failure` contract requires accounting facts to be transported rather than reconstructed from incidental musical coordinates. r1184 corrected the physical continuity scope and produced deterministic same-path drift. It also showed that the existing recorder already receives calls for some paths that are intentionally excluded from accepted render continuity.

r1185 preserves the r1184 drift classifier exactly. It asks one narrower measurement question only after a drift already exists: whether the exact byte state found immediately before that accepted boundary was itself observed by an excluded recorder call during the temporal window since the previous accepted observation of the same stream.

This is a state-bridge observation, not causal attribution. The excluded path, semantic kind and musical contents are not selectors. The only positive evidence is equality of the complete-byte fingerprint plus temporal placement inside the same-stream observation window.

## Multiplicity and ambiguity

One eligible matching excluded state is enough to report that one excluded-path state bridges the observed byte state. Two or more matching observations remain ambiguous even when their bytes are identical, because the audit cannot identify which persistence event supplied the accepted path. Zero matches remains unexplained.

Adding an unrelated excluded observation with a different fingerprint cannot change any existing classification. Duplicating a matching excluded observation changes EXPLAINED to AMBIGUOUS rather than selecting one by ordering, rarity or first-wins behavior.

## Authority boundary

This contract covers selector-sensitive audit classification only. r1185 changes no music, productive selector, writer family, StructuralWriteAuthority surface, repair scope, threshold, MasterApply, proof, permit, release authority or static destructive-effect inventory.

The existing r1184 accepted render-stream continuity, carrier census and drift count remain authoritative for whether drift exists. `EXPLAINED_BY_EXCLUDED_PATH_STATE`, `AMBIGUOUS_EXCLUDED_PATH_STATE` and `UNEXPLAINED` are read-only accounting labels for an already-detected drift. None may be converted into a repair decision without a later independently governed take.
