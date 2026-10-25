# Dokumentierte Abnahme r1512

Auswertung vom 25.10.2026, Paketversion 0.10.1839.

| Prüfung | Beobachtetes Ergebnis |
| --- | --- |
| Take-Lifecycle | r1512 angenommen, vollständiger SHA-256 im nativen Beleg |
| Native Tests | 2.482 bestanden, 0 fehlgeschlagen, 1 ignoriert |
| Build / Strict Clippy / finaler Exit | jeweils 0 |
| Privater Zehnerkorpus | 10/10 erfolgreich, vollständig, Renderer bestanden |
| Geplanter Effekt | 16 gemeinsame Registeralternativen in sieben Stücken bestätigt |
| XML-Erhalt | Noten, Rhythmus, Stimmen, Balken und Bindungen erhalten |
| Weitere XML-Änderung | nur automatisch abgeleitete Systemabstände in Etüde 4 |
| PDF-Readback | 50 Seiten vorher/nachher; 16 Seiten pixelverschieden |
| Etüde 4 | maximale Hilfslinienzahl fünf auf drei; neue Ausgabe visuell geprüft |

## Belegarten

- [Nativer Acceptance-Datensatz](native_acceptance.txt): unveränderte Kopie des
  Annahmebelegs aus dem übergebenen Export.
- [Nativer RunOutcome](native_run_outcome.txt): unveränderte Kopie der strukturierten
  Laufprojektion; sie weist unter anderem die Grenzen ihrer eigenen Zuständigkeit aus.
- [Maschinenlesbare Zusammenfassung](summary.json): redaktionelle Auswertung des
  Laufs und der unabhängigen XML-/PDF-Prüfung, mit expliziten Grenzen.
- [SHA-256-Inventar der angenommenen Runtime-/Tool-/Testdateien](accepted_runtime_sha256.tsv):
  aus dem tatsächlich exportierten Source-Snapshot berechnet und gegen den hier
  veröffentlichten Baum geprüft. Es ist ein Dateivergleich, keine neue Signatur.

Der Hash des angenommenen Takes ist:

```text
ea977b4a07e176a72f046b256cb0a4258df678241f32ecd491822ff05c5ed3e8
```

Git enthält zusätzlich eigene öffentliche Dokumentation, die Git-Ausschlussregeln,
die Lizenz und den Root-Cargo.lock. Der Take-Hash bezeichnet das originale Paket,
nicht einen GitHub-Download oder jeden späteren Dokumentationsstand. Die eingefrorenen
Take-Deklarationen werden für die öffentliche Beschreibung nicht umgeschrieben.

Der [Git-Quellimport von r1512](https://github.com/Funcry241/EtuSchlopf/commit/dff05fefb66e09916f8097922bceeea5f4930fd3)
hält den angenommenen Quellstand vor der zusätzlichen öffentlichen Beschreibung
fest. Das Hash-Inventar bindet auch den zum nativen Snapshot passenden Root-Cargo.lock.

## Reichweite und Reproduzierbarkeit

Der komplette private WAV-/MIDI-Korpus und seine musikalischen Ausgaben werden
hier nicht veröffentlicht. Der native Korpuslauf lässt sich daher nicht allein
aus einem frischen Checkout identisch wiederholen. Die veröffentlichten Auszüge
machen die konkrete Abnahme nachvollziehbar; sie ersetzen weder die Eingaben
noch die vollständigen ursprünglichen Laufartefakte.

Der Readback verglich alle 20 MusicXML-Dateien und die gerenderten PDF-Seiten.
Etüden 1, 7 und 8 behalten ihre XMLs bytegenau und ihre PDF-Seiten pixelgleich.
Die musikalische Bewertung bleibt von der Softwarefreigabe getrennt. Bekannte
untere Register-/Schlüsselprobleme, unter anderem E3/T16 und E6/T22, sind nicht
durch diese Änderung gelöst. Es wird keine empirische Spielbarkeitsgarantie oder
fehlerfreie Gesamtnotation behauptet.
