# r775 – ReportSanitizerClosedWorldLegacyVocabularyRetirement

## Authority

r775 basiert ausschließlich auf dem real `dark_green`/`released` angenommenen r774
`ExternalMetadataOpaqueTransportClosure`.

r766 hatte die Retirement-Gleichung

```text
RetirementProof = ProducerClosure + IngressDomainClosure + SanitizerOrderClosure
```

bewusst nur teilweise geschlossen. r774 hat die fehlende Ingress-Grenze geschlossen:
`ExternalMetadata` wird opaque transportiert; `InternalReportVocabulary` bleibt Sanitizer-owned.

## Closed-World-Schnitt

Der erste Literal-Scan war absichtlich nur ein Kandidatenfinder. Der anschließende
Format-Producer-Audit hat gezeigt, warum Literal-Abwesenheit alleine nicht genügt:
`src/wav_truth_articulation.rs` erzeugt `Review {}` dynamisch. Deshalb bleibt die komplette
`Review`-Familie konservativ erhalten. Ebenso bleiben die historischen `v0.10.14...`-
Hygiene-Rewrites erhalten, solange aktuelle Reportpfade dynamische `v{...}`-Formatierung
besitzen.

Pensioniert werden auf dem exakten r774-SourceSnapshot nur feste Rewrites, die

1. im gesamten aktuellen `src/**/*.rs` außerhalb `src/report_sanitizer.rs` keine exakte
   Producer-Occurrence besitzen,
2. nicht zur dynamisch belegten `Review`-Familie gehören,
3. im Hygiene-Sanitizer nicht zur dynamischen Versionsfamilie gehören und
4. nicht von einer vorgeschalteten Sanitizer-Stufe erzeugt werden können.

Ergebnis:

- 50 Behind-Bars-Rewrites pensioniert,
- 15 Hygiene-Rewrites pensioniert,
- insgesamt **65 Normalisierungsrechte** entfernt.

Das Maschineninventar der pensionierten Zweige liegt in
`docs/report_sanitizer_closed_world_legacy_vocabulary_retirement_r775.tsv`.

## Bewusst erhalten

Der Behind-Bars-Tisch behält 43 Rewrites: alle 13 mit aktuellem exaktem Source-Producer plus
die konservativ erhaltene `Review`-Familie. Insbesondere `Review 0` bis `Review 9` bleiben,
weil `wav_truth_articulation.rs` `Review {}` produktiv formatieren kann.

Im Hygiene-Sanitizer bleiben 14 feste Rewrites: neun historische `v0.10.14...`-Rewrites wegen
der noch vorhandenen dynamischen `v{...}`-Producerfamilie sowie fuenf weiterhin erreichbare oder dynamisch erzeugte
Rewrites (`start_slot_`, `same-analysis-area`, `measures `, `Measures `, `safe_atomic_jump`).
`safe_atomic_jump` ist im realen r774-Resulttext vorhanden und wird aus `safe_atomic_{reason}`
plus einem dynamisch aufgebauten `jump...`-Reason zusammengesetzt; auch hier reicht Literal-
Abwesenheit im Source nicht als Retirement-Beweis.

Die bestehenden Legacy-Marker-Gates bleiben vollständig bestehen. r775 schwächt daher keinen
fail-closed Detector: unerwartet wieder auftauchendes Legacy-Vokabular wird nicht durch einen
neuen Fallback versteckt.

## Contract

`tests/report_sanitizer_external_ingress_contract.rs` bindet den post-r774-Zustand ohne neue
Testfunktion:

- `ExternalMetadata` bleibt ausdrücklich opaque,
- der Behind-Bars-Rewrite-Tisch enthält exakt 43 konservativ belegte Einträge,
- der Hygiene-Sanitizer enthält exakt 14 feste `.replace(...)`-Rewrites,
- die dynamisch belegten `Review`- und Versionsfamilien bleiben vorhanden,
- die bereits in r766 pensionierten sechs Sanitizer-Order-Zweige bleiben abwesend.

## Nicht geändert

Keine MusicXML-, PDF-, Musik-, Voice-, Tie-, Duration-, Beam-, Layout-, Writer-, Repair-,
Recovery-, Auswahl- oder Release-Semantik. Kein neuer Sanitizer, kein neuer Owner, keine neue
Writer-Reachability. r775 entfernt ausschließlich Closed-World-belegte Report-
Normalisierungsrechte nach der akzeptierten r774-Origin-Grenze.
