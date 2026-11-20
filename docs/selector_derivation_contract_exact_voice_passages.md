# Exact comparison of local voice assignments

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
ParentErrorClassContract: docs/selector_derivation_contract_beam_connection_voice_separation.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: A closed original primary group binds both ends of the queried connection. The reference Master identifies the split under the actually selected allocation.
CausalFeature: A passage fixes its external context and quantifies over joint assignments under explicit local constraints. A witness identifies the source occurrences whose assignments distinguish the two outcomes.
CausalFeature: Both witnesses establish ambiguity. A forced relationship requires a witness and an exhausted search for a counterexample in the stated domain. Missing bindings and search limits leave the corresponding conclusion open.
NonCausalContextExcluded: Work, measure number, pitch, register, stem direction, particular channel labels, occurrence identifiers and corpus counts.

This observation runs at the allocation owner after the selected plan has bound
its source gaps. Its output is diagnostic evidence. The caller forwards the
observations into the full local report and the console included in the
assistant export.

## Reading a passage

The local question is whether two identified notes can occupy the same
serialization channel. Each vector assigns one channel to each occurrence in
the reported order. The occurrence key records source, attack, fragment and
the bound singleton emission ordinal. The report gives the selected reference
vector and whether it also satisfies the local model.

The window spans the original group. Ordinary pitched notes on its staff can
use existing primary or secondary channels for that staff. Notes crossing the
window and notes pinned at a bar boundary keep their channels. The search
accounts for written and sounding occupation, fixed external sources, written
rests, bound boundary neighbors, hard equalities and channel equality for
already connected original pairs. Chords, ties or incomplete note bindings
can leave the passage open.

The search visits up to 8,192 channel choices for a passage with at most 24
variables. Two witnesses suffice for ambiguity. An empty model or an
unfinished search supplies no proof of necessity. The constraints and fixed
boundary define the scope of every forced or excluded result.

For an originally split target, the diagnostic submits up to eight positive
model witnesses to the existing component owner, stopping at the first
success. That check verifies the concrete allocation, the gain of the queried
Master edge, and the retention of each previously present edge. A companion
outside a sealed beam component needs its own authority. The diagnostic does
not grant it. A refusal describes the submitted plan. The trace records the
number of positive plans observed and how many were checked.
