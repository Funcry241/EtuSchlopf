# r1513 - LowerClefBeamInteriorContextDefinition

Parent: accepted r1512, candidate SHA-256
ea977b4a07e176a72f046b256cb0a4258df678241f32ecd491822ff05c5ed3e8.
Source: EtuSchlopf_v0_10_1839_r1512_unbuilt_source.zip from the
20261025-112259 assistant export. ChangeClass: KodexOnly.
Status: Stage-A candidate; native acceptance/build/clippy/tests/corpus NOT RUN.

## Result of the investigation

The earlier suggestion that the clef planner merely needs more lookahead was too
broad. solve_readability_course already uses a whole-part dynamic program.
scan_lower_pitches_with_beam_audit combines tie and primary-beam origins;
collect_readability_transition_candidates sums all attacks sharing an origin
before generating candidates. An internal preference change therefore need not
reach the optimizer as an independent boundary at all. More lookahead or a local
pressure-weight adjustment cannot by itself restore that missing alternative.

The same rule is protected in solver admission, insertion normalization and
postwrite validation. The existing policy intentionally forbids lower clef
surfaces inside an open primary beam; this is not a newly discovered accidental
omission in just one function. The accepted earlier scope and chord contracts
do not authorize changing this policy.

## Concrete output readback (evaluation only)

The following is final-document evidence, not a replay of transient productive
planner inputs and not a governed class-distribution measurement.

| Output | Observed sequence | Consequence |
| --- | --- | --- |
| Etude 6, bar 22, lower | F-sharp5 tie starts at 264/96, continues at 288/96 on beam begin; F-sharp2 at 336/96 is beam continue; C-sharp3 at 360/96 is beam end | The desired fresh low-note boundary lies inside an existing primary beam, and that beam inherits the earlier tie/beam origin |
| Etude 7, bar 6, lower | G-sharp4/D5 tie starts at 216/96; continuation at 288/96 is beam begin; F-sharp3 at 336/96 is beam continue; E-sharp3 at 360/96 is beam end | Again, a fresh low-note boundary is internal to a beam linked to a preceding tie |

The divisions are 96 per quarter; values above are measure-relative positions in
that unit. The final Render BeamAnchor sections independently report the low
tails sharing earlier effective anchors (Etude 6: notes 578/579 -> 574;
Etude 7: notes 97/98 -> 91). Other report sections describe other artifacts and
have different note ordinals; they must not be conflated with the Render rows.

Final PDF inspection confirms six/four ledger lines for the Etude-6 low pair
and three/four for the Etude-7 pair in treble clef. A bass clef before those
pairs is a proposed visual remedy, not a verified released course. No corrected
PDF, native prewrite replay, new selector distribution or writer success is
claimed. The shared final-state pattern does not prove identical earlier
execution histories.

## First deliverable and why it is separate

This take freezes only
docs/selector_derivation_contract_lower_clef_beam_interior_context.md, records the
investigation and advances the candidate metadata. No musical Rust source, test,
dependency, gate or existing error-class contract changes.

src/take_change_control.rs requires an error_class_definition in an accepted
parent before any selector-sensitive classification or repair references it.
Adding the definition and its implementation together would violate that rule.
The new contract defines missing internal boundary context independently of
known identities, excludes hard ledger/jump thresholds, retains UNKNOWN and
separates class membership from safety and repair permission.

## Next implementation boundary after native acceptance

1. Read the actual clef-planner input for the relevant productive call, using
   existing input-bound trace facilities; distinguish it from final rescan.
2. Prove an independent fresh-attack boundary with complete group/chord/tie and
   display-scope evidence. Compare against the entire frozen baseline with the
   unchanged cost and surrounding clef carry. Do not assume two low notes win.
3. If a clef-only alternative is justified, reconcile the existing owner layers
   with the same proved boundary policy. Keep all original musical relations and
   receipt/adoption obligations; no late PDF fix and no beam splitting.
4. Execute independent synthetic/metamorphic tests, actual writer/adoption and
   committed reentry checks. Then evaluate all ten real pieces and inspect every
   changed PDF page. If any prerequisite is unresolved, retain the baseline.

Repeated 8va continuation labels are a separate display topic and remain out of
this take. The lower-domain octave/clef comparison left unresolved by r1512 is
also not implicitly solved by this definition.

## Apply and expected effect

Place the full managed-tree take ZIP in the usual take location and run:

    .\run.cmd --take --all

Expected: native admission or a fail-closed diagnostic, unchanged musical XML
and pixel-equivalent PDFs. PDF metadata may differ. Acceptance is determined
only by the native hash-bound lifecycle record; it is not claimed here.
