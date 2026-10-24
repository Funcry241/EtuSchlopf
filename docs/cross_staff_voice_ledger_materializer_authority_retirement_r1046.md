# r1046 r04 CrossStaffVoiceLedgerMaterializer Authority Retirement

Parent: r1045 `UpperInitialRestMeasureLocalMultiplicityClosure`

Surface: MI-A06 `CrossStaffVoiceLedgerMaterializer`

Change class: `MusicMutation` with `authority_retirement_v1`

## Why r04 differs from the rejected candidates

The first r1046 candidate removed internal authorized writer calls but left `apply_cross_staff_voice_ledger_materializer` itself classified as a productive writer root. The Authority-Retirement Kodex therefore found no pure monotone productive authority loss and rejected SHA-256 `0bc8103495f361eb07fa2158dc9a65ac4d0c87e8cc1eceabb165e625c054e43d`.

r02 then transformed the MI-A06 root into a trampoline/read-only observer and detached it. The Semantic-Capability-Diff correctly rejected SHA-256 `28539d6cc0343d430e7aa11ad410c263dd07ee2bb31744c934cd1c3e35929650`, because a detached Music writer root was changed at the same time.

r03 then applied the correct semantic cut, but its package accidentally carried an undeclared empty `README.md`; Take-Change-Control rejected SHA-256 `9327eb2bdf9bd9794b3affc803e1f9e244ae1ffd19d4a84d314b956636b142a2`. r04 is rebuilt from pristine r1045, preserves the r03 semantic cut, and keeps `README.md` byte-identical to the parent.

r04 follows the gate literally. It is rebuilt from r1045. The complete MI-A06 implementation file `strict_staff_visible_lane_rest_rewrite_tail.rs` remains byte-identical. Only the productive caller in `human_pdf_pianistic_quality_core.rs` stops invoking the MI-A06 function. A function-value keepalive keeps the legacy symbol compile-live but is not an invocation expression.

## Authority result

Before r04:

`build_behind_bars_rule_check_rest_and_tie_audit -> apply_cross_staff_voice_ledger_materializer -> existing CrossStaffVoiceLedger writer family`

After r04:

`build_behind_bars_rule_check_rest_and_tie_audit -X-> apply_cross_staff_voice_ledger_materializer`

The detached root is byte-identical to r1045. The surrounding owner still invokes all its other pre-existing writer surfaces unchanged. Therefore WriterReachability can only narrow; no writer root is added, moved or rewritten.

## Multiplicity and Beam status

MI-A06 is closed by authority retirement rather than by inventing a new local multiplicity selector. Its legacy candidate/ranking implementation, including `max_actions=1`, remains physically present but productively unreachable from this owner. No replacement selector and no new conflict group are introduced.

Historical A baseline remains 12. After acceptance of r1046: 5 closed, 7 open, 41.7 percent. The Beam parent measurement remains 147 Same-Line, 144 Cross-Voice, 90 occurrence-identical and 54 unresolved. r1046 claims no Beam-Occurrence reduction.

## Validation target

No test symbol or test topology is changed, so the expected strict count remains 1352. Acceptance still requires the normal Windows `--take --all` path with Build, Clippy, strict tests, complete 10-project corpus and renderer evidence.
