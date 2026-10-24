# Preserving layout journal custody — r1389

Parent: accepted r1388. Class: RefactorOnly. music_effect: none (all XML and PDF
outputs must stay byte/pixel identical to r1388).

Selected-Master custody stopped in all ten projects at the first PRESERVING owner,
`MasterNotationPrelude/VisualLayoutPressure` (E044 visual layout defaults), with
`Unsupported course continuation owner boundary`. The next boundary,
`PianisticReadability`, uses E013 local staff distance. Both change no note.

Each of the two frozen writers now records its own operation journal from its own
token and attaches it to the receipt it has just minted (same pattern as physical
births on EXACT receipts):

- E044: the token's single insertion at its insertion offset.
- E013: the measure-local operations of the token, produced branch by branch by
  `local_staff_distance_operations`. The replacement itself is still produced by the
  unchanged legacy planner; the journal is attached only when it replays exactly the
  authorized replacement and the actual document write (`enclosed`).

`OwnerReceipts::push_preserving` moves an attached journal into the existing
receipt-identity keyed transport list. `RhythmicRepresentationTransport::
preserving_layout` binds it to the original receipt (E044/E013 descriptor, buffer
owner, pre/post fingerprints) and refuses any removed or inserted fragment that
contains `<note`, `<backup`, `<forward`, `<measure` or `</measure`. Course custody
continues through the existing `continue_recorded_block` path: every complete note
must stay byte-identical, carried by the recorded operations. No geometry is matched.

Writer signatures, receipt mint and verifier, SSC proof, commit and adoption are
unchanged; the journal is observation only and is never read by them. A missing or
refused journal stays sticky Unknown at its owner step. Frozen callers
(notation_endpoint_layout_core.rs, #57 binding) are untouched.

Tests (synthetic, actual owners): defaults, new staff layout and raised distance carry
equal coincident notes with distinct identities; a receipt without journal keeps the
unsupported boundary; stale state is refused; a journal that rewrites a complete note
is refused even though it replays; every legacy planner branch is reproduced byte for
byte by the journal.

Expected Windows effect: the first selected-Master Unknown moves from
VisualLayoutPressure to the next uncarried boundary (MasterXmlReplacementRequest).
