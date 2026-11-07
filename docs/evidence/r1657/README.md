# Native Abnahme r1657

Version 0.10.1839, angenommener Take r1657, vollständiger Korpuslauf mit zehn Etüden. Quelle ist der Benutzerexport `assistant_export_latest(20261107-015927).zip`. Das Manifest meldet `Mode: corpus_run`, `FullCorpus: true`, `ProjectEvidence: 10/10` und `FinalExit: 0`. Der native Lauf meldet 2968 bestandene Tests und drei ignorierte Tests sowie erfolgreiche Build- und Clippy-Prüfungen.

Der angenommene Take hat SHA-256 `c39b53cfcd4082409576182a089434b5c46cf418b42f030385c25dc6d62ef862`. Der eingebettete Quellsnapshot hat SHA-256 `ed1ebb467b09078024da1f197979c780b5235937afcc24208bccca5f42fe75e3`. Das sind unterschiedliche Artefakte.

Die Originalbelege liegen als [Annahme](native_acceptance.txt), [Lifecycle-Handover](take_lifecycle_handover.txt), [Metadaten](take_acceptance_evidence_metadata.txt), [Kandidatenabsicht](take_acceptance_candidate_intent.txt) und [semantischer Snapshot](take_acceptance_semantic_effect_snapshot.tsv) bei. Diese Kopien erzeugen keine neue Abnahmeautorität.

[Dateihashes des angenommenen Quellsnapshots](accepted_source_sha256.tsv) erlauben den Abgleich. Die GitHub-Veröffentlichung übernimmt dessen Laufzeitquellen. Öffentliche Einstiegsdokumente werden separat aktualisiert und vorhandene GitHub-Dateien erhalten. Daher ist der GitHub-Baum kein bytegleiches Repackaging des Take-ZIP.

Die Mathematik-Seite unterscheidet die unverändernden Optimierungen bis r1655 von der bewussten Schlüsseländerung in r1657 und den noch nicht nativ abgenommenen Kandidaten r1658/r1659. Eine neue Windows-Binärdatei gehört nicht zu dieser Veröffentlichung.
