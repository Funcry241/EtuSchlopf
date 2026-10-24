# r1029 — NotationVoiceMaterializationSafetyConjunctionAudit

## Parent and change class

- version: `0.10.1839`
- logical parent: accepted `r1028` / `RegisterNotationOctaveKindBoundaryEvidenceAudit`
- change class: `AuditOnly`
- music mutation: no
- render write: no
- new writer/repair/selection/release authority: none

r1029 returns to the already visible Etüde-6 Takt-1 voice/beam problem. It does not repair the notation. It closes the previously scattered materialization preconditions into one fail-closed read-only conjunction before any later writer is allowed.


## r02 recovery after rejected first candidate

The first r1029 candidate (SHA-256 `d60c0f540be2769d5e2a4a1cb5a5f6437a767aa5a8008dde2d7c0da62545b233`) was rejected before installation by Take-Change-Control because newly added AuditOnly lines referenced `structural_write_authority::` directly. Repository rule r753 also makes a second boundary explicit: AuditOnly must not add a new call-site or transport edge in mixed productive source.

r02 therefore changes no mixed productive Rust source at all. The existing authorized `append_notation_voice_metric_line_baseline_overlay_audit` call already receives the required frozen channel plan inside the dedicated Audit path. r02 derives only primitive read-only `(source_event_id, channel, role_label)` rows there and feeds those rows to the new conjunction helper. The helper and its tests contain no newly added StructuralWriteAuthority reference, writer token or productive call-site. The first candidate remains rejected Failure Evidence and is not a parent.

## Product witness

In the current r1028 final PDF, Etüde 6 Takt 1 shows the three post-rest right-hand 16th-note groups as visually split/singly flagged although the final Render-MusicXML carries primary-beam intent across alternating presentation voices.

The existing architecture already supplies three independent facts for the three post-rest metric lines:

1. `MetricNotationLine` finds the line without using current `<voice>` labels or beam tags as selectors.
2. `MetricLineBaselineOverlayCounterfactual` can rebind only the candidate sources to the existing same-staff primary channel while preserving every non-candidate channel binding and introducing no new visible same-channel conflict.
3. `UpstreamLineagePreservationAudit` shows that the members already carried one uniform pre-lane representation voice before technical lane/channel splitting.

For the current r1028 E6/T1 witness the three consecutive lines are the source sets:

```text
[0,1,2,3]   interval 96-192
[4,5,6,7]   interval 192-288
[8,9,10]    interval 288-384
```

These source IDs are expected evidence from the frozen parent and are not selectors in productive code.

## New conjunction

r1029 r02 adds `MaterializationSafetyConjunctionAudit` inside the already declared `NOTATION-VOICE-PRE-SERIALIZATION-CONSTRAINT-CLOSURE-AUDIT` body. It reuses the already-authorized AuditOnly baseline-overlay entry point; no new mixed-source call-site or data edge is introduced. No new top-level report section is introduced.

A current-split `MetricNotationLine` receives

```text
MATERIALIZATION_SAFETY_CONJUNCTION_PROVEN_ANALYSIS_ONLY
```

only when all of the following hold simultaneously:

- each candidate member has an exact unique `(measure,pitched_ordinal)` occurrence binding;
- each `(SourceEventId,SoundingAttackId)` has exactly one physical occurrence for this first writer domain;
- all candidate members have one uniform pre-lane voice;
- no visible rest or non-candidate pitched occupancy overlaps the candidate interval on the same staff;
- the current serialized relation is split across primary/secondary presentation channels;
- exactly one existing same-staff primary channel is already anchored by the candidate;
- rebinding only candidate sources to that primary channel realizes one target relation;
- all non-candidate source/channel bindings remain unchanged by construction;
- the overlay creates no new visible same-channel occupancy conflict.

Any missing conjunct remains `BLOCKED_MATERIALIZATION_SAFETY_CONJUNCTION`.

## Important boundary

This is still not a writer authorization and not a PDF-improvement claim. The audit proves only that the S0 source-to-channel rebinding is locally safe under the current first-writer-domain model. It does not mutate `<voice>`, does not touch beam tags, does not call a renderer, and does not create a new StructuralWriteAuthority path.

A later MusicMutation may be considered only after the real Windows run proves the conjunction on E6/T1 and the full corpus shows that the new classification remains conservative.

## Tests

Two generic unit tests are added:

1. a synthetic uniform-upstream, current-split metric line must close the complete materialization-safety conjunction without a write;
2. the same geometry with alternating upstream voice lineage must remain fail-closed.

Expected Strict-test cardinality from accepted r1028: `1336`.

## Acceptance

Early gate: `Etüde Nr. 6`.

Required:

- PRE-TAKE gate green;
- `cargo fmt --check` green;
- Clippy warning budget `0`;
- `1336/1336` Strict tests green;
- Master MusicXML byte-identical to r1028;
- Render MusicXML byte-identical to r1028;
- PDF pixel-identical to r1028;
- no new writer reachability or repair authority;
- E6 contains materialization-safety-proven rows for the three T1 post-rest source sets `[0,1,2,3]`, `[4,5,6,7]`, `[8,9,10]`;
- those rows report exact occurrence binding, uniform upstream lineage, complete visible occupancy, one target primary channel and no new visible channel conflict;
- full-corpus run remains 10/10 and may report other proven candidates, but any blocked case must remain blocked rather than being generalized from E6/T1.

If product artifacts change, a missing proof is silently treated as green, or the audit acquires writer authority, r1029 is rejected.
