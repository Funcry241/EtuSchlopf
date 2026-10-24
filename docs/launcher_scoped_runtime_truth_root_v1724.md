# Launcher Scoped Runtime Truth Root

Version: 0.10.1724  
Take: 1C of the Observability Session Architecture

## Purpose

This take connects the existing Rust launcher to the shared transport-neutral
`etuschlopf::observation` domain closed through v0.10.1723. It does not add UDP,
MQTT, snapshots, a Watch process, persistence, commands, or any music-facing
behavior.

The launcher now owns the root `SourceStateOwner` for each invocation. Its
initial state remains `Preparing`. Semantic transitions are emitted only at
real launcher actions:

- `Building` before build-plan materialization, including strict Clippy/build
  work or verified build reuse.
- `LauncherTests` only when the existing integrated launcher test gate actually
  runs.
- `RunningTarget` immediately before a real project, corpus, or visual-preflight
  target begins.
- `Finalizing` only after that target has returned and before existing cleanup,
  export, report, and summary finalization.
- `Completed` or `Failed` from the actual launcher outcome paths.

`Launcher::set_phase()` and all console color/animation code remain presentation
only and do not write observation state.

## Session hierarchy

The launcher creates a fresh `ObservationSessionId` and is source `launcher`
with no parent.

For a single project it starts the child with:

```text
session = launcher session
source = project:1
parent = launcher
```

For a corpus run it starts:

```text
session = launcher session
source = corpus
parent = launcher
```

The already existing corpus scheduler then creates `project:<index>` children
with parent `corpus`. `CorpusRunId` is unchanged and remains evidence identity,
not observation identity.

## Failure isolation

Observation state changes are in-memory only. There is no I/O, lock-bound
transport, queue, worker, socket, broker, file, or console output attached to
them. Therefore this take cannot add pipeline backpressure or a new musical
failure path.

## Verification target

The first acceptance gate remains one strict single-project run:

```powershell
.\run.cmd --take --strict "Etüde Nr. 6"
```

Required evidence:

- launcher and music binary compile under strict policy;
- launcher observation tests pass when the existing launcher test suite is run;
- final project outcome remains green;
- Master/Render MusicXML remain unchanged except for the software version;
- PDF rendering remains visually identical.

A full corpus run is not required merely to establish this launcher root. It
becomes relevant when the transport/failure envelope needs the real
multi-process neutrality proof.


## Architecture pin synchronization

`src/bin/etuschlopf_launcher.rs` is a compile-time pinned security surface. The take therefore updates its two required pin owners, `src/identity_debt_build.rs` and `src/identity_gate_trust_anchor.rs`, only to bind the new launcher fingerprint and the resulting build-policy fingerprint. No additional runtime capability is introduced by this synchronization.

## Call-graph name closure

v0.10.1723 renamed the shared terminal transitions to `mark_completed()` and
`mark_failed()` before this SecurityOnly binding. The names are unique within
the source tree and therefore do not collide with existing writer-reachable
`complete()` helpers in the security/visual-preflight code. No semantic
capability scanner exception or allowlist is introduced.
