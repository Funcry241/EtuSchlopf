# Productive Write Inventory Security Owner Test Decoupling – r102

This RefactorOnly take changes test ownership only.

The regression test `post_cutover_build_gate_requires_zero_external_musicxml_surfaces` no longer assumes that the zero-external-MusicXML security contract must physically live in `src/identity_debt_build.rs`.

Instead, it scans the compile-time build-gate source domain (`src/identity_debt_build.rs` plus `src/build_gate/*.rs`) and requires the two canonical contract markers to have exactly one source owner.

This preserves the security assertion while allowing a later SecurityOnly take to relocate that already-existing contract to its dedicated inventory-security owner. No runtime, music, writer, renderer, threshold, report or security decision changes in r102.
