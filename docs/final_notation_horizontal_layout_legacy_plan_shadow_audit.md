# FinalNotationHorizontalLayoutLegacyPlanShadowAudit

## Zweck

Take 8 der horizontalen Layoutmigration bleibt `RefactorOnly` und read-only. Der reale
v0.10.1829-r02-Lauf hat fuer Etuede Nr. 6 bewiesen, dass die Rohfakten der fruehen
horizontalen Planung bis zum finalen snapshot-gebundenen Notationszustand massiv
driften: 44/44 Slots, 235 Feldwerte und 15 Fact-Familien.

Dieser Take beantwortet deshalb vor einem produktiven Planning-Cutover die naechste
Frage: Welche **konkreten Legacy-Layoutentscheidungen** wuerden sich allein dadurch
aendern, dass exakt dieselbe Legacy-Policy auf den finalen Notationsfakten statt auf
dem fruehen Zwischenzustand arbeitet?

## Architektur

Die bisher gemischten horizontalen Decision-Teile von `VisualLayoutPressure` und
`PianisticReadability` werden in read-only Planfunktionen getrennt. Die bestehenden
Wrapper wenden anschliessend unveraendert ihre vertikalen `staff-distance`- bzw.
Octave-Y-Oberflaechen an; produktives Verhalten bleibt dadurch unveraendert.

Nach `FinalNotationLayoutSnapshotClosure` wird aus dem bestehenden strukturellen
Legacy-Seed und der finalen snapshot-gebundenen Notationsquelle ein zweiter
`HorizontalLayoutLegacyPlan` berechnet. Dieser Shadow-Plan besitzt **kein Schreibrecht**
und wird nicht materialisiert. Er wird nur gegen den produktiven fruehen Plan
verglichen.

Gemeldet werden:

- `Slots`
- `ChangedSlots`
- `WidthChanges`
- `SystemBreakChanges`
- `PageBreakChanges`
- `ProvenanceChanges`
- `PolicyWrite nein`

## Unveraenderte Policy

Unveraendert bleiben insbesondere:

- `choose_auto_pianist_layout`
- `compute_page_breaks`
- `compute_system_breaks`
- Zweitakt-Grundraster
- DenseRegister-Scores und 1680-Cap
- VisualLayoutPressure-Scores, Thresholds und Breakselector
- PianisticReadability-Scores und 2050/2300-Ziele
- `FinalShortSystemBalance`
- produktiver One-Materializer aus v0.10.1828
- alle vertikalen Writer
- Music-/Render-Writer
- MuseScore und PDF-Quellenauswahl
- Security-Gates

Keine neue Layoutpolicy, keine Musikmutation, keine Repair-/Recovery-Authority und
keine Etueden-/Taktspeziallogik.

## Entscheidung fuer den Folgetake

`ChangedSlots == 0` wuerde einen direkten Policy-Schritt erlauben. `ChangedSlots > 0`
beweist dagegen, dass der produktive Planning-Cutover auf den finalen Notationszustand
vor dem Global-Reflow erfolgen muss.
