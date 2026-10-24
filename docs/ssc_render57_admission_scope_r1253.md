# r1253 exact writer and inline-test transition correction

Accepted parent r1252; named SscK2GovernanceRevisionKodex; KodexOnly.

The r1252 documents remain byte-identical historical contracts. This named revision corrects their future writer transition: the whole-file expected result is exactly one legacy construction region replaced by the bound V2 region, followed by exactly one replacement of `assert_eq!(production.matches("xml.insert_str(").count(), 1);` with `assert_eq!(production.matches("xml.insert_str(").count(), 0);`. Both construction regions and both assertion forms have exact before/after multiplicity checks. The old assertion requires a production operation that V2 deliberately removes; retaining it prevents a valid full test run. No other inline test or file byte may change in that transition.

The existing range validator equality remains mandatory. Visibility, attributes, unrelated writers and helpers remain bound by whole-file equality. The transition still requires MusicMutation and all existing parent semantic and selector-contract admission checks. This KodexOnly take does not install V2 or modify productive source.

After V2 installation, byte-identical writer files remain admitted. Any further writer-file change is rejected, including rollback, until an explicitly named governance revision defines a later construction contract. No general post-installation edit permission is granted here.

Governance tests exercise the real installed writer in both legacy and V2 phases; phase reconstruction exists only in tests. They admit only the complete paired transition and reject region-only edits, assertion-only edits, assertion weakening or duplication, range-validator changes, visibility changes and unrelated appended writers. Synthetic fixture tests continue to protect cfg attributes, independent helpers and the envelope validator. The checker never reconstructs or rewrites its parent.

The original causal error class and immutable V1/V2 fixtures are unchanged. Six OPEN obligations, #57 DEFERRED status, #67, WAV product decisions, SSC proof authority and disabled enforcement are unchanged. Runtime repair, full through-tests, released-artifact closure and corpus measurement remain separate obligations.
