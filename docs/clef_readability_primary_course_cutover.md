# ClefReadabilityPrimaryCourseCutoverLegacyShadowClosure

## Scope

Take 3 of the Clef Readability block, based directly on the still accepted v0.10.1713 tree. The rejected v0.10.1714 and v0.10.1715 candidates are not installation bases. The writer-free readability solver proven in v0.10.1713 becomes the primary owner of the committed lower-staff F4/G2 course. No new writer is introduced. Existing FinalNotation structural write boundaries materialize the plan.

## Semantic capability closure

The first v0.10.1714 candidate was rejected because a new helper, `RustigClefCoursePlan::materialized_candidate()`, performed XML mutation inside `structural_write_authority` and therefore became a new writer root. v0.10.1715 removed that helper, but the semantic gate still correctly rejected the candidate.

The second rejection exposed the deeper classification issue. Two long-existing functions, `apply_readability_surface()` and `apply_readability_transition()`, are pure dynamic-programming state/cost helpers and never mutate MusicXML. Their historical `apply_` names nevertheless make them structural writer roots under the semantic capability inventory. In v0.10.1713 they were reachable only through the shadow-planning side and therefore were not part of the audit entrypoint's writer-reachability set. Once the productive Clef writer called the same Readability solver, those two false writer roots became transitively reachable from the audit-domain `real_main` entrypoint.

v0.10.1716 fixed the ownership vocabulary instead of weakening the gate: the pure functions are renamed to `account_readability_surface()` and `account_readability_transition()`. Their bodies and all solver semantics remain unchanged. They are no longer classified as writer roots because they do not write and no longer carry writer-like names.

A static replay of the gate's writer-reachability algorithm against the accepted v0.10.1713 tree shows:

- `pipeline_orchestrator_report_compaction.rs::real_main`: 223 writer roots before, 223 after; +0 / -0;
- all Audit-domain functions: 0 writer-reachability deltas;
- all Security-domain functions: 0 writer-reachability deltas.

The productive materialization remains inside pre-existing writer boundaries only.

## Legacy shadow compile closure in v0.10.1717

With the capability closure proven, Strict Clippy reaches the main binary and correctly reports the old removal-only Rustig course and its hysteresis helpers as dead after the primary cutover. Take 3 deliberately does not delete that legacy decision family yet; retirement is reserved for Take 4 after the musical cutover is green.

v0.10.1717 therefore keeps the legacy planner alive only as postwrite evidence. `derive_clef_readability_shadow_course_audit()` evaluates the old removal-only course without materializing it and reports `LegacyRustigRemovalCandidates`. This gives Take 4 a direct retirement signal while preserving a strict writer-free boundary: the legacy plan can be observed but cannot mutate Master or Render MusicXML. The same initializer is expressed directly rather than assigning fields after `Default::default()`, closing the Clippy `field_reassign_with_default` warning mechanically.

## Decision ownership

Before this take, local ledger/readback rules created clef surfaces and Rustig could only clean or normalize them productively. The v0.10.1713 shadow proved a legacy-independent course over register-preference-run boundaries, including safe insertion candidates. This take uses exactly that same solver for productive planning.

The productive path is now:

```text
ClefReadabilityObservation
        ->
ClefReadability course solver
        ->
complete F4/G2 plan
        ->
existing Structural Write Authority
        ->
MusicXML
```

Legacy local `>3`, sustained-preference and ledger-readback surfaces may still be generated earlier in the pipeline, but they are inputs/candidates only. They no longer own the committed final clef course. Their retirement belongs to Take 4.

## Materialization

The existing course plan is extended from removal-only to two safe edit kinds:

- removal of already proven pure lower-staff clef `<attributes>` blocks;
- insertion of a pure F4/G2 `<attributes>` block immediately before an observed Staff-2 attack boundary selected by the readability solver.

All edits are source-bound and applied in descending source order. The materializer verifies:

- source note fragments unchanged;
- exact resulting byte length from removed/inserted bytes;
- exact resulting clef count from removed/inserted clefs;
- insertion points are UTF-8 boundaries and begin at an observed `<note` start;
- insertion points do not overlap removal ranges.

The existing FinalNotation checkpoint remains the render write authority. Master materialization uses the existing in-memory structural boundary. No raw writer is added.

## Idempotence

A primary-materialized pure clef surface carries the marker `etuschlopf-clef-readability-primary-course`. If a later run of the same solver would describe that exact surface as OMIT plus an identical INSERT at its immediately following attack, the pair is canonicalized to KEEP. This changes no policy or cost; it makes repeated FinalNotation planning byte-idempotent and keeps postwrite evidence honest.

## Policy unchanged from v0.10.1713

- ledger lines are graded costs, not a hard maximum;
- one clef transition costs 24 pressure units;
- a return after at most two Staff-2 attacks adds one further 24-unit short-excursion cost;
- transition candidates are starts of maximal register-preference runs;
- no Etude, measure or pitch selector exists.

## v0.10.1713 corpus gate

The accepted shadow corpus was 10/10 dark green and behavior-neutral versus v0.10.1712. On the actual render artifacts the full-course shadow reduced:

- clef transitions: 85 -> 51;
- short excursions: 12 -> 0;
- ledger pressure: 6031 -> 1802.

An independent replay of the same render-course costs matched all ten reported final costs. Under those planned render courses the maximum observed ledger-line requirement was at most four; Etude 5 fell from a current maximum of nine to four. These are corpus acceptance facts, not hard-coded product limits.

## Allowed output delta

Only lower-staff F4/G2 clef surfaces and their necessary layout consequences may change. Pitch, onset, duration, voice, staff, note count, articulation, beam semantics, rests and octave-shift policy are outside this take.

## Postwrite audit

The existing clef-readability report becomes a primary-course postwrite audit. It re-evaluates Master MusicXML and the independently exported render MusicXML artifact with the same writer-free solver and reports the count of primary materialized insertion markers. A green cutover should show that committed courses are already preferred, without a second decision algorithm.

## Acceptance

Run exactly:

```powershell
.\run.cmd --take --strict --all
```

The full ten-project corpus must remain Build/Clippy/Final green. All MusicXML differences against v0.10.1713 must be clef-surface-only and preserve pitch/onset/duration/voice/staff. PDF changes are expected only as consequences of the new clef course.
