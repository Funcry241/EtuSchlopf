# r1504 - Beam anchor distance audit

Parent: accepted r1503 / K8, candidate SHA-256
`b3f49ac8af1f4b491e220b6a28baeadfa380ddedf8159e1b1051263816c97841`.
ChangeClass: RefactorOnly (the mixed planner/audit module cannot accept new
AuditOnly lines under the unchanged parent policy). The cancelled documentation-only r1504 was not delivered.

## Purpose and output

The existing CLEF-READABILITY-PRIMARY-COURSE-POSTWRITE-AUDIT section of each
.result.txt now includes a BeamAnchorObservations count and complete BeamAnchor
rows, separately under MasterMusicXML and RenderMusicXMLArtifact. No new option,
launcher, dependency, renderer or secondary command is needed. A normal take run
also exercises the accepted r1503 parent bootstrap's HIMEM display.

The existing scanner supplies these facts at the point where it computes the
final-document event's anchor. Its productive wrapper supplies no observation
sink. The audit supplies a local sink; no global state, mutable request, timing,
writer, candidate filtering or new musical threshold is introduced.

Every visible pitched staff-2 event carrying a recognized level-1 beam appears,
including chord members, hooks, begin/end and unresolved continuations. Rows are
not capped or limited to a known project, measure, pitch, voice or rare outcome.
A part without a supported initial F4/G2 course still reports its raw evidence.
Absence of a Render artifact remains unavailable, with no Master substitution.

Columns:

- Event: serialized part-local note ordinal (all notes, including rests and hidden
  or upper events), printed measure label, actual measure ordinal, voice and staff.
- ChordHeadNote: current resolved chord head, or none for Unknown.
- TieNote: tie-derived anchor before combination with the beam.
- StoredBeam: the per-voice origin still held in the scanner at this event. A begin
  on a tied continuation can inherit an older tie origin; this is not necessarily
  the position of the beam's begin tag.
- BeamNote: beam-derived anchor after the existing Unknown mask.
- EffectiveNote: the scanner's combined clef-transition anchor. It is not evidence
  that a transition was proposed, selected, authorized or written there.
- StoredDistance: serialized note ordinal, measure ordinal and byte differences
  from StoredBeam to Event. These are not beats or elapsed musical durations.
  Repeated or nonnumeric printed measure labels do not affect these distances.
- BetweenXML: counts of actual backup, forward and attributes elements strictly
  after the stored-origin note and before Event. Parsed XML excludes comments and
  CDATA imitations. Missing origin or unparseable part yields unknown, never zero.

none is absence of the corresponding anchor, not a validity verdict. In particular,
a stored beam origin may remain present for Unknown while BeamNote and EffectiveNote
are correctly absent. An unmatched continuation may have a known chord head but
no stored beam or effective anchor. No classification of either as a musical
error, no repair authorization and no new reset policy are supplied.

## Deliberate limits

This is a final-document rescan of released Master/Render XML, not logging of the
productive prewrite inputs, every solver trial, a complete pipeline execution or
intermediate documents. A zero final-document distance cannot prove that no
intermediate beam influence occurred. Distance alone does not establish an
illegal connection. Structural continuity, foreign-head involvement, tie origin
and actual planned/written course must be reviewed together before any fix.
Existing no-beam and invisible events still affect scanner state as before; they
are not new rows. Upper or hidden heads can appear as positional origins.

Output size grows with the number of observed beamed lower pitch events. A source
with no observations reports zero. No stdout progress or receipt is parsed to
produce evidence. No musical authority is derived from the new text.

The generic selector gate also requires an implementation contract for the new
diagnostic control. It refers to the unchanged accepted chord-anchor definition
and documents projection of its existing self/Unknown anchor facts. It supplies
no remote-beam error class, distance verdict or productive authority.

## Verification and native acceptance

Synthetic tests compare scans with/without observation, cross-measure links with
repeated labels, exact XML boundaries versus comment/CDATA imitations, foreign
head/voice metadata, Unknown with an older stored origin, inherited tie origins,
unmatched continuations, octave reset, independent parts, unsupported courses,
complete report output, unavailable Render and unchanged plans around the audit.

The existing product scanner body and productive planner/writer blocks are
preserved except for the optional local observation sink. Parent-relative take
admission and unchanged signatures/pins are required, followed by the prescribed
strict test command, strict Clippy and optimized product build. Final XML probes
against r1503 test the unchanged planner's output, and do not replace native
pipeline evidence.

After installation run from the project root:

```cmd
.\run.cmd --take --all
```

Native acceptance requires all ten projects green, all 20 XMLs byte-identical to
r1503, PDF note images unchanged and no clef reentry. .result.txt changes are
expected from the added read-only evidence. E6/T22 and beam repair remain outside
this take. Read the resulting BeamAnchor rows before defining any repair.
