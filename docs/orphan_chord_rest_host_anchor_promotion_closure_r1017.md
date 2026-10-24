# r1017 — OrphanChordRestHostAnchorPromotionClosure

Parent: r1016 `StrictStaffOrphanChordGapNoWriteClosure`.

## Accepted evidence

The accepted r1016 full-corpus run is 10/10 dark-green and leaves exactly one incomplete visible rhythmic occupancy group: E4 T30, staff 2. The final render contains a hidden rest host with duration 24 followed by two visible pitched `<chord/>` members of the same voice/staff/duration and no visible pitched anchor. r1016 removed the downstream StrictStaff overlay without changing a PDF pixel, so the remaining defect is the upstream orphan-chord representation itself.

## First r1017 candidate rejection

The first r1017 payload (SHA-256 `e8be2c40b219159d92fec59fc0c3a744e50383c151da136e4f3a392df1d6b8f2`) was rejected before installation by `Semantic-Capability-Diff`. The candidate introduced two new direct writer roots into derived Audit WriterReachability: the new helper `stage_final_orphan_chord_rest_host_anchor_promotion_candidate#1` contained `String::replace_range`, and `ChordGestureDistributionCapability::issue#2` also became a direct writer root because its validator constructed an expected note via `replace_range`. The gate was correct; r1016 remained the parent.


## r02 runtime-test rejection and r03 correction

The corrected r02 payload passed PRE-TAKE-DIFF-GATE, Clippy and Build, then failed one `--bin etuschlopf` unit test. The failure was in the newly added self-test fixture, not in the productive selector: the fixture constructed its hidden rest host with `print-object="no"` only, while the production predicate deliberately requires the exact nonspacing carrier pair `print-object="no"` plus `print-spacing="no"`. Therefore the fixture produced zero anchor-promotion candidates and its own `candidate_accepted` assertion failed. r03 changes only that fixture to model the already-required production precondition. Productive r1017 semantics are byte-for-byte unchanged from r02 apart from this test literal.


## r03 runtime-test rejection and r04 correction

r03 passed PRE-TAKE-DIFF-GATE, Clippy and Build but still failed the same `--bin etuschlopf` target. Static reconstruction of the corrected fixture shows the selector now does find the promotion, but the stage then rejects its own authorized candidate because `FinalNotationReadbackObservation::pitched_payload_signature` deliberately includes the final `is_chord` bit. Therefore the intended `F#4: chord-member -> anchor` transition makes the legacy byte-exact payload equality false even though pitch MIDI, duration, voice, staff, hidden state and onset are unchanged.

r04 keeps selector, writer authority and FinalChord continuation semantics unchanged. Inside the already-existing stage it accepts a promoted payload only when (a) the chord-insensitive pitched payload multiset is identical before/after and (b) the number of pitched chord-members falls by exactly `anchor_promotions_planned`. Existing exact payload equality remains the first accepted case. This is a readback proof correction, not a wider mutation authority.

## Corrected r02 decision

The corrected candidate is rebuilt directly from r1016 and keeps the same r1017 music decision without either new writer root:

- the selector is folded into the already-existing `stage_final_orphan_chord_host_rest_candidate`; no new function symbol is added;
- promoted note text is derived immutably from source slices and `format!`, not by a local mutation primitive;
- `ChordGestureDistributionCapability::issue` validates the exact host/member pair immutably and delegates the byte write to its already-existing `consume -> apply_bound_edits` writer;
- the existing FinalChord orphan-host continuation proof accepts exactly one same-onset `<chord/>` member -> ordinary pitched-anchor promotion while keeping pitch, duration, staff, voice and ties fixed.

## Bounded transformation

For an immediate pair consisting of a hidden non-pitched rest host and a visible pitched chord member with identical positive duration, voice and staff:

1. remove the hidden rest host;
2. remove only the first member's `<chord/>` marker;
3. leave all remaining chord members unchanged.

No pitch is invented, deleted or moved. Raw cursor duration, sounding payload and lane identity remain stable. There is no project, measure, pitch, voice or staff selector. Existing forward-to-hidden-rest orphan-host normalization remains supported in the same stage.

## Expected E4/T30 witness

Around S2/V2 `[288,312)`:

- hidden rest host: `1 -> 0`;
- first F#4 member: `<chord/> -> pitched anchor`;
- D#5 remains `<chord/>`;
- pitched cardinality and payload remain unchanged;
- `S0IncompleteAtomicGroups`: `1 -> 0` expected;
- `BlockedIncompleteChordBindings`: `1 -> 0` expected;
- Master MusicXML remains unchanged; the change is final Render/PDF source only;
- the PDF must now show a real visual delta at the target location.

The take is not closed until the normal single-project gate confirms Semantic-Capability-Diff, build, Clippy, integrated tests, readback and the visible PDF delta.
