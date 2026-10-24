# Typed final report projection ownership v0.10.1525

## Problem

The final project report was assembled and finalized inside `real_main()` by roughly 1,100 lines
that treated one mutable `String` as projection target, state machine, compaction source, section
index and terminal artifact. Twenty missing-section repairs were hidden in a macro and four more
repairs queried the raw string directly.

## Ownership cut

The complete block now lives in
`src/pipeline_shards/final_report_projection_owner.rs`. It is a scoped lexical owner included once
at the exact point where all previously computed audit evidence exists. This avoids replacing the
old string coupling with a synthetic transport structure containing hundreds of unrelated audit
references.

The shared contract in `src/final_report.rs` enforces three states:

```text
DraftReport
  projection and pre-compaction transforms are allowed

CompactedReport
  reconstruction is complete; only bounded section recovery, pruning and final sanitizers remain

FinalizedReport
  immutable terminal artifact; required machine contracts validated
```

The Structural Write Authority accepts only `FinalizedReport` for the project result sink. A raw
report string therefore cannot reach the terminal write from the orchestrator.

## Retired string surgery

```text
append_missing_report_sections! macro: removed
open text.contains() repair gates:       4 -> 0
raw report write argument:               &str -> &FinalizedReport
real_main report block:                  ~1,100 lines -> one owner expression
```

The underlying report remains text because the output format itself is a text contract. The
retired part is unowned mutation and ad-hoc final-state discovery, not text serialization.

## Final machine contract

Finalization requires exactly one occurrence of each existing section:

```text
TYPED-LOWER-CLEF-SURFACE-MATERIALIZATION
SILENT-VOICE-COVERAGE-WRITE-OUTCOME
```

It also requires exactly one existing launcher-bound line:

```text
RendererOutcome: observed_from_existing_writer_audit.
```

Validation is read-only and runs after all historical sanitizer and writer-trace projections.

## Non-goals

No report wording beyond the version string, section order, release predicate, console result, pass order, music mutation,
MusicXML serialization, PDF rendering, launcher contract, take bootstrap or security policy is
changed.


## v0.10.1526 continuation

v0.10.1526 removes `Deref/DerefMut` from `CompactedReport` and replaces substring presence checks
with the exact-heading section ledger documented in `typed_report_section_ledger.md`.

## v0.10.1527 continuation

v0.10.1527 closes the remaining generic producer surface. Post-compaction producers declare their
allowed exact section/line identities, producer-owned presence checks are retired, mixed bundle
states fail closed and the legacy whole-document sanitizer surface is consolidated behind one owned
rewrite transition.

## v0.10.1553 continuation

The lower-clef projection no longer exposes the retired project-blind-pair and
semantic-writer envelope names. The final report projects the musical decision
audit and the typed materialization outcome as separate exact sections. This is
a report-identity migration only; writer authority and physical sink ownership
remain unchanged.

## v0.10.1560 continuation

The standalone clef-hysteresis report section and its required-section contract are retired.
The underlying hysteresis predicate remains integrated into `RustigClefCoursePlan`; ownership is
reported through the central end-to-end Rustig line instead of a duplicate exact section.

## v0.10.1839 r727 continuation

The report-only `ResidualLowerOctaveContextAudit` family is retired together with its exact
`RESIDUAL-LOWER-OCTAVE-CONTEXT-AUDIT-NO-WRITE` finalization requirement. r726 removed the producer
but left this historical required-section edge behind, causing the normal run to fail closed at
`FinalReportContract`. r727 closes that contract edge without changing productive register/octave,
clef, writer, blocker, release, renderer or artifact authority.

