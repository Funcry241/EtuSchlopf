# r1241 ReleasedLineageShadowTruthClosure

Parent: accepted r1240, candidate 6a5c8c1d2b9ee87689f10fe614c44cbaaab58d29e6cb5af80c9fb7e9f4c41c30. ChangeClass RefactorOnly.

## Four corrections

The existing record_final_master_lineage borrows existing committed opaque event identities and prunes only observational Master records outside that final lineage. Render records remain untouched. No new mint, global commit counter, lineage event, productive ordering or selection is introduced. Releasing a cloned lineage preserves shared identities; identical XML from independent losing candidates does not confer membership.

ADDITIVE wrapper and fragment origin capture require the original receipt pre_fingerprint to match the supplied exact pre-state before retaining a snapshot. The receipt identity remains the key. This is an extra consistency check, not fingerprint-as-identity. Missing capture remains visible in the independent denominator. Frozen ADDITIVE verifier/mint implementation and receipt fields remain unchanged.

The original F9 test moves to a cfg(test) child file of writer_entry_boundary, retaining access to the same private mint without widening production visibility. Test XML no longer contaminates the file's include_str self-audit.

NAOpenGroups counts distinct nonempty open= entries from the unchanged ssc_exit_remaining_v1.txt. All fifteen dependency groups and all normative files remain unchanged.

## WouldDeny boundary

The frozen observer denies complete certificates bound to different borrowed event bindings. The current legitimate visitor creates both positive certificates together under one shared binding; otherwise it emits Unknown. Therefore zero Deny in the corpus is a construction property, not evidence that arbitrary transformations passed an independent negative verifier. This take does not change verdict semantics. Future enforcement requires its own explicit contract for absent, invalid, foreign or stale authorization.

## Validation and acceptance

Synthetic tests cover committed losing candidates with identical output, repeated release of a cloned lineage, ADDITIVE capture with a foreign pre-state, and the existing real-owner origin/adoption/report tests. Compile, Strict Clippy, parent admission and all-target tests must pass before delivery.

Corpus expectation derived from r1240 released-lineage artifacts: Master 841, Render 725, total 1566; WouldPermit 10, WouldDeny 0, Unknown 1556, MissingOriginCapture 0, NAOpenGroups 15. These are acceptance targets, not locally fabricated measurements. Original FLP/WAV inputs and Windows renderer are not available here; actual MusicXML byte equality, normalized/pixel PDF parity and unchanged MasterLineage/RenderLineage rows require the subsequent Windows corpus acceptance.

## Source/test line balance

Rust source and tests: +107 / -17; net +90 lines against accepted r1240.
