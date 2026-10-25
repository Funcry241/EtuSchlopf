# Dokumentierte Abnahme r1521

Auswertung vom 25.10.2026, Paketversion 0.10.1839. Vergleichsbasis ist der
angenommene und zuvor veröffentlichte Stand r1520.

| Prüfung | Beobachtetes Ergebnis |
| --- | --- |
| Take-Lifecycle | r1521 angenommen; Acceptance-Hash stimmt mit dem finalen Lifecycle überein |
| Native Tests | 2.513 bestanden, 0 fehlgeschlagen, 1 ignoriert; 15 Testprogramme |
| Build / Strict Clippy / finaler Exit | jeweils 0 |
| Privater Zehnerkorpus | 10/10 erfolgreich und vollständig; Renderer bestanden; RunSeverity `dark_green` |
| Quell-Readback | alle 1.835 Take-Dateien bytegleich im nativen Source-Snapshot; zusätzlich Root-Cargo.lock |
| XML gegen r1520 | alle zehn Master-Dateien und alle zehn Render-Dateien bytegleich |
| PDF-Binärvergleich | alle zehn erneut erzeugten PDFs haben andere Bytes; kein Rastervergleich durchgeführt |

## Inhalt

Die README erfasst die vorhandenen Launcher- und Programmoptionen einschließlich
Aliasen, Standardwerten, Beispielen und der Watch-GUI. Der private absolute
Transferpfad wird durch `target/etuschlopf_neuro_evidence/` unter der jeweiligen
Projektwurzel ersetzt. Einzelprojekt- und Korpusexport behalten ihre vorhandenen
Payload-, Publikations-, Commit- und Retention-Verträge. Alte externe Dateien
bleiben unberührt.

Die [Take-Beschreibung](../../project_local_evidence_r1521.md) dokumentiert die
Erstellung und die damalige lokale Prüfung vor der nativen Abnahme. Die hier
veröffentlichten Auszüge dokumentieren den anschließenden angenommenen Lauf.

## Unveränderte native Auszüge

- [Acceptance-Datensatz](native_acceptance.txt)
- [Finaler Lifecycle-Handover](native_lifecycle_handover.txt)
- [RunOutcome](native_run_outcome.txt)

Angenommener Take-SHA-256:

```text
c5f9d0a6dc7cb225835fab947e680264833a3c23ae4b7ceb1abe45d0b00d3c43
```

## Redaktionelle Auswertung und Grenzen

Die [Zusammenfassung](summary.json) und das
[Runtime-/Tool-/Test-Hashinventar](accepted_runtime_sha256.tsv) stammen aus dem
nativen Export. Die 515 Inventarpfade entsprechen dem bisherigen öffentlichen
Runtime-Inventar; ihre Hashes werden aus dem aktuellen Snapshot neu berechnet
und gegen die publizierten Dateien geprüft. Das ist ein Dateivergleich, keine
zusätzliche Signatur oder Authority.

Die Laufzusammenfassung wurde mit den einzelnen Testresultaten abgeglichen.
Alle zehn Ergebnisberichte und der Korpusbericht stimmen in Größe und FNV64
mit ihren Manifestbindungen überein. Die 20 XML-Dateien wurden direkt gegen den
angenommenen r1520-Export verglichen. Es gibt weder einen neuen PDF-Rastervergleich
noch eine vollständige visuelle Seitenprüfung; aus XML-Gleichheit allein wird
keine Pixelgleichheit abgeleitet.

Der private WAV-/MIDI-Korpus und seine vollständigen musikalischen Ausgaben
werden nicht veröffentlicht. Ein frischer Checkout kann diesen Korpuslauf daher
nicht identisch reproduzieren. Die öffentliche README und Erläuterungen ergänzen
den nativen Quellstand. Die Take-Deklaration bleibt unverändert; GitHub-Archiv
und angenommenes Take-Paket sind unterschiedliche Artefakte.
