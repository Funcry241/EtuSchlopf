# r1108 TakeFailureEvidenceFoundationArchiveClosure

## Parent and recovery lineage

Parent: canonical accepted r1105 `CorpusIdentityKodexTestDebtCompileClosure`.

ChangeClass: `KodexOnly`.

r1106 and r1107 are rejected diagnostic evidence only and are not parents. r1108 reapplies the generation-bound Failure-Evidence foundation directly on accepted r1105 and closes the Assistant-export archive boundary exposed by the rejected r1107 attempt.

## Generation boundary

The bootstrap that opens a Take transaction remains sovereign through commit or rollback. The installing r1108 Take therefore cannot retroactively protect its own installation attempt with the incoming bootstrap. After r1108 is accepted, the r1108 bootstrap becomes the accepted parent bootstrap for the following Take and the Failure-Evidence capability is operational from that following Take onward.

Invariant:

`BootstrapAuthority(Take N) = AcceptedParentBootstrap(N-1)`.

## Failure-Evidence foundation

For a later Take, after the incoming source is applied, verified and local Cargo.lock resolution is complete, the accepted parent bootstrap binds the exact installed candidate source before reexec. Persistent evidence is keyed by Take revision plus candidate SHA outside the transactional source tree under `target/etuschlopf_take_rejection_evidence_v1`.

The child receives only an attempt-local write-ahead directory. Launcher stage plus strict Clippy and strict all-target evidence are registered by typed role before the corresponding operation. The parent does not scan arbitrary log directories and does not use file modification time as attempt identity.

Rollback and canonical Rejection remain first-order authority. Evidence finalization runs only after rollback and cannot suppress rollback or canonical Rejection. Historical Rejections without a persistent capsule remain explicitly `legacy_evidence_unavailable`.

## Archive closure

The current Failure Assistant export and later post-Acceptance exports may contain exactly one additional top-level subtree: `take_attempt_evidence/`.

The Assistant ZIP writer now admits recursion only below that named subtree. Traversal is deterministic, symlinks and non-file/non-directory entries are rejected, depth is bounded, relative path segments are validated and the existing ZIP-path safety validator is reused. This is not a generic recursive-export permission.

If `rejection_suffix` is empty, `take_attempt_evidence/` is removed and not recreated. A normal green post-Acceptance export with no rejected suffix therefore remains structurally unchanged.

## Regression witnesses

The existing failure-handover regression now requires the nested `take_attempt_evidence/.../REJECTION_EVIDENCE_METADATA.txt` member to be present in the emitted ZIP. The existing post-Acceptance regression requires both staging and ZIP to contain no `take_attempt_evidence/` entry when the rejection suffix is empty.

No additional test function is introduced by this archive closure. Together with the two foundation tests, the expected strict all-target inventory remains 1,393 tests; the real Windows run is authoritative.

## Non-authority

No MusicMutation, Writer, Repair, Selector, Threshold or Release authority is added. Evidence remains diagnostic with `authority=none`.
