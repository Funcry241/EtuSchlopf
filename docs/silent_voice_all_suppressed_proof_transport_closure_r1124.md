# r1124 SilentVoiceAllSuppressedProofTransportClosure

Parent: accepted v0.10.1839 r1123 `CorpusManifestProjectWriteOutcomeFailureEvidenceProjection`. ChangeClass: `RefactorOnly`.

## Purpose

The fresh r1123 full-corpus run is individually green for all ten projects but the hash-bound CorpusRunManifest self-check still fails. r1123 localizes the remaining relation to one nonzero SilentVoice `CandidateRowsAfter` value whose `ExactCandidateDelta` proof does not reach the manifest.

The producer has a concrete control-flow gap. `SilentVoiceVisibleMeasureCoverage` positively checks each planned visible-rest materialization against the bound canonical shadow-rest role. When every concrete replacement is rejected by that proof, the filtered replacement list is empty and the function returns before the later readback block that normally computes `ExactCandidateDelta`. The XML is correctly left unchanged, but the positive reason for withholding every write is lost.

## Change

The existing producer now recognizes only the fully proven all-suppressed case before that early return:

- there was at least one originally planned concrete replacement;
- the canonical-shadow suppression counter equals the number of originally planned replacements;
- the filtered replacement list is empty;
- no independent cross-staff rest suppression is active on this early-return branch;
- source parsing and backup/forward cursor validation are green.

Only in that state the existing audit projects `ExactCandidateDelta=ja` before returning. `candidate_accepted` remains false, `forward_nodes_accepted` remains zero, the source XML is returned byte-identically and no commit/write authority is created. All partial, unbound, parse-failed or otherwise unexplained no-write states remain fail-closed.

The CorpusRunManifest predicate and r1122 residual classification are unchanged.

## Regression transport

The existing SilentVoice writer regression is extended instead of adding another writer-caller function. The added case drives the real producer with a bound frozen voice plan in which every planned rest replacement is independently proven to introduce the canonical shadow-rest conflict. It then checks the actual producer result through the real final-report legacy-mutation relabeling and `CorpusRunManifest::project_evidence` consumer.

The regression requires simultaneously:

- returned XML byte-identical to input;
- `candidate_accepted=false` and zero accepted/replaced forward nodes;
- raw `CandidateRowsAfter` remains nonzero;
- the actual producer line contains `ExactCandidateDelta ja`;
- the manifest consumer projects zero unresolved candidate rows;
- no RenderXmlWrite or NotationMutation is claimed.

With unchanged r1123 producer control flow, the new positive `ExactCandidateDelta` assertion is expected to fail; the r1124 source change is the only intended reason it becomes green.

## Expected corpus effect

The same musical corpus must remain 10/10 project-green. If the r1123 Etüde-2 residual is this all-suppressed path, the same `--all` run should additionally close the CorpusRunManifest self-check and return corpus exit 0. MusicXML, render MusicXML and PDF outputs must remain unchanged. If the corpus still reports an unproven residual, no further normalization is permitted; the next take must use the fresh evidence instead.

## Admission closure r02

The synthetic through-path fixture does not embed literal concrete measure identities in newly added source lines; its measure labels are formatted placeholders, while all semantic assertions remain synthetic and relocatable. The selector derivation contract uses the implementation classification required by the existing Take-Change-Control gate (`SelectorChange: yes`, `SelectorRole: repair_scope`) and references the already frozen parent error-class contract unchanged.
