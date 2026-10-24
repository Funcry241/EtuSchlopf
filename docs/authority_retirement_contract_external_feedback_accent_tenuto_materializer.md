# ExternalFeedback Accent-Tenuto Materializer Authority Retirement Contract

AuthorityRetirementContract: authority_retirement_v1
AuthorityChange: retirement_only
ParentAuthoritySurface: src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::materialize
RetirementScopePaths: src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs, src/structural_write_authority/mod.rs, src/structural_write_authority/render_measure_articulation_authority.rs
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

r1081 is rebuilt independently from canonical accepted r1077 `RejectionLearningEvidenceBoundaryClosure`.
Rejected r1078, r1079 and r1080 remain failure evidence only and are not parents.

r1078 cut the authority at a direct MusicXML mutation entrypoint and was rejected before installation because
the semantic-capability delta was not a pure monotone writer-authority loss.

r1079 moved the cut below the selector into the private source-bound materializer. That shape passed the
full pre-take authority-retirement gate, installed transactionally, and then failed during post-apply reexec.

r1080 kept that production cut and updated the two directly stale MI-A11 tests. It again passed the full
pre-take authority-retirement gate and installed transactionally, but the post-apply reexec still returned
exit 101. The failure export again contains only startup/hygiene output and not the compiler/test stderr.

Static source reconstruction identifies the remaining direct consequence of the admitted materializer
retirement: `structural_write_authority::replace_bound_note_articulation_authorized` has no surviving caller.
Because the root crate enforces warnings/dead-code fail-closed, r1081 closes only that causally orphaned
writer island:
- remove the now-unreferenced re-export;
- remove `replace_bound_note_articulation_authorized`;
- remove the now-unreachable private `RenderEditEffect::NoteArticulation` variant and fold its identical
  validation arm back to the still-live `ArticulationBatch` case.

The r1080 production retirement and test closure are otherwise unchanged. MI-A11 candidate discovery,
scorewide `candidates.len() != 1`, target selection, source fingerprint binding, thresholds, outer
productive owner and MI-A12 remain unchanged from accepted r1077. No replacement writer is introduced.
