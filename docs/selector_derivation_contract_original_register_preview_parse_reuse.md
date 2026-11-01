# Original register preview: reuse one immutable source parse

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
ParentErrorClassContract: docs/selector_derivation_contract_joint_register_reading_context.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
NonCausalContextExcluded: corpus identity, printed labels, pitch identity as selector, rarity, discovery order and elapsed time

CausalFeature: every preview in one original-input comparison reads the exact same unchanged XML source
CausalFeature: original document-order and measure ranges are observations of that source, independent of the candidate proposal

This performance-only take is based on accepted r1573. The comparison still
visits every original candidate in the same order, with unchanged selection,
complete-course costs, UNKNOWN behavior, topology, protection and writers.
No candidate result is cached. No old C3 integration is added.

A private context borrows the exact original input for one comparison. It lazily
stores its XML parse, including the existing document-order parse error. The
callback's two repeated original-source parses become one lazy parse over the
entire comparison. A comparison that never reaches the callback still does not
parse. The context is dropped before productive source materialization.

The existing direction-generation wrapper retains its original parse/error
behavior. Its extracted helper consumes the same source and document. Candidate
directions, prepared MusicXML, carried lower-clef state and the complete solver
still execute for every proposal. Candidate-dependent data is never reused.

Differential tests retain the parent's complete preview as a test-only reference
and compare full cost maps and errors across empty, shifted, continued and
missing-part proposals, relocated labels, initial G/F clefs, unsupported clocks,
repeated/reversed proposal visits and invalid XML. They require a nonempty valid
baseline so equivalence is not established solely on UNKNOWN results.

Native acceptance must verify byte equality of all twenty MusicXML files against
r1573. A total runtime improvement has not been measured. Candidate-output XML
parsing and the larger repeated solver cost remain; no corpus speedup threshold
is claimed. The pending C3 Shadow/Live repair and export changes are separate.
