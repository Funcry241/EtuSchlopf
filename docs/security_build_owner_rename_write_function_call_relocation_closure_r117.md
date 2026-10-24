# r117 — SecurityBuildOwnerRenameWriteFunctionCallRelocationClosure

Parent: committed dark-green `0.10.1839 r115`.

The rejected r116 pure build-owner rename exposed one remaining semantic-capability false positive.
Because the Cargo build script is moved byte-identically from `src/identity_debt_build.rs` to
`src/security_build_orchestrator.rs`, the semantic inventory reports the unchanged writer-reachable
edge `main#1 -> run` as one removed `WriteFunctionCall` at the old path and one added
`WriteFunctionCall` at the new path. SecurityOnly correctly rejected that pair because the closed
build-owner rename contract previously authorized only the corresponding `SecurityNegativeCheck`
relocation identities.

r117 extends only that already-closed `security_build_owner_rename` contract. Removed/added
`WriteFunctionCall` records are accepted only when every such record belongs respectively to the
old/new Cargo build-owner path, the symbol sets are bijective, and domain, fingerprint and detail
are identical for each old/new symbol pair. The resulting authorization contains only those exact
record keys. `WriterReachability`, modified writer calls, writer calls in other files, and unrelated
SecurityOnly writer capability changes remain fail-closed.

A regression fixture now makes the byte-identical rename include a writer-reachable
`main -> run -> write_musicxml` path, reproducing the r116 failure class inside the semantic gate.

No build-owner rename occurs in r117. No music, writer implementation, renderer, pipeline, report,
console, recovery or calibration semantics change.
