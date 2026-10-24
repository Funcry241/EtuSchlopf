# r346 FinalRenderRawCursorDecisionOwnershipEvidence

Parent: r345 `HiddenChordToneRecoverySharedDecisionOwnerClosure`

ChangeClass: `AuditOnly`

## Reale Ausgangslage

r345 ist im gelieferten Einzelprojektlauf GREEN und released: `FinalExit 0`, Build 0,
Clippy 0 und 1088/1088 integrierte Tests. Master- und Render-MusicXML sind gegen r344
byteidentisch; die sechs PDF-Seiten sind pixelidentisch. `HiddenChordToneRecovery` ist
damit als gemeinsamer DecisionOwner-Refactor bestaetigt.

## Naechstes Ownership-Signal

Der r345-UnifiedDomainOwnership-Bestand zeigt im Domain-Bereich `Other` einen spaeten
FinalRender-Cursor-Pfad. Die Quellpruefung zeigt zwei produktive Grenzen fuer dieselbe
Raw-Cursor-Endinvariante:

1. `CursorLaneEndInvariantPrimaryBoundary` in
   `report_hygiene_cursor_localbest_trace_tail.rs` um die bestehende Measure-Cursor-Closure.
   Sie wird fuer Master und Render im InitialRender-Stadium angewandt und darf einen roten
   Measure-Cursor wieder in einen gruene Endlage ueberfuehren.
2. `FinalRenderMasterMeasureFallback` in
   `pipeline_orchestrator_final_notation_support.rs` an der letzten Render-Notationsgrenze.
   Nach spaeteren Render-Writes prueft sie dieselbe Raw-Cursor-Endinvariante erneut und kann
   fuer einen roten Render-Takt den korrespondierenden gruene Master-Takt als Quelle waehlen,
   sofern Metrik und nicht-timeline-relevante Surface uebereinstimmen.

Der aktuelle Etuede-6-Lauf ist an dieser Grenze gruen (`HardViolations 0`), daher ist der
spaete Pfad dort No-op. Produktiv erreichbar bleibt er trotzdem.

## Warum das noch kein produktiver Cut ist

Mehrere Zeitpunkte allein beweisen keine Mehrfach-Authority. Der fruehe Cursor-Abschluss
liegt vor weiteren Render-Materialisierungen; ein finaler committed-state Readback ist daher
grundsaetzlich legitim. Offen ist aber, ob der spaete Pfad nur dieselbe bereits entschiedene
Cursor-Wahrheit materialisiert oder mit der Master-Quellwahl eine zweite eigenstaendige
Recovery-Entscheidung besitzt.

r346 friert deshalb ausschliesslich diese Ownership-Frage als explizites Pattern ein:

`FinalRenderRawCursorClosure`

- PrimaryBoundary: `CursorLaneEndInvariantPrimaryBoundary`
- LateBoundary: `FinalRenderMasterMeasureFallback`
- SameInvariant: `raw_cursor_measure_end`
- SharedDecisionOwner: `none`
- OwnershipState: `multiple_productive_raw_cursor_closure_boundaries_open`

Erst ein realer GREEN-Lauf darf einen Folgetake legitimieren, der entscheidet zwischen:

- einem gemeinsamen DecisionOwner mit mehreren Materialisierungszeitpunkten, oder
- der Pensionierung des spaeten Master-Measure-Fallbacks zugunsten eines finalen fail-closed
  Raw-Cursor-Gates.

## Nicht geaendert

- keine produktive Rust-Datei
- keine MusicXML-/Render-/PDF-Logik
- keine Pass-Reihenfolge
- keine Schwelle oder Gewichtung
- kein neuer Writer
- keine neue Recovery-Schicht
- keine Security-/Gate-Lockerung
- Beam, RestSpanConsolidation, CrossStaff, RestExistence und HiddenChordToneRecovery bleiben
  geschlossen

Der beabsichtigte MusicXML-/Render-/PDF-Delta ist null.
