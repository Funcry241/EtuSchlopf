# r1399: ChordScopedStaffProjectionDefinition

Parent: accepted r1397. ChangeClass: KodexOnly. Definition-only metadata Take.
No Rust, tests, dependency, writer, renderer, owner, receipt or adoption changes.

## Problem and resulting contract

A consumer can render successfully and retain every pitched occurrence while
changing some presentation staves. MuseScore 4.7.4 imports a chord into one shared
object whose staff-movement property is overwritten during member insertion.
A per-occurrence staff obligation cannot be proven by note count or partwide
slot availability alone.

`selector_derivation_contract_chord_scoped_staff_projection.md` freezes this
independently derived class without selector-sensitive Rust. It supplies no
repair or release authority. The earlier capacity, incoming-tie, renderer-cache
and memory classes remain unchanged. The closure brief now requires this staff
boundary before a channel-repair Take is materialized.

## Targeted synthetic native evidence

Independent synthetic MusicXML documents contain one two-note whole-note chord,
no ties, no octave spans and no real project material. Every conversion uses
MuseScore 4.7.4 without force. Native re-export retains both pitches and their
intervals but assigns the head to the last member's staff:

| Probe | Required staves | Native result | Meaning |
| --- | --- | --- | --- |
| Upper head, lower member | distinct | both on member staff | contradiction reproduced |
| Lower head, upper member | permuted distinct | both on member staff | staff permutation preserves contradiction |
| Same-staff positive control | homogeneous | both unchanged | valid neighboring class |
| Renamed, divisions scaled, time-relocated and independently duplicated | distinct in each group | one staff mismatch per group | no known label, location, scale or singleton dependence |

These are independent causal-policy evaluations, not a new product classifier
or actual-writer proof. Their scripts, inputs, native exports and full stdout/
stderr are delivered as separate evaluation evidence, outside the managed Take.
No real corpus artifact or witness location becomes a normative fixture.

## Limits

r1399 changes no music and does not fix the capacity or staff defect. It is not
the originally proposed PartwideChannelAssignment implementation. The earlier
native witness failed its explicit staff-preservation prerequisite; pretending
that conversion success satisfied it would waive the material contract.

The next productive Take must consume accepted definitions, prove complete
canonical compatibility and retain pitch, interval, staff, ties, chord-host,
rest and cursor obligations. Its feasibility and total remaining Take count
are not claimed by this definition Take.
