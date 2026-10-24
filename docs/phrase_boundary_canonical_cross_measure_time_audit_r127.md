# Phrase Boundary Canonical Cross-Measure Time Audit Compile Recovery r127

## Parent

Accepted parent: EtuSchlopf v0.10.1839 r125 `MusicXmlParserWriterReachabilityDecouplingRefactor`.

r126 attempted the canonical cross-measure audit on r125 but failed compilation during strict Clippy because `src/report.rs` still called `phrase_boundary_evidence::append_report_section()` while the r126 replacement of `phrase_boundary_evidence.rs` had accidentally dropped that already-existing r125 bridge function.

r127 is a fresh `AuditOnly` recovery candidate on r125. The failed r126 revision identity is not reused.

## Recovery scope

r127 carries the intended r126 cross-measure implementation and restores exactly the missing read-only report bridge from r125:

`report::write_text_report -> phrase_boundary_evidence::append_report_section -> append_snapshot`

The bridge only reads the thread-local audit snapshot and appends diagnostic text. It does not mutate MusicXML, sounding events, phrase structure or writer state.

No Clippy warning cleanup is bundled into this recovery. The warning budget remains diagnostic-only; the r126 blocker was compiler error E0425, not the existing stylistic warnings.

## Musical purpose

The audit reuses the canonical parsed MusicXML metric owner `musicxml::parse_musicxml() -> Score -> Part -> Measure` after the accepted r125 parser writer-reachability decoupling.

It projects exact canonical time across measure boundaries and keeps the existing contracts:
- barline is context only, never phrase evidence;
- tie-stop-only onsets are continuity, not attacks;
- positive and negative evidence remain separate;
- no scalarization or PhraseConfidence;
- no PhraseSpan, SlurIntent, NotationPolicy or AnchorResolver;
- no MusicXML, notation, master or sounding-event mutation;
- fail-closed on incomplete or non-exact metric binding.

## Expected Etüde Nr. 6 audit

Diagnostic expectation, not ground truth:
- TrueAttacks 871
- BoundaryObservations 868
- SameMeasure 785
- CrossMeasureRhythmResolved 83
- CrossMeasureRhythmUnresolved 0
- PositiveGapSupport 72
- ExplicitRestSupport 67
- ContiguousOpposition 794
- OverlapOpposition 2
- PromotedBoundaryCandidates 0
- PauseGeneratedPositions 0

## Acceptance

The real Windows one-command gate remains authoritative. r127 is accepted only if Semantic-Capability-Diff, build, strict Clippy, strict all-target tests, renderer/readback and final release decision are green.
