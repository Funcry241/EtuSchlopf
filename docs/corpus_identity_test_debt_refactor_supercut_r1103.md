# r1103 CorpusIdentityTestDebtRefactorSuperCut

Parent: **r1102** (`CompletedSchemaSchlopfMusicXml40TransitionRetirementLedgerClosure`)

ChangeClass: **RefactorOnly**

## Accepted parent evidence

The current assistant export binds r1102 as accepted. Its strict all-target run reports
**1,391 passed, 0 failed**. r1103 is built directly from the complete r1102 source
snapshot, not from rejected r1101 or any candidate branch.

## Purpose

E32 already rejects newly introduced normative test evidence that binds correctness to a
concrete EtuSchlopf corpus project or to an absolute measure witness. The accepted tree,
however, still contains grandfathered tests written before that admission boundary.
This take begins the physical closure of that asymmetry.

The invariant is:

**real finding -> causal class -> synthetic minimal case -> relocatable invariant**

A concrete project label or absolute score location may motivate a test, but it may not
survive as the test's normative identity.

## Exact live-suite inventory at r1102

The r1102 all-target run contains **195** test functions that still match the current E32
concrete-identity admission predicates:

- **49** carry a concrete Etuede identity;
- **147** carry a concrete numeric measure witness;
- **1** is in both sets.

Seven additional lexical hits live in separately compiled tool / qualified-component
test source and are not part of the EtuSchlopf `cargo test --all-targets` total.

The 195 live tests split by current Take path authority:

- **156 non-Kodex tests** (this take);
- **39 Kodex tests** (explicit later KodexOnly closure; not mixed into this RefactorOnly
  take).

A single take cannot legally modify both groups under the current ChangeClass contract.
The split is therefore an authority boundary, not an attempt to leave debt behind.

## This take

r1103 rewrites the 156 non-Kodex tests in **40 source files** so that their existing
invariants no longer use a real corpus project label or a source-level absolute measure
witness as normative identity.

For project identity:

- concrete `Etüde Nr. N` witnesses are replaced by synthetic project labels;
- equality / inequality relationships inside each test are preserved;
- no production selector, manifest parser, report parser or path behavior is changed.

For measure identity:

- numeric MusicXML measure values remain only fixture-local schema data;
- their source representation is detached from the normative `<measure number=N>`
  witness form while preserving the exact runtime XML bytes;
- no source-event, rhythm, writer, layout, parser or report expectation is changed;
- no real Etuede/takt provenance is introduced as replacement evidence.

This is test-evidence syntheticization only. Productive code paths are unchanged.

## Admission proof

Against the same line predicate used by
`validate_corpus_independent_test_admission`, the r1103 added lines contain:

- **0** concrete Etuede identities;
- **0** concrete measure identities;
- **0** non-synthetic corpus-fixture dependencies introduced by this cut;
- **0** selector-sensitive added-line sets;
- **0** multiplicity-sensitive added-line sets.

After this phase the only live all-target concrete-identity tests are the **39 Kodex
owners** intentionally deferred to a separate KodexOnly take. The seven non-root-tool
hits are outside the EtuSchlopf all-target suite and remain outside this take's authority.

## Test-count contract

No `#[test]` function is added, removed or renamed in the 40 changed files.

Expected post-take all-target total remains:

**1,391 passed, 0 failed.**

A different unexplained total is a blocker.

## Non-scope

This take does not:

- change music, `.musicxml`, `.render.musicxml` or PDF output;
- change a writer, selector, threshold, repair or release authority;
- delete useful general tests merely because their old fixture used a numeric MusicXML
  measure number;
- modify Kodex paths;
- modify the frozen SchemaSchlopf-r021 qualified component;
- claim full zero identity debt before the later 39-test Kodex closure.

## Completion condition

This phase is complete only if normal Take acceptance, Build, Clippy, post-apply reexec
and all-target tests are green at **1,391/0**. The next step is then a separate KodexOnly
closure of the remaining 39 live-suite concrete-identity tests from the newly accepted
parent.
