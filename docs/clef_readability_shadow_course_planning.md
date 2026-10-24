# ClefReadabilityShadowCoursePlanning

## Scope

v0.10.1709 ist Take 2 des vierteiligen Clef-Readability-Blocks. Der Take beantwortet ausschließlich die Frage **„Wie bewerten wir alternative Clef-Kurse?“**. Die neue Entscheidung wird scoreweit als Shadow/Audit berechnet, aber nicht materialisiert.

Unveraendert bleiben insbesondere:

- die produktive lokale `>3`-Hilfslinienentscheidung in `notation.rs`,
- die A4-basierte fruehe Clef-Erzeugung,
- `sustained_lower_staff_measure_clef_preference(...)`,
- der bisherige produktive Rustig-Course-Plan und seine Kosten,
- Hysterese und Lower-Clef-Surface-Materialisierung,
- alle Structural Write Authorities,
- Master-/Render-MusicXML und PDF.

## Kein zweiter Planner

Der neue Modus lebt im vorhandenen `rustig_clef_planning.rs` und benutzt dieselbe scoreweite F4/G2-Surface-Beobachtung, dieselben Pitch-/Oktavbeobachtungen und denselben Frontier-/Dynamic-Programming-Ansatz wie der bestehende Rustig-Kursplanner. Es entsteht kein zweiter Writer und keine zweite Materialisierungsgrenze.

Der produktive Planner bleibt unangetastet. Die neue Frontier ist ausschliesslich eine Shadow-Bewertung des vorhandenen scoreweiten Clef-Surface-Kandidatenraums.

## Readability-Kosten

Take 2 fuehrt bewusst nur wenige benannte Dimensionen ein:

### LedgerPressure

Die bestehende graduelle Kurve bleibt erhalten:

```text
ledger_pressure = ledgers^2 + max(ledgers - 3, 0) * 8
```

Hilfslinien sind damit Kosten, keine Verbote.

### ClefTransitionCost

Der Shadow-Modus uebernimmt zunaechst die bestehende Rustig-Einheitskosten fuer einen sichtbaren Clef-Wechsel. Diese Kosten bleiben separat im Audit sichtbar.

### ShortExcursionCost

Ein Rueckweg zum vorherigen Clef nach hoechstens zwei sichtbaren Lower-Staff-Attacks wird als **short excursion** beobachtet.

Die Referenzkosten entsprechen genau dem Ledger-Druck eines einzelnen Tons mit vier Hilfslinien:

```text
4^2 + (4 - 3) * 8 = 24
```

Das bedeutet ausdruecklich nicht:

```text
vier Hilfslinien = Grenze
```

sondern:

```text
ein isoliertes Vier-Hilfslinien-Ereignis darf lesbarer sein
als ein sehr kurzer Clef-Ausflug mit Rueckkehr.
```

Mehrere oder laengere Hochlagen koennen weiterhin klar fuer den Clef-Wechsel gewinnen.

### ContourDisruption

Take 2 fuehrt noch kein unabhaengiges Konturgewicht ein. Im Audit wird eine kurze Clef-Insel als begruendete erste Projektion einer Konturunterbrechung ausgewiesen. Damit bleibt die Policy klein und nachvollziehbar.

## Shadow Evidence

Der finale Result-Report erhaelt die Section:

```text
CLEF-READABILITY-SHADOW-COURSE-PLANNING
```

Sie zeigt pro beobachtetem Part unter anderem:

```text
CurrentCourse
ReadabilityCourse
LedgerPressure current -> shadow
ClefTransitions current -> shadow
RedundantClefs current -> shadow
ShortExcursions current -> shadow
ContourDisruption current -> shadow
Decision KEEP / CHANGE
Reason ...
```

Bei Abweichungen werden die im Shadow-Kurs entfallenden vorhandenen Clef-Surfaces dynamisch inventarisiert. Projekt-/Taktidentitaeten sind keine Produktivselektoren und beeinflussen weder Kandidaten noch Entscheidung.

## Golden Cases

### A - kurzer hoher Ausflug

```text
F4
-> kurzer G2-Ausflug
-> einzelnes/kurzes hohes Ereignis
-> sofortige Rueckkehr F4
```

Der Shadow darf `F4` bevorzugen, wenn der zusaetzliche Ledger-Druck unter den expliziten Short-Excursion-Kosten bleibt.

Der reale menschliche Kontrollfall ist Etuede 6, Takt 32 unten. Erwartet ist im **Shadow** sinngemaess:

```text
CURRENT:     F4 -> G2 -> F4
READABILITY: F4
```

Die produktiven Master-/Render-Dateien muessen in Take 2 trotzdem weiterhin `F4 -> G2 -> F4` enthalten.

### B - laengere hohe Passage

Mehrere zusammenhaengende hohe Attacks erzeugen genuegend LedgerPressure, sodass der G2-Kurs weiterhin gewinnen darf.

### C - laengere Rueckkehr in Tieflage

Nach einer echten Hochlage muss der Rueckweg nach F4 erhalten bleiben, wenn die folgende Tieflage im G2-Clef deutlich teurer waere.

### D - stabiler sinnvoller Kurs

Ein bereits guter Kurs bleibt unveraendert. Kein Clef-Wechsel wird nur erzeugt, um den Shadow-Modus sichtbar zu machen.

## Writer- und Musikneutralitaet

Die neue API besitzt nur:

- Shadow-Cost-Breakdown,
- Shadow-Frontier-State,
- Shadow-Course-Audit,
- Result-Projection.

Sie besitzt keine XML-Ersetzung, keinen Dateisink, keine MaterializationPlan-Ausgabe und keine Write Authorization.

Der Report deklariert explizit:

```text
WriteAllowed nein
MusicXmlMutation nein
ClefMaterialization nein
WriterAuthorityChanged nein
```

## Take-2-Abnahme

Der Vollkorpus muss weiterhin gruen sein. Gegenueber dem akzeptierten v0.10.1708-Ergebnis gilt:

```text
Master-MusicXML  identisch nach Versionsnormalisierung
Render-MusicXML  identisch nach Versionsnormalisierung
PDF              pixelidentisch
Writer-Evidence  unveraendert
```

Neu und absichtlich unterschiedlich sind nur Version-/Inventory-Metadaten und die Shadow-Audit-Section in den Result-Dateien.

## Stop-Gate vor Take 3

Kein Cutover, wenn die Shadow-Evidence:

- grosse unerwartete Clef-Kaskaden vorschlaegt,
- stabile Passagen ohne nachvollziehbaren Gewinn veraendert,
- viele nicht erklaerbare Abweichungen erzeugt,
- oder der Golden Case nur durch projekt-/taktbezogene Sonderlogik erreicht wird.

Take 3 beginnt erst nach fachlicher Pruefung des gesamten Shadow-Inventars.
