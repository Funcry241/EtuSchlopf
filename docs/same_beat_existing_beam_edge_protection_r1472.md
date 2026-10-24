# r1472: preserve existing beam connections within one beat

## Required parent and authorization

Parent: r1471 SameBeatPartialBeamOverwriteCausalDefinition. Local preparation may
start before its native acceptance; installation is conditional on that actual
accepted parent. The frozen error-class contract is byte-unchanged.
Wolfgang authorized the reviewed music effect on 2026-10-22 and asked Maus to
build the next take while the r1471 native run is in progress.

Class: GovernedChange. Music effect declared: beam-only changes in Render;
Master, notes, rests, slurs, ties, staff and voice content must remain unchanged.
Five visible corrections and roughly 160 secondary-beam changes toward Master
are hypotheses, not acceptance results. Any extra musical change needs review.

## Product behavior

The existing writer still selects its original first eligible voice and requires
both the original first three-sixteenth group and its disjoint final pair.
After that unchanged selection, each group is independently withheld when a
claimed input primary edge crosses its boundary inside the same metric beat.
Outgoing begin/continue OR incoming continue/end is sufficient. Rests inside
the beat do not erase a claim. An unreadable boundary refuses that group
conservatively; it is not reported as proven positive connectivity. Chord members and hidden notes keep the existing
selection treatment; chord XML is not rewritten. A protected first voice does
not fall through to another voice. Cross-beat separation remains the old policy.
The original sealed candidate, source binding, receipt, materializer and sink
remain the sole actual write course.

An exactly matching group remains eligible for the existing normalization.
Known open case O1: that normalization can still discard a higher beam level
when the complete proposed group matches the original primary group. This take
does not claim to fix it. The two unresolved cross-voice beam cases are separate.

## Optional immediate evidence and actual writes

Native command: `.\run.cmd --take --all --writer-trace`.
The existing CLI trace flag is forwarded to corpus children. Optional changed
measure snapshots now retain complete XML instead of truncating at 14,000 bytes.
No new public artifact or musical control flag is introduced.

For every original selected two-group proposal, MetricProtectionObservation
contains the output path and input measure offset, exact immediate input
measure, original member byte offsets,
protection decisions and the original two-group plan's all-level edits. Its
semantics are explicit: strip all prior beam elements on those proposal notes,
then insert the listed levels. NOT_WRITTEN denotes an unwritten counterfactual,
never a physical writer or adopted transition. All levels absent from that old
plan would have been removed by its existing strip operation.

Existing optional WriterTrace snapshots separately show successful persisted
file writes before/after exact readback. Those can include intermediate/trial
paths and are not automatically final adoption. Existing sovereign epoch/
lineage evidence and final exported XML remain separate observations. No trace
row is manufactured for a prevented physical write. The default automatic
primary-only trace remains unchanged and cannot establish secondary writes.

## Tests and preflight

Synthetic tests cover the reviewed P1-P10 and G1-G5 cases, one-sided input
claims, labels and relocation, independent measure duplication, first-voice
selection preservation, chord byte preservation and higher-level retention on
protected groups. Exact matching higher-level normalization is pinned as the
existing O1 behavior, not silently changed. A trace-on/off comparison requires
identical materialized music. Trace tests cover child forwarding, complete large
snapshots, filtered measures and prevented plans with zero physical writes.

An auxiliary surface-only probe runs the same synthetic cases against extracted
unchanged-parent and candidate resolver bodies and the actual BeamPlan model.
Its receipt stubs do not test custody or adoption. Parent: 13 expected failures
and 10 passes; candidate: 23 passes. The measurement test is excluded from this
limited probe and belongs to the full product suite. This demonstrates that the
protection tests reach prior writes; it is not a replacement for product tests.
Full strict tests and strict Clippy results are recorded in the delivery review.
The unchanged parent's tree, lint-integrity, dependency and take gates must pass
on the final incoming tree and on the extracted ZIP.

## Native evaluation

After native r1471 acceptance, compare the new complete export against r1470
(the musical output of r1471 must be byte-identical). The included read-only
`tools/assistant_diagnostics/compare_beam_take_r1472.py` accepts both export ZIPs.
It checks ten Master files byte-exactly, maps corresponding Render notes by
ordinal/timing/lane, enumerates all beam levels and checks non-beam XML content.
Insignificant indentation is compared separately from byte identity. Master
beam correspondence is reported only for congruent ordinal/timing/lane mappings;
no nearest-pitch association supplies identity. Changed primary sites and PDF
images still require human review. Optional `--pdf-pixels` compares all pages
at 108 dpi when PyMuPDF is installed. The comparator reports missing trace data
as an error; absence of an automatic primary trace row is not proof of no write.

## Pre-patch path roles

Metadata: ZIP_KIND and docs, including the implementation contract.
Audit: src/audit_only/writer_trace.rs and the read-only Python helper under
tools/assistant_diagnostics (explicit diagnostic owner path).
OtherSource: the existing lower-beam writer shard, corpus orchestration shard
and synthetic support file. No Kodex path,
validator, authority binding, Cargo dependency or toolchain policy is changed.
Selection restrictions and read-only observation are reviewed separately. The
new contract consumes only the unchanged accepted-parent error definition.

## Completed local delivery checks

Rust 1.98.0. Full strict all-target test suite: 2,393 passed, 0 failed,
1 ignored. Added tests: 24 musical/selection invariants and 4 trace invariants.
Strict all-target Clippy with `-D warnings`: passed, zero warnings.
The final diagnostic wording is also checked by that Clippy build. The full
suite preceded only a report-label clarification, with no subsequent musical
logic changes. No local corpus or renderer run was performed.

The exact r1470 export also independently reproduces the prior primary trace
counts: 192 StructuralLowerBeam changes, 187 later reversals (171 LowerBeeQueen,
13 Gould, 3 External), five unreversed sites. That descriptive association uses
stable ordinal/timing/lane plus enharmonically equivalent chromatic pitch. It
is not source-occurrence provenance and proves no secondary-beam attribution.
The five visible improvements and approximately 160 secondary changes remain
native evaluation hypotheses.

The final uncompressed tree and re-extracted ZIP are both validated with the
unchanged parent gates. Delivery remains conditional on actual native r1471
acceptance; no accepted-parent evidence has been fabricated locally.
