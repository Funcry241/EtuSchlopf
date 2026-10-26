# r1522 - StaffNotationTemporalContextDefinition

Version 0.10.1839. ChangeClass KodexOnly. Stage A: causal definition only.
Parent: native accepted r1521, accepted Take SHA-256
`c5f9d0a6dc7cb225835fab947e680264833a3c23ae4b7ceb1abe45d0b00d3c43`.
Physical source: the embedded r1521 snapshot from the 20261025-201718 export,
ZIP SHA-256 `83a7c603ceb3bfee47a9fe6f80dc602fa9ab1e4ff5c8ed6675867ca3133aaef6`.
The export captures r1521 while reexec is in flight; subsequent native acceptance
is separately recorded in the published r1521 evidence. Neither this source
snapshot nor local preflight implies native r1522 acceptance.

## Investigation and bounded conclusion

The current clef planner observes all visible pitched lower-staff heads, including
heads serialized in other voice streams. The problem is not a general exclusion
of extra voices. Pitches, clef surfaces and several protection queries instead
use byte intervals or serialized prefix traversal. A later stream can contain a
clef effective before a note already serialized in an earlier stream.

The independent readback recovered 90 actual planner-entry frames (55 distinct
byte contents) from the native console log and checked each recorded SHA-256 and
byte length. The observed temporal disagreement is already present in actual
inputs, not merely a reconstruction from the final XML. The input frames do not
bind a particular writer commit or prove what a corrected solver would select.

Final-output diagnostic evaluation found changed lower-head reference costs in
three pieces of the ten-piece corpus. This was an independent diagnostic, not a
classifier or governed distribution of the newly frozen error class. Counts and
real note/measure identities do not define class membership. They cannot narrow
the future implementation toward the observed corpus.

Protection queries need the same time basis as costs: a serialized prefix can
lack a lower tie even while another, later-serialized stream contains a matched
tie continuation at that musical time. This is evidence against a cost-only
patch, not a claim that an illegal clef was actually written. The supported new
beam-boundary observer already has temporal checks for its own bounded scope;
that must not be generalized into a proof for every existing planner consumer.

## This take

Adds only:

- docs/selector_derivation_contract_staff_notation_temporal_context.md;
- this investigation and handoff document.

Updates only ZIP_KIND.txt and src/build_gate/take_change_class.txt as candidate
metadata. No executable Rust, tests, dependencies, gate conditions, trust-anchor
pins, existing contracts, music or notation policy changes. Existing nested
component and Watch lockfiles are retained byte-exactly. Root Cargo.lock is local
transactional state and is excluded from the incoming managed-tree ZIP.

The contract separates a causal observation error from any later repair
permission. It includes UNKNOWN, correct serial traversals as positive neighbors,
unchanged legitimate ledger excursions, equivalent serialization invariance and
independent candidate duplication. It grants no new octave or clef optimization.

## Required next stage after native acceptance

A later repair-scope implementation must reference the unchanged accepted
error-class contract. Resolve observation, course ordering, candidate admission,
protection, normalization and postwrite checks within the existing owner and
receipt/adoption paths. Retain the existing cost weights and all protected
musical events. Complete baseline/alternative comparison must use the same
proven time relation, including surrounding carried state and attack accounting.
Unknown evidence grants no new write and must not be called a proved baseline.

Use independent synthetic and metamorphic cases, then actual materialization,
receipt/adoption, persisted readback and reentry. Only afterwards evaluate the
real full corpus and all PDFs. Joint clef/octave optimization remains a separate
unimplemented musical task; the local low-cost example is not a permitted plan.

## Validation status

Local validation is recorded with the delivered candidate. Native Windows
build, strict Clippy, tests, full corpus and acceptance for r1522 have not run in
this environment. The expected musical effect of Stage A is unchanged XML and
unchanged visible PDF content; this expectation is not a new rendered proof.

## Apply

Place the delivered Take ZIP in the usual project or user Downloads location,
then run from the existing r1521 project:

    .\run.cmd --take --all

The native hash-bound lifecycle record decides acceptance. Return the generated
assistant export before a Stage-B implementation is built against that accepted
parent. No --force and no separate install command are needed.
