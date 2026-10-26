# Dokumentierte Abnahme r1526

Auswertung vom 26.10.2026, Paketversion 0.10.1839. Angenommener Parent und
Ergebnisvergleich: r1525. Vorheriger öffentlicher Git-Stand: r1521.

| Prüfung | Beobachtetes Ergebnis |
| --- | --- |
| Take-Lifecycle | r1526 angenommen; Paket-, Acceptance- und Lifecycle-Hashes geprüft |
| Native Tests | 2.538 bestanden, 0 fehlgeschlagen, 1 ignoriert; 15 Testprogramme, Einzelresultate summiert |
| Build / Strict Clippy / finaler Exit | jeweils 0; Clippy ohne Warnungen |
| Privater Zehnerkorpus | 10/10 erfolgreich und vollständig; Renderer bestanden; RunSeverity `dark_green` |
| Quell-Readback | alle 1.845 Take-Dateien bytegleich im nativen Source-Snapshot; zusätzlich Root-Cargo.lock |
| XML gegen r1525 | alle zehn Master-Dateien und alle zehn Render-Dateien bytegleich |
| PDFs gegen r1525 | alle zehn Binärdateien verschieden; alle 50 Seiten bei 96 dpi mit Poppler pixelgleich |

## Änderungen seit dem öffentlichen r1521-Stand

r1522 definiert den stafflokalen Zeitkontext, r1523 implementiert dessen
Verwendung in der bestehenden unteren Schlüsselplanung, r1524 korrigiert die
Ordnung nicht überlappender Ursprungsbeobachtungen. r1525 erhält vorhandene
Ergebnisse bei der Root-Bereinigung. r1526 erhält die Vierdateien-Sicherung bei
regulären Einzelaufrufen mit unterstützten Optionen und friert die kausale
Stage-A-Eingabedefinition für einen späteren Runtime-Schnitt ein.

Die historischen Take-Beschreibungen enthalten ihre damaligen lokalen
Prüfstände. Die hier veröffentlichten Auszüge dokumentieren den anschließend
nativ angenommenen r1526-Lauf; ihre Angaben werden nicht rückwirkend umgeschrieben.

## Unveränderte native Auszüge

- [Acceptance-Datensatz](native_acceptance.txt)
- [Finaler Lifecycle-Handover](native_lifecycle_handover.txt)
- [RunOutcome](native_run_outcome.txt)

Angenommener Take-SHA-256:

```text
adbf1d92e54a6ae0dbfc778d1f9a571fdf7fefa56bca432287a22568920f1070
```

## Redaktionelle Auswertung und Grenzen

Die [Zusammenfassung](summary.json) und das
[Runtime-/Tool-/Test-Hashinventar](accepted_runtime_sha256.tsv) sind read-only
Auswertungen des Exports. Das bisherige Inventar wird um die beiden neuen
Runtime-/Testdateien erweitert; Hashes stammen aus dem gebundenen Snapshot.
Alle zehn Ergebnisberichte und der Korpusbericht stimmen in Größe und FNV64
mit ihren Manifestbindungen überein. Die Lifecycle-Bindungen für Acceptance,
Kandidatenabsicht, Metadaten und semantischen Snapshot wurden geprüft.

Die 20 XML-Dateien wurden direkt gegen r1525 verglichen. Der PDF-Vergleich
verwendet Poppler `pdftoppm`, 96 dpi und exakten RGB-Pixelvergleich. Er belegt
Gleichheit bei dieser Rasterung, keine neue vollständige visuelle oder
musikalische Bewertung und keine unveränderte Darstellung seit r1521.

Der private WAV-/MIDI-Korpus und seine vollständigen musikalischen Ausgaben
werden nicht veröffentlicht. Ein frischer Checkout reproduziert diesen
Korpuslauf daher nicht allein. Das vorgebaute Windows-Anwenderpaket, vollständiger
Eingabe-/Recovery-Schutz, MIDI ohne WAV und unabhängige Fremdstück-Abnahme sind
offen. Der öffentliche Quellcode erfüllt diese offenen Anforderungen nicht
allein durch seine Veröffentlichung.

README und öffentliche Erläuterungen ergänzen den nativen Quellstand.
Runtime, Tests, Gates, Pins und Take-Deklaration werden bytegleich übernommen;
GitHub-Archiv und angenommenes Take-Paket bleiben unterschiedliche Artefakte.
