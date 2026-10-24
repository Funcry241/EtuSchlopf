# Causal serialized temporal precondition at the former cursor repair boundary

SelectorDerivationContract: causal_class_first_v1
SelectorChange: no
SelectorRole: error_class_definition
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: not_yet_measured
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: none
ErrorClassDefinitionIndependentOfKnownWitness: yes
CausalFeature: the serialized temporal replay observes cursor or note-end overflow, backup underflow, contradictory chord anchors or a disallowed final cursor position
CausalFeature: the boundary cannot establish supported replay coverage for the supplied document
NonCausalContextExcluded: project, measure label, staff, voice, pitch identity, input filename, occurrence count and corpus distribution
ErrorClassName: serialized_temporal_precondition_failure_at_cursor_boundary

This transition becomes authoritative only upon acceptance of its KodexOnly parent.
A local draft does not authorize implementation release.

Temporal failures are exactly the existing serialized_temporal_replay temporal_violation,
not its legacy hard_violation: cursor/note-end overflow, backup underflow, contradictory
chord anchors/durations and disallowed final cursor position. Rest durations participate
in exactly the same replay; crossing-rest overflow creates no independent classifier.
Implicit incomplete measures retain the replay semantics. Valid partial backups alone
remain green at this boundary; later gates retain their existing independent decisions.
The boundary may not change any XML byte, mint a receipt or adopt a repaired candidate.
Failure propagates through the existing Result path to project and batch, without
quarantine, fallback, candidate substitution or downstream processing.

Unsupported coverage is an independent rejection cause, never an empty green replay.
The bounded grammar has one score-partwise part and at least one measure, explicit
initial positive integer divisions and simple positive integer beats/beat-type, with
inherited context thereafter. Meter arithmetic must fit i64 and produce an integral
positive measure duration. At most one temporal-context attributes block before temporal events per
measure; non-temporal attributes do not change the replay context; at most one divisions and one time per block; time contains exactly beats
then beat-type. Note/backup/forward have one positive i64 duration except grace notes
with no duration. Multipart, additive/compound time syntax, senza-misura, malformed
or absent initial context and mid-measure changes are not silently modelled. No
project, measure number, staff/voice identifier, known witness or corpus frequency
selects class membership. Unsupported extension requires a separate contract.

The private successful proof binds the checked immutable input bytes and SHA-256.
The late render epoch may bind only that same sovereign Master and export document.
A proof for A cannot bind B. A caller-constructed audit is not a successful proof.

General sticky Unknown custody and adoption remain an explicit independent obligation.
Shared #54, shared #36 and the independently used CursorLaneRepair sink stay unchanged.
Child cannot authorize its own transition. The exact successor list below binds the
proposed implementation. Admission and qualification evidence are recorded separately.

## Necessity review

Temporal violation is necessary because this boundary promises a temporal precondition;
legacy partial-backup flags alone are explicitly excluded. Coverage is necessary because
an unparseable or unsupported input cannot supply that observation. Rest ends use the
same time calculus, not a separate witness classifier. Exact byte/hash binding is
necessary because a successful check on A says nothing about B. No criterion derives
from a known corpus location. The initial implementation mistakenly treated non-temporal
attributes as context changes; that was corrected to the stated divisions/time semantics
before any freeze. The first 20-input result for that erroneous draft was unsupported,
not a failure of musical time. The corrected implementation is independently measured;
the defect class was not narrowed by project or witness identity.

## Bound parent-to-child transition

The accepted semantic/output reference is r1444 (A4), Take SHA-256
b0c0308a93234b7c0bc947cc5c75945dc4d96d47c02728636869b64e675b2420.
The parent changes governance only. Its productive runtime remains byte-identical to r1444.
The build-script entry point imports the existing canonical SHA-256 module once;
all entry crates and the new governance share that same implementation. Hash code,
build profiles, gate order and gate outcomes are not changed by this import.
The child inherits the parent's validator, TrustAnchor and causal contract byte-identically.
It cannot add or amend its own authority. This document defines no generic exception
for read-only checkers, audit edits, owner retirement or proof dependency changes.

