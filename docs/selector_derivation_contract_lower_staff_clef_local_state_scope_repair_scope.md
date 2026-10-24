# Selector Derivation Contract - Lower-Staff Clef Local State Scope Repair Scope

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
ParentErrorClassContract: docs/selector_derivation_contract_lower_staff_clef_original_source_envelope_r1252.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: a lower-staff F clef relocation is evaluated only inside its own preceding local state scope, bounded by the measure start or the end of the last attributes, backup or forward element before that clef
CausalFeature: the insertion target is the first visible lower-staff event of that local scope, so the relocation envelope never contains an attributes, clef, backup, forward or measure boundary and touching envelopes of different clefs stay independent
CausalFeature: the low/rest and treble-register evidence for a relocation is read from the same local scope whose effective clef the relocation changes, never from events of another clef or cursor domain
CausalFeature: the planned batch is realized only by the accepted V2 original-source construction; an authorization reject leaves the source byte-identical and is transported as a failed #57 owner execution without receipt, planned post, sink write or adoption
NonCausalContextExcluded: project identity, etude identity, concrete measure number, source ordinal, byte offset value, discovery order and corpus frequency
NonCausalContextExcluded: scorewide candidate count, global first/last ranking, pitch spelling and any known witness location

## Repair-scope derivation

The accepted r1252 parent defines the error class `source_bound_clef_moves_consumed_outside_their_original_independent_state_envelope` without selector-sensitive code. Its causal features are stale-coordinate consumption, overlapping envelopes and relocation across attributes, clef, temporal-cursor or measure boundaries. This implementation narrows the existing #57 planner only for authority and safety: the former target, the first lower-staff event of the whole measure, is replaced by the first lower-staff event after the last state or cursor boundary that precedes the clef. The existing musical decision (preceding low/rest evidence and no treble-register evidence) is unchanged in kind and is evaluated over exactly the interval whose effective clef changes.

Each clef forms its own local candidate. Because every clef block is itself an attributes boundary, two candidates of the same measure have disjoint or touching envelopes and never share an insertion target. No candidate count, ranking or measure-wide multiplicity grants or vetoes a write. Duplicating an independent measure preserves the first measure's planned bytes.

The writer region is replaced exactly by the accepted V2 fixture region through the bound whole-file transition. V2 rejects crossings and overlaps at issue and at writer consumption, builds the candidate from original segments and commits once through a single scanner-visible `xml.replace_range(.., &candidate)`.

Authorization failure is no longer a panic. The planner returns the rejected batch as an error, the #57 trial propagates it before any planned post or receipt exists, and RenderSovereignState treats it as a failed owner execution (no sink write, no adoption, closure failure recorded); the initial-render stage propagates the error.

## Executable evidence

Synthetic, relocatable through-tests cover the actual planner, the V2 writer, the #57 trial, its file-bound BLOCK receipt, the path-bound RenderSovereignState adoption and the verified Render artifact. Cases: single relocation, independent duplicated measures (candidate-duplication probe), touching envelopes, shared-target, nested and duplicate-source batches, F-G-F, backup, forward and attributes boundaries, multibyte text inside and before the envelope, and the opening clef. An independent roxmltree-based oracle accepts only intended local advances over low/rest/hidden lower-staff events, protects foreign event bytes and order, attribute order and cursor domain, and the end-of-measure clef carry; it rejects parse-valid, sound-stable scope violations.

This contract grants no SSC proof, population status, OPEN removal, READY or enforcement authority. Real corpus results are measurement only.

## Released-artifact through-test and limits

An additional isolated process exercises an intended advance over a preceding low pitched event, real #57 trial and bound BLOCK receipt, sovereign adoption, bound canonical/voice/occurrence plans, the actual Live finalize_pdf_source_before_renderer_authorized owner, final Master lineage recording and close_render_sovereignty. The independent clef oracle checks the adopted advance and verifies pitched-event clef states plus end-of-measure carry through the PDF-source commit. All logical stage markers are explicit gate setup; this is not a full orchestrator or external renderer test. Separate-process isolation prevents global closure counters from interacting with the parallel test suite. The readback helper admitted by r1254 remains byte-identical and the existing single-scope regression is retained.

The preceding-rest case remains tested through trial, receipt, adoption and witnessed artifact. A separate exploratory PDF through-path showed the existing FinalNotation/PDF bundle places that F clef after the preceding rest, while preserving pitched-event clefs and measure carry. This downstream rest-key placement is not changed here, and final placement before that rest is not claimed. The mixed Staff-1/Staff-2 collector issue noted during review remains outside this error-class repair. Authorization rejection is directly tested at writer/adoption level; the planner-to-trial Err propagation is source-checked, without a new runtime injection point. No general Deny coverage, population closure or enforcement readiness is claimed.
