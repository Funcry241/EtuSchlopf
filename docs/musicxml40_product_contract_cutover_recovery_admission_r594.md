# MusicXML 4.0 Product Contract Cutover Recovery Admission — r594

## Status

KodexOnly recovery admission. No runtime, writer, dependency, schema, MusicXML, PDF, or musical mutation is performed by this take.

## Parent authority

- EtuSchlopf 0.10.1839 r592 `MusicXml40ProductContractAdmissionBoundary`
- r592 remains the installed GreenBase.

## Rejected candidate

The one-shot cutover candidate r593 `MusicXml40ProductContractCutover` was rejected fail-closed before installation.

Rejected package SHA-256:

`29cbcb53b77ce9cdfd758d05e273c7e217747d0a2a53edfd0a7d0551e946669f`

Reject cause:

`Semantic-Capability-Diff` detected a newly added `MacroOrGeneratedSource` capability from a second `env!("CARGO_MANIFEST_DIR")` occurrence in `src/audit_only/format40_conformance_foundation.rs`. The macro was introduced only by a new test and is not required by the productive MusicXML-4.0 contract.

The rejected r593 candidate is not authority and must not be reused.

## Recovery decision

Do not widen `ValidationOnly` and do not admit new macro/generated-source capability. The corrected product cutover must remove that unnecessary new macro occurrence.

Because r592 intentionally pinned the one-shot materialization to revision r593, a new candidate cannot simply be renumbered without a new prior admission. This r594 take therefore changes governance only and moves the exact one-shot target to:

- revision `r595`
- name `MusicXml40ProductContractCutover`
- document `docs/musicxml40_product_contract_cutover_r595.md`
- parent revision `r594`

All substantive r592 cutover constraints remain unchanged: exact r020 removal, exact r021 addition, exact MusicXML-4.0 XSD bytes, unchanged MusicXML-3.1 schema bytes, exact Cargo dependency transition, exact writer declaration `3.1 -> 4.0`, persisted-byte Gate A and Gate B, and no shadow validator or music mutation.

## Completion boundary

r594 is complete when the existing fail-closed governance accepts exactly the new r595 target while productive runtime and product output remain byte-identical to r592.
