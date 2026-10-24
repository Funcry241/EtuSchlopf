# ClefReadabilityShadowFinalSourceClosure

## Scope

This is an audit-only closure for Take 2 of the clef-readability block.

The v0.10.1710 full-corpus run proved that the first shadow planner is behavior-neutral, but it also exposed a source-boundary gap: the report evaluated `best.layout_xml` only, while the final PDF-source render snapshot may contain a different lower-staff clef course after later render/readback stages.

## Change

The same writer-free readability planner is now evaluated twice:

1. `MasterMusicXML` from `best.layout_xml`
2. `FinalRenderSnapshot` from the already available `final_render_snapshot_xml`

Both projections stay inside the same report section and retain the same named cost dimensions.

Shadow omission is additionally restricted to `SurfaceEvent::removable`, the existing structural fact that the attributes block contains exactly one removable lower-staff clef and no unrelated attributes payload.

## Not changed

- no MusicXML mutation
- no render mutation
- no clef materialization
- no writer-authority change
- no ledger-pressure weighting change
- no clef-transition weighting change
- no short-excursion weighting change
- no local `>3` policy change
- no Take-3 cutover

## Stop gate

Take 3 is allowed only after the full corpus shows that both Master and FinalRenderSnapshot shadow courses are musically explainable and no final-PDF-source clef cascade is introduced.
