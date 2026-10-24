# SecurityCapabilityRetirement Relocation Identity Regression Closure — r65

Purpose: re-apply the r64 SecurityCapabilityRetirement relocation-identity refinement on committed GREEN r63 with the regression fixture corrected after r64 was rolled back by Strict tests.

r64's production control-plane change was not rejected by pre-take security gates. Build and Clippy passed, but one new test fixture accidentally placed each distinct `marker` statement on the same source line as its `return Err(...)`. Because `normalized_rejection_surface()` intentionally fingerprints the complete rejection-bearing source line, the two deliberately distinct functions did not share the coarse rejection fingerprint the test claimed to construct. The take transaction therefore rolled back to r63.

r65 keeps the r64 production logic unchanged and changes only that test construction: the distinct marker statements are placed on separate non-rejection lines, while the `return Err(...)` lines are identical. This establishes the intended adversarial case: equal coarse SecurityNegativeCheck rejection fingerprints, but distinct full identifier-invariant function bodies.

The retirement law remains unchanged from the intended r64 refinement:

- exact kind/path/symbol/prior-capability-fingerprint binding remains mandatory;
- retirement ledger remains append-only;
- new retirements remain SecurityOnly;
- stubs remain forbidden;
- true relocation of the same semantic function body remains fail-closed;
- unrelated negative checks that merely share a coarse rejection fingerprint do not block a future retirement.

r65 itself retires zero capabilities and changes no Assistant Export, transfer publication, persisted-result vocabulary, music, renderer, planner, writer, report or output behavior.
