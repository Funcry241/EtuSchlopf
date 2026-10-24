# MusicXML Parser Writer-Reachability Decoupling Refactor r125

## Parent

EtuSchlopf v0.10.1839 r123 `PhraseBoundaryTieAwareAttackContinuityAuditNoMutation` remains the accepted parent.

r124 `PhraseBoundaryCanonicalCrossMeasureTimeAuditNoMutation` was rejected fail-closed by the real Windows Semantic-Capability-Diff gate. The audit code called `musicxml::parse_musicxml()`, and the call graph classified that parser as writer-reachable.

## Root cause

The parser itself does not write MusicXML. The writer reachability comes from its helper `strip_doctype()`: the helper used `String::replace_range()` to remove DOCTYPE declarations. The semantic capability system intentionally treats `replace_range()` inside the MusicXML domain as a writer primitive. Therefore any caller of `parse_musicxml()` inherits writer reachability through `parse_musicxml -> strip_doctype`.

This is a capability-shape problem, not a reason to weaken AuditOnly.

## r125 scope

r125 is `RefactorOnly`. It does not implement cross-measure phrase evidence.

`strip_doctype()` is rewritten as a forward read/copy projection:

- copy unchanged text before each DOCTYPE;
- skip the DOCTYPE span identified by the unchanged `find_doctype_end()` contract;
- continue from the original immutable source;
- if a DOCTYPE is malformed, return exactly the prefix that the previous implementation retained.

There is no `replace_range()` and no `truncate()` in the helper after r125. The returned text is intended to remain byte-for-byte behavior-equivalent to the previous helper for all accepted and malformed cases. Focused tests pin unchanged XML, multiple declarations, internal subsets/quotes and malformed-declaration truncation.

## Why this is preferable to an audit-side meter parser

A temporary recovery design could have re-parsed `divisions`, meter and cursor extent inside the phrase audit. That would avoid writer reachability but would create a second metric interpretation path. r125 deliberately rejects that approach.

The architectural target remains one metric/parser truth. After r125 is accepted, a later AuditOnly revision may attempt the canonical `parse_musicxml()` read path again. If the semantic gate now confirms that the parser is no longer writer-reachable, cross-measure phrase timing can reuse the existing `Score -> Part -> Measure` owner directly.

## Behavior and authority

r125 changes no:

- score semantics;
- parsing rules;
- metric values;
- MusicXML output;
- notation;
- sounding event;
- phrase evidence output;
- pass order;
- thresholds;
- writer authority;
- security policy.

No security exception or capability allowlist is added.

## Next gate

Run the normal real Windows take gate on Etüde Nr. 6. r125 is useful only if the complete build/strict/render result remains green. Do not pre-authorize r126 before that result.
