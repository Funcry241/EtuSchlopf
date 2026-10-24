# r996 InventoryReferenceEvidenceClosure

## Parent und Klasse

r996 baut direkt auf dem real ausgefuehrten und dunkelgruenen r995 `NotationVoiceBoundS0TransformationContractAudit` auf. r995 bleibt bytegenau erhalten; der Take ist `OutcomeProjection` und aendert ausschliesslich den optionalen `--inventory`-/Assistant-Export-Pfad, dessen versionierte Vertraege und die dafuer gebundenen Launcher-/Trust-Anchor-Metadaten. Musikpipeline, MusicXML/PDF, Writer-/Repair-/Selection-/Release-Authority, CMS-1 und die r995-NotationVoice-Auditdatei bleiben unveraendert.

## XLSX-3

XLSX-2 beseitigte den frueheren Excel-Reparaturfehler, entfernte dabei aber auch die sichtbaren Filter-Buttons. XLSX-3 schreibt deshalb genau einen `<autoFilter>` in `xl/tables/table1.xml`, mit exakt demselben `ref` wie die echte Tabelle. `sheet1.xml` darf weiterhin keinen eigenen AutoFilter besitzen. Der native Readback verlangt genau einen Table-Filter, identischen Bereich, eine echte Tabelle, genau ein Blatt `Inventory` und die eingefrorene Kopfzeile. Der bestehende Kodex-Test `table_contract_is_single_sheet_filterable_and_frozen` behaelt seinen Namen und damit seine Capability-Identitaet.

## ESI-3

ESI-3 trennt Lexiktreffer weiter von beweisbaren Referenzen und schliesst zwei konkrete Owner-Luecken:

- statische `include!("...")`-Fragmente erben alle beweisbaren einschliessenden Function-Owner; Mehrfach-Includes werden nicht first-wins auf einen Owner reduziert,
- `macro_rules!`-Body-Referenzen erben beweisbare Invocation-Owner,
- `self.method()` darf auf das eindeutige Methodensymbol desselben `impl` aufgeloest werden; andere Dot-Member-Calls bleiben ohne Receiver-Proof mehrdeutig,
- nicht syntaktisch plausible Simple-Name-Treffer bleiben `Lexical Hits` und erzeugen keine Reference-/Reachability-Evidenz,
- qualifizierte Function-Values koennen als `Value Binding` Reachability tragen.

Zusatzlich bindet ESI-3 neben Signatur- und Body-Fingerprint einen namensneutralen `signature_shape_fingerprint` fuer reine Duplikat-/Replacement-Evidenz. JSON wechselt auf `schema_version=3`.

## ARCH-4

ARCH-4 behaelt A/B/C/D/OFFEN fail-closed, macht aber die bisher unsichtbare A/B-Beweispipeline explizit.

Fuer A werden pro privater produktiv unerreichbarer Kandidatenkomponente `A Kandidat`, Component-ID, Closure-Status und konkrete Blocker exportiert. Blocker sind u.a. ownerlose Quelle, produktive/test/tool/UNKNOWN-Abhaengigkeit oder eine andere unerreichbare Komponente. Die A-Schwelle wird nicht gelockert.

Fuer B werden identische Signature-Shape-/Body-Gruppen als Kandidatenevidenz ausgewiesen. Eine B-Klassifikation entsteht weiterhin nur bei einem explizit historisch benannten (`legacy_`, `obsolete_`, `deprecated_`, `shadow_`) Symbol mit genau einem produktiv erreichbaren, nicht Test/Tool gebundenen, fingerprint-identischen Gegenstueck. Governance-/Build-Gate-/Audit-Paare bleiben mit `INDEPENDENCE_NOT_PROVEN` blockiert, damit fail-independent Duplikation nicht als Redundanz behauptet wird.

Die Excel-Tabelle erhaelt dafuer eigene filterbare A-/B-Diagnosespalten; der Kennzahlenblock weist Kandidaten, Closure-/Blocker- und Fingerprint-Gruppen separat aus.

## Assistant Export

Bei `--inventory` akzeptiert der Assistant Export nur die exakte Kombination `ESI-3 / ARCH-4 / CMS-1 / XLSX-3`. Einzelprojekt- und Vollkorpus-Semantik, atomare Publikation, Stale-Cleanup und Aufnahme von JSON/XLSX in den Assistant Export bleiben bestehen.

## Tests und Acceptance

Zu r995 mit 1.297 Strict-Tests kommen sechs neue generische Inventory-Regressions-Tests hinzu: static-include ownership, macro-body ownership, sichere `self.method()`-Aufloesung, lexical-only member field, A-Blockerdiagnostik und fingerprint-identischer Legacy-B-Beweis. Erwarteter Strict-Testbestand: **1.303**.

Dieser Erstellungscontainer besitzt keinen Rust-/Cargo-Toolchain. Autoritative Acceptance bleibt `./run.cmd --take --all --inventory`: Build/Clippy gruen, 1.303 Strict-Tests, 10/10 Vollkorpus, erfolgreicher JSON/XLSX-3-Readback, sichtbare Excel-Filterbuttons ohne Reparaturmeldung und gebundener Assistant Export.
