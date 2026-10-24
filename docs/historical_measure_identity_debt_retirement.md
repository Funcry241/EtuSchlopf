# HistoricalMeasureIdentityDebtRetirement

Version: v0.10.1615 r01
Change class: RefactorOnly

## Purpose

This take removes historical absolute-measure identity from productive type,
function, variable, file and writer-ledger names without changing selection,
mutation, pass order or report content.

The affected operators were already scorewide and structural:

- metric lower-lane beam continuity formerly carried `M10` in local names,
- written-register octave-shift suppression formerly carried `M39`,
- a retired always-false `M33` audit still occupied the PDF-source evidence
  ledger.

## Changes

- rename the metric-beam audit, resolution, helpers and report projector to
  `ScorewideMetricBeamContinuity*`,
- rename the written-register octave-shift audit and helpers to structural
  register/octave terminology,
- rename the containing phrase/slur/register shard accordingly,
- remove the always-false retired M33 audit and its inert evidence-ledger row,
- remove exact historical measure wording from productive comments.

## Invariants

- candidate detection is byte-for-byte equivalent apart from identifiers,
- no condition, threshold, ordering key or XML edit changes,
- no new writer root or external write surface,
- report section headings and effective numeric values remain unchanged; only historical fallback labels are renamed,
- Master MusicXML, Render MusicXML and PDF remain unchanged after version
  normalization,
- no project, etude, measure, pitch or locator selector is introduced.

## Expected verification

One target-system run is sufficient:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Expected: build/clippy/final 0/0/0, zero warnings, MusicXML only differs in the
software version and all six PDF pages are pixel-identical to v0.10.1614.
