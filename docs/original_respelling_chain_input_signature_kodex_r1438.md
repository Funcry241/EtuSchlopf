# Closed original-chain input migration for population owner 60

Parent: accepted r1437, version 0.10.1839, Take SHA-256
`e1cbf766310920914bfd1a97604989b5cc250702bbf7d276edbe00cd7ba715af`.
Change class: KodexOnly. No productive owner, callback, runtime data type or
musical output is changed by this take.

## Reason

The accepted error class in
`selector_derivation_contract_partial_original_tie_chain_respelling.md` requires
original connectivity. The existing #60 callback receives two strings and no
original chain carrier. Reconstructing lost source/course membership from output
pitch, onset, staff or voice is not an implementation of that contract. The
current return-only migration explicitly rejects additional parameters.

The independent need is to transport already bound original evidence to the
existing owner, rather than relax the complete-chain check or invent provenance.
This take admits one explicit typed input for that owner only. It is not the
music repair and does not authorize a generic context parameter.

## Exact closed signature

Only the existing EXACT/M owner
`src/pipeline_shards/key_signature_enharmonic_report_core.rs::canonical_master_functional_respelling_from_render`
may migrate from its original frozen signature to:

```rust
fn canonical_master_functional_respelling_from_render(
    master_xml: &str,
    render_xml: &str,
    original_chains: &crate::master_state_identity::OriginalFunctionalRespellingChains,
) -> (Result<String, String>, Option<ExactTransformationReceipt>)
```

The gate matches the exact owner, original frozen signature, EXACT identity
kind and complete migrated signature. A mutable reference, another type or
parameter name, added parameter, generic, changed return, extra receipt,
another owner or changed original signature remains refused. The generic
return-only migration itself is unchanged and still refuses this input.

The current original function and its already admitted receipt-return form
remain valid. No function is migrated at runtime in this take.

## Physical surface and trust boundary

The signature migration is recognized both at owner-signature validation and
at normalization to the existing physical-surface baseline. Instance count,
buffer mutation count and file-write count retain their original bounds.
No population/exclusion row, effect/domain classification, nested-owner pair,
factory binding expression, execution order or budget changes. The trust anchor
is synchronized to the final gate bytes explicitly; unrelated pins are unchanged.

## Required later runtime proof

`OriginalFunctionalRespellingChains` is deliberately not defined here. A later
implementation must use a sealed, immutable carrier bound to the actual original
execution, its input/output snapshots and complete original source membership.
It must carry the original #65 binding/emission relation, including encoded
fragments, through any preceding original #59 exact operation before #60 uses
it. Missing, stale, foreign, incomplete, ambiguous or conflicting original
evidence must refuse the affected transaction, without fallback reconstruction.

The carrier may neither create source/course identity nor select a replacement
spelling. Existing tonal evidence and original-owner authority remain separate
positive prerequisites. Original receipts remain in execution order; no hidden
thread-local or string-encoded evidence channel is admitted. The complete-chain
check, SSC/adoption, strict disposition rules and terminal-gate boundary remain
unchanged. Runtime tests must cover renamed and relocated independent copies,
internal ties, chord/hidden occurrences and conflicting or absent evidence.

The existing #59 carrier construction and single original #60 invocation remain
the route. Any later callback/carrier implementation still has to pass the
accepted parent's semantic, reachability, population and signature gates.
This signature admission alone does not prove that runtime implementation.

## Tests and limits

Synthetic tests exercise the exact signature admission and adversarial forms,
then both actual owner and physical-surface checks, including refused increases
of buffer writes, file writes and instance count. The previous generic migration
continues to reject parameter drift. No corpus case defines these tests.

No #60 correction, E007 continuation, Render custody, terminal discharge,
musical release or playability claim is supplied by this take.
