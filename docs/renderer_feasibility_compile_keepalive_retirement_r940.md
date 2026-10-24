# r940 RendererFeasibilityCompileKeepaliveRetirement

## Ziel

Der historische r42-DecisionPreparation-Compile-Keepalive fuer RendererFeasibility wird physisch entfernt, nachdem die vorbereitete Capability inzwischen produktiv verdrahtet ist.

## Aktueller Reachability-Beweis

- `promote_renderer_feasibility_constraint_candidate` wird produktiv im Metric-/FinalNotation-Pfad aufgerufen.
- `build_renderer_feasibility_constraint_candidate` und `renderer_feasibility_constraint_segment_partition` werden im produktiven RendererFeasibility-Modul verwendet.
- `HorizontalLayoutDecisionOwner::RendererFeasibility` wird beim produktiven Promote fuer Page-/System-Owner gesetzt.
- Der r42-Block in `notation_endpoint_layout_core.rs` bindet diese Symbole dagegen nur side-effect-free als Function Items / Enum-Wert und besitzt keine Runtime-Wirkung.

## Schnitt

Entfernt werden ausschliesslich:

- der historische r42-DecisionPreparation-Kommentar;
- `_renderer_feasibility_segment_partition`;
- `_renderer_feasibility_candidate_builder`;
- `_renderer_feasibility_candidate_promoter`;
- `_renderer_feasibility_decision_owner`.

Keine RendererFeasibility-Funktion, kein Planner, keine Selection, keine Thresholds, kein Writer, kein Layout und keine MusicXML-/PDF-Semantik werden veraendert.

## ChangeClass

`AuditOnly`: Der aktuelle Take-Change-Control-Vertrag erlaubt in einer bestehenden music-sensitiven Rust-Datei einen reinen Cross-Role-Rueckbau, wenn nur bestehende Zeilen entfernt werden. Es wird keine neue Musik-, Writer-, Repair-, Recovery- oder Decision-Capability eingefuehrt.

## Erwarteter Beweis

Tree Integrity, Change-Control, Semantic Capability Diff, Strict Clippy, Build/All-Targets, gezielte Regression und realer Musiklauf muessen gruen bleiben. Fuer diesen Retirement-Take wird keine Musik- oder Artefaktaenderung erwartet.
