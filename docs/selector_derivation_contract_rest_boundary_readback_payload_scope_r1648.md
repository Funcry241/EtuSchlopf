# Exact rest readback XML on demand

SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: error_classification
ParentErrorClassContract: docs/selector_derivation_contract_semantic_obligation_accounting_failure.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
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
CausalFeature: As in the existing actual rest-boundary readback, a transition can be investigated only from its actually observed operands. All observations, document/measure fingerprints, exact ranges and occurrence references remain in the report regardless of the display mode.
CausalFeature: The already parsed writer-trace request controls rendering of intermediate XML bytes only. The same measure normalization scopes the displayed payload. Byte-identical content is still compared by its full bytes.
NonCausalContextExcluded: project and measure identity as evidence of defect membership, staff, voice, onset, duration, pitch, register, rarity, and CLI detail as productive authority.

The selector designation describes a read-only diagnostic projection. The
parent error class remains unchanged. The display filter does not narrow the
physical observation or its complete occurrence accounting.

The normal report preserves the checkpoint observations, document and measure
hashes, byte counts, and exact occurrence references. It records explicitly that
the intermediate measure XML payload was omitted.

The existing `--writer-trace` / `--writer-trace=all` request includes all
captured measure XML. `--writer-trace=5,7` includes the exact captured payload
for those measure labels, under the same normalization as the writer trace.
Byte-identical measure references keep their original identifiers. XML line
framing and byte-exact recovery remain unchanged for emitted payloads.

This concerns the existing read-only report producer. Native acceptance against
r1647 checks all 20 Master and Render MusicXML files, keeps the rest readback
summary and reference records, and measures the report and bundle sizes.
