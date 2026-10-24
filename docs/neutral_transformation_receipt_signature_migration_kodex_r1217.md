# r1217 NeutralTransformationReceiptSignatureMigrationKodex

## Parent and scope

Parent is only accepted/released r1216 `ExclusionOwnerReceiptSignatureMigrationKodex`, canonical SHA-256 `b3c1b58b10f40de06dfd550a3af191584054789f62275c7268cf6c33152198d0`.

This is a narrow `KodexOnly` prerequisite for later SSC Master/Render lineage work. It changes only the frozen destructive-population signature gate and its existing byte trust anchor. It does not introduce runtime receipt structs and it does not migrate a productive owner.

## Event receipt, not state domain

A transformation receipt proves one concrete execution / byte transition by its real owner. Master versus Render is not encoded into the neutral receipt name; the later adoption boundary decides which sovereign lineage consumes the event.

Therefore r1217 admits exactly four neutral receipt names in addition to the already legal legacy names:

| Frozen contract | Legacy name retained | Neutral name additionally admitted |
|---|---|---|
| `EXACT` | `MasterExactReceipt` | `ExactTransformationReceipt` |
| `BLOCK_*` | `MasterBlockReceipt` | `BlockTransformationReceipt` |
| exclusion `PRESERVING` | `MasterPreservingReceipt` | `PreservingTransformationReceipt` |
| exclusion `ADDITIVE` | `MasterAdditiveReceipt` | `AdditiveTransformationReceipt` |

This is not permission for two receipts from one execution. The only admitted signature delta remains the existing single `Option<Receipt>` return extension. Function name, generics, parameters and original return payload stay byte-contract equivalent to the frozen signature.

The Population `M/R/B` field remains frozen inventory metadata and is not converted into a second type authority. The exclusion ledger intentionally remains without a newly invented domain column.

## Fail-closed exclusions

The r1216 exclusion boundary remains unchanged: only frozen `PRESERVING` and `ADDITIVE` owners are eligible. `IMPLEMENTATION_SURFACE`, `IMPLEMENTATION_TRANSPORT_SINK`, `ADOPTION_CONSOLIDATION_BOUNDARY`, `NEVER_ADOPTED`, macro-generated exclusions and every other class remain ineligible.

The gate still rejects parameter/generic/function drift, the wrong receipt class, an extra return payload and any attempt to turn a non-eligible exclusion into a receipt owner.

## Runtime deferral

r1217 deliberately does not:

- define `ExactTransformationReceipt`, `BlockTransformationReceipt`, `PreservingTransformationReceipt` or `AdditiveTransformationReceipt` as Rust runtime structs;
- rename an existing `Master*Receipt` runtime type;
- change `Clone`/`Copy` behavior;
- introduce `RenderAdoptedState` or Render Genesis;
- alter OwnerReceipts, PatchedXml, request transport or adoption;
- change a Population/Exclusion row, classification, domain or write budget;
- add or remove productive writer authority.

Those are later RefactorOnly/runtime tasks after this Kodex is accepted/released.

## Regression contract

Existing r1212/r1216 negative-check identities are retained. Their positive fixtures now prove both legacy and neutral receipt names. Negative fixtures additionally prove that neutral names cannot cross receipt classes and that `IMPLEMENTATION_TRANSPORT_SINK` remains ineligible.

## Expected effect

No MusicXML, Render MusicXML, PDF, ranking, selection or musical effect. The subsequent 10/10 run is expected to reproduce the r1216 product bytes while establishing the accepted governance parent for Astra T1.
