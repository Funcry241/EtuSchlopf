# r1274 SscK2GovernanceRevisionKodex: proof bundle pre-admission

Parent accepted r1273; KodexOnly; named K2 governance revision. Runtime unchanged.

Why a predecessor is needed: the accepted r1273 checker binds `src/ssc_evidence_proof.rs` (r1270 WAV firewall) and `src/master_state_identity.rs` (r1272 #57 positive binding) by whole-file FNV and freezes the V2 type surface. Admission runs with the parent-compiled checker. RefactorOnly may not change Kodex paths and KodexOnly may not change runtime sources, so the PRESERVING/E036 proof runtime (r1275) needs these bindings widened first, to exactly its reviewed bytes.

Stage-A selector derivation contract (no selector code in this take): `docs/selector_derivation_contract_ssc_preserving_direction_proof_classes.md`. r1275 must reference it unchanged as ParentErrorClassContract.

Admitted change set (exhaustive): proof file {r1273, r1275}; identity file {r1273, r1275}; surface V2 or additive V3 (complete, not hybrid); new bundle contract document. Every other compiled K2 document, OPEN/remaining file, population and #57/#67 binding is unchanged. No enforcement, exemption, selector, ranking or musical authority is introduced.

Installation parent: during the take transaction the incoming checker also validates the r1273 backup tree. The two additive documents may be absent only in that exact parent (revision r1273 and its unchanged K2 checker bytes 0x9b8e5ad37dc56336); any other tree must carry both. An earlier r1274 package (SHA-256 71156a192030efbf222546a9ecd7b735d6d10363748960282092f7108e868ff8) lacked this rule and was rejected at the Root-Cargo.lock resolution check.

Tests: four new K2 tests prove exactly two admitted proof/identity values, rejection of any other bytes, and V2-or-V3-never-hybrid. All prior K2 tests remain byte-identical in assertion content.

Follow-up: r1275 SscPreservingDirectionProofBundle (RefactorOnly) must carry exactly the admitted bytes. A later KodexOnly take may drop the r1273 alternatives after r1275 acceptance.
