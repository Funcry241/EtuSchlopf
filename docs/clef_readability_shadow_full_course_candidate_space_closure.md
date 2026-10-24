# ClefReadabilityShadowFullCourseCandidateSpaceClosure

## Purpose

Close the last semantic Take-2 gap before any productive clef cutover.

v0.10.1712 proves that the same writer-free readability policy can observe both Master MusicXML and the independently existing render MusicXML artifact. The corpus also proves that the previous shadow is still constrained by legacy clef surfaces: it can remove an existing F4/G2 surface, but it cannot propose a missing transition when the legacy local `>3` logic did not already create one. That means the previous shadow is a global selector over a legacy candidate set, not yet a complete clef-course owner.

## Full candidate space

The shadow now derives writer-free lower-staff attack groups from the shared clef observation. For every attack group it compares raw F4 and G2 ledger pressure. A new transition candidate is created only at the start of a maximal run whose lower-pressure clef differs from the preceding non-tied run.

These register-preference-run boundaries are sufficient for the current additive ledger model: inside one maximal run, moving a chosen transition toward the start of the run cannot increase ledger pressure while the number of clef transitions stays unchanged. No `>3` threshold is used to create the candidate.

Existing removable clef surfaces remain candidates as well. The shadow can therefore:

- retain an existing clef surface;
- omit a removable existing clef surface;
- propose a new pure-clef insertion at a register-preference-run boundary.

It still writes nothing.

## Readability cost scale

Opening the candidate space makes the historical productive Rustig unit transition cost (`1`) unsuitable as an editorial shadow cost: with transitions available at genuine musical boundaries it underprices clef churn.

The shadow therefore uses one explicit reference scale:

- isolated four-ledger event: `24` pressure units;
- one clef transition: `24` readability units;
- a return to the previous clef after at most two lower-staff attacks: one additional `24` short-excursion cost.

Four ledger lines remain allowed. The number `24` is a common comparison unit, not a ledger-line prohibition. The productive Rustig planner retains its historical unit transition cost in this Take.

## Generic proofs

The existing Golden Cases remain and the shadow adds two missing-candidate proofs:

- a sustained high passage with only an initial F4 source surface may produce `F4 > G2 > F4` by shadow insertions;
- one isolated B4 attack under F4 remains `F4`, so opening the candidate space does not turn the four-ledger reference case into automatic clef churn.

## Not changed

- audit-only;
- no MusicXML mutation;
- no PDF mutation;
- no productive Rustig cutover;
- no legacy local `>3` removal;
- no sustained-preference removal;
- no writer or materialization change;
- no Etude, measure or pitch selector.

The full 10-project corpus is required again. Take 3 remains blocked until the expanded shadow shows a stable, explainable course inventory.
