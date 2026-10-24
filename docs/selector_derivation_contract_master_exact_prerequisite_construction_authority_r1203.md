# Selector Derivation Contract - Master EXACT Prerequisite: Construction and Authority Closure r1203

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
ParentErrorClassContract: docs/selector_derivation_contract_semantic_obligation_accounting_failure.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: an EXACT edit script is only meaningful if every edit range is interpreted against the byte state it was bound to; a capability that binds several ranges against one source and then applies them in an order where an earlier edit can change the byte length before a later range reapplies stale offsets, independent of which note, lane value or etude is involved
CausalFeature: applying the bound edits in descending start order is the construction that keeps every range valid; overlapping bound ranges cannot be ordered safely and are refused
CausalFeature: the readback after application checks only structural properties of the edited note itself (one exact note block ending at the arithmetically shifted note end, lane values equal to the bound replacement values); it never consults position, measure, pitch, project or any corpus property
CausalFeature: a productive destructive write that bypasses StructuralWriteAuthority cannot mint an IndividualEditIdentity and cannot be accounted; the single-note restify patch is therefore routed through the already existing restification batch authority, which binds exact source bytes, checks the exact-note to exact-rest-note contract (duration, voice and staff preserved) and consumes once
CausalFeature: when the authority refuses, the patch variant stays the unchanged input (no write), which is the same refusal behaviour as the already authorized multi-note restify path of the same owner
NonCausalContextExcluded: project identity etude identity measure number page system staff number voice number onset duration pitch register hand label clef spelling known witness location and corpus rarity
NonCausalContextExcluded: expected corpus byte deltas; the corpus is used only to confirm that no Master, Render or PDF byte changes (measurement_only)
NonCausalContextExcluded: candidate names, variant labels, report verdict strings and issue types as authority or ordering criteria

## Derivation from the accepted parent class

`semantic_obligation_accounting_failure` requires provenance-bound accounting across every transformation boundary. Population v1 (r1202 basis) lists two Master rows whose own construction prevents such accounting before any identity architecture can be laid over them:

- row 4 `apply_bound_staff_voice_move_authorized`: stale offsets when the voice value changes its byte length (construction defect);
- row 63 `apply_restify_patch`: raw splice without StructuralWriteAuthority (authority defect, no identity mint).

r1203 repairs only these two properties. It does not change which patch is proposed, which variant is chosen, or any selector, ranking or adoption rule.

## Multiplicity

Each capability binds and applies its own edits. Duplicating an independent patch creates an independent capability; no count, uniqueness or ranking across patches participates.

## Authority boundary

No new writer root, no new public function, no new writer family. The restify patch delegates to the existing canonical `apply_atomic_restification_batch_authorized`; the staff/voice move stays in its existing capability. Render rows 57 and 67 are explicitly out of scope.
