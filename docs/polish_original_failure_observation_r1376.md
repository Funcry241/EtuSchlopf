# Original owner failure observation — r1376

Parent: accepted r1374 GestureRelocationCaptureOrderClosure, SHA256
c920be0dddf920a0e5319f59983495e4f943644b842dd1e64c37da6b1c9ae163.
Class: RefactorOnly. r1375 was rejected and is not this Take's source parent.

This is a diagnostic prerequisite for correcting the observed Polish/Layout
and parser failures. It does not claim to repair either runtime failure.
The existing original owner calls, receipts, verifier, SSC verdict and fatal
adoption remain unchanged. No catch, retry, replacement or new write is added.

The main process chains the original panic hook first, then emits a forced
backtrace and any currently recorded original owner bytes. The observational
carrier is thread-local and cannot move between threads. Lexical guards restore
prior context during ordinary return and unwinding. NotationPolish records its
original input; the existing ArrangementVariant handoff records its exact
before/after buffers. The already existing receipt-coverage failure branch also
emits the direct original Polish pair, without parsing or normalizing either.

Documents are published only on panic or the existing coverage failure, in
64-byte hexadecimal chunks with exact byte count and SHA256. Each record is
written under one stderr lock and bounded by begin/end markers. Existing child
console and failure exports carry those records. There is no new file writer,
registry mutation, proof rule, selector, writer authority or feedback consumer.
The evidence is observation only and cannot permit adoption or music mutation.
Kernel access violations do not invoke Rust panic hooks; the preceding parser
panic can now carry its original input and backtrace. An absent context remains
absent; it is never reconstructed from output geometry.

Two synthetic tests cover exact Unicode/CRLF/tab byte roundtrip, chunk boundaries
and integrity metadata, independent thread context, nesting and restoration on
original panic unwinding without replacing the payload. Corpus cases remain
runtime evaluation only. A separate local Rust 1.98.0 / roxmltree 0.20.0 optimized
probe passed 800 concurrent large synthetic parses and all last-child relations;
that excludes no input-specific or Windows/optimizer-sensitive failure.

No r1375 custody continuation is promoted into accepted state. Row 50, further
Master/Render custody, canonical pitched roles, channel allocation, unforced
PDF and dynamic resource completion remain separate open work. The normal
single-command workflow stays ./run.cmd --take --all, without Force.
