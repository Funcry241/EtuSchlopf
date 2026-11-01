# Joint boundary observation reuse: unchanged single-insertion semantics

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
ParentErrorClassContract: docs/selector_derivation_contract_joint_register_reading_context.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
NonCausalContextExcluded: corpus identity, printed labels, pitch identity as selector, rarity, discovery order and elapsed time

CausalFeature: repeated single-insertion capability probes consume the exact same immutable part bytes and original boundary observations
CausalFeature: the existing supported temporal precondition makes single-probe eligibility independent of other probes

## Scope and accepted parent

This separate performance take is based on accepted r1567, not on a rejected C3
candidate. It contains no r1568--r1572 integration, repair or diagnostic changes.
The current beam/voice investigation stays separate. The old common register
course selection remains in place and is not optimized by this first step.

The existing observe_joint_boundaries probes one Bass insertion with no
removals for each lower pitched event's original clef-transition anchor. Every
probe previously rebuilt the same immutable part observations. The new local
context borrows exactly that part source and retains its supported TemporalPart
and original beam-boundary set. Existing surface and pitch scanners can consume
these same observations. Their other callers retain their original wrapper
behavior, including serial fallback for unsupported temporal input.

## Equivalence argument

The original initial TemporalPart precondition remains required. With that
supported context and exactly one insertion and no removals, sorting, duplicate
removal, removal overlap and same-position insertion collisions cannot change
the single probe. Source bounds, UTF-8 boundary, note-start requirement and tie
protection stay identical. A missing beam result is rejected by the existing
normalizer before any serial fallback can admit it. Some(false) is outside a
group; Some(true) requires the exact original fresh-attack boundary exception.
These are the unchanged single-insertion conditions, memoized by original byte
anchor only within the immutable part context. No global cache, source hash
approximation, voice identity reconstruction, new writer or new permission is
introduced. A different source creates a different context.

The productive normalizer, clef writer and existing guard functions are
unchanged. The prepared context does not change domain cuts, states, costs,
ordering, retained source IDs, groups, original anchors or C3 selection rules.
The result's insertion set and ordered fixed-surface vector remain equal.

## Verification and observation

Synthetic differential tests retain the parent's old boundary implementation
as a test-only reference. They compare complete results across ordinary notes,
ties, complete/unknown beams, cross-staff beams, chord members, hidden material,
unsupported clocks and octave directions, with independently renamed labels.
Another test compares each memoized answer with the unchanged productive
single-insertion normalizer, including invalid positions and repeated probes.
An explicitly invoked ignored synthetic benchmark compares the actual old and
new implementations and asserts complete result equality. Its times are local
measurements, not a native corpus speedup claim or release threshold.

Console-only PhaseTiming substeps report Prepare, Scan, Anchors and Total under
InitialRenderTransaction/JointBoundaries, with part, request and unique-query
counts and a completion flag. Early unsupported returns remain unsupported;
the timing cannot authorize planning. XML titles are used only as sanitized
console labels, never as cache keys or selectors. No project-report content is
added. Native acceptance must compare all twenty MusicXML files with r1567;
report differences other than timing must be inspected, not silently filtered.
The larger common-course optimization and whole-corpus runtime goals remain
separate subsequent work.
