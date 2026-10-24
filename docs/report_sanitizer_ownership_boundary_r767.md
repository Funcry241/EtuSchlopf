# r767 – ReportSanitizerOwnershipBoundary

## Klasse

`RefactorOnly`

## Parent

`r766 ReportSanitizerOrderRetirementAndExternalIngressAudit` – real `dark_green`, `released`, `FinalExit 0`.

## Problem

r766 hat repository-resident bewiesen, dass externe MusicXML-Partiturmetadaten ueber `Score.title` und `CheckReport.score_title` in dieselbe globale Rewrite-Domaene gelangen wie interne Legacy-Berichtssprache. Solange Sanitizer-Implementierung und ExternalMetadata-Ingress weder nach Owner noch nach Herkunft getrennt sind, waere ein semantischer Cut zu `ExternalMetadata = opaque` architektonisch unsauber.

## Cut

r767 veraendert keine Rewrite-Regel.

1. Die bestehenden Report-Sanitizer und ihre rein reportbezogenen Helper ziehen byteinhaltlich aus
   `src/pipeline_shards/audio_salience_discrete_action_conflict_gate_tail.rs`
   nach
   `src/report_sanitizer.rs`.
2. `src/pipeline_shards/final_report_projection_owner.rs` ruft dieselben Funktionen nun explizit ueber diesen dedizierten Report-Owner auf.
3. `src/final_report.rs` fuehrt die Herkunftsklassen
   `InternalReportVocabulary` und `ExternalMetadata` ein.
4. Die `Partitur: ...`-Zeile wird beim Parsen als `ExternalMetadata` klassifiziert; Render-Reihenfolge und Bytes bleiben unveraendert.
5. Der existierende Whole-Document-Rewrite bleibt absichtlich global. Damit ist r767 noch **kein** `ExternalMetadataOpaqueTransportClosure`.

## Harte Nichtaussagen

r767 behauptet nicht, dass ExternalMetadata bereits unsanitisiert transportiert wird. r767 aendert weder Replacement-Mengen noch Replacement-Reihenfolge noch Release-Entscheidung. Es fuegt keinen Writer, keinen Reader, keine MusicMutation, keine neue Repair-/Recovery-Authority und keine etuedenspezifische Produktlogik hinzu.

## Nachweisziel

Der reale Windows-Lauf muss `dark_green`, `released`, `FinalExit 0` erreichen. Master-MusicXML, Render-MusicXML und PDF sollen gegen r766 byteidentisch bleiben; der Result-Report soll bis auf revisions-/laufbezogene Metadaten inhaltlich unveraendert sein. Die Unit-/Contract-Tests pinnen zusaetzlich:

- Sanitizer-Implementierung nicht mehr im gemischten Audio-Pipeline-Shard,
- ExternalMetadata-Origin im Final-Report-Modell,
- bestehender Legacy-Whole-Document-Rewrite erreicht ExternalMetadata weiterhin,
- r766-Retirement der sechs algebraisch unerreichbaren Replacements bleibt geschlossen.

## Spaetere Grenze

Erst nach erfolgreicher Annahme von r767 darf ein separater Semantik-Take den Rewrite von `ExternalMetadata` abschalten. Dieser spaetere Take muss dann den Output-Diff ausdruecklich als beabsichtigte Reportsemantik deklarieren.
