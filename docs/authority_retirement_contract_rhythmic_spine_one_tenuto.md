# r1037 r04 RhythmicSpineOneTenuto Productive Reachability Retirement Contract

AuthorityRetirementContract: authority_retirement_v1
AuthorityChange: retirement_only
ParentAuthoritySurface: src/pipeline_shards/rhythmic_spine_one_tenuto_apply.rs::add_single_tenuto_to_note_fragment
RetirementScopePaths: src/pipeline_shards/rhythmic_spine_one_tenuto_apply.rs
CandidateClassChange: forbidden
SelectorRelationChange: forbidden
TargetSelectionChange: forbidden
MultiplicityDecisionChange: forbidden
NewPositiveWriteDecision: forbidden
ReplacementWriter: forbidden
WriterCapabilityDirection: decrease_only
CalibrationThresholdChange: forbidden
ProductDeltaScope: removal_of_parent_writes_only
CausalClassFirstBypass: forbidden
CandidateDuplicationBypass: forbidden

## Bound retirement

r1037 r04 is rebuilt independently from accepted r1036. The rejected r02/r03 candidates are Failure Evidence only.

MI-A01 remains detectable read-only. Lane candidate construction, score-wide ranking, `candidates.into_iter().next()`, Hinge target selection, measure-order tie-break and the existing `max_size` threshold remain unchanged from r1036. The local `add_single_tenuto_to_note_fragment` adapter no longer calls the central SingleTenuto writer and always returns `None`; therefore the productive call-graph edge from this legacy surface to the SingleTenuto writer disappears and no Tenuto can be committed by this path.

The existing central `add_single_tenuto_to_note_fragment_authorized` facade is deliberately kept byte-identical in r1037. The local adapter retains only a function-value reference to that facade so Rust/Clippy still sees the legacy symbol as referenced, while the semantic call graph sees no invocation edge. Physical removal of that now-unreachable legacy facade is intentionally deferred to a later small cleanup take and is not part of this MusicMutation.

No replacement writer is introduced. No structural-write-authority source file changes. Expected product delta: Master-MusicXML, Pitch, Onset, Duration, Staff, Voice and sounding timeline remain unchanged; Render/PDF notation loses only Tenuto marks formerly introduced through this productive RhythmicSpine call edge.
