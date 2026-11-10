# Note visibility: close caller-side pre-repair at the existing owner

Parent: natively accepted r1687. ChangeClass: GovernedChange.
MusicalCandidateSelectionChange: no. The existing callers still decide which original note is
hidden. No new musical error class, carrier selection, voice assignment,
rest consolidation rule, sounding edit or writer capability is introduced.

## Cause and owner

The existing row-20 `hide_note_fragment_print_object_no` owner appended a new
attribute without recognizing an explicit `print-object="yes"`. A direct call
could therefore produce duplicate XML attributes. The carrier caller worked
around this by changing yes to no before invoking the owner. That change
escaped the owner's original-input receipt; other unnormalized calls remained
exposed to malformed output. The test-only carrier helper repeated this repair.

The original owner now reads the note's own opening attributes, replaces an
explicit yes value, preserves an existing no, or inserts the missing attribute.
Quotes, whitespace and attribute entities do not create a separate musical
selection. Unrelated child attributes and comments cannot suppress the actual
note operation. Malformed or unsupported input remains unchanged. Both callers
pass the original bytes without a preparatory visibility edit. The existing
row-20 mint, original fragment binding, duration promotions and consumer remain
responsible for every actual change. No receipt or custody is synthesized.

## Verification and scope

Synthetic checks cover absent/yes/no attributes, quoting, whitespace, encoded
attribute values, unrelated children/comments, quoted greater-than signs,
invalid input, idempotence, child-byte preservation and original-source receipt
consumption. Rest and pitched fragments use the same existing owner; the caller
selection stays unchanged. No corpus excerpt or address defines a test.

The accepted corpus is evaluation only. Expected ordinary output is unchanged;
previously unsupported explicit-yes fragments may now complete with one valid
attribute and an authentic original-input receipt. Native XML/PDF comparison
remains required after the single implementation take. C4 and the complete
mathematics bibliography remain byte-identical to the accepted parent.

## Declared implementation effects

The existing row-20 hiding operation is implemented in the already included
canonical structural receipt-ownership shard, alongside its duration-promotion
consumer. Its Rust module, function signature, original mint type, descriptor,
callers and sink contracts stay unchanged. The existing row-20 wrapper keeps the original mint and calls a private
`mutate_note_visibility_fragment` materializer under the existing
`master_adoption` consolidation boundary. This function owns the actual field
splice; it selects no score candidate and introduces no receipt or sink.
The actual field write belongs to StructuralWriteAuthority, rather than adding
a newly recognized raw splice writer inside the legacy pipeline shard. The unchanged parent explicitly
rejects that noncanonical alternative. No legacy delegate retains a splice.

The existing GovernedChange class declares both the owner relocation and the
visibility effect. No governance rule, descriptor, containment envelope,
verifier, mint capability or public API is changed. No new musical candidate,
caller root or post-processing pass is introduced.
