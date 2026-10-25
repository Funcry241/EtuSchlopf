# r1510 - RegisterPlannerInputReadback

Parent: accepted r1509, SHA-256
`e16ca7354500e6076ea9b701ec2d6f51ef7cb7db297756857b9bc745e92bf783`.
ChangeClass: RefactorOnly. Native acceptance of this candidate is pending.

## Reason for the observation

The missing primary-beam fields in the parsed Score do not establish that their
absence causes the octave fragmentation. A replay of the exported E4 endpoint
through the unchanged parent planner separates the high tail in bar 7 into a
two-object segment. The existing four-object eligibility rule then refuses an
octave span there. Segment boundaries and window costs are a competing cause.
This is an endpoint replay, not proof of the actual pre-K00 planning input.
No beam-owner invocation or canonical gesture binding was established by it.

Before adding a new transport path or changing musical policy, capture the
actual bytes supplied to the existing planner. This take supplies that evidence.
It does not yet improve the printed octave spans.

## Productive path

The existing `registernotation::beobachte_vor_k00` copies its immutable string
input before the existing parse call. The capture lives in the same resettable
session as its planning result; the report also includes that result's plan hash
when available. Neither the parser, planner, costs, thresholds, segment boundaries,
materialization request, writer nor adoption path changes.

The current final-report projection owner appends the declared section
`REGISTER-PLANNER-INPUT-READBACK`. The retired full report is not the export path.
The section records byte count, SHA-256 and all original UTF-8 bytes as bounded
hex rows. Hex protects titles, line endings and XML content from report vocabulary
rewrites. Session cloning shares immutable bytes through Arc; no second parse or
extra pipeline planning call is introduced. Text reports grow by approximately
2.2 times the captured XML size; compressed exports should grow substantially less.

Parse failure still retains the observed input, without an old successful plan
hash. A session reset removes the capture. Absence is reported as NOT_OBSERVED.
Canonical gesture ownership is NOT_EVALUATED; the capture confers no musical
authority and is not a positive witness for the r1509 error class.

## Readback and next decision

The assistant can recover an input from the next result export with:

```text
python tools/assistant_diagnostics/extract_register_input.py INPUT.result.txt NEW.musicxml
```

The helper checks unique capture boundaries, complete hex rows, byte count and
SHA-256 before creating the new file. Existing output files are never overwritten.
The recovered XML is diagnostic input, not a replacement public score.

Next: replay this actual input through the same planner, compare its reported
plan hash, then distinguish segmentation/eligibility effects from display costs
and independently owned rhythmic context. Only then select the smallest musical
change with a matching accepted causal definition. No fixed bar identity, blanket
15ma ban or new ledger-line threshold is introduced here.

The accompanying two-sentence addition under essence E01 asks for a serious
alternative cause and a smallest falsifying experiment, specifically before
adding a transport path. It remains advisory.

## Local validation

The complete real binary test target compiled on Linux with Rust 1.98, without
reported warnings, using the unchanged installed build gate. The actual binary
passed all 61 register tests and all 3 new input-report tests; the Python helper
passed 3 tests. An isolated real-module probe additionally checked the current
report projection. A 381,030-byte endpoint XML survived capture, report rewriting
and verified extraction byte-for-byte. This roundtrip is not a pre-K00 corpus run.
The exact parent incoming validator and dead-code integrity check pass. Windows
native/corpus/clippy acceptance is still pending.

## Acceptance

Apply in the accepted r1509 installation with `.\run.cmd --take --all`. No force switch. The unchanged parent validates the
declared tree and dead-code integrity before native installation.

Expect unchanged master/render XML and unchanged printed musical content across
the full corpus. Result reports gain the input section. The fresh Windows run
must establish native tests, build, clippy, corpus and public-artifact completion;
the parent's green results are not new r1510 results. Send the new assistant
export so the real input, its byte hash and plan replay can be examined.
