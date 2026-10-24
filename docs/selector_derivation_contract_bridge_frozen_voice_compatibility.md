# Existing bridge compatibility with the sealed voice plan: repair scope

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
ParentErrorClassContract: docs/selector_derivation_contract_beam_connection_voice_separation.md
ErrorClassDefinitionIndependentOfKnownWitness: yes
ErrorClassName: beam_connection_voice_separation
CausalFeature: a committed primary beam connection relates actual pitched occurrence containers in one serialization voice lane
CausalFeature: an actual subsequent voice assignment would place connected containers in different serialization voice lanes while retaining the connection
CausalFeature: the same conflict can be predicted before a proposed connection when an available sealed assignment binds its members to different serialization lanes
NonCausalContextExcluded: project identity, file name, measure label, staff number, numeric voice label, pitch, spelling, register, exact duration, exact member count, rest-gap presence, corpus frequency, witness counts and final PDF resemblance
NonCausalContextExcluded: independent candidate count and discovery order never determine local class membership

ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write

## Repair scope

Restrict the existing bridge writer's proposed groups to containers and their
attached chord resources whose exact current input is canonically bound and whose
already sealed serialization plan uses one common voice label. Withhold a whole
group on split labels or incomplete binding. No labels, voices, allocations,
notes, rests or durations are written by this guard. Existing selection and
beam hierarchy remain unchanged for admitted groups. An earlier available sealed
plan is not claimed to predict late alignment; that separate repair is deferred.

The guard reads authenticated canonical bindings. Range keys are exact-input
locators, never identity constructors. Numeric voice label values do not matter,
only equality in the sealed plan. Independent group duplication cannot change
another group's admission; no corpus count, ranking or witness name is consulted.
Synthetic tests must cover complete/incomplete binding, stale input, distinct
sealed lanes, chord members, exact no-op refusal, existing E050 receipt validity,
renaming, relocation, label permutation and independent duplication.

## r1484: provisional writing of sealed-split groups

The sealed plan is a forecast; the bound final assignment may reunite members
(original groups are aligned as one component). A group whose sealed labels split
is therefore no longer withheld outright. It may be written only provisionally,
and only when E050 binds its complete extension evidence in the same call
(r1483 custody: authentic positive origin, complete read-only resources). Groups
without bound evidence are withheld exactly as before; this is decided by
repeating the same writer pass with only the evidenced provisional groups
offered, never by a partial or reconstructed write.

A provisional connection is decided at withdrawal: it stays only if the bound
final assignment places every member in one lane; split or unknown labels
withdraw it. Without a bound late plan no voice changes after the sealed
projection, so a provisional connection whose current members are split is
withdrawn. Loss of provisional custody before this decision, or adopted
provisional custody that reaches no decision, fails the run visibly. Ordinary
(same-lane) admission, the withdrawal rule for ordinary connections, the voice
decision and every global check are unchanged. Known corpus coordinates do not
occur in the selector; independent groups are decided locally.

## r1485: withholding diagnostics only

A withheld split group whose rewrite would leave every owned beam level unchanged
is counted as `unchanged_split_groups` instead of `split_groups`. The admission,
provisional writing and withdrawal decisions are unchanged; only the diagnostic
classification of an already withheld group is refined.
