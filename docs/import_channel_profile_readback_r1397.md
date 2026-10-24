# r1397 — Import-channel profile readback (AuditOnly)

Parent: r1396 (KodexOnly, relief error classes). ChangeClass: AuditOnly.
No writer, selector, owner, receipt, adoption, renderer command or musical
output change. Master and Render MusicXML bytes are unaffected; only the
per-project result report gains one section.

## Purpose

The frozen class
`docs/selector_derivation_contract_partwide_renderer_channel_capacity.md` (r1326)
requires complete partwide resource evidence before any repair, and a later
repair needs an acceptance readback that shows whether the consumer would still
discard material. Until now the result report only carried
`VoiceTimelineCollisions`, which is zero in every project of the r1394/r1395
corpus although one project fails unforced import and two lose material silently.

`src/audit_only/import_channel_profile_readback.rs` recomputes the documented
MuseScore 4.7.4 first-pass reservation (`allocateStaves`, `allocateVoices`,
`VoiceOverlapDetector`, second-pass `determineStaffMoveVoice`) from the persisted
Master and Render MusicXML and reports per source:

- `SlotUse` per staff (four slots per staff for the whole part),
- `OverlappingChannels` with the first measure that made each channel overlapping,
- `UnreservedChannels` (channel and staff without a slot),
- `LostHeadsAndRests`, `LostPitchedOccurrences`, `LostVisiblePitchedOccurrences`,
  `LostUnregisteredMembers` (chord members whose channel was never registered),
- `PeakConcurrentChannels` per staff,
- `ProfileStatus admissible | exhausted | unknown`.

## Evaluation on the r1395 render XMLs (diagnostic only, no selector)

| Result | Projects |
| --- | --- |
| admissible | seven of ten |
| exhausted, fatal on unforced import | one: the main channel becomes overlapping through rests on the other staff, receives no slot on either staff, 497 heads/rests lost |
| exhausted, silent | two: 6 and 1 visible pitched occurrences lost although the conversion succeeds unforced |

Peak concurrent registering channels per staff is at most 2 in every project,
so the four-slot profile is not exceeded by simultaneous material; the defect
is the assignment. The readback agrees exactly with an independent Python
recomputation of the same import code.

## Authority boundary

Readback only. It moves no note, chooses no channel, staff, hand or voice and
grants no repair authority. `ProfileStatus admissible` is necessary but not
sufficient for musical completeness; exact pitch/onset/duration continuity must
still be proven separately (r1326, rejected diagnostic shortcuts).

## Tests

Seven synthetic unit tests: the r1326 counterexample shape (crossing rest makes a
channel overlapping and unreserved), label renaming plus duration scaling
invariance, the same material without the crossing rest, cursor-only
`print-spacing="no"` material, a fifth ordinary channel on one staff, an
unregistered chord-member channel, and the report status line.
