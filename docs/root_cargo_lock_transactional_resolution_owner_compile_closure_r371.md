# Root Cargo.lock Transactional Resolution Closure — r371

Parent: r368 `RootCargoLockRoleAuthoritySecurityIdentityClosure` (real GREEN).

## Decision

Root `Cargo.lock` remains `LocalTransactional`: it is never incoming Take payload. The Take transaction now owns its lifecycle.

1. `TakeTransaction::prepare` backs up the existing root lock bytes, including the distinction between present and absent.
2. Incoming Take application still copies only `TakeManaged` roots; `Cargo.lock` cannot arrive from a Take.
3. After apply and before public reexec, the authenticated open `TakeTransaction` invokes one Cargo `check` without `--locked`. Cargo alone may reconcile the local lock with the newly installed manifest. The build gate runs with the active transaction identity during this step.
4. Every later public Root-Cargo build, Clippy and test path uses `--locked`. A missing or further-changing lock therefore fails closed.
5. Rollback restores the exact parent lock bytes or removes a lock that did not exist before the transaction. Commit preserves Cargo's validated transaction-local lock.

This closes the previous gap in which `Cargo.lock` was build identity and source-snapshot state but lived outside source rollback. No dependency, music, writer, repair or selection authority is changed.
