# LowerStaffChordAttackProjection (r1502)

MusicMutation on accepted r1501/K7. Parent definition remains byte-unchanged:
`docs/selector_derivation_contract_lower_staff_chord_attack_projection.md`.

The writer-free LowerChordObservation follows serialized chord heads before staff,
visibility and rest filtering. It reports first lower observation, further member,
or Unknown. Actual backup/forward/attributes, part and measure boundaries sever
attachment; comments and CDATA do not impersonate boundary elements. A hidden,
upper or rest head still supplies a positional head. This is current-document
observation, not recreated Source/Attack/Fragment provenance.

The planner separates starts_attack (entry partition) from counted_attack (known
attack evidence). Unknown starts its own entry, retains pitch pressure and has no
anchor, count or duration. Existing passage_attack_count reads counted_attack.
A known tie-stop keeps the prior chain origin; all self-anchors use the chord head.
An unknown tie-start removes its local tie origin and an unknown beam-begin removes
its voice's origin, preventing a later continuation from laundering missing
attachment. No general beam-continuity, tie-identity, scope-map or fallback repair.

E054 uses the same facts. Only the first visible pitched lower observation of a
resolved chord contributes its own duration. Further members and Unknown contribute
no count/duration, but retain pitch pressure. A group starting at an orphan member
cannot emit a clef or change active_clef; a pending sustained preference remains
available for the next permitted group. The opening-measure fallback also refuses
an orphan member, even if it is upper-staff or unpitched and a later valid group
requests a change. Thresholds and signatures stay unchanged.

Synthetic tests cover swapped head/member order, hidden/rest heads, multiple
voices, boundary orphans including live tie/beam continuations, true tie-stop,
E054 duration and orphan output, exact note preservation and insertion atomization.
The new test module runs the real #58 preparation/exact receipt/replay/write/witness/
adoption path through existing crate-visible APIs in an isolated child test process
so its process-global measurement resets cannot contaminate parallel tests. All pinned checkpoint files stay
unchanged. Independent duplication and two measure bases provide identity-free
controls. Reentry and receipt/journal bindings are checked at E024.

The Stage-A definition and all existing selector/gate/registry/launcher/Cargo bytes
remain unchanged. Any implementation contract is selected from the actual unchanged
gate's result, not by tuning code wording. This ordinary document is not itself a
new authority. Required source/type/path and semantic admission run before packaging.

Expected corpus acceptance: 20/20 final XMLs byte-identical to r1501, Windows PDFs
pixel-identical except metadata, reentry 0, no beam/tie changes. End-state probe
neutrality is not a full pipeline proof; the full Windows corpus run remains the
final breadth check. Any XML difference stops acceptance for writer-trace analysis.
Beam-fragment continuity and E6/T22 remain outside this take.
