# r1062 BatchChildFailureStderrVisibilityClosure

## Change class

`RefactorOnly`

## Parent

Accepted `v0.10.1839 r1061`.

## Problem

The full-corpus `--all` owner starts each project as a child `etuschlopf.exe`. The parent deliberately captures a narrow stdout projection, but the child's stderr is currently connected to `Stdio::null()`. A project can therefore fail before it writes its normal `.result.txt` while the only diagnostic that explains the failure is discarded. The corpus owner then has only an exit code and can create only a generic `BATCH-PROJEKT-FEHLER` wrapper.

This made the rejected primary-beam materialization candidates observable only as ten project exits, not as their concrete fail-closed runtime cause.

## Closure

The child process keeps its existing stdout contract unchanged. Its stderr is changed from `Stdio::null()` to `Stdio::inherit()`.

The inherited handle is already inside the existing launcher process/log boundary, so the child's existing stderr diagnostics become part of the normal `etuschlopf.console` evidence without adding a new file sink, report reader, selector, writer authority, music mutation, or recovery path.

## Explicit non-changes

- no MusicXML or PDF mutation
- no beam, voice, staff, pitch, onset, duration, tie, rest, clef, or layout rule change
- no project-selection or corpus-selection change
- no concurrency or worker-budget change
- no stdout-capture change
- no new child log file
- no new writer root or productive write surface
- no proof/report reentry
- no change to exit-code semantics or fail-closed behavior

## Acceptance

A failing child project must still return the same nonzero exit and keep the corpus red, but its pre-result stderr diagnostic must now be visible in the parent launcher's existing console log. Successful project behavior and all generated musical artifacts must remain unchanged.
