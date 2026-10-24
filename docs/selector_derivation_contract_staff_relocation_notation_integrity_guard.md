# Selector derivation contract: editorial voice integration safety

Stage B of causal_class_first_v1, r1303, parent accepted r1302. The unchanged
parent error-class contract supplies the voice timeline collision class. Its
run separation and unequal overlapping intervals are necessary to distinguish
legal cross-staff notes and coincident intervals from colliding voice relabels.
No corpus identity or distribution narrows the predicate.

The read-only offer guard consumes AtomicNote run, measure, onset, duration,
voice and pitched/non-chord state. Its source scope exactly matches existing
integration scalar writes (visible, same measure/staff/source voice, pitched,
explicit voice range). Destination occupancy includes hidden sounding notes
and all staffs. It runs only after GuidedSearch, at the integration offer in
run_editorial_voice_decision_pass. rewrite_fullrest, hide, scoring, selection,
adoption and add_safe_atomic_variants are unchanged. No new writer is reached.

Independent candidate duplication cannot change permission: the predicate is
existential over conflicting intervals, so repeating either a source or an
occupancy row leaves the result unchanged. Tests probe source and destination
duplication before productive writes. No singleton or global rank authorizes
an integration. A collision-free offer remains subject to existing selection.
The audit observer only reads transitions; it has no authority in this guard.

```text
SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: repair_scope
ParentErrorClassContract: docs/selector_derivation_contract_staff_relocation_notation_integrity_error_class.md
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: measurement_only
CorpusDistributionMayRefineClassToKnownWitness: no
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: withhold only an EditorialVoiceDecision integration offer when a visible pitched note whose voice scalar would change overlaps a pitched non-chord note of the destination voice in another backup-bounded run of the same measure, unless onset and duration coincide; inspect destination notes on every staff and regardless of visibility
NonCausalContextExcluded: project, etude, absolute measure number, part id, specific pitch values or registers, renderer behaviour, owner run order, candidate profile or rank, warning counts, corpus frequency and WAV-derived values
```
