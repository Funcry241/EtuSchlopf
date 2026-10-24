# r1016 – StrictStaff Orphan-Chord Gap No-Write Closure

Parent: accepted r1015 `StrictStaffCrossVoiceAmbiguityNoWriteClosure`.
Rejected r1013/r1014 remain non-parents.

## Proven local cause

The accepted r1015 corpus contains one blocked incomplete chord binding in Etüde 4.
At the E4 measure-30 lower-staff interval `[288,312)`, the master/render basis contains:

- a hidden rest in staff 2 / voice 2;
- two visible pitched `<chord/>` members in the same staff/voice and interval;
- no visible non-chord pitched anchor for those chord members.

StrictStaff intentionally excludes chord members from its visible lane intervals.  The
orphan chord is therefore invisible to the lane occupancy calculation and the interval
appears as a rest gap.  r1015 subsequently reports one StrictStaff write/adoption for E4,
and the committed render contains the exact StrictStaff overlay shape at that interval:

`backup 384 -> forward 288 -> visible rest 24 -> forward 72`.

That visible rest is the catalogue defect E04-02: a rest and sixteenth-note chord are
printed simultaneously in the lower staff.

## Narrow code decision

StrictStaff remains a repair/materialization writer and gains no new positive authority.
Before the existing r1015 cross-voice ambiguity guard and before `gap_parts` are built,
remove only parent gaps that overlap a visible pitched chord member on the same staff
and current render voice.

This is a negative, gap-scoped write-right rule:

- it does not infer or manufacture a missing chord anchor;
- it does not relabel, delete, hide, shorten, or move any existing note/rest;
- it does not alter the r1015 same-staff/other-voice guard;
- it does not suppress unrelated gaps in the same lane;
- it adds no project, measure, pitch, threshold, provenance, identity, or new-writer selector.

A normal chord with a visible non-chord anchor already occupies its lane through that
anchor and therefore does not create this gap shape.  The new guard is for the ambiguous
orphan-chord boundary only.

## Expected causal delta

For the E4 T30 witness, StrictStaff must no longer synthesize the visible staff-2/voice-2
rest `[288,312)`.  The pre-existing hidden rest and visible chord members are not changed
by this Take.  The appended StrictStaff overlay for this gap must disappear from the
committed render and PDF.

The existing r1015 E6 cross-voice no-write behavior is intentionally unchanged.  In
particular the E6 T40 witness must remain `candidate_transition 0->0` / readback `0`.

## Invariants

The first execution must preserve:

- master MusicXML byte-identically;
- sounding pitched event multiset;
- pitch/onset/duration/staff/voice of existing pitched material;
- r1015 E01-01 and E06-04 closures;
- the r1012/r1015 E6 T40 causal witness semantics;
- no new visible-rest synthesis outside the pre-existing StrictStaff candidate class;
- 1332 integrated tests unless a pre-existing environment/gate truth proves otherwise.

The permitted local product delta is the disappearance of StrictStaff rests whose parent
gap overlaps visible same-lane pitched chord members, plus resulting layout/PDF reflow.

## Abort criterion

Reject/roll back the Take if the E4 T30 visible rest survives, any pitched event changes,
the master changes, the E6 T40 r1015 witness regresses, an already closed catalogue defect
reappears, or the Take changes music outside the proven same-lane chord-overlapped gap class.

## First execution

Run only:

`./run.cmd --take "Etüde Nr. 4"`

Do not start with `--all` or `--inventory`.
