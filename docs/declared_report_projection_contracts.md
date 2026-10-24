# Declared report projection contracts v0.10.1527

## Closed producer boundary

v0.10.1526 introduced the exact-heading `ReportDocument`, but its generic
`append_projection` entrypoint still accepted any bytes from a producer. The owner knew that a
producer was late, but it did not know what that producer was allowed to materialize.

v0.10.1527 replaces the generic surface with declared projection contracts:

```rust
ProjectionIdentity::Section("EXACT-HEADING")
ProjectionIdentity::ExactLine("exact_legacy_line")
```

A producer is invoked through `project_section`, `project_exact_line` or `project_bundle`.
The ledger checks both the report pre-state and the isolated producer output.

## Fail-closed rules

For each declared producer:

```text
all identities absent   -> run producer and validate output
all identities present  -> do not run producer
partial bundle present  -> error
identity present >1     -> error
expected output absent  -> error
expected output >1      -> error
undeclared real section -> error
```

This makes the ledger the only owner of presence, duplication and bundle completeness. A producer
receives only an empty local output buffer; it never receives the current report serialization.

## Legacy compatibility

Some historical compact audits intentionally consist of one exact row rather than a heading plus
underline. Two later structural audits also preserve their old section-like first line without an
underline. They are registered as `ExactLine` identities, retaining every output byte while making
their ownership explicit.

The PianoVoice wait-tail helper formerly inspected its own output buffer with `contains`. That
stateful wrapper is removed; the pure section producer is now called under the exact ledger-owned
section contract.

## Retired private proof sink

The former feedback structuralization compatibility audit and `verify_no_projection` sink were
retired in r783 after the minimal non-linkable feedback-origin projection became the sole live
feedback report surface. No private producer is executed merely to prove that it emits zero bytes.

## Whole-document rewrites

Established sanitizers still operate on the complete report text. They are consolidated behind the
single owned `rewrite_document` transition. After every transition, `ReportDocument` is rebuilt
before any further producer contract is evaluated.

## Non-goals

No report wording or ordering beyond the package version, console result, release predicate,
pipeline order, music mutation, MusicXML serialization, PDF rendering, launcher behavior,
bootstrap behavior or security policy is intentionally changed.
