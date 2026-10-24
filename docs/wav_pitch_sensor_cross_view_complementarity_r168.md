# r168 — WavPitchSensorCrossViewComplementarityAudit

## Status

- Version: `0.10.1839`
- Revision: `r168`
- Parent: accepted `r167`
- ChangeClass: `AuditOnly`
- WavPerception plan: Take 2/4

## Why this take exists

The accepted r167 full-corpus run is the first complete Rust-only external sensor measurement.
Across all ten current corpus projects Basic Pitch produced 8,530 events for 10,166
FrozenLegacySourceOracle events. Exact 80 ms one-to-one matching yields 7,028 matches,
about 82.4% precision, 69.1% recall and 75.2% F1.

This is insufficient for Basic Pitch to become a sole source sensor. The raw event-count
upper bound alone is 8,530 / 10,166 = 83.9% recall even before pitch/timing errors are
considered. The observed high-register weakness is corpus-wide rather than isolated.

r168 does not tune Basic Pitch and does not introduce another learned model. It asks the
next bounded question: whether the already accepted physical AttackRise Top-8 evidence
covers a meaningful fraction of the oracle pitches missed by the independently frozen
Basic Pitch event projection.

## Hard anti-leakage ordering

For every project the existing report order first executes Basic Pitch:

WAV -> Rust/tract -> frozen Basic Pitch event projection -> SENSOR_OUTPUT_FROZEN.

The physical probe then builds and freezes its WAV-only attack locations and 88-key
rankings. r168 reads only the already frozen Basic Pitch projection before loading the
FrozenLegacySourceOracle. Only after both evidence channels are frozen is complementarity
evaluated.

No FLP value selects Basic Pitch events, physical candidates, thresholds, attack locations,
model parameters or rankings.

## Measurement

On the existing bounded physical probe attacks, r168 reports at 80 ms:

- probe-conditioned oracle pitch instances;
- Basic Pitch recovered instances;
- physical AttackRise Top-8 recovered instances;
- union recovered instances;
- Basic-only, physical-only, both and neither;
- the same decomposition restricted to MIDI >= 84.

This is intentionally probe-conditioned. It is not a new transcription metric and it does
not claim source multiplicity recovery. Its purpose is to test independent sensor
complementarity before any fusion algorithm is designed.

## Frozen behavior

Unchanged:

- Basic Pitch ONNX bytes and SHA-256;
- Basic Pitch thresholds and Rust decoder;
- tract version;
- physical Goertzel/AttackRise probe;
- WAV-only timing owner;
- evidence-only authority;
- productive FLP source authority;
- no MIDI transport;
- no CanonicalSourceEvent promotion;
- no staff/hand/voice, phrase or writer authority;
- no MusicXML/PDF mutation.

## Decision use

A high number of `PhysicalOnly` recoveries, especially in the high register, is evidence
that a later typed Rust sensor-fusion candidate may be worth testing.

If `Neither` remains large and high-register union coverage remains weak, Basic Pitch plus
the existing physical view is not enough for source parity; the next step must then be a
different strong piano sensor or an honest stop/re-scope decision.

## One-command contract

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

No additional user command is introduced.
