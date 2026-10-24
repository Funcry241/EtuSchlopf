# Fresh early launcher failure evidence

ChangeClass: KodexOnly
AcceptedParent: r1361
AcceptedParentSha256: c7d068aac379e7c533c109b5272b983984166a4f225502cd6ad87533a61d4e6a
MusicalAuthorityChange: none
CachePolicyChange: none
SelectorChange: no

The rejected r1366 managed source and restored r1361 were byte-verified. Its
child returned 101 before a launcher marker. The exported console was identical
to the previous attempt, so it establishes neither a fresh course failure nor
a compiler cause. The missing early launcher Cargo message remains unknown.

This separate KodexOnly cut changes startup/export observation only. All musical
sources, selectors, SSC boundaries and locks remain byte-identical to accepted
r1361. Rejected resource and course instrumentation is not mixed into this cut.

The bootstrap removes only the shared derived child console before starting a
child. Missing files are allowed; other IO errors prevent starting that child
through the existing reexec error/rollback path. The collector requires a stage
record in the current attempt directory. A shared console without it is marked
not_current_attempt. The flat failure projection uses the captured console in
the current rejection directory rather than blindly reading the shared log.

For the first transition from the older accepted parent, the incoming run.cmd
truncates the shared console before compiling the launcher, writes the inherited
current evidence-directory binding and startup invocation, and records the early
stage. It embeds the actual Cargo stdout/stderr and saved exit code before the
original failure branch. The older parent's existing stage/console collector
can read these existing protocol fields without an unsupported registry role.
The native launcher still claims its own writer PID and stage after handoff.
No stage value or diagnostic field provides musical or acceptance authority.

Cargo commands, incremental/cache settings, test selection, gates, exit handling,
rollback and recovery rules remain intact. No retry, force or compiler workaround
is introduced. This does not prove a fix for the startup or earlier music failure.

Two portable tests use the actual console supersession and collector, checking
absent current logs, byte-exact early compiler capture, stage evidence and IO
refusal. A Windows-only test compiles a small synthetic Cargo executable that
prints an error and returns 101, executes the actual incoming run.cmd and checks
that the current early failure survives capture and the prior log is excluded.
The Windows test is cross-checked locally; native Windows execution is pending.

Required production Clippy targets, the full test suite and optimized binaries
are checked. Unchanged main-test vectors have a separate Rust 1.98 all-target
Clippy debt; their refactor cannot be mixed into this KodexOnly take. No lint is
suppressed. No complete green corpus or all-PDF success is claimed. The working
budget remains exceeded and no new final-take count is promised.

User entry: ./run.cmd --take --all
