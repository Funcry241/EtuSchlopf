# GlobalHorizontalPageCountPreferenceFrontierShadow

## Anlass

Der v0.10.1838-Vollkorpus bestaetigt die Kernarchitektur `Final Facts -> One Plan -> One Materializer -> RendererAuto`, zeigt aber eine verbliebene Legacy-Kopplung im produktiven Reflow-Cutover. Sieben von zehn Projekten brechen vor der finalen horizontalen Materialisierung ab. Die reproduzierten Blocker sind:

1. `MaxShadowSystemMeasures > MaxLegacySystemMeasures`.
2. `NonterminalPagesBelowThreeSystems != 0`.

Beide Regeln waren als konservative Migrations-Safety sinnvoll. Korpusweit sind sie aber nicht als dauerhafte globale Wahrheit geeignet: Das erste Gate konserviert das alte Zweitakt-Systemmaximum; das zweite koppelt vertikale Seitenfuellung an eine feste Systemanzahl, obwohl neue Systeme mehrere Takte tragen koennen. Gleichzeitig darf die zweite Schutzidee nicht einfach entfernt werden, weil eine unveraenderte alte Seitenzahl nach Systemkompression reale duenne Seiten erzeugen kann.

## Scope

Dieser Take ist strikt read-only. Er veraendert keinen produktiven Plan und lockert kein bestehendes Cutover-Gate.

Unmittelbar nach dem globalen System-Reflow-Candidate, aber **vor** dessen produktiver Promotion, wird eine PageCount-Feasibility-Frontier berechnet. Damit ist die Evidence auch dann logisch vor dem heutigen Fail-Closed-Punkt verfuegbar.

## Feasibility-Basis

Die bisherige Auto-Seitenzahl wird als `PreferredPages` gelesen, nicht als neue harte Wahrheit des Shadows.

Die niedrigste zu testende Seitenzahl wird datengetrieben aus

- der vom globalen System-Reflow gefundenen Systemanzahl und
- der hoechsten bereits akzeptierten Systemanzahl pro Seite

abgeleitet. Dadurch werden unmoeglich niedrige Seitenzahlen gar nicht erst getestet.

Fuer jede getestete Seitenzahl gelten:

- exakte strukturelle Slotabdeckung,
- keine leere Seite,
- `ForcedOverCapacitySingletons == 0`,
- Systemlast <= bereits akzeptierter Global-Reflow-Capacity,
- maximale Systemlaenge <= der im selben finalen Global-Reflow-Candidate tatsaechlich beobachteten maximalen Systemlaenge,
- Systeme pro Seite <= der bereits akzeptierten vertikalen Obergrenze.

Bewusst **nicht** uebernommen werden:

- das Legacy-Maximum der Takte pro System,
- ein Minimum von drei Systemen pro Nicht-Endseite,
- der alte Min/Max-Systeme-pro-Seite-Korridor als Untergrenze,
- der +/-1-Page-Slot-Korridor,
- eine neue fest erfundene Zielseitenzahl.

## Ausgabe

Die Konsole erhaelt genau eine neue Evidence-Zeile:

`HorizontalLayoutGlobalPageCountPreferenceShadow`

Sie meldet unter anderem:

- `PreferredPages`
- `TestedPages`
- `FeasiblePages`
- `LowestFeasiblePages`
- `ClosestLowerFeasiblePages`
- `PreferredPagesFeasible`
- `VerticalMaxSystemsPerPage`
- `MaxSystemMeasures`
- `AcceptedSystemCapacity`
- Metriken des naechsten niedrigeren feasible Kandidaten
- `PolicyWrite nein`

## Nicht Teil dieses Takes

- keine produktive PageCount-Promotion
- keine Aenderung der bestehenden GlobalSystemReflow-Safety-Gates
- keine Aenderung am Joint Page/System Cutover
- keine Width-Policy
- keine neue StructuralWriteAuthority
- keine Repair-/Recovery-Schicht
- kein Fallback auf Legacy- oder Shadow-Geometrie
- keine Etueden-/Takt-Sonderlogik

## Zweck fuer den naechsten Thread

Der naechste Thread soll aus dieser Frontier und einem erneuten Korpuslauf ableiten, ob und in welchem Bereich die alte fixe Seitenzahl zu `preferred page count + admissible range` demotiert werden kann. Erst danach ist ein produktiver PageCount-/Safety-Cutover gerechtfertigt.
