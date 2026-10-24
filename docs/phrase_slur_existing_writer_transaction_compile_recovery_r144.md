# Phrase Slur Existing Writer Transaction Compile Recovery r144

Estimated plan position: Take 10/11.

Parent authority: accepted r141. Rejected r143 is not a parent.

This recovery is limited to the compile blocker exposed by the real Windows strict-Clippy run of r143. The previous candidate carried unnecessary phrase-slur audit/report plumbing and imported a horizontal-layout helper through an export that did not exist. r144 removes that extra surface instead of changing lint, report, or Security policy.

The final render MusicXML still receives exactly the same established horizontal-layout materialization as r141. The only prewire is a StructuralWriteAuthority wrapper that accepts the already typed `PhraseNotationIntentProjection` slice, observes only its length, and delegates to the existing `materialize_horizontal_layout_file_authorized` writer. No phrase intent is materialized as notation in this revision.

Invariants:

- no `<slur>` write;
- no AnchorResolver;
- no phrase/slur report section;
- no FinalizedReport mutation;
- no new threshold or number-level capacity;
- no master or sounding-semantic change;
- no Security policy change;
- no Clippy cleanup beyond the blocking compile errors.

The purpose is solely to establish a compile-valid, behavior-preserving writer transaction boundary for the next productive phrase-slur writer take.
