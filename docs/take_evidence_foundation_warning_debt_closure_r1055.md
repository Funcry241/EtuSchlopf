# r1055 TakeEvidenceFoundationWarningDebtClosure

## Classification

- Parent: accepted r1054 `TakeEvidenceFoundationPrewire`
- Parent SHA-256: `13c4ac5893f9171294c6ef8323e5633efac4254ef018d596ebe4ff096b416b21`
- Change class: `KodexOnly`
- MusicMutation: no
- Productive music logic: unchanged
- Evidence semantics: unchanged
- Acceptance/Rejection authority: unchanged

## Parent proof

The fresh assistant export contains the canonical acceptance marker

`target/etuschlopf_take_rejection_ledger_v1/0.10.1839__0.10.1839/accepted-r1054-13c4ac5893f9171294c6ef8323e5633efac4254ef018d596ebe4ff096b416b21.txt`.

The exported source snapshot is the accepted r1054 source and still contains the three Clippy `too_many_arguments` warnings introduced by r1054 in:

- `multi_revision_evasion::record_rejection` (9/7),
- `rejection_error_with_persistence` (9/7),
- `rollback_then_record_rejection` (10/7).

Therefore this take is a mechanical warning-debt closure on top of accepted r1054, not a replay of r1054/r02.

## Change

The already accepted rejection facts are grouped into a typed `RejectionRecord` value containing exactly the same fields that r1054 already persisted:

```text
from_version
to_version
take_revision
zip_sha256
rejection_stage
candidate_intent
snapshot
reason
```

`record_rejection` now receives `project_root + RejectionRecord`. The two bootstrap helpers receive the same typed record instead of duplicating its individual fields.

No field is added, removed, reordered semantically, re-derived, ranked or given new authority.

## Governing invariants

```text
Evidence = Observation
Evidence != Authority
```

and

```text
r1054 evidence semantics
==
r1055 evidence semantics
```

The following remain unchanged:

- first-authoritative `rejection_stage`;
- exact Incoming candidate intent;
- existing `semantic_effect_snapshot.tsv`;
- schema-2 rejection metadata and legacy schema-1 readability;
- canonical `accepted-r...txt` Acceptance authority;
- advisory Acceptance sidecar semantics;
- r1053 rollback/rejection durability ordering.

## Warning-debt target

Expected strict-Clippy delta:

```text
too_many_arguments: 3 -> 0
architectural_budget: 3 -> 0
```

No lint suppression is added. No Clippy threshold or policy is weakened.

## Test expectation

No new behavior is introduced, therefore the strict test inventory is expected to remain at the r1054 count. All r1054 Evidence Foundation regression witnesses must remain green.

## Explicit non-goals

r1055 does not change Candidate selection, Acceptance, Rejection, recovery ordering, Beam logic, music logic, writer authority, repair authority, renderer behavior, MusicXML, PDF output, Evidence schemas, or the r1054 Evidence Foundation contract.
