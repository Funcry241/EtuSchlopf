# WAV-only Rescue Selector — structural closure r175

**Project:** EtuSchlopf  
**Version:** `0.10.1839`  
**Revision:** `r175`  
**ChangeClass:** `RefactorOnly`  
**Capability:** WAV-only Source/Rescue Selector  
**Final capability status:** `RETIRED`  
**Productive authority:** `NONE`

## Parent evidence

The accepted r174 full-corpus run is the parent of this closure.

It proved:

- full corpus: **10/10**
- `FinalExit: 0`
- `BuildExit: 0`
- `ClippyExit: 0`
- all ten project results: **OK**
- no blocker project
- no warning project

The r174 plumbing-removal run was compared with the accepted r173 retirement
run. All ten master MusicXML files and all ten render MusicXML files were
byte-identical. PDF file bytes were not used as the musical parity invariant;
the authoritative musical/render-XML surfaces showed no delta.

## Structural-closure boundary

r175 does not change a sensor, feature, classifier, threshold, candidate,
selection decision, source event, writer, MusicXML or renderer behavior.

The closure makes the already executed retirement durable.

The active Rust source must not regain the retired experiment vocabulary or
source surfaces for:

- physical rescue Top-N selectability
- frozen rescue feature rows
- TRUE-RESCUE / FALSE-ADDITION selector labels
- complete-candidate safety evaluation
- `TIMING_UNMATCHED_40MS` selector evaluation
- standardized-centroid discriminant
- leave-one-etude/project-out selector training
- 80/90/95-percent selector calibration
- corpus post-hook for selector evaluation
- runtime selector-retirement module

A dedicated integration test scans active `src/` and `tools/` Rust code and
fails if these retired surfaces reappear. Documentation and the test itself are
not productive code and are intentionally outside that scan.

## Retained independent capabilities

The following are not the retired selector and therefore remain:

- generic WAV decoding / immutable WAV observation
- WAV timing and PerformanceTimeMap audit evidence
- Rust/ONNX Spotify Basic Pitch audit sensor
- physical pitch/harmonic evidence
- cross-view complementarity evidence
- the relocatable Rust Basic-Pitch sensor identity/runtime fix from r171

These retain evidence-only semantics and grant no WAV-only source-event
promotion authority.

## Dependency and dead-path closure

The selector-specific r170-r173 Rust modules were already removed in r174.
No selector-only root Cargo dependency was introduced by that experiment and
none remains to remove in r175.

Current root dependencies retain independent active consumers:

- `anyhow`
- `clap`
- `flate2`
- `roxmltree`
- `serde`
- `zip`

The external Basic-Pitch audit tool remains an independent retained sensor
capability; its `anyhow`, `minreq`, `sha2` and `tract-onnx` dependencies are
therefore not selector debris.

r175 adds no dependency.

## Historical evidence

The compact authoritative historical record remains:

`docs/wav_pitch_rescue_selector_retired_experiment_history_r174.md`

It preserves the positive ranking result and the failed complete-candidate
safety result without retaining executable selector plumbing.

The decision remains:

> WAV-only Source/Rescue Selector: **RETIRED**.

No Better-Classifier, recalibration, high-register special case, feature
expansion or etude-specific optimization cycle is authorized by this closure.

## Acceptance

First:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Required: normal strict gates green and unchanged musical result.

Then:

```powershell
.\run.cmd --all
```

Required: ten expected projects, 10/10 OK, strict gates green, no dormant
selector regression, and no new musical/render-XML behavior.

A green full corpus closes the three-take exit sequence:

1. r173 — Authority/Experiment Retirement
2. r174 — Experimental Plumbing Removal
3. r175 — Structural Closure
