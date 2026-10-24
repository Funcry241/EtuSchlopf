# WavPitchExternalSensorRuntimeBootstrapRecovery — r158

## ChangeClass

`AuditOnly`

## Parent and evidence

Parent: accepted **v0.10.1839 r157 `WavPitchExternalSensorBenchmarkBoundary`**.

The accepted r157 single-project run remained dark green, but the external sensor process produced no sensor evidence because the audit harness required a pre-existing Python 3.11 or 3.10 installation. The recorded reason was `external_sensor_benchmark_process_failed`; the PowerShell audit stated `Python 3.11 oder 3.10 nicht gefunden`.

This is infrastructure evidence only. It is not negative evidence about TransKun and does not change the WAV-only GO/STOP question.

## Recovery

r158 keeps the r157 benchmark boundary and adds one internal bootstrap fallback:

1. Prefer an already available compatible Python 3.11/3.10 interpreter.
2. If none exists, download the official 64-bit CPython **3.11.9 NuGet build** into `target/wav_pitch_external_sensor_audit/runtime/`.
3. Verify the downloaded package against the pinned SHA-256.
4. Extract it locally below `target/`; do not install Python system-wide.
5. Verify the exact Python runtime version after extracting the fingerprint-pinned package.
6. Create the existing isolated venv below `target/`.
7. Install the already-pinned `transkun==2.0.1` and `mido==1.3.3` there.
8. From the start of inference onward, keep the existing offline variables and Python socket-connect guard unchanged.

Pinned CPython bootstrap package:

```text
Identity: python
Version: 3.11.9
Source: https://api.nuget.org/v3-flatcontainer/python/3.11.9/python.3.11.9.nupkg
SHA-256: 9283876d58c017e0e846f95b490da3bca0fc0a6ee1134b2870677cfb7eec3c67
```

The one-time bootstrap may use the network. That network use is explicitly reported as setup evidence and occurs before inference. Inference itself remains the object of `WindowsOffline` qualification.

## One-command contract

User interaction remains exactly:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

No Python installation, package-manager command, environment setup or second launcher is required from the user.

After a green single-project sensor observation, the existing next step remains:

```powershell
.\run.cmd --all
```

## Authority invariants

Unchanged from r157:

- TransKun is evidence-only.
- MIDI is transport only.
- no `CanonicalSourceEvent` promotion;
- no staff/hand/voice authority;
- no phrase authority;
- no writer authority;
- productive source authority remains FLP;
- sensor output is frozen before FLP/result evaluation;
- no MusicXML/PDF mutation is introduced.

## Scope

This recovery changes no Rust sensor contract, no writer/source/phrase code and no launcher/security control plane. It only makes the already-accepted r157 audit executable on a Windows machine without preinstalled Python.
