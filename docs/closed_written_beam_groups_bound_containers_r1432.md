# Closed written beam groups with bound chord containers

Parent: accepted r1431, v0.10.1839. MusicMutation: corrected comparison may admit
an existing channel-sharing proposal. No new writer or beam-state mutation.

The old comparator treated orphan continue/end strings as groups, preserved
incomplete chains and flushed a chain on every unmarked pitched chord member.
The physical head/member representation and closed group grammar are corrected
within the existing StructuralWriteAuthority sharing context. The canonical
producer, source capture, all original cohort/owned-state checks, role readers,
consumer profile and candidate enumeration remain unchanged.

Each physical member points to the exact output head occurrence using the parsed
anchor index and complete producer receipt. This relation is preserved across
label continuation. Only an emitted head/member pair on the same staff and voice
shares connections in this staff/voice-scoped graph. A cross-staff or foreign-voice
member stays a separate comparison input and receives no inherited connection;
its existing exact unknown-context preservation proof remains available. Every completed group expands its head connections to these
members without copying beam states. Actual states remain untouched. No original
group is used to suppress a written connection. New edges still need positively
captured common original cohorts; a different or missing cohort refuses output.

Synthetic tests cover orphan and incomplete groups, interruptions, replacement
begins, every supported level, cross-beat groups, bound unmarked chord members,
receipt-bound producer projection, true edge loss and forbidden new connections.
Renaming, relocation and independent duplication must retain the same result.
The native Linux MuseScore experiment is evaluation only; the Windows full corpus
and native B1/B2/B3 final acceptance remain distinct from local test success.
No promise is made that all remaining import losses disappear.
