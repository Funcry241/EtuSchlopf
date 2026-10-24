# r1317 AtomicReceiptDiagnostics

Parent: accepted r1316, Version 0.10.1839,
SHA-256 8fca7661a3b256dd8a5574cba302f348412c8ae21aa400ce833fb1bf9a06e3d2.
ChangeClass: RefactorOnly. Parent error-class contract remains byte-identical.

This is a new implementation candidate after rejected r1315, not a replay of its
package. The two diagnostic points are now bound by exactly one implementation
contract to the causal definition accepted in r1316.

## Output

- `ExactReceiptRefusal`: owner, population row, verifier refusal reason,
  pre/post fingerprints and document byte lengths, edit count and up to eight
  origin-bound spans as (start, end, replacement byte length).
- `AtomicPatchReceiptCoverage`: iteration and patch label as observation locators,
  receipt count, base/receipt-end/proposed-document fingerprints, byte lengths,
  first discontinuous receipt index and last supplied owner.

Output contains no document text. Stderr writes are best effort. The first marker
is emitted on an already refused changed Exact-owner transition, including owners
outside Master adoption; it alone proves neither project nor release failure.
The second is emitted only for mechanical continuity/coverage mismatch at the
existing SafeAtomic call. Their values do not influence candidate evaluation,
selection, receipts, permissions, writing or adoption.

`MasterLineage::commit_owner_receipts`, the Exact edit verifier, the bound
BlockTransformationReceipt construction, SSC proof/verdict sources, Master/Render
adoption and serialization voice plan remain byte-identical to the parent. No
whole-document receipt, fallback, retry, candidate skip or weakened error exists.

## Tests and local checks

Three added synthetic tests cover silence for unchanged/covered transitions,
empty/partial coverage and discontinuity even when the end matches. The first
also probes repeated and independent observations. The existing synthetic
adoption follow-through is extended to show that projecting the diagnosis does
not consume receipts before their real enforced adoption. Existing fatal tests
are retained.

The preparing environment has no Rust toolchain or corpus inputs. Native
Parent-Admission, compilation, Clippy and Rust tests have not run here. Static
checks reproduce the parent selector detector and contract checks, compare the
final declaration/hash diff, verify frozen items and the incoming full tree,
and read every packaged byte back from the ZIP. These are not native gate results.

## Target run

Keep the accepted r1316 project, put only the new candidate into the usual Take
input and run:

```text
./run.cmd --take --all
```

The existing transactional bootstrap must perform native admission and checks.
No --force. After the run, inspect the new Assistant-Export's acceptance/rejection
evidence and both diagnostic markers.

This take contains no r1313 music fix. A green run of r1317 therefore does not
reproduce the r1313 candidate or remove the nine known voice collisions. If the
receipt failure recurs in the Parent-derived run, the markers identify its owner
and patch. If it does not recur, a controlled repeat of the original r1313 remains
necessary. Windows access violations still require separate crash evidence.
