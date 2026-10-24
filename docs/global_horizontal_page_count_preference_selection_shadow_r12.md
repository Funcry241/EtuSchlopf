# GlobalHorizontalPageCountPreferenceSelectionShadow r12

## Korpusbeweis aus r11

Der r11-Vollkorpus umfasst alle zehn Etueden und 31 feasible PageCount-Candidates. Fuer jeden Kandidaten erreicht der separate Thin-first-Gegenfakten-DP exakt `thinLB`.

Damit sind zwei Dinge gleichzeitig bewiesen:

1. die bisherige Thin-Page-Eigenschaft ist innerhalb der vorhandenen harten Capacity-/MaxSystemMeasures-/VerticalMaxSystemsPerPage-Grenzen prinzipiell optimierbar;
2. Thin-first darf nicht selbst produktive Innenordnung werden, weil die Reduktion von `legacyThin` bei mehreren PageCounts mit erheblich schlechterer System-/Page-Slot-Balance erkauft wird und terminale Ein-System-Seiten entstehen koennen.

Die produktive Candidate-Basis einer spaeteren PageCount-Authority soll deshalb weiterhin die bereits etablierte `FullFrontier` sein.

## Neue read-only PreferenceSelection

r12 waehlt ausschliesslich fuer Evidence genau einen Candidate aus der `FullFrontier`. Es wird weder ein neuer Candidate berechnet noch ein Plan materialisiert.

Die lexikographische Shadow-Reihenfolge ist:

1. `systems`,
2. `singletons`,
3. `forced_over_capacity_singletons`,
4. `legacyThin`,
5. `avoidableThin = legacyThin - thinLB`,
6. `abs(pages - PreferredPages)`,
7. `system_count_balance_penalty`,
8. `page_slot_balance_penalty`,
9. `slack_square_sum`,
10. strukturelle Candidate-Reihenfolge.

Die ersten drei Dimensionen bewahren die bereits vorhandenen harten Feasibility-Fakten. `legacyThin` und `avoidableThin` sind reine vertikale Preferences. Erst danach entscheidet die Naehe zum bisherigen PageCount. So bleibt `PreferredPages` wirksam, ohne wieder harte Authority zu werden.

Wichtig: Die `ThinFirstFrontier` ist keine Candidate-Quelle. Sie bleibt ausschliesslich das Gegenfaktum, das beweist, welche Thin-Untergrenze prinzipiell erreichbar waere.

## Erwartete Korpusprojektion

Aus der bereits vorliegenden r11-Matrix folgt fuer dieselbe generische Ordnung:

- Etuede 1 -> 3 Seiten,
- Etuede 2 -> 4 Seiten,
- Etuede 3 -> 4 Seiten,
- Etuede 4 -> 3 Seiten,
- Etuede 5 -> 6 Seiten,
- Etuede 6 -> 6 Seiten,
- Etuede 7 -> 6 Seiten,
- Etuede 8 -> 6 Seiten,
- Etuede 9 -> 5 Seiten,
- Etuede 10 -> 6 Seiten.

Diese Liste ist ausschliesslich erwartete Regressionsevidence; es existiert keine Etueden-/Takt-spezifische Produktionslogik.

## Fail-closed

Wenn die `FullFrontier` nicht leer ist, muss die Selection aus genau dieser Frontier stammen und `ForcedOverCapacitySingletons == 0` besitzen. Eine leere Frontier bleibt wie bisher reine Evidence (`PreferenceSelection none`) und fuehrt durch r12 nicht zu einem neuen Abbruch. Nur eine interne Selection-/Frontier-Inkonsistenz bricht fail-closed ab.

## Unveraendert

- `PolicyWrite nein`,
- keine PageCount-Promotion,
- keine Aenderung an GlobalSystemReflow-/JointPageSystem-Cutover,
- keine Aenderung der bestehenden Legacy-Safety-Gates,
- kein Writer oder Materializer,
- keine Width-Autoritaet,
- keine Repair-/Recovery-Schicht,
- kein Etueden-/Taktwissen,
- keine MusicXML-/PDF-Aenderung.

Nach einem realen 10er-Korpuslauf kann die Selection-Regel als konkrete Grundlage fuer einen separaten `MusicMutation`-Cutover bewertet werden.
