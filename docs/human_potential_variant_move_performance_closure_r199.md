# HumanPotential Variant Move Performance Closure r199

## Ausgangslage

r198 wurde vor dem Kopieren durch das Take-Change-Control abgewiesen, weil sein Change-Manifest versehentlich noch den r197-Diff deklarierte. r199 ist die reine Packaging-/Manifest-Recovery desselben fachlichen Ownership-Schnitts auf dem weiterhin autoritativ gruenen r197-Stand.

Im globalen HumanPotential-Pfad wurde ein bereits besessener `ArrangementVariant` unmittelbar nach `evaluate_arrangement_variant` noch einmal mit `clone()` dupliziert, obwohl das Original danach keinen Leser mehr hatte. `ArrangementVariant` enthaelt eine vollstaendige `layout_xml` sowie mehrere Berichts- und Statistikstrukturen. Der globale Pfad kann bis zu 24 Kandidatenversuche pruefen.

Ausserdem wurde `diagnostic_arrangement.stats` geklont, obwohl `diagnostic_arrangement` nach dieser Entnahme nicht weiter verwendet wird.

## Aenderung

- `diagnostic_arrangement.stats.clone()` -> Besitzuebergang von `stats`
- `human_variant.clone()` -> direkter Besitzuebergang in `decision_variant`

## Sicherheits- und Verhaltensgrenze

Es werden keine Bedingungen, Scores, Kandidaten, Schwellen, Reihenfolgen, Guard-Pruefungen, Hint-Repair-Schritte oder Writer-Autoritaeten geaendert. Die Aenderung betrifft ausschliesslich Rust-Ownership und reduziert unnoetige tiefe Kopien.

Die betroffene Produktdatei ist kein architektur-gepinnter Security-Pfad. r199 deklariert den tatsaechlichen r197->r199-Diff vollstaendig; der fehlerhafte r198-Manifestzustand wird nicht uebernommen.
