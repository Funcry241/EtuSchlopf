# Solver pool retention projection (RefactorOnly, prepared against frozen r1558)

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
ParentErrorClassContract: docs/selector_derivation_contract_retained_payload_outside_future_observation.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: the existing SubstanceGuard full consumer requires zero blockers and zero warnings; these facts are immutable after pool insertion, so an otherwise rejected entry with a blocker or warning cannot later be adopted from this pool
CausalFeature: the remaining pool consumers read iteration and final source-selection and solver-frontier projections plus the unchanged decision key, not the excluded entry's document or other full payload
CausalFeature: existing candidate generation and primary selection still operate on full candidates before insertion; a projected entry is never a new candidate, a new identity or a substitute winner
CausalFeature: one ordered pool preserves original indices and multiplicity, and full eligible entries remain movable into the incumbent unchanged under the original SubstanceGuard visible-count, hint and decision-key rules
CausalFeature: prior incumbents are conservatively retained in full even if the permanent exclusion predicate would permit projection, limiting the scope of early destruction to never-adopted rejected payload
NonCausalContextExcluded: project etude measure pitch staff voice identity values profile labels iteration values corpus size fitted memory coefficients and historical winner frequency as reasons to compact an entry

## Parent and delivery boundary

The r1558 definition is referenced unchanged. Preparation and provisional local
checks against its frozen candidate do not establish native acceptance of that
parent. This implementation is not delivered for installation until the actual
r1558 acceptance and source binding have been verified from the native export.
No source, test, gate or dependency from the rejected r1557 sampler is included.

## Data representation and unchanged readers

A single ordered SolverCandidatePool owns entries with the exact carried source
selection row, solver frontier row and CandidateDecisionKey. Eligible entries
and prior incumbents also own their full SolverCandidate. Excluded rejected
entries do not retain a full candidate. Projection precedes destruction.

- Initial batch generation and initial best/guard selection still use the original
  Vec<SolverCandidate>. Only the leftover pool is converted, in its existing order.
- Guided refinement and all repair rounds perform the original comparisons before
  appending a rejected entry. Every previous incumbent uses push_previous and
  retains its payload. Rejected batches retain iteration order through extend.
- next_iteration consumes the same carried iteration values and current incumbent.
- SubstanceGuard evaluates every slot, in the same order after parallel results
  are sorted by original index. Its compatibility, hint window, visible count,
  comparison order and strictly increased-visible adoption requirement are
  unchanged. A compatible slot must have a full payload by construction.
- finish consumes the exact extracted source-row construction and frontier-row
  construction. Duplicate source-ID consistency checks and row order remain.
  The selected final incumbent is projected fresh and appended last as before.

The mathematical criterion is existing permanent ineligibility, not observed
nonselection. A new eligible alternative can still replace best; historical
corpus behavior is not used as an assumption. Compacting two independent excluded
entries never changes eligibility or ranking of an unrelated full entry.

## Lifetime and evidence boundary

SolverCandidate itself has no custom Drop. Its master state retains document
bytes, committed lineage event values and optional shared custody/capture. Pool
compaction does not consume OwnerReceipts or AdditiveTransformationReceipt values:
committed lineage records hold committed contracts and opaque identities, not
pending receipt guards. Rejected payload destruction must mint no identity,
change no evidence counter and remove no globally adopted evidence. Existing
ADOPTED_EVIDENCE remains owned by its original process-wide collection.

Shared Arc-backed immutable continuations may remain alive through other owners;
removing this pool's reference does not claim exclusive ownership or a guaranteed
amount of freed memory. No ownership-count-based candidate decision is added.
Existing input snapshots and generation order remain unchanged. The incumbent
and all prior incumbent payloads remain full to avoid moving their release point.

The exact reachable field/ownership inventory and all pool access sites must be
reviewed with the final diff. Any unexpected consumer or lifetime effect blocks
release rather than justifying dropped evidence or an expanded projection.

## Validation

Synthetic tests exercise: full compatible adoption with identical document;
permanent warning exclusion despite potentially greater visible content; exact
projection values and interleaved row order; unchanged first equal alternative;
no adoption without visible gain; independent excluded duplication; and full
retention of prior incumbents. No real corpus fixtures or music writers are used.

Before delivery: actual final package versus accepted parent, unchanged gates,
strict tests and Clippy/build as appropriate. Native acceptance requires all
Master and Render XML byte-equal to r1558/r1556 and unchanged candidate,
SourceSelection, SolverFrontier fingerprint and SubstanceGuard evidence. Timing
lines may differ; evidence differences are not explained away as timing.

Memory savings are unmeasured. A native same-host before/after single-project run
may record the live process peak and paging/private memory separately. A model
or candidate-count reduction is not a substitute for measured allocation savings.
The conservative prior-incumbent retention means the static count of ineligible
rows is an upper bound on compacted entries, not a promised number of releases.
