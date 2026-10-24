# Tied-Tail Accent Attack Ownership Error Class

SelectorDerivationContract: causal_class_first_v1
SelectorChange: no
SelectorRole: error_class_definition
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: not_yet_measured
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: none
ErrorClassDefinitionIndependentOfKnownWitness: yes
CausalFeature: a visible pitched non-chord tied continuation that already carries or is independently planned to carry an accent while exactly one immediately preceding same-staff same-voice same-relational-pitch tie-start attack ends at that continuation onset and the attack itself carries no existing or independently planned accent
NonCausalContextExcluded: scorewide count of such tied-tail occurrences, project identity, measure number, concrete pitch identity, source order, corpus rarity, global uniqueness, global first-next ranking and any known witness address

The defect class is local ownership ambiguity inside one tie relation. Each qualifying tie relation is
self-contained. A second qualifying relation in an independent local scope neither resolves nor creates
the first relation's ambiguity.

Therefore scorewide `candidates.len()==1` is not causal evidence. A later implementation may block only
the matching local candidate (or an explicitly proven shared conflict group) and must prove candidate
duplication invariance.

This take changes no MI-A12 selector, blocker, accent candidate or writer behavior.
