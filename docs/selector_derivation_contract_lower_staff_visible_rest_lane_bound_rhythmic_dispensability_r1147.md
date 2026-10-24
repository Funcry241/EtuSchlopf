# Selector Derivation Contract - Lower Staff Visible Rest Lane Bound Rhythmic Dispensability r1147

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
ParentErrorClassContract: docs/selector_derivation_contract_visible_rest_elision_rhythmic_obligation.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: an existing lower-staff visible-rest anchor candidate is already selected by the pre-existing local technical-anchor heuristic
CausalFeature: if the same staff/voice lane contains visible pitched material, the complete visible-rest interval must be positively covered by that same lane before whole-carrier elision is authorized
CausalFeature: partial same-lane coverage or same-lane sound outside the concrete rest interval does not prove dispensability of the complete visible rest carrier
CausalFeature: a candidate lane with no same-lane visible pitched material remains governed by the pre-existing technical-anchor selector and existing StructuralWriteAuthority
CausalFeature: the change is conservative preservation only; it creates no rest, reveals no hidden rest and does not change pitched pitch onset duration staff voice chord or tie identity
NonCausalContextExcluded: project identity etude identity measure number page number system number corpus rarity source byte offset or discovery order
NonCausalContextExcluded: exact meter divisions tick values rest duration beat position number of rests number of notes exact staff number numeric voice label pitch register clef or spelling
NonCausalContextExcluded: uniqueness in the current corpus and any feature selected only to retain or remove a known witness

## Derivation

Accepted r1144 already contains the causal error-class contract `visible_rest_elision_without_bound_rhythmic_dispensability_proof`. The lower-staff anchor suppression owner can replace an already visible rest carrier by cursor-only time. Its existing selector uses local anchor heuristics, repeated-rest evidence and cross-voice/cross-staff context.

Those observations do not by themselves prove that the concrete rest interval is rhythmically dispensable when the same final staff/voice lane also carries visible pitched material. For a whole-carrier rest-to-forward rewrite, complete interval coverage is therefore the minimum positive sounding redundancy proof available to this owner. A gap in that coverage means the existing visible rest must be preserved.

This repair does not broaden the positive technical-anchor branch for lanes with no same-lane visible pitched material. It only removes deletion authority when a sounding lane exists but fails to cover the complete rest interval.

## Authority and invariants

The existing selector and the existing `StructuralWriteAuthority::apply_lower_visible_rest_suppression_authorized` materialization path remain the only owner/write path. No writer root, sink, checkpoint, repair authority or release authority is added.

The change affects only whether an already selected local candidate is allowed to reach that existing authority. Pitched semantic timeline, raw cursor balance, parseability and later render readbacks remain under their existing fail-closed gates.

## Multiplicity and normative tests

The additional proof is local to one candidate interval and one staff/voice lane. Duplicating or relocating an independent candidate cannot change another candidate's coverage proof.

Normative tests exercise the pure interval-bound decision directly and contain no score identity, project identity or concrete measure witness. They prove that partial/outside same-lane sounding does not authorize complete rest elision, while complete same-lane coverage and the pre-existing no-same-lane technical branch retain their prior authority.
