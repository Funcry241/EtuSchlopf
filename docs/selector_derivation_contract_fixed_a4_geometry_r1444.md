# Fixed A4 page geometry at E044

SelectorDerivationContract: causal_class_first_v1
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionMayRefineClassToKnownWitness: no
SelectorChange: yes
SelectorRole: repair_scope
CorpusDistributionPurpose: measurement_only
ParentErrorClassContract: docs/selector_derivation_contract_renderer_page_geometry_unbound.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: the complete document geometry is absent, incomplete or contradictory to the accepted fixed A4 header policy at its existing E044 layout boundary
NonCausalContextExcluded: document identity, pitch, onset, staff, voice, measure label, occurrence count, worker budget, renderer result, project or corpus distribution

This implementation uses only the immutable parent class. E044 inserts the
fixed scaling and page-layout before system/staff layout. Consistent existing
geometry is retained; inconsistent or partial header geometry is refused without
write or receipt. Already-marked complete geometry is an exact no-op.
A fresh header receives the existing owner-conserved Preserving receipt and its
original one-insertion representation journal. No new mint or file sink exists.

Numeric equality uses the parsed values of the frozen decimal constants, no
tolerance. The product is nominal A4, scaling 7/40, four 15 mm margins. Odd/even
native margins of identical values are represented by type=both. Fonts,
appearance and existing system/staff distance decisions are not imported.

The parent does not authorize unrelated code. No change to the initial render
stage, #56, cursor repair, channel choice, phrase policy, layout search, release
or failure plumbing. Existing geometry rejection is owner safety on a single
header transaction, not musical candidate uniqueness.

Acceptance requires all native projects and PDFs, explicit A4 in every consumer
source, unforced -o conversion, schema green, exact semantic pitch/start/duration
preservation and reported voice/phrase/cue differences. The native A4 preview
predicts the same final XMLs after removing only the inserted header fields;
any exception remains a finding requiring disposition. The fresh qualified run
and a repeat via the alternate starter must establish the new byte reference.
