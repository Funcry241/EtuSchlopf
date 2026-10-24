# r766 – ReportSanitizerOrderRetirementAndExternalIngressAudit

## Zweck

r766 ist ein verhaltensneutraler `RefactorOnly`-Block auf der real angenommenen Green Base r765. Er verbindet zwei Seiten desselben Sanitizer-Problems: sichere physische Retirement-Arbeit und die repository-residente Grenze, die eine weitergehende pauschale Loeschung gerade verbietet.

## Physisch pensioniert

In `sanitize_report_hygiene_polish()` werden exakt sechs Replacement-Zweige entfernt:

- `area_h1`
- `area_h2`
- `area_s1v1`
- `area_s2v1`
- `area_s2v2`
- `slot start_slot_`

Sie sind nicht lediglich im aktuellen Korpus unbeobachtet, sondern durch die vorgeschaltete Sanitizer-Reihenfolge algebraisch unerreichbar: `sanitize_report_hardcoded_measure_artifacts()` normalisiert `area_...` bereits vor `sanitize_report_hygiene_polish()`, und innerhalb des Hygiene-Sanitizers wird `start_slot_` vor dem spaeteren `slot start_slot_`-Muster ersetzt.

## Warum nicht mehr geloescht wird

Die restlichen No-Internal-Producer-Replacements werden bewusst nicht pensioniert. MusicXML `movement-title` bzw. `work-title` wird als `Score.title` eingelesen, gelangt als `CheckReport.score_title` in die Berichtspraembel `Partitur: ...` und durchlaeuft danach die globalen Report-Sanitizer. Damit koennen externe Partiturmetadaten dieselben Tokens einspeisen, auch wenn kein interner Producer existiert.

Das neue Integrationstest-Contract `tests/report_sanitizer_external_ingress_contract.rs` bindet genau diesen Quellfluss read-only und prueft zusaetzlich, dass die sechs pensionierten Branches nicht wiederkehren, waehrend die vorgeschalteten Normalisierungen bestehen bleiben.

## Beweisgrenze

`RetirementProof = ProducerClosure + IngressDomainClosure + SanitizerOrderClosure`.

Hier ist die `SanitizerOrderClosure` fuer sechs Branches positiv bewiesen. Fuer die uebrigen No-Producer-Zweige ist die `IngressDomainClosure` explizit **nicht** geschlossen, solange externe Metadaten und interne Berichtssprache dieselbe globale Rewrite-Domaene teilen.

## Nicht geaendert

Keine Musikmutation, kein Writer, kein Reader, kein ProofReentryReader, keine Auswahl-, Voice-, Tie-, Duration-, Beam-, Layout- oder Release-Semantik. Keine historische Dokumentation wird modernisiert. Keine Formatting-Insel wird in diesen fachlichen Sanitizer-Block hineingemischt.
