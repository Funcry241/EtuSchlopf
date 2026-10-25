# r1514 - ClefPlannerExactInputReadback

Parent: accepted r1513, SHA-256
fdb4270fb2d193b719bc188f7ffd8176cdd14d466ef42adcfad89454c1205ee8.
ChangeClass: RefactorOnly. Native r1514 acceptance is pending.

## Why this observation is needed

The r1513 export confirms 2482 tests passed, none failed, one ignored, ten-piece
corpus dark-green, and the exact accepted r1513 payload. Its complete source
snapshot matches the delivered candidate byte-for-byte.

The earlier export contained 55 distinct clef-planner input length/FNV64 bindings.
None matched any exported Master/Render XML or the ten recoverable pre-register
inputs. These final documents therefore cannot be relabelled as the transient
clef-planner inputs. This take closes that byte-observation gap only.

## Behavior

At every actual derive_clef_readability_primary_course_plan entry, before its
existing part/initial-surface guards, a read-only helper emits the complete input
to the existing stderr channel. The unchanged owner fingerprint, byte length and
SHA-256 bind zlib-compressed hex data in a schema-1 begin/end frame. All compressed
hex rows are at most 128 characters. One stderr lock covers each whole frame.
Compression uses the existing flate2 dependency; no dependency is added.

Repeated calls remain repeated observations. There is no score filter, cap,
global cache, global counter, deduplication or candidate selection. The snapshot
is at function entry, including calls that later have no eligible parts. Existing
per-part trace guards remain unchanged; an absent solution trace is not an empty
successful plan. An encoding failure emits an explicit unavailable record. stderr
I/O remains best-effort and cannot veto the original computation.

The input binding connects the snapshot to existing candidate/solution traces.
It does not distinguish identical repeated calls by identity, identify a productive
phase, prove a writer commit, or prove receipt/adoption. In particular Master,
Render, preparation and probe roles must not be inferred from log order or PDF
resemblance. Those require independently bound caller/receipt evidence.

No pitch scanner, structural scope, candidate, cost, optimizer, normalization,
materializer, receipt, adoption or existing class contract is changed. The only
planner-source addition is an unconditional observational call before its original
body. No new error classifier or counterfactual solver is installed, so the frozen
r1513 class is preserved without adding an implementation contract.

## Extraction

Use the exported console log or an individual project console log:

    python tools/assistant_diagnostics/extract_clef_inputs.py etuschlopf.console.txt clef_inputs

The destination must be new. All frames are validated before any output directory
is created. Each observation receives its own indexed MusicXML file even when its
bytes equal another observation. index.json records original line locations,
length, SHA-256, FNV64 and compressed length. No XML is parsed or reserialized.

The extractor rejects incomplete/nested/orphaned frames, unknown schema/encoding,
unexpected internal lines, unavailable captures, malformed hex, length/hash/FNV
mismatches, decompression errors, oversized decoded content and trailing data.
It reports no successful empty observation set. Complete captures cannot prove
that an entire external log itself has not been truncated outside their frames.

## Verification

The accepted r1513 outputs were compared with r1512: all 20 XMLs are byte-equal
and all 50 PDF pages pixel-equal at 144 dpi with the existing PDFium diagnostic.

Local evidence on this implementation: three new Rust tests pass (UTF-8/CRLF,
empty/long input, row bounds, repeated observations, unchanged input and existing
report rewrites), and five Python extractor tests pass, including valid streams
with wrong SHA/FNV/length or trailing data. The actual Rust producer also feeds
the strict Python extractor successfully for two identical Unicode/CRLF calls.

All 49 existing clef-planner tests pass. Their 66 pre-existing planner trace lines
form the same multiset as on r1513. From their actual new stderr frames the strict
extractor recovers 63 observations with 53 distinct SHA-256 values; every existing
eligible input trace has a verified length/FNV capture binding. This is synthetic
entry/transport and unchanged-planning evidence, not a real-corpus reexecution or
proof that any particular captured planning call committed a musical write.

Native acceptance, complete Windows launcher/strict/all-target tests, ten-piece
run and changed-output review remain required. Expected musical effect: all 20
XML files byte-identical to accepted r1513; all 50 PDF pages pixel-identical.
The console log grows by the observational frames; repeat-call count and total
capture cost need measurement in the real pipeline. No musical improvement or
new clef repair is claimed.

Apply with the existing single command:

    .\run.cmd --take --all
