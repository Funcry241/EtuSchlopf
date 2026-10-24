# WavPitchSensorQualificationBoundaryAudit — r156

## ChangeClass

`AuditOnly`

## Why this take exists

The accepted r155 full-corpus probe demonstrates a stable but insufficient ceiling for the
hand-built physical pitch view: corpus-weighted AttackRise recall is about 36.1% at Top-1,
73.2% at Top-5 and 79.2% at Top-8; Presence recall at Top-8 is about 81.5%.
Only about 17.8% of false Top-8 candidates are explained by the deliberately simple
harmonic-parent relation, and same-pitch rearticulation is recovered at both attacks for
about 58.9% of evaluated pairs.

Those measurements justify sensor qualification, not a threshold/partial heuristic spiral.

## Frozen boundary before model choice

r156 introduces no model, runtime or sensor output. It freezes
`wav_pitch_sensor_qualification_v1` as an evidence-only contract.

A later sensor candidate must bind at minimum:

- sensor identity and version;
- model identity/fingerprint;
- feature schema;
- exact source WAV fingerprint;
- derived audio-view identity;
- mapping back to `OriginalFrameIndex`;
- Windows/offline runtime qualification;
- deterministic decision output.

It may not obtain:

- MIDI authority;
- `CanonicalSourceEvent` promotion authority;
- staff/hand authority;
- phrase authority;
- writer authority;
- network dependency in the qualified runtime.

## Candidate research context

Current research candidates may include lightweight ONNX-capable polyphonic transcription
or piano-specific event models. Candidate names, versions and external benchmark results
are deliberately not encoded into the runtime contract in this take. The contract must
remain stable regardless of which candidate wins.

## Hard invariants

- productive FLP source authority unchanged;
- existing WAV physical probe unchanged;
- Phrase/Slur pipeline unchanged;
- no new dependency;
- no model file;
- no external sensor process;
- no runtime import of sensor evidence;
- no MusicXML/PDF mutation.

## Gate

`WAV PITCH SENSOR QUALIFICATION BOUNDARY READY`

means only that the evidence/provenance/authority boundary is frozen. It does **not** mean
that any learned sensor is qualified or selected.
