# WAV Performance Time Map Residual Audit r154

> Packaging/Change-Control recovery: r153 was rejected before installation because its declared diff still referenced the r152 documentation path. r154 carries the same PerformanceTimeMap audit capability on the still-green r152 parent with a corrected manifest.

## Position

WavPerception plan position: **Take 2/4**.

Parent: accepted full-corpus **v0.10.1839 r152 `WavMetricZeroOriginGridRefinementAudit`**.

ChangeClass: **AuditOnly**.

## Why r154 exists

The r152 full-corpus run closes the global Phase-A metric family much more strongly than the earlier audits:

- all ten Etudes rank the approximately 130 BPM / frame-zero grid as `AudioZeroOriginGrid#1`;
- `RefinedQuarterFrames` is tightly bounded between 20,356 and 20,360 frames across the corpus;
- the refined BPM range is approximately 129.961-129.986;
- even Etude 10, whose coarse tempo-vote family still gives 211 BPM two more raw votes than 129 BPM, is correctly re-ranked by WAV-only zero-origin grid coherence;
- all ten first-nonzero observations lie exactly or effectively on the refined frame-zero grid.

The remaining failures are no longer global tempo or origin failures. Etudes 2, 4, 8 and 10 miss the unchanged oracle-compatibility gate only in the residual tail:

- median residual remains 4-6 ms;
- span ratio remains approximately 100%;
- Match40 remains 83.9%-86.8%;
- Match80 is 87.7%-89.5%;
- P90 residual is 85-110 ms.

This pattern is consistent with local performance timing variation rather than a wrong global metric family. The original WavPerception architecture explicitly separated canonical score time from `PerformanceTimingResidual`; r154 now measures that distinction without changing canonical score time.

## Authority boundary

The execution order is:

```text
WAV
 ↓
attack / pulse evidence
 ↓
coarse tempo families
 ↓
zero-origin grid refinement
 ↓
AudioZeroOriginGrid#1 frozen
 ↓
local WAV-only timing residual windows
 ↓
monotone PerformanceTimeMap frozen
 ==============================
 evaluation boundary
 ==============================
FrozenLegacySourceOracle
```

FLP cannot:

- choose the global tempo family;
- choose the refined quarter length;
- choose the origin;
- create a local timing-map node;
- choose a local residual;
- reorder or repair the PerformanceTimeMap.

The oracle is evaluation only.

## Local timing residual

The r154 map uses the already WAV-selected `AudioZeroOriginGrid#1` as the canonical Phase-A grid hypothesis.

The audio timeline is divided into windows of four quarters. Within every sufficiently supported window, attack residuals to the nearest sixteenth grid are measured. When enough dual energy+transient attacks exist, those attacks own the local observation; otherwise the full pulse-attack family is used.

Residuals are:

1. estimated locally by a median;
2. unwrapped against the preceding residual so a gradual timing drift can cross the modulo-sixteenth boundary without jumping to a false adjacent phase;
3. lightly median-smoothed across neighboring map nodes;
4. filtered so mapped time remains strictly monotone.

The map includes the fixed project-time origin:

```text
metric frame 0 -> audio frame 0
```

This preserves the r152 Phase-A input contract.

## Domain separation

`Canonical Score Time` still answers:

> Where does the event belong musically/notationally?

`PerformanceTimeMap` answers only:

> At which physical WAV frame was that canonical metric position locally performed?

The map never rewrites MusicXML ticks, measure positions, phrase timing or source events.

## Evaluation

Only after the complete map has been frozen is the legacy onset oracle loaded.

The post-hoc evaluation uses the same unchanged compatibility contract as r152:

- span ratio 80%-125%;
- Match40 >= 75%;
- Match80 >= 90%;
- median residual <= 25 ms;
- P90 residual <= 80 ms;
- at least eight unique legacy onsets.

The new gate is:

```text
WAV PERFORMANCE-TIME-MAP AUDIO-IDENTIFIABILITY READY
```

It means only that the WAV-only map itself satisfies the oracle compatibility contract. FLP cannot rescue, select or alter it.

## Tests

r154 adds synthetic audit tests proving that:

1. an audio-only map can follow a gradual accumulated timing residual that a fixed global grid cannot represent as accurately;
2. the map retains the fixed frame-zero origin;
3. PerformanceTimeMap projection preserves strict metric order.

## Explicit non-changes

r154 does not change:

- productive FLP source authority;
- `CanonicalSourceEvents`;
- pitch reconstruction;
- staff/hand/voice logic;
- existing WAV articulation;
- canonical score time;
- phrase evidence or slur materialization;
- writer roots;
- master or render MusicXML behavior;
- PDF policy;
- dependencies;
- network behavior.

No model, MIDI authority, pitch authority or source cutover is introduced.

## Decision after r154

First run Etude 6. Compare:

```text
AudioIdentifiabilityEvaluation#1
PerformanceTimeMapEvaluation
PerformanceTimeMapGate
```

If the map remains audio-identifiable and improves or preserves the residual distribution, run the full corpus. If all ten pieces become compatible, the timing/metric half of Take 2 can be closed and Take 2 can move to pitch, harmonic explainability and same-pitch rearticulation evidence.
