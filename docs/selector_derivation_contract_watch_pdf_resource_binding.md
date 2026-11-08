# Watch PDF resource binding implementation

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
ParentErrorClassContract: docs/selector_derivation_contract_watch_pdf_resource_identity.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write

CausalFeature: the current observation session, encoded resource label and exact in-flight request entry determine whether an asynchronous response may update that resource's visible state; the server additionally requires the requested session to equal its current session before reading a PDF.
CausalFeature: loading, ready and failed are outcomes of that resource request; an unrelated snapshot cannot retry a failed request, and explicit retry remains single-flight for its address.
NonCausalContextExcluded: project spelling is used only as a resource address; no named score, measure, music, occurrence count, timing threshold or corpus distribution grants a read or a visible result.

The unchanged parent definition owns the class. This implementation does not
broaden musical selection: it restricts UI responses to their actual request
and leaves the existing project-root, freshness and PDF-read checks in place.
Two independent resources, reordering, consistent renaming and session changes
are exercised synthetically in the browser. Duplicate controls of the same
address share a read without merging their focus identities. No product file
is written by the Watch UI and no proof or release authority is created.
