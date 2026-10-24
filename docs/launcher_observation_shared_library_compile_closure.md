# Launcher Observation Shared Library Compile Closure

## Failure proved by r02

The failed v0.10.1722 domain attempt proved that a launcher-only domain pre-stage cannot live
as an unused private module of the `etuschlopf` music binary. `src/main.rs` deliberately
uses `#![deny(dead_code)]`, so `SourceRole::Launcher`, `LauncherStage`, launcher context
constructors and launcher state transitions correctly failed compilation while no launcher
hook consumed them yet.

This is not repaired with `#[allow(dead_code)]` and not with fake calls from the music
pipeline.

## Closure

`src/observation.rs` is now compiled once as the package library surface:

```text
src/lib.rs
    -> pub mod observation
```

The main music binary imports that same package module instead of privately compiling a
second module instance. The existing Project and Corpus observation call sites remain the
same semantically; only the module ownership changes.

This gives the following dependency shape:

```text
etuschlopf library
    observation
       ^        ^
       |        |
   main bin   launcher bin (next SecurityOnly take)
```

The next SecurityOnly launcher take can therefore import exactly the same
`etuschlopf::observation` types without modifying `src/observation.rs`, without duplicating
launcher DTO/state semantics, and without weakening the music binary's dead-code policy.

## Scope

In the corrected r04 package this compile closure is delivered together with the launcher-domain delta directly from v0.10.1721. It changes no Observation semantics introduced by that domain delta and adds no
runtime writer, output, transport, socket, MQTT, Watch, console line, MusicXML mutation,
PDF mutation, rule, gate or release authority.

`CorpusRunId` remains untouched and separate from `ObservationSessionId`.

## Acceptance

The immediate proof is the same targeted strict command. The expected closure is that the
five r02 dead-code errors disappear while the existing Project/Corpus runtime observation
continues to compile and behave unchanged.
