# SecurityCapabilityRetirement Contract — r63

Purpose: introduce an explicit security-control-plane mechanism by which a protected negative/security capability can later be intentionally retired without faking its continued existence, weakening the semantic capability gate, or hiding the removal behind a rename/stub.

Baseline: the currently installed r61 payload. r63 itself changes no Assistant Export, transfer publication, persisted result vocabulary, music, renderer, planner, writer, report or result behavior relative to that baseline.

Rules:
- the retirement ledger is append-only;
- retirement is allowed only in `SecurityOnly` takes;
- schema version 1 permits only `SecurityNegativeCheck` retirement;
- every new retirement entry binds exact capability kind, source path, symbol and prior capability FNV64;
- the bound capability must exist in the current installed source and must actually disappear in the incoming candidate;
- a new entry without the corresponding removal fails closed;
- a removal without a newly-added exact retirement entry fails closed;
- changing or deleting an existing retirement entry fails closed;
- leaving a stub or weakened body under the retired function identity does not count as retirement;
- moving the same capability/fingerprint to another path does not count as retirement;
- the contract does not authorize writer creation, writer relocation, threshold changes, music mutations, proof re-entry or unrelated security changes;
- r63 introduces the contract with zero retirement entries.

The first intended consumer is a later take, after r63 has committed green, that cleans up the remaining historical Assistant-Export archive security surface through the already-installed retirement contract. That later take must not define its own authorization rule.
