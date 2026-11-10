# Source voice and metric beam preparation

## Accepted baseline

The Windows export records acceptance of r1716 with candidate SHA-256
`6a42a2cc2be860d3a9e6cb0bb55a7b5d105c7f2195bd3d57939e856af005681a`.
Its embedded source files match that candidate byte for byte. The export
additionally contains the resolved Cargo lockfile. Build, Clippy and rendering
succeeded. The published Master has 754 visible attacks and the Render has 752.
Their attack and duration multisets agree with the corresponding local runs.

## Evaluation of the exported fugue

This evaluation is comparison evidence and supplies no runtime selection.

| Observation | Current output | Relevant boundary |
|---|---|---|
| Upper staff, bar 3 | At beat two the visible sixteenths carry no beam, begin, end, no beam. The parallel pair of eighths has no beam. The alto also reaches the lower staff on beat three. | Establish source voice continuity separately from beaming of each already allocated course. |
| Lower staff, bar 8 | At beat four the tied-in A-flat eighth and the following G and F sixteenths carry begin, end, no beam. The surface is identical in Master and Render. | Trace group coverage at the metric beam owner, including the tied-in duration-bearing container. |
| Bar 4 | The reference alto alternates between staves in the current output. | Check where independent voice continuity becomes staff or chord presentation. |
| Bars 20 and 27 | Each corresponding unison shares one visible head. | Determine whether the two source occurrences have independent written obligations before changing glyph count. |
| Bar 14 | The remaining spelling discrepancy is C-sharp in place of D-flat. | Re-evaluate the tonal spelling owner after the actual voice course is established. |

A read-only scan finds 189 complete contiguous quarter-beat windows of
visible beamable non-chord heads in the same written lane in Master and 197 in
Render. Of these, 31 in Master and 34 in Render lack a single complete primary
chain for the whole window. The scan reconstructs the MusicXML cursor through
notes, backups and forwards. It excludes gaps and groups that cross the beat
boundary, includes tied-in note containers and does not infer musical voice
from technical labels. The count is a screening result. Intentional
subgrouping, clef boundaries and other owner evidence still require review.

## Owner sequence

1. For source voice continuity, use the causal ledger from
   `src/midi.rs` to trace individual source occurrences into
   `src/midi_seed.rs` and the frozen serialization voice channels.
   MIDI already carries track and channel provenance in the causal
   ledger. The canonical event consumed by the seed carries an ordinal,
   time and pitch. A later take must explicitly qualify a source voice
   relation before granting it any effect on assignment. Staff and stem
   choices are separate decisions.
2. For primary and secondary connections, start at
   `add_beat_scoped_beam_groups` and its caller
   `enforce_beat_scoped_beaming_and_rest_completion_in_xml`.
   Compare the complete input lane at that boundary with its actual
   emitted group and carry that result into the written beam readback.
   The local r1716 owner capture already shows an ungrouped terminal
   sixteenth at the lower-staff example. The final markers alone cannot
   establish whether that owner received the complete written course.
3. Keep the two candidate domains independent. If the metric owner already
   receives a full supported lane, a beam-only change can be considered
   without changing voice or staff. If it receives a different course,
   establish who owns the representation difference before making a beam
   decision on the later course. If an emitted connection was lost, use
   the existing ordered interference and voice separation definitions.

## Acceptance for a later implementation

Use the two new error-class definitions and their separate synthetic cases.
Preserve the pitch, attack, release and visible occurrence multisets for any
beam-only implementation, including both sides of a tie. Record complete
ordered connections at each relevant level and preserve legitimate single
flags. For a voice representation change, verify that independent source
occurrences stay distinguishable through the seed and serialization owners,
including legitimate cross-staff presentation and supported shared glyphs.
Evaluate the real score only after these proofs. Compare Master, Render and
the PDF, including the existing four-page presentation.
