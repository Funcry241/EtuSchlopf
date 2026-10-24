# r1116 StrictAllTargetsReceiptRepositoryObservationBindingClosure

Parent: accepted v0.10.1839 r1115. ChangeClass: KodexOnly.

The existing test receipt omitted managed repository files read by tests outside
of tests/examples/benches. Its repository signature now covers the union of its
previous scope and MANAGED_DIRS / MANAGED_ROOT_FILES from the existing canonical
managed-source-tree owner. All regular files are bound by relative path, length
and bytes; root and recursive symlinks or nonregular entries fail closed.
Missing optional files remain absent, and subsequent creation changes the key.
Root Cargo.lock keeps its LocalTransactional role and existing basis binding.
Product/build and Clippy source signatures are unchanged.

Before physical all-target test execution the old receipt is invalidated and the
complete receipt material is captured. After a successful process the complete
material is captured again. Only exact equality allows persistence of the already
compared bytes. Drift or read/invalidation failure is an error, not a successful
validation claim. A failed process cannot leave the previous receipt reusable.
The receipt contract is versioned v3; v2 receipts do not match.

This is conservative repository observation binding, not complete test hermeticity.
External inputs and transient concurrent changes are not claimed to be covered.
No dependency scanner, registry, musical writer or new lifecycle is introduced.

Synthetic regressions exercise the real signature and receipt owner: content,
addition, removal, rename, invalid root/directory types, unchanged product basis,
stable reuse material, pre-run invalidation and changed repository/binary material.
The existing test/fixture regression and negative-check identities are retained.
