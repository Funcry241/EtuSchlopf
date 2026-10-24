# r1473: forward the existing optional corpus writer trace

Parent: accepted r1472, SHA-256
`14bf7e5ea89556c7b0e5474424bdf087074945d4813c2d03e2de34ce36b85ec5`.
Class: KodexOnly, because the launcher is an explicit Kodex path.
Authorization: existing request to deliver the beam fix with its measurement
course; this take repairs the omitted outer trace forwarding only.

## Change

The launcher parser already retains --writer-trace and --writer-trace=<list>.
execute_corpus_target previously replaced the complete execution arguments with
only --all. Its actual invocation now uses corpus_program_args, which keeps
--all and forwards only those existing trace arguments verbatim. All other
corpus argument behavior remains unchanged. No new option, environment channel,
selection rule, validator, music writer or authority is introduced.

The inner corpus-to-project forwarding was installed in r1472 and is unchanged.
Its complete snapshots are also unchanged: r1472 already removed the historical
14,000-byte XML truncation. compact_measure_labels still abbreviates only the
human-readable changed-measure list after 18 labels; it omits no snapshot XML.

## Tests and scope

Four new launcher tests exercise parse_args -> ExecutionRequest -> the argument
builder actually used by execute_corpus_target. They cover the bare flag,
explicit all and measure-list values on either side of --all, verbatim values,
default invocation and exclusion of unrelated options. Existing r1472 trace
unit tests cover the second hop and complete XML snapshots.

Pre-patch and final roles: src/bin/etuschlopf_launcher.rs and its existing
exact-byte pin owner src/identity_gate_trust_anchor.rs are Kodex; ZIP_KIND.txt
and this document are Metadata. The existing change manifest is the declared
intent. All pipeline, beam, voice, renderer, audit, qualified-component and
validator sources are byte-identical to accepted r1472. The existing
LAUNCHER_FNV1A64 assertion is synchronized to the exact final launcher bytes,
as required by the unchanged parent. Its check remains active; no pin entry,
validation predicate, strict lint or capability check is removed or weakened.

No new local music pipeline or renderer run is needed for this forwarding fix.
Local launcher tests, strict Clippy and unmodified parent gates are the checks;
native acceptance must additionally prove the actual trace reaches all ten
projects and that music output stays unchanged.

## Native command and acceptance

`.\run.cmd --take --all --writer-trace`

Require accepted r1473 with zero Clippy warnings and successful strict tests.
The optional WriterTrace report must exist for all ten projects, including
MetricProtectionObservation entries. Its presence must not be inferred from
the always-on primary-only trace. Real persisted writer snapshots and unwritten
original proposals remain explicitly distinct.

All ten Master and ten Render XMLs must remain byte-identical to r1472. PDF page
pixels must remain identical; PDF metadata may differ. The additional reports
may be substantially larger because snapshots are complete.

## Remaining music acceptance issue

r1472 is technically accepted. Its beam changes match the five expected primary
corrections and 165 secondary-state corrections toward Master, but E9/T32 onset
336 additionally changes technical voice 4 -> 2. This diagnostic take neither
reverts nor newly approves that musical acceptance deviation. The two remaining
E6 bridge/late-channel conflicts are also unchanged. Their next fix requires
actual late-writer evidence and a separately reviewable music scope.

## Local validation and evidence limit

Rust 1.98.0: all 216 launcher tests pass; strict Clippy over all targets
finishes with zero warnings. The unchanged parent validators accept the tree,
lint integrity, dependency transition and declared take. These checks are
repeated on the extracted delivery archive. No local corpus render was run.

The optional snapshots identify physical writer boundaries. They do not
necessarily distinguish internal planning steps: serialization_plan combines
PrimaryBeamVoicePartition edits and append_proven_alignment edits in one bound
plan before the final commit. Attribution between those internal steps may
therefore still require their existing diagnostics or a separate diagnostic
change. This take does not claim to resolve that attribution.
