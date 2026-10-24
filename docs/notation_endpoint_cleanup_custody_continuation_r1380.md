# Notation endpoint cleanup custody — r1380

Parent: accepted r1379. Class: RefactorOnly. Unchanged music is the acceptance
requirement. Based on Dachs' Row6 input, reviewed against the actual owner path.

The existing EXACT Row6 deletes visual orphan slur/tied endpoints and then empty
notations wrappers. Sounding MusicXML tie elements are not selected by the actual
orphan cleanup. Custody previously compared tie and tied together, so even the
unchanged owner's notation-only deletion left the first selected-Master Unknown.

Read the original edit script at its actual bound pre-state. Every edit touching a
retained complete note must be wholly inside that note, have an empty replacement
and remove only self-closing tied/slur endpoint elements or an empty notations
wrapper. Parse the original deleted fragment as XML; foreign namespaces, sounding
tie, nonempty wrappers, other nodes, text, malformed fragments and non-self-closing
endpoints do not qualify. This readback is placed after the Render module boundary;
the isolated Raw-Splice test keeps its original dependency scope.

Only the original AllowedRangeRemovalExactMint selects this read-only continuation
case. Pitch, rest, chord, grace, every sounding tie, complete physical inventory,
exact intervals, staff segments, course, history and original identities remain
protected. Other callers and owners retain the original strict disposition rule.
This is no new orphan detector or writer, no new receipt, no mint, no role or
channel permit and no changed SSC verdict, original commit or musical selection.
An observation failure remains sticky Unknown; it neither grants nor vetoes the
original sovereign adoption. Missing original receipts remain fatal there.

Dachs' four synthetic actual-owner tests are retained. Additional tests go through
actual arrangement genesis, normalization, Row5 visibility, the actual orphan
cleanup including its second empty-wrapper receipt, selected-Master observation
and original adoption. Independent courses, empty context, no-op repetition,
dropped original receipts and excluded fragment classes are covered. Source
labels and corpus identities do not derive production conditions.

Windows acceptance requires the normal ./run.cmd --take --all, followed by all 20
XML files byte-identical to r1379, every PDF page pixel-identical and unchanged
released Master/Render lineage and sovereignty closure. No native corpus outcome
is claimed before that export. PRESERVING transitions and later custody boundaries
remain outside this scoped take.
