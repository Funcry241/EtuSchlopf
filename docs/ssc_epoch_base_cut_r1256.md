# r1256 SSC epoch-base cut

Accepted parent r1255; named SscK2GovernanceRevisionKodex; KodexOnly.

Every live gate path presents only the installed parent and the incoming child: the parent-compiled pre-copy pass, the child-compiled reexec against `transaction/backup`, the build-time current-tree check and source rollback to the parent. No path validates a tree older than the immediate parent. Exceptions that only admit such older trees are therefore removed instead of carried.

The epoch base is the accepted r1255 state: K2 governance installed, SSC evidence/proof contract installed, the V2 proof surface present, #67 relocated and the #57 V2 writer installed. A parent or child below that base is rejected fail-closed with an explicit "older than the epoch base" reason.

Removed historical admissions:

- K2 installation branch r1241 -> r1242.
- Neutral-registry and completion-registry absence epochs for r1242-r1244 and their pinned legacy checker hashes.
- #57 r1250/r1251 contract-group absence epochs and the r1251 checker pin.
- The V1 SSC proof/shadow validation branch and its V1-only validators; a pre-V2 proof surface is rejected.
- The SSC contract installation branch r1238 -> r1239 and the contract-take runtime exclusion.
- The K-R one-shot Legacy -> Relocated #67 consumption permit; a Legacy parent is rejected, and Relocated still cannot revert.
- The #57 V1 construction validator and the legacy -> V2 writer transition. The installed V2 writer is frozen byte-for-byte until a named governance revision defines a later construction contract; the installed writer must carry exactly one bound V2 region.

Thirty removed negative checks are retired append-only in `kodex_capability_retirements.txt` with their exact before-fingerprints. Rewritten tests assert the closed epochs and the unchanged live protections.

Unchanged and still binding: the r1238 exit-dependency baseline and initial remaining set, OPEN monotonicity, the r1250 population baseline with its exact transition predicates and rollback rejection, the K2 checker pin for unnamed takes, all current compiled K2 documents, V2 surface/observer/A4 equality, physical anchor scope, the RSP receipt-surface governance, K-R #67 source proof and no-revert rule, and the trust anchor.

The superseded #57 V1 documents (`ssc_render57_construction_contract_v1.txt`, `ssc_render57_clef_batch_legacy_v1.txt`, `ssc_render57_clef_batch_atomic_v1.txt`) and the V1 shadow fixtures (`ssc_future_type_surface_v1.txt`, `ssc_shadow_observer_future_v1.txt`) are no longer referenced by the checker. They remain physically in this take because the r1255 parent-compiled gate or trust anchor still binds them; a later KodexOnly take may delete them together with their trust-anchor pins.

No runtime source, population status, OPEN entry, #67 status, WAV decision, SSC proof authority or enforcement changes.
