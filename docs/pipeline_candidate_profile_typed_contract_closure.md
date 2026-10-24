# PipelineCandidateProfileTypedContractClosure

## Scope

`v0.10.1651 r01` replaces the profile display-name protocol in the FLP candidate and immediate solver control path with one closed typed contract. The change is `RefactorOnly`: it does not add a profile, mutation, threshold, pass, score, gate, or project-specific rule.

The regular module boundary is `crate::pipeline_profile`. Its crate-visible surface is limited to catalog construction, the opaque `FlpImportProfile`, typed lineage, and the three legacy display projections still required by reports. Profile fields and lineage bits are private.

## Inventory of the 38 former semantic string reads

The inventory counts atomic semantic reads, not source lines. Pure formatting, report output, and filenames are excluded.

| ID | Former file:function | String component | Controlled decision | Former fallback | Typed owner |
|---:|---|---|---|---|---|
| 1 | `flp.rs:apply_residual_critical_pass` | prefix `residual_` | residual pass admission | reject pass | `runs_residual_pass()` |
| 2 | same | token `_micro` | residual iteration budget | 3 passes | `residual_max_passes()` |
| 3 | same | token `targeted` | residual iteration budget | 3 passes | `residual_max_passes()` |
| 4 | same | token `playable` | residual iteration budget | 3 passes | `residual_max_passes()` |
| 5 | `flp.rs:residual_move_register_outliers` | token `_micro` | register move cap | score-based cap | `register_move_cap()` |
| 6 | same | token `targeted` | register move cap | score-based cap | `register_move_cap()` |
| 7 | `flp.rs:residual_remove_critical_spans` | token `_micro` | span-removal cap | score-based cap | `critical_removal_cap()` |
| 8 | same | token `targeted` | span-removal cap | score-based cap | `critical_removal_cap()` |
| 9 | `flp.rs:residual_remove_critical_jumps` | token `_micro` | jump-removal cap | score-based cap | `critical_removal_cap()` |
| 10 | same | token `targeted` | jump-removal cap | score-based cap | `critical_removal_cap()` |
| 11 | `flp.rs:apply_hand_trajectory_solver` | prefix `trajectory_` | trajectory pass admission | reject pass | `runs_trajectory_pass()` |
| 12 | same | prefix `trajectory_deep_safe` | trajectory iteration budget | 12 passes | `trajectory_max_passes()` |
| 13 | `flp.rs:apply_hotspot_reductions` | prefix `hotspot_` | hotspot pass admission | reject pass | `runs_hotspot_reduction()` |
| 14 | same | prefix `trajectory_` | hotspot pass admission | reject pass | `runs_hotspot_reduction()` |
| 15 | same | prefix `residual_` | hotspot pass admission | reject pass | `runs_hotspot_reduction()` |
| 16 | same | token `_micro` | per-hotspot cap | 8 | `per_hotspot_cap()` |
| 17 | same | prefix `residual_targeted` | per-hotspot cap | 8 | `per_hotspot_cap()` |
| 18 | same | prefix `residual_` | per-hotspot cap | 8 | `per_hotspot_cap()` |
| 19 | same | token `deep_safe` | per-hotspot cap | 8 | `per_hotspot_cap()` |
| 20 | same | prefix `trajectory_safe` | per-hotspot cap | 8 | `per_hotspot_cap()` |
| 21 | same | prefix `hotspot_jump_safe` | per-hotspot cap | 8 | `per_hotspot_cap()` |
| 22 | same | prefix `hotspot_playable_safe` | per-hotspot cap | 8 | `per_hotspot_cap()` |
| 23 | same | prefix `hotspot_span10` | per-hotspot cap | 8 | `per_hotspot_cap()` |
| 24 | same | prefix `hotspot_melody_bass_safe` | per-hotspot cap | 8 | `per_hotspot_cap()` |
| 25 | `flp.rs:reduce_hotspot_spans` | prefix `hotspot_melody_bass_safe` | notes retained per staff | 2 | `max_notes_kept_per_staff()` |
| 26 | `staff_voice_move_restify_atomic_patch_tail.rs:local_best_refinement_profiles` | complete display name | derive four numeric limits | `(12,18,60,60)` | `local_refinement_basis()` |
| 27 | same:`critical_micro_repair_profiles` | complete display name | derive four numeric limits | `(12,18,58,60)` | `critical_micro_basis()` |
| 28 | same:`parse_profile_parameters` | `s<digits>` | span seed | whole parse fails | `ProfileParameters::span_limit()` |
| 29 | same | `j<digits>` | jump seed | whole parse fails | `ProfileParameters::jump_limit()` |
| 30 | same | `lh<digits>` | left-register seed | whole parse fails | `ProfileParameters::left_high_soft_limit()` |
| 31 | same | `rl<digits>` | right-register seed | whole parse fails | `ProfileParameters::right_low_soft_limit()` |
| 32 | same:`parse_named_number` | arbitrary key prefix | authorize numeric suffix interpretation | `None` | one-time projected/unprojected construction |
| 33 | `pipeline_orchestrator_report_compaction.rs:run_arrangement_solver_stage` | prefix `trajectory_` | guard-candidate exclusion | treat as non-trajectory | `CandidateLineageStep::Trajectory` |
| 34 | `hidden_chord_reduction_texture_rebuild_tail.rs:run_consensus_broken_gesture_extension` | prefix `gauss_chain_` | choose broken-gesture lineage/name | standalone broken gesture | `CandidateLineageStep::GaussChain` |
| 35 | same:`run_consensus_gesture_probe` | prefix `consensus_gauss_chain_broken_gesture_tail_` | tail already applied | add tail prefix | `CandidateLineageStep::Tail` |
| 36 | same | prefix `consensus_broken_gesture_` | combine Gauss and broken history | generic prefixing | `CandidateLineageStep::Combined` |
| 37 | same | replacement of `consensus_broken_gesture_` | manufacture combined state | generic prefixing | `with_broken_gesture()` plus display projection |
| 38 | same | prefix `consensus_gauss_chain_broken_gesture_` | combined state already present | generic prefixing | `CandidateLineageStep::Combined` |

