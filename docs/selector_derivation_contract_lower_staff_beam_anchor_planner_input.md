# Existing chord/beam anchor observation (r1504)

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
ParentErrorClassContract: docs/selector_derivation_contract_lower_staff_chord_attack_projection.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: the accepted chord-attachment contract requires self-position anchors to reference the resolved serialized head and forbids a surviving older tie or beam origin from granting an unresolved member a clef-transition anchor
CausalFeature: the existing scanner already carries the head-derived self-anchor, tie-derived anchor, stored per-voice primary-beam origin, masked beam anchor and combined effective anchor; the diagnostic projects these exact facts without changing their computation or replacing missing attachment
CausalFeature: part-local serialized note and measure distances plus intervening real XML cursor/state elements describe the context of those existing anchors; a stored origin can predate the beam begin due to the existing tie policy, so stored origin and effective anchor are reported separately
NonCausalContextExcluded: project identity, etude identity, printed measure labels, absolute byte values, voice/staff label values, pitch, register, rarity, discovery order, corpus frequency, output count and independent multiplicity as class membership or writing authority


This take projects the same existing anchor facts at the actual eligible planner
input, before global normalization and writing. Cross-measure rows are descriptive
context selected by different measure ordinals, not an error class or continuity
threshold. All such rows are retained; a summary counts all observed beam events
and explicitly reports zero contextual rows. Stored and usable anchors remain
separate, including Unknown. Part-local and whole-input fingerprints bind facts
and the actual supplied part solution. Candidate listing calls the unchanged
candidate collector on the unchanged pitches. No alternate solution is computed.
The frozen chord contract and prior final-document diagnostic contract are unchanged.
No diagnostic row or stderr failure grants writing, ranks, vetoes or alters a plan.
Duplication is tested by projecting independent parts with identical facts; only
part labels change. The trace is evidence, not a counterfactual causality proof.
