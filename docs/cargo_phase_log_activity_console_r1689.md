# Cargo phase and log-age hints during an interactive run

Parent: natively accepted r1688. ChangeClass: KodexOnly.
This is a read-only console projection, not a build, test, process or music rule.

The launcher already has one in-place running line with elapsed time. This take
adds the last observed Cargo phase and the age of actual output in the current
log. It covers launcher-owned Cargo commands and the residual test-prewarm wait.
The initial synchronous run.cmd builds before the Rust launcher starts keep their
existing display. No shell helper, additional command or external dependency is
introduced.

Only recognized Cargo messages select the display labels for compilation,
checking, Cargo's file-lock wait, test execution or completed preparation.
A living child and a spinner do not establish CPU activity. Log age comes from
the file's last modification time, not from when the reader happened to inspect
it; in particular the first prewarm observation cannot make old output look new.
A future or unavailable timestamp omits age. Missing or unreadable logs remove
information and cannot fail or finish a command. Raw log strings and escape
sequences are not printed to the terminal; supported SGR colour is parsed only.

The shared command reader starts at the current log end before spawning its
child, so earlier commands cannot supply its phase. The prewarm reader starts
at zero only in the separate file just truncated by that same prewarm start.
Reads and pending lines are bounded. Partial lines wait for a newline; oversized
or invalid messages cannot invent phases. Observed truncation discards earlier
phase and pending data. Polling is restricted to interactive in-place progress.
Redirected/line-mode output keeps its existing start and completion lines.
All extra hints use the existing running-line width budget and muted tail.

Command arguments, environment, spawn order, scheduling, polling interval,
child exit statuses, tests, validation receipts, cache decisions and release
rules remain unchanged. Child completion remains authoritative. No project
status or log wording grants a successful build or test result. The reader
writes no log, report, timing receipt or musical file.

Synthetic tests cover real phase messages, nonverdict/unknown messages, colour,
partial/oversized/invalid input, shared-log boundaries, truncation, prewarm log
age, unavailable logs, clock reversal and width limits without sleeps. The native
acceptance must preserve all twenty XML documents and all PDF page pixels.

The existing launcher byte pin is synchronized by its original trust-anchor
owner to the final launcher source. The same exact-byte assertion remains
active; no admission or validation condition is relaxed.
