# Course custody census at the Master adoption boundary — r1368

Parent: hash-bound accepted r1367, version 0.10.1839 (same-version Take).
ChangeClass: RefactorOnly. No music selector, owner, receipt, adoption verdict,
mint capability or writer allowlist changes. No new error class.

## Problem this measures

Owned rhythmic custody currently ends at the first unsupported owner step. In the
accepted r1367 corpus every project reports Unknown at the same first step
(ConsensusGesture/GaussChain, OwnerStep=1, PopulationRow=26). A failed
continuation makes every derived state Unknown, so each corpus run exposes only
the first custody gap. Post-solver stages carry no custody at all.

## What is added

`MasterAdoptedState::adopt_materialized_document` and
`adopt_in_place_transformation` (the path-bound PRESERVING adoption delegates to
the first) write one stderr line per byte-changing Master transition before the
unchanged `commit_owner_receipts` call:

`ETUSCHLOPF: CustodyCensus: transition=…; receipts=N; unchanged=…; text_only=…;
staff_moved=…; disposition_changed=…; removed=…; added=…; owners=[KIND:ROW:OWNER:
births=…:staff_capture=…]; projection_only=true`

Counts come from the existing physical continuation inventory of the actual
pre and post documents, paired in fixed order: identical bytes, equal
staff/interval/disposition (text only), equal interval/disposition (staff move),
equal staff/interval (disposition change); the rest is removed/added. An
unreadable inventory is reported as text, never as a count.

## Boundaries

- Observation only. The line is never read by an owner, the lineage commit, the
  SSC adoption verdict or any later stage. stderr failures are ignored as in the
  existing r1365 projection.
- `commit_owner_receipts` and its wiring stay byte-identical. The pinned
  `render_adopted_state.rs` and the Render lineage are not touched; Render-domain
  transitions are therefore not covered by this census.
- Pairing issues no identity and continues no custody. A count is evidence of a
  physical change class per transition, not proof of an owner's semantics.
- The in-place path clones the pre document once per adoption for the census.
- Implementation lives in a separate `impl OwnerReceipts` block after
  `mod render_lineage;` so the raw-splice receipt harness, which compiles the
  receipt core before that line, is unchanged.

## Use

Aggregating the lines by owner row and stage yields the complete custody gap
list of one corpus run, grouped into the families removal, birth, staff
transition, disposition, text-only and unsupported receipt kind.

## Tests

`tests/unit/custody_census.rs`: synthetic single-measure documents only;
unchanged, text-only, staff move, disposition change, split as removed/added,
repeated equal notes, unreadable inventory, and the line format for an empty
receipt set.

## r1372 cleanup: only informative census lines

The census line is now written only when the transition changes notes beyond
text (staff move, disposition change, removed, added), when the inventory is
unreadable, or when a receipt kind is not yet carried by the course
continuation (ADDITIVE, BLOCK without its own birth capture, PRESERVING other
than notation polish, clef islands and pianist layout). Text-only transitions
of carried kinds are omitted. On the accepted r1370 corpus this reduces the
lines from 11538 to 4953 without losing any custody gap. Counting and
classification are unchanged; the line remains projection only.

## Selected Master only

The census served its purpose for the solver phase: solver custody is complete
since r1374. It is now measured only on the selected Master, from
`SelectedSolverMaster` to the export, i.e. where `MasterAdoptedState` already
carries selected custody. Unselected solver trials no longer compute it.
The census is now part of `SelectedCustody::observe`, next to the custody
continuation of the same transition; there is no separate condition. This is
the reason for the structure, not a selector: it chooses no candidate, receipt,
writer or musical outcome. (A first draft guarded the old call with an `if` on
the selected custody; the lexical selector gate would have classified that as
decision logic, which it is not. The observation-owned form says what it is.)

Measured cost before this change: one census call reads the complete physical
inventory of both documents (about 24 ms for the largest corpus project in an
optimized build). The r1368 corpus computed it at least 11538 times; the solver
phase grew by about 110 s CPU between r1367 and r1368, matching that count.
Expected effect: the solver-phase lines (SafeAtomicCandidateProducer,
ConsensusGesture) disappear, Master-boundary lines are unchanged, XML and PDF
outputs stay byte-identical.

Known limit: clones of the selected Master used as editorial or replacement
trials still carry selected custody, so their census and custody lines appear
alongside the accepted transition (several `MasterXmlReplacementRequest` lines
per project). This is unchanged observation, not adoption evidence.
