# AbsorbedPreOwnerWriterRetirementSuperwurf

## Version

`v0.10.1774 r01`

## Ziel

Dieser Refactor entfernt zwei produktive Vor-Authorities, deren vollstaendige fachliche Wirkung von unmittelbar spaeteren allgemeineren Ownern absorbiert wird. Es wird keine Musikregel, Schwelle, Passordnung des verbleibenden Owners oder Security-Grenze geaendert.

## 1. DynamicLowerRestBridgeBeam -> ExternalFeedbackClosure

Der pensionierte Dynamic-Writer selektierte ausschliesslich sichtbare Staff-2/Voice-2-`note-rest-note`-Tripel innerhalb derselben Zaehlzeit mit 16tel-Aussenknoten und maximal zwei 16teln Restlaenge. Der spaetere `structural_lower_rest_bridge_triples`-Owner von `ExternalFeedbackClosure` selektiert dieselbe Struktur fuer **jede** Voice auf Staff 2, erlaubt Aussenknoten bis zwei 16tel und materialisiert denselben vollstaendigen Beam-Level-1+2-Plan.

Damit gilt fuer die Beam-Wahrheit:

`Targets(Dynamic) subset Targets(External)`

und fuer jeden Dynamic-Zielzustand:

`Normalform_Dynamic(x) = Normalform_External(x)`

also das Absorptionsgesetz:

`External(Dynamic(S)) = External(S)`.

Der reale 1773-Korpus bestaetigt, dass Dynamic kein toter Pfad war: 58 Zellen in 9/10 Projekten wurden produktiv geschrieben. Gerade deshalb ist die Pensionierung ein Authority-Cut und kein Dead-Code-Cleanup. Nach Entfernung sieht External diese bisher vor-normalisierten Zellen selbst.

Entfernt werden der Dynamic-Detector/Writer/Audit, seine exklusive `LowerRestBridgeBeam`-Checkpoint-Authority und die Report-/Source-Ledger-Projektion. Der allgemeine External-Owner bleibt unveraendert.

## 2. ShortRestGlyphSpecificPlacement -> FinalPianoRestGlyphCanonicalHeight

Der pensionierte ShortRest-Writer selektierte sichtbare Nicht-Chord-Pausen mit positiver Dauer aus der Untermenge `quarter/eighth/16th/32nd/...`. Der unmittelbar folgende Final-Piano-Owner selektiert **alle** sichtbaren Nicht-Chord-Pausen mit positiver Dauer und setzt fuer jede seine kanonische clef-aware Display-Position.

ShortRest aendert keines der Selektionsmerkmale des Final-Piano-Owners: Rest/Note-Identitaet, Visibility, Staff, Voice, Duration, Type, Chord-Status und Clef-Kontext bleiben unveraendert. Daher gilt strukturell:

`Targets(ShortRest) subset Targets(FinalPianoRest)`

und

`FinalPianoRest(ShortRest(S)) = FinalPianoRest(S)`.

Im 1773-Korpus wurde ShortRest 10/10 aufgerufen, erzeugte 122 Kandidaten, durfte aber 0 Writes committen. Die Pensionierung entfernt zugleich den zu starken fremddomaenenabhaengigen absoluten RawCursor-Veto dieses Vorwriters; der Final-Piano-Owner besitzt bereits den passenden delta-basierten RawCursor-Stabilitaetsvertrag.

Entfernt werden der komplette ShortRest-Shard, sein Audittyp, sein `PostFragmentRestGlyph`-Checkpoint-Sink sowie aktive RepairLoop-/Reportprojektionen. `FragmentedRestChainTransactionalWrite` und `PianoFinalRestGlyphCanonicalHeight` bleiben unveraendert.

## Harte Nichtaenderungsbedingungen

Der Wurf ist `RefactorOnly`. Erwartung gegen v0.10.1773 r01:

- Master-MusicXML semantisch invariant, abgesehen von Versionsmetadaten.
- Render-MusicXML semantisch invariant, abgesehen von Versionsmetadaten.
- PDF invariant.
- `CommittedPrimaryClefReentryChanges = 0`.
- `CommittedRestPlacementCandidates = 0`.
- `CommittedHigherBeamCandidates = 0`.
- `CommittedRawCursorGreen = ja`.
- `CommittedPitchedEventMultisetStable = ja`.
- `RendererMusicXmlWrites = 0`.
- ExternalFeedbackClosure uebernimmt die zuvor von Dynamic vor-normalisierten Rest-Bridge-Zellen.
- Kein neuer Writer, keine neue Musikregel, kein neuer Selektor, keine Security-Aenderung.

## Nicht Teil dieses Wurfs

`ShadowRest`, `FinalBeamLostBegin`, `Variant-A Next-Beat Cleanup`, `PartialBeatPostRestTripleBeam`, Layout/VisualLayoutPressure, Master-Solver und Security bleiben unveraendert. Insbesondere wird die Write-Footprint-Leakage von Variant A nicht mit einem algebraisch output-invarianten Retirement vermischt.

## Revision r02 – Compile-Closure

r01 erreichte den realen Strict-Clippy-Compile und zeigte genau zwei physische Schnittfehler des Retirement-Schnitts: Beim Entfernen der DynamicLowerRestBridge-Familie wurde die unmittelbar folgende `#[derive(Debug, Clone, Default)]`-Zeile des weiterhin produktiven `ExternalFeedbackClosureRenderWriteAudit` mit entfernt; beim Entfernen der ShortRest-Familie lag der generische, vom weiterhin produktiven `PianoFinalRestGlyphCanonicalHeight` verwendete Helper `current_rest_display_position()` innerhalb des geloeschten Blocks. r02 stellt ausschliesslich diese beiden ueberlebenden Schnittstuecke wieder her. Keine pensionierte Authority, Capability, Musikregel oder Passordnung kehrt zurueck.

