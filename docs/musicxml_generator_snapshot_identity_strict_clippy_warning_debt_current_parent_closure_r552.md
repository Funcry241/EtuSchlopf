# r552 – MusicXmlGeneratorSnapshotIdentityStrictClippyWarningDebtCurrentParentClosure

## Purpose

Close the sole strict Clippy warning left by the successfully installed r550 MusicXML generator/snapshot identity provenance slice, using the actual installed r550 state as parent.

## Parent and rejected-candidate handling

- Parent: `r550`.
- `r549` remains rejected and is not a parent.
- `r551` was rejected by Take-Change-Control because it was incorrectly declared against r547 after r550 had already committed; it is not a parent.

## Productive effect

None. The only Rust change is the mechanical Clippy-equivalent replacement:

```rust
files.sort_by_key(|left| normalize_path(left));
```

for the previous comparator form. The normalized-path sort order and generator source-fingerprint input set remain unchanged.

No writer, music, MusicXML version, XSD, supports, dependency, Cargo.lock, repair authority, XML metadata, or productive semantic surface is changed.

## Expected gate result

The already proven r550 GeneratorIdentity / SnapshotIdentity contract remains qualified while strict Clippy warning debt returns to zero. XSD 3.1 conformance remains unproven and `WriteAllowed` remains `no`.
