# Phrase Slur Existing Writer Root Productive Recovery r146

Estimated plan position: Take 11/12.

Parent authority: accepted r144. Rejected r145 is diagnostic evidence only and is not a parent.

r145 demonstrated that placing String/XML mutation inside a new helper changes the semantic writer-root set and propagates WriterReachability to the Audit-domain `real_main`. r146 keeps the accepted r144 call topology and writer root, but moves all phrase-slur helper logic to read-only planning.

The only phrase-slur String/XML mutation (`insert_str`) and persisted render write (`std::fs::write`) occur inside the already accepted `materialize_final_render_notation_with_phrase_projection_authorized` StructuralWriteAuthority root. Helpers only parse final render MusicXML, resolve exact typed anchors, inspect materialized system starts, allocate the already-established MusicXML slur number-level capacity, and return immutable insertion plans.

Productive policy:

- input is only the already typed `allow_structural_pending_visual` projection;
- exact part/staff/voice/measure-ordinal/onset/single-pitch anchors only;
- missing, ambiguous, multi-pitch, same-note, or final-system-crossing endpoints are withheld;
- any pre-existing `<slur>` surface withholds phrase-slur materialization in this first productive revision rather than guessing ownership;
- number allocation consumes `PHRASE_SLUR_NUMBER_LEVEL_MAXIMUM = 16` unchanged;
- mutation is render-only after final horizontal-layout materialization;
- candidate MusicXML must parse, preserve note-event cardinality and the exact sounding/timeline fingerprint, and contain exactly balanced start/stop pairs per slur number before persistence;
- persisted render XML is exact-readback verified;
- master MusicXML, phrase classification, tonal truth, report semantics, thresholds, release/proof readers, `real_main`, Security policy, and historical Clippy debt are unchanged.

Security objective:

- no new writer root;
- no helper contains `replace_range`, `insert_str`, `fs::write`, or another productive XML mutation primitive;
- no new writer-like helper name;
- upstream WriterReachability root set must remain identical to r144.
