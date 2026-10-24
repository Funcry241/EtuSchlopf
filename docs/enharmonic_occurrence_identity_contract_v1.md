# Enharmonic occurrence identity

Contract: enharmonic_occurrence_identity_v1
Status: accepted musical policy definition by the project owner (2026-10-12);
no productive issuer, transition or writer permission is added by this document.
ChangeClass of the introducing Take: KodexOnly.

## Decision

A physical occurrence keeps its identity when only its spelling changes.
Spelling is a notation attribute decided by engraving, like stem direction,
beaming or clef. It is not occurrence identity, course membership or role.

An enharmonic respelling continues the same occurrence if and only if all of
the following are unchanged:

1. sounding pitch: the MIDI number computed from step, alter and octave
   (not the `<pitch>` text; B#3 = C4 = 60 and Cb4 = B3 = 59 change the octave text),
2. onset and duration (licensed support),
3. staff and course presentation segment,
4. rest/chord/grace disposition and tie marks.

## Musical rationale

On the piano, F#/Gb on the same key are the same sounding event. The source is
a recording/MIDI pitch number; spelling is derived by the pipeline from key,
harmonic function and voice leading. Editors respell for readability without
changing musical content. Recovering identity from spelling would therefore
encode an engraving choice as musical substance.

## Engraving constraints that belong to the rule

- Tied notes are spelled alike. A respelling applies to the complete tie chain.
  A spelling change inside a tie chain is not continued by this rule; it stays
  Unknown unless a separate explicit transition authorizes it (e.g. a key change).
- One key is not written twice with different spellings within one chord.
- A change of sounding pitch is never a respelling. Owners such as the harmonic
  pitch swap (population row 8) need their own explicit transition rule.

## Boundary

This contract defines what counts as the same occurrence for custody
continuation. It does not select a spelling, authorize a respelling writer,
change the functional respelling owner (row 60) or the Master export (row 59),
and it does not license any other disposition change. Implementations must
compare computed MIDI numbers and must reject any other difference.
