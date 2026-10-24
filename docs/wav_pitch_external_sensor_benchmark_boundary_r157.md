# WavPitchExternalSensorBenchmarkBoundary — r157

## ChangeClass

`AuditOnly`

## Parent and planning position

Parent: accepted full-corpus **v0.10.1839 r156 `WavPitchSensorQualificationBoundaryAudit`**.

WavPerception planning position remains **Take 2/4**. This take does not complete Take 2 and does not begin `CanonicalSourceBoundary`.

## Why this take exists

r155 measured the deliberately simple physical pitch probe as useful cross-view evidence but insufficient as a sole source sensor. r156 froze `wav_pitch_sensor_qualification_v1` before any learned/external candidate was allowed to influence design.

r157 performs the first candidate benchmark *behind that already-frozen boundary* while preserving the normal EtuSchlopf take workflow. There is no separate user-facing benchmark command.

The candidate is pinned to:

- SensorIdentity `TransKun`;
- SensorVersion `2.0.1`;
- CPU-only inference;
- exact package/model fingerprint readback;
- exact source WAV SHA-256;
- normalized event projection with `OriginalFrameIndex = round(onset_seconds * source_sample_rate)`;
- MIDI as transport only.

## One-command contract

The user-facing single-project qualification remains exactly:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

The accepted take transaction reexec executes the benchmark audit as part of the normal result projection. No second benchmark launcher, project-path parameter, Python command or manual dependency command is part of the user workflow.

After a green single-project gate, the existing full-corpus command remains:

```powershell
.\run.cmd --all
```

## Execution boundary

The candidate runtime is isolated below:

```text
target/wav_pitch_external_sensor_audit/
```

The managed source contains only the audit harness under:

```text
tools/wav_pitch_external_sensor_audit/
```

`run.cmd` and the launcher are unchanged.

If the pinned Python runtime packages are not yet present, the audit harness may perform a one-time setup into its isolated `target` runtime. This setup is not sensor inference and owns no evidence or musical authority.

For inference/evaluation the harness sets upstream offline variables and injects a Python socket-connect guard. A candidate that cannot run under that guard is not offline-qualified.

## Hard anti-leakage order

For every project:

```text
exact WAV
 ↓
TransKun inference
 ↓
MIDI transport parsed into normalized typed event projection
 ↓
source WAV / package / model / event fingerprints frozen
 ↓
SENSOR_OUTPUT_FROZEN
 ↓
only then prior qualified result + FrozenLegacySourceOracle FLP are read
 ↓
evaluation
```

The sensor process receives the WAV path only. It receives no FLP path, no legacy event list and no canonical score-event repair hints.

## Evidence projected into the normal result

The audit reports at minimum:

- sensor and oracle event counts;
- Pitch+Onset Precision / Recall / F1 at 80 ms;
- chord-tone recall at 80 ms;
- same-pitch rearticulation recovery at 80 ms using the existing <=96 canonical-tick pair definition;
- true duplicate pitch+onset multiplicity recovery;
- bass recall;
- high-register recall;
- false sensor events and simple harmonic explainability;
- package/model/source-WAV/frozen-projection fingerprints;
- runtime offline qualification state;
- Etüde-6 deterministic repeated normalized projection.

The detailed raw/frozen benchmark artifacts remain under `target` and do not become product inputs.

## Authorities that remain forbidden

The candidate has no:

- productive pitch authority;
- MIDI authority;
- `CanonicalSourceEvent` promotion authority;
- staff/hand/voice authority;
- phrase authority;
- writer authority;
- Master-MusicXML mutation right;
- render mutation right;
- FLP replacement or fallback role.

Productive source authority remains FLP throughout r157.

## Gate semantics

The single-project result may state:

```text
EXTERNAL SENSOR PIECE EVIDENCE OBSERVED
```

This means only that the pinned candidate executed behind the r156 contract and produced measurable frozen evidence.

It does **not** mean:

- sensor qualified for productive use;
- `CanonicalSourceEvents` ready;
- Take 2 complete;
- WAV source authority selected.

The corpus GO/STOP decision is deliberately deferred until the normal `--all` run has produced all ten project measurements.

## Hard invariants

- no `Cargo.toml` change;
- no new Rust dependency;
- no `run.cmd` change;
- no launcher/security-control-plane change;
- no writer/source/phrase mutation;
- no MusicXML/PDF mutation caused by this take;
- existing physical Goertzel/AttackRise evidence remains independent cross-view evidence;
- r156 qualification contract is not weakened for TransKun.
