# r572 PhraseNotationPolicyPersistenceSemanticCapabilityClosure

Parent: accepted r569.

This is a fresh rebuild of the productive phrase-notation super-take after the
rejected r570 and r571 candidates. Neither rejected candidate is a parent.

The r572 cut keeps every Audit-owned source and the release orchestration source
byte-identical to r569. Productive phrase selection and persistence live only in
StructuralWriteAuthority.

Policy:

- input eligibility remains the already-existing `allow_structural_pending_visual` projection;
- the current pre-phrase Render-MusicXML is independently projected into part/staff/voice attack lanes;
- a pending span is authorized only if the same complete intrinsic melodic form
  (relative pitch-centroid deltas plus chord-member counts) occurs in a second,
  non-overlapping window of the same lane;
- rhythm is not part of this particular positive evidence class;
- cross-lane-only repetition, rhythm-only repetition, fuzzy similarity, scalar
  scores, edit distance, ML, project-specific selectors, attack-count limits and
  measure-span limits grant no authority.

Writer transaction:

- the already-established phrase writer root builds an in-memory candidate;
- parseability, note cardinality, sounding/timeline fingerprint and exact slur
  pair balance must hold before persistence;
- only then is Render-MusicXML written through StructuralWriteAuthority;
- the persisted bytes are independently reloaded through a neutral technical
  MusicXML loader and the same invariants are checked again;
- failure restores the pre-phrase Render-MusicXML bytes and aborts;
- Master-MusicXML phrase content is never written by this cut.

Semantic-capability closure:

- `src/pipeline_shards/pipeline_orchestrator_release_support.rs` is byte-identical
  to r569, so the existing ProofReentryReader surface is not modified;
- no function that contains proof/report/result/manifest/evidence/readback
  semantics directly opens or reads a file;
- the neutral technical loader contains only MusicXML loading semantics and does
  not consume proof/report artifacts;
- no calibration threshold is introduced or retired.
