# Selector Derivation Contract - CompleteTailBeatBeam Local Multiplicity Authority

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
ParentErrorClassContract: docs/selector_derivation_contract_complete_tail_beat_beam_multiplicity_error_class.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
WriterTopologyContract: existing_writer_root_only_v1

CausalFeature: every CompleteTailBeatBeam occurrence is classified by the already accepted local_tail_beat_beam_serialization_incompleteness parent class and is authorized independently of scorewide candidate multiplicity
CausalFeature: the conservative explicit conflict group is the current source-event serialization mutation domain; more than one candidate inside that same domain remains fail-closed because the existing writer has no proof that the edits commute
CausalFeature: candidates in distinct non-overlapping source-event mutation domains are processed without global ranking; after every successful candidate the current XML is reparsed and all locators are rebound before another token may be issued
CausalFeature: a candidate that fails local authorization or materialization cannot suppress a candidate in a different conflict group; all locally unambiguous current candidates are attempted before the pass can conclude that no progress is possible
CausalFeature: GenericityEvidence requires identity metamorphoses, local negative controls and an explicit independent-candidate duplication invariant, but observed scorewide candidate_count is diagnostic only and grants no write right
CausalFeature: independent duplication is a positive metamorphic proof; duplicating a candidate into another mutation domain preserves two locally feasible class members, while duplicating it inside the overlapping source-event domain remains a negative conflict control
CausalFeature: post-write semantic equivalence scales the permitted encoding-only normalization by the actually proven chord removals, so a batch of independent local writes is validated by the same per-occurrence invariant rather than by a one-candidate constant

NonCausalContextExcluded: project identity, etude identity, absolute measure number, concrete pitch, concrete voice label, concrete staff number, page, system, corpus rarity, discovery order, first-or-last occurrence and global candidate count
NonCausalContextExcluded: scorewide len()==1, candidate_count==1, unique_candidate helper semantics, take(1), first-wins selection, global ranking and any cap that can turn an independent second occurrence into a veto
NonCausalContextExcluded: the observed number of candidates in the current ten-piece evaluation corpus; corpus multiplicity is measurement only
NonCausalContextExcluded: historical regression origin, prior revision identity and renderer/PDF outcome as repair-scope selectors

## Implementation binding

The accepted r1097 parent freezes the causal error class before this selector-sensitive
take. r1100 leaves that parent contract byte-identical.

The existing detector still sees only identity-free StructuralView data. The writer root,
private verifier token and source-bound locator remain the same. The change is solely the
scope in which uniqueness is allowed to matter:

- scorewide multiplicity is never a write veto;
- local overlapping-source-event ambiguity remains a safety veto;
- independent mutation domains are rebound and handled one by one;
- no stale locator is reused after a committed candidate;
- no candidate wins by discovery order because unsuccessful local candidates do not stop
  attempts in other independent conflict groups, and successful candidates trigger a
  fresh complete re-detection before continuation.

The duplicate-candidate proof is executed inside the existing synthetic tests so no new
test-function capability surface is introduced.
