# Native pitched visibility retention (r1394)

Parent: hash-bound accepted r1393, candidate SHA256
97c38c1ea334639b3c5b3dc9dca53b98e4b92a8145ea62b94f093ece4867c03c.

MusicMutation safety repair under the unchanged accepted r1393 selector
contract and semantic obligation error class. No new normative obligation,
writer root, owner, descriptor, mint or override of SSC bindings.

MuseScore 4.7.4 imports a pitched note with print-spacing="no" as cursor
movement. A synthetic C4/E4/G4 chord retains all three notes before the
r1393 visibility exchange but loses E4 when frozen owner #5 conceals it.
Unforced native MusicXML roundtrip and MIDI output agree: [60,64,67] becomes
[60,67]. With explicit print-spacing="yes", [60,64,67] remains intact.
The physical pitch inventory and successful lineage tests did not detect this.

The planner now refuses before either writer unless the to-be-concealed
source note explicitly has print-spacing="yes". XML attribute parsing uses
the exact source note and supports both quote styles. Owner #5 preserves
that existing attribute, so the formerly printed tone remains native-sounding.
All original owners, original receipts and #57 file bindings stay unchanged.
No XML preconditioning is allowed. Without safe spacing, no exchange node
is created and the existing notation remains. This can reduce harmonic
micro-delta proposals. Tie refusal, pairing, ranking, budget and adoption remain.

Positive tests use synthetic source documents containing the prerequisite.
Negative tests cover absent/no/misnamed attributes, relocation and unchanged
bytes with no receipt. The actual creator-bound custody fixture exercises
refusal with complete unchanged source identity. No identity is reconstructed.

This prevents native loss introduced by this exchange. It does not prove that
all concealed source notes elsewhere in the pipeline participate in playback.
Broader owner #5 changes require legitimate resolution of its #57 binding in
a separate change. Windows acceptance and the full product corpus are pending.
