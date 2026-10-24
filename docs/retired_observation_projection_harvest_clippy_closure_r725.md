# r725 RetiredObservationProjectionHarvestClippyClosure

## Ausgang

r724 `RetiredObservationProjectionHarvestWave1` wurde nach erfolgreicher Vorpruefung im Reexec von `#![deny(dead_code)]` gestoppt. Der entfernte KeySignature-LateSpan-Reportconsumer hatte einen zusammenhaengenden historischen Map-/Profil-/Barline-Auditrest als unerreichbaren Code freigelegt. r724 ist deshalb kein gruener Parent. r725 wird unabhaengig auf r723 konstruiert und umfasst den beabsichtigten r724-Harvest plus exakt die vom Strict-Clippy bewiesene Dead-Code-Closure.

## Pensioniert

1. `KeySignatureEnharmonicSpan` late-span observation/report projection inklusive eigener Consolidation-Audit-Struktur, Transportfeld und ausschliesslich diese pensionierte Projektion pruefender Tests.
2. `ConfusingTiedTailAccentSuppress` retired report-wrapper inklusive Audit-Transport, Reportsektion und Builder/Formatter.
3. Die nach Entfernung des LateSpan-Consumers nun unerreichbare `KeySignatureEnharmonicMap`-Profilinsel (`KeySignatureEnharmonicMapRow`, `KeySignatureEnharmonicMapAudit`, `CandidateKeyProfile`, `KEY_PROFILES`, Map-/Score-Helfer und `max_measure_number`).
4. Die ebenfalls unerreichbaren alten KeySignature-change/barline Beobachtungshelfer (`key_signature_change_mark_count`, `key_signature_change_boundary_measures`, `key_signature_change_double_barline_count`, `measure_has_right_barline_style`).

## Explizit erhalten

- aktive scoreweite tonale Entscheidung und `KeySignatureEnharmonicSpelling`-Materialisierung;
- `TonalSegmentation` als read-only Evidenz;
- `carried_key_signature_fifths`, `key_signature_measure_fifths` und die aktiven Tonal-Segmentation-Verbraucher;
- `unique_confusing_tied_tail_accent_regression_candidates` und die zugehoerigen Negativtests;
- TieTail-Verbraucher im committed-state/Noether/AccentReserve-Pfad;
- alle StructuralWriteAuthority-, Recovery-, Change-Control-, Semantic-Capability- und Build-Gates.

## Beweisgrenze

Die zusaetzliche r725-Closure folgt direkt aus dem Strict-Clippy-Gegenbefund: alle 13 gemeldeten Symbole waren nach dem beabsichtigten Harvest nicht mehr konstruiert bzw. benutzt. Vor der Entfernung wurde repositoryweit geprueft, dass diese Namen ausserhalb ihrer eigenen Definitionsinsel nicht referenziert werden. Aehnlich benannte, weiterhin aktive Helpers werden nicht entfernt.

## Invarianten

- keine produktive Writer-Reachability wird hinzugefuegt oder erweitert;
- kein bestehender produktiver Writer wird entfernt;
- kein musikalischer Selektor, Threshold oder Mutation-Pfad wird veraendert;
- kein lebender committed-state Regression-Detector wird entfernt;
- die aktive tonale Authority und ihre Tests bleiben;
- der oeffentliche Report wird nur um bereits pensionierte Grabsteinsektionen kleiner;
- `#![deny(dead_code)]` bleibt unveraendert harte Compile-/Clippy-Grenze.

Dieser Take ist `RefactorOnly` und wird unabhaengig von r724 auf r723 deklariert.
