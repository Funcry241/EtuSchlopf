# r1690 — Preexisting test retirement admission and corrected Cargo activity

Parent: accepted r1688, SHA-256 c5d263bb41202f7fecd72dd59e0f5a16f3db859a1cd487b13510c577e13d9853.
Change class: KodexOnly. No test retirements or productive music changes in this take.

## Native failure correction

r1689 was rejected on Windows at reexec_exit (child 101) and rolled back to r1688.
The executed candidate and restored parent snapshots match their frozen source trees.
The shared-log test called set_len on an append-only file handle. Windows rejected
that test operation with access denied. The fixture now drops its append handle and
truncates through File::create. All five Cargo activity tests remain active.

This take reapplies the r1689 read-only Cargo phase and original log-age display.
Only the existing interactive launcher progress line changes. Exit handling,
subprocess arguments, environment and redirected output behavior remain unchanged.
The initial run.cmd compilation and separate silent Watch build are outside scope.

## Owner correction

The retirement validator previously rejected a test deletion whenever any incoming
negative-check body matched, including an unchanged counterpart already in the parent.
That confuses a preexisting duplicate with a new relocation.

The validator now distinguishes that case only under a deletion-only test proof:

- The counterpart has the same parent-bound capability key and fingerprint.
- No function is added; every retained function has identical unmasked signature,
  body, domain, visibility and calls.
- Managed file sets, including documentation fixtures, are identical. All managed non-Rust fixtures remain byte-identical
  except the append-only retirement ledger and declared change manifest.
- Changed Rust files must equal the original source after removal of complete,
  directly attributed #[test] functions. Optional obsolete leading //! documentation
  can be deleted; new comments or source text cannot be introduced.
- Root configuration, README, build script and command controls remain identical.
- The trust anchor permits only changed 16-hex-digit values inside otherwise identical
  exact FNV assertion lines; assertions, names and other source bytes remain intact.
  The normal compile-time assertions and parent admission checks still apply.

Existing append-only ledger, exact fingerprint, actual removal and no-stub requirements
are unchanged. New copies, relocations, renames, changed test literals, helpers,
constants, imports, attributes, fixtures and configuration still reject the exception.
The proof does not establish that two tests express the same obligation: the audit
must justify each proposed retirement separately. It only fixes relocation detection.

Three new unit tests exercise the admission matrix (18 variants), LF/CRLF deletion
boundaries and exact pin assertion syntax. Existing relocation, distinct-body,
fingerprint and append-only tests remain active. No thresholds are relaxed.

## Next take

Only after native acceptance of this foundation: remove the audited nine duplicate
tests and one historical vocabulary guard. Their exact original draft is preserved
in EtuSchlopf_Testbereinigung_Audit.md; no deletion is bundled into this foundation.
