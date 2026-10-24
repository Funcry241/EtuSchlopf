# GlobalHorizontalPageSystemReflowShadowPlanner

## Zweck

Der produktive GlobalSystemReflow von v0.10.1833 beseitigt bei unveraenderten Page-Grenzen den groessten Teil der historischen Break-Accretion, laesst im beobachteten Etuede-6-Lauf aber zwei Singleton-Systeme zurueck. Dieser Take prueft read-only, ob eine gemeinsame globale Optimierung von Page- und Systemgrenzen diese Restschuld beseitigen kann, ohne Seitenzahl, Width-Policy oder Writerarchitektur anzutasten.

## Autoritative Eingaben

- finaler fingerprint-gebundener Notationszustand,
- identity-freie `HorizontalLayoutFactSnapshot`-Slots,
- der bereits produktive finale HorizontalLayoutPlan nach `GlobalSystemReflow`.

Aus dem autoritativen Plan werden abgeleitet:

- exakte Seitenanzahl,
- kleinste/groesste aktuelle Seite in strukturellen Slots,
- kleinste/groesste aktuelle Systemzahl je Seite,
- maximale bereits akzeptierte Systemlaenge,
- maximale bereits akzeptierte Systemlast.

Die typische Systemkapazitaet bleibt dieselbe datengetriebene Median-Kapazitaet des GlobalSystemReflow. Die Shadow-Kapazitaet ist das Maximum aus dieser typischen Kapazitaet und der bereits akzeptierten maximalen produktiven Systemlast.

## Gemeinsame DP-Partition

Die Seitenzahl ist hart unveraendert. Eine Shadow-Seite darf nur zwischen `authoritative_min_page_slots - 1` und `authoritative_max_page_slots + 1` Slots enthalten. Ihre Systemzahl muss innerhalb der bereits produktiv akzeptierten Min/Max-Systemzahl pro Seite bleiben. Kein Shadow-System darf mehr Slots enthalten als das produktiv bereits akzeptierte Maximum.

Fuer jedes moegliche Page-Segment wird eine kapazitaetsgebundene Systempartition berechnet. Darauf baut eine zweite DP ueber alle Seiten auf. Die Zielfunktion ist lexikographisch:

1. weniger Systeme,
2. weniger Singletons,
3. weniger Forced-Over-Capacity-Singletons,
4. gleichmaessigere Systemzahl pro Seite,
5. gleichmaessigere Seitenlaenge,
6. geringere quadratische Restkapazitaet,
7. deterministische strukturelle Tie-Breaks.

## Safety

Der Take ist read-only. Es gibt keinen StructuralWriteAuthority-Aufruf und keine Mutation des produktiven Plans. Fail-closed wird bereits im Shadow verlangt:

- gleiche Slotanzahl,
- exakt gleiche Seitenanzahl,
- Seitengroesse nur im abgeleiteten Korridor,
- Systemzahl pro Seite nur im produktiv akzeptierten Korridor,
- keine laengeren Systeme als produktiv bereits akzeptiert,
- keine Systemlast ueber der gebundenen Kapazitaet,
- Page- und Systembreak nie gleichzeitig am selben Slot.

## Erwartete Evidence fuer Etuede 6

Lokale Projektion auf v0.10.1833 r02:

- 44 Slots,
- 6 Seiten,
- Systeme 21 -> 19,
- Singletons 2 -> 0,
- SystemBreakChanges 8,
- PageBreakChanges 4,
- Page-Slots 7..8 -> 6..9,
- Systeme/Seite 3..4 -> 3..4,
- MaxSystemMeasures 3 -> 3,
- AcceptedSystemCapacity 504,
- MaxShadowSystemLoad 491,
- ForcedOverCapacitySingletons 0,
- `PolicyWrite nein`.

Erst ein real DARK GREEN Windows-Lauf darf diese Projektion als Grundlage fuer einen spaeteren produktiven Page+System-Cutover autorisieren.
