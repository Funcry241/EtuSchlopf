# r1204 DestructiveTransformationPopulationFreezeGate

Parent: accepted/released **v0.10.1839 r1203** (`MasterExactPrerequisiteConstructionAuthorityClosure`).

Change class: **KodexOnly**. This take adds no music, selection, repair, writer, renderer, proof, permit, threshold or runtime authority.

## Purpose

Freeze the Phase-0 destructive-transformation population before the Master `AdoptedState` conversion.

The frozen semantic table lives at:

- `src/build_gate/destructive_transformation_population_v1.tsv`
- 68 rows
- 61 `DESTRUCTIVE`, 7 `UNKNOWN`
- 42 `EXACT`, 24 `BLOCK_*`, 2 `GENESIS`
- 25 Master, 32 Render, 11 both
- 9 `OPUS_VERIFIED`, 59 `AGENT_TRACED`, 0 `NEEDS_REVIEW`

Rows 4 and 63 are marked `RESOLVED_R1203`. Rows 57 and 67 remain `DEFERRED_RENDER_PREREQUISITE` and are outside r1205.

## Owner identity

Persistent anchors are **owner path + function signature**, never source line numbers. Line numbers produced by the existing scanner are used only transiently to bind a current physical surface to its current signature.

A registered owner may disappear. There is deliberately no reverse-presence requirement: retirement and consolidation are monotonic and must not be blocked merely because an old sink no longer exists.

## Exclusions

`src/build_gate/destructive_transformation_exclusions_v1.tsv` names non-population surfaces explicitly: implementation primitives, additive/preserving surfaces, never-adopted trials, and the single future Master-adoption consolidation boundary.

The future consolidation boundary is intentionally narrow:

`src/structural_write_authority/master_adoption/**`

New productive XML mutation functions there are admitted only when their function name begins with one of:

- `adopt_`
- `mutate_`
- `apply_exact_`
- `apply_block_`

and the surface is not directly called from `real_main`.

This exception permits r1205 to replace/retire old physical sinks with a typed adoption boundary. It is not permission for a new independent destructive transformation.

## Physical mutation closure

The gate does **not** implement a second mutation scanner. It consumes the existing `productive_write_path_scan` result.

`destructive_transformation_surface_baseline_v1.tsv` freezes the accepted-r1203 physical inventory as 55 owner/signature groups representing 59 scanner surfaces, with upper bounds for duplicate instances and XML-buffer/file-write counts.

Current surfaces must therefore be either:

1. an accepted-r1203 baseline group covered by a population row or explicit exclusion; or
2. a typed Master-adoption consolidation surface under the narrow boundary above.

All other new productive XML mutation surfaces fail closed.

Old groups are allowed to disappear.

## Trust binding

The gate implementation and all three TSV truth files are byte-pinned by `identity_gate_trust_anchor.rs`. `inventory_kodex.rs` invokes the freeze immediately after the existing productive-write scan, before the existing mandatory StructuralWriteAuthority boundary continues.

## Scope boundary

This gate freezes the source-derived population and current physical mutation coverage. It does not claim that a high-level semantic transformation can be reconstructed from XML geometry or runtime corpus counts. It introduces no edit identity and no permit authority.

The r1205 architecture remains responsible for transporting `GENESIS + ordered(EXACT|BLOCK)*` in the productive Master state.

## Expected product effect

None.

- Master MusicXML: unchanged from accepted r1203.
- Render MusicXML: unchanged from accepted r1203.
- PDF notation: unchanged from accepted r1203.
- No WAV, AccentReserve, Render-AdoptedState, Proof or Permit work.
