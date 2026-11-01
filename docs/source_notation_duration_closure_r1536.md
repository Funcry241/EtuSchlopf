# r1536: source notation duration closure

Parent: natively accepted r1535, SourceNotationTimeProjection.
ChangeClass: GovernedChange. Native admission and corpus evidence for r1536
remain pending.

## Defect and correction

The existing source-to-notation predicate accepted odd multiples of three
in its 96-divisions notation domain. Although the local serializer names a
dotted 64th of nine units, downstream metric partitioning can leave a
three-unit fragment and panic with `Normalization partition changed support`.
A local note-value name does not establish closure of the full pipeline.

The existing predicate now admits only values at least six and divisible by
six. Both ordinary durations and ordinary onsets use that same predicate.
Unsupported events reach the existing source-bound grace classification or
its existing early refusal, before destructive output preparation. No source
time is rounded and no source event is mutated.

The six-unit bound follows the current serializer's closed value grid, not
project identity, pitch, staff, voice, measure number or the observed corpus.
9/15 are regression witnesses, not the scope definition. Harmless contexts
that previously happened to complete do not establish general safety.

## Unchanged authority and scope

The frozen error-class definition and its accepted implementation contract
remain byte-identical to r1535. This correction adds no selection branch,
writer, timing approximation, ranking rule or new musical capability. It
corrects the numeric bound of the existing plain-value safety predicate in
the previously defined class: unsupported source timing must be explicitly
classified or refused rather than silently approximated or allowed to panic.

Meter support, grace hypotheses, conflict handling and independent candidate
multiplicity remain unchanged. Valid plain values and admitted source-bound
grace projections keep their prior plan bindings. Ordinary odd-three-unit
timing can now be refused even if a particular earlier context happened to
complete. This input-scope restriction is explicitly declared as music_effect.

## Regression and acceptance

The added test checks early refusal of 9/15-unit durations and onsets and
acceptance of six-unit control values. Runtime probes cover the complete
9, 15, ..., 93 family with directly adjoining notes, preservation of previous
products on refusal, D7 classification and 4/4/Menuett parity. The existing
native corpus remains an independent required check.

F2 (held voice in the same track/channel), F3 (removed grace host), N7 (missing
first-bar note), N8 (hand assignment) and N9 are not repaired by this take.
