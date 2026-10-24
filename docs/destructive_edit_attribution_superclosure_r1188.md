# r1188 Destructive Edit Attribution Super Closure Packaging Recovery

Recovery note: rejected r1187 never installed and is not a parent. Its ZIP was rejected by Take-Tree-Integrity because the local-transactional root `Cargo.lock` was accidentally present as a top-level payload path. r1188 is therefore a fresh revision directly on accepted r1186; the functional attribution design is unchanged, and root `Cargo.lock` is excluded.

Parent evidence: accepted r1186 reports 30 `CALLER_SUPPLIED_REPLACEMENT` typed authority surfaces, 25 PRODUCTIVE and 5 RETIRED. The source signatures resolve to 17 direct functions plus 13 functions emitted by `define_replacement_authority!` across six StructuralWriteAuthority modules.

The frozen core inventory is:

- bounded_semantic_rewrite_authority: `apply_voice_overfull_crop_edits_authorized`
- final_buffer_surface_authority: `apply_visibility_staff_distribution_batch_authorized`, `apply_chord_gesture_distribution_batch_authorized`
- render_measure_articulation_authority: `apply_bound_missing_rest_batch_authorized`, `apply_bound_articulation_batch_authorized`, `apply_bound_staccato_persistence_batch_authorized`
- rest_voice_family_authority: nine caller-supplied replacement batch entries
- staff_voice_atomic_authority: `apply_atomic_restification_batch_authorized`, `apply_atomic_staff_voice_scalar_batch_authorized`
- strict_staff_visible_lane_authority: thirteen `define_replacement_authority!` entries

Contract: mint identity at the typed authority boundary, attach one opaque `(batch nonce, input ordinal)` identity to each concrete bound edit before authority-local sorting, carry it through exact binding and application, and never recover identity from a physical locator. The captured callsite is provenance only and cannot serve as semantic evidence, proof or permit.

Audit states: `ORIGIN_ONLY` means the entry was observed but that individual edit never survived into an issued capability; `BATCH_ONLY` means the individual identity was bound but the complete authority transaction did not reach a successful applied result; `EXACT` means that same transported identity belonged to an edit in a completely successful authority transaction. Integrity mismatches are counted separately and must be zero.

Out of scope: musical repair, selector changes, ProofTypes, carrier-release permits, occurrence/receipt final-epoch closure, and any inference from function names or report labels.
