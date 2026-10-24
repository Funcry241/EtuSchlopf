# GlobalHorizontalPageCountThinFirstCounterfactualShadow r11

## Ausgangslage

Der r10-Vollkorpus zeigt fuer alle zehn Projekte die vollstaendige feasible PageCount-Frontier. Die bisherige exakte Auto-Seitenzahl ist ueberall feasible, aber nirgends die einzige feasible Seitenzahl. Alle 31 beobachteten PageCount-Candidates besitzen 0 Singletons und 0 Forced-Over-Capacity-Singletons.

Die r10-Candidates sind jedoch innerhalb jeder festen Seitenzahl noch nach der alten read-only Candidate-Reihenfolge gewaehlt:

1. minimale Systemzahl,
2. minimale Singletons,
3. minimale Forced-Over-Capacity-Singletons,
4. System-Count-Balance,
5. Page-Slot-Balance,
6. Slack-Quadratsumme.

`legacyThin` wird nur gemessen. Dadurch ist noch offen, ob ein bei r10 duenner Candidate innerhalb derselben Seitenzahl eine alternative Partition mit weniger duennen Nicht-Endseiten besitzt und welche Kosten dieser Tausch haette.

## Neues Gegenfaktum

r11 behaelt die bestehende `FullFrontier` byte-/entscheidungsseitig unveraendert und berechnet parallel eine zweite read-only `ThinFirstFrontier`.

Der Thin-first-DP benutzt exakt dieselben:

- finalen HorizontalLayoutFacts,
- getesteten PageCounts,
- AcceptedSystemCapacity,
- MaxSystemMeasures,
- VerticalMaxSystemsPerPage,
- Segmentpartitionen,
- Slotbindung.

Nur die rein diagnostische Tie-Break-Reihenfolge des zweiten Gegenfaktums lautet:

1. Systeme,
2. Singletons,
3. Forced-Over-Capacity-Singletons,
4. `legacyThin`,
5. System-Count-Balance,
6. Page-Slot-Balance,
7. Slack.

Damit wird keine Produktionspolicy vorweggenommen. Es werden lediglich zwei Enden eines realen Trade-offs sichtbar: die bisherige Balance-first-Projektion und ein Thin-first-Gegenfaktum.

## thinLB

Jeder Thin-first-Candidate meldet zusaetzlich `thinLB`, die rein kombinatorische Untergrenze fuer nichtterminale Seiten unter drei Systemen bei gegebener Gesamt-Systemzahl und Seitenzahl.

Diese Untergrenze ignoriert bewusst die konkrete Slot-/Demand-Geometrie. Sie ist daher kein Freigabekriterium, sondern nur eine Kontrolle:

- `legacyThin == thinLB`: der Thin-first-Candidate erreicht die kombinatorische Untergrenze;
- `legacyThin > thinLB`: konkrete horizontale/vertikale Constraints verhindern die theoretische Untergrenze oder der lokale DP benoetigt noch breitere Pareto-Evidence.

## Fail-closed

Die Menge der feasible PageCounts muss fuer Balance-first und Thin-first exakt identisch sein. Driftet sie, bricht der Shadow ab. Beide Pfade bleiben Writer-frei und materialisieren nichts.

## Unveraendert

- `PolicyWrite nein`,
- keine produktive PageCount-Auswahl,
- keine Promotion in den HorizontalLayoutPlan,
- keine Aenderung an GlobalSystemReflow-/JointPageSystem-Cutover,
- keine Gate-Abschwaechung,
- keine Width-Autoritaet,
- keine Repair-/Recovery-Schicht,
- kein Etueden-/Taktwissen.
