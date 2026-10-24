# r1449 — adoption-boundary audit and next implementation

Parent is accepted r1448, Take SHA-256
`87dc3edfd09d50b8beffbcfcab2e34a9adcf4d41a94857efa91b02f8fac7afe7`.
This take adds the separate normative proof regime and this audit, plus take
metadata. It changes no Rust byte, existing contract, validator or executable
configuration. It does not claim closure of the custody/adoption hole.

## Source findings

`master_adoption/adopted_state.rs` carries `SelectedCustody` as an observer.
`continued` retains the first Unknown; both materialized and in-place adoption
still commit the original owner receipts and document. Its existing regression
test explicitly allows unsupported observational continuation to remain Unknown
after an otherwise accepted owner edit. The r1377/r1379 observational contracts
forbid turning that diagnostic directly into an adoption veto.

`master_adoption/render_adopted_state.rs` carries document and Render lineage,
with issuer/root/epoch freshness, exact replay, sink, witness and commit checks.
It does not carry `OwnedRhythmicContinuation`. `RenderMasterEpoch` binds the
sovereign Master bytes and lineage head, but not its received course custody.
Consequently, sovereign byte/receipt correctness does not itself establish
received-course continuation through the Render path.

`RenderTransitionReceipts` distinguishes document, fragment, generated,
enclosing and instruction placements. Its block adapter extracts the original
block receipt from `CapturedBlockReceiptInput`; the comment correctly states
that beam observation is not an adoption capability. Such receipt handling must
not be mistaken for an existing general rhythmic custody transport.

The normal Render adoption path verifies the planned candidate before the
callback. The FinalNotation checkpoint may return its own #58 receipt from that
callback; it is extended and verified after the write. The current sovereignty
contract covers that sequence. A new pre-write custody proof must explicitly
prepare this additional owner event before a productive write, retaining the
same authority and readback checks. Adding a guard only to `commit` is insufficient.

## Native observation, not a classifier

The supplied repeat export `assistant_export_latest(20261018-173102).zip` contains
one selected-custody Unknown line each for Etude 2 and 3 at
`MasterXmlReplacementRequest`, reporting editorial consolidation support refusal.
Both later report complete selected custody. These log lines alone do not prove
healing, because the project label does not identify the candidate instance.

The source `run_editorial_voice_decision_pass` creates each variant on
`best.clone()`, applies its request, and only then evaluates acceptance. Therefore
an observation at that request is not evidence that the variant became `best`.
This audit does not infer the exact native winning variant from those lines and
does not reclassify a failed trial as a product failure. A runtime implementation
must establish selection at the actual transition, with candidate-local proof.

## Sequence and evidence limits

The proof regime is frozen in this KodexOnly parent before runtime activation.
Native acceptance of r1449 precedes release of a runtime child. There is no new
runtime child in this package, no pin exception and no reserved successor revision.
The implementation must carry original captures, introduce the opaque proof,
separate candidate construction from selected adoption, and prove refusal before
each productive sink. Its scope and any required bounded governance transition
must be reviewed against the actual successor bytes.

The temporal boundary from #56 and the independent system-break phrasing issue
remain separate. The accepted r1444 output reference is not rebased. Local source
neutrality and package-admission evidence are recorded outside the take to avoid
self-referential package hashes. Native tests, corpus rendering and output
comparisons are claimed only when a fresh supplied native export proves them.
