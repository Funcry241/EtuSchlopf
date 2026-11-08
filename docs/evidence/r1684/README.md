# Native Abnahme r1684

Der vollständige native Korpuslauf wurde am 8. November 2026 angenommen. Grundlage ist `assistant_export_latest(20261108-150557).zip`; dessen ZIP-Prüfung und die Vollständigkeit der angekündigten Dateien wurden geprüft.

| Prüfung | Ergebnis |
| --- | --- |
| Canonical verdict | accepted, r1684 |
| Vollständiger Korpus | true, 10/10 Projektbelege |
| Manifest | gültig, hash-bound full corpus |
| FinalExit / BuildExit / ClippyExit | 0 / 0 / 0 |
| Native Tests | 2992 bestanden, 0 fehlgeschlagen, 4 ignoriert, 15 Testprogramme |
| Native Testlaufzeit | 42,51 Sekunden |
| Eingebetteter Quellstand | vollständig, 2070 Dateien |

Die Laufzeitangabe betrifft die Tests und ist keine Messung der gesamten Korpusverarbeitung. Eine allgemeine Beschleunigung wird damit nicht behauptet.

## Bindung an Take und Quellstand

| Artefakt | SHA-256 |
| --- | --- |
| Angenommenes Take-ZIP r1684 | `6a10566ed1dc2db8bb52989c37d432945d4c95dcf525842f472be0a862d9a8bb` |
| Eingebettetes vollständiges Quell-Snapshot-ZIP | `ec869f02d4ba7162b5793643d09248ecfa93f091dbc3af3404c1304d0724b59d` |

Das Take-ZIP und der anschließend exportierte Quell-Snapshot sind unterschiedliche Artefakte. Ihre Prüfsummen dürfen nicht gleichgesetzt werden. Die ZIP-Prüfung des eingebetteten Snapshots war fehlerfrei; sein SHA-256 stimmt mit dem Exportmanifest überein.

[Lifecycle-Handover](take_lifecycle_handover.txt), [originaler Annahmebeleg](native_acceptance.txt), [Annahmemetadaten](take_acceptance_evidence_metadata.txt), [Kandidatenabsicht](take_acceptance_candidate_intent.txt) und [semantischer Effektsnapshot](take_acceptance_semantic_effect_snapshot.tsv) wurden aus dem nativen Export übernommen. Die im Handover gebundenen Bytezahlen und SHA-256-Werte der vier Annahmeartefakte wurden geprüft. Der leere Effektsnapshot gehört zur Dokumentationsrevision und hebt die im Korpuslauf beobachteten Renderänderungen der bestehenden Engine nicht auf.

[Exportmanifest](manifest.txt) und [Laufergebnis](run_outcome_audit.txt) dokumentieren den frischen Korpuslauf. Das Manifest beschreibt auch Dateien des privaten vollständigen Exports; deren Aufzählung bedeutet nicht, dass sie hier veröffentlicht sind.

[Dateiprüfsummen des angenommenen Quell-Snapshots](accepted_source_sha256.tsv) erfassen alle 2070 Dateien, einschließlich `Cargo.lock`. Der GitHub-Baum übernimmt diesen Quellstand mit öffentlichen Dokumentationsanpassungen und erhält vorhandene GitHub-Dateien sowie historische Belege. Er ist kein byteidentisches Take-ZIP. Abweichende öffentliche Einstiegsdateien sind `README.md`, `docs/README.md`, `docs/CONTRIBUTING.md` und `docs/community/issue_templates/01_fehler.md`; `docs/DEVELOPMENT.md` und dieser Belegordner ergänzen den Snapshot.

## Inhalt und Grenzen

r1684 selbst überarbeitet die Mathematik-Dokumentation auf dem angenommenen Laufzeitstand r1683. Rust-Quellen, Tests, Gates und Abhängigkeiten sind gegenüber r1683 unverändert. Die [Mathematik-Seite](../../MATHEMATIK.md) unterscheidet 34 Verfahrensfamilien, produktive Entscheidungen, Audits, historische Modelle und offene Beweisgrenzen.

Der kumulative Stand seit r1659 enthält unter anderem die C3-Evidenz für ursprüngliche Pausenanker und Verbesserungen der Watch-Webansicht. C3 kann damit weitere zulässige Originaldarstellungen übernehmen; systemübergreifende Gruppenabhängigkeiten bleiben als offene Grenze ausgewiesen. Die native Korpusabnahme ist kein allgemeiner Nachweis von Fehlerfreiheit, Lesbarkeit oder Spielbarkeit beliebiger Stücke.

Aus diesem angenommenen Quellstand wurde anschließend die [Windows-Testversion r1684](https://github.com/Funcry241/EtuSchlopf/releases/tag/v0.10.1839-r1684-windows-preview) gebaut. Ihre separaten Paketprüfungen bestätigen den isolierten Start ohne Entwickler-PATH, unveränderte Eingaben und ein frisch gerendertes PDF-Startbeispiel. Das Release enthält die passende EXE, das Quellarchiv, das Startbeispiel, SHA-256-Prüfsummen und die Bindung an Build-Commit und Workflow. Die native Korpusabnahme und die Windows-Paketprüfung bleiben getrennte Belege.
