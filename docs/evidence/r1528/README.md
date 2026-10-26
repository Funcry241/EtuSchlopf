# r1528 report verdict projection candidate

Based on natively accepted r1527 (take SHA-256 4e16dca358b1e28ae3c78bdae6a96343f85725ea126f58ab49313bd41309a815).
Native r1527: 2546 tests passed, 0 failed, 1 ignored; build/clippy passed; corpus 10/10 exit 0; 20 XMLs byte equal and 50 PDF pages pixel equal at 96 dpi to r1526.

r1528 adds four exact internal verdict vocabulary mappings in the existing report-only sanitizer. Rejection outcomes, musical decisions, XML, writer/recovery owners and the legacy-authority guard are unchanged. Unknown review authority still fails. Three synthetic unit tests cover the verdict family, unknown authority rejection and unchanged adopted musical facts. The packaged smoke test now requires real exit 0 and nonempty XML and report artifacts; the former expected-denial allowance is removed.

Native r1528 acceptance and corpus/PDF comparison are pending. This branch is a candidate build, not an accepted native take.
