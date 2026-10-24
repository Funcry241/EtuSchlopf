# r1216 ExclusionOwnerReceiptSignatureMigrationKodex

Parent: accepted/released **v0.10.1839 r1215** `MasterLineageContinuityAndEditorialRequestReceipts`.

Change class: **KodexOnly**. This is the narrow governance prerequisite for the subsequent Master closure. It is **not** the later SSC-K2 Retirement Codex.

## Purpose

r1212 already permits one exact return-only signature migration for frozen **population owners**: EXACT owners may expose `Option<MasterExactReceipt>` and BLOCK owners may expose `Option<MasterBlockReceipt>`, while function name, generics and parameters remain byte-semantically frozen.

The remaining Master stage gaps are mostly wrappers around already-frozen **exclusion owners**. Before those owners can return typed receipts, the r1204 freeze gate needs the same kind of narrowly bounded return-only migration for exactly two frozen exclusion classes:

- `PRESERVING` → `MasterPreservingReceipt`
- `ADDITIVE` → `MasterAdditiveReceipt`

No other exclusion classification is eligible.

## Admitted migration

For a frozen eligible exclusion owner:

- owner path and symbol stay unchanged;
- function name and generics stay unchanged;
- every parameter, parameter order and parameter type stay unchanged;
- the frozen exclusion classification stays unchanged;
- the original return `R` may become exactly `(R, Option<Receipt>)`;
- an implicit unit return may become exactly `Option<Receipt>`;
- `Receipt` is fixed by the frozen classification: `MasterPreservingReceipt` for `PRESERVING`, `MasterAdditiveReceipt` for `ADDITIVE`.

Anything else remains fail-closed: parameter drift, additional payloads, wrong receipt type, implementation/transport/never-adopted exclusions, macro owners, new owners or changed classification.

## Physical surface budgets

A migrated exclusion owner that is also a productive XML write surface is mapped back to its **frozen exclusion signature solely for the existing r1204 surface-budget lookup**. Instance, XML-buffer mutation and file-write budgets are not widened.

The existing r1212 population-owner migration is left intact and is checked by an explicit regression test. Population v1 remains exactly **68 rows**; neither the population TSV nor the exclusions TSV nor the physical surface baseline is changed.

## ADDITIVE contract boundary

This take formally reserves `MasterAdditiveReceipt` as the only typed receipt return accepted for a frozen `ADDITIVE` exclusion owner. It does **not** introduce the runtime receipt type, migrate an owner, mint a receipt, alter a lineage or grant write authority. The subsequent RefactorOnly Master-closure take may introduce and use the runtime type under this already-frozen governance rule.

## Synthetic gate coverage

Positive tests prove:

- PRESERVING return-only migration;
- ADDITIVE return-only migration;
- implicit-unit ADDITIVE migration.

Negative tests prove rejection of:

- noneligible exclusion classes;
- parameter drift;
- PRESERVING↔ADDITIVE receipt substitution;
- extra return payloads.

A separate regression test proves the r1212 population-owner migration still accepts EXACT only with `MasterExactReceipt` and does not inherit the new exclusion contract.

## Product and authority effect

None. No productive owner signature changes in this take. No new writer root, no writer reachability, no write budget, no selector/ranking logic, no population row, no Genesis migration, no product mutation and no musical change.

Expected product effect: **none**.
