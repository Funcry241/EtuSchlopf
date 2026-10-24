# r1096 HistoricalNormativeTestDebtRetirementSuperCutManifestRecovery

Parent: **r1094** (`HistoricalSnapshotSyntheticInvariantRetirementCursorTopologyRecovery`)

ChangeClass: **RefactorOnly**

r1095 was rejected at `dependency_preflight` before any transaction because its
`src/build_gate/take_change_class.txt` accidentally still described the accepted r1094
change set. r1095 has no authority. r1096 carries the same intended historical-test
retirement directly from canonical r1094 with an exact physical-diff manifest.

## Scope

This take retires a homogeneous class of normative test debt whose remaining purpose is
to remember already-retired implementation history rather than to prove a timeless
product invariant.

E32 applies: a historical path, revision, retired selector, retired report vocabulary,
or past cutover is not itself a normative specification.

## Retired standalone historical tests

The following test files are deleted:

- `tests/corpus_pdf_stability_projection_retirement.rs`
- `tests/source_selection_soft_register_deletion_retirement_contract.rs`
- `tests/source_selection_staff_rhythm_authority_coupling_audit.rs`
- `tests/wav_pitch_rescue_selector_retirement_closure.rs`

They collectively pin retired vocabulary, removed source surfaces, an r639 source-code
snapshot, or retirement-history documents.

No current productive owner depends on these files. Where a current invariant exists,
it is already covered by current semantic tests:

- corpus output completeness and renderer evidence are covered by the typed corpus
  manifest / run-outcome contracts;
- canonical source retention is separately kept out of candidate scoring by current
  source-selection guard tests;
- the retired soft-register surface is already absent from and independently checked
  by the current full-causal source-selection contract;
- WAV decoding, attack/pulse, harmonic evidence and external-sensor behavior have
  direct module-level tests and do not need a selector-retirement remembrance test.

## Retired history-only assertions inside surviving test files

Two history-only tests are removed while their files and current invariant tests remain:

- report-sanitizer retired replacement-table cardinality/history pin;
- legacy batch page-count transport survival during the console-foundation migration.

Neither assertion expresses a timeless product invariant; both bind a past transition
state.

## Explicit non-scope

This take does **not**:

- change `.musicxml`, `.render.musicxml` or PDF behavior;
- change any productive writer, selector, threshold or repair authority;
- change `GenericityEvidence` or `candidate_count == 1`;
- rename historical tests merely to hide their origin;
- claim that all historical test debt is now zero.

The next corpus-independence inventory must classify remaining history-named tests by
meaning, not by lexical name alone. Protocol backwards-compatibility and fail-closed
migration boundaries may remain normative when they express a current supported
contract.

## Expected test delta

r1094 has 1,409 passing tests. This cut removes fourteen history-only tests and adds no
replacement test functions.

Expected all-target total after the cut: **1,395 passing tests**.

The lower test count is intentional. Test count is not a quality metric.
