# r1279: schlankere Korpus-Uebergabe und Export-Kellerputz

Parent: accepted r1278 `SscK2GovernanceRevisionKodex`. Klasse: `OutcomeProjection`. Genau ein Take; keine SSC-, Musik-, Owner- oder Enforcement-Aenderung.

## Aktueller Exportvertrag

Der Korpus-Assistant-Export enthaelt den vollstaendigen aktuellen Source-Snapshot, Korpusmanifest und Bericht, alle manifestgebundenen Projektresultate, Master-/Render-XML und PDF, aktuelle Logs, RunOutcome, Context und Handoff. Acceptance-/Lifecycle- und erforderliche Failure-Belege bleiben beim bestehenden spezialisierten Nach-Acceptance-Exportweg.

Der Projektkatalog wird im Korpuslauf nur als `project_file_catalog.tsv` erzeugt. `target` bleibt als Verzeichnis mit `generated_descendants_excluded` sichtbar; generierte Unterdateien werden nicht rekursiv katalogisiert. Geplante Handover-Outputs werden weiterhin explizit aufgefuehrt, auch wenn ihr Pfad unter `target/` liegt. Das ist keine vollstaendige rekursive Bestandsaufnahme. Manifest und Context deklarieren diese Grenze.

Die redundante Markdown-Darstellung wird fuer Korpuslaeufe nicht erzeugt oder eingebettet. `visual_preflight` behaelt seinen vollstaendigen Katalogvertrag mit Markdown und TSV. Der bereits begrenzte Normal-Run bleibt unveraendert.

Der Korpus-Handoff verweist auf den TSV und auf `corpus_emergence.result.txt` mit Dateiname, Bytezahl und SHA-256 statt den Bericht erneut einzubetten. Der vollstaendige Bericht bleibt in derselben ZIP. Fehlt er, ist dies als `run_result_missing` sichtbar. Andere Required Inputs, Snapshot-Fallbacks, Console- und Handoff-Budgets bleiben erhalten.

## Schneller Einstieg aus dem Paket

1. `assistant_handoff_latest.txt` fuer die Lesereihenfolge und den Zustand zur Exporterzeugung lesen.
2. `take_lifecycle_handover.txt` und den dort hashgebundenen Acceptance-Beleg fuer den endgueltigen Take-Status pruefen, sofern vorhanden. Der Handoff kann vor der Acceptance entstanden sein und ist selbst kein Verdict.
3. `assistant_context.json` und `source_snapshot/SOURCE_SNAPSHOT.txt` fuer die aktuelle Source-Bindung pruefen; den genau einmal enthaltenen Source-Snapshot verwenden.
4. Fuer den naechsten Auftrag die aktuellen Repository-Vertraege und Gates lesen. TSV, Korpusbericht, volle Projektresultate, Strict-Logs und Transition History werden bei Bedarf aus demselben Paket gelesen.

Die historische r1115-Dokumentation beschreibt den damaligen ausschliesslichen Normal-Run-Schnitt. Ab r1279 gilt die begrenzte TSV-Katalogprojektion zusaetzlich fuer Corpus; der Visual-Preflight-Vertrag bleibt davon getrennt. Historische Abnahmeberichte und SSC-Vertraege werden nicht umgeschrieben.

## Kellerputz und Grenzen

Entfernt werden die doppelte Corpus-Katalogerzeugung/-Aufnahme und die optionale Corpus-Einbettung des Katalogs und Berichts. Die gemeinsame Markdown-Routine, vollstaendige Rekursion und allgemeinen Omission-/Snapshot-Helper bleiben wegen anderer Exportmodi gebraucht. Kein kuenstlicher Zeilenabbau und kein Governance-Retirement.

Die Tests pruefen einen synthetischen fehlgeschlagenen Corpus-Export einschliesslich vollstaendiger gebundener Resultate und Score-Payloads, korrekter Fehlerprojektion, Source-Snapshot und TSV-Grenze. Weitere Tests erhalten Visual-Preflight-Inhalte, fehlenden Corpus-Bericht, Handoff-Determinismus, Omission-Reservierung und Fallbacks.

Lokale Parent-Admission, Transaktions-Reexec, Tests, Clippy und Strict-Build werden im separaten Auslieferungsbericht belegt. Die Windows-Abnahme erfolgt mit `./run.cmd --take --all`; sie ist durch lokale Source-Pruefungen nicht ersetzt. Paketgroesse und Musikneutralitaet werden an den jeweiligen Artefakten gemessen.
