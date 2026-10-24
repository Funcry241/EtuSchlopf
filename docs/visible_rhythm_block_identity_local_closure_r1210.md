# r1210 VisibleRhythmBlockIdentityLocalClosure

## Scope

One transformation identity only: frozen destructive-population row #52, `src/arrangement.rs::normalize_visible_rhythm_musicxml`, classified as `DESTRUCTIVE / BLOCK_DOCUMENT / M`.

## Contract

`MasterBlockIdentity::bind_visible_rhythm_normalization(pre_xml, post_xml)` binds one execution of the normalizer to:

- opaque `block_id`;
- owner locator;
- origin locator;
- frozen effect class `DESTRUCTIVE`;
- frozen identity kind `BLOCK_DOCUMENT`;
- parameter-contract digest anchored to population-v1 row #52;
- pre-document fingerprint;
- post-document fingerprint.

`normalize_visible_rhythm_musicxml` creates this identity for every execution, including a no-measure no-op execution where pre and post bytes are equal. `ArrangementResult` transports the identity as `visible_rhythm_block_identity`.

At the existing productive `build_atomic_candidate` call site, the transported identity must exist and validate against the exact XML before and after normalization. Failure propagates fail-closed through the existing `anyhow::Result<SolverCandidate>` path.

## Deliberate boundary

This is not yet the global Master lineage. The BLOCK receipt is not stored in `SolverCandidate`, because the transformations preceding a later atomic normalization are not yet all represented. r1210 therefore establishes the BLOCK carrier/validation semantics without skipping an unmodelled predecessor in an ordered chain.

No transformation is added or removed, and population-v1 itself is unchanged.

## Non-goals

No current-Master identity, no EXACT receipt, no ordered `(EXACT|BLOCK)*` vector, no swap/drop adoption change, no writer relocation, no selector/ranking change, no Render-to-Best change, no Render AdoptedState, no WAV, no AccentReserve, no Proof/Permit.
