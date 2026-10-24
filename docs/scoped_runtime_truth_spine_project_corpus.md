# Scoped Runtime Truth Spine – Project + Corpus

## Scope

This is Take 1A of the Observability Session Architecture. It adds only the in-process
runtime truth spine for the productive project pipeline and the corpus scheduler.
There is no UDP, MQTT, Watch application, serialization, socket, background thread,
console output, artifact write, music rule change, or music mutation in this take.

The launcher is intentionally not changed here. The installed take change controller
classifies `src/bin/etuschlopf_launcher.rs` and its modules as Security paths, while the
project/corpus runtime lives outside that role. A single RefactorOnly take therefore
cannot legally change both sides. The architecture is unchanged; implementation follows
the existing security boundary as Take 1A (Project + Corpus) and Take 1B (Launcher).

## Scoped Runtime Truth Map

### Project authority

The productive project orchestration remains `real_main()` in
`src/pipeline_shards/pipeline_orchestrator_report_compaction.rs`.

The six existing natural stage boundaries are the observation authority:

1. `run_project_run_preparation_stage` -> `ProjectRunPreparation`
2. `run_arrangement_solver_stage` -> `ArrangementSolver`
3. `run_master_notation_and_salience_stage` -> `MasterNotationAndSalience`
4. `run_initial_render_transaction_stage` -> `InitialRenderTransaction`
5. `run_metric_beam_and_final_notation_stage` -> `MetricBeamAndFinalNotation`
6. `run_release_report_and_exit_stage` -> `ReleaseReportAndExit`

No lower-level function becomes an Activity merely because it exists. Existing
`console_progress(...)` calls remain narration and have no observation write authority.

### Corpus authority

The corpus authority is `run_all_batch_orchestrator()` in
`src/pipeline_shards/rest_glyph_retention_batch_corpus_tail.rs`.

The semantic scheduler boundaries are:

- plan created -> `PreparingProjects`
- child process successfully spawned -> `RunningProjects`
- child `try_wait()` reports completion -> `CollectingProjectResult`
- remaining work resumes -> `RunningProjects`
- corpus report/final manifest begins -> `FinalizingCorpusReport`
- final corpus result -> `Completed` or `Failed`

`BatchProjectProgress` and `BatchProgressSnapshot` remain presentation-friendly
projections of already existing counters. `batch_progress_frame()` and
`batch_visual_heartbeat()` remain presentation only and do not change SourceState or
Revision.

### Launcher authority map for Take 1B

No launcher code changes are made in this take. The real action boundaries identified for
Take 1B are:

- build materialization in `materialize_launcher_build()` / `run_cargo_build()`
- integrated launcher tests in `run_integrated_launcher_tests()`
- target start at `invoke_logged_program*()`
- target exit from the returned `RunResult`
- final outcome at the target-specific completion path and `print_summary()`

`Launcher::set_phase()` remains presentation-only. Its `effective_phase()` and
`final_blue_seen` behavior are console color state, not runtime authority.

## Observation domain

`src/observation.rs` contains the transport-neutral runtime types:

- `ObservationSessionId`
- `SourceId`
- `ObservationSessionContext`
- `SourceRole`
- `SourceStatus`
- `ProjectStage`
- `CorpusStage`
- `SourceStage`
- `SourceProgress`
- `SourceState`
- `SourceStateOwner`

There is no global sequence. `Revision` is monotonic only within one SourceState owner.
Re-entering the identical semantic state leaves Revision unchanged.

## Session propagation

Until the launcher becomes the session root in Take 1B, a direct project or corpus run
creates a local ObservationSessionId if none is inherited. The corpus process passes the
same session ID to every child process through private environment context:

- `ETUSCHLOPF_OBSERVATION_SESSION`
- `ETUSCHLOPF_OBSERVATION_SOURCE`
- `ETUSCHLOPF_OBSERVATION_PARENT_SOURCE`

A corpus child receives `project:<index>` as SourceId and `corpus` as ParentSourceId.
When Take 1B is installed, the launcher will provide the root session context instead;
Project and Corpus already consume it without changing their domain model.

`CorpusRunId` is not read, written, derived, reused, or changed by this runtime context.

## Revision semantics

The owner starts at Revision 1. A revision increments only if one of these semantic state
fields changes:

- Status
- Stage
- Progress

Session identity, Source identity, role, parent relation, and project label are immutable
for the lifetime of the owner.

Therefore:

`same SourceId + same Revision => same SourceState`

Unit tests cover unchanged-state re-entry, stage changes, terminal completion, corpus
progress changes, and session/source inheritance for corpus children.

## Failure isolation

Observation state updates allocate only small in-memory values and read inherited
environment context. No transport exists in this take. No observation operation writes a
music artifact, report, manifest, PDF, MusicXML, rule, or release decision.

The project orchestration marks the local SourceState failed before returning any existing
stage error. The normal terminal exit marks it Completed for exit 0 and Failed otherwise.

The corpus scheduler marks normal final exit Completed or Failed after the existing corpus
outcome calculation. Existing scheduler, output validation, manifest, and report behavior
remain authoritative and unchanged.

## Next scope

Take 1B adds only the Launcher SourceState root and session propagation into the target
process. It remains a separate SecurityOnly take because that is the repository's existing
change-control boundary.

UDP, `SourceSnapshotV1`, reannouncement, Watch, MQTT, and UI remain out of scope until
Take 1 is complete.
