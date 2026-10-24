# r1060 Renderer-Visible Primary-Beam Geometry Readback

## Purpose

This take adds read-only evidence for the already-frozen primary-beam serialization/voice-partition error class. It reads the final MuseScore PDF vector geometry and binds visible Level-1 beam continuity back to already exact cross-voice occurrences.

## Boundary

- ChangeClass: `AuditOnly`
- MusicMutation: `no`
- RuleChange: `no`
- Productive selector: `none`
- Repair authority: `none`
- Second renderer run: `none`
- OCR: `none`
- Render-MusicXML beam tags are not accepted as renderer-visible geometry.

## Readback

The final PDF is consumed after the normal MuseScore render. Existing PDF object/stream parsing is extended with vector-path observation. Measure/staff cells are bound from five staff-line vectors. Visible stem columns are accepted only when the complete measure/staff occurrence order has unique onsets and the detected stem cardinality exactly matches the Render-MusicXML stem-bearing occurrence cardinality. Filled four-edge beam polygons are then projected to sloped beam centerlines. A beam may connect two occurrence stems only when its x-span covers both columns and its centerline intersects both bound stem segments, preventing an unrelated beam elsewhere on the staff from satisfying the readback.

For an already exact cross-voice occurrence cohort, adjacent members are renderer-connected only when a detected beam span geometrically covers both bound stem columns. `visible_fragments = member_count - connected_adjacent_pairs`. Any ambiguous staff cell, stem cardinality, occurrence identity, or geometry remains `UNAVAILABLE`.

## Intended evidence

The report adds `RendererVisiblePrimaryBeamGeometryAudit` rows and supplies renderer-visible fragment counts to the existing frozen error-class projection. This may close the renderer-geometry obligation, but it does not create canonical beam authority and it does not grant write permission. A fragmented occurrence whose canonical authority is still unresolved remains fail-closed.

## Invariants

1. PDF geometry is observation only.
2. No project, etude, measure, page, pitch, voice number, or known witness location selects eligibility.
3. Exact occurrence identity precedes renderer geometry binding.
4. Ambiguity yields `UNAVAILABLE`, never a guessed visible fragment count.
5. The existing frozen error-class definition remains unchanged.
6. No writer or repair behavior changes in this take.