The only productive call sites authorized are the existing Master-export and Render
boundaries in pipeline_orchestrator_initial_render_stage.rs. Both return the typed
CursorLaneTemporalCheckFailure through the existing stage Result. The unchanged
observed_project_try propagates the Err and marks the project failed. The unchanged
batch controller records a failed project and returns FinalExit 1. No quarantine,
retry with the unchecked document, success audit on failure or downstream release is
permitted. A missing Render source is skipped explicitly, not checked or declared green.
Success is a statement at this boundary only, not a promise about later project gates.

The child removes both heuristic repair branches (backup/forward plus reanchor-rest
visibility, and chord-tag insertion) and the #56 crossing-rest rewrite invocation.
The shared crossing-rest writer used by #54, #36 rest visibility and the independently
used CursorLaneRepair artifact sink remain unchanged. The correction obligation for
an upstream temporal fault remains with its causal owner. This boundary refuses it.

Rows #24 and #56 retain historical identities/effect metadata but use the existing
RETIRED_SSC_K2 status and its established governance. Their descriptors, block mints,
exclusive repair helpers and receipt transport are removed. The documentation map
records retirement. Only their two containment-envelope rows are removed. Five origin
labels change from 'Master after #56 and before #65' to 'Unchanged Master after temporal
check and before #65'; artifact identities, ordering and other origin fields stay fixed.
The obsolete per-measure scan_raw_cursor_measure wrapper is removed with its last
callers. The similarly unused receipt-discarding adapter
apply_lower_staff_secondary_reanchored_lane_rest_visibility_suppression is removed;
the shared receipt-bearing #36 implementation and all its remaining callers stay unchanged.
The replay engine, current-meter interpretation, full-document observer and
all existing temporal predicates remain byte-identical.
After installation, every tree validation binds the full serialized_temporal_replay.rs
file to the exact successor SHA-256, independently of change class. Its audit_only
location grants no AuditOnly exception: changing its temporal decision or any other
bytes is refused, including in a later AuditOnly take. A future semantic evolution
requires a separate preceding KodexOnly governance transition; the runtime child
cannot authorize it. The first-transition pin alone is not the steady-state protection.

At the productive call site SolverCandidate::layout_xml returns exactly
self.master_state.document(). No master mutation occurs between checking that borrow,
copying final_export_xml and binding the epoch. Their byte equality is structural,
not inferred from corpus results. Existing epoch-head validation remains mandatory.

The late Master epoch requires the private successful CheckedCursorTemporalInput,
the original sovereign Master and the unchanged export document. Equality and SHA-256
must bind the same bytes. Audit counters alone cannot construct that proof. No new
mint, write authority, receipt family, sink, quarantine route or adoption authority is
introduced. Existing #65/#59/#60 processing remains after the check. No general
SelectedCustody::Unknown adoption hole is claimed closed by this transition.

Legacy audit labels remain available for comparison, with zero writes and a new
explicit input hash, checked-measure count, temporal-violation count, unsupported-input
count and observed partial-serialization count. A temporal or coverage failure carries
its nonzero rejection audit in the propagated error rather than a green repair record.

The initial-render construction pin admits only the exact successor stage. The formerly
unactivated r1441 cursor-evidence-borrow successor is not admitted as an alternative.
The existing #57 positive construction pin also admits exactly its successor test file,
whose only dependency change is obtaining the real checked temporal proof. Historical
RSP test trees use a frozen copy of the accepted origin table, independently of the
new live label epoch; their table-mutation counterprobes remain unchanged. The receipt
matrix loses exactly the two retired owner bindings. Three other
existing epoch tests/fixtures obtain the same real proof; missing temporal context in
synthetic fixtures is supplied explicitly. E007 production phrase projection, including
its system-break limitation, stays unchanged. A mismatched-A/B epoch counterprobe must
exercise the real epoch constructor and refuse binding.

