# r141 PhraseSlurNumberLevelCapacityPrewire

Estimated take 9/10 of the phrase-development plan.

Parent: accepted r139 `PhraseNotationIntentReadOnlyProjectionEndpointRecovery`.

## Purpose

Stage the MusicXML slur `number` level capacity as an explicit writer-neutral notation contract before any productive phrase-slur writer is introduced.

The rejected r140 candidate correctly failed closed because it introduced both a writer capability and a newly inventoried calibration threshold in the same same-version revision. This take does not weaken, rename around, or bypass that rule. It separates the two capabilities across revisions.

## Contract

`PHRASE_SLUR_NUMBER_LEVEL_MAXIMUM = 16` is a notation-capacity constant only. It does not select phrases, change SlurIntent, rank candidates, alter any score, or open a write path. The value reflects the MusicXML number-level capacity used for overlapping notation objects.

The constant is compile-time referenced so the public build remains compatible with `#![deny(dead_code)]`, while producing no runtime behavior.

## Invariants

- no phrase evidence change;
- no tonal/parallelism/closure change;
- no PhraseStructure or SlurIntent change;
- no AnchorResolver;
- no MusicXML writer;
- no render/master/sounding mutation;
- no Security rule change;
- no report semantic change.

A later independent MusicMutation take may consume this already-existing notation-capacity contract without introducing or changing a calibration threshold in the writer revision.
