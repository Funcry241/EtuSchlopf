# Security Build Orchestrator Closure Parent-Primitive Recovery — r106

Parent: committed dark-green r103.

r104 and r105 were rejected before source copy by the existing
SecurityNegativeCheck parent-module relocation contract. The failures exposed an
important boundary condition of the final orchestrator split: four helper
functions are themselves classified as executable SecurityNegativeCheck
capabilities. Moving them into child modules would require artificial qualified
parent calls solely to satisfy relocation reachability, even though their
natural callers live inside the child security owners.

r106 therefore keeps these four executable gate primitives in
`src/identity_debt_build.rs` with their exact r103 bodies and rejection surfaces:

- `take_bootstrap_compile_isolated`
- `parse_feedback_observable_sink_allowlist`
- `parse_security_boundary_adversarial_matrix`
- `read_baseline`

The child owners consume these parent primitives directly. They are not copied,
wrapped, shadowed, retired or weakened. Because these four capabilities no
longer relocate, the module-relocation contract does not need synthetic parent
execution anchors for them.

All other closure work remains in place: path/read/rerun context support lives
in `security_build_context`, feedback implementation support remains in
`feedback_structuralization_security`, adversarial implementation support
remains in `security_boundary_adversarial_security`, inventory implementation
support remains in `inventory_security`, and the public observable feedback sink
register remains reduced to the exact four real sink paths.

This deliberately leaves `identity_debt_build.rs` slightly larger than the
rejected r104/r105 candidate. The target is architectural truth, not minimum
line count: the file is now a compact fail-closed orchestrator plus four
security primitives whose parent ownership is itself part of the enforced
capability contract.

No music, writer, renderer, pipeline, threshold, report, console, calibration
or recovery semantics change.
