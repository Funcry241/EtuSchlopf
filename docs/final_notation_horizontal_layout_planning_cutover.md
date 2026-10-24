# FinalNotationHorizontalLayoutPlanningCutover

Dieser Take schliesst die Zeitautoritaet der horizontalen Layoutplanung.

Der in den vorherigen Takes eingefuehrte zentrale PlanOwner und One-Materializer
bleiben unveraendert. Geaendert wird ausschliesslich der Zeitpunkt, an dem die
bestehende Legacy-Policy autoritativ entscheidet:

1. Die fruehe MasterNotationAndSalience-Phase berechnet ihren bisherigen
   Legacy-Plan nur noch als read-only Vergleichs-/Provenienzbeweis.
2. Dort wird keine horizontale Geometrie mehr materialisiert.
3. Nach dem bestehenden FinalPdfSourceCommit wird die ausgewaehlte finale
   Notationsquelle gelesen.
4. Genau dieselbe Legacy-Policy wird auf diesen finalen, strukturellen Facts
   erneut ausgefuehrt.
5. Dieser finale Plan ist die einzige produktive horizontale Decision-Wahrheit.
6. Derselbe Plan wird ueber genau denselben
   `HorizontalLayoutPlanMaterialization`-Owner auf Master- und Render-MusicXML
   materialisiert.
7. Erst danach werden FinalNotationLayoutSnapshot und PlanContract geschlossen
   und vor MuseScore exakt verifiziert.

Der reale v0.10.1830-r01-Shadow-Audit fuer Etuede 6 bewies:
44 Slots, 19 Decision-Aenderungen, 17 Width-Aenderungen,
3 Systembreak-Aenderungen, 0 Pagebreak-Aenderungen und 6 Provenienz-Aenderungen.
Damit ist der Cutover fachlich begruendet; er fuehrt keine neue Layoutformel,
keinen neuen Threshold, keine neue Page-Policy und keinen Global-Reflow ein.

Unveraendert bleiben insbesondere:
- `choose_auto_pianist_layout`
- `compute_page_breaks`
- `compute_system_breaks`
- Zweitakt-Grundraster
- VisualLayoutPressure-Formeln und Selector
- PianisticReadability-Formeln
- FinalShortSystemBalance-Semantik
- Width-Caps und Width-Ziele
- MuseScore
- alle Musik-/Render-Entscheidungen vor dem finalen Layout-Cutover

Keine Etueden-/Taktidentitaet, keine Repair-/Recovery-Schicht, keine zweite
Layoutpolicy und keine Security-Abschwaechung.
