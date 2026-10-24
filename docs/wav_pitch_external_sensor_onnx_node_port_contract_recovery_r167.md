# r167 — WavPitchExternalSensorOnnxNodePortContractRecovery

## Status

- Version: `0.10.1839`
- Revision: `r167`
- Parent: accepted `r166`
- ChangeClass: `AuditOnly`
- WavPerception plan: Take 2/4

## Observed r166 evidence

r166 installed successfully. The productive EtuSchlopf run remained dark green. The Rust-only audit crate built, the exact pinned Basic Pitch ONNX model was made available and verified, and `tract` parsed the model.

The sensor did not yet produce events. The concrete failure was:

`ONNX-Eingang fehlt: serving_default_input_2:0`

Therefore r166 contains no negative pitch-sensor evidence. It is an ONNX interface-binding failure after successful Rust/runtime/model bootstrap.

## Root cause

Spotify's official ONNX Runtime integration addresses the model input as
`serving_default_input_2:0` and requests these three outputs:

- `StatefulPartitionedCall:1` — note
- `StatefulPartitionedCall:2` — onset
- `StatefulPartitionedCall:0` — contour

r166 incorrectly used `tract::Graph::find_outlet_label` for these export names. `find_outlet_label` searches tract outlet-label metadata; the accepted r166 runtime demonstrates that the model input export name is not represented there.

The exact model bytes and SHA-256 remain valid and unchanged.

## Recovery contract

The input side is unambiguous: the fingerprint-pinned Basic Pitch graph must expose exactly one model input. r167 therefore binds input index 0 and fails closed if input arity is not one.

The output side must not be guessed positionally because note and onset both have shape `[1,172,88]`. r167 therefore uses tract's graph-node-name contract:

`set_output_names([note, onset, contour])`

with the exact upstream export node names above. Before rebinding, the original output-node names are captured so a failed contract reports the actual tract-visible graph names.

After inference, r167 additionally fails closed unless:

- note shape is `[1,172,88]`;
- onset shape is `[1,172,88]`;
- contour shape is `[1,172,264]`.

This retains semantic binding by exported graph node identity while avoiding the invalid r166 assumption that export names are tract outlet labels.

`FeatureSchema` advances to
`basic_pitch_icassp2022_onnx_rust_note_event_original_frame_v2`
to bind the corrected ONNX port contract.

## Unchanged authority and execution boundaries

- Rust-only sensor path.
- No Python, venv, pip, PyTorch or PowerShell sensor wrapper.
- `tract-onnx = 0.22.1` unchanged.
- Basic Pitch model SHA-256 unchanged:
  `2c3c1d144bfa61ad236e92e169c13535c880469a12a047d4e73451f2c059a0ec`.
- `evidence_only`.
- No MIDI transport and no MIDI authority.
- No `CanonicalSourceEvent` promotion.
- Productive Source Authority remains FLP.
- FLP is not opened before `SENSOR_OUTPUT_FROZEN`.
- No staff/hand/voice, phrase or writer authority.
- No MusicXML or PDF mutation.

## One-command contract

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

No additional user command is introduced.
