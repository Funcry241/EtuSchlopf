# r1521 – ProjectLocalEvidenceAndCompleteReadme

Version 0.10.1839, ChangeClass OutcomeProjection. Parent ist der im gelieferten
Export hashgebunden angenommene r1520, Take-SHA-256
`9c9874a4bb48606bb2e3c48b2570067f3af89188b6e90e55b9984ea7de447ef7`.

## Auftrag und Wirkung

Die README dokumentiert die vorhandenen Programm- und Launcheroptionen samt
Aliasen, Standardwerten, Beispielen und der Watch-GUI. Sie weist auch auf die
tatsaechliche Argumentweitergabe im Launcher-Korpuspfad hin.

Der private absolute Transferpfad des Neuro-Evidence-Producers entfaellt.
Machine-Evidence bleibt erhalten und wird ausschliesslich unter
`<Projekt-Root>/target/etuschlopf_neuro_evidence/` publiziert. Der bereits
vorhandene Projekt-Root wird bis zum Korpus-Publikationssink durchgereicht;
Einzelprojekt- und Korpussink leiten ihren Zielordner daraus ab.

Es gibt keinen neuen Konfigurationsschalter und keinen externen Fallback.
Bestehende externe Dateien bleiben unberuehrt. Die Retention arbeitet weiter
mit derselben Dateinamens-Allowlist, Hash-/Manifestpruefung, Schutzmenge und
Kohortenbegrenzung, jetzt ausschliesslich im projektlokalen Zielverzeichnis.
Machine-Evidence und Assistant-Export bleiben getrennte Trust-Domains.

Musikpipeline, Notation, Renderer, Schema-2-Payload, Korpuspopulation,
Commit-Vertrag, atomare Publikation und Rollback bleiben unveraendert.
Der eine betroffene Neuro-Exporter-FNV im bestehenden Trust-Anchor wird auf
die finalen Source-Bytes gebunden; keine Gate-Bedingung wird veraendert.
Historische Dokumente beschreiben weiterhin ihre damaligen Pfade. Der
aktuelle Speicherort steht in README und Producer-Trust-Domain-Dokument.

## Pruefung

Zwei synthetische Regressionen pruefen die realen Publikationspfade:
Einzellauf-Evidence in zwei unabhaengigen Projektwurzeln sowie publizierte
vorbereitete Archiv-/Commit-Dateien im Projekt-Root mit erhaltenen Fremddateien.
Die zweite Fixture testet den Publikationssink, nicht die vorgelagerte
Vollkorpus-Mitgliedschaft. Die bestehenden Payload-, Container-, Commit- und
Retention-Tests bleiben erhalten. Der unveraenderte Parent-Take-Pruefer wird
gegen den vollstaendigen incoming Baum ausgefuehrt.

Lokal bestanden: alle 16 Neuro-Evidence-/Retention-Tests inklusive der beiden
neuen Publikationsregressionen sowie Strict-Clippy fuer den Launcher mit
`-D warnings`, `-D clippy::correctness`, `-D clippy::suspicious` und
`-D clippy::perf`. Der unveraenderte Parent-Pruefer meldet sechs deklarierte
und sechs tatsaechliche Aenderungen, keine undeclarierten Aenderungen, keine
entfernte Capability, keine Music-, Calibration-, Writer-Family- oder
Unknown-Capability-Aenderung.

Lokale Compiler-/Testbelege sind keine native Windows-Annahme. Ein neuer
musikalischer Vollkorpuslauf ist in dieser Arbeitsumgebung nicht belegt.

## Anwendung

Das Take-ZIP im Projektordner oder Benutzer-Downloads bereitstellen und aus
dem bestehenden r1520-Projekt starten:

```cmd
.\run.cmd --take --watch --all
```

Die resultierenden Machine-Evidence-Dateien liegen danach unter
`target\etuschlopf_neuro_evidence\` dieses Projekts. Ein bestehender
NeuroSchlopf-Consumer muss fuer neue Dateien diesen lokalen Ausgabeordner
verwenden; alte externe Archive werden durch diesen Take nicht migriert.
