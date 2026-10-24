# r166 — WavPitchExternalSensorRustBasicPitchManagedTreeResidueRecovery

## Status

- Version: `0.10.1839`
- Revision: `r166`
- Parent: accepted `r161`
- ChangeClass: `AuditOnly`
- WavPerception plan: Take 2/4

## Recovery from rejected r165

r165 was rejected before installation because the live accepted r161 tree contained a legacy runtime-generated file at `tools/wav_pitch_external_sensor_audit/__pycache__/sitecustomize.cpython-311.pyc`. The r165 payload correctly omitted Python, but its change manifest did not declare removal of that already-present live-tree residue.

r166 keeps r161 as its only installed parent, preserves the Rust-only Basic Pitch payload unchanged, and explicitly declares deletion of that `.pyc` residue. This is change-control closure only; it adds no Python runtime, no sensor authority and no new musical behavior.

`r162` and `r163` were rejected before installation by Take-Change-Control because Python bytecode contaminated the managed tree. `r164` was never installed; the Python-based sensor route was deliberately abandoned before application. Therefore `r161` remains the only valid parent for this cut.

## Purpose

The external pitch/event benchmark is moved to a Rust-only execution boundary. The former TransKun/Python/venv/pip/PyTorch sidecar is retired from the managed audit path.

The benchmark candidate in this revision is Spotify Basic Pitch's ICASSP 2022 ONNX model. This is only a candidate sensor used to measure evidence quality. It is not selected as productive source authority.

## Execution boundary

The managed source tree contains a small standalone Rust audit crate under:

`tools/wav_pitch_external_sensor_audit/`

The main EtuSchlopf crate does not gain ONNX or HTTP dependencies. At audit time the three managed audit-crate source files are mirrored into:

`target/wav_pitch_external_sensor_audit/rust_tool_src/`

and Cargo builds the audit executable with a separate target directory:

`target/wav_pitch_external_sensor_audit/cargo_target/`

Any generated `Cargo.lock`, downloaded crates and build products therefore remain under `target/` and outside the managed Take tree. The root `Cargo.toml` and productive dependency graph are unchanged.

The nested audit crate is pinned to Rust `1.88`, matching the root project contract, and uses `tract-onnx = 0.22.1`, whose declared MSRV is Rust 1.85.

## Model provenance

Candidate:

- Sensor identity: `SpotifyBasicPitchOnnxRust`
- Sensor version: `ICASSP2022-ONNX`
- Model file: Spotify Basic Pitch `saved_models/icassp_2022/nmp.onnx`
- Exact SHA-256: `2c3c1d144bfa61ad236e92e169c13535c880469a12a047d4e73451f2c059a0ec`
- Exact byte count: `230444`
- Upstream license: Apache-2.0
- Input label: `serving_default_input_2:0`
- Note output: `StatefulPartitionedCall:1`
- Onset output: `StatefulPartitionedCall:2`

If the model is not already cached under `target/`, the Rust audit executable may fetch it once from a pinned upstream/mirror URL. The exact byte count and SHA-256 are verified before use; mismatch fails closed. The model is not embedded into the managed Take tree.

Cargo dependency bootstrap may likewise require network access on the first audit-crate build. Once the compiled audit binary and exact model are present, the sensor inference path itself contains no network request. `setup_network_used` records whether model/bootstrap setup was required; this is distinct from inference authority.

## Rust-only signal path

The sensor path is:

```text
44.1 kHz stereo PCM24 WAV
  -> Rust decode/downmix
  -> deterministic Rust FIR low-pass + exact 2:1 decimation to 22.05 kHz
  -> Basic Pitch ONNX inference via tract
  -> Rust polyphonic note-event decoder
  -> direct OriginalFrameIndex projection
  -> SENSOR_OUTPUT_FROZEN
  -> only then FrozenLegacySourceOracle FLP evaluation
```

There is no Python runtime, no virtual environment, no pip, no PyTorch and no PowerShell sensor wrapper. There is also no MIDI transport layer in this path.

The ONNX decoder ports the upstream Basic Pitch note/onset thresholds and polyphonic note-decoding structure into Rust. The candidate output is represented directly as typed event evidence.

## Timing ownership

The benchmark must not read the current `<title>.result.txt`; that report is being constructed while the audit runs. Instead the already-qualified WAV-only `quarter_frames` owner is passed directly in memory from EtuSchlopf to the Rust audit process together with the exact WAV SHA-256.

The benchmark maps each retained Basic Pitch window frame directly back to its physical source window position and then to the original 44.1 kHz frame. This avoids treating the concatenated model frame index as physical time across window boundaries.

`Canonical Score Time` remains owned by EtuSchlopf. Sensor timing is evidence only.

## Anti-leakage ordering

The Rust tool verifies the WAV fingerprint and produces all sensor events without opening FLP content. It writes:

- frozen sensor event projection;
- sensor freeze manifest;
- freeze SHA-256.

Only after `SENSOR_OUTPUT_FROZEN` exists may the tool require/open the FLP file and evaluate the frozen sensor events against the `FrozenLegacySourceOracle`.

## Authority prohibitions

Unchanged and explicit:

- authority: `evidence_only`
- MIDI authority: `none_no_midi_transport`
- CanonicalSourceEvent promotion: `none`
- staff/hand/voice authority: none
- phrase authority: unchanged
- writer authority: none
- productive source authority: FLP unchanged

No MusicXML/PDF mutation and no WAV source cutover occur in r166.

## Measurement surface

The Rust evaluator retains the r157/r161 benchmark surface:

- pitch+onset precision / recall / F1 at 80 ms;
- chord-tone recovery;
- same-pitch rearticulation;
- multiplicity recovery;
- bass recall;
- high-register recall;
- false sensor events;
- simple harmonic explainability of false events;
- deterministic repeat for `Etüde Nr. 6`.

A full-corpus GO/STOP decision remains deferred until a green single-project Take run is followed by the normal full-corpus run.

## One-command contract

User operation remains exactly:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

No Python installation, package command, model command or second launcher is a user responsibility.

## Validation limitation before delivery

The artifact-building environment used for this Take has no Rust compiler installed. Therefore the nested Rust audit crate cannot be compiled here. Syntax, source-tree diff, provenance constants, Python-path retirement and Take packaging are checked statically; the existing Windows fail-closed pre-take/build gate remains the first authoritative compilation test.
