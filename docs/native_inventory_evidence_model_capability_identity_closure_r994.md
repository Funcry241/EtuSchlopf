# r994 NativeInventoryEvidenceModelCapabilityIdentityClosure

r994 baut direkt auf dem real akzeptierten und dunkelgruenen r992 `NotationVoiceOccurrenceAttackMismatchDecomposition` auf. Der Take ist `OutcomeProjection` und aendert ausschliesslich den optionalen nativen `--inventory`-/Assistant-Export-Pfad sowie dessen Contracts. Die r991/r992-Notation-Voice-Audit-/Projection-Aenderungen bleiben erhalten; Musikpipeline, MusicXML/PDF, Writer-/Repair-/Selection-/Release-Authority, Schwellenwerte und CMS-1 werden nicht veraendert.

Der Kandidat r993 mit derselben fachlichen ESI-2/ARCH-3/XLSX-2-Aenderung wurde vor Installation vom Semantic-Capability-Diff abgelehnt, weil der bestehende negative Kodex-Test `table_contract_is_single_sheet_filterable_and_frozen` umbenannt worden war. r994 ist ein unabhaengiger Kandidat direkt auf r992, behaelt diese historische Capability-Identitaet exakt bei und staerkt nur deren Assertions fuer XLSX-2; r993 ist rejected Failure Evidence und kein Parent.

## Anlass

Der erste native Inventory-Zyklus hat drei getrennte Messprobleme sichtbar gemacht:

1. XLSX-1 schrieb denselben Tabellenbereich als Worksheet-AutoFilter und als Table-AutoFilter. Excel reparierte die Datei und meldete die Entfernung von AutoFilter und Tabelle aus `xl/tables/table1.xml`.
2. ESI-1 bestimmte Test-Kontext ueber ein lokales Rueckwaerts-Textfenster. Dadurch konnten `#[cfg(test)] mod ...`-Nachfahren als Produktion erscheinen und ein Attribut eines vorherigen Items auf ein folgendes Item uebergreifen.
3. Referenzen/Reachability basierten zu stark auf globalen Simple-Name-Treffern und `pub*` als Root. Dadurch waren Referenzzahlen bei Namensduplikaten irrefuehrend, Function-Value-Bindings fehlten, dot-method calls konnten ohne Receiver-Proof zu stark gebunden werden, und restricted visibility konnte zu produktiv wirken.

## ESI-2

ESI-2 ist ein ausdruecklicher Methodenbruch gegenueber ESI-1. Source-Scope und semantische Symbolidentitaet `relative_source_path + qualified_symbol` bleiben erhalten, aber die abgeleiteten Kontexte/Referenzen/Reachability sind eine neue Messserie.

- Item-Attribute werden token-strukturell gebunden. `#[cfg(test)]` auf Modul/Trait/Impl/Funktion wird nur in dessen syntaktische Nachfahren vererbt. Sibling-Leakage ist ausgeschlossen.
- `cfg(any(...))` gilt nur dann als test-only, wenn jede Alternative Test voraussetzt; `cfg(all(...))` wenn mindestens eine notwendige Teilbedingung Test voraussetzt. Nicht beweisbare Negationen bleiben konservativ nicht-test-only.
- Sensitive cfg/cfg_attr/test/bench/dead_code/export/no_mangle/link_section/used/proc-macro-Evidenz blockiert Removal-Proof.
- String-/Char-Literale werden nicht als Source-Namen geparst, gehen aber als Inhalts-Hash in Signatur-/Body-Fingerprints ein. Whitespace/Kommentare beeinflussen den Token-Fingerprint nicht.
- `Lexical Hits` bleiben reine Diagnose. `Resolved References`, `Ambiguous References`, `Incoming Calls` und `Value Bindings` werden getrennt ausgewiesen.
- Ein dot-member call wird ohne Receiver-Proof selbst bei global eindeutigem Methodennamen nicht als sichere Kante behauptet.
- Function-Value-/Callback-Bindungen propagieren Reachability.
- `pub(crate)`, `pub(super)` und `pub(in ...)` sind keine automatischen produktiven Roots. `pub` ist nur auf beweisbarer direkter `lib.rs`-Exportoberflaeche ein harter Root; sonst bleibt es ein uncertain public root und propagiert UNKNOWN statt YES.
- ESI-2 JSON ist `schema_version=2` und exportiert zusaetzlich Module-Scope, geteilte Referenzmetriken, Sensitive-Declaration-Status sowie Signature-/Body-Fingerprint.

