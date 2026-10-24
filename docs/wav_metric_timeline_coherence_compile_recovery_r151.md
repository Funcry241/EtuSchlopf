# Wav Metric Timeline Coherence Compile Recovery r151

## Position

WavPerception plan position: **Take 2/4**.

Parent: accepted full-corpus **v0.10.1839 r149 `WavAttackPulseMetricReconstructionAuditPackagingRecovery`**.

ChangeClass: **AuditOnly**.

## Why r151 exists

The r149 full-corpus run proved that the WAV-only front end can generate metric hypotheses with strong local oracle agreement, but it also exposed a measurement flaw: the oracle evaluator matched every projected legacy onset independently to the nearest pulse attack. In dense piano textures, a globally compressed timeline can therefore look locally plausible.

Concrete corpus examples were 174.308 BPM and 226.930 BPM candidates that reached the old local residual gate even though their projected full-piece timing is much shorter than the observed attack timeline.

r151 carries the intended r150 audit-evaluator correction after r150 was rejected before installation by the Rust compile gate. The rejection was caused solely by two `format_args!` values being passed positionally after named arguments in `src/report.rs`. r151 fixes only that compile surface and otherwise preserves the intended r150 semantics.

r151 corrects the **audit evaluator**, not the productive music path and not the WAV hypothesis generator.

## Unchanged anti-leakage boundary

The order remains hard-coded:

```text
WAV
 ↓
channel-preserving observation
 ↓
attack evidence
 ↓
pulse / tempo families
 ↓
phase / origin hypotheses
 ↓
all audio hypotheses frozen
 ==============================
 evaluation boundary
 ==============================
FrozenLegacySourceOracle
```

FLP still cannot generate or tune attacks, tempo families, phases, origins or audio ranks.

## Audio-side observability added

r149's single `AudioRank` mixed three concepts: tempo-family order, phase-family order and absolute-origin expansion.

r151 keeps them separate in the audit output:

- `TempoFamilyRank` — rank of the WAV-only tempo family;
- `PhaseFamilyRank` — rank of the WAV-only sixteenth-grid phase family inside that tempo;
- `AudioRank` — final bounded-hypothesis rank after origin expansion.

The report also prints the first three frozen audio hypotheses and the first three distinct WAV-only tempo families **before oracle evaluation**. This lets the corpus answer whether the oracle-compatible tempo family is already highly ranked by audio evidence.

## Global timeline-span coherence

For each frozen audio hypothesis the oracle evaluator now compares two spans:

```text
ProjectedLegacySpanFrames
=
(last unique legacy onset tick - first unique legacy onset tick)
    projected with the already-frozen audio QuarterFrames
```

and

```text
PulseAttackSpanFrames
=
last WAV-only pulse attack frame - first WAV-only pulse attack frame
```

The ratio is diagnostic evaluation evidence only. It cannot influence audio hypothesis generation.

A deliberately broad coherence band is used:

```text
80.0% <= SpanRatio <= 125.0%
```

This is not a tempo model and not a production threshold. It only rejects grossly compressed or expanded oracle mappings while leaving substantial room for leading silence, terminal audio tails and performance timing.

## Ordered one-to-one evidence

r149's local nearest-neighbour metrics are retained for continuity, but r151 additionally reports monotone one-to-one matching at 40 ms and 80 ms:

```text
Ordered40
Ordered80
```

A pulse attack can therefore no longer explain arbitrarily many projected legacy onsets in these additional diagnostics.

The ordered metrics are **reported only** in r151; no new threshold is attached to them yet. The full corpus must first show their empirical distribution.

## Qualification semantics corrected

The old text `WAV METRIC RECONSTRUCTION READY` was too strong. r149 only demonstrated that an oracle-compatible candidate could sometimes be found among the frozen WAV hypotheses.

r151 therefore names the gate accurately:

```text
WAV METRIC ORACLE-DISCOVERABILITY READY
```

or

```text
WAV METRIC ORACLE-DISCOVERABILITY NOT READY
```

The gate requires:

- at least 8 unique legacy onsets;
- global span ratio 80.0%-125.0%;
- Match40 >= 75%;
- Match80 >= 90%;
- median residual <= 25 ms;
- P90 residual <= 80 ms.

This proves **oracle-discoverability only**. It does not claim that WAV alone can yet select the unique productive tempo/phase/origin.

## Explicit non-changes

r151 does not change:

- WAV attack generation;
- tempo-family generation;
- phase/origin generation;
- FLP productive source authority;
- source-event construction;
- pitch reconstruction;
- hand/staff/voice logic;
- existing WAV articulation;
- phrase evidence or phrase materialization;
- writer roots;
- master MusicXML;
- render semantics;
- PDF policy;
- dependencies or network behavior.

It also removes the r149 audit-only `needless_range_loop` Clippy warning in the envelope channel loop without changing its numerical behavior.

## Decision after r151

The r151 full-corpus output must answer two separate questions:

1. **Oracle discoverability:** does a globally coherent frozen WAV hypothesis explain the legacy timeline?
2. **Audio identifiability:** what `TempoFamilyRank` and phase/origin ambiguity does that coherent hypothesis have before oracle evaluation?

Only after those are separated can Take 2 proceed to genuine audio-side metric disambiguation or, if the tempo family is already robust, to pitch/attack-independence evidence.

## Compile-recovery provenance

r150 `WavMetricTimelineCoherenceAudit` was rejected before installation because `src/report.rs` mixed named and later positional `format_args!` arguments. The installed productive parent therefore remains accepted r149. r151 is the unique recovery revision and contains the same intended AuditOnly timeline-coherence payload with the format arguments made explicitly named.
