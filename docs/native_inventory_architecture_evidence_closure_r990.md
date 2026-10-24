# r990 NativeInventoryArchitectureEvidenceClosure

r990 baut direkt auf dem real akzeptierten und dunkelgruenen r989 `NotationVoiceCausalLanePressureAuditCompileClosure` auf. Der Take ist `OutcomeProjection` und aendert ausschliesslich die native Architekturklassifikation des optionalen `--inventory`-Pfads. Die r989-Notation-Voice-Auditdateien bleiben byteidentisch; Musikpipeline, MusicXML/PDF, Writer-/Repair-/Selection-/Release-Authority sowie CMS-1 und ESI-1 bleiben unveraendert.

Anlass ist die erste native ARCH-1-Baseline aus r987: 885 Symbole wurden als D klassifiziert, weil Modul-/Dateipfade wie `structural_write_authority` oder `build_gate` bereits als Authority-/Gate-Evidenz genuegten. Gleichzeitig blieben private, produktiv unerreichbare, intern referenzierte Funktionsinseln fail-closed OFFEN, obwohl eine ganze abgeschlossene Komponente statisch als Entfernungskandidat beweisbar sein kann.

## ARCH-2

ARCH-2 ist ein ausdruecklicher Methodenbruch gegenueber ARCH-1. r987 bleibt historische ARCH-1-Baseline; der erste gruene ARCH-2-Inventory-Lauf wird die ARCH-2-Baseline. ESI-1, CMS-1 und XLSX-1 bleiben in ihrer Semantik unveraendert.

### D: symbolbezogene Authority

D wird nicht mehr aus einem Authority-/Gate-Wort im Modul- oder Dateipfad abgeleitet. Ein produktiv erreichbares Symbol wird D nur noch, wenn der **eigene einfache Symbolname** eine versionierte Authority-Regel erfuellt: Einstiegspunkt `main`, explizite Authorize-/Authorized-/Writer-Authority-/Owner-Semantik, Selection-/Canonical-Authority-/Owner-Semantik oder im Governance-Kontext ein eigener Validate-/Verify-Name. Hilfsfunktionen innerhalb `structural_write_authority` und `build_gate` fallen dadurch regulaer nach C zurueck, sofern sie produktiv erreichbar sind.

### A: geschlossene unerreichbare Komponenten

A bleibt fail-closed und wird erweitert, ohne `productive_reachability=NO` pauschal mit Loeschbarkeit gleichzusetzen. Fuer private freie Funktionen mit `productive_reachability=NO` wird die lexikalische Herkunft jeder Referenz bestimmt. Kandidaten werden zu undirektionalen Removal-Komponenten verbunden. Eine Komponente ist nur dann A, wenn fuer **jedes** Mitglied jede eingehende lexikalische Referenz aus einem Mitglied derselben Komponente stammt; ownerlose/globale Referenzen, Referenzen aus anderen Symbolen, sensible cfg/export/test/bench/proc-macro/used-Attribute oder andere offene Voraussetzungen verhindern A.

Damit koennen gegenseitig oder selbst referenzierende tote Funktionsinseln erstmals als A erkannt werden, ohne einzelne `reference_count > 0`-Symbole ungesichert fuer loeschbar zu erklaeren.

## Erwartete Wirkung

Die r987-Voranalyse ergab qualitativ eine deutliche Verschiebung von path-only-D nach C und erstmals eine kleine Zahl belastbarer A-Komponenten. Da r989 gegenueber r987 zusaetzliche read-only Audit-Symbole enthaelt, sind alte konkrete Erwartungszahlen **keine** Acceptance-Vorgabe fuer r990. Autoritativ sind ausschliesslich die Werte des realen r990 `--all --inventory`-Laufs; sie duerfen nicht durch Zielwertanpassung erzwungen werden. B bleibt konservativ und wird nur mit konkretem Gegenstueck vergeben.

## Acceptance

Autoritative Acceptance ist `run.cmd --take --all --inventory` mit Build, Clippy, Strict-Tests, 10/10-Corpus, erfolgreichem Inventory-Readback und gebundenem Assistant Export. Der akzeptierte r989-Stand hat 1.283 Strict-Tests; die zwei ARCH-2-Regressions-Tests in `inventory_export.rs` erhoehen den erwarteten Bestand auf **1.285**. Dieser Erstellungscontainer besitzt keinen Rust-/Cargo-Toolchain; der Take bleibt bis zum realen Windows-Lauf unbuilt.
