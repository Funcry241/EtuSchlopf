# Wav Attack / Pulse / Metric Reconstruction Audit Packaging Recovery r149

## Position

WavPerception plan position: **Take 2/4**.

Parent: accepted full-corpus **v0.10.1839 r147 `WavObservationLegacyOracleQualification`**.

ChangeClass: **AuditOnly**.


## Packaging recovery

The first r148 package was rejected by the existing Take-Tree-Integrity gate before installation because it accidentally contained the unmanaged root `Cargo.lock`. r149 is a packaging/revision recovery from the still-authoritative accepted r147 parent. It removes only that forbidden top-level path and assigns a unique revision/name so parallel r148 files cannot create same-revision ambiguity. The fachliche AuditOnly source payload is otherwise unchanged.

## Purpose

r149 carries the same fachliche Take-2/4 audit payload as the rejected r148 and asks one empirical question only:

> Can the WAV, without FLP assistance, generate a bounded family of attack/pulse/metric hypotheses whose timing is compatible with the frozen legacy source onsets?

It does **not** reconstruct pitch and does **not** change source authority.

## Anti-leakage boundary

The order is hard-coded:

```text
WAV
 ↓
channel-preserving decoded observation
 ↓
energy-rise + transient-rise evidence
 ↓
audio-only attack candidates
 ↓
audio-only pulse / tempo families
 ↓
audio-only sixteenth-grid phase families
 ↓
frozen bounded metric hypotheses
 ================================
 evaluation boundary
 ================================
FrozenLegacySourceOracle unique onsets
 ↓
residual / coverage evaluation only
```

The FLP is loaded only after the audio hypotheses have already been built. It cannot generate attack positions, tempo, phase, origin or metric hypotheses.

## Physical time correction

r147's audit type name `OriginalSampleIndex` is corrected to `OriginalFrameIndex`.

A WAV frame is the simultaneous set of channel samples at one sample-rate position. For stereo, one frame contains two samples. Seconds remain a derived projection only.

## Attack evidence

No universal scalar confidence is introduced.

Two independent local evidence families are retained:

- positive energy rise;
- positive sample-to-sample transient rise.

Local maxima are selected independently per family under a duration-scaled diagnostic capacity bound. Candidate provenance records whether energy, transient, or both families support the candidate. For pulse inference only, candidates closer than a small frame radius are collapsed conservatively, preferring dual-family evidence, then transient evidence, then energy evidence.

This collapse is an audit pulse view. It is not a productive simultaneity, chord or articulation decision.

## Pulse / tempo hypotheses

Pulse hypotheses are generated from local inter-attack intervals. Multiple harmonic interpretations are retained. Quarter-note duration is bounded to 40–240 BPM for this Phase-A audit.

The existing Source-Parity contract remains:

- 96 canonical ticks per quarter;
- 4/4 is still the existing domain contract;
- no general meter detector is introduced.

For each tempo family, audio attacks vote on a sixteenth-grid phase. Multiple phase families and multiple possible absolute metric origins are retained. A fixed metric origin is not guessed from WAV frame 0 or from first nonzero audio.

## Oracle evaluation

After hypothesis freeze, raw FLP note events are projected to canonical 96-PPQ timing and reduced to unique onset positions **for timing evaluation only**. r147's event-multiplicity oracle remains unchanged; duplicate pitch/onset events are not deleted from the source oracle itself.

For each audio hypothesis, the audit reports:

- quarter duration in original WAV frames;
- BPM projection;
- candidate metric origin in original WAV frames;
- audio tempo/phase support votes;
- legacy-onset coverage within 10/20/40/80 ms;
- median onset residual;
- P90 onset residual;
- fraction of pulse attack candidates explained within 40 ms by the projected legacy-onset timeline.

Only the three best oracle evaluations are printed. The audio hypothesis family itself is built before the oracle is loaded.

## Diagnostic qualification contract

`WAV METRIC RECONSTRUCTION READY` requires the best evaluated hypothesis to satisfy all of:

- at least 8 unique legacy onsets;
- Match40 >= 75%;
- Match80 >= 90%;
- median residual <= 25 ms;
- P90 residual <= 80 ms.

These are audit qualification thresholds only. They grant no productive source, metric, pitch or writer authority.

Failure yields `WAV METRIC RECONSTRUCTION NOT READY` with explicit reason. No repair loop and no FLP-guided retry is permitted.

## Explicit non-changes

r149 does not change:

- FLP productive source authority;
- productive FLP parser / PianoSeedBuilder;
- existing WAV articulation or salience behavior;
- pitch reconstruction;
- hand/staff/voice logic;
- phrase classification, SlurIntent or phrase materialization;
- master MusicXML;
- sounding/timeline truth;
- render writer roots;
- PDF policy;
- model dependencies;
- network behavior;
- release semantics.

No new Cargo dependency is introduced.

## Expected next decision

If the full corpus shows robust metric reconstruction, the next WavPerception work may add pitch/attack-independence/harmonic-explainability evidence on top of this time map.

If a fundamental corpus-wide failure class appears, it must be named from evidence before any `Take 2b/4` is designed.
