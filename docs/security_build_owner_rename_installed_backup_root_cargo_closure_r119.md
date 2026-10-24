# Security Build Owner Rename Installed Backup Root Cargo Closure — r119

This SecurityOnly foundation closes the installed-reexec path-resolution defect exposed by the rejected r118 rename candidate.

The semantic capability inventory already resolves project root control files through `root_file_base(root)`, which transparently selects `root_files/` for a transaction backup tree. The build-owner rename validator previously bypassed that established abstraction and read `current_root/Cargo.toml` directly. During installed reexec, the current/parent tree is the transaction backup where the root manifest is intentionally stored at `backup/root_files/Cargo.toml`; therefore the rename validator failed even though the same tree had already been inventoried successfully.

r119 changes only the rename validator to resolve both current and incoming Cargo manifests through the existing `root_file_base` contract and adds a regression test that reproduces the installed-reexec backup layout. No build-owner rename occurs in this take. No music, writer, renderer, pipeline, report, console, recovery, calibration, take transaction layout, or capability policy is changed.
