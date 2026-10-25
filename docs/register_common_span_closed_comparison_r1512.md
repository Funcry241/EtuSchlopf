# r1512 - RegisterCommonSpanClosedComparison

Parent: accepted r1511, take SHA-256
`6aaf1fd728c6000a41f51b085628c1127793b7584fc3bd36213fb8dafe81965b`.
ChangeClass: MusicMutation. Windows/native full-corpus acceptance is pending.
The accepted common-span error-class definition remains byte-identical.

## Behavior and ownership

Pitch-only search fences can prevent the existing register planner from comparing
a better common octave span. Simply removing them loses valid separate high
islands because the existing optimizer chooses at most one window per segment.
This take retains the complete original plan as an immutable baseline and compares
one common alternative inside each independently closed component.

The existing window search now accepts required baseline event coverage. Ordinary
baseline calls use an empty set and reproduce the parent. Common-window calls
require all previously octave-covered events before optimizing; final projected
coverage is checked again. The cost and tie-break order are unchanged. Every
visible note object in the full context contributes, including loco margins, with
the existing semantic-span penalties. Equal or worse cost preserves the baseline.
All other temporal fences, the four-object minimum, existing improvement threshold,
display anchors, tie safety and topology constraints remain in force.

The lower-domain evaluator can choose bass/treble alternatives for separate
windows. Its actual clef-owner context is absent from this input model, so this
take preserves those plans and reports unresolved authority. No clef or beam
policy is inferred or transported from later output.

The new choice remains a RegisterNotationPlan and uses the unchanged visibility
projection, StructuralWriteAuthority BLOCK #50 owner, representation transport,
receipts and sovereign Master adoption. Only tests are added near the pipeline;
that productive writer/adoption code is unchanged. No alternate XML write path,
force switch, threshold adjustment or new repair authority is introduced.

## Measurements on actual captured inputs

All ten captured input hashes match the latest accepted r1511 export. The new
baseline plan hashes reproduce all ten native parent hashes exactly.

| Piece | Planner replacements |
| --- | ---: |
| 1 | 0 |
| 2 | 1 |
| 3 | 2 |
| 4 | 3 |
| 5 | 3 |
| 6 | 1 |
| 7 | 0 |
| 8 | 0 |
| 9 | 2 |
| 10 | 4 |

There are 234 local comparison records and 16 selected alternatives in seven
pieces. These are logical planner results, not committed writer/adoption counts.
Debug builds replayed each complete piece in less than half a second locally;
this does not replace a full-pipeline timing measurement.

In E4, the new semantic plan includes one 8va from the start of bar 5 through the
end of bar 7. The previously isolated high tail is covered; the separate bar-8
fences remain. Evaluation addresses do not occur as productive selectors.

A separate read-only diatonic calculation on the actual input spellings and
existing treble clef found 90 visible note objects with changed mappings. Their
summed ledger burden fell from 88 to 34; no changed object newly exceeded three
ledger lines. In E4, the maximum among changed objects fell from five to three.
These are pre-K00 plan calculations, not PDF geometry or new selection authority.
The calculation counts visible objects, which can include coincident heads.

## Verification and acceptance

The register suite includes synthetic coverage of separated islands, constrained
window search, positive/negative independent duplication, time/label relocation,
real register excursions, low margins, necessary 15ma, long rests, incoming and
following-measure tie continuations, overlap, hidden chord carriers, mixed
durations, the minimum size, unsupported clef context and open dependencies.
The actual existing parser/planner suite plus these tests passed 73 cases locally.

An additional native binary integration test exercises the actual octave writer,
its original BLOCK receipts, complete receipt coverage, sovereign Master adoption
and cloned committed readback, starting from explicitly synthetic test genesis.
This is not an arrangement-genesis or full-corpus integration claim. Tests live
under tests/unit and execute through their real owning modules.

Apply in the accepted r1511 installation with `.\run.cmd --take --all`.
Fresh native build, clippy, all tests, ten-piece corpus, writer/receipt/adoption
readback and PDF output remain required. Compare all changed pages after export.
Pieces 1, 7 and 8 should preserve their XML; all lower-domain register plans are
preserved. Other downstream behavior must be checked on the real outputs rather
than inferred from the planner. Known unrelated lower-clef and gesture-context
questions are not declared solved.
