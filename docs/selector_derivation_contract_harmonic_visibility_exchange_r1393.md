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
CausalFeature: overwriting the pitch of a printed played tone with the pitch of a concealed coincident played tone removes the overwritten tone from every epoch and doubles the other one, so its sounding obligation has no lineage to the final document
CausalFeature: the exchange keeps both played tones as their own note elements with unchanged pitch onset duration staff voice and chord membership and moves only the print state through the existing frozen show (#11) and conceal (#5) owners
CausalFeature: a tone carrying a tie or tied mark is part of a sounding continuation and cannot change its printed state alone, so a tied shown or concealed tone refuses the exchange
NonCausalContextExcluded: piece identity measure label pitch register pitch class interval size voice label value staff number corpus distribution converter outcome and independent candidate count

Parent: accepted r1392. Implementation scope only; no new error class.
The harmonic micro-delta graph, its pairing of a concealed and a printed
coincident chord tone, its ranking, budget and adoption decision are unchanged
except that the materialized node is the visibility exchange instead of the
pitch overwrite, and its node proof checks that only the print state of exactly
these two tones changed. Details: docs/harmonic_visibility_exchange_r1393.md.
