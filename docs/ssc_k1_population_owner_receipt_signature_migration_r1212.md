# SSC-K1 — Population Owner Receipt Signature Migration Kodex

Parent: accepted/released r1211 `MasterIdentityFoundationAndCandidateLineageClosure`.

## Purpose

r1204 froze the 68-row destructive transformation population by semantic owner and exact owner signature. r1211 deliberately preserved those signatures and used a same-thread receipt collection scope for owners that still returned only XML or mutated XML in place. SSC-2 needs to retire that transition mechanism and let an existing frozen owner return its already-minted typed receipt directly.

SSC-K1 changes only the freeze policy. It does not change any population owner, writer, selector, lineage, MusicXML or product output.

## Receipt-only signature migration rule

A frozen population owner may keep its existing semantic identity while changing only its return type to expose the typed transformation receipt required by the already-frozen `identity_kind`.

The rule is derived directly from population v1; SSC-2 does not need to edit a Kodex file or register per-owner exceptions.

The following remain exactly frozen:

- owner path and symbol;
- function name and generics;
- every function parameter and its order/type;
- `effect_class`;
- `identity_kind`;
- productive write-surface budgets.

Only the return shape may change.

For an `EXACT` owner:

- `fn owner(args) -> R`
- may become `fn owner(args) -> (R, Option<MasterExactReceipt>)`.

For a `BLOCK_*` owner:

- `fn owner(args) -> R`
- may become `fn owner(args) -> (R, Option<MasterBlockReceipt>)`.

For an owner with implicit unit return:

- `fn owner(args)`
- may become `fn owner(args) -> Option<MasterExactReceipt>` for `EXACT`, or
- `fn owner(args) -> Option<MasterBlockReceipt>` for `BLOCK_*`.

`GENESIS`, macro-generated owners, argument changes, effect/identity changes, additional payloads and any other return shape remain fail-closed.

## Physical write-surface baseline

If such a migrated owner is also a productive write surface, the migrated signature is mapped back to its frozen population signature solely for lookup of the existing r1204 surface budget. Mutation counts and file-write budgets are not widened. A new owner, a different return shape, a changed parameter list or a larger budget remains a coverage violation.

## SSC boundary

This KodexOnly take performs no owner migration itself. It only establishes the narrow automatic rule needed by SSC-2 to replace r1211's same-thread receipt return channel with typed owner returns without requiring a mixed RefactorOnly/KodexOnly take.

The migration rule and `ADOPTION_BOUNDARY_V1` are temporary SSC scaffolding and are candidates for explicit retirement during the later SSC cleanup phase.

Expected product effect: none.
