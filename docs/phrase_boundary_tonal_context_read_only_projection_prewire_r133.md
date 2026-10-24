# Phrase Boundary Tonal Context Read-only Projection Prewire r133

## Estimated plan position

Take 6/10 (estimated) on the optimized phrasing path.

The denominator increases from 9 to 10 because the r132 hard stop exposed one architectural
prerequisite: the existing score-wide tonal-evidence owner is internal to the Music-domain tonal
shard. An AuditOnly take may not modify that Music-domain surface, and duplicating the tonal profile
logic inside the phrase audit would create a forbidden shadow truth. r133 therefore performs only a
behavior-preserving read-only projection prewire. The actual Tonal/Closure evidence remains the next
AuditOnly take.

## Parent

Accepted DARK-GREEN parent: EtuSchlopf v0.10.1839 r132
`PhraseBoundaryParallelismStructuralNoveltyEvidenceAuditNoScalarization`.

The 10-etude hard stop after r132 confirms:

- 10/10 projects released dark green;
- 6,978 true attacks and 6,949 boundary observations;
- Rhythm + Metric + Melodic + Parallelism + Structural Novelty expose substantial structure beyond
  explicit pauses;
- important ambiguity remains between structural joints and actual tonal closure;
- no phrase boundary is promoted and no scalarization exists.

## Why a prewire is necessary

The tonal profile truth already exists in
`src/pipeline_shards/key_signature_enharmonic_report_core.rs`. It owns the 24 profile candidates,
weighted score evidence and the best-vs-second evidence margin used by existing tonal consumers.

The phrase audit must reuse that owner. It must not copy the profile table, scoring weights or
profile-selection algorithm into `src/audit_only`.

Because `AuditOnly` is deliberately forbidden from changing Music-domain APIs or callers, the
read-only cross-domain projection has to be established first under `RefactorOnly`.

## Changes

r133 adds one narrow read-only projection function to the existing tonal owner:

`scorewide_tonal_evidence_projection_from_xml(xml)`

It returns only:

- existing best profile name;
- tonic pitch class;
- major/minor flag;
- existing best score;
- existing second-best score.

It does **not** return a key-signature write decision, final-section write authorization or any
notation capability.

The established phrase-boundary capture receives the canonical master MusicXML text that its
Music-domain caller already has in memory. In r133 the audit calls the read-only tonal projection and
intentionally discards the result. This is a prewire only: no phrase report field, evidence-family
count, threshold, classification or persisted result changes.

A following AuditOnly take can consume the same projection from inside
`phrase_boundary_evidence.rs` without changing the Music-domain caller again.

## Behavior contract

r133 is behavior-preserving apart from normal installed revision identity:

- phrase report content unchanged from r132;
- boundary counts unchanged;
- EvidenceDiversity unchanged;
- no TonalDescriptor field yet;
- no cadence class;
- no local-key claim;
- no PhraseConfidence;
- no scalarization;
- no PhraseSpan;
- no SlurIntent;
- no MusicXML mutation;
- no sounding-event mutation;
- no writer capability;
- no Security policy change;
- no external data, corpus, model or learned weight.

## Required next step

Install r133 on a known-green project. If Build, Clippy, Semantic-Capability and output invariants are
green, the next take may be AuditOnly and add Tonal/Closure + Anti-Closure descriptors using this
single existing tonal owner.
