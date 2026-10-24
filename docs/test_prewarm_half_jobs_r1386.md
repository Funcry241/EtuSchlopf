# Gentler strict test prewarm — r1388

Parent: accepted r1385 (r1386 and r1387 with identical content were rejected only by a build-script access violation and by an interrupted transaction, before any gate verdict on the content). Class: KodexOnly. No musical, gate, receipt or
validation change.

Since the parallel strict test prewarm (documented in r1306), a normal run
compiles the all-target test binaries in `target/bc` while Clippy and the
productive build compile in `target/strict`. Both Cargo invocations used the
full job count (logical threads minus one), so up to about twice as many
optimizing compilers as hardware threads ran at the same time. This is the peak
memory phase of every take.

The Windows Application log shows the first compiler access violation on the
evening the prewarm appeared and repeated, non-reproducible access violations at
varying offsets in rustc, etuschlopf and the take bootstrap afterwards. This take
does not claim that the prewarm causes them; it reduces the peak load while the
hardware is examined.

Change: the prewarm alone uses half the Cargo jobs (at least one). The later
authoritative test step, Clippy and the productive build keep the full job count.
The prewarm still only warms a derived cache; the test step decides alone.

Expected effect: lower peak memory during build; possibly slightly longer
prewarm residual wait. Outputs byte-identical to r1385.
