# ClefReadabilityShadowRenderArtifactClosure

## Purpose

Close the remaining Take-2 evidence ambiguity before any productive clef cutover.

`final_render_snapshot_xml` is the selected PDF-source snapshot. When the PDF source is Master MusicXML, that value intentionally falls back to the master even if an exported `.render.musicxml` artifact exists. Therefore a report section labelled as a final render snapshot could describe the master rather than the independent render artifact.

## Change

The clef-readability shadow now evaluates:

1. `MasterMusicXML` from `best.layout_xml`;
2. `RenderMusicXMLArtifact` by independently reading the existing render MusicXML output path.

The read is audit-only. If the render artifact is absent, the report says so explicitly and does not substitute master XML under a render label.

## Not changed

- no clef policy change;
- no cost or threshold change;
- no productive Rustig change;
- no writer change;
- no MusicXML or PDF mutation;
- no Take-3 cutover.

The purpose is solely to ensure that the two artifact surfaces which a later cutover may change are both directly observed.
