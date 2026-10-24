# r1021 OrphanChordAnchorPromotionPlanComplexityReduction

Parent: accepted r1020 `OrphanChordAnchorBeamContinuationWarningDebtClosure`.

r1017-r1019 established one local final-render transaction that can promote a visible pitched chord member from a hidden rest host and, for the uniquely bound metric-tail case, complete the promoted anchor's beam state. r1020 closed the resulting strict-Clippy tuple-complexity warning with a private tuple alias.

r1021 is `RefactorOnly`. The private anonymous three-field tuple contract is replaced by one named data carrier with exactly the existing values:

- `edits`
- `anchor_promotions`
- `beam_completions`

The existing local closure remains local. Its result is carried as one plan value through `stage_final_orphan_chord_host_rest_candidate`; the same authorized batch writer receives the same edit vector, and the same two cardinality conditions are checked from the same values.

No helper function, writer, capability, selector, threshold, XML edit, report contract, dependency, lint suppression or gate is added. In particular, the r1017 anchor promotion and r1019 beam continuation rules are unchanged.

Expected proof:

- PRE-TAKE-DIFF-GATE `RefactorOnly` green;
- strict Clippy zero warnings;
- all existing tests green;
- Etuede 4 Master MusicXML byte-identical to r1020;
- Etuede 4 Render MusicXML byte-identical to r1020.

After a green single-project gate, freeze instead of another cleanup take.
