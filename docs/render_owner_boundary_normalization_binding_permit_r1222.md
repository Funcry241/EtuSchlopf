# r1222 RenderOwnerBoundaryNormalizationBindingPermit — K-R3

Parent: accepted/released r1221 RenderOwnerBoundaryNormalizationPermit, version 0.10.1839,
Take SHA-256 `86718169121b2521eea8cb3fb3da8478579f88e303eaa645ae8e4730a9b24efe`.
The supplied Lifecycle confirms accepted r1221 as the highest revision; r1222 is the next
free revision. No future Refactor revision is reserved.

Class: KodexOnly. Expected product effect: none.

## Exact amendment

The existing #67 gate freezes these two future wrapper statements:

```rust
let candidate = same_lane_strict_interior_raw_splice(&final_candidate, edit_start, edit_end, &replacement);
let (candidate, same_lane_splice_receipt) = same_lane_strict_interior_raw_splice(&final_candidate, edit_start, edit_end, &replacement);
```

Only `mut` on the wrapper binding is removed. The helper's internal `mut candidate`
remains necessary for the unchanged four Raw-Splice statements. In accepted r1221, the
wrapper only reads the extracted helper result, so the former frozen caller triggered
Rust 1.98.0's `deny(unused_mut)`; removing `mut` was rejected by that same frozen gate.
K-R3 resolves precisely that contradiction without lint suppression, artificial mutation,
new caller, mutable argument, alternative target or productive Rust changes. The gate
requires exactly the immutable call for the corresponding existing return form.

The effective permit parent is renewed to r1222 RenderOwnerBoundaryNormalizationBindingPermit.
The only immediate consumer remains RenderOwnerBoundaryNormalization, strictly RefactorOnly.
The r1220 base TSV stays byte-identical for accepted-bootstrap compatibility; every field is
validated before the three fixed effective permit bindings are applied. Old permit parents,
wrong names/classes/parents and direct RenderSovereigntyClosureMaxi consumption are rejected.
A missing target in K-R3 remains legal; a missing target after the authorized migration is fatal.

The target helper and signature, raw `(edit_start, edit_end, replacement)` identity,
EXACT/R classification, DEFERRED_RENDER_PREREQUISITE, one Raw-Splice, one buffer-splice
and zero file-write budgets remain unchanged. The original splice must disappear from
the old wrapper atomically; duplicate helpers/callers/splices and generic relocation remain
forbidden. No aggregate receipt, generic receipt list or nested receipt capability is added.

Row69, the separate #2/DirectionRelocation boundaries, last #58 committed render basis,
G1–G5, S3/S4, population/exclusion/nested-owner tables and every physical budget are unchanged.
Every productive Rust file and Cargo dependency is byte-identical to accepted r1221.
No physical extraction, receipt migration, RenderAdoptedState, Thread-local retirement,
Shadow Permit, Evidence/Proof, WAV or MusicMutation is part of this Take.

## Validation contract

The complete managed tree must pass the actual accepted-r1221 preflight. The existing
negative suite remains active. New tests admit only the two immutable bindings, reject
mutable/changed-geometry calls and the previous permit parent, and compile/execute both
synthetic caller forms under `deny(warnings)` with the real Rust compiler. The former
mutable binding must fail compilation specifically with `unused_mut`.

Actual results are recorded below before packaging. Windows linked tests, launcher Acceptance
and fresh 20/20 MusicXML / 51/51 PDF comparisons remain Windows execution checks; their byte
identity is expected from unchanged product sources, not claimed as locally regenerated.

## Executed validation

- Rust: rustc 1.98.0 (88d9e12ae 2026-08-18).
- Actual accepted-r1221 TakeChangeControl preflight: passed, 7 declared / 7 actual changes;
  0 productive music, writer-family or unknown semantic changes.
- Standalone scanner/TakeChangeControl suite: 174 passed, 0 failed, including 13 Render
  contract tests and real compiler execution of the immutable caller forms.
- Actual full-tree synthetic future relocation: passed; K-R3 remains Legacy with absent
  target, while only the authorized immediate RefactorOnly child becomes Relocated.
- Windows MSVC all-targets cargo check, strict profile: passed (69 seconds).
- Canonical Windows strict Clippy correctness/suspicious/perf groups: passed (62 seconds),
  zero warnings. The canonical product Clippy command does not add all-targets test linting.
- Trust anchor: all 47 exact-byte pins verified. Every TSV contract/budget is byte-identical.
- Changed gate/test rustfmt check: passed. Full cargo fmt --check has 3294 baseline diffs
  in both r1221 and K-R3; normalized outputs are identical. No baseline source was repaired.
- Productive Rust changes: zero. Cargo manifest, dependencies and local root lock unchanged.

A linked Windows build, the full Windows application test suite and fresh FullCorpus
MusicXML/PDF generation were not executed in this Linux environment. The accepted Windows
parent results are not represented as new K-R3 results. Root Cargo.lock is excluded from
the single full managed-tree ZIP; no outer directory or child Take is included.
