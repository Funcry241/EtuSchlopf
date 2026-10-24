# r1028 — RegisterNotationOctaveKindBoundaryEvidenceAudit

## Parent and change class

- version: `0.10.1839`
- logical parent: accepted `r1027` / r02 freeze
- change class: `AuditOnly`
- music mutation: no
- master apply: no
- render write: no
- repair selection authority: none

r1028 does not broaden the r1027 Same-Lane repair and does not change any notation. It binds a new visual/register question to final Render-MusicXML evidence before any later repair decision is allowed.

## New bound product case

The current Etüde-4 PDF shows a register-course boundary around T22–T25 that deserves classification before any octave or clef rewrite:

- the 8va course that starts in T22 remains active for exactly the first visible pitched attack of T23;
- it then stops and a 15ma course starts before another pitched attack on that staff;
- the 15ma course in turn remains active for exactly the first visible pitched attack of T25 before returning to 8va.

This is not declared musically wrong by r1028. The new audit proves the exact visible boundary geometry and asks whether this measure-leading one-attack carry is a real readability class or an intentional consequence of the sounding register.

## Audit definition

A `OneAttackCarry` row is emitted only when all of the following are observed in the final Render-MusicXML:

1. an octave-shift span starts in an earlier measure;
2. in the later boundary measure exactly one visible, pitched, non-grace, non-chord attack on that staff occurs before the span stops;
3. a new octave-shift span with a different `size` starts on the same part/staff/number before any further pitched attack;
4. both spans are closed in the final Render-MusicXML.

The rule is project- and measure-identity-free. It does not choose a replacement size, move a boundary, change a pitch, infer sounding pitch from the graphical shift, or grant write authority.

## Expected r1027-corpus evidence

The current frozen Render-MusicXML corpus predicts exactly two `OneAttackCarry` boundaries, both in Etüde 4:

```text
E4 / M23 / Staff 1 / Number 1
8 -> 15
PriorStart M22
NextStop M25
AttacksBeforeSwitchInMeasure 1
PriorSpanAttacks 17
NextSpanAttacks 31
WrittenPitchDeltaSemitones +4
OneAttackCarry ja

E4 / M25 / Staff 1 / Number 1
15 -> 8
PriorStart M23
NextStop M25
AttacksBeforeSwitchInMeasure 1
PriorSpanAttacks 31
NextSpanAttacks 9
WrittenPitchDeltaSemitones -16
OneAttackCarry ja
```

Expected per-project summary for Etüde 4:

```text
ClosedSpans 18
KindChangeBoundaries 3
OneAttackCarryBoundaries 2
```

Expected corpus-wide `OneAttackCarry` count from the r1027 freeze artifacts: `2`, both Etüde 4. The audit does not turn this uniqueness into repair authority.

## Report contract

A new section is appended to `.result.txt` after the existing Registernotation report:

```text
OCTAVE-KIND-BOUNDARY-EVIDENCE-AUDIT
```

It reports final Render-MusicXML read/parse status, closed octave spans, all adjacent different-size octave-kind boundaries without an intervening pitched attack, and the exact one-attack carry subset.

The section is evidence-only. It does not affect formal blocker/warning/hint counts, release decision, selection, MusicXML, PDF, or any writer graph.

## Tests

Two pure audit tests are added:

1. an E4-shaped synthetic 8 -> 15 boundary crossing a measure start is classified as `OneAttackCarry`;
2. an otherwise similar size change whose first span starts in the same measure is not classified as a measure-leading carry.

No productive writer is called by either test. Expected strict test cardinality from the r1027 baseline becomes `1334`.

## Acceptance

Early gate: `Etüde Nr. 4`.

Required:

- PRE-TAKE gate green;
- `cargo fmt --check` green;
- Clippy warning budget `0`;
- `1334/1334` strict tests green;
- Master MusicXML byte-identical to r1027;
- Render MusicXML byte-identical to r1027;
- PDF pixel-identical to r1027;
- only report/audit evidence changes;
- E4 audit reports `ClosedSpans 18`, `KindChangeBoundaries 3`, `OneAttackCarryBoundaries 2`;
- E4 contains the M23 `8->15` and M25 `15->8` one-attack rows above;
- normal full-corpus run reports no additional `OneAttackCarry` cases outside E4.

If the new audit changes product artifacts, creates any writer reachability, changes the formal release summary, or fails to reproduce the two expected E4 rows, r1028 is rejected.

## Decision boundary after r1028

Even if r1028 is fully green, no octave mutation follows automatically. A later take must still prove whether the E4 one-attack carry is visually inferior and identify the existing semantic owner that may change the register course while preserving sounding events and final product invariants.
