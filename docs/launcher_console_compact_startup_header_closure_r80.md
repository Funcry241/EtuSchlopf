# Launcher Console Compact Startup Header Closure r80

## Parent

Real committed parent: EtuSchlopf 0.10.1839 r79 `LauncherConsoleSegmentRoleAccentFinalClosure`.

## Scope

This SecurityOnly take closes one concrete interactive presentation defect left after r79: the startup header is substantially wider than the surrounding launcher UI because it repeats job information and exposes the technical log path inline.

No terminal-width detection, wrapping heuristic, new capability source, dependency or adaptive renderer is introduced. The defect is removed at its source by reducing the interactive information set.

## Interactive startup header

Before, the launcher projected all of the following in one interactive line:

- project/target;
- logical system threads;
- `CargoJobs`;
- `PipelineAutoHint`;
- `Cargo --jobs`;
- console-log path.

The new interactive projection is intentionally compact:

`[EtuSchlopf v<version>] <target> · System <logical_threads> · Jobs <cargo_jobs>`

`CargoJobs` and `Cargo --jobs` represented the same effective launcher-owned Cargo job count and therefore were redundant in the human-facing header. `PipelineAutoHint` remains technical scheduling evidence rather than startup narrative. The console-log path remains available in the final `Result` projection and the full thread plan remains in the technical console log.

## Regression contract

A focused launcher test pins the representative 20-thread / 19-job header and explicitly rejects reintroduction of `CargoJobs`, `PipelineAutoHint`, `Cargo --jobs` and `Log:` into that interactive startup projection.

## Security / authority preservation

The launcher is a pinned security-control-plane surface, so its fingerprint and the dependent identity-build trust anchor are rebound explicitly. No Security capability is retired and no scanner is bypassed.

## Non-goals

No change to the shared Presentation/RenderPolicy contract, `TerminalCapability`, stdout/stderr capability resolution, `NO_COLOR`, redirect behavior, progress rendering, child routing, Take bootstrap, Take transaction semantics, final `RunOutcome`, `ReleaseDecision`, `RunSeverity`, DARK GREEN authority, music, MusicXML, PDF content, renderer, planner, writer, artifact payloads, Assistant Export, Neuro Evidence or `--all` audit evidence.

In particular, r80 does **not** claim that redirect, `NO_COLOR`, warning/error or rollback edge cases are newly validated. Those remain real-run acceptance topics rather than speculative code changes.
