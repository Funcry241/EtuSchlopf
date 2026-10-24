# r1057 TakeTransitionHistoryFoundation

## Classification

- Parent: accepted r1056 `TakeEvidenceFoundationResidualClippyClosure`
- Parent Take SHA-256: `7522f442292706a7cc3d7b938f360265560075f352b4bfe965bd098aefcc70ef`
- Change class: `KodexOnly`
- MusicMutation: no
- Productive music logic: unchanged
- Acceptance/Rejection authority: unchanged
- Transition History authority: advisory evidence only

## Parent proof and timing boundary

The r1056 assistant export is an in-reexec snapshot: it already contains the r1056 source snapshot and green run evidence while the r1056 transaction can still be in `reexec`. Canonical r1056 Acceptance is established only afterward by the parent bootstrap through the existing acceptance ledger. The post-run handover confirms the durable `accepted-r1056-7522f442292706a7cc3d7b938f360265560075f352b4bfe965bd098aefcc70ef.txt` marker and no remaining pending transaction.

Therefore this Take must preserve the distinction:

```text
SourceSnapshot != Acceptance
RunSeverity    != Acceptance Authority
in-flight      != canonical Verdict
```

## Change

The Assistant Export gains one machine-readable artifact:

```text
transition_history.json
```

The projection reads existing durable Take lifecycle evidence only. It does not write or modify lifecycle evidence.

Canonical history records are projected from:

```text
target/etuschlopf_take_rejection_ledger_v1
```

Acceptance records remain authoritative only because the existing `accepted-r...txt` files already are canonical lifecycle records. Rejection records remain authoritative only because their existing SHA-bound ledger directories already are canonical rejection records.

Where a matching advisory Acceptance Evidence sidecar exists under:

```text
target/etuschlopf_take_acceptance_evidence_v1
```

its exact identity is verified before its Candidate Intent and Semantic Effect evidence are associated with the accepted transition.

## Effect representation

Candidate Intent and rejection reason are copied as exact UTF-8 evidence when available and are additionally SHA-256 bound.

`semantic_effect_snapshot.tsv` is not duplicated into Transition History. The projection records only:

```text
status
bytes
sha256
nonempty_rows
```

This keeps the history compact while preserving exact byte identity of the existing Effect artifact. Transition History does not reinterpret Effect rows or infer a semantic correction class.

## In-flight boundary

The current Take transaction is projected separately under:

```text
current_in_flight
```

When the pending marker exists, the record is explicitly:

```text
authority = none
canonical_verdict = not_yet_canonical
```

The source revision/name, transaction state, candidate SHA/revision, Candidate Intent and candidate Effect fingerprint may be observed, but the current Candidate is never inserted into canonical history before a durable Acceptance or Rejection record exists.

For r1057 itself, the expected Assistant Export timing is therefore:

```text
canonical_history_through_revision = r1056
current_in_flight.take_revision     = r1057
current_in_flight.transaction_state = reexec
current_in_flight.canonical_verdict = not_yet_canonical
```

After r1057 is later canonically accepted, a subsequent export may naturally project r1057 as historical Acceptance because the already-existing lifecycle authority has then become durable.

## Legacy evidence

Legacy records are not migrated and missing facts are not guessed. If an old rejection has no `take_revision` or `rejection_stage`, Transition History emits `unknown` / `not_applicable_or_legacy_unknown`. Missing Acceptance sidecars remain `not_present` rather than being reconstructed from filenames, docs or later knowledge.

Malformed or incomplete historical evidence becomes a non-authoritative `projection_issues` entry. It does not create or alter an Acceptance/Rejection verdict.

## Assistant Export integration

`transition_history.json` is generated for Normal, Corpus and VisualPreflight Assistant Exports. In VisualPreflight it becomes a required root artifact, is indexed from `assistant_context.json`, and the Assistant Context schema advances additively from 11 to 12.

No Take package, ledger schema, recovery order, semantic capability rule, music writer, renderer, MusicXML or PDF path is changed.

## Regression witnesses

The launcher tests add witnesses that prove:

1. durable accepted/rejected records are projected as canonical history;
2. verified Acceptance Evidence augments an accepted transition without becoming authority;
3. an open `reexec` Candidate is emitted only in `current_in_flight` with `not_yet_canonical`;
4. the same Candidate is not prematurely emitted as an accepted historical transition;
5. legacy missing revision/stage facts remain explicitly unknown.

## Governing invariant

```text
Past durable verdicts -> structured facts
Current candidate      -> observation only
Structured facts       -> evidence only
Evidence               != Authority
```

## Explicit non-goals

r1057 does not:

- change Acceptance or Rejection criteria;
- change the Take bootstrap or lifecycle ordering;
- migrate historical ledgers;
- infer missing historical identities;
- classify textual rejection reasons into new semantic classes;
- duplicate historical Semantic Effect payloads;
- build the Materialization Contract;
- build the deterministic Materializer;
- build Candidate synthesis;
- build a Parent Verifier;
- resume Beam repair work;
- mutate music, MusicXML, render MusicXML or PDF output.

## Next boundary

Only after this projection is accepted should the next architecture step define which Candidate/package fields are semantic input and which are deterministic materialization output.
