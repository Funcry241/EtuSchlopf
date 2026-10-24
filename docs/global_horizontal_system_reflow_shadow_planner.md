# GlobalHorizontalSystemReflowShadowPlanner

## Zweck

Take 10 ist die erste neue horizontale Layoutpolicy nach Abschluss von Owner-, Materializer- und Zeitautoritaet. Sie bleibt bewusst read-only. Der reale v0.10.1831-r02-Lauf hat den produktiven Final-Notation-Cutover DARK GREEN bestaetigt und exakt den zuvor gemessenen Decision-Drift materialisiert: 44 Slots, 19 geaenderte Decisions, 17 Width-Aenderungen, 3 Systembreak-Aenderungen, 0 Pagebreak-Aenderungen. Damit darf neue Policy erstmals auf der finalen Notationswahrheit experimentieren.

## Eingangsvertrag

Der Shadow-Planner erhaelt nur:

- den finalen, noch nicht horizontal materialisierten Notationsstand,
- die daraus extrahierten identity-freien `MeasureHorizontalLayoutFacts`,
- die autoritativen finalen Legacy-Page-Grenzen,
- die bereits akzeptierte Legacy-Systemlast nur als konservative Kapazitaetsuntergrenze.

Er liest keine Projekt-, Etueden- oder absolute Taktidentitaet und besitzt keinen Writer.

## Demand-Modell

Pro strukturellem Slot gilt:

`Demand = nominaler Measure-Anteil + bestehender VisualLayoutPressure-Score`.

Es wird absichtlich keine zweite Notations-Fact-Formel eingefuehrt. `VisualLayoutPressure` bleibt der bereits vorhandene read-only Druckindikator; der nominale Anteil verhindert, dass druckarme Measures als geometrisch kostenlos gelten.

Die typische Systemkapazitaet wird nicht als feste Pixel-/Width-Zahl kalibriert. Sie ist scoreweit der Median der finalen Slot-Demands multipliziert mit drei typischen Slots. Pro bestehender Seite wird diese Kapazitaet auf mindestens die dort bereits produktiv akzeptierte maximale Legacy-Systemlast angehoben. Der Shadow darf damit nie behaupten, ein bereits akzeptiertes Legacy-System sei allein wegen einer strengeren neuen Kapazitaetsgrenze unzulaessig.

## Globale Partition pro fixer Seite

Pagebreaks bleiben in Take 10 unveraenderte harte Grenzen. Innerhalb jeder Seite untersucht eine deterministische Dynamic-Programming-Partition zusammenhaengende Systeme mit 1 bis 4 Slots. Vier ist nur ein Kandidatenmaximum und keine Zielgroesse; durch die medianbasierte typische Kapazitaet koennen vier Slots nur dann gemeinsam bestehen, wenn sie aussergewoehnlich leicht sind oder die Seite bereits eine entsprechend hohe produktiv akzeptierte Last besitzt.

Die Auswahl ist lexikographisch:

1. minimale Systemzahl innerhalb der Kapazitaetsgrenzen,
2. minimale Singletonzahl,
3. minimale Zahl erzwungener ueberkapazitiver Einzel-Slots,
4. minimale Summe quadratischer Restkapazitaet,
5. strukturell deterministische Tie-Break-Reihenfolge.

Mehrslot-Systeme oberhalb der gebundenen Kapazitaet sind unzulaessig. Ein einzelner dichter Slot darf nicht verschwinden und wird deshalb als `ForcedOverCapacitySingleton` sichtbar gehalten.

## Audit

Der produktive Legacy-Plan bleibt unveraendert und wird weiterhin allein materialisiert. Der Shadow meldet aggregiert:

- Slots / Pages,
- LegacySystems / ShadowSystems,
- LegacySingletons / ShadowSingletons,
- Drei- und Vier-Slot-Systemzahlen,
- SystemBreakChanges / AddedBreaks / RemovedBreaks,
- TypicalSystemCapacity / MaxPageCapacity / CapacityRaisedPages,
- ForcedOverCapacitySingletons,
- NonterminalPagesBelowThreeSystems,
- MaxLegacySystemMeasures / MaxShadowSystemMeasures,
- `PageBreakChanges 0`,
- `PolicyWrite nein`.

Keine konkrete Taktidentitaet wird ausgegeben oder fuer eine Entscheidung benutzt.

## Vorfeldprojektion Etuede 6

Eine lokale read-only Reproduktion auf der v0.10.1831-r02 Render-MusicXML ergibt voraussichtlich:

- 44 Slots / 6 Seiten,
- 26 Legacy-Systeme -> 21 Shadow-Systeme,
- 9 Legacy-Singletons -> 2 Shadow-Singletons,
- 1 Legacy-Drei-Slot-System -> 4 Shadow-Drei-Slot-Systeme,
- 0 Vier-Slot-Systeme,
- 11 geaenderte Systembreaks, davon 3 hinzu und 8 entfernt,
- 0 Pagebreak-Aenderungen,
- 0 erzwungene Ueberkapazitaets-Singletons,
- 0 nichtterminale Seiten mit weniger als drei Shadow-Systemen,
- maximale Systemlaenge weiterhin 3.

Das ist keine Release-Evidence. Der Windows-Strict-Lauf muss die Rust-Implementierung und den echten Korpuspfad bestaetigen.

## Unveraendert

Unveraendert bleiben insbesondere:

- der produktive `FinalNotationHorizontalLayoutPlanningCutover`,
- der produktive `HorizontalLayoutLegacyPlan`,
- alle Width-Entscheidungen und Width-Ziele,
- alle bestehenden Legacy-Systembreaks und Pagebreaks,
- `FinalShortSystemBalance`,
- der One-Materializer,
- Master-/Render-MusicXML,
- MuseScore/PDF,
- alle vertikalen Layoutwriter,
- alle Musik-/Render-Entscheidungen,
- Security-Gates.

Kein Repair-/Recovery-Layer, keine zweite produktive Wahrheit und keine Etueden-/Taktspeziallogik.
