# Selector Derivation Contract - Tied-Tail Accent Attack Ownership Local Blocker

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
ParentErrorClassContract: docs/selector_derivation_contract_tied_tail_accent_attack_ownership.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: each blocked accent row independently matches the accepted tied-tail attack-ownership class: a visible pitched non-chord tied continuation with an existing or independently planned accent, exactly one immediately preceding same-staff same-voice same-relational-pitch tie-start attack ending at the continuation onset, and no existing or independently planned accent on that attack
CausalFeature: the blocker is local to its AccentAttackOwnershipKey; duplicating an independent qualifying tie relation adds only its own blocker and cannot remove, redirect or globally rank the first blocker
NonCausalContextExcluded: scorewide tied-tail candidate count, project identity, etude identity, concrete measure identity as target selector, concrete pitch identity, corpus rarity, discovery order and global first-next ranking
NonCausalContextExcluded: historical candidates.len()==1, any one-blocker global budget, and any condition making one independent candidate depend on another independent candidate

The accepted r1084 parent freezes the error class before this selector-sensitive implementation.

`confusing_tied_tail_accent_keys` already enumerates local class members. r1085 removes the historical
scorewide uniqueness wrapper from productive authority and filters both accent writer paths with all locally
matching ownership keys.

The three `raw_selected.len() - selected.len() <= 1` checks are removed because they encode the same
historical one-blocker assumption. This cannot add a parent accent candidate: raw selection is unchanged,
and r1085 only removes rows that independently match the accepted local safety class.

Candidate-duplication tests prove that duplicating the error-class witness yields two detector candidates and
that adding an independent second blocker neither unblocks nor redirects the first local decision.

The committed readback now counts all remaining tied-tail accent candidates and requires zero.


## r1087 callable-surface closure

The parent pipeline shard already exposed
`unique_confusing_tied_tail_accent_regression_candidates`. r1087 deliberately retains that callable
identity while changing its return semantics to the full count of local error-class members. The legacy
name is compatibility-only and carries no uniqueness authority.

The two historical tests that encoded "one witness => 1, duplicated witness => 0" are removed completely.
No replacement test functions are introduced in that legacy shard. Candidate-duplication invariance is
instead proven by the local blocker test in the existing articulation owner, which does not add a new
pipeline repair/write callable surface.
