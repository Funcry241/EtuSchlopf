# r1515 - LowerClefFreshBeamBoundary

Parent: accepted r1514,
d69b4895a41aed42c9a5d40b7eb1b9eb67affd9703adfbb56ff28e60edc5c762.
ChangeClass: MusicMutation. Native acceptance is pending.

## Resulting behavior

The existing clef owner can compare a fresh, originally resolved untied lower attack inside
a complete mixed-preference primary beam as a reading-reference boundary. An unchanged complete
baseline is retained and all its candidate choices remain in the extended
search. A replacement requires strict whole-part improvement under the unchanged
ledger, transition, redundancy and short-excursion costs.

The pure safety observer requires same-measure, same-voice, visible pitched lower
heads and resolved chord members, closed begin/continue/end structure, stable
duration units and cursor arithmetic, no internal state/cursor/octave fence, no
member tie and no conflicting lower temporal occupancy or other beam span. Existing tie
guards remain active. Unknown provides no new insertion permission.

This same observation supplies anchors, solver eligibility, normalization and
postwrite safety, including pure clefs immediately before a proved head. Notes,
beams, ties, staff/voice, sounding music, octave directions and writer/receipt/
adoption ownership are not changed by the clef materialization itself.

## Prior evaluation and limits

The 90 captured r1514 entries contain 55 distinct byte states. The unchanged
planner replay reproduced all 32 native detailed solution and candidate-list
occurrences. A fixed-baseline counterfactual then established that advancing an
existing bass return could reduce a complete course from 368 to 284 in one
Etude-6 entry and from 478 to 460 in one closed Etude-7 entry, without extra
transitions. These locations are evaluation addresses only.

Some intervening Etude-7 entries have an end under a different serialized voice
and therefore no complete group in the existing observation. They do not gain a
new interior boundary. Later closed entries can be evaluated independently; a
later PDF or equal pitches cannot repair the missing earlier relation.

The earlier r1513 report's statement that E-sharp3 needed four treble ledger
lines was wrong: it needs three, as does F-sharp3. F-sharp2 and C-sharp3 in the
Etude-6 case still need six and four. The frozen class is unchanged and does not
use a ledger-count threshold. The intended repair is not confined to these two
pieces; every independently proved scope is compared under the same rule.

Expected declared musical effect: lower clef course changes where the complete
extended course strictly improves the baseline. Unrelated output stability and
all note/beam/tie conservation must be checked on the actual native outputs;
planner changes alone do not prove a committed write or final page improvement.

Apply on accepted r1514 with:

    .\run.cmd --take --all

Build/clippy, the full test suite, ten-piece corpus, committed readback/reentry
and every changed PDF page require native validation. No native acceptance or
corrected PDF is claimed by this document.
