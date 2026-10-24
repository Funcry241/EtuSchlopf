# r1044 - Upper Initial Rest Measure-Local Causal Class and Debt Sync

## Purpose

r1044 is an AuditOnly causal-class and live-debt synchronization take for MI-A04. It does not change productive Rust, MusicXML, render MusicXML, PDF output, selector behavior, writer behavior, thresholds, rankings or weights.

The historical r1035 inventory identifies MI-A04 in `apply_upper_staff_initial_rest_and_beat_local_coalescing` as a class-A multiplicity debt: locally discovered initial-rest candidates are collected scorewide, assigned structural-feature scores, sorted globally, tie-broken by source position and finally limited by the caller's `max_initial_rests=1`.

## Source fact frozen before any selector change

The current r1043 source already computes initial-rest eligibility entirely from one physical measure:

- first visible upper pitched entry and its onset;
- whether a visible upper-start rest already covers the interval;
- local lower/cross-staff material before the upper entry;
- same-voice staff switches;
- existing octave-context evidence;
- the current metric-rest authority for the interval `[0, first_onset]`.

Only after those local facts are established does the current implementation add score bonuses (`first_onset==24`, lower/cross-staff evidence, staff-switch count, octave context), sort all candidates scorewide and obey `max_initial_rests`. The productive caller currently passes `max_initial_rests=1`.

r1044 freezes the causal class `upper_staff_initial_visible_silence_after_local_cross_staff_context` before any attempt to remove those scorewide authority surfaces.

## Multiplicity conclusion

For the initial-rest branch, current discovery creates at most one candidate per physical measure because it selects exactly one `upper_first` event per measure. Different measures are physically disjoint XML insertion domains. Therefore a second independently eligible measure is not evidence that the first candidate became ambiguous.

The following are explicitly non-causal and may not remain positive or negative write authority in the later closure:

- `first_onset == 24` score bonus;
- aggregate numeric candidate score;
- scorewide sort/rank;
- source-byte-offset tie break;
- `max_initial_rests=1` / global winner cap.

The later closure must not alter the local eligibility predicates merely to obtain more candidates and must not bundle the separate beat-local coalescing cap into MI-A04.

## Debt synchronization

The immutable historical r1035 inventory remains unchanged.

- MI-A01: productive authority retired in r1037; physical writer-surface cleanup in r1039; report verdict cleanup in r1040.
- MI-A02: productive authority retired in r1041; central facade intentionally remains compile-live pending separately classified cleanup.
- MI-A03: measure-local multiplicity closure accepted in r1043; candidate duplication across independent measures is regression-proven while same-measure ambiguity remains fail-closed.
- MI-A04: causal class frozen in r1044; productive selector remains unchanged in this take.

Thus the historical A baseline remains 12, three A surfaces are productively closed through r1043, and nine remain productively open. MI-A04 remains one of those nine until a later accepted MusicMutation closes its scorewide multiplicity authority.

## Validation boundary

ChangeClass: `AuditOnly`.

Expected product delta: none.

Expected productive Rust delta: none.

The accepted r1043 source, tests and runtime behavior remain the parent truth. r1044 only adds the causal-class contract and the updated live debt projection needed to make the next selector-sensitive take admissible under `causal_class_first_v1`.
