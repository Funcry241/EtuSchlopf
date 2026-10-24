# Selector Derivation Contract - Upper-Staff Initial Visible Silence Error Class

SelectorDerivationContract: causal_class_first_v1
SelectorChange: no
SelectorRole: error_class_definition
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: not_yet_measured
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: none
ErrorClassDefinitionIndependentOfKnownWitness: yes
ErrorClassName: upper_staff_initial_visible_silence_after_local_cross_staff_context
CausalFeature: one physical measure has a first visible non-chord upper-staff pitched event with positive onset not exceeding one canonical quarter-note and no existing visible upper-staff rest already covering the interval from measure start to that event
CausalFeature: the current metric-rest authority represents the complete interval from measure start to that first upper event as exactly one visible rest duration
CausalFeature: the measure already contains local evidence that the upper entry follows active cross-staff or octave-context material, namely lower-staff pitched material before the upper entry, a same-voice staff switch, or the existing octave-context marker used by the current owner
CausalFeature: the admissible materialization is confined to the same physical measure and inserts only the missing visible upper-staff rest for the already-selected upper voice without changing sounding pitch, sounding onset, sounding duration, note count, or another measure
CausalFeature: candidates in different physical measures have disjoint XML insertion domains and therefore are independent transactions rather than alternatives of one scorewide transaction
NonCausalContextExcluded: project identity, etude identity, concrete measure number, page number, system number, corpus rarity, discovery order, or source byte offset
NonCausalContextExcluded: first_onset equals 24, any numeric score bonus, scorewide ranking position, tie-break position, first or last candidate position, or scorewide candidate count
NonCausalContextExcluded: max_initial_rests, take-one behavior, global winner selection, or any presentation-order preference
NonCausalContextExcluded: exact pitch spelling, pitch contour, register value, or known witness location beyond the existing local cross-staff or octave-context ownership evidence

## Causal definition

An `upper_staff_initial_visible_silence_after_local_cross_staff_context` exists when one physical measure begins with locally evidenced cross-staff or octave-context activity, its first visible upper-staff pitched entry occurs after time zero, and the metric-rest authority proves that the complete preceding upper-voice silence is representable by one visible rest. The missing rest is therefore a measure-local visibility/serialization obligation, not a scorewide competition between measures.

This definition grants no new write authority. r1044 leaves `apply_upper_staff_initial_rest_and_beat_local_coalescing`, its score computation, scorewide sort order, `max_initial_rests=1` call site and structural writer byte-identical. It freezes only the causal class needed for a later multiplicity-invariance change.

## Conflict-group boundary for a later repair scope

The smallest justified conflict group for the initial-rest branch is one physical measure. Current discovery computes one `upper_first` event per measure and can therefore create at most one initial-rest candidate for that measure. Initial-rest candidates in different measures cannot overlap in XML byte ownership because each candidate inserts immediately before that measure's selected first upper event.

A later repair-scope implementation may remove scorewide ranking and the scorewide `max_initial_rests=1` cap as write authority, evaluating each already-eligible measure-local initial-rest candidate independently. It must preserve the existing local eligibility predicates, metric-rest proof, overlap protection against other local rest rewrites, structural-write authority, sounding-signature invariants, parse validity and backup/forward validity. It may not broaden eligibility or alter the separate beat-local coalescing multiplicity surface in the same take.

## Necessity review

1. Positive upper-entry onset is necessary; without preceding silence there is no missing initial rest.
2. Absence of an already covering visible upper rest is necessary; otherwise materialization would duplicate visible silence.
3. The single-rest metric plan is necessary; otherwise the existing metric authority does not justify one inserted visible rest.
4. Local cross-staff, staff-switch or octave-context evidence is necessary because the current owner uses it to distinguish a genuinely active multi-staff context from arbitrary upper-voice delay.
5. Measure-local confinement is necessary to prove candidate independence and prevent scorewide multiplicity from becoming authority.
6. The exact value `first_onset==24`, score bonuses, source offsets and global cap are not causal; changing a second independent measure cannot change whether the first measure has the missing visible silence.

## Corpus boundary

r1044 intentionally does not infer a new corpus distribution from known witnesses and performs no productive selector-sensitive Rust change. The current persisted r1043 result projection does not expose this owner's `upper_visual_initial_rest_*` counters as a stable corpus-wide surface, so r1044 makes no unsupported claim about how many live corpus measures currently satisfy the class. Distribution may be measured later only after this definition is accepted and must not narrow the class.
