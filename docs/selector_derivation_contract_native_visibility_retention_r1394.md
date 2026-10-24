# Harmonic micro-delta as a visibility exchange: no played tone is lost (r1393)

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
ParentErrorClassContract: docs/selector_derivation_contract_semantic_obligation_accounting_failure.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: concealing a pitched source note through an owner that inserts cursor-only print-spacing removes its native sounding participation even when its physical pitch element survives
CausalFeature: the frozen conceal owner preserves an existing explicit print-spacing attribute, so requiring print-spacing yes on the source note establishes a native-preserving path without changing the owner or preparing substitute XML
CausalFeature: absent or no print-spacing cannot authorize this exchange and must refuse before either original writer runs
NonCausalContextExcluded: piece identity measure label pitch register pitch class interval size voice label staff number source identity value corpus distribution ranking and independent candidate count

Parent: accepted r1393. The error class and original no-played-tone-loss obligation
are unchanged. This repair narrows implementation authority only to the subset
that existing frozen owners can realize without native loss. It introduces no
new normative semantics and does not change owner #5, owner #11 or SSC bindings.
Synthetic negative tests establish refusal without mutation; positive fixtures
originate with explicit safe spacing. Attribute names are parsed structurally,
not selected by physical pitch or corpus rarity. No XML preconditioning or
identity reconstruction is admitted. Native MuseScore and MIDI tests measure
consumer behavior; they do not derive a corpus-dependent selector.
