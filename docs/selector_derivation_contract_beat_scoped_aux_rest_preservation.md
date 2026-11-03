# Preserve visible rests at the beat-scoped auxiliary elision owner

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
ParentErrorClassContract: docs/selector_derivation_contract_visible_rest_elision_rhythmic_obligation.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: the beat-scoped rest owner selects an existing visible rest for elision using inserted staff-local rests or a refused cross-staff completion together with raw auxiliary channel observations
CausalFeature: those observations do not supply a complete interval-bound proof that this concrete rest's rhythmic obligation is dispensable
NonCausalContextExcluded: project, etude, measure, page, pitch, register, staff/channel label values, exact durations, candidate count and discovery order

The unchanged accepted parent requires preservation of an existing visible
carrier when its dispensability is unproven. The existing beat-scoped auxiliary
selector and its physical candidate footprints are retained. The sole helper
`beat_scoped_mark_note_hidden`, called only by this selector, now returns the
original note bytes. The original exact pre/post receipt comparison stays in
place: equality produces None and issues no receipt. The helper's historical
name and mint descriptor remain for compatibility; they authorize no new edit.

The actual note map retains the original physical occurrence and the hidden
counter stays zero because no candidate was hidden. Other metric rest splits,
rest creation, cursor guards, source custody, canonical role owners and their
positive technical-rest decisions remain unchanged. No already hidden carrier
is made visible, no new negative-rest context is issued and no arbitrary gap
becomes a rest obligation.

The existing actual-owner synthetic regressions now expect preservation of the
one visible rest while its hidden neighbors remain hidden. It also exercises
all-visible independent carriers, renamed channels, swapped staffs, different
measure labels, independent repetition and idempotence. Exact XML bytes, the
physical original note map, pitched timeline and cursor remain checked. The
original owner-capture regression additionally requires zero receipts and zero
birth captures for the exact no-op, while stale requested states stay refused.

The ordinary contiguous beam selector and the independently guarded E050
visible-rest bridge selector are byte-unchanged. Preserving existing pauses may
make their original prerequisites available later, but creates neither a
connection permit nor original group evidence. Missing canonical beam custody
or a separating final assignment retains every existing refusal.
