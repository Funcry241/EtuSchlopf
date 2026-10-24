# Selector Derivation Contract - Master Identity Foundation and Candidate Lineage Closure r1211

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
CausalFeature: accounting identity must start at the event that creates or transforms a Master state; an identity computed from content can be reproduced for arbitrary bytes and cannot start accounting, so identities are minted once at the owner's execution as opaque process-local values and validation compares bindings only
CausalFeature: a minted identity is legitimate only if its owner minted it; the mint capability is a type whose constructor is private to the owner module and whose binding to a frozen population descriptor (owner, signature, effect and identity contract) exists only in the identity module
CausalFeature: an EXACT receipt exists only when the owner's bound edits, applied in pre-state coordinates, reproduce the post-state byte-exactly and leave every byte outside the bound ranges unchanged; every other execution is represented by an untrusted PendingGap, never by an EXACT claim
CausalFeature: a lineage element is appended only when its pre-state equals the lineage head and its minted identity is not already present; otherwise it is refused, counted as a continuity violation and the stretch is bridged by an explicit PendingGap
CausalFeature: owners whose frozen signature returns only XML hand their minted receipt to the immediate caller through a collection scope on the same thread; a receipt minted without an open scope is counted and attached nowhere
NonCausalContextExcluded: project identity etude identity measure number page system staff voice onset duration pitch register hand label clef spelling known witness location and corpus rarity
NonCausalContextExcluded: candidate names, iteration numbers, profile names, variant labels, report verdicts and corpus counts as identity, ordering or lineage criteria
NonCausalContextExcluded: XML geometry as a source of edit ranges; EXACT ranges come only from the owner's bound edits, never from diffing documents

## Derivation from the accepted parent class

`semantic_obligation_accounting_failure` requires provenance-bound accounting across every transformation boundary. r1206-r1210 transported first identities but derived ids from content and allowed any crate caller to bind them. r1211 replaces that with minted, sealed identities and an ordered lineage whose unmodelled stretches are explicit, untrusted gaps.

## Multiplicity

Every execution mints its own identity. Duplicating an independent candidate or execution creates independent identities; no count, uniqueness or ranking across candidates participates in any decision. Two byte-identical, independently created documents carry different identities.

## Authority boundary

Lineage is provenance only. It changes no music, selector, ranking, target, candidate construction, writer root, StructuralWriteAuthority decision, adoption, Render-to-Best reload, WAV, AccentReserve, Proof or Permit. No receipt, gap or lineage summary is read by any productive decision.
