# FinalNotationTimelineClearanceFactOwnerExtraction r513

## Ausgangspunkt

r512 `FinalNotationTimelineClearanceEdgeEvidenceAudit` ist fuer Etuede 1 installiert und dark green. Der read-only `RendererSystemBreakReadbackRhythmicShape` rekonstruiert `musicxml_timeline_columns_v1` und zeigt im finalen Etuede-1-Render genau einen `following_accidental_clearance_edge`: Takt 3. Derselbe Takt besitzt im bestehenden HorizontalLayout-Demand nur 117 und wird mit Takt 4 und 5 in ein Dreiersystem gepackt.

Diese Evidenz rechtfertigt noch keinen direkten Demand-Cutover aus dem Audit. Ein Audit darf nicht durch bloesse Wiederverwendung seiner internen Erkennungsfunktion zur produktiven Layout-Authority werden. Ebenso soll die Clearance-Erkennung nicht ein zweites Mal im produktiven Layout nachgebaut werden.

## RefactorOnly-Extraction

r513 extrahiert die bereits in r512 bewiesene, writerfreie Timeline-/Clearance-Faktenrekonstruktion aus `src/audit_only/renderer_system_break_readback.rs` in den neuen neutralen Owner `src/final_notation_timeline_facts.rs`.

Der neue Owner beobachtet ausschliesslich bereits finale MusicXML-Struktur:

- Measure-Slots und numerische Measure-Reihenfolge,
- Timeline-Spalten ueber Note/Chord/Backup/Forward,
- gepitchte Spalten und maximale Stack-Groesse,
- sichtbare 16tel-Restkanten mit direkt folgendem expliziten oder geaenderten Vorzeichen bei gleicher Staff/Voice- und diatonischer Pitch-Position.

Er besitzt keine Demand-, Capacity-, System-/Page-, Renderer-, Mutation-, Writer-, Repair- oder Release-Authority.

Der bestehende RendererSystemBreakReadback-Audit ruft nach r513 ausschliesslich diesen neutralen Fact-Owner auf. Seine Projektion, Feldnamen, Fehlertexte und bestehenden Regressionen bleiben erhalten. Es wird noch kein produktiver Layout-Consumer hinzugefuegt.

## Essenz

r513 setzt die bestehende Architekturlehre um: writerfreie Evidenz kann geteilt werden, aber AuditOnly-Code darf nicht selbst zur produktiven Authority werden. Die semantische Fact-Identitaet wird einmal zentral besessen; Audit und ein spaeterer bestehender HorizontalLayout-Owner duerfen dieselben Fakten lesen, ohne parallele Clearance-Wahrheiten zu erzeugen.

## Nicht geaendert

- keine MusicXML-/Render-/PDF-Aenderung,
- keine Restposition,
- keine `measure/@width`- oder `default-x`-Writes,
- keine Demandformel oder Demandzahl,
- keine Capacity- oder System/Page-Policy,
- keine Calibration-Schwelle,
- kein Renderer-Feedback-Cutover,
- keine StructuralWriteAuthority-/Writer-Callgraph-Aenderung,
- keine Etueden-/Takt-/Pitch-Sonderregel,
- keine Wiederbelebung pensionierter HorizontalLayout- oder Register-Capabilities.

`ChangeClass=RefactorOnly`. r513 ist ausschliesslich die neutrale Fact-Owner-Extraction vor einem spaeteren, separaten produktiven Spacing-Cutover.
