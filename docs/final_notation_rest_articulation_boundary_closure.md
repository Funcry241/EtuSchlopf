# FinalNotationRestArticulationBoundaryClosure

## Ziel

Dieser einzelne Mini-Schönheitstake trennt ausschließlich die bereits vollständig
abgegrenzte Rest-/Artikulations-Render-Stufe von der bisherigen gemeinsamen
`pipeline_orchestrator_final_notation_support.rs`. Es entsteht keine neue
Funktion, kein neuer Vertrag und keine neue musikalische Entscheidung.

## Physische Ownership-Grenze

Unverändert als vollständige Rust-Items verschoben werden:

- `RestAndArticulationRenderStageOutcome`
- `RestAndArticulationRenderStageInputs`
- `run_rest_and_articulation_render_stage`

Die Items liegen anschließend in
`pipeline_orchestrator_rest_articulation_render_stage.rs` und werden unmittelbar
nach `pipeline_orchestrator_final_notation_support.rs` im selben privaten
`pipeline`-Namensraum eingebunden. Der verschobene Block bleibt byteidentisch.

`pipeline_orchestrator_final_notation_support.rs` endet danach ausschließlich
mit der vollständigen `FinalRenderNotationAuthority`-Stufe.

## r02 Cross-File-Preflight-Closure

Der produktive r01-Payload bleibt unverändert. Der Take-Bootstrap bewertet das
physische Verschieben des Input-Structs konservativ als Feldentfernung aus der
alten Datei. Deshalb wird die bestehende Aufrufdatei mit einem reinen
Ownership-Kommentar als geänderte Cross-File-Aufrufstelle deklariert. Es wird
keine Rust-Expression und keine Signatur verändert.

## Verhaltenstreue

Unverändert bleiben insbesondere:

- Rest- und Artikulationslogik
- Writer- und Checkpoint-Reihenfolge
- Musikregeln und Schwellenwerte
- XML-Snapshot-Auswahl
- Fehlertexte und Auditprojektionen
- Reports und Releaseentscheidungen
- Master-/Render-MusicXML- und PDF-Erzeugung

Es gibt keine Etüden- oder Takt-spezifische Änderung und keine neue öffentliche
API.

## Erfolgskriterium

- Der verschobene Rest-/Artikulationsblock ist byteidentisch zu v0.10.1679.
- Der verbleibende Final-Notation-Authority-Block ist byteidentisch zu v0.10.1679.
- Beide Quelldateien besitzen vollständige Item-/Attributgrenzen.
- Strict-Take für Etüde Nr. 4 baut und läuft grün.
- Normalisierte Master-/Render-MusicXML bleiben identisch.
- Das PDF bleibt pixelidentisch.

## Stopkriterium

Mit grünem Artefaktbeweis endet dieser einzelne Miniwurf. Es folgt kein weiterer
Schönheitstake aus Clippy-, Naming- oder Zerlegungsinteresse. v0.10.1679 bleibt
bis zu einem vollständigen neuen 10/10-Korpuslauf der korpusautoritative Stand.
