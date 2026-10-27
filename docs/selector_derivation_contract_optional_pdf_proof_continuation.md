# Optional PDF proof continuation: repair scope

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
ParentErrorClassContract: docs/selector_derivation_contract_optional_pdf_proof_lifecycle.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: PDF proof creation is already bound to pdf_render_requested; the existing release continuation must consume that same request and eligibility context rather than XML source selection alone
CausalFeature: no-PDF and blocked-before-renderer runs have no PDF commit to continue; eligible requested PDF rendering still requires the original committed source and voice plan without fallback
NonCausalContextExcluded: project, filename, corpus, measure, pitch, staff, voice, note count, duration, density, ranking, process order and runtime

## Owner and exact scope

Requires native acceptance of r1531 and its unchanged parent error-class
contract. Local comparison against the prepared r1531 tree proves only
compatibility with that proposed parent, not its native acceptance.

Only the existing E007 PDF-commit continuation condition inside
run_render_pdf_source_selection_stage is narrowed to:

    pdf_render_requested && render_xml_selected_for_pdf

The request flag is computed as final_blockers == 0 && !no_pdf in the existing
Metric/FinalNotation owner and already transported explicitly to this stage.
The commit-creation condition, source selection, source label, Master/Render
sovereignty, layout adoption, final snapshot checks, persisted product schema
validation, report, renderer invocation and final exit gates are unchanged.
No alternate Master selection, commit creation, new helper, new writer or
new productive call edge is introduced. Both original missing-commit and
missing-voice-plan errors remain strict in the eligible requested PDF path.

The code applies independently to each local run. Adding or renaming another
project does not change the request, selected source or decision for this run.
There is no global ranking, cardinality or first-winner decision.

## Evidence and limitations

The exact runtime Rust bytes were locally built and passed 2564 tests with
0 failures and 1 ignored, plus strict productive Clippy. Real process probes
cover Master and Render selection with --no-pdf, including repetitions:
Exit 0, two mandatory XML products, truthful not_requested, preserved source
selection, unchanged inputs and stable repeated XML bytes. The accepted r1530
fails the same Render/no-PDF counterexample at the E007 commitment requirement.

The requested PDF probe retains the committed-source continuation but ends
with renderer export failure in this local environment. No positive PDF or
Windows result is claimed. A dedicated blocked-before-renderer process probe
is not supplied. Existing independent blocker and product gates are unchanged.
Native acceptance and the native ten-piece comparison remain pending.
