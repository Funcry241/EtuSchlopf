# r1205 MasterMutationRequestBoundaryClosure

Parent: accepted/released **v0.10.1839 r1204** (`DestructiveTransformationPopulationFreezeGate`).

Change class: **RefactorOnly**. The two rejected r1205 `MasterAdoptedStateGenesisExactBlockSuperClosure` candidates are not parents and contribute no installed source.

## Cause isolated by the rejected super-take

The rejected super-take made three previously non-writer helpers call the new Master adoption mutation sink directly:

- `build_editorial_voice_trial_candidate`
- `build_micro_materialized_candidate`
- `rejected_micro_materialized_candidate`

The accepted `NO-NEW-REPAIR-AUTHORITY` Kodex correctly rejected that topology. A helper that creates or evaluates a trial must not acquire productive writer reachability merely because Master state identity is being introduced.

The required ownership split is therefore:

`trial/repair builder -> immutable source-bound request -> already writer-reachable owner -> StructuralWriteAuthority materialization`

not:

`trial/repair builder -> productive mutation sink`.

## This take

A narrow request type is added below the r1204-frozen future consolidation boundary:

`src/structural_write_authority/master_adoption/`

`MasterXmlReplacementRequest` contains only the exact pre-materialization source bytes and the already-produced replacement bytes. Its fields are private. It carries no selection, score, target, threshold, repair-scope or adoption decision.

`apply_block_master_xml_request` is the only new productive XML mutation surface. It is source-bound and fail-closed: the current candidate XML slot must still equal the source bytes captured by the request. The function performs the replacement itself inside `StructuralWriteAuthority`; it does not delegate to an external legacy writer.

The three trial helpers now return only validation output plus a request. They do not call a productive writer.

Materialization moves one level up to owners that already possessed productive WriterReachability in accepted r1204:

- `run_editorial_voice_decision_pass`
- `run_certified_materializing_local_rewrite_portfolio`

No candidate selection, ordering, ranking, eligibility, target derivation or musical bytes are changed.

## Deliberately not included

This is not the Master adopted-state super-take. It does not introduce:

- GENESIS identities;
- EXACT/BLOCK lineage;
- private/opaque `layout_xml`;
- swap/drop/field adoption APIs;
- AtomicPatch identity transport;
- worker identity transport;
- Render-derived Master reload removal;
- Render-AdoptedState;
- WAV or AccentReserve changes;
- Proof or Permit.

Rows 57 and 67 of the frozen population remain `DEFERRED_RENDER_PREREQUISITE`.

## Expected product effect

None. For the full ten-project corpus:

- Master MusicXML: byte-identical to accepted r1204;
- Render MusicXML: byte-identical to accepted r1204;
- PDF notation: identical to accepted r1204;
- selection/adoption results: unchanged.

Any unexplained product delta blocks acceptance.
