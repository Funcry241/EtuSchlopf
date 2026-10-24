# r1203 MasterExactPrerequisiteConstructionAuthorityClosure

Parent: accepted/released r1202 `EtudeSpecificAccentReserveAuthorityRetirement`. Rejected revisions are never parents.

## Purpose

The Master super-take (`MasterAdoptedStateGenesisExactBlockSuperClosure`) is meant to prove the new identity architecture. It must not also repair the prerequisites that proof relies on. r1203 closes the two Master prerequisites from population v1 on their own, so that a later delta can be attributed cleanly as construction fix vs. authority change vs. adoption/identity.

## Row 4: `apply_bound_staff_voice_move_authorized` (construction)

Before: `StaffVoiceMoveCapability::apply` replaced the voice value first and then the staff value (or inserted a `<staff>` before `</note>`) at offsets bound against the original source. Whenever the voice value changed its byte length (e.g. `1` → `10`), the staff edit landed at a stale offset and corrupted the note.

After:
- all bound edits are collected and applied in descending start order;
- overlapping ranges are refused;
- readback: the edited note must be one exact note block ending at the arithmetically shifted note end, and its `<voice>`/`<staff>` values must equal the bound replacement values. Otherwise the capability returns `None`, and the unchanged caller contract (`expect`) fails closed.

In the corpus, the only producer keeps the voice value (`target_voice = note.voice`), so the length never changes and the result is byte-identical. The defect is demonstrated by corpus-independent unit tests (voice growth, shrink, staff insertion after a grown voice, equal length, stale source).

## Row 63: `apply_restify_patch` (authority)

Before: a raw `push_str` splice of the restified note without StructuralWriteAuthority, so no bound-source check and no IndividualEditIdentity.

After: the patch delegates to the existing canonical `apply_atomic_restification_batch_authorized` with exactly one edit. The authority binds the exact source bytes, checks exact note → exact rest note with duration, voice and staff preserved, mints the destructive edit identity and consumes once. On refusal the variant stays the unchanged input, which is the behaviour of the already authorized multi-note restify path of the same owner.

The patch producer uses the same eligibility as that multi-note path (not a chord, pitch present, not tie-bound) and the same restify function, so the authority accepts every corpus patch. Expected result: byte-identical.

## Out of scope

- Render rows 57 (clef-move offsets in print cleanup) and 67 (same-lane splice), deferred to Render preparation.
- AccentReserve structural retirement, WAV decision, Proof/Permit, any Adoption/Identity architecture.
- Population freeze gate (next take).

## Expected product delta

None. Master MusicXML, Render MusicXML and PDF of all 10 etudes byte-identical to r1202. Any deviation blocks.

## Kodex notes

- `MusicMutation`, two productive owner files. No new writer root, so the audit WriterReachability of `real_main` strictly narrows (the raw splice `apply_restify_patch` stops being a root of its own and now reaches only the existing canonical batch authority).
- The selector-sensitive lines (ordering/readback checks) are covered by `docs/selector_derivation_contract_master_exact_prerequisite_construction_authority_r1203.md` (role `repair_scope`, basis `authority_or_safety_only`, parent class `semantic_obligation_accounting_failure`).
- Root `Cargo.lock` is not part of the take payload.
