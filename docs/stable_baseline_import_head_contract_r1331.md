# r1331 — Stable accepted baseline and import-head contract

Parent: accepted r1326, v0.10.1839.
Accepted archive SHA-256:
97297979570975130de4780cf8ab5db20d22d3f5c9a619c061789b636c70bfcd.
ChangeClass: KodexOnly.

This candidate contains the exact accepted r1326 Rust sources. It incorporates
none of the rejected r1327–r1330 allocator/profile changes. It adds only an
independently derived headless-import-channel error-class definition and this
report, plus normal Take revision/manifest metadata. Rust line delta: +0/-0.

This is a baseline/contract Take, not a musical repair or proof of full native
pitch coverage. The accepted baseline already passed the Windows ten-project
run. Full native tone coverage of that baseline remains unproved. Renderer
commands and force fallback are unchanged; force prohibition is deferred until
positive convergence evidence exists.

## Why a broader label search alone is insufficient

Read-only diagnostics on the four original WAV/MIDI projects reproduce the
rejected r1330 stops. Their exact frozen same-role conflict graphs yield headed
clique certificates. Every certified vertex has a directly source-bound,
participating ordinary pitched head in the complete written projection. Each
pair in a role's clique is already an incompatible canonical class. Different
canonical presentation roles cannot share a channel under the existing owner.
Hence the sum of those clique sizes is a lower bound on required distinct headed
channels, independent of greedy order, alternative coloring, particular label
values or consumer admission priority.

Measured lower bounds: Nr. 1 >=7; Nr. 6 >=9; Nr. 7 >=10; Nr. 10 >=9. The native
profile has at most eight partwide staff/voice slots. Thus the frozen r1330
obligations for Nr. 6, 7 and 10 admit no role-preserving distinct-key channel
assignment. This is a sufficient integer certificate, not a claim that the
measured lower bounds equal each graph's exact chromatic number. Nr. 1 is not
proved infeasible by this bound and needs independent head/capacity closure.

A future repair must work before these unsatisfiable obligations are frozen and
must establish the required positive canonical musical/structural authority.
Dropping an edge, merging presentation roles, fabricating a chord host, adding
import-key aliases or suppressing tones cannot be authorized by infeasibility.
The certificate authorizes no music mutation. The generic mathematical argument
comes from the existing constraint model and independent consumer capacity;
measured projects and opaque IDs provide evidence only, never selector features.

## Diagnostic correction

The initial assistant-side native reconstruction script joined a source ID to
the next occurrence's span. That auxiliary reconstruction was invalid; the
original reported large native losses and demand table must not be reused.
The productive r1330 Rust profile and its four runtime stops were unaffected.
The corrected reconstruction joins projected_note_start and source_event_id
within the same occurrence row, checks exact row coverage, and deletes previous
native output files before each import. Native corrected diagnostics retain
required-coverage defects but far fewer lost tuples than initially reported.
The separate diagnosis package contains corrected raw results and certificates.

## Next Take

Place this archive in the normal Take input and run:

    ./run.cmd --take --all

A successful run establishes the unchanged accepted executable behavior under
new revision r1331 and freezes the separate registration-error class before any
productive repair using it. It does not establish musical convergence. Preserve
the resulting accepted export as the stable baseline; later musical work must
start from that accepted parent and prove exact native represented coverage.

## Local validation of the final baseline

Accepted-parent Take admission passed: three declared and three actual metadata/
documentation changes, zero music-sensitive changes and zero writer families.
All 336 Rust files match accepted r1326 byte-for-byte (+0/-0 Rust lines).
The locked offline all-target test suite passed 1923 tests in 15 targets, zero
failures and zero warnings. Clippy correctness/suspicious/perf checks passed with
zero warnings. The optimized strict main build passed under Rust 1.98.0 Linux.
The four original projects completed successfully under the unchanged baseline
with MuseScore Studio 4.7.5 Linux, each producing Master/Render MusicXML, PDF and
result report. The initial subset batch had a harness setup failure for Nr. 1:
the isolated working directory lacked the bundled normative XSD resources.
After those exact shipped resources were added, Nr. 6/7/10 completed successfully
and the isolated Nr. 1 recheck returned exit 0, zero blockers and zero warnings.
The original subset batch itself returned exit 1 and is not reported as a clean
corpus run; its four-project manifest also correctly reports that this is not the
full expected ten-project corpus. This project smoke validation confirms local
execution of the accepted pipeline; it does not prove full tone coverage or
replace the Windows ten-project acceptance run. Native inventory defects and
the required canonical replanning remain open.
