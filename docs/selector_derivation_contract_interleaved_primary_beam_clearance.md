# Clearance for a horizontal primary beam

SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: repair_scope
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ParentErrorClassContract: docs/selector_derivation_contract_final_stem_error_class.md
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
CausalFeature: complete single-level same-staff beam with equal, single visible heads within the staff and plain explicit stems
CausalFeature: all foreign heads in the closed anchor-onset span lie strictly on the same side, including a nearby head strictly inside the span
CausalFeature: the planned direction puts the beam on that side and no uncertain printed head or visible rest or grace is present in the span
NonCausalContextExcluded: work, file, printed measure, page, octave value, head pitch value, voice number, occurrence identifier, corpus frequency and discovery order

The stem owner first resolves the existing rhythmic and voice-role decisions.
For the qualified flat primary beam it can then establish which side is
available across the beam's onset span. A held attack that began before
the first anchor contributes to sounding voice occupancy. Its head is outside
the beam span. Every visible foreign head inside the span must lie strictly
on the side currently faced by the beam.

The reach-risk interval in staff steps comes from probing the project renderer
with synthetic flat primary groups across the five-line staff. It includes the
beam band and head contour. A matching head is a risk indication. The
one-sided ordering is the reason that moving the beam away clears those
heads. Intervening rests and graces, an equal-height foreign head and heads
on both sides leave the decision open. Ledger anchors, octave geometry,
secondary beams and nonplain stem directions remain outside this clearance
rule. It writes the existing stem scalars for the complete group using the
preserving notation owner. The exact renderer result is checked in the PDF.
