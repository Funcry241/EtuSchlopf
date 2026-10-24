# LegacyClefDecisionRetirement

## Scope

Take 4 of the Clef Readability block. This is a behavior-neutral retirement take relative to the accepted v0.10.1718 primary-course result.

No clef policy, planner cost, note data, staff/voice assignment, octave notation, beam logic, rest logic or writer boundary changes are introduced.

## Accepted proof from v0.10.1718

The full 10-project corpus is dark green and released:

- BuildExit 0
- ClippyExit 0
- FinalExit 0
- Master postwrite audit: `MaterializationCandidates 0` for every project
- Render postwrite audit: `MaterializationCandidates 0` for every project
- Master and Render: `LegacyRustigRemovalCandidates 0` for every project

The last item is the explicit retirement proof for the historical Rustig removal/hysteresis course. On all 20 accepted Master/Render artifacts, that legacy planner proposes no additional clef removal after the ClefReadability primary course is materialized.

## Retirement

The following historical decision path is removed from `rustig_clef_planning.rs`:

- `CLEF_CHANGE_COST = 1`
- `CourseCandidate` and the old surface-only DP course
- `solve_course()` / `retain_best()`
- `ClefContextEvent` / `ClefContextNote`
- clef-context hysteresis thresholds and scanners
- `derive_rustig_clef_plan()`
- the temporary `LegacyRustigRemovalCandidates` audit projection
- tests that exist only for the retired planner

The productive `ClefReadability` observation, full-course candidate space, readability costs, primary planner and the existing materialization boundaries remain unchanged.

## Deliberate non-scope

Earlier local clef-surface generators such as the A4 readability producer, ledger-bounded readback and register-pressure/island preparation are not removed in this take. They may still participate in earlier pipeline phases and therefore require a separate intermediate-state proof before deletion. Since Take 3 already established one committed final F4/G2 owner, these earlier surfaces are non-authoritative inputs, not final-course owners.

No Take 5 is implied by this document. The current block stops when the accepted Take-4 corpus proves output identity and the retired Rustig branch is absent.

## Required proof

Relative to accepted v0.10.1718:

- BuildExit 0, ClippyExit 0, FinalExit 0
- Master MusicXML identical after version normalization
- Render MusicXML identical after version normalization
- PDF pages pixel-identical
- identical final clef course
- postwrite `MaterializationCandidates 0` for all Master and Render artifacts
- no reintroduction of a legacy Rustig course
- net Rust line reduction