The source dependency firewall retains all previous exclusions. Its one exact checked
epoch method may mention the new proof type; only that single fully-qualified type
occurrence is excluded from the legacy substring 'tempo' check after the complete
method SHA-256 matches. This is a typed read-only input dependency, not tempo evidence,
calibration or a writer capability. Other occurrences and other methods remain checked.

The parent binds complete before/after SHA-256 sets for the first transition and an
exact changed-path set. The source classifier may admit only the finite observational
and control deltas caused by this bound retirement: the main module declaration,
unused per-measure observer removal, the two removed repair functions, the three
controlled population/origin/envelope tables and removal of the old receipt transport
from the two release stages. Two existing test readers (path_bound_receipt_mismatch_is_rejected_before_productive_write
and ssc_render_owner_commit_is_observed_without_changing_adoption_or_bytes) change only
their synthetic document constants to the complete temporal fixture; their rejection,
receipt verification, observation and closure assertions remain in force. Their full
file bytes are part of the same successor binding. New writer/recovery/identity-provenance/calibration checks
run before this narrow classification allowance. Summaries retain all deltas.
The inherited TrustAnchor pre-binds the old and successor table fingerprints; runtime
validation couples the installed checker, retired population and origin epoch. The
child cannot change TrustAnchor or the validator. No generic pin weakening is allowed.

## Required evidence before release

Local qualification uses the unchanged Rust/Cargo/Clippy 1.98 toolchain and existing
Strict profiles. It includes production Strict build, strict Clippy with correctness,
suspicious and perf denied, and the normal locked test --all-targets contract with its
existing root-package opt-level=0 test override. No toolchain/profile/optimization
change is part of either take. The parent and child must each pass admission; changes
to the local draft after qualification invalidate the affected result.

Fast causal synthetic tests exercise both real call-site adapters: clean unchanged
input, valid partial serialization unchanged, true temporal errors rejected, rest
crossing the boundary rejected, and unparseable/unsupported coverage rejected. They
also check exact A/B byte binding, a real epoch refusal, inheritance of supported meter,
non-temporal attributes and invariance under independent duplication. They do not
instantiate the solver or depend on corpus identities. Negative admission probes must
refuse changed boundary bytes, changed checker/epoch, unrelated changed paths,
resurrected owner authority, and child changes to its parent validator.

The 20 captured native r1442 boundary inputs are a separate preflight measurement,
not features of the class or synthetic test inputs. Each records source, bytes,
SHA-256, supported replay and result. They were green and the old #56 was unchanged,
without receipt. The final acceptance remains a fresh native r1446 run against accepted
r1444: 10/10, FinalExit 0, PDFs with -o without -f, all 20 Master/Render XMLs byte-identical,
and new violation/unsupported counts zero. Any byte difference blocks acceptance;
it may not be explained away by the old input measurement. Parent native acceptance
precedes implementation release. The exact successor list below binds the proposed local child. Its publication is
not a claim that local qualification or final native acceptance has already occurred.

## Exact successor files

All changed child files are listed, including metadata and the change manifest.
The compiled first-transition guard binds all payload successors except the manifest,
whose declared fingerprints and complete changed-path set are checked by admission.
SHA-256 is computed over the full file bytes. No successor permits an implicit extra
file. Shared parent governance files are inherited unchanged and are not child changes.

