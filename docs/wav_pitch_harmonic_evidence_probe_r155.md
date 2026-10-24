# WAV Pitch / Harmonic Evidence Probe Audit r155

## Position

WavPerception plan position: **Take 2/4**.

Parent: accepted full-corpus **v0.10.1839 r154 `WavPerformanceTimeMapResidualTakeChangeControlRecovery`**.

ChangeClass: **AuditOnly**.

## Why r155 exists

The r154 full-corpus run closes the Phase-A timing investigation as far as the present source-parity question needs it.

Across all ten Etudes:

- the WAV-only zero-origin grid remains tightly concentrated around 20,356-20,360 frames per quarter (about 129.96-129.99 BPM);
- the local PerformanceTimeMap remains monotone;
- typical local offsets are tiny (median roughly 1-4 ms, P90 mostly 5-8 ms);
- Etudes 2, 4, 8 and 10 still retain 80-110 ms residual tails even though the WAV-only map itself finds only a few milliseconds of local timing displacement.

Therefore the residual tail is not plausibly explained by another global tempo/origin heuristic or by a larger local rubato correction. The next uncertainty is perceptual: which pitches belong to independent attacks, which spectral peaks are partials, and whether same-pitch rearticulation produces fresh attack evidence.

r155 measures this without promoting any pitch into the productive source path.

## Authority boundary

Execution order:

```text
WAV
 ↓
decoded immutable observation
 ↓
WAV-only attack / pulse / zero-origin grid / PerformanceTimeMap
 ↓
audio-only bounded attack-probe selection
 ↓
88-key spectral presence + spectral attack-rise evidence
 ↓
audio evidence frozen
 ==========================================
 evaluation boundary
 ==========================================
FrozenLegacySourceOracle pitch/onset events
```

FLP cannot:

- select an attack probe;
- select or move an audio frame;
- choose a pitch candidate;
- change a spectral rank;
- change harmonic-parent evidence;
- change the time map.

The oracle is evaluation only.

## Bounded probe, not transcription

r155 intentionally does **not** attempt complete Audio-to-MIDI.

At most 192 pulse attacks are selected, spread across the full recording. Selection is WAV-only and prefers dual energy+transient attacks within each temporal bucket when available.

For every selected attack, all piano pitches MIDI 21..108 receive two independent observations:

- post-attack spectral presence;
- positive post-minus-pre spectral attack rise.

The audit retains separate evidence dimensions rather than introducing a universal scalar confidence.

No pitch threshold exists. No event is called “recognized”.

## Spectral probe

The probe uses deterministic Goertzel measurements at the exact equal-tempered piano frequencies.

For each pitch:

- the analysis window is at least 40 ms;
- low pitches may receive up to four periods, capped at 180 ms;
- a 5 ms gap separates the pre-attack reference from the attack;
- pre/post power is normalized for window length;
- attack-rise is `max(post - pre, 0)`.

The mono signal used by this measurement is a deterministic derived view obtained by averaging the preserved original channels. It does not replace the channel-preserving WAV observation authority.

## Harmonic explainability

For each audio pitch row, r155 also records whether stronger lower-pitch attack-rise evidence could explain it as a low-order partial family (octave / twelfth / double-octave parents).

This evidence is not used as a write gate and does not delete a candidate.

In post-hoc oracle evaluation, false Top-8 rows are additionally classified as harmonically explainable when they align with low-order partial offsets of a true source pitch. This quantifies the piano overtone-phantom problem rather than assuming it away.

## Evaluation

Only after the complete probe set has been frozen are canonical legacy pitch/onset events loaded.

The report measures, conditional on probe attacks that lie within 40 ms of an oracle onset:

- number of matched probe attacks;
- number of oracle pitch instances evaluated, preserving source multiplicity;
- attack-rise Recall@1 / @3 / @5 / @8;
- spectral-presence Recall@8;
- false Top-8 pitch candidates;
- fraction of false Top-8 candidates explainable as low-order harmonics;
- short same-pitch rearticulation pairs and whether both attacks recover the repeated pitch in Top-8.

These are diagnostics only. r155 introduces no quality threshold for pitch promotion.

## Explicit non-changes

r155 does not change:

- productive FLP source authority;
- `CanonicalSourceEvents`;
- canonical score time;
- existing WAV articulation;
- staff/hand/voice logic;
- phrase evidence or slur materialization;
- writer roots;
- master or render MusicXML;
- PDF policy;
- dependencies;
- network behavior.

No model, MIDI authority, pitch authority or source cutover is introduced.

## Decision after r155

Run Etude 6 first.

If the physical probe already places a useful fraction of oracle pitches in Top-5/Top-8 and the false Top-8 population shows a substantial harmonically explainable component, the next step can refine attack independence / harmonic suppression and then test full-corpus source-event reconstruction.

If the probe performs fundamentally poorly even on strong attacks, r155 is evidence to benchmark an external or learned sensor rather than accumulating ad-hoc spectral repair logic.
