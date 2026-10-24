# r1180 — WholeRestDisplay selected-candidate interval witness

## Parent
Accepted/released r1179.

## Scope
Evidence transport only.

The productive `WholeRestDisplayCoordinateReadbackTruthFix` already computes an
`AtomicVisibleRestRoleObservation` for every visible rest and stores the actually
selected shadow-bookkeeping rows. Before r1180 the persisted pipeline line kept
only aggregate counts and total forward duration, so the selected row that
consumed the surviving E2 rest carrier could not be bound to its interval and
coverage evidence.

r1180 extends the existing selected-row audit payload with the values already
computed at selection time and serializes them into the same existing pipeline
line.

## No authority change
r1180 does not alter:
- `atomic_visible_rest_is_shadow_bookkeeping_candidate`
- the `>= 50%` overlap predicate
- candidate enumeration or ordering
- `render_forward_fragment`
- replacement byte ranges
- StructuralWriteAuthority
- parse/cursor/sounding invariants
- writer reachability
- final MusicXML, render MusicXML or PDF semantics

## Evidence fields
Each actually selected row reports staff, voice, onset, duration, overlap count,
same-voice overlap count, summed coverage, union coverage, geometry-union
coverage, multiplicity peak, real-voice-1 rest-overlap, and the role decision.

The witness is corpus-independent. No Etude, measure, pitch, duration, onset,
scope number or output expectation is encoded in the selector or in a test.
