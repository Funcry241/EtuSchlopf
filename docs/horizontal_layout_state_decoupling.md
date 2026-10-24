# HorizontalLayoutStateDecoupling

## Zweck

Take 5 der horizontalen Layoutmigration trennt die letzten beiden produktiven
Legacy-Entscheidungen von bereits materialisierter horizontaler Geometrie.

Bis v0.10.1826 war die Decision-Ownership zwar zentral im
`HorizontalLayoutLegacyPlanBuilder`, aber zwei Policy-Schritte lasen weiterhin
den geschriebenen XML-Zustand:

- `PianisticReadability` verwendete `new-system="yes"` aus dem Measure-XML.
- `FinalShortSystemBalance` rekonstruierte Systemstarts und den finalen
  Page-Break erneut aus materialisierten `<print>`-Direktiven.

Damit blieb Geometrie trotz zentralem Decision-Owner eine produktive
Zwischenwahrheit.

## Neuer Vertrag

`PianisticReadability` bezieht den bereits von den vorherigen Legacy-Schichten
geplanten Systemzustand nun aus dem strukturellen Slot des gemeinsamen
PlanBuilders. Die weiterhin benoetigte Clef-Oberflaeche bleibt eine separate
Notation-/Legacy-State-Beobachtung.

`FinalShortSystemBalance` bestimmt die Systemfolge nun ausschliesslich aus den
strukturellen Slots des PlanBuilders:

- erster Slot ist Systemstart,
- weitere Systemstarts entstehen aus geplantem `new-system` oder `new-page`,
- Systemlaengen werden aus Slotabstaenden berechnet,
- der Page-Status des letzten Slots stammt aus dem Plan.

Erst nachdem diese identity-freie Entscheidung getroffen wurde, wird fuer den
noch unveraenderten Legacy-Writer die exakte `new-system="yes"`-Oberflaeche des
letzten strukturellen Slots lokalisiert.

## Bewusst unveraendert

Dieser Take zentralisiert die Writer noch nicht. Die vorhandenen Width-,
System-, Page- und FinalShort-Writer laufen weiterhin an denselben Stellen und
werden nach jedem Schritt gegen den PlanBuilder gelesen. Damit sollen MusicXML
und PDF gegen v0.10.1826 unveraendert bleiben.

Unveraendert bleiben insbesondere:

- `choose_auto_pianist_layout`
- `compute_page_breaks`
- `compute_system_breaks`
- das Zweitakt-Grundraster
- alle Dense-/Visual-/Pianistic-Scores, Thresholds und Caps
- alle Width-Ziele
- `select_balanced_visual_system_breaks`
- `select_balanced_visual_page_breaks`
- die Semantik von `FinalShortSystemBalance`
- alle StructuralWriteAuthority-Writer
- PDF-Quellenauswahl und MuseScore

## Architekturwirkung

Nach diesem Take gibt es fuer die horizontale Legacy-Policy keinen Grund mehr,
einen bereits geschriebenen Break als Entscheidungseingang zu behandeln:

```
Facts + Legacy policy
        |
        v
HorizontalLayoutLegacyPlanBuilder
        |
        +--> PianisticReadability liest geplanten Systemzustand
        |
        +--> FinalShortSystemBalance liest geplante Systemfolge
        |
        v
bestehende Writer (noch unveraendert)
```

Damit ist der folgende One-Materializer-Take erstmals moeglich, ohne einen
Shadow-XML-Puffer als zweite Wahrheit einzufuehren.

## Paritaetsaudit

Der produktive Take-4-Gate beweist unmittelbar vor `PianisticReadability`
`PlanBuilder == materialisierte horizontale Direktiven`; der neue geplante
Systemzustand ist dort daher derselbe Boolean wie der bisherige XML-Readback.

Fuer `FinalShortSystemBalance` wurde die alte nummernbasierte und die neue
slotbasierte Entscheidung auf allen zehn gelieferten Korpus-Masterdateien
verglichen. Ergebnis: 10/10 identische Merge-Entscheidung. Die bekannten
Merges der Etueden 2, 4, 6, 8 und 10 bleiben erhalten; die anderen fuenf
Stuecke bleiben unveraendert ohne Merge.

## Security

Keine Projekt-, Etueden- oder absolute Taktidentitaet wird zur
Layoutentscheidung hinzugefuegt. `FinalShortSystemBalance` arbeitet fuer seine
Entscheidung nur mit strukturellen Slot-Indizes. Der konkrete XML-Range wird
erst nach der Entscheidung als Writer-Locator ermittelt.

Keine neue Repair-/Recovery-Schicht, keine neue Writer-Authority und keine
Security-Abschwaechung.
