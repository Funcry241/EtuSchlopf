# r1326 — Freeze the partwide renderer channel-capacity error class

Parent: accepted r1324, SHA-256
8a54b35d877bbd49554ed7db8e3f330376d7704c74a1e814d72efe0f438152c6.
ChangeClass: KodexOnly. r1325 was rejected and restored this parent.

This candidate defines the new causal class before introducing productive repair
logic, as required by the unchanged accepted Take selector control. It changes
no Rust, no renderer command and no musical output. The accepted pitched-pair
class does not authorize rest-channel repair or establish consumer capacity.

## Exact failure evidence

The uploaded renderer shadow contains the actual failed primary-beam voice-partition
inputs. Their byte identities agree with the Windows freshness evidence:

| Diagnostic input | Bytes | FNV-1a64 |
| --- | ---: | --- |
| Etude 6 constraint candidate | 720288 | d77e41e3ec12ea60 |
| Etude 7 constraint candidate | 487696 | d812ba5dad5c2216 |

Both failures reproduce locally with official MuseScore Studio 4.7.5 Linux x86_64,
an Xvfb TCP display and ordinary `-o` export. No force flag is used. The Windows
run used MuseScore Studio 4.7.4. Linux process exit 40 corresponds to converter
code 1320 modulo 256. Detailed application logs identify load error 2009:

| Diagnostic input | Imported measure / staff | Found | Expected |
| --- | --- | --- | --- |
| Etude 6 | 19 / 2 | 18/16 | 4/4 |
| Etude 6 | 22 / 2 | 21/16 | 4/4 |
| Etude 6 | 27 / 2 | 18/16 | 4/4 |
| Etude 6 | 28 / 2 | 24/16 | 4/4 |
| Etude 7 | 6 / 2 | 17/16 | 4/4 |

These identities and locations are diagnostic evidence only; they are not class
features or future productive selectors. All ten primary-beam shadow XMLs pass
MuseScore's bundled MusicXML XSD. Written type/dot/duration checks pass. The
written lower primary sequence in each listed measure sums to the expected
measure duration. Removing directions, beams, notations, ties, stems and print
elements does not remove the failures. One failed measure contains no rest.

Debug imports of the exact failing inputs report `too many voices` and failed
staff/voice mappings. The importer has partwide bounded reservations. At an
unavailable channel, its `note` function returns before zeroing a chord member's
cursor contribution. The surrounding measure loop then advances that duration.
The lower measure overfill can therefore originate in rejected upper chord
members, rather than in an oversized lower note or a whole-measure rest.

## Independent causal counterexample

A synthetic two-staff, two-measure score uses division 24 and 4/4. In measure A,
an upper channel has a quarter-note C/E chord and a dotted-half rest; a lower
channel has a whole-note G. A separate lower quarter rest uses the upper channel
label and overlaps the upper head. In measure B, four other upper labels have
successive quarter-note D, F, A, B heads, one at each beat; the lower channel again
has a whole-note G. There are eight pitched occurrences in total. Cursor runs
use explicit backups and forwards. No historical score data is used.

The other upper labels reserve all four ordinary upper slots. The overlapping
cross-staff channel receives no upper slot. Its rejected chord member incorrectly
advances the import cursor, shifting the lower whole note by a quarter. MuseScore
reports lower staff measure A as 5/4 instead of 4/4 and refuses unforced export.

| Synthetic counterfactual | Unforced result | Pitched readback |
| --- | --- | --- |
| Original complete part | Load error 2009, converter 1320 | No output |
| Reuse one compatible upper channel for the four successive heads | Exit 0 | All 8 exact pitch/onset/duration/staff occurrences retained |
| Replace the cross-staff rest by an equal-duration forward | Exit 0 | Only 7 of 8 retained; the last upper note is lost |

This separates a complete repair witness from a merely successful conversion.
Numeric label renaming `v -> 17 + 5*v`, staff permutation, measure-name changes
and division/duration scaling from 24 to 37 reproduce all three outcomes. The
compacted witness still preserves all eight exact pitched occurrences. These are
diagnostic experiments, not productive mutation authority.

## Rejected diagnostic shortcuts

All-rest channel separation, partwide staff-disjoint labels and primary-rest staff
normalization each make both historical inputs import without force. They do not
prove musical completeness. Native MusicXML readback exposes substantial loss:

| Diagnostic variant | Source pitched inventory | Missing exact pitch/onset/duration occurrences | Added exact occurrences |
| --- | ---: | ---: | ---: |
| Etude 6, staff-disjoint labels | 1176 | 188 | 4 |
| Etude 6, primary-rest normalization | 1176 | 183 | 0 |
| Etude 6, rest-label separation | 1176 | 186 | 0 |
| Etude 7, staff-disjoint labels | 1021 | 84 | 1 |
| Etude 7, primary-rest normalization | 1021 | 83 | 1 |
| Etude 7, rest-label separation | 1021 | 90 | 1 |

These differences measure exact written MIDI pitch/onset/duration inventories,
not total sounding attacks, subjective audibility or parent PDF loss. Shorter
chord members and import normalization can contribute to a readback difference;
therefore these totals must not all be attributed to channel dropping. Independent
debug mapping failures and the exact synthetic counterexample establish actual
loss. No experimental historical XML rewrite is included in this Take.

## Next implementation requirements

After acceptance, derive admissible partwide channels from complete canonical
written, role, rest and cursor evidence within the existing owners. Preserve
HARD SAME and positive pitched chord-host bounds. Reuse must be universal, not
just compatible with one prior member. An unforced conversion must be followed
by exact material continuity validation; successful PDF production is insufficient.
The current rest and host evidence may need additional closure before any repair.

The r1325 runtime force prohibition remains rejected and is not introduced here.
Its eventual replacement is conditional on complete musical convergence and
Windows corpus acceptance. Local MuseScore 4.7.5 probes do not replace Windows
4.7.4 validation or an original-input pipeline run.

## Candidate validation

The exact restored r1324 source in the r1325 failure export is the baseline;
it matches the locally retained r1324 source byte for byte. All 336 Rust files,
Cargo metadata, pinned orchestrator, receipt authority and existing class
contracts remain byte-identical. The incoming archive excludes only the local
root Cargo.lock and retains nested tool locks.

Native admission calls the unchanged parent implementation of
`take_change_control::validate_incoming_take` against the complete candidate and
parent trees. It passes as KodexOnly with three declared/actual metadata changes,
zero semantic function changes and zero writer-family changes. No new Rust tests
are added and the unchanged parent test suite is not rerun for this documentation
candidate. Windows Take acceptance remains pending.

Put this definition candidate into the normal Take input and run:

    ./run.cmd --take --all
