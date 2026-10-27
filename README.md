# EtuSchlopf

**Klaviernoten aus zusammengehörigen MIDI- und WAV-Dateien – als PDF und zum Bearbeiten in MuseScore.**

EtuSchlopf ordnet musikalische Ereignisse zu einer zweihändigen Klavierdarstellung
und bearbeitet unter anderem Stimmen, Rhythmus, Pausen, Balken und Register.
Das Ergebnis ist ein Ausgangspunkt für deine weitere musikalische Arbeit.

**[Windows-EXE herunterladen · Testversion r1526 · 7,7 MB](https://github.com/Funcry241/EtuSchlopf/releases/download/v0.10.1839-r1526-windows-preview/EtuSchlopf-Windows-x64.exe)**

Windows x64 · kein Rust oder Cargo erforderlich · MuseScore für PDF · MIDI und WAV als Eingabe

[Versionshinweise und Prüfsumme](https://github.com/Funcry241/EtuSchlopf/releases/tag/v0.10.1839-r1526-windows-preview) ·
[Erstes Stück](#dein-erstes-stück) · [Grenzen der Testversion](#aktueller-stand) ·
[Technische Dokumentation](docs/DEVELOPMENT.md)

## Ein Blick auf die Noten

[![EtuSchlopf-Notenbeispiel: erstes Klaviersystem der Etüde Nr. 1, Entwicklungsstand r1537](docs/images/etude-1-r1537-ausschnitt.png)](docs/examples/etude-1-r1537.pdf)

*Etüde Nr. 1 aus dem eigenen Evaluationskorpus: unveränderter PDF-Ausschnitt,
Entwicklungsstand r1537. Bild anklicken für die vollständige PDF.
Die herunterladbare EXE enthält den älteren Stand r1526; das Beispiel zeigt r1537.*

## Für wen ist EtuSchlopf gedacht?

Für Menschen, die zu einem Stück eine MIDI-Datei und eine passende WAV-Datei
besitzen und daraus eine Klavierfassung zum Lesen und Weiterbearbeiten gewinnen
möchten. MIDI liefert die Notenereignisse und ihre Zeitstruktur; WAV steht als
Klangaufnahme für die Audio-Beobachtung zur Verfügung. Beide Dateien werden
im derzeitigen Ablauf benötigt.

Das Projekt ist experimentell und privat gepflegt. Prüfe am eigenen Stück,
ob dir die Fassung bei der weiteren Arbeit hilft; musikalische Nachbearbeitung
kann erforderlich sein.

## Dein erstes Stück

Du brauchst eine MIDI-Datei und eine gleichnamige WAV-Datei, zum Beispiel:

```text
Mein Stück.mid
Mein Stück.wav
```

1. Lade die Windows-EXE herunter. Für die PDF-Ausgabe brauchst du außerdem **[MuseScore](https://musescore.org/de/download)**.
2. Ziehe deine `.mid`- oder `.midi`-Datei auf die EXE. Alternativ startest du die EXE und gibst den Pfad zur MIDI-Datei ein.
3. Öffne nach erfolgreichem Lauf die PDF im angezeigten Ergebnisordner.

Neben deiner MIDI entsteht der Ordner **`EtuSchlopf-Ergebnisse`**. Jeder Lauf
bekommt darin einen neuen Unterordner. Das Programm arbeitet mit Kopien deiner
Eingaben; Originaldateien und frühere Laufordner bleiben erhalten.

## Deine Ergebnisse

| Du möchtest … | Öffne … |
| --- | --- |
| die Noten lesen oder ausdrucken | `Mein Stück.pdf` |
| die Noten in MuseScore bearbeiten | `Mein Stück.render.musicxml` |
| eine Fehlermeldung genauer nachvollziehen | `Mein Stück.result.txt`, sofern der Lauf einen Bericht erzeugt hat |

Für die Bearbeitung in MuseScore ist die Datei mit **`.render.musicxml`** die
richtige Wahl. Weitere technische Dateien können im Ergebnisordner liegen.

## Wenn der Start nicht klappt

- **WAV fehlt:** Lege eine echte gleichnamige WAV-Datei neben die MIDI. MIDI ohne WAV wird derzeit noch nicht unterstützt.
- **Keine PDF:** Prüfe, ob MuseScore installiert ist. Mit `EtuSchlopf-Windows-x64.exe "Mein Stück.mid" --no-pdf` kannst du nur die Notendateien anfordern.
- **Datei wird abgelehnt:** Bewahre die konkrete Meldung auf. Nicht jede MIDI-Struktur wird unterstützt; manchmal hilft ein erneuter Export aus deinem Musikprogramm.
- **Lauf fehlgeschlagen:** Frühere Ergebnisordner bleiben bestehen. Eine Datei aus einem fehlgeschlagenen Lauf ist noch kein freigegebenes Ergebnis.

## Aktueller Stand

EtuSchlopf ist eine **Vorschau in Entwicklung**. Es gibt noch keinen abgeschlossenen
Anwendertest auf einem sauberen Windows-Rechner mit MuseScore und unabhängiger
Bewertung fremder Stücke. Bei der herunterladbaren Testversion r1526 wurde auch ein einfaches synthetisches
Stück an einer bestehenden Berichtprüfung abgelehnt. Lesbarkeit und Spielbarkeit
musst du daher am eigenen Ergebnis prüfen; eine fertige druckreife Fassung
für jede MIDI wird nicht versprochen.

Die herunterladbare Windows-Testversion enthält **r1526**. Einzelheiten zu den
[Prüfbelegen](docs/evidence/r1526/README.md), die
[vollständige technische Optionsreferenz](docs/DEVELOPMENT.md#vollständige-aufrufreferenz)
und Hinweise für die [Arbeit am Quellcode](docs/DEVELOPMENT.md) liegen auf eigenen Seiten.

## Quellstand und Beiträge

Der veröffentlichte Quellstand ist **r1533 / 0.10.1839**, nativ angenommen:
2.564 Tests bestanden, Build und Clippy erfolgreich, privater Zehnerkorpus
vollständig erfolgreich. Gegen r1532 sind 20 XML-Dateien bytegleich und alle
50 PDF-Seiten bei 144 dpi pixelgleich. Die [Prüfbelege](docs/evidence/r1533/README.md)
beschreiben Umfang und Grenzen. Der Download oben bleibt die separate Testversion
**r1526**; mit diesem Quellupdate wird keine neue EXE veröffentlicht.

Für die Arbeit am Quellcode gelten die [technische Referenz](docs/DEVELOPMENT.md)
und die [Beitragsregeln](docs/CONTRIBUTING.md). Der [Dokumentationswegweiser](docs/README.md)
führt zu Architektur, Diagnose und Community-Dateien.

Fehlerberichte sind willkommen. Bitte nenne die verwendete Version, die konkrete\nMeldung und – bei einem Notationsfehler – die betroffene Stelle.\n[Beiträge](docs/CONTRIBUTING.md) · [Sicherheit](docs/SECURITY.md) ·\n[Umgang miteinander](docs/CODE_OF_CONDUCT.md) · [Barrierefreiheit](docs/ACCESSIBILITY.md)\n\n## Lizenz\n
EtuSchlopf steht unter der [MIT-Lizenz](LICENSE).
[Fremdkomponenten und Lizenzen](docs/THIRD_PARTY_NOTICES.md).
