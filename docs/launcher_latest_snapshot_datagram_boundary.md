# Launcher Latest Snapshot Datagram Boundary

Version: 0.10.1727

## Scope

This SecurityOnly take attaches the already accepted launcher `SourceStateOwner` to the
same best-effort loopback `SnapshotDatagramPublisher` already used by Project and Corpus.

The shared Observation Domain, `SourceSnapshotV1` protocol, UDP transport implementation,
Project instrumentation, Corpus instrumentation, music pipeline, writers and rendering
remain unchanged.

## Runtime path

```text
Launcher SourceStateOwner
        |
        | semantic transition
        v
SourceSnapshotV1 projection
        |
        v
SnapshotDatagramPublisher
        |
        v
127.0.0.1:47621 / UDP
```

The publisher is created from the initial Launcher `Preparing` state. Every existing
semantic launcher transition publishes the resulting full snapshot:

- `Preparing` is announced by publisher creation.
- `Building`
- `LauncherTests` when actually entered.
- `RunningTarget`
- `Finalizing`
- terminal `Completed` or `Failed`

`Launcher::set_phase()` remains presentation-only and receives no observation authority.

## Failure isolation

The transport contract is inherited unchanged from v0.10.1726:

- loopback only
- nonblocking UDP
- no receiver dependency
- send/socket failure is not a launcher or musical failure
- only latest snapshot retained
- periodic same-revision reannouncement
- no event queue
- no MQTT
- no Watch
- no external writer or command path

The launcher also continues to propagate the same `ObservationSessionId` to its child
Project or Corpus process. The new datagram publication therefore makes the existing
root source externally visible without changing session identity or child semantics.

## Change-control boundary

Only the pinned launcher Security source and its mandatory pin owners change
productively. The Observation Domain, protocol and transport files are byte-identical
to v0.10.1726.
