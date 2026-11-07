# Native Abnahme r1659

Version 0.10.1839, angenommener Take r1659. Quelle ist der Benutzerexport `assistant_export_latest(20261107-092418).zip`. Das Manifest meldet `Mode: corpus_run`, `FullCorpus: true`, `ProjectEvidence: 10/10` und `FinalExit: 0`. Build und Clippy sind erfolgreich. Der native Lauf meldet 2969 bestandene Tests und drei ignorierte Tests.

Der [endgültige Lifecycle-Handover](take_lifecycle_handover.txt) und der [hashgebundene Annahmebeleg](native_acceptance.txt) bestätigen den Take. Die Prüfsummen des Annahmebelegs, der [Metadaten](take_acceptance_evidence_metadata.txt), der [Kandidatenabsicht](take_acceptance_candidate_intent.txt) und des [semantischen Snapshots](take_acceptance_semantic_effect_snapshot.tsv) wurden gegen den Handover geprüft. Der ältere Generierungs-Handoff beschreibt den Take noch als laufend und besitzt keine endgültige Urteilsautorität.

| Artefakt | SHA-256 |
|---|---|
| Angenommenes r1659-Take-Paket | `5c22063c9ddcdda33e1102b8ed07ba978557cfa542dbfb66da0fbec060b8c495` |
| Eingebetteter vollständiger r1659-Quellsnapshot | `3607d145c888c9ee6c6cd5d72f34b7a41c58e09a7b5990d4e04adcd8fc0d2639` |

Die beiden Hashes gehören zu unterschiedlichen Artefakten. [Dateihashes des angenommenen Quellsnapshots](accepted_source_sha256.tsv) ermöglichen den Abgleich. Der GitHub-Baum enthält dessen Laufzeitquellen sowie separat aktualisierte öffentliche Dokumentation und bestehende GitHub-Dateien. Er ist kein bytegleiches Repackaging eines Take-ZIP.

## Vergleich mit r1657

Alle zehn Master- und alle zehn Render-MusicXML-Dateien sind bytegleich. Die zehn PDFs wurden mit demselben Renderer bei 144 dpi verglichen. Alle 39 Seiten haben identische Pixel und Seitenabmessungen. Das beweist die Erhaltung des dargestellten Notenbilds bei dieser Auflösung, keine darüber hinausgehende musikalische Fehlerfreiheit.

| Beobachtung | r1657 | r1659 |
|---|---:|---:|
| Korpuslaufzeit | 9:49 min | 10:19 min |
| Summierte Projektlaufzeiten | 28:17 min | 29:55 min |
| Gleichzeitig verarbeitete Projekte | 3 | 3 |

Die beobachtete Gesamtlaufzeit ist 30 Sekunden länger. Ein Gesamtgeschwindigkeitsgewinn ist nicht belegt. Einzelne Läufe erlauben keine isolierte kausale Zuordnung der Differenz.

r1658 verwendet bereits erfasstes physisches Inventar für die Nachfolgerprüfung. r1659 konsolidiert identische Custody-Census-Klassifikationen, verwendet geliehene Schlüssel und beendet die Prüfung identischer Bytes früh. Multiplizitäten bleiben erhalten. Die lokalen Vorprüfungen meldeten 2187 bestandene Tests für r1658 und 2188 für r1659, jeweils drei ignorierte Tests. Die native Abnahme des kumulativen Stands ist der oben dokumentierte r1659-Lauf.

Die Windows-Testversion ist weiterhin r1602. Eine neue Windows-Binärdatei gehört nicht zu dieser Veröffentlichung.
