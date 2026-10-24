# Phrase Boundary Evidence Foundation Audit r122

## Purpose

r121 was rejected fail-closed by Take-Tree-Integrity before installation because its package accidentally contained the non-managed root Cargo.lock. r122 is the recovery take from the still-authoritative v0.10.1839 r120 parent. It carries the same phrase-boundary evidence foundation without reopening the retired pause-delimited slur writer; the packaging defect is removed and the revision identity is unique.

The take is `AuditOnly`. It adds no MusicXML mutation, no notation mutation, no sounding-event mutation and no master apply. It has no phrase classification, no phrase span, no slur intent, no notation policy and no anchor resolver.

The architectural direction remains:

`Evidence -> Structure -> Intent -> Notation`

r122 implements only the first word: `Evidence`.

## Existing truth reused

The foundation does not introduce a second MusicXML note parser. The audit hook runs after the metric/final-notation stage and reuses the existing `parse_atomic_notes()` / `AtomicNote` projection for:

- staff,
- voice,
- measure-local onset,
- duration,
- pitch,
- visibility,
- source byte identity.

A small read-only source-location adapter binds each already parsed atomic observation to its enclosing part and measure ordinal. This adapter is locator context only; it does not reinterpret pitch, rhythm, staff or voice.

The source contract is the canonical master MusicXML after the metric/final-notation stage.

## Observation-position contract

r122 deliberately does **not** let pauses create phrase candidates.

For each `(part, staff, voice)` lane, visible pitched events are reduced to attack points. Simultaneous chord members at the same measure-local onset are collapsed into one attack observation. Every pair of adjacent attacks creates one **boundary observation position**.

Therefore:

- a rest may support an already existing observation position;
- a positive notated gap may support an already existing observation position;
- exact temporal continuation may oppose a boundary interpretation;
- overlap may oppose a boundary interpretation;
- no rest can create a position by itself;
- no observation position is promoted to a phrase-boundary candidate in r122.

This is intentionally a complete local observation lattice, not a list of claimed phrase boundaries.

## Evidence retained in r122

### Rhythmic evidence

For same-measure adjacent attacks only, the audit retains exact temporal relation:

- positive gap,
- exact continuation,
- overlap,
- explicit rest inside a positive gap.

Positive support and opposition are separate fields. They are not added together.

### Melodic evidence

r122 records only descriptors:

- nearest pitch distance between adjacent attack pitch sets;
- register-centroid delta.

There is no melodic threshold and no melodic phrase verdict.

### Metric context

r122 records only whether two adjacent attacks belong to the same measure or cross a measure boundary.

A barline is **not** phrase evidence.

### Voice continuity

Lane identity remains `(part, staff, voice)`. No voice collapse and no majority vote are allowed.

Same-measure exact continuation or overlap is retained as opposition to a boundary interpretation. Cross-measure continuity remains unresolved in r122 because the existing `AtomicNote` projection exposes measure-local time but not yet a canonical cross-measure time span for this audit owner.

## Explicit unresolved boundary

Cross-measure rhythmic/continuity evidence is intentionally left unresolved.

r122 does not invent measure duration, infer a barline pause, or treat a measure transition as closure. A later take may connect the phrase owner to an existing canonical metric span if the real source owner can be reused without parallel truth.

## No scalarization

The following do not exist in r122:

- `PhraseConfidence`,
- weighted phrase score,
- learned weights,
- `WEAK/MEDIUM/STRONG`,
- probability-like confidence,
- automatic boundary promotion.

The public report states this explicitly.

## External-data boundary

No external corpus, phrase annotation, model, embedding, learned weight or runtime lookup table is introduced.

## Report projection

The audit is captured through the already admitted audit-only orchestrator boundary and projected through the existing report preamble path. The productive final report owner and productive writer graph are unchanged.

The report section is:

`PHRASE-BOUNDARY-EVIDENCE-FOUNDATION-AUDIT-NO-MUTATION`

It exposes:

- source availability,
- atomic and pitched-observation counts,
- lane count,
- attack count after chord collapse,
- boundary-observation count,
- same-measure vs. cross-measure unresolved counts,
- rhythmic support/opposition counts,
- melodic-descriptor availability,
- evidence-family coverage,
- observation density,
- a deterministic bounded sample of boundary evidence rows.

The density is diagnostic only. It has no target range and no ground-truth meaning.

## Tests added

The audit owner contains direct tests proving the intended foundation semantics:

1. rests support but do not create observation positions;
2. exact continuation is negative boundary evidence, not a phrase verdict;
3. cross-measure rhythm stays unresolved instead of treating the barline as a boundary;
4. lanes are not collapsed or majority-voted;
5. identical input produces identical evidence.

## Hard stop after r122

r122 is not permission to add phrase spans or slurs.

The next decision must be based on real 10-project corpus output. In particular inspect:

- observation density by lane and piece,
- fraction of same-measure vs. unresolved cross-measure positions,
- whether rhythmic support is dominated by explicit rests,
- whether lane identity remains musically sensible,
- whether the complete observation lattice is computationally/report-wise manageable.

Only after that evidence is understood should the next take connect additional theory families such as tonal closure, parallelism or structural novelty.
