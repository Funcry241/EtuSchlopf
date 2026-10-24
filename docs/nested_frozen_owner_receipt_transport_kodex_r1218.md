# r1218 NestedFrozenOwnerReceiptTransportKodex

## Parent and reason

Parent is only accepted/released r1217 `NeutralTransformationReceiptSignatureMigrationKodex`, canonical SHA-256 `45c033ecd0f9b72b05db7d262d9b75f7582a6305615640d8baf82d0b69a62d5d`.

Astra's T1 preflight isolated one real governance gap: population owner #59 performs its own frozen EXACT opening-`<fifths>` transition and then invokes the separate frozen EXACT owner #60 through the already-frozen `functional_respelling: FunctionalMasterRespellingOperation` parameter. r1217 permits each owner to add exactly one classification-matched `Option<Receipt>` return, but it has no legal way for the outer #59 call to return both distinct event receipts in execution order without a forbidden side channel.

This KodexOnly take closes only that transport gap. It does not implement T1, does not define runtime receipt structs, and does not change any productive owner.

## Frozen pair

The new pinned table `src/build_gate/destructive_transformation_nested_owner_v1.tsv` contains exactly one row:

- outer population: `59`
- inner population: `60`
- carrier parameter: `functional_respelling: FunctionalMasterRespellingOperation`
- binding site: `src/pipeline_shards/pipeline_orchestrator_metric_final_notation_stage.rs`
- binding expression: `structural_write_authority::FunctionalMasterRespellingOperation::new(canonical_master_functional_respelling_from_render,)`
- execution order: `outer_then_inner`

The freeze gate rejects a second row, another pair, a changed carrier, a changed binding path/expression, a changed order, non-`EXACT` identity, or non-`M` domain.

It additionally verifies that the frozen carrier construction occurs exactly once and that #59 consumes its `functional_respelling` carrier exactly once through `.apply(...)`.

## Narrow signature capability

Only the listed outer owner may additionally migrate from its frozen original return to:

`(R, Option<ExactTransformationReceipt>, Option<ExactTransformationReceipt>)`

where `R` is #59's original frozen return payload (`io::Result<()>`). Function name, generics and parameters remain byte-contract equivalent to the frozen signature.

The first receipt slot belongs to frozen outer owner #59; the second belongs to frozen inner owner #60. Runtime T1 must still verify each receipt's sealed descriptor and continuity before adoption.

This capability does **not** admit:

- `OwnerReceipts` as a return payload;
- a generic vector/list of receipts;
- arbitrary second or third receipts;
- legacy `MasterExactReceipt` pairs;
- a parameter/out-parameter side channel;
- a new carrier type or changed parameter list;
- a second nested-owner pair;
- new writer reachability or write budgets.

Any future nested-owner pair requires a new Kodex change because the v1 table and its exact single row are compile-time/trust pinned.

## Existing r1217 rules remain

The normal r1217 single-`Option<Receipt>` migration remains authoritative for all other population owners and eligible exclusions. Population v1 stays at 68 rows. The existing exclusions, domains, classifications and physical surface budgets are byte-identical.

No productive Rust owner, selector, ranking rule, MusicXML mutation, Render mutation, WAV logic, Master adoption, file reload or musical result changes in r1218.

## Tests

The freeze gate includes positive and negative tests for the exact nested return shape. It rejects `OwnerReceipts`, a missing inner receipt, parameter drift and legacy paired receipt names. The table-shape test rejects additional rows and wrong owner pairs.

## Expected effect

No MusicXML, Render MusicXML, PDF, selection, ranking or musical effect. r1218 is only the narrow governance parent required before the Master export/lineage T1 work can legally transport the separate #59 and #60 events.
