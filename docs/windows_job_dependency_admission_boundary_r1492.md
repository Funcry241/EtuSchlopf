# Windows job dependency admission boundary (A0 / r1492)

Parent: r1491 / 0.10.1839. ChangeClass: KodexOnly. This take defines the
future dependency transition only. Cargo.toml, Cargo.lock, run.cmd, launcher,
bootstrap, recovery and all music owners remain byte-identical to the parent.
There is no new unsafe code or Windows runtime module in this take.

## Exact future transition

Only KodexOnly may append exactly the following two lines to the existing
root Cargo.toml. Every existing line, including the package version, stays
unchanged. LF and CRLF represent the same lines, as in ordinary version-only
admission. No other textual variation, dependency or feature is authorized.

```toml
[target.'cfg(windows)'.dependencies]
windows-sys = { version = "=0.61.2", default-features = false, features = ["Win32_Foundation", "Win32_Security", "Win32_System_JobObjects", "Win32_System_SystemServices", "Win32_System_Threading"] }
```

The canonical constant and transition predicate live in take_change_control.
Manifest admission and semantic Cargo-capability admission consume that same
classification. The semantic exception admits only the added root Cargo
windows-sys record after the full exact manifest transition is verified;
all unrelated capability deltas still undergo the existing policy.

A parent already containing the target table or windows-sys entry cannot use
this transition. Removing, replacing, expanding or appending the dependency
again fails closed. Ordinary later package-version-only updates remain legal
while the installed dependency stays unchanged. This is no general dependency
or unsafe admission. A1 must still pass all remaining parent gates.

## Feature preflight

An isolated draft of the A1 Windows boundary was checked with Rust 1.98 for
x86_64-pc-windows-msvc, using exactly the dependency line above. It covers named
CreateJobObjectW/OpenJobObjectW, KILL_ON_JOB_CLOSE without breakaway, current
process assignment, active-process accounting and IsProcessInJob, plus
CloseHandle/GetLastError/GetCurrentProcess. JOB_OBJECT_QUERY requires the
SystemServices feature. The source and command logs accompany the separate
review evidence, outside the incoming Take tree. This is compilation evidence;
Windows 10 process lifetime, nested jobs and abort behavior are not proved here.

The r1491 lock contains exactly one windows-sys package, version 0.61.2,
with windows-link 0.2.1. A0 changes neither manifest nor lock. The later direct
dependency changes the root etuschlopf dependency list in its locally resolved
lock; the existing resolve_root_cargo_lock path remains authoritative.

## Gate owner preflight

The final planned tuples are KodexOnly / Kodex / modified for
src/take_change_control.rs, src/build_gate/semantic_capability_diff.rs and
src/identity_gate_trust_anchor.rs; KodexOnly / Metadata / modified for
ZIP_KIND.txt; and KodexOnly / Metadata / added for this document.
The change manifest declares the exact final payload hashes and excludes its
own self-reference according to the existing contract.

The installed K2 protocol requires the canonical package name
SscK2GovernanceRevisionKodex for changes to its frozen checker files. A0 uses
that existing named-governance route; it changes no SSC rule, population,
retirement document or music owner.

Whole-byte pins for the two changed policy owners belong to
identity_gate_trust_anchor; both are coordinated from final bytes. The
build orchestrator and kodex_build_context consume installed change-control
and semantic inventory. source_integrity_kodex binds the existing module
imports and scanner API; these call sites remain. recovery_transaction_order
binds bootstrap admission and recovery witnesses; bootstrap and run.cmd remain
unchanged. The adversarial boundary gate checks the trust-anchor assertions;
these remain. The retirement, render-sovereignty, receipt-normalization,
destructive-population/carrier and corpus-independent test gates continue to
consume the changed sources without any retirement or suppression.

Synthetic tests exercise exact addition through dependency preflight, full
incoming admission and the semantic gate independently. Negative cases cover
version drift, default features, platform, missing/extra/reordered features,
extra dependencies, existing dependency changes, package edits, duplicate
addition, removal, post-install version change and unauthorized change classes.
The existing recovery tests remain unmodified.

## Migration boundary

After A0 is accepted, build A1 from that accepted parent, then perform explicit
--init-guard, Windows tests, and only after green Windows results A2. A0 does
not implement the guard or repair existing crash windows. Before installing A1,
Otter backs up the complete project including target. During A1 installation:
no second start and no Ctrl+C. A1 supplies the short Windows test guide.
