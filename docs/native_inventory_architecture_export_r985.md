# r985 NativeInventoryArchitectureExport

r985 baut **direkt auf dem autoritativen Rollback-Stand r982** auf. r983 wurde verworfen und auf r982 zurueckgerollt; r983 ist deshalb weder Parent noch Quelle fuer diesen Take. Der Take ist `OutcomeProjection` und fuehrt keine Musikmutation, keine neue Writer-/Repair-/Selection-/Release-Authority und keine Aenderung an Master-/Render-MusicXML oder PDF ein.

## Ziel

`--inventory` wird als Launcher-Exportmodus fuer den jeweils ausgefuehrten Scope eingefuehrt:

- `run.cmd "Etüde Nr. 6" --inventory` erzeugt eine PROJECT-Inventur fuer genau diesen aktuellen Lauf.
- `run.cmd --all --inventory` erzeugt eine CORPUS-Inventur und bindet CMS-1 ausschliesslich an den aktuellen vollstaendigen 10/10-Corpus-Run. Dabei bleiben die Observation-Session-ID der zehn CMS-1-Snapshots und die eigenstaendige Corpus-Manifest-Run-ID als zwei getrennte Authorities erhalten und werden nicht gleichgesetzt.
- `--inventory` wird vom Launcher konsumiert und nicht an die Musik-Binary weitergereicht.
- Normale Laeufe ohne `--inventory` entfernen stale aktuelle Inventory-Artefakte aus dem Projektroot.

## Native Vertraege

- `ESI-1`: deterministisches Source-/Symbolinventar; Source-Scope wird aus AIC-1 uebernommen, aber nativ gemessen.
- `ARCH-1`: deterministische handlungsorientierte Architekturklassifikation A/B/C/D/OFFEN.
- `CMS-1`: bereits bestehende native Laufmessung; PROJECT direkt, CORPUS nur als aktuelle gebundene 10/10-Aggregation.
- `XLSX-1`: reine Darstellung desselben Inventory-Modells.

AIC-1 bleibt als historischer assistant-seitiger Vertrag unveraendert. Seine alten A/B/C/N/OFFEN-Zahlen sind nicht mit ARCH-1 vergleichbar. Der erste reale gruene r985+-`--inventory`-Lauf ist die neue Zeitreihen-Baseline.

## ARCH-1

- **A – Loeschbar:** nur bei statisch geschlossenem Nachweis innerhalb des vollstaendigen ESI-1-Scopes; unbekannte Voraussetzung => OFFEN.
- **B – Zusammenfuehrbar / redundant:** nur mit konkret gefundenem Gegenstueck/Zielpfad.
- **C – Notwendige Unterstuetzung:** produktiv erreichbarer Support bzw. gebundene Test-/Tool-Infrastruktur.
- **D – Autoritativer Kern:** gebundener Einstieg bzw. explizite Writer-/Materializer-, Selection-, Canonical- oder Governance-Authority-Regel.
- **OFFEN:** Default bei nicht ausreichender Evidenz.

A ist keine automatische Loeschfreigabe; eine Quellentfernung braucht weiterhin einen eigenen EtuSchlopf-Take und alle bestehenden Gates.

## Artefakte und Excel

Der aktuelle Root-Stand besitzt hoechstens das Paar:

```text
EtuSchlopf.inventory.json
EtuSchlopf.inventory.xlsx
```

Die XLSX-Datei besitzt genau **ein** Tabellenblatt `Inventory`. Die echte Excel-Tabelle beginnt in A1, besitzt fuer jede Spalte AutoFilter und friert Zeile 1 ein. Der Vertrags-/Kennzahlenblock liegt rechts auf demselben Blatt. Es werden keine weiteren Worksheets erzeugt.

Neue Dateien werden zuerst als `.partial` geschrieben und read-back-validiert. Erst danach wird das bisherige gueltige Paar ersetzt; `.previous`/`.partial` werden bereinigt.

Wenn `--inventory` angefordert wurde, muessen JSON und XLSX vollstaendig erzeugt und in den Assistant Export uebernommen werden. Andernfalls darf der Inventory-Lauf nicht gruen enden.

## Assistant Export

Bei einem Inventory-Lauf werden beide Inventory-Artefakte inklusive SHA-256, Run-ID, Scope und Contract-Versionen in `assistant_export_latest.zip` gebunden. Ohne `--inventory` werden keine stale Inventory-Dateien als aktuelle Evidenz exportiert.

## Acceptance

Dieser Chat kann keinen Rust-/Cargo-Gate ausfuehren. Der Take bleibt daher bis zum realen Windows-Lauf **unbuilt**. Autoritative Acceptance ist der reale `run.cmd --take --all --inventory`-Lauf mit Build/Clippy/Tests/10/10-Corpus sowie gebundenem ESI-1/ARCH-1/CMS-1 und erfolgreichem XLSX-Readback. Erwarteter Strict-Testbestand nach den drei neuen Inventory-Unit-Tests: **1.280**.
