# r1075 ValidationBreadthProofPlacementClosure

## Zweck

r1075 folgt direkt auf den kanonisch angenommenen r1074 `FailureAssistantHandoverLedgerIoFixtureClosure` und schliesst den letzten Prozesswiderspruch zwischen der lebenden Launcher-Semantik und E24 der Architektur-Essenz.

Die Runtime besitzt bereits **keine allgemeine Regel**, die jeden aktuellen Same-Version-Take automatisch zu `--all` hochstuft. Die verbliebene Essenzformulierung erklärte den Vollkorpus jedoch pauschal zum notwendigen Abschluss- und Breitenbeweis jeder Änderung. Das führte zu unnötiger diagnostischer Breite selbst bei reinen Kodex-, Handover-, Dokumentations- oder Test-Fixture-Cuts.

## Geschlossener Vertrag

Die Beweisbreite folgt ab r1075 der tatsächlich invalidierten Evidenzdimension:

1. Architektur- und Change-Control-Gates gelten immer entsprechend der Änderungsklasse.
2. Die kleinste gezielte Regression beweist zuerst die konkret betroffene Invariante.
3. Breitere Tests werden nur dort gefordert, wo deren Evidenzgrundlage durch den Diff invalidiert wurde.
4. Vollkorpus ist Pflicht, wenn corpusweite Generalisierung/Verteilung selbst Teil des Beweises ist, ein aktueller expliziter Contract ihn fordert oder gezielte Evidenz breitere Auswirkungen zeigt.
5. Ein Vollkorpus ist **kein zeremonieller Pflichtschritt allein deshalb, weil ein Take existiert**.

Produktive Änderungen an Musik-, Writer-, Repair-, Selection-, Layout- oder corpusweiter Entscheidungssemantik können weiterhin einen Vollkorpus als Breitenbeweis benötigen. Gleiches gilt für neue/änderte Fehlerklassen, Multiplicity-Invarianz und Korpusverteilungen. Der Korpus bleibt Regressionsevidenz und niemals Schreibautorität.

## Runtime-/Regression-Closure

`src/bin/etuschlopf_launcher.rs` ändert keine produktive Entscheidung. Zwei rein kodexseitige Präzisierungen werden eingefroren:

- Der Kommentar an den historischen versionsgebundenen Corpus-Contracts stellt klar, dass diese keine generische Post-Take-Policy sind.
- Ein neuer Test beweist für `VERSION -> VERSION`, dass `version_bound_post_take_decision(...)` unabhängig von vorhandenem Export/Vorgängerevidenz `NoAction` liefert. Ein aktueller Same-Version-Take wird damit nicht automatisch zum Vollkorpus promoviert.
- Der bestehende Promotion-Helper-Test bleibt mit seiner bisherigen Identität und Semantik unverändert; damit entsteht keine künstliche Negative-Check-Renaming-Schuld.

Die alten versionsgebundenen Sonderverträge für 0.10.1265→0.10.1266 und 0.10.1286 bleiben bytefunktional erhalten.

## Grenzen

- ChangeClass: `KodexOnly`
- keine MusicMutation
- keine Writer-/Repair-/Selection-/Layout-Authority
- keine Acceptance-/Rejection-/Recovery-Änderung
- keine Änderung der Strict-Clippy-, Build- oder All-Target-Testpflicht
- keine neue Dependency-Engine, Registry oder Validation-Matrix
- kein pauschaler Vollkorpus-Abschalter; explizite aktuelle Corpus-Contracts bleiben autoritativ
