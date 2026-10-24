# START HERE – EtuSchlopf Architecture Essence Bootstrap

Diese Datei zuerst lesen, wenn ein neuer Chat, Thread oder Bearbeiter ohne verlaessliche Projektgeschichte mit EtuSchlopf beginnt.

Zuerst den kompakten Projektstatus in [050_aktueller_stand.md](050_aktueller_stand.md)
lesen; danach die folgende Repository-/Contract-Lesereihenfolge einhalten.
Der Kurzstand ist advisory only und ersetzt weder aktuelle Laufbelege noch Gates.

## Zweck

EtuSchlopf muss seine fundamentale Architekturkompetenz aus dem aktuellen Repository rekonstruieren koennen. Chat-Erinnerung, menschliche Erinnerung und Uebergabetexte duerfen die Arbeit beschleunigen, besitzen aber keine normative Autoritaet.

## Kompakte Korpus-Uebergabe

Fuer Korpus-Assistant-Exporte ab r1279 beschreibt `docs/corpus_assistant_export_bounded_catalog_closure_r1279.md` die kurze Lesereihenfolge, TSV-Kataloggrenze und Belegverweise. Der Handoff ist eine Projektion zur Exporterzeugung; der endgueltige Accepted-Status kommt weiterhin aus dem hashgebundenen Lifecycle-/Acceptance-Beleg.

## Cold-Start-Reihenfolge

1. Bei einem Assistant-Export zuerst `assistant_context.json` und den darin gebundenen eingebetteten Source-Snapshot bestimmen. Ein vollstaendiger Source-Snapshot ist Architektur-/PhysicalStart-Evidence, aber nicht automatisch ein take-kompatibler Paketbaum; der aktuelle Managed-Tree-Vertrag entscheidet separat ueber den eingehenden Take-Inhalt.
2. Im aktuellen Repository `docs/assistant_memory/architecture_essence_bootstrap_v0.json` lesen.
3. Danach `docs/assistant_memory/020_architecture_essence_v0.md` lesen.
4. Fuer native `--inventory`-Artefakte zuerst `docs/assistant_memory/source_inventory_contract_v3.json` (ESI-3), `docs/assistant_memory/architecture_classification_contract_v4.json` (ARCH-4) und `docs/assistant_memory/inventory_xlsx_contract_v3.json` (XLSX-3) lesen. Diese Vertraege besitzen die aktuelle native deterministische Repository-/Architektur-/XLSX-Projektion; sie erzeugen keine Musik-, Writer-, Repair-, Selection- oder Release-Authority.
5. Fuer historische/assistant-seitige Architektur-Inventur-Aufgaben `docs/assistant_memory/architecture_inventory_contract_v1.json` lesen. AIC-1 bleibt advisory-only und wird durch ESI-3/ARCH-4 nicht rueckwirkend umgedeutet. Seine alten A/B/C/N/OFFEN-Werte sind weder mit ARCH-1/2/3/4 vergleichbar. ESI-1/2 und ARCH-1/2/3 sowie XLSX-1/2 bleiben als historische versionierte Vertraege erhalten.
6. Danach bei Bedarf `docs/assistant_memory/architecture_inventory_assistant_fast_path_v1.json` lesen. AIF-1 bleibt ein Effizienzprofil fuer den historischen AIC-1-Assistant-Schritt.
7. Fuer die konkrete Aufgabe die zustaendigen kanonischen Contracts, Authorities und Gates aus dem aktuellen Source-Baum bestimmen. Bei neuer Selektor-/Fehlerklassenlogik E09/E15/E24 vor dem Candidate lesen: Fehlerklasse kausal und witness-blind definieren, Korpusverteilung erst danach messen, Repair-Scope getrennt ableiten. Bei neuen oder geaenderten Tests zusaetzlich E32 anwenden: der reale Korpus ist Evaluation, normative Tests muessen corpusunabhaengig und synthetisch bleiben.
8. Frische Laufartefakte fuer den aktuellen Runtime-Zustand verwenden; alte Resultate oder Uebergaben nicht als aktuellen Zustand behandeln.
9. Fallhistorie nur bei Bedarf aus `docs/assistant_memory/010_known_failure_patterns.md` und `docs/assistant_external_memory.md` hinzunehmen.
10. Historische Bauauftraege nur bei Bedarf lesen: `docs/assistant_memory/040_stimmen_custody_plan.md` (alter Custody-Zensus), `docs/assistant_memory/041_bauauftrag_row26_gesture_relocation.md` (Row 26 umgesetzt) und `docs/assistant_memory/042_bauauftrag_abschluss_unforced_pdf.md` (teilweise umgesetzt, Restthemen offen). Den aktuellen Auftrag und Pausenstatus aus `docs/assistant_memory/050_aktueller_stand.md` bestimmen; die alten Reihenfolgen sind keine Fortsetzungsanweisung. Fuer Enharmonik bleibt `docs/enharmonic_occurrence_identity_contract_v1.md` zustaendig.

