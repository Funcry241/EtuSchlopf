# r1063 RendererExplicitCliRejectImmediateFallbackClosure

## Change class

`RefactorOnly`

## Parent

Accepted `v0.10.1839 r1062 BatchChildFailureStderrVisibilityClosure` is required as the parent. If r1062 is not canonically accepted in the local transition provenance, this candidate must not become the next parent and must fail closed.

## Problem

The renderer process is already synchronous at this boundary: `run_renderer_command()` returns only after the renderer process has ended. The existing failure classifier can then prove `cli_option_rejected` from explicit MuseScore parser diagnostics such as `Unknown options: s, w`, `Unknown option 's'`, or `Unknown option 'w'`.

Before this take, the compatibility loop still applied the generic three-second nonzero/missing-output settle window before consulting that already available failure class. In the current full corpus, Etudes 1, 2, 3, 6 and 7 each encounter three such explicit parser rejects before the successful renderer command. Those fifteen already-final parser rejects therefore pay about 45 seconds of aggregate project wait although that invocation never entered conversion.

## Closure

Immediately after each completed renderer process the already existing `renderer_failure_class_v1751()` classification is computed once.

A new narrow predicate permits immediate compatibility fallback only when all three conditions hold:

1. the process has a concrete nonzero exit code;
2. no non-empty PDF exists at process end;
3. the diagnostic class is exactly `cli_option_rejected`.

For that conjunction only, the PDF settle loop is not entered. The attempt is represented as an immediate missing-output failure with zero settle samples and zero settle milliseconds, then the existing compatibility loop proceeds to the next already existing variant.

## Explicitly preserved behavior

- Generic nonzero failures retain `renderer_post_process_settle_timeout_v1750()` and its three-second grace.
- Exit `1320` with `no_diagnostic_text` is unchanged.
- Exit zero without an immediately visible PDF retains the historical confirmation window.
- A PDF already present at process end retains the historical confirmation path even if diagnostics also contain CLI-like text.
- An absent numerical exit code does not qualify for the immediate fallback.
- `render_command_variants()` is unchanged: no variant is removed, reordered, or newly introduced.
- `-f` is not promoted.
- Renderer discovery, shadow winner reuse, executable validation and renderer success criteria are unchanged.
- No MusicXML, render MusicXML, pitch, onset, duration, staff, voice, beam, tie, rest, clef, layout, writer, selector, solver, candidate-width, release, receipt, gate, or fail-closed authority changes.

## Regression coverage

Three focused tests prove:

1. explicit nonzero `cli_option_rejected` without PDF qualifies for immediate fallback;
2. generic nonzero/no-diagnostic failure does not qualify;
3. existing PDF, exit zero, or missing numerical exit each prevents the bypass even with CLI-reject text.

The r1062 source snapshot contains 1,387 passing tests. With these three added tests and no test retirement, the expected count is 1,390 if all existing test inventory remains otherwise unchanged.

## Expected corpus evidence

For the five currently affected final-renderer projects, if the observed failure pattern remains identical:

- `VariantsAttempted` remains `8/11`;
- `FailedBeforeSuccess` remains `7`;
- `NonzeroMissingOutputShortWaits` should fall from `7` to `4`;
- `FailureEvidence` still records the three `cli_option_rejected` failures;
- `SettleMsBeforeSuccess` should fall from about 21 seconds to about 12 seconds per affected project;
- the successful command and final renderer artifact remain unchanged.

Because two projects run concurrently, the approximately 45 seconds are aggregate project wait, not a guaranteed 45-second wall-clock reduction. Acceptance must be based on the actual full-corpus run and phase evidence, not on that arithmetic alone.

## Required run

```powershell
.\run.cmd --take --all
```

The take is acceptable only if the normal gates remain green and the full corpus remains 10/10 with unchanged musical/render artifacts and renderer success criteria. If any generic nonzero path loses its grace window, any variant order changes, or r1062 is not the canonical accepted parent, reject fail-closed.
