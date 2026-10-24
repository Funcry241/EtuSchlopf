# Security writer relocation admission

This gate closes a false-positive class in the semantic capability diff. Moving security-owned orchestration functions between launcher-owned modules previously appeared as new writer authority because capability identity included file path and function symbol.

The optional manifest field is:

```text
security_writer_relocation=closed_internal_graph_v1
```

It is valid only for `SecurityOnly`. The incoming change must keep all of the following invariant:

- every changed Rust or command source remains in the Security domain;
- writer-family deltas are only added or removed internal `WriteFunctionCall` and `WriterReachability` nodes;
- no writer-family capability is modified in place;
- writer sink roots are identical;
- direct writer authorities are identical;
- externally exported writer capabilities are identical;
- both removed and added internal graph nodes exist, proving an actual relocation rather than one-sided expansion.

Restricted `pub(super)` functions are module-internal and are not treated as exported writer surfaces. `pub` and `pub(crate)` remain exported for this proof.
