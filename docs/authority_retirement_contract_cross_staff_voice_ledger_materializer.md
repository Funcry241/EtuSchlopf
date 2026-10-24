# r1046 r04 CrossStaffVoiceLedgerMaterializer Productive Authority Retirement Contract

AuthorityRetirementContract: authority_retirement_v1
AuthorityChange: retirement_only
ParentAuthoritySurface: src/pipeline_shards/human_pdf_pianistic_quality_core.rs::build_behind_bars_rule_check_rest_and_tie_audit
RetirementScopePaths: src/pipeline_shards/human_pdf_pianistic_quality_core.rs
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

r1046 r04 is rebuilt independently from accepted r1045 `UpperInitialRestMeasureLocalMultiplicityClosure`. The three rejected r1046 candidates remain Failure Evidence only:

- first candidate SHA-256 `0bc8103495f361eb07fa2158dc9a65ac4d0c87e8cc1eceabb165e625c054e43d`: retirement contract declared without a pure monotone productive writer-authority loss;
- r02 SHA-256 `28539d6cc0343d430e7aa11ad410c263dd07ee2bb31744c934cd1c3e35929650`: the intended writer root was detached but its function body was changed in the same MusicMutation, which the Semantic-Capability-Diff correctly requires to be a separate capability change;
- r03 SHA-256 `9327eb2bdf9bd9794b3affc803e1f9e244ae1ffd19d4a84d314b956636b142a2`: the semantic cut was correct, but packaging changed `README.md` without declaring it. r03 is rejected packaging evidence and is not a parent.

r04 preserves the r03 semantic cut but is rebuilt from the pristine accepted r1045 snapshot, so `README.md` is byte-identical to r1045 and not part of the diff. r04 therefore does not modify `src/pipeline_shards/strict_staff_visible_lane_rest_rewrite_tail.rs` at all. In particular, `apply_cross_staff_voice_ledger_materializer`, its candidate construction, score-wide ordering, `max_actions`, `.take(max_actions)`, structural metric beam planning, local note-beam adapter and both existing authorized writer calls remain byte-identical to accepted r1045.

The sole Rust source change is in the already-existing productive owner `build_behind_bars_rule_check_rest_and_tie_audit`. Its invocation of `apply_cross_staff_voice_ledger_materializer(&legacy_candidate_xml, 1)` is removed. A non-invoking function-value keepalive preserves the legacy private symbol for compile/Clippy purposes without creating a semantic call-graph edge. No replacement writer, observer, selector or target relation is introduced.

Consequently the owner can only lose the MI-A06 writer root from its existing WriterReachability set. All other BehindBars writer calls remain unchanged. The detached MI-A06 root itself is unchanged. This is the exact monotone authority loss required by `authority_retirement_v1`.

MI-A06 candidate-count/attempt counters in this legacy owner are no longer populated because the retired surface is no longer invoked. This is an observational consequence of removing the productive invocation, not a new candidate class or selector. Physical cleanup of the now-unreachable MI-A06 implementation and its authorized facades is deliberately deferred to a later separately classified transition.

No Beam Occurrence identity is retrofitted and no reduction of the current 54 unresolved Beam occurrences is claimed.
