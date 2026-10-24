# r1323 — Correct standard chord geometry before further implementation

Parent: accepted r1320, SHA-256
 afae1a1c2a8d0ab8ee895b972cce5d99a3934416cfc45d1a6b5c0aa46966c1ae.
ChangeClass: KodexOnly. Rust line balance: +0 / -0. All 336 Rust files remain
byte-identical to the actual restored parent exported after r1322 rejection.

## Observed result

r1322 passed Windows admission, zero-warning strict Clippy, strict build and
all 1,921 tests. The corpus failed in project 7 with
`FrozenSerializedVoicePlan HARD SAME class has incompatible written obligations`.
The other nine project processes completed. The take rolled back to r1320.
Only console, strict Clippy and strict test logs were retained; actual candidate
notation and the conflicting pair's identities were not exported. Nine completed
projects therefore do not establish musical collision counts, and the exact
historical failing pair is not yet identified.

## Confirmed general definition error

The accepted definition incorrectly required equal durations for a legal chord.
The W3C MusicXML 4.0 specification explicitly permits shorter chord members while
forbidding a chord member longer than its preceding head. The existing canonical
owner already retains shorter members and can assign HARD SAME to that exact
physical chord. r1322's equal-end predicate incorrectly classifies that standard
legal family as contradictory. This is a general source/specification defect;
no assumption is made that it identifies the exact pair in the project-7 abort.

Source: https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/chord/

The corrected definition requires exact positive pitched-head/canonical chord
evidence and member intervals contained in that head's written extent. Equal
onset or equal duration alone never authorize a chord. Unknown/rest/orphan hosts
remain outside positive authorization; independent overlaps remain conflicts.

## Native validation

A synthetic production-path probe against r1322 used a positive pitched chord
head with duration 8 and a canonical shorter member with duration 4 at the same
onset. The existing factory supplied the canonical HARD SAME edge. The production
serialization-plan builder rejected it with the same generic HARD SAME message.
This reproduces the general false-positive family, without identifying the
historical project-7 pair. The probe is analysis-only and is not included here.

Native Linux `cargo check --locked --all-targets` completed successfully for
r1323 with the installed Rust 1.98.0 toolchain. This is not a Windows strict or
original-input corpus run. No Rust test or production code changes are included.

## Why this is a definition-only take

The accepted parent-class contract must already exist unchanged before an
implementation can consume it. The live Take-Change-Control implementation rejects
a productive selector candidate that modifies its own parent class in the same
take. r1323 therefore corrects the normative class on accepted r1320 while keeping
all productive Rust, gates, receipts and SSC-pinned orchestration byte-identical.
This is required by the actual parent-contract rule, not a proposed new approval
flow or permission request. This candidate grants no acceptance bypass and does
not relax any executable gate.

After acceptance, the subsequent musical implementation can consume this corrected
class, bind the actual pitched head interval, test shorter and equal chord members,
reject oversized/noncanonical members, and retain detailed evidence for any genuine
HARD SAME contradiction. It must preserve the universal channel check and main-role
labels. A duration-ratio, named project or known measure exception is excluded.

## Boundaries and next command

This candidate changes only documentation and take metadata; it cannot fix musical
collisions. Standard-based class correction is independent of corpus witness
values. Windows corpus convergence and actual unforced MuseScore import remain
unverified. The conditional removal and gate prohibition of MuseScore -f remain
pending until the musical candidate completes its authorized full run.

Put only r1323 into the normal Take input and run:

    ./run.cmd --take --all
