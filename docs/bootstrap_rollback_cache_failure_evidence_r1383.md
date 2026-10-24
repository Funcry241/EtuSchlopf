# Bootstrap rollback cache evidence — r1383

Parent: accepted r1380. KodexOnly. No music changes; rejected r1381/r1382 layout
changes are not included. A missing or genuinely empty dedicated bootstrap cache
contains no package artifacts to invalidate. Directory reads and hidden entries
remain fail-closed; nonempty caches retain the exact package-scoped Cargo clean.
Source restore and byte verification still precede cleanup; failed nonempty clean
still prevents rolled_back and preserves pending recovery provenance.
Cargo stdout/stderr are captured separately as original bytes under
`target/etuschlopf_logs/bootstrap_rollback_cache.stdout.log` and `.stderr.log`.
On failure, command identity, exit code and decoded output enter the bootstrap
error log. Logging errors cannot change the actual Cargo success/failure result.
This closes the empty-directory case and missing diagnostics; it does not claim
to repair every possible nonempty-cache failure. The original Windows Cargo
stderr for the earlier Exit 101 is unavailable. r1382's cleanup succeeded.
An installing candidate cannot replace its own parent-owned rollback behavior;
this protection becomes active as a parent only after acceptance.
Synthetic tests check absent/empty/hidden-entry handling and an actual subprocess
cleanup refusal with exact stdout/stderr preservation. Existing source-restore,
package-clean and restart tests remain intact. Acceptance requires a fresh green
Windows corpus and unchanged music relative to r1380.
