# r1156 SemanticObligationStructuralObservationIdentityShadowCompileClosure

## Classification

- Parent: accepted r1152 `SemanticObligationAccountingGenesisSemanticWorld`
- Rejected predecessors: r1153 (numeric Measure fixture identity), r1154 (missing selector-derivation implementation contract), and r1155 (post-apply reexec compile failure in the read-only `child_text` helper lifetime); none is a parent
- Change class: `AuditOnly`
- Frozen error class changed: no
- Selector-derivation implementation contract: `docs/selector_derivation_contract_semantic_obligation_structural_observation_identity_shadow_r1156.md`
- MusicMutation: no
- Productive selector/classifier: no
- Semantic contract `C`: none
- Obligation genesis / ledger: none
- Release veto: none
- Writer / repair / recovery authority: none

## Purpose

r1152 freezes `semantic_obligation_accounting_failure` and requires later consumers to distinguish the genesis semantic world from the proof procedure. r1153 attempted this consumer-side foundation but was rejected before transaction start because its synthetic test fixture carried a numeric measure label. r1154 removed that fixture identity but was rejected in dependency preflight because the new read-only decision logic also requires a selector-derivation implementation contract. r1155 added exactly that contract and passed the full bootstrap preflight and transactional install, but the post-apply reexec failed in Strict Clippy because `child_text` declared its return value with the XML-input lifetime even though `roxmltree::Node::text()` returns data tied to the node/document borrow lifetime in this call shape. r1156 reapplies the same deliberately weaker consumer-side foundation and selector contract directly to accepted r1152 and changes only that helper return lifetime from `&'input str` to `&'a str`. It observes persisted structure and the availability of persisted identity evidence, but it does not instantiate a musical obligation kind or emit class membership.

The new shadow therefore answers only two questions:

1. Which structural facts can be deterministically re-read from the persisted Master and Render MusicXML bytes?
2. Which explicit persisted identity/correspondence carriers are actually present, without reconstructing identity from physical coordinates?

The answer to question 2 is allowed to be `UNAVAILABLE`. That is evidence about the current architecture, not a reason to infer identity from staff, voice, onset, pitch, measure or document position.

## Content-addressed semantic-world schemas

The human-readable schema labels are not the semantic-world identity. Their canonical content is hashed and the SHA-256 is emitted into every project result section.

Structural observation schema:

```text
Version: semantic-obligation-structural-observation-v1
SHA-256: 1c55abf2bcedded7672f77fca9d263cb6cd85a4b43ff45968f5340c66eaafa4b
```

Identity/correspondence observation schema:

```text
Version: semantic-obligation-identity-correspondence-observation-v1
SHA-256: 61b9e6acc02b9850f20fa2ec9ac41c3080ffba14d7b65bfdb655cc88dcfdba48
```

The Rust tests bind these hashes back to the exact canonical schema strings. A future semantic change to either schema therefore produces a different content identity even if somebody accidentally reuses the readable version label.

## Structural observation v1

The reader consumes only the already available persisted Master and Render MusicXML strings at the final-report Audit owner. It does not request private material-lineage state from a productive mixed-source owner.

The canonical structural projection records, in document order:

- part and measure ordinals as coordinates only;
- MusicXML document-cursor time;
- note onset and duration;
- literal staff and voice fields;
- note `print-object` / `print-spacing` plus derived visibility;
- rest versus pitched material;
- pitch spelling/value fields as observed data;
- chord membership;
- tie start/stop;
- beam states;
- accent presence;
- rest display-step/display-octave;
- forward and backup time transport;
- divisions and time-signature attribute updates;
- clef changes;
- octave-shift start/continue/stop directions.

The projection receives its own SHA-256 per artifact. Staff, voice, onset, pitch and all other coordinates remain structural facts only and acquire no identity authority.

## Identity/correspondence observation v1

The first schema is intentionally conservative.

- It observes explicit note-element `id` attributes if they are present.
- Equal explicit element IDs across Master and Render are reported only as element-ID overlap.
- Generic note-element IDs are **not** promoted to semantic occurrence identity.
- No persisted semantic lane-identity carrier is defined in v1.
- Semantic occurrence correspondence therefore remains `UNAVAILABLE_NO_PERSISTED_SEMANTIC_OCCURRENCE_IDENTITY_IN_SCHEMA_V1`.
- Semantic lane correspondence remains `UNAVAILABLE_NO_PERSISTED_SEMANTIC_LANE_IDENTITY_IN_SCHEMA_V1`.
- Matching measure/staff/voice/onset/pitch coordinates never create correspondence.

This is the intended fail-closed result until a separate architecture step exposes a provenance-bound persisted identity carrier under an appropriate owner and change class.

## Report surface

The normal per-project result gains exactly one read-only section:

```text
SEMANTIC-OBLIGATION-STRUCTURAL-OBSERVATION-SHADOW
```

It reports:

- readable schema versions plus content SHA-256 identities;
- Master and Render parse status;
- structural counts and projection SHA-256;
- explicit element-ID counts/duplicates/completeness;
- semantic occurrence/lane correspondence availability;
- explicit tombstones: no semantic contract, no obligation genesis, no ledger and no release veto.

The report section is evidence only. It does not classify `semantic_obligation_accounting_failure`, because no concrete `C` exists yet.

## Parent-corpus expectation

A direct inspection of the accepted r1152 assistant export predicts that the current ten Master/Render pairs contain no note-level explicit IDs usable even as element-ID overlap. Consequently the expected first corpus result is structural observation available for all ten projects while semantic occurrence and lane correspondence both remain unavailable.

That result is useful: it establishes the next architecture boundary without filling it by coordinate matching.

## Regression coverage

Four AuditOnly tests are added:

1. the canonical schema content hashes remain pinned to the declared SHA-256 identities;
2. structural projection observes rests, forward/backup time, accents and octave-shift facts without a semantic classifier;
3. explicit element IDs can be observed without becoming semantic occurrence/lane lineage;
4. equal physical coordinates without explicit identity do not create correspondence.

A full Windows `--take --all` run is still required for admission because this Linux environment has no Cargo toolchain. The added selector-derivation contract is governance-only and grants no productive selection or write authority. Product Master MusicXML, Render MusicXML and PDF geometry are expected to remain unchanged; only `.result.txt` gains the new shadow section.

## Compile closure from rejected r1155

Rejected r1155 reached post-apply reexec and failed with one compiler error at `child_text`: the helper promised `Option<&'input str>` while the returned text carried lifetime `'a`. r1156 changes only that helper signature to `Option<&'a str>`. No observation rule, schema content, selector derivation, report field or authority boundary changes. The schema SHA-256 identities therefore remain unchanged.

## Next boundary

r1156 does not authorize the next step. If the corpus confirms that semantic occurrence/lane identity is unavailable in persisted artifacts, the missing persisted identity transport must be treated as its own architecture boundary. It must not be smuggled into an `AuditOnly` take by adding a private-state bridge from a mixed productive source.
