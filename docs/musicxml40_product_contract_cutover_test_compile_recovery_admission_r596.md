# MusicXML 4.0 Product Contract Cutover Test-Compile Recovery Admission — r596

## Status

KodexOnly recovery admission. No runtime, writer, dependency, schema, MusicXML, PDF, or musical mutation is performed by this take.

## Parent authority

- EtuSchlopf 0.10.1839 r594 `MusicXml40ProductContractCutoverRecoveryAdmission`
- r594 remains the installed GreenBase after rollback of rejected r595.

## Rejected candidate

The one-shot cutover candidate r595 `MusicXml40ProductContractCutover` was rejected fail-closed during the integrated Rust test gate and rolled back before installation.

Rejected package SHA-256:

`1c5c113df7d55e886cabb53416e155326444b3f4d21eaf749fc67798b90c887c`

Reject cause:

`tests/persisted_schema_validation_gate.rs` failed to compile with Rust error `E0515`: helper `declared_product_version` returned `Option<&str>` borrowed from a locally owned `roxmltree::Document`. The productive EtuSchlopf build and Clippy gate were green; the failure is test-only and does not falsify the MusicXML-4.0 product-contract design, SchemaSchlopf-r021 integration, XSD bytes, writer cutover, or runtime Gate-A / Gate-B separation.

The rejected r595 candidate is not authority and must not be reused.

## Recovery decision

Do not widen `ValidationOnly`, do not add a lifetime escape, and do not weaken the executable Gate-A / Gate-B regression. The corrected test must own its returned product-version value (for example `Option<String>`) or perform the comparison while the parsed document is still alive.

Because r594 intentionally pinned the one-shot materialization to revision r595, a corrected candidate cannot simply reuse or renumber that rejected package without a new prior admission. This r596 take therefore changes governance only and moves the exact one-shot target to:

- revision `r597`
- name `MusicXml40ProductContractCutover`
- document `docs/musicxml40_product_contract_cutover_r597.md`
- parent revision `r596`

All substantive r592/r594 cutover constraints remain unchanged: exact r020 removal, exact r021 addition, exact MusicXML-4.0 XSD bytes, unchanged MusicXML-3.1 schema bytes, exact Cargo dependency transition, exact writer declaration `3.1 -> 4.0`, persisted-byte Gate A and Gate B, no shadow validator, no new macro/generated-source capability, and no music mutation.

## Completion boundary

r596 is complete when the existing fail-closed governance accepts exactly the new r597 target while productive runtime and product output remain byte-identical to r594.
