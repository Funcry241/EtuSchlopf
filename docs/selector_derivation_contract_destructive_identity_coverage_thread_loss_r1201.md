# Selector Derivation Contract - Destructive Identity Coverage and Thread-Locality Loss r1201

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
ParentErrorClassContract: docs/selector_derivation_contract_semantic_obligation_accounting_failure.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: the accepted parent class requires accounting identity to be continued across every transformation boundary; a destructive transformation surface whose typed entry never mints an IndividualEditIdentity cannot start such accounting, so identity coverage is decided solely by whether the entry's owner closure calls the opaque identity constructor
CausalFeature: a whole-file persisting sink transports complete documents rather than individual edits, so identity coverage is not applicable to it; a surface that is not productively reachable is retired regardless of minting
CausalFeature: an accounting observation issued on a thread whose ledger is inactive is lost without error; whether a disposition depends on this loss is decided only by comparing the unchanged owner-thread ledger with a process-wide mirror that applies the identical transition rules to all observations, edit identity by edit identity
CausalFeature: runtime mints are associated with static minting callsites only by exact source path and line of the track_caller location; an unmatched origin remains unmatched and is counted rather than guessed
CausalFeature: nominal-gate rules are syntactic properties of source text fixed before measurement: literal integer coordinate tables compared with a measure or scope binding, measure bindings initialised with or compared to literal integers greater than one, check-outcome fields only ever assigned literal true and read in a logical condition, numeric gate fields only ever assigned literal zero and compared with zero, and bool gate parameters every caller passes as literal true
CausalFeature: association of a flagged gate with identity-bearing edits is region-level through the flagged function, its direct callers and its direct callees; it never proves that the flagged construct decided an edit and never authorizes removal, repair or writing
NonCausalContextExcluded: project identity etude identity measure number page system staff voice onset duration pitch register hand label clef spelling known witness location and corpus rarity
NonCausalContextExcluded: expected counts of lost dispositions, expected share of NO_SELECTION_WITNESS explained by thread locality, known r1200 worker-thread callsites and any feature chosen to reproduce a predetermined corpus result
NonCausalContextExcluded: carrier counts, sounding signatures, report labels, verdict strings and effect-family names as coverage or gate classifiers; only owner-closure minting, reachability, surface kind, exact callsite location and the fixed syntactic gate rules decide

## Derivation from the accepted parent class

`semantic_obligation_accounting_failure` makes provenance-bound accounting mandatory across every boundary and treats missing correspondence as missing accounting. r1188–r1200 built that accounting for the caller-supplied replacement family. r1201 measures, without changing any product behaviour, where that accounting cannot start (surfaces without identity) and where it is lost by mechanism rather than by musical process (thread-local ledger state).

## Multiplicity

Duplicating an independent surface, callsite or flagged gate adds its own row and cannot change the class of any other row. No scorewide count, uniqueness or ranking participates in any classification.

## Authority boundary

Classification is AuditOnly measurement. It changes no music, selector, candidate construction, writer family, StructuralWriteAuthority surface, repair scope, threshold, MasterApply, Proof, Permit, adoption boundary or release authority. No flagged gate is removed or altered by this take.
