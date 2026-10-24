# r1105 CorpusIdentityKodexTestDebtCompileClosure

Parent: **r1103** (`CorpusIdentityTestDebtRefactorSuperCut`)

ChangeClass: **KodexOnly**

## Accepted parent evidence

The current assistant export binds r1103 as accepted with candidate SHA-256
`96153115e465c341ec8cf8c1be468181021c0227c6103ab5d237a137ff606ae1`.
Its strict all-target run reports **1,391 passed, 0 failed**. The complete bound r1103
source snapshot is the only source parent of this take.

## Purpose

r1103 removed grandfathered concrete project / absolute-measure identity from all 156
live non-Kodex normative tests. Exactly **39 live all-target Kodex tests in six Kodex
owners** remained because the current ChangeClass boundary correctly forbids mixing
those files into the preceding RefactorOnly take.

This recovery take closes that live-suite Kodex remainder while preserving the exact test-function
set and each general invariant. r1104 was rejected after apply because two test-only
`DiagnosticCoordinate.measure_number` fixtures used `usize::MAX` although the field is `u32`;
the normal product build therefore passed while the integrated all-target test compilation
returned exit 101. r1105 rebuilds the same semantic cut directly from accepted r1103 and
uses `u32::MAX` for those two unrelated-coordinate witnesses.

The invariant remains:

**real finding -> causal class -> synthetic minimal case -> relocatable invariant**

Concrete corpus identity may be diagnostic input, but it may not be normative test
identity.

## Exact scope

The 39 tests are distributed as follows:

- `src/feedback_structuralization_gate.rs`: 2
- `src/bin/etuschlopf_take_bootstrap.rs`: 2
- `src/bin/etuschlopf_launcher.rs`: 20
- `src/build_gate/identity_debt_scan.rs`: 3
- `src/bin/etuschlopf_launcher/neuro_evidence_export.rs`: 9
- `src/bin/etuschlopf_launcher/assistant_export.rs`: 3

No test function is added, deleted or renamed.

## Transformation

Launcher, bootstrap, assistant-export and Neuro evidence fixtures use synthetic project
labels while preserving all equality, inequality, path, archive, renderer, parsing and
provenance relationships being tested.

The hard-code-negative corpus handover test is strengthened from rejecting one concrete
project example to rejecting the generic `Etüde Nr.` family marker.

The two feedback noninterference tests use a structurally impossible / unrelated
`u32::MAX` diagnostic coordinate instead of assigning an absolute numeric measure
witness in the test source.

The three IdentityDebtScan tests are special: they must still prove that the scanner
rejects concrete identity payloads. Their runtime payload bytes therefore retain the
same forbidden source constructs, but those constructs are assembled synthetically
from symbolic placeholders plus independent numeric arguments. The normative test
source itself no longer carries the concrete identity witness.

## Trust-anchor coordination

The six changed Kodex owners are whole-byte pinned by
`src/identity_gate_trust_anchor.rs`. This take rebinds exactly those six pins from their
final candidate bytes. No unrelated trust-anchor pin changes.

## Corpus-Independence admission proof

Against the same added-line predicates used by
`validate_corpus_independent_test_admission`, the changed source adds:

- **0** concrete Etuede identities;
- **0** concrete numeric measure identities;
- **0** historical regression test symbols;
- **0** non-synthetic corpus-fixture dependencies.

After this cut, the EtuSchlopf `cargo test --all-targets` suite has no remaining
grandfathered concrete-identity tests outside the explicit gate-self-test owners
`src/take_change_control.rs`, `src/identity_debt_inventory.rs`, and
`tests/global_identity_debt_inventory.rs`. Those owners intentionally construct
forbidden payloads in order to prove that the gates reject them; they are not musical
normative witnesses and remain explicitly exempt from Corpus-Independence admission.

## Test-count contract

No `#[test]` function is added, removed or renamed.

Expected post-take all-target total remains:

**1,391 passed, 0 failed.**

A different unexplained total is a blocker.

## External tool boundary

Seven lexical concrete-identity fixture hits remain outside the root EtuSchlopf
all-target suite: four in the frozen qualified SchemaSchlopf-r021 component and three in
the separately compiled Watch tool. They are not silently claimed closed by this
KodexOnly take. The Watch fixtures can be handled by a later non-Kodex refactor; the
frozen qualified component requires its own component/version authority rather than an
in-place mutation.

## Non-scope

This take does not change music, MusicXML, rendered MusicXML, PDF, writer authority,
selector logic, repair scope, thresholds, release policy, test count, qualified
SchemaSchlopf bytes, or Watch-tool bytes.

## Completion condition

r1105 is complete only if the normal Take transaction, semantic-capability diff,
trust-anchor validation, Build, Clippy, post-apply reexec and strict all-target suite are
green at **1,391/0**.
