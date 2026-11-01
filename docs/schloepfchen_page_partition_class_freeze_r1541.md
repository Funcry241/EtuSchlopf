# r1541: Schloepfchen page-partition class foundation

Parent: accepted r1540 AlignedHalfBarNotePreservation, v0.10.1839.
Accepted Take SHA-256:
`eb4391d33c0b05eaf5e2c05bbd4f83ce2a7fc0fbfed21830d4d2f9963b3d67e8`.
The matching acceptance receipt and lifecycle handover accompany the current
export. Its transition_history.json is explicitly an older pre-acceptance
advisory snapshot; it does not override that later acceptance receipt.

## This Take

ChangeClass: KodexOnly. Runtime, Rust sources, tests, tools, dependencies,
calibration, trust pins and gates remain byte-identical to r1540. The only
changes are Take identity, this explanation and the new error-class definition
`docs/selector_derivation_contract_redundant_page_retention.md`, plus the
mandatory manifest replacing the old transition declaration.

This is the prerequisite first Take for the new class P. It activates no page
compaction. A normal run is expected to retain the accepted r1540 notation and
page distribution. The isolated four-page PDF experiment is not installed as
an alternative source or represented as a productive result of this Take.

## Why a separate first Take is necessary

The accepted parent's src/take_change_control.rs requires a selector-sensitive
implementation to reference an error-class contract already present unchanged
in the accepted parent. The parent class cannot be added or modified together
with its implementation. The newly agreed page-retention class is not the
existing renderer_page_partition_nonrealization class: that existing class
observes a submitted plan that the consumer does not realize. Here a retained
plan may be fully realized while a less extensive same-system assignment is
possible. Reusing that unrelated class would misstate the cause and authority.

The independent existing horizontal nonrealization contract likewise does not
license a new page preference. Neither it nor the page nonrealization contract
is modified. The existing safety rule is followed without an exception.

## Subsequent work

Only after native acceptance of this exact Take may a separate implementation
reference its class definition unchanged. No later revision is reserved here.
That implementation must preserve the ordered systems for class P, retain the
existing plan/materializer authority, stay inside the admitted frontier, obtain
fresh complete renderer readback and keep musical and readability proofs.

Explicit vertical-spacing mutation is class V; horizontal regrouping is class
H. They are outside this first scope. The full formula still requires verified
minimum readable widths and distances; measured example gaps are not defaults.
No particular page count, title-page exception or corpus identity is a selector.

## User run

Apply this source Take through the existing `run.cmd --take --all` workflow in
the accepted r1540 project. This Take contains no executable and no root
Cargo.lock. The existing user-side Windows gates own installation and
acceptance. Git and public releases are outside this Take.
