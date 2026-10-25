# Basic Pitch sensor provenance

This audit tool evaluates Spotify Basic Pitch's ICASSP 2022 ONNX model as fallible, evidence-only WAV pitch/event evidence.

- Upstream: `spotify/basic-pitch`
- Model: `basic_pitch/saved_models/icassp_2022/nmp.onnx`
- Expected SHA-256: `2c3c1d144bfa61ad236e92e169c13535c880469a12a047d4e73451f2c059a0ec`
- Expected bytes: `230444`
- Upstream license: Apache-2.0
- Copyright 2022 Spotify AB
- License text: `../../docs/licenses/Basic-Pitch-Apache-2.0.txt`
- Model output labels used: `StatefulPartitionedCall:1` (note), `StatefulPartitionedCall:2` (onset)
- Model input label documented upstream: `serving_default_input_2:0`

The model is not a source authority. No MIDI produced by Basic Pitch is accepted as a source authority; this Rust audit decodes note-event evidence directly from ONNX outputs and projects timing back to EtuSchlopf `OriginalFrameIndex`.

The model may be fetched only during pinned bootstrap if it is absent from the audit runtime cache. Fingerprint mismatch fails closed. Inference itself performs no network requests.

The model is not included in this repository. On 2026-10-25 the primary
upstream URL returned exactly the byte count and SHA-256 listed above.
Redistributors who bundle the model must retain the Apache-2.0 license and
applicable upstream notices; the root MIT license does not replace them.
