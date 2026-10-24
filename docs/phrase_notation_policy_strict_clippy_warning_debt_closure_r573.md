# r573 – PhraseNotationPolicyStrictClippyWarningDebtClosure

## Purpose

Close the single current Strict-Clippy warning introduced by the accepted r572 phrase-notation policy/persistence slice.

## Parent

Parent is the accepted r572 `PhraseNotationPolicyPersistenceSemanticCapabilityClosure`. The rejected r570 and r571 candidates remain non-authoritative.

## Change

The only Rust change mechanically collapses one nested condition in
`src/structural_write_authority/phrase_notation_policy.rs` exactly as suggested by
`clippy::collapsible_if`:

- the existing guards `visible && duration > 0 && !is_rest` remain unchanged;
- the existing `note_pitch_midi(child)` `Some` requirement remains unchanged;
- the body that records `PolicyRawNote` remains unchanged.

This is a syntax-only control-flow refactor. It changes no phrase candidate, no
intrinsic signature, no repetition evidence, no NotationPolicy decision, no
writer authority, no MusicXML mutation, no threshold, no scoring, no lane rule
and no persistence invariant.

## Warning-debt closure

The r572 full-corpus export is 10/10 OK and final-clean at project level but the
typed run outcome is yellow because the Strict-Clippy log contains exactly one
`clippy::collapsible_if` warning in this file. r573 closes exactly that current
warning debt and declares `warning_debt_closure=closed_current_warning_debt_v1`.

Expected result after a fresh run: Strict Clippy `TotalWarnings: 0`; with the
already-green project/corpus conditions preserved, the warning-debt cutover no
longer downgrades the typed release projection from released/dark-green.
