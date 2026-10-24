# WAV-only Rescue Selector — retired experiment history

**Project:** EtuSchlopf  
**Version:** `0.10.1839`  
**Plumbing-removal revision:** `r174`  
**Structural-closure revision:** `r175`  
**Capability status:** `RETIRED`  
**Productive authority:** `NONE`

## Purpose of this record

This file is the compact historical record retained after removal of the
WAV-only rescue-selector experiment plumbing. It is documentation only. No
selector, model, calibration threshold, fusion path, source-event promotion
path, `CanonicalSourceEventCandidates` path, dormant feature switch or
productive fallback is retained by this record.

The generic WAV reader, Rust/ONNX Basic Pitch sensor integration and generally
useful physical pitch evidence are separate capabilities and are not retired by
this experiment result.

## What was tested

The experiment asked whether physical WAV-only pitch candidates that complement
the independent Basic Pitch view could be selected with sufficiently high,
cross-etude precision for eventual source-event authority.

The candidate features were frozen before the legacy source oracle. Oracle
information was used only afterwards for evaluation labels. Evaluation used
leave-one-etude-out separation over the ten-etude corpus.


## Rank-only precursor

r169 had already shown that direct physical Top-N rescue selection was not
sufficient for source-event promotion:

- Top1: **48.85 %** rescue precision, **26.87 %** miss recall
- Top3: **19.67 %** rescue precision, **55.00 %** miss recall
- Top5: **11.86 %** rescue precision, **64.74 %** miss recall
- Top8: **7.50 %** rescue precision, **72.41 %** miss recall

These results motivated the later multi-feature experiment; the active Top-N
selectability calculation itself is removed in r174.

## Positive research result

The multi-feature representation contained real cross-etude ranking
information.

Accepted r171 conditioned-candidate result:

- multi-feature mean Average Precision: **47.1 %**
- `rise_rank` mean Average Precision: **32.1 %**
- the multi-feature ranking was better in all ten held-out folds

This remains a valid research result. It does not grant productive authority.

## Complete-candidate safety result

r172 closed the evaluation universe at:

- all frozen candidates: **12,892 / 12,892**
- true rescues: **798**
- unsafe or false additions: **12,094**
- `TIMING_UNMATCHED_40MS`: **2,221**

Complete-universe mean Average Precision:

- multi-feature: **43.9 %**
- `rise_rank`: **24.2 %**

Held-out operating points derived only from the nine training etudes:

| Training precision target | Held-out precision | Rescue recall |
| --- | ---: | ---: |
| 80 % | 57.6 % | 1.8 % |
| 90 % | 70.5 % | 1.5 % |
| 95 % | 83.3 % | 0.6 % |

At the 90 % target the worst held-out fold was:

- precision: **0.0 %**
- recall: **0.0 %**

## Precommitted exit contract

Before the complete r172 result was evaluated, continuation required all of:

- worst-fold precision at least **90 %**
- practically useful rescue recall, clearly above **1–3 %**
- materially lower cross-etude instability
- stable advantage over `rise_rank`
- no etude-specific rules

r172 failed this contract.

## Decision

**WAV-only Source/Rescue Selector: RETIRED.**

The ranking advantage is retained as historical evidence only. It is not a
justification for a better-classifier, recalibration, feature-expansion,
high-register-special-case or other post-hoc optimization cycle.

The productive EtuSchlopf source path remains the existing deterministic
authority. No WAV-only selector, fusion, threshold authority or source-event
promotion was adopted.

## Lifecycle

- `r173`: authority/experiment retirement; productive authority explicitly set
  to none and further optimization forbidden.
- `r174`: selector-specific feature-vector, LOEO, calibration, candidate-safety,
  corpus-hook and retirement-runtime plumbing removed from active source.
- `r175`: structural closure adds a permanent dormant-path regression guard,
  records dependency/dead-path cleanliness, and closes the three-take exit after
  strict single-project plus full-corpus verification.

Do not reopen this experiment without genuinely new external evidence that
changes the scientific premise rather than merely offering another
post-hoc model or threshold.
