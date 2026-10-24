# r1470 – Closed audit sections and fixed measure probe retirement

RefactorOnly. Parent: accepted r1469. No declared music change.
Prepared by Dachs (Claude) while Maus had no budget; review by Maus recommended.
One cleanup theme: report-only code without decision consumers.

## Removed

1. Two read-only report producers whose output was consumed only by the result file:
   - `src/audit_only/semantic_obligation_structural_observation_shadow.rs`
     (section SEMANTIC-OBLIGATION-STRUCTURAL-OBSERVATION-SHADOW);
   - `src/audit_only/octave_kind_boundary_evidence.rs`
     (section OCTAVE-KIND-BOUNDARY-EVIDENCE-AUDIT in `report.rs`).
2. The only fixed measure number in production code: the local geometry probe in
   `strict_staff_local_release_core_receipt_ownership.rs` (fixed measure/staff/
   interval/pitch spec and the `scope 40 staff 2` gap-trace filter). It filled only
   `local_geometry_probe_line`; the adoption decision `invariants_ok` never read it.
   Removed with it: the fields `gap_trace` and `local_geometry_probe_line` of
   `StrictStaffOverfullCoverageNormalizationTrial`, the report line
   `StrictStaffLocalGeometryWitness`, and in
   `visible_rhythmic_occupancy_phase_delta_audit.rs` only the probe types, the probe
   function, its two private helpers and its three tests. The module itself and its
   phase-delta report stay.

The gap trace returned by the materialization function is no longer stored
(binding `_`); the function and its decisions are unchanged.

## Consumer check per section

For each candidate the section heading, producer key and every emitted line prefix
were searched in `src/`, `tests/`, `tools/`, `run.cmd` and `strict.cmd`.
No launcher filter, gate, trust anchor or take-control list reads them.

Deliberately kept:

- `source_selection_full_causal_audit_shadow_plan` (largest result section): it is
  the only reader of about twenty fields of the productive source-selection
  projection (`midi_seed.rs`, `source_selection.rs`). Removing it fails strict Clippy
  with dead fields; that cleanup touches productive code and is a separate theme.
- `final_chord_visible_rest_interval_epoch_witness_audit` and
  `lower_staff_overlapping_rest_bound_dispensability_audit`: the launcher projects
  their lines and the `Reanchor…` lines of the same section into the assistant
  project summary.
- `staff_local_rest_ownership_audit`, `source_event_causal_ledger`: their inputs
  are read only by these sections; removal would cascade into orchestrator stage
  outcomes.
- `canonical_measurement_snapshot`, `mutation_ownership_evidence`,
  `report_sanitizer_rewrite_scope_projection`: launcher or pinned consumers.

## Class

The probe lives in a MusicSensitive path (`src/structural_write_authority/`) and
the report-line removal changes a production struct. That is a behaviour-neutral
refactor, not an audit-only change: RefactorOnly. No owner is moved, no path
renamed, no path role adapted, no added line carries decision control flow.

## Acceptance

All 20 XMLs byte-identical to accepted r1469. Result files differ only by the two
removed sections and the missing `StrictStaffLocalGeometryWitness` line.
Strict Clippy and strict tests green; unchanged parent/candidate validators.
