# r1041 r04 SingleClearStaccato Productive Materialization Authority Retirement Contract

AuthorityRetirementContract: authority_retirement_v1
AuthorityChange: retirement_only
ParentAuthoritySurface: src/pipeline_shards/strict_staff_visible_lane_rest_rewrite_tail.rs::set_pitched_note_duration_type_add_staccato
RetirementScopePaths: src/pipeline_shards/strict_staff_visible_lane_rest_rewrite_tail.rs
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

r1041 r04 is rebuilt independently from accepted r1040. The rejected r1041 candidates remain Failure Evidence only: first candidate SHA-256 4678e6dbbc70acf46369ed35106c4b514780add2d590642da5c489e353513b6d; r02 SHA-256 80e86c76fc5fdf9797dbb612cae048f24758b2caf7f82f6a1c0411f718022ba7; r03 SHA-256 e0457849dca5a0242f3ef714231d0eb246ab92b4f2aa62194af93e35bcfadeea.

MI-A02 remains detectable read-only. `articulation_single_clear_candidate_rows`, `clear_rows.len() == 1`, `clear_rows.first()`, staff/voice/placement guards, target binding, chord-cohort observation, ordering and all existing multiplicity decisions remain byte-identical in the outer SingleClearStaccato path.

The local adapter `set_pitched_note_duration_type_add_staccato` no longer invokes `set_pitched_note_duration_type_add_staccato_authorized` and always returns `None`. The central authorized writer facade and its structural-write-authority implementation remain byte-identical to r1040. The adapter retains only a function-value reference to that facade so Rust/Clippy keeps the already-existing legacy symbol compile-live; there is no invocation expression and therefore no productive call-graph edge from MI-A02 to that writer. This follows the already accepted r1037 retirement pattern.

No replacement writer is introduced. No structural-write-authority source file changes. No per-candidate broadening is introduced. Physical removal of the now-productively-unreachable central writer facade is deliberately deferred to a later separately classified cleanup take. Expected product delta is limited to removal of notation writes formerly materialized through this MI-A02 adapter; pitch/onset/staff/voice selection rules and the score-wide multiplicity gate itself are unchanged.
