# Launcher Observation Domain Closure

## Scope

This is the non-Security bridge between Take 1A and launcher integration. The existing
change controller classifies `src/observation.rs` as `OtherSource` and the launcher as
`Security`, so they cannot legally change in one take. This take extends only the shared
transport-neutral Observation Domain. No launcher file, transport, console, writer, music
rule, MusicXML, PDF, report, or release authority changes.

## Added launcher runtime domain

`SourceRole` now contains `Launcher`. `LauncherStage` contains only the real coarse action
spine needed by the already mapped launcher boundaries:

- `Preparing`
- `Building`
- `LauncherTests`
- `RunningTarget`
- `Finalizing`

Terminal success/failure remains represented by `SourceStatus`, not duplicated as stage
variants. `SourceStage` therefore gains `Launcher(LauncherStage)`.

## Session root semantics

`ObservationSessionContext::launcher_new()` always creates a fresh root session with:

- `SourceId = launcher`
- no parent
- `SourceRole = Launcher`

The launcher context can derive exactly the immediate child contexts needed by the next
take:

- corpus target -> `corpus`, parent `launcher`
- direct project target -> `project:1`, parent `launcher`

A corpus context continues to derive `project:<index>` children exactly as in v0.10.1721.
`CorpusRunId` remains completely separate and untouched.

## Revision semantics

`SourceStateOwner::launcher()` starts at Revision 1 and `Preparing`.
`enter_launcher_stage()` increments Revision only when semantic state changes. Re-entering
the same launcher stage leaves Revision unchanged, preserving the invariant:

`same SourceId + same Revision => same SourceState`

Focused unit tests cover launcher root identity, child session inheritance, parent/source
addresses, and launcher revision idempotence.

## Mechanical cleanup

Because `src/observation.rs` is already the only productive source file in scope, the three
new Deep-Clippy redundant-closure observations introduced in v0.10.1721 are removed in the
same file. This changes no behavior.

## Next take

The following take is SecurityOnly and changes the launcher side only. It will import this
completed shared domain, own exactly one launcher `SourceStateOwner`, advance it at the real
build/test/target/finalization actions, and pass the root ObservationSession context to the
child target process. `Launcher::set_phase()` remains presentation-only.

UDP, snapshots, MQTT and Watch remain out of scope.
