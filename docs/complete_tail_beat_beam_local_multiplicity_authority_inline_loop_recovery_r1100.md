# r1100 CompleteTailBeatBeamLocalMultiplicityAuthorityInlineLoopRecovery

Parent: **r1097** (`CompleteTailBeatBeamMultiplicityErrorClassFoundation`)

ChangeClass: **MusicMutation**

## Purpose

This take performs the CompleteTailBeatBeam anti-singularization cutover from the
accepted r1097 causal-class foundation.

Scorewide candidate multiplicity is observation only. Independent candidates with
disjoint source-event mutation domains may be authorized and materialized locally.
Candidates that compete for the same source-event domain remain fail-closed.

## Recovery from rejected r1098 and r1099

The rejected r1098 candidate duplicated the protected writer-entry source call sites.
The rejected r1099 candidate fixed the textual call-site duplication with a new helper,
but that helper itself became one additional Audit-WriterReachability node and was
correctly rejected before transaction start.

r1100 introduces no helper and no new productive function node. The existing
`execute_authorized_cursor_neutral_writer_candidate` function itself owns the local
multiplicity loop. The protected primary writer call and renderer-direction writer call
therefore each remain a single source call site inside the same existing productive
owner.

After every successful independent write, the current XML is reparsed and candidates
are rebound before another candidate can receive authority. A local candidate that
shares source-event ordinals with another current candidate is not admitted to the
local write path.

## Invariants

- no project, etude, measure, pitch or corpus-frequency identity selects a candidate;
- scorewide `candidate_count == 1` is not write authority;
- independent candidate duplication cannot disable an already valid local candidate;
- local overlapping conflict groups remain fail-closed;
- no new writer root or productive helper is introduced;
- the accepted r1097 causal error-class contract remains byte-identical.