| Successor path | Bytes | SHA-256 |
| --- | ---: | --- |
| `ZIP_KIND.txt` | 138 | `97e236c375c8cf7845e4013260ef522740377658a21cc265d8ea1769a9b1a170` |
| `docs/render_opus_population_map_r1225.tsv` | 20265 | `3e17108eb93bab93f229ba50425f006b0f772f25befdc9fea9a983e9363da733` |
| `docs/selector_derivation_contract_cursor_temporal_check_r1446.md` | 2219 | `4e3c5eb53df56f9e398aceb7b74563cf7b6c387341e13c3c557e47664bcfa543` |
| `src/audit_only/serialized_temporal_replay.rs` | 19877 | `9bf86d21e8e42c1e0b47cad29e226fdd629b7d873b6d69324d2553f2006d662f` |
| `src/build_gate/destructive_transformation_population_v1.tsv` | 29662 | `77a90a3e30ac4d0728cfeae543e567387d69a53aa867e52455fe81442379f463` |
| `src/build_gate/render_origins_v1.tsv` | 2015 | `c2702e3971d92c247c5789a3750bb1a3c154011cd1133445fdc7f3c7f7e959c8` |
| `src/build_gate/ssc_containment_envelopes_v1.tsv` | 13989 | `535fcedabf3a867c28d732f15722cd843a26d219eb48ffdd157bbd25033825f9` |
| `src/build_gate/take_change_class.txt` | 2336 | `599d3a5b03762b90e7d0af2c9e35b1956f0ac61fd682911292dc9b7b5c01e63e` |
| `src/cursor_lane_temporal_check.rs` | 7137 | `613c9599d6b4a3fbddc5713dc187691ff0f657965f5369ad7c3376a53cc73793` |
| `src/main.rs` | 2422 | `5c0c5e953391cf177c8fb710a54d0a3ea823867713e5f32b2ad7a2b103c11f62` |
| `src/master_state_identity.rs` | 159724 | `49a9757503c5ff6ede3ce7308f1bd07c013fcdf8f49f2264183a89979cbc8932` |
| `src/master_state_identity/render_lineage.rs` | 72638 | `f06c4704e15a8a5bcdf133acd938274a7c0d72060a4a3a34f7cb7b8de605d628` |
| `src/master_state_identity/tests.rs` | 66735 | `5f76922456d7ce8e10f2a9509a07363e912c890bb4a1cec628a6eec0d7e10899` |
| `src/pipeline.rs` | 8615 | `d3333cd6a742116982440d4ab653950ecbd28692f109a44a62ee60585935c38b` |
| `src/pipeline_shards/pipeline_orchestrator_clef_layout_support.rs` | 19802 | `1b2417156b87085b9df497cdd85d6afcd2ff2fdfb4154c93d524d1438ec9d226` |
| `src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs` | 74875 | `c8e3ab28bb11574b5a72c00ff9e1cb038f3a97d413ece3e624528a645c1e28cd` |
| `src/pipeline_shards/pipeline_orchestrator_release_report_exit_stage.rs` | 8529 | `9bff3ee977aa56743b8e7b785db63c155cd3c473f76c7428f72413c764d8d142` |
| `src/pipeline_shards/pipeline_orchestrator_release_support.rs` | 27340 | `eaa6cda7e6d2143bc401e31653bda57ca3086f319e98714f8f5d82d72ef16802` |
| `src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs` | 152757 | `588f2194e477e1ae4b5837afeecd65089baac5cbcac53a5535c6303a4826537d` |
| `src/pipeline_shards/report_hygiene_cursor_localbest_trace_tail.rs` | 17462 | `7dd472227a839b2044243d8c24af3eff23ab0abd80aa38ee1d81fa36e3803bef` |
| `src/structural_write_authority/final_notation_phrase_projection_transaction.rs` | 46477 | `ca8e798a9b5758fb1d9a4cd584c8c55d9cf45e424d63784cf295a492ecb09726` |
| `src/structural_write_authority/master_adoption/render_adopted_state_tests.rs` | 37826 | `9a932b5f5badbcf758af43dc4562d512033cfee627a2642badd7eee5be1a1dc3` |
| `src/structural_write_authority/render_lineage_receipt_tests.rs` | 6097 | `883bda12586434c97afd7ac753b38905f0851d5ddc845a68aa42a361bc00ac06` |
| `tests/support/render_authority/row57_local_clef_scope_through_trial.rs` | 36236 | `54ac06aa7acd3556d49c5a160519f1fba735a3095088725bdf5dddd2b856769d` |
| `tests/unit/cursor_temporal_boundary.rs` | 5019 | `57e09db5aa90d5f44fa96963e097f8d1c8b3de40caa9411887ce28d3846c1f19` |
