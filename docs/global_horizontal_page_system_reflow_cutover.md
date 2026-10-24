# GlobalHorizontalPageSystemReflowCutover

## Zweck

Take 13 promoviert den in v0.10.1834 r01 real DARK GREEN bestaetigten gemeinsamen Page+System-Shadow in den autoritativen finalen HorizontalLayoutPlan.

Der bestaetigte Shadow fuer Etuede 6:

- 44 strukturelle Slots
- exakt 6 Seiten bleiben erhalten
- 21 -> 19 Systeme
- 2 -> 0 Singletons
- 8 Systembreak-Aenderungen: 3 hinzu / 5 entfernt
- 4 Pagebreak-Aenderungen: 2 hinzu / 2 entfernt
- Page-Slots 7..8 -> 6..9
- Systeme pro Seite 3..4 -> 3..4
- maximale Systemlaenge 3 -> 3
- AcceptedSystemCapacity 504
- MaxShadowSystemLoad 491
- ForcedOverCapacitySingletons 0
- PolicyWrite nein

## Authority-Schnitt

Der Joint-Planner bleibt writerlos. Die Promotion findet im bestehenden finalen Horizontal-Planning-Owner statt:

```text
FinalNotationFacts
      |
      v
GlobalSystemReflow
      |
      v
Joint Page+System Candidate
      |
      v
fail-closed Promotion
      |
      v
ONE final HorizontalLayoutPlan
      |
      v
ONE bestehender Materializer
```

Die Dimension `measure/@width` wird nicht veraendert.

## Fail-closed Bedingungen

Die Promotion wird abgewiesen, wenn:

1. Candidate-/Audit-Slots nicht exakt zum autoritativen Plan passen.
2. Die Seitenzahl nicht exakt erhalten bleibt.
3. Candidate-Systemzahl oder Candidate-Singletons gegenueber dem autoritativen Plan steigen.
4. `ForcedOverCapacitySingletons != 0`.
5. `MaxShadowSystemMeasures > MaxAuthoritativeSystemMeasures`.
6. `MaxShadowSystemLoad > AcceptedSystemCapacity`.
7. Page-Slot-Groessen den aus dem produktiven Plan abgeleiteten +/-1-Korridor verlassen.
8. Systeme je Seite den bereits akzeptierten Min/Max-Korridor verlassen.
9. `new-page` und `new-system` denselben strukturellen Slot belegen.
10. Die tatsaechlichen hinzugefuegten/entfernten Page- oder Systembreaks nicht exakt der Shadow-Evidence entsprechen.
11. Die Promotion Width-Entscheidungen veraendert.

## Provenienz

Page- und Systemdimension des promovierten Plans tragen `HorizontalLayoutDecisionOwner::GlobalPageSystemReflow`. Width-Direktive und Width-Owner werden aus dem bereits autoritativen finalen Plan unveraendert uebernommen.

Der Planner erzeugt kein MusicXML und ruft keine StructuralWriteAuthority auf. Es entsteht kein zweiter Materializer.

## Nicht Teil dieses Takes

- keine Width-Demotion
- keine Aenderung der Seitenzahl
- keine Etueden-/Takt-Sonderlogik
- keine Repair-/Recovery-Schicht
- keine Renderer-Aenderung
- keine Security-Ausnahme
