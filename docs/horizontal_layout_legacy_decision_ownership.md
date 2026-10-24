# HorizontalLayoutLegacyDecisionOwnership

## Zweck

Take 4 der horizontalen Layoutmigration gibt den **bestehenden** horizontalen
Legacy-Entscheidungen erstmals einen gemeinsamen typisierten Owner, ohne die
Materialisierung bereits zu zentralisieren und ohne die Layoutpolicy zu ändern.

Der Take folgt auf:

1. `HorizontalLayoutFactExtraction` – Rohfakten statt impliziter Policy-Evidence.
2. `FinalNotationLayoutSnapshotClosure` – finaler Notationszustand vor Renderer gebunden.
3. `HorizontalLayoutPlanContractProjection` – immutable read-only Zielvertrag der
   final materialisierten horizontalen Direktiven.

## Problem der heutigen Legacy-Reihenfolge

Die aktuelle Policy ist sequenziell zustandsabhängig:

- die Basispaginierung materialisiert Page-/System-Breaks,
- `DenseRegisterNotation` kann Width erhöhen,
- `VisualLayoutPressure` kann Width/System/Page ergänzen,
- `PianisticReadability` liest bereits materialisierte `new-system`-Geometrie und
  kann Width erhöhen,
- `FinalShortSystemBalance` kann einen finalen Systembreak neutralisieren.

Darum wäre ein sofortiger vollständig vorgelagerter Planner in diesem Take keine
verhaltensneutrale Refaktorierung: Er müsste die alte Zustandsfolge simulieren
oder bereits Policy verändern.

## Neuer Vertrag

Take 4 führt deshalb einen zentralen `HorizontalLayoutLegacyPlanBuilder` ein.

### 1. LegacySeed

`apply_pianist_layout()` erzeugt **vor** `insert_layout_prints()` einen
identity-freien strukturellen Seed. Pro Measure-Slot enthält er nur:

- bereits vorhandenes explizites `width`,
- final erwartetes Basis-`new-system`,
- final erwartetes Basis-`new-page`.

Der Seed speichert keine Etüden-, Projekt- oder Taktidentität.

### 2. Decision Ownership vor jedem vorhandenen Writer

Die bestehenden Legacy-Schichten melden ihre Entscheidung zuerst an denselben
PlanBuilder und rufen danach unverändert ihre vorhandene
`StructuralWriteAuthority` auf:

- `DenseRegisterNotation`
- `VisualLayoutPressure`
- `PianisticReadability`
- `FinalShortSystemBalance`

Nach jedem Schritt wird exakt geprüft, dass Decision-Plan und materialisiertes
XML dieselben horizontalen Direktiven tragen.

### 3. Immutable Close

Nach der letzten Legacy-Layoutentscheidung wird der Builder zu
`HorizontalLayoutLegacyPlan` geschlossen. Dieser Plan enthält pro strukturellem
Slot:

- `explicit_width`
- `starts_new_system`
- `starts_new_page`
- typisierte Provenienz des jeweils besitzenden Legacy-Entscheiders.

Der Plan ist read-only und besitzt **kein Schreibrecht**.

### 4. Finaler Readback

Vor MuseScore muss der geschlossene Decision-Plan:

- vollständige Provenienz besitzen,
- exakt mit der snapshot-gebundenen finalen Notationsquelle übereinstimmen,
- exakt mit dem in Take 3 eingeführten
  `HorizontalLayoutPlanContractProjection` übereinstimmen.

Der bestehende Post-Renderer-Readback aus Take 2/3 bleibt unverändert.

## Bewusst unverändert

Dieser Take ändert nicht:

- `choose_auto_pianist_layout()`
- `compute_page_breaks()`
- `compute_system_breaks()`
- die Zweitakt-Grundregel
- sämtliche Density-/Pressure-/Readability-Scores
- Thresholds oder Caps
- `select_balanced_visual_system_breaks()`
- `select_balanced_visual_page_breaks()`
- `FinalShortSystemBalance`-Policy
- Width-Ziele
- Page-/System-/Width-Writer
- PDF-Quellenauswahl
- MuseScore-Aufruf
- musikalische Mutation
- Passorder der vorhandenen Legacy-Entscheider.

Insbesondere ist dieser Take **noch nicht** der One-Last-Writer-Take.

## Identity-Regel

Der neue Plan speichert keine Takt- oder Etüdenidentität. Bestehende numerische
Measure-Locators bleiben ausschließlich als lokale Legacy-Lokalisierung für
bereits vorhandene Selektoren/Writer erhalten; sie werden nicht Bestandteil des
Planvertrags oder seiner Provenienz.

## Sicherheitsziel

Der Take führt keinen neuen Repair-/Recovery-Owner und keinen neuen
Layoutwriter ein. Er schwächt keine Security-Invariante ab. Falls ein bislang
unbekannter horizontaler Writer nach PlanClose existiert, muss die finale
Plan-vs-Snapshot-Prüfung fail-closed abbrechen.

## Abnahme

Ein-Befehl-Abnahme:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Für diesen `RefactorOnly`-Take gilt als gewünschte Verhaltensparität:

- Build/Strict/Launcher/Program/Renderer GREEN,
- Master-/Render-MusicXML gegenüber v0.10.1825 nur Softwareversionsdelta,
- keine neue Musikmutation,
- keine neue Layoutdirektive,
- finaler Decision-Plan == Take-3-Planvertrag == snapshot-gebundene Quelle.

Erst der folgende Take darf die verteilte Materialisierung auf genau einen
Materializer umstellen.