## Autoritaetsgrenze

Die Essenz ist **advisory only**.

Sie darf:

- bekannte Hochrisikomuster frueh sichtbar machen,
- bevorzugte Loesungsraeume nennen,
- Pre-Patch-Fragen bereitstellen,
- die Suche nach fachlichem Owner, Capability, Authority und realer Call-Site beschleunigen.

Sie darf nicht:

- MusicMutation, Writer-, Repair-, Release-, Proof- oder andere produktive Autoritaet erzeugen,
- einen kanonischen Contract oder ein Gate ersetzen,
- einen Gate-Fehler abschwaechen oder umdeuten,
- aus historischer Fallidentitaet eine produktive Entscheidung ableiten.

## Maschinenlesbarer Einstieg

```text
docs/assistant_memory/architecture_essence_bootstrap_v0.json
```

Fuer native Architektur-Inventuren gelten zusaetzlich:

```text
docs/assistant_memory/source_inventory_contract_v3.json
docs/assistant_memory/architecture_classification_contract_v4.json
docs/assistant_memory/inventory_xlsx_contract_v3.json
```

ESI-3/ARCH-4/XLSX-3 gehoeren zum optionalen `--inventory`-Outcome-Export. AIC-1 bleibt daneben **advisory only** und beschreibt weiterhin nur den historischen assistant-seitigen Inventurpfad; EtuSchlopf-Runtime extrahiert oder klassifiziert **fuer AIC-1** keine Funktionsinventur. Die vollstaendige Symbolgrundgesamtheit wird assistant-seitig aus dem gebundenen physischen Source-Snapshot per Rust-Syntax/AST bestimmt; erst danach interpretiert die KI jedes Symbol genau einmal.

AIF-1 ist nur ein Effizienzprofil fuer genau diesen Assistant-Schritt: ein Source-Pass baut Symboluniversum und mechanischen Repository-Index; Wiederverwendung ist ausschliesslich an denselben Source-Snapshot-SHA gebunden. A/B/C/N/OFFEN, Thema, Authority- und Wahrheitsinterpretation werden nicht als Cache-Wahrheit konserviert. Nach finaler Interpretation soll genau ein AIC-1-Workbook gerendert werden.

Dieses JSON dupliziert bewusst **nicht** die vollstaendigen Essenz-Regeln. Es indexiert die eine Textquelle, beschreibt die Cold-Start-Reihenfolge und markiert die Autoritaetsgrenze maschinenlesbar.

## Primaere Essenz

```text
docs/assistant_memory/020_architecture_essence_v0.md
```

Kernsatz:

> Erinnerung kann verloren gehen. Essenz bleibt im Projekt. Essenz lenkt. Authority schreibt. Proof beobachtet. Gate entscheidet. Korpus beweist Breite.

## Fallhistorie

```text
docs/assistant_memory/010_known_failure_patterns.md
docs/assistant_external_memory.md
```

Diese Dateien bewahren konkrete historische Gegenbeispiele und Diagnosemuster. Sie sind nachgeordnet und duerfen nicht als aktuelle Projektwahrheit oder produktive Selektoren verwendet werden.

## Integritaets-Readback

Die technische Aufloesbarkeit dieser Cold-Start-Schicht wird read-only durch

```text
tests/architecture_essence_cold_start_readback.rs
```

gegen

```text
docs/assistant_memory/030_cold_start_readback_contract.md
```

regressiv geprueft. Dieser Readback beweist nur, dass Bootstrap, Essenz, AIC-1 und Discovery-Hinweise im aktuellen Repository technisch aufloesbar bleiben und AIC-1 seine advisory/nonproductive Grenze behaelt. Er bewertet keine KI-Klassifikation und erteilt keinerlei fachliche oder normative Erlaubnis; diese bleibt ausschliesslich bei den aktuellen kanonischen Contracts, Authorities und Gates.


**r1031 Proxy-Identity-Sperre:** Eine neue kausale Fehlerklasse wird zuerst ohne neuen selector-sensitiven Code als `error_class_definition` eingefroren. Klassifikations- oder Repair-Code darf erst in einem spaeteren Take auf diesen unveraenderten Vertrag aus dem akzeptierten Parent verweisen.
