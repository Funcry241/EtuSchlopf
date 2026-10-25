# r1517 - OttavaContextObservation

Parent: accepted r1516, SHA-256
`d3d796470650b0b9aa31a051b157cf268295a9b689bf721047878b51e50fe0d2`.
ChangeClass: RefactorOnly, because the existing register/session and productive
materialization integration files have mixed roles. Native acceptance pending.

## Product integration

The existing REGISTER-PLANNER-INPUT-READBACK result section retains its complete
hash-bound InputHex capture and now includes OttavaContext rows. No new report
section, option, dependency, launcher or command is introduced.

At the successful existing materialization witness boundary, the already computed
visible order and actual validated continue directions are copied into the
existing per-run register session. Each carrier retains part, staff, ordinal,
local onset, display and actual MusicXML line number. Session reset clears the
capture together with the original planner and witness. No report call reruns
planning or creates a materialization order.

The report projects captured semantic plans and affected/context event identities,
validated visible plans, fingerprints and actual line assignments. For every
captured carrier it compares the real early domain events at its entry and shows
the preceding positive gap with both boundary identity sets and foreign-staff
activity. Canonical interval units are the captured planner scale; local onsets
remain in their own measure's divisions. They must not be compared as if both
were quarter notes. No duration threshold is added.

The final census reads the final Master snapshot and available persisted Render
artifact separately. Every serialized continue is shown with exact artifact and
direction hashes, descriptive part/measure/staff/line metadata, words and their
print flags, placement/size/offset, declared system/page restart and document-order
position before the first pitched same-staff event. Text labels are hex encoded
to survive report sanitization. Line state is part-local and invalid/missing or
overlapping starts remain UNKNOWN. The census never supplies a musical verdict.

## Deliberate evidence limits

The Master observation is a final in-memory snapshot, not a new persisted-byte
readback claim. Render has no Master fallback. Final rows do not reconstruct
previous planner identity. LinkToCapturedCarrier remains not_proven: a later
repair needs actual transport/adoption evidence across downstream representation
changes. The report supplies both epochs for investigation, not an invented join.
XML layout declarations do not prove the visible PDF line, so PdfLineContinuity
and ReaderPreference remain UNKNOWN. This take authorizes no cue removal.

## Preservation and validation

Existing register choices, silent-bridge conditions, MusicXML direction generation,
layout projection, receipts and export guards remain unchanged. The only addition
at the productive call site is copying the already validated continuation fields
into diagnostic storage. Audit code cannot write music and is called by the
existing read-only report projection.

Synthetic checks cover all four displays, layout restart and later same-measure
cues, missing stage/Render data, invalid XML, stopped/changed/ambiguous line state,
independent parts and duplication. Parent-relative take preflight, strict tests,
strict Clippy and product build are required before delivery. Native corpus
acceptance must still verify 20/20 XML byte identity and unchanged PDF images;
the exported source alone lacks the WAV/MIDI inputs for a full native run here.

Apply once from the project root:

    .\run.cmd --take --all
