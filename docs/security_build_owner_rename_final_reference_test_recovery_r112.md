# Security build-owner rename final-reference test recovery (r112)

Parent: real dark-green r110.

This recovery reapplies the closed final-reference foundation attempted by r111, but fixes the productive-write-inventory basename normalizer used by the security build-owner rename comparator. r111 searched for a literal backslash-escaped quote sequence (`include_str!(\\"...\\")`) instead of the actual Rust source form (`include_str!("...")`), so its new bootstrap regression test failed after Build and Clippy.

The correction is intentionally narrow: it changes only the matcher representation so the already intended exact `include_str!` basename substitution can be recognized. The allowed ownership FNV synchronization remains restricted to `PINNED_SECURITY_GATE_CORE_OWNERSHIP_FNV1A64`; unrelated source-integrity pins and all non-path text deltas remain rejected. The Cargo build owner is not renamed by this take. No music, writer, renderer, pipeline, report, console, recovery or calibration semantics change.
