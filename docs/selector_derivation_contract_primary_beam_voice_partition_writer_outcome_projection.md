# Selector Derivation Contract - Primary-Beam Voice-Partition Writer-Outcome Projection

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
ParentErrorClassContract: docs/selector_derivation_contract_final_primary_beam_serialization_voice_partition.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: writer-outcome projection may recognize PrimaryBeamVoiceElementsWritten only as typed post-write evidence for a repair already authorized by the frozen final primary-beam serialization-voice partition error class and its independently accepted repair-scope and materialization-safety proofs
CausalFeature: a positive PrimaryBeamVoiceElementsWritten value can satisfy the existing RenderXmlWrite write-cause consistency equation only for the same project result row that already reports the writer outcome; it cannot create, rank, broaden or choose any musical repair candidate
CausalFeature: absence of PrimaryBeamVoiceElementsWritten is backward-compatible zero evidence and malformed evidence remains fail-closed rather than being coerced into writer authority
CausalFeature: independent project rows and independent repair candidates remain multiplicity-invariant because each row is validated locally and no corpuswide count, ordering or uniqueness participates in writer-outcome acceptance
NonCausalContextExcluded: project identity, etude identity, measure number, page number, system number, source-id value, attack-id value, metric-line identity and known witness location
NonCausalContextExcluded: exact corpus count, exact number of written voice elements, discovery order, project order, rarity, first-match status and whether a row corresponds to any historical rejected take
NonCausalContextExcluded: pitch, register, staff number, numeric voice label, beam-group count and any target-discriminating musical context not already authorized by the frozen parent error class and accepted repair-scope evidence

## Governance role

The Rust change in `src/corpus_run_manifest.rs` is selector-sensitive only in the Take-Change-Control syntactic sense: it contains a fail-closed decision over a typed writer-outcome field. This contract therefore records `SelectorChange: yes` as required by the governance gate. It does **not** introduce a new musical selector, repair candidate generator, ranking rule, target locator or writer authority.

The causal error class remains byte-identical in the accepted parent. The new projection can observe only whether a later independently authorized writer reports a nonzero count of already materialized Primary-Beam voice-scalar writes. It cannot infer class membership from that count and cannot use the count to select a repair.

## Multiplicity invariance

Duplicating an independent candidate or adding another independent project row cannot change the validity of an existing row. Each row's `RenderXmlWrite` consistency is checked against its own typed write causes. Global counts are sums for reporting only and never decide whether one local row is eligible, selected or writable.

`CandidateDuplicationProbe: required_before_productive_write` remains binding on the later MusicMutation take; this AuditOnly projection closure performs no productive write itself.
