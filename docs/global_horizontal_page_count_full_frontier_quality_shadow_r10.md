# GlobalHorizontalPageCountFullFrontierQualityShadow r10

## Korpusbefund vor diesem Take

Der persistierte r09-Vollkorpus umfasst 10/10 Projekte auf v0.10.1839. Produktiv bleiben Etueden 6, 8 und 10 gruen; Etueden 1, 2, 3, 4, 5, 7 und 9 brechen an den unveraenderten Legacy-Reflow-Safety-Grenzen fail-closed ab.

Die PageCount-Frontier ist dagegen fuer alle zehn Projekte vollstaendig vorhanden. Jede bisherige `PreferredPages`-Zahl ist feasible, zugleich existiert fuer jedes Projekt mindestens eine niedrigere feasible Seitenzahl ohne Forced-Over-Capacity-Singleton. Das beweist, dass der alte PageCount keine harte Feasibility-Wahrheit ist.

Der bisherige Shadow reicht aber noch nicht fuer eine Auswahlpolicy. Er persistiert nur den `ClosestLowerFeasiblePages`-Kandidaten. Bei Etueden 2–4 besitzt dieser 5-Seiten-Kandidat weiterhin nur 2..3 Systeme pro Seite. Ob der ebenfalls feasible 4-Seiten-Kandidat die historische Thin-Page-Situation besser aufloest und welche Balance-/Slack-Kosten dabei entstehen, ist im Korpus noch nicht sichtbar.

## Erweiterte Evidence

r10 behaelt dieselben harten Feasibility-Grenzen und keine produktive Policy. Der bisherige untere Test-Rand `ceil(fixed_page_reference_systems / VerticalMaxSystemsPerPage)` wird jedoch als noch page-legacy-kontaminiert erkannt. Der neue read-only Unterrand wird aus der global minimalen Systempartition ohne Page-Grenzen unter derselben akzeptierten SystemCapacity und demselben MaxSystemMeasures abgeleitet. Dieser Wert wird als `HorizontalMinimumSystems` ausgewiesen. Danach wird jeder feasible Kandidat bis `PreferredPages` als `FullFrontier` projiziert.

Pro Kandidat werden gemeldet:

- `HorizontalMinimumSystems`: global minimale Systemzahl ohne Page-Grenzen unter denselben Hard-Constraints,
- `pN`: Seitenzahl,
- `sys`: Systemanzahl,
- `sing`: Singleton-Anzahl,
- `legacyThin`: Zahl nichtterminaler Seiten mit weniger als drei Systemen; dies ist ausschliesslich die Messprojektion des bestehenden Legacy-Guards,
- `slots`: minimale..maximale Takte/Slots pro Seite,
- `spp`: minimale..maximale Systeme pro Seite,
- `maxLoad`: maximale Systemlast,
- `forced`: Forced-Over-Capacity-Singletons,
- `sysBal`: bereits intern berechnete System-Count-Balance-Kosten,
- `slotBal`: bereits intern berechnete Page-Slot-Balance-Kosten,
- `slack`: bereits intern berechnete Slack-Quadratsumme.

Die Candidate-Qualitaetswerte werden aus dem bereits bestehenden `PageCountPreferenceCandidate` gelesen. Die einzige zusaetzliche Suche ist die read-only globale Minimal-Systempartition zur Entkopplung des Test-Unterrandes von alten Page-Grenzen. Keine produktive Candidate-Reihenfolge, kein Score und keine Auswahl wird veraendert.

## Architekturabsicht

Der naechste Architekturentscheid soll nicht `6 Seiten -> 5 Seiten` codieren und auch nicht `mindestens 3 Systeme/Seite` als neue harte Wahrheit konservieren. Stattdessen soll der Vollkorpus zeigen, wie sich PageCount-Naehe, Thin-Page-Symptom, Balance und horizontale Slack-Kosten entlang der gesamten feasible Frontier gegeneinander verhalten.

Erst danach darf entschieden werden, ob der historische Minimum-Systeme-Guard zu einer Soft Preference demotiert und wie diese Preference gegen `PreferredPages` geordnet wird.

## Unveraendert

- `PolicyWrite nein`,
- keine produktive PageCount-Promotion,
- keine Aenderung der beiden bestehenden GlobalSystemReflow-Safety-Gates,
- keine Aenderung am Joint Page/System Cutover,
- keine Width-Policy,
- kein Writer/Materializer,
- keine Repair-/Recovery-Schicht,
- keine Identitaets-/Etueden-/Taktlogik.
