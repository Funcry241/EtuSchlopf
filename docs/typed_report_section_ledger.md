# Typed report section ledger v0.10.1526 / v0.10.1527

## v0.10.1526 starting point

v0.10.1525 established `DraftReport -> CompactedReport -> FinalizedReport` and prevented a raw
report string from reaching the authorized sink. v0.10.1526 then replaced the permeable compacted
string with a byte-preserving `ReportDocument`:

```text
ReportDocument
  ordered ReportBlock::Raw
  ordered ReportBlock::Section { ReportSectionId, exact_text }
```

Section identity is derived only from a heading line followed by its underline. Body substrings,
index entries and diagnostic references do not create a section. Parsing and rendering preserve the
exact preamble, blank lines, heading/underline bytes, section bodies and ordering.

The v0.10.1526 bridge still exposed generic post-compaction projection methods. Those methods were
useful to establish the ledger without changing report bytes, but they did not declare what each
producer was allowed to create.

## v0.10.1527 current boundary

The generic bridge is retired. Post-compaction mutation now has only these surfaces:

```text
project_section     -> one exact heading identity
project_exact_line  -> one exact historical line identity
project_bundle      -> a closed set of exact identities
rewrite_document    -> owned whole-document sanitizer transition
finalize            -> required machine-contract validation
```

`CompactedReport` exposes no `Deref`, `DerefMut`, mutable serialization, generic append operation or
producer-owned presence query. Each rewrite reparses and reindexes the document before another
projection contract runs.

## Final contract

Finalization requires exactly one occurrence of:

```text
TYPED-LOWER-CLEF-SURFACE-MATERIALIZATION
SILENT-VOICE-COVERAGE-WRITE-OUTCOME
```

The launcher-bound line remains exact:

```text
RendererOutcome: observed_from_existing_writer_audit.
```

## Non-goals

No report wording or ordering beyond the version string, release decision, pipeline order, music
mutation, MusicXML serialization, PDF rendering, launcher behavior, bootstrap behavior or security
policy is intentionally changed.

## v0.10.1560 continuation

The standalone clef-hysteresis report section and its required-section contract are retired.
The underlying hysteresis predicate remains integrated into `RustigClefCoursePlan`; ownership is
reported through the central end-to-end Rustig line instead of a duplicate exact section.

## v0.10.1839 r727 continuation

`RESIDUAL-LOWER-OCTAVE-CONTEXT-AUDIT-NO-WRITE` is no longer a ledger-required final section.
Its only producer and transport were retired as a report-only audit family; retaining the exact
section requirement after producer retirement was a contradictory contract state and correctly
failed closed in r726. The remaining required-section and renderer-outcome contracts are unchanged.


## v0.10.1839 r783 continuation

The obsolete `verify_no_projection` compatibility surface is retired together with the last
feedback-structuralization producer that depended on it. The active feedback report path remains
`project_section` over the canonical minimal non-linkable feedback-origin projection; no report
bytes, writer authority, or music semantics are added by this retirement.
