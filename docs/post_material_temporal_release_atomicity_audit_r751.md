# Post-Material Temporal Release-Atomicity Audit · r751

## Scope

r751 is `AuditOnly` and starts directly from accepted r750. It changes no product music and grants no MusicXML writer authority.

## Why this audit exists

The rejected r749 candidate tried to shorten durations already in `midi_seed` and also reordered mixed-duration same-onset groups so that a longer member became the serial chord carrier. r750 evidence shows that this is earlier than the already-bound safe identity frontier: source identity survives the legacy material decision and an exact post-material SourceDuration technical-lane plan is already bound afterwards.

The existing r750 plan reports exact interval cohorts, collision-free lane assignment, lane counts per staff and cross-measure segmentation demand. r751 does not construct a competing plan. It only projects the release-atomic consequence of those existing fields.

## New report statements

Inside the existing SourceSelection causal audit, r751 reports:

- whether one serial lane per staff is sufficient at the bound post-material frontier,
- whether the bound exact-interval plan therefore requires technical voice/lane representation,
- whether cross-measure source intervals require tie/segmentation semantics,
- whether Duration + Segmentation + Tie + technical Voice/Lane materialization form one coupled representation cut at this frontier.

## Fail-closed boundary

This audit does not claim that the post-material plan is already the final MusicXML writer authority. It does not modify `midi_seed`, CandidateMaterialLineage, StructuralWriteAuthority, FinalChordEncoding, tie writers, voice writers or public-artifact release rules. If the bound plan is absent or inconsistent, the new projection reports `AMBIGUOUS_FAIL_CLOSED`.
