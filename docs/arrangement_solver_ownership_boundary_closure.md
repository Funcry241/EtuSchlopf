# ArrangementSolverOwnershipBoundaryClosure

## Ziel

Der erste Take des letzten Architektur-/Schönheitssuperwurfs schließt ausschließlich die fachliche Ownership-Grenze des Arrangement-Solvers. `pipeline_orchestrator_arrangement_solver_stage.rs` soll nur den Stage-Vertrag und die musikalische Solver-Steuerung besitzen.

## Verschobene vollständige Items

Aus `pipeline_orchestrator_arrangement_solver_stage.rs` werden genau 191 physische Schlusszeilen mit fachfremden Items entfernt; davon sind 188 nichtleere/inhaltliche Schnittzeilen einschließlich der vollständigen Itemblöcke.

Zum bestehenden Final-Report-Projection-Owner wechseln byteidentisch:

- `append_visual_pdf_triage_audit_no_music_change_section`
- `append_visual_pdf_triage_locator_ranking_no_music_change_section`

Zum bestehenden Clef-/Layout-Support wechseln byteidentisch:

- `LOWER_CLEF_SURFACE_SEMANTIC_ROLE`
- `derive_lower_clef_rustig_surface_decision`
- das vollständige `#[cfg(test)]`-Modul `rustig_clef_surface_decision_tests` einschließlich seines Attributs
- `append_typed_lower_clef_surface_materialization_result_section`

## Statischer Grenzbeweis

Die Solver-Datei endet nach dem vollständigen Item `run_arrangement_solver_stage`; keine Attribute, Docs, Konstanten, Tests oder Funktionsköpfe bleiben an der Schnittkante zurück. Die verschobenen Blöcke werden gegen den v0.10.1676-Ausgangstext byteweise geprüft.

Die Clef-Surface-Items verwenden bereits bestehende Clef-/Layout-Helfer (`ClefSurfaceNoteEvent`, `collect_clef_surface_note_events`, `clef_context_readability_penalty`) und liegen damit nun bei ihrem natürlichen Owner. Die beiden Visual-PDF-Triage-Funktionen werden ausschließlich aus `final_report_projection_owner.rs` projiziert und liegen nun direkt an dieser Verwendung.

## Verhaltenstreue

Keine Solver-Bedingung, kein Profil, kein Kandidatenvergleich, kein Iterationszähler und kein musikalischer Pass wird geändert. Ebenso unverändert bleiben Writer, MusicXML-/PDF-Pfade, Reports als Ausgabe, Releasebedingungen, Fehlertexte, Allow-Direktiven, Musikregeln und Schwellenwerte.

## Erfolgskriterium

- `pipeline_orchestrator_arrangement_solver_stage.rs` enthält ausschließlich Solver-Vertrag und Solver-Steuerung.
- Alle verschobenen Items sind gegenüber v0.10.1676 byteidentisch.
- Der Strict-Take für Etüde Nr. 4 baut und läuft grün.
- Normalisierte Master-/Render-MusicXML sowie PDF bleiben gegenüber dem v0.10.1676-Beweisstand identisch.

## Stopkriterium

Nach geschlossener Ownership-Grenze wird in diesem Take keine Solver-Funktion zerlegt und keine weitere Clippy-/Kosmetikbaustelle begonnen. Take 2 beginnt erst nach grünem Beweis dieses Takes.