## ARCH-3

ARCH-3 ist wegen der neuen ESI-2-Evidenz ein ausdruecklicher Methodenbruch gegenueber ARCH-2. Die Buchstaben behalten ihre handlungsorientierte Bedeutung: A loeschbar, B zusammenfuehrbar/redundant, C notwendige Unterstuetzung, D autoritativer Kern, OFFEN fail-closed.

A bleibt streng: nur `productive_reachability=NO`, private freie Funktion, keine sensitive/mehrdeutige Evidenz und eine geschlossene Removal-Komponente duerfen A werden. Mehrdeutige Referenzen blockieren A. B bleibt ohne konkretes Gegenstueck 0/unerzwungen. D bleibt symbolbezogen wie ARCH-2; r994 behauptet **keine** neue Gleichsetzung von `D.WRITER_AUTHORITY_SYMBOL` mit der separaten ProductiveWritePath-Generierung.

Der XLSX-Kennzahlenblock zeigt jetzt zusaetzlich OFFEN-/Reachability-, Root-, Namensmehrdeutigkeits-, Reference-Resolution- und D-Unterklassen. Zielwerte fuer A/B/C/D/OFFEN sind keine Acceptance-Vorgabe; autoritativ sind ausschliesslich die Werte des realen r994-Laufs.

## XLSX-2

XLSX-2 behaelt genau ein Blatt `Inventory`, eine echte Tabelle `EtuSchlopfInventory` ab A1 und die eingefrorene erste Zeile. Der Filter ist ausschliesslich Eigenschaft der echten Tabelle: weder Worksheet noch Table-Part schreiben einen zusaetzlichen `<autoFilter>`-Block. Die Worksheet-Relationship bindet exakt `/xl/tables/table1.xml`; Table-Readback verlangt `headerRowCount=1`, `totalsRowCount=0`, `totalsRowShown=0`, den exakten Tabellenbereich und die exakte Spaltenzahl. Die fruehere Excel-Reparaturform wird vom nativen Readback abgewiesen.

## Assistant Export

Wenn `--inventory` erzeugt wurde, akzeptiert der Assistant Export nur die exakte Kombination `ESI-2 / ARCH-3 / XLSX-2` (CMS-1 bleibt unveraendert) und transportiert die drei Contract-Versionen zusammen mit JSON/XLSX. Ohne `--inventory` bleibt das bisherige Verhalten und Stale-Cleanup unveraendert.

## Gates und Acceptance

r992 hat 1.289 Strict-Tests. Gegenueber dessen fuenf Inventory-Unit-Tests besitzt r994 elf: struktureller Test-Scope, cfg-any/all-Fail-Closed-Verhalten, Value-Binding-Reachability, receiver-unbewiesene Member-Ambiguitaet, Literal-Fingerprint, restricted-visibility Roots sowie die bestehenden A-/D-/XLSX-Regressionsfaelle. Erwarteter Strict-Testbestand ist daher **1.295**.

Dieser Erstellungscontainer besitzt keinen Rust-/Cargo-Toolchain. Der Take bleibt bis zum realen Windows-Lauf unbuilt. Autoritative Acceptance ist `run.cmd --take --all --inventory` mit Build, Clippy ohne Warning-Debt, 1.295 Strict-Tests, 10/10 Vollkorpus, erfolgreichem JSON/XLSX-2-Readback und gebundenem Assistant Export.
