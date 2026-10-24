# r1192 Occurrence Receipt Epoch Super Closure — Selector-Contract Recovery

## Classification

- Parent: accepted/released r1188 `DestructiveEditAttributionSuperClosurePackagingRecovery`
- Change class: `RefactorOnly`
- Music mutation: no
- Productive selector change: no
- Selector-sensitive audit classification: yes, governed by `docs/selector_derivation_contract_occurrence_receipt_epoch_lineage_r1192.md`
- Writer authority change: no
- Proof/Permit authority: no
- Final-artifact path authority: existing `SourcePlan.render_musicxml_out` only
- Recovery: rejected r1189, r1190 and r1191 are not Parents; r1192 is based directly on accepted/released r1188

## Recovery reason

r1189 was rejected before installation because two incoming FNV64 declarations did not match the packaged bytes. r1190 corrected all incoming hashes but was also rejected before installation because the new read-only receipt-lineage classifier is selector-sensitive under the accepted Proxy-Identity Selector Kodex and therefore requires exactly one implementation contract derived from a causal error-class contract that already exists unchanged in the accepted parent.

r1192 carries the same Occurrence/Receipt/Epoch implementation directly from accepted r1188 and adds exactly that missing governance contract. It does not use either rejected candidate as a Parent and introduces no additional product decision logic beyond the already proposed audit classifier.

## Accepted parent evidence

The accepted r1188 full corpus established opaque edit attribution for the frozen caller-supplied replacement authority class. Across ten projects it observed 9,475 individual edit inputs, of which 9,451 were actually applied and therefore `EXACT`; `EditBATCH_ONLY` was 0, `EditORIGIN_ONLY` was 24 and attribution-integrity errors were 0. The 24 origin-only inputs were not applied.

r1188 deliberately stopped at successful in-memory application. It did not bind an applied identity to the final persisted Render-MusicXML epoch.

## Purpose

r1192 extends only the receipt lineage of the exact r1188 `IndividualEditIdentity`.

At successful authority application the identity is bound to the complete input and output document states. If a later attributed authority transaction consumes that exact output state, existing receipts are transported into the successor state. This is an exact state succession; no physical occurrence lookup is involved.

Project setup also binds the audit session to the already authoritative final Render-MusicXML output path from `SourcePlan`. The existing Render-Epoch recorder then presents every observed write boundary to the receipt ledger. Only a write whose path is exactly that configured final artifact path can establish or advance a final-epoch receipt.

## Receipt classes

`CONTINUED`
: The complete document state produced by the applied edit is itself observed at the final Render artifact epoch.

`REBASED`
: The receipt crosses one or more exact attributed in-memory document-state transitions before the descendant state reaches the final Render artifact epoch.

`RESEALED`
: A receipt-bearing state already belonging to the exact final Render artifact stream crosses a later explicitly observed persisted write on that same final path. This preserves historical receipt identity across a representation epoch; it is not a claim that the original edit effect is still visible.

`UNKNOWN`
: No identity-safe path from that applied edit receipt to the final observed Render artifact epoch exists in the available transport. The audit must leave it unknown rather than reconstructing identity from physical XML coordinates.

## Forbidden recovery

r1192 must not infer identity from measure, staff, voice, onset, duration, pitch, XML range, note ordinal, proximity, function name, report label, project identity or corpus uniqueness. The final path is not guessed either. It is transported directly from the existing SourcePlan into the observation session before writes begin.

If more than one identity-safe lineage history reaches the same receipt, the audit retains only provenance that is common to independently produced equal states and reports the most conservative already-proven lineage mode for that same receipt. It never chooses a producer by order, rarity, location or corpus uniqueness.

## Interpretation boundary

The receipt is historical provenance. A final receipt proves only that the same opaque edit identity has an identity-safe receipt lineage into the final observed Render artifact epoch. It does not prove that the edit's semantic effect survives unchanged, that the edit remains visible or necessary, that the original decision had sufficient evidence, that a later ProofType exists, or that the edit deserves a write Permit.

Those questions remain outside this take.

## Accounting closure

Every `Applied` edit is terminally classified into exactly one of:

`CONTINUED | REBASED | RESEALED | UNKNOWN`

The report emits `AppliedEdits`, `Accounted`, `AccountingMismatch` and `ReceiptIntegrityErrors`. Acceptance requires `AccountingMismatch = 0` and `ReceiptIntegrityErrors = 0`; it does not require `UNKNOWN = 0` because missing identity-safe evidence is allowed to remain unknown.

## Synthetic regressions

Five read-only regressions cover direct continuation, exact in-memory rebasing, explicit final-path resealing, shadow-only unknown, and equal-byte independent producers retaining only their common receipt intersection.

## Acceptance

The take must pass dependency preflight, Take-Change-Control, format, tests, build, Clippy and full 10/10 corpus. Master and Render MusicXML must remain byte-identical to accepted r1188. PDF visible geometry must remain stable; volatile PDF metadata bytes are not part of this RefactorOnly invariant.

## r1192 mechanical re-execution recovery

r1191 reached re-execution but strict Clippy rejected `contains_key` followed by `insert` under `-D clippy::perf`, while the zero-warning receipt gate also observed `too_many_arguments` on the transition observer. r1192 changes no lineage semantics: the state-receipt insertion uses the BTreeMap entry API, and the already coupled complete-document before/after values are passed as one transition pair so the observer has seven arguments.
