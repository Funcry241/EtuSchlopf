# Dokumentierte Abnahme r1520

Auswertung vom 25.10.2026, Paketversion 0.10.1839.

| Prüfung | Beobachtetes Ergebnis |
| --- | --- |
| Take-Lifecycle | r1520 angenommen; Annahmebeleg und Lifecycle-Hash stimmen überein |
| Native Tests | 2.511 bestanden, 0 fehlgeschlagen, 1 ignoriert |
| Build / Strict Clippy / finaler Exit | jeweils 0 |
| Privater Zehnerkorpus | 10/10 erfolgreich und vollständig; Renderer bestanden |
| Quell-Readback | alle 1.834 Take-Dateien bytegleich im nativen Source-Snapshot; zusätzlich Root-Cargo.lock |
| XML gegen r1518 | alle zehn Master-Dateien bytegleich; fünf Render-Dateien unverändert |
| Geplanter Effekt | ausschließlich neun erzeugte Oktav-Wort-Wrapper in fünf Render-Dateien entfernt |
| Sonstige XML-Bytes | unverändert; insbesondere Noten und octave-shift-Anweisungen |
| PDF-Seitenzahlen | für jede Etüde unverändert; zusammen 50 Seiten |

## Unveränderte native Auszüge

- [Acceptance-Datensatz](native_acceptance.txt)
- [Finaler Lifecycle-Handover](native_lifecycle_handover.txt)
- [RunOutcome](native_run_outcome.txt)

Die hashgebundene finale Annahme entscheidet. Der früher erzeugte Handoff im
Originalexport beschreibt noch einen laufenden Take und ist allein kein Urteil.
Der im Lifecycle enthaltene zurückgewiesene Versuch r1519 ist Vorgeschichte;
der aktuelle angenommene Kandidat ist r1520.

Angenommener Take-SHA-256:

```text
9c9874a4bb48606bb2e3c48b2570067f3af89188b6e90e55b9984ea7de447ef7
```

## Redaktionelle Auswertung und Grenzen

Die [Zusammenfassung](summary.json) und das
[Runtime-/Tool-/Test-Hashinventar](accepted_runtime_sha256.tsv) wurden aus dem
nativen Export erstellt. Das Inventar wird gegen die veröffentlichten Dateien
geprüft; es ist ein Dateivergleich, keine zusätzliche Signatur oder Authority.

GitHub stand zuvor auf r1512. Der Quellimport enthält daher auch die angenommenen
Zwischenschritte r1513 bis r1518. Der hier berichtete Vergleich der 20 XML-Dateien
bezieht sich ausdrücklich auf den zuletzt angenommenen Stand r1518. Die neun
Entfernungen verteilen sich auf Etüden 1, 3, 4, 6 und 9 (2/1/2/3/1).

Der vollständige private WAV-/MIDI-Korpus und seine musikalischen Ausgaben werden
nicht veröffentlicht. Ein frischer Checkout kann diesen Korpuslauf deshalb nicht
identisch reproduzieren. Seitenzahlen und die erfolgreichen nativen Linienprüfungen
ersetzen keine neue visuelle Einzelprüfung aller Seiten. Eine allgemeine
Spielbarkeitsgarantie wird nicht behauptet.

Die öffentliche README und Erläuterungen ergänzen den nativen Quellstand.
Take-Deklarationen bleiben unverändert. Ein GitHub-Archiv ist kein installierbarer
Take; der obige Hash bezeichnet das originale angenommene Paket.