Result after the refactor: **38 -> 0 behavior-relevant profile display-name reads**.

## Closed contract

`CandidateProfileKind` is exhaustive and maps by compiler-checked `match` to exactly one `CandidateProfileFamily` and one fixed trait set. The raw profile constructor is not crate-visible. This prevents arbitrary combinations such as targeted+deep-safe or melody-bass-safe without its declared profile kind.

`ProfileParameters` are carried as typed numbers. A private construction flag records whether legacy numeric parameters were projected into the old display label. It is set once by the catalog, is not recomputed from the label, and preserves the two historical parse fallbacks exactly after non-profile candidates overwrite the display text.

`CandidateLineage` is an opaque bit contract. It records imported, trajectory, hand-repaired, Gauss-gesture, Gauss-chain, broken-gesture, combined, and tail history. `Tail` can only be added after a typed combined history; otherwise the transition returns `None`.

## Module boundary

```text
src/pipeline_profile/
    mod.rs
    model.rs
    catalog.rs
    lineage.rs
    legacy_name_projection.rs
```

The profile catalog moved out of the shared 41-shard `include!` namespace. Immediate consumers import the small crate-visible surface explicitly. The remaining pipeline shards are deliberately unchanged structurally.

## Historical identity classification in the touched area

- Productive profile semantics containing an etude number, measure number, concrete pitch, or feedback origin: **0**.
- The former `Etüde-Nr.-6-Sonderfälle` wording in the directly touched profile orchestration comment is neutralized to `projektspezifische Sonderfälle`.
- Generic `target_measure` fields in adjacent solver/notation code are structural locator vocabulary, not profile identity and not changed by this take.
- Existing etude labels in the neighboring lower-dual-edge file occur only in regression tests. That file is touched solely to call the typed span getter; the tests and their expected generic behavior remain unchanged.
- `write_sister_feedback_beam_checkpoint_authorized` is an existing writer/report compatibility identifier outside the profile block. It neither constructs nor selects a profile and remains outside scope.
- Identity-debt scanners, baselines, and security pins are unchanged.

## Parity and metamorphic proof surface

Rust tests added inside the module cover:

- display-name mutation with identical kind/parameters across all profile policy methods and threshold edges;
- one unique typed object for each of the 248 representative currently catalog-produced legacy display names;
- exact representative legacy name projection;
- fail-closed tail lineage;
- preservation of imported history through later transformations.

Independent static parity simulation against the v0.10.1650 string policy checked 250 representative profiles (248 currently catalog-produced plus the two historical trajectory kinds), 2,000 policy vectors, and 500 parameter/fallback vectors. Result: **0 mismatches**.

The take intentionally does not claim a build, Clippy, corpus, MusicXML, or PDF proof from the packaging environment. Those gates remain mandatory during the one-command installation run.

## Required acceptance run

```powershell
.\run.cmd --take --strict --all
```

Acceptance remains the full contract from the task: strict build/Clippy green, zero identity debt/external surfaces, 38 -> 0 semantic name reads, complete legacy mapping, display-name neutrality, and full ten-project MusicXML/PDF/candidate/budget/mutation parity.
