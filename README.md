# EtuSchlopf

**EtuSchlopf erzeugt aus Projektmaterial eine Klavierfassung und prüft die
zulässigen Verarbeitungsschritte bis zu MusicXML, PDF und Ergebnisbericht.**
Die Implementierung ist Rust-basiert; der gepflegte Einstieg ist der
Windows-Launcher `run.cmd`. Die lokale Watch-GUI zeigt Laufzustände,
Projektfortschritt, Zeiten und Links zu den PDFs.

Das Projekt hat zwei eng verbundene Gegenstände:

- **Musik:** Ereignisse, Stimmen, Hände, Rhythmus, Pausen, Balken, Register und
  Layout müssen zu einer lesbaren Klavierdarstellung zusammenfinden.
- **Entwicklung mit KI:** Vorschläge werden an kausalen Fehlerklassen,
  Zuständigkeiten, erhaltenen Identitäten und ausführbaren Kontrollen gemessen.
  Ein plausibler Patch oder ein überzeugender KI-Text reicht nicht als Beleg.

Nach Angabe des Projektinhabers wird der Code vollständig mit KI-Assistenten
erzeugt. Zielsetzung, Gegenprüfung und Abnahme liegen beim Menschen. Die im Repository hinterlegten Verträge, die Architektur-Essenz
und der Kaltstartpfad machen die Arbeit über Modell- und Gesprächswechsel hinweg
nachvollziehbar. Gates sichern viele konkrete Grenzen; sie sind selbst endliche
Programme und bleiben Gegenstand von Gegenbeispielen und Prüfung.

**Zum Verständnis des Projekts:**

- [Was tatsächlich implementiert ist, einschließlich SchlopfFormel und Mathematik](docs/IMPLEMENTED_FOUNDATIONS.md)
- [Forschungsbezüge und ihre genaue Rolle](docs/RESEARCH_CONTEXT.md)
- [Dokumentierte Abnahme von r1521](docs/evidence/r1521/README.md)
- [Kaltstart ohne Gesprächsgedächtnis](docs/assistant_memory/000_start_here.md)

**Geprüfter Stand am 25.10.2026:** r1521, Paketversion 0.10.1839; 2.513 Tests
bestanden, ein Test ignoriert, Build/Clippy erfolgreich, privater Evaluationskorpus
10/10. Das sind Belege dieses konkreten nativen Laufs, keine universelle
Spielbarkeitsgarantie und kein öffentlich vollständig reproduzierbarer Korpustest.
Das Notenbild bleibt in Arbeit. Der Git-Stand enthält die angenommene r1521-Quelle
und eine gesonderte öffentliche Dokumentationsschicht; Git-Commit und Take-Annahme
sind unterschiedliche Nachweise.

*Zwischen Ereignis und Zeichen liegt eine Entscheidung.*

## Inhalt

- [Eingaben und Ausgaben](#die-eingänge)
- [Installation und erster Lauf](#eine-maschine-in-gang-setzen)
- [Watch-GUI](#die-gui-etuschlopf-watch)
- [Alle Optionen, Aliase und Beispiele](#vollständige-aufrufreferenz)
- [Architektur und Verarbeitung](#vom-ereignis-zur-gestalt)
- [Revisionen und Takes](#revisionen-takes-und-rückkehr)
- [Lizenz](#lizenz)

## Die Eingänge

Ein Projekt beginnt im regulären Ablauf mit zwei gleichnamigen Dateien:

```text
Mein Projekt.wav
Mein Projekt.mid
```

Die zeitliche und musikalische Ereignisstruktur und die beobachtete Klangspur
werden in der Verarbeitung miteinander in Beziehung gesetzt. Ihre Rollen bleiben
unterscheidbar. Ein Hinweis aus einer Beobachtung wird erst durch die jeweils
zuständige Verarbeitung zu einer zulässigen Entscheidung.

Am anderen Ende stehen normalerweise vier Dateien:

| Datei | Rolle |
| --- | --- |
| `Mein Projekt.musicxml` | musikalischer Master für den weiteren Ablauf |
| `Mein Projekt.render.musicxml` | für die sichtbare Notation bestimmte Fassung |
| `Mein Projekt.pdf` | durch einen externen Renderer gesetzte Seiten |
| `Mein Projekt.result.txt` | Befunde, Laufbelege und Ergebnisprojektionen |

Nach erfolgreichen Läufen legt der separate Machine-Evidence-Export seine
ZIPs und Korpus-Commits unter `target/etuschlopf_neuro_evidence/` im jeweiligen
Projektordner ab. Ein fest eingebauter privater Transferordner wird dafür
nicht benötigt. Die Aufräumlogik arbeitet nur in diesem lokalen
Evidence-Verzeichnis; bereits vorhandene externe Archive werden nicht
verschoben oder gelöscht. `assistant_export_latest.zip` bleibt separat im
Projekt-Root.

Die beiden XML-Fassungen haben verschiedene Aufgaben. Ein Unterschied zwischen
ihnen braucht einen erklärbaren Übergang. Die PDF zeigt, was der Renderer daraus
macht. Der Bericht hält fest, was der Lauf tatsächlich beobachten konnte.

## Eine Maschine in Gang setzen

Der gepflegte Einstieg ist der Windows-Launcher `run.cmd`. Das Projekt verwendet
Rust; die festgelegte Toolchain steht in
[`rust-toolchain.toml`](rust-toolchain.toml). Für die PDF-Ausgabe wird eine
passende externe MuseScore-Installation benötigt. Einige Beobachtungspfade haben
weitere Voraussetzungen, die der jeweilige Lauf prüft.

Festgelegt sind **Rust 1.98.0 mit Clippy und rustfmt**. Die separat aufrufbaren
PDF-Diagnosewerkzeuge verwenden **Python ab 3.10**, **pypdfium2 5.14.0** und
für ihren Renderer-Vorabcheck **MuseScore 4.7.4**. Ihre Einrichtung steht in
[`tools/assistant_diagnostics/README.md`](tools/assistant_diagnostics/README.md).

1. Repository klonen und in den Projektordner wechseln.
2. Rust mit `rustup` bereitstellen und die festgelegte Toolchain verfügbar machen.
3. Die eigenen gleichnamigen WAV-/MIDI-Eingaben in den Projektordner legen.
4. Ein Projekt starten:

```cmd
.\run.cmd "Mein Projekt"
```

Alle erkannten Projekte im Ordner verarbeiten:

```cmd
.\run.cmd --all
```

Die Projektdateien des ursprünglichen Evaluationskorpus werden hier nicht
mitgeliefert. Die genannten Befehle setzen eigene geeignete Eingaben voraus.
Ein frischer Checkout allein erzeugt daher noch keine Klavierseiten.

Der Launcher ist Teil des Arbeitsablaufs. Er koordiniert unter anderem Build,
Prüfungen, Lauf und Belegausgabe. Strict ist der Standard. Wiederverwendung
bereits geprüfter Ergebnisse ist an die dafür vorgesehenen Belege gebunden.
Ein vorhandener Build-Cache ersetzt diese Bindung nicht.

## Die GUI: EtuSchlopf Watch

Mit `--watch` öffnet der Launcher die grafische Laufansicht im Browser:

```cmd
.\run.cmd --watch --project "Mein Projekt"
.\run.cmd --watch --all
```

`--show` und `--display` sind gleichwertige Aliase. Der Launcher baut und startet
das mitgelieferte Watch-Programm bei Bedarf automatisch. Die Browseransicht ist
lokal unter `http://127.0.0.1:47623/` erreichbar.

Die GUI zeigt den aktuellen Launcher-, Korpus- und Projektzustand, die laufende
Verarbeitungsphase, Lauf- und Phasenzeiten sowie den Zeitpunkt der letzten
Beobachtung. Im Korpuslauf kommen Fortschrittsbalken, aktive Projekte und eine
aufklappbare Projektliste hinzu. Projektlinks öffnen die bereitgestellten PDFs;
die Ansicht lädt diese vorab als Browser-Blobs, damit geladene PDFs nach dem
Laufende weiter geöffnet werden können.

Watch ist eine beobachtende Oberfläche. Die musikalische Verarbeitung startet
über den Aufruf von `run.cmd`; die GUI verändert weder Eingaben noch
Notationsentscheidungen. Ist Watch nicht verfügbar, meldet der Launcher dies
und setzt den musikalischen Lauf fort. Der vom Launcher gestartete
Watch-Prozess wird am Laufende beendet; die Browserseite bleibt geöffnet und
erhält danach keine neuen Zustände.

Das Watch-Programm liegt in
[`tools/etuschlopf_watch/`](tools/etuschlopf_watch/). Es lässt sich auch separat
mit `tools\etuschlopf_watch\run.cmd` starten. Bei diesem separaten Einstieg
kann es bei nicht verfügbarer Webansicht auf die native Watch-Oberfläche
zurückfallen; der Windows-Launcher fordert ausdrücklich die Webansicht an.

## Vollständige Aufrufreferenz

Die folgenden Tabellen erfassen die aktuellen Launcher- und Programmoptionen
einschließlich Aliasen, Standardwerten und entfallenen Schaltern. Grundlage sind
[`src/cli.rs`](src/cli.rs),
[`src/bin/etuschlopf_launcher.rs`](src/bin/etuschlopf_launcher.rs) und der
[Take-Bootstrap](src/bin/etuschlopf_take_bootstrap.rs).

### Launcher: Projekt, GUI, Prüfungen und Exporte

Aufruf: `.\run.cmd [Launcher-Optionen] "Projektname" [Programmoptionen]`.
Für den regulären Lauf ist ein Projekt, `--all` oder `--visual-preflight`
anzugeben. `--clippy-deep` ist ein eigenständiger Diagnosemodus.

| Option / Argument | Wirkung und Standard |
| --- | --- |
| `"Projektname"` | Verarbeitet das gleichnamige WAV-/MIDI-Paar. Namen mit Leerzeichen in Anführungszeichen setzen. |
| `--project "Projektname"`, `--project="Projektname"` | Explizite Projektauswahl; der Launcher übersetzt sie in das positionale Programmargument. |
| `--all` | Verarbeitet alle erkannten WAV-/MIDI-Paare und erstellt den Korpusbericht sowie gebundene Korpusbelege. Aliase: `-all`, `/all`. |
| `--watch` | Startet die lokale Watch-GUI für den Lauf. Standard: aus. Aliase: `--show`, `--display`. |
| `--strict` | Kompatibilitätsschalter: Strict samt regulärem Clippy- und Testvertrag ist bereits der Standard. `strict.cmd` ruft `run.cmd --strict` auf. |
| `--force` | Erzwingt einen Build statt der Wiederverwendung eines verifizierten Programm-Binarys. Im Visual-Preflight erzwingt es außerdem erneutes Rendern statt Renderer-Wiederverwendung. Beim Take kann es auch die erneute Übernahme eines gleichen Versionsstands verlangen. Gates bleiben wirksam. |
| `--clippy-deep` | Eigenständige vertiefte Clippy-Diagnose, ohne musikalischen Projektlauf; Ausgabe in `etuschlopf.deep-clippy.txt`. Mit `--force` kombinierbar. |
| `--visual-preflight` | Startet ohne Projektauswahl den eigenen Visual-Preflight-Ablauf mit dessen gebundener Projektauswahl und Renderer-Prüfung. Aliase: `-visual-preflight`, `/visual-preflight`, `--quick-visual`, `--fast-visual`. Mit Projekt oder `--all` bleibt der normale Strict-Lauf das Ziel; das Visual-Preflight-Testgate ist dort bereits integriert. |
| `--assistant-export-pdf` | Nimmt PDFs zusätzlich in den optionalen PDF-Exportumfang des Assistant-Exports auf. Standard: Minimalexport ohne PDF-Opt-in. Aliase: `--export-pdf`, `--with-pdf`. Die normale PDF-Erzeugung steuert weiterhin `--no-pdf`. |
| `--inventory` | Ergänzt einen Einzelprojekt- oder Korpuslauf um `EtuSchlopf.inventory.json` und `EtuSchlopf.inventory.xlsx`, gebunden an Lauf und Quellstand; Verträge ESI-3, ARCH-4, CMS-1 und XLSX-3. Standard: aus. |
| `--take` | Lässt den transaktionalen Bootstrap ein geeignetes Source-/Take-ZIP aus Projektordner oder Benutzer-Downloads auswählen und prüfen. Mit Projekt oder `--all` wird anschließend der entsprechende Lauf gestartet. Ein Assistant-Export ist kein installierbarer Take. |

**Weitergabe im Korpuslauf:** Der aktuelle Launcher startet das Programm bei
`--all` mit `--all` und gegebenenfalls `--writer-trace[=…]`. Weitere
Programmoptionen wie `--jobs`, `--no-pdf`, `--layout-pages` oder `--corpus-out`
werden in diesem Launcherpfad nicht weitergereicht. Die Launcheroptionen
`--watch`, `--inventory`, `--force` und die Exportauswahl verarbeitet der
Launcher selbst. Für einen Batch mit weiteren Programmoptionen ist der direkte
Programmaufruf unten maßgeblich.

Der eigenständige Visual-Preflight wählt sein Projekt aus dem frischen,
hashgebundenen Korpusmanifest und dessen Hotspot-Bericht. Fehlt eine aktuelle
Auswahl, ist zunächst ein vollständiger Korpuslauf erforderlich.

### Programm: Verarbeitung, Ausgaben und Diagnose

Diese Optionen gehören zum Programm `etuschlopf`. Bei einem normalen
Einzelprojekt gibt `run.cmd` sie an das Programm weiter. Der direkte Aufruf
lautet `etuschlopf [OPTIONS] [INPUT]`.

| Option / Argument | Wirkung und Standard |
| --- | --- |
| `[INPUT]` | Projektname bzw. Projektpfad zur Ermittlung des WAV-/MIDI-Paars. |
| `--all` | Ordner-Batch mit eigenen Ausgaben pro Projekt und anschließendem Korpusbericht. Aliase: `-all`, `/all`. |
| `--corpus-emergence` | Wertet vorhandene `*.result.txt` zu einem Korpusbericht mit PassOrder-/Obstruction-Audit aus, ohne Musikmutation. Für diesen Modus ohne Projekt den direkten Programmaufruf verwenden. |
| `--corpus-out <PFAD>` | Pfad des Korpus-/Mathematikberichts. Standard: `corpus_emergence.result.txt`. |
| `--use-corpus-cache` | Erlaubt im Einzellauf, eine vorhandene `corpus_emergence.result.txt` als rein beobachtenden Cache zu lesen. Standard: ignorieren. |
| `--out <PFAD>` | Pfad des Ergebnisberichts. Standard: `<Projekt>.result.txt`. |
| `--musicxml-out <PFAD>` | Pfad der erzeugten Master-MusicXML-Datei. Standard: `<Projekt>.musicxml`; die Renderfassung wird separat abgeleitet. Die Ausgabe ist keine alternative Eingabequelle. |
| `--pdf-out <PFAD>` | Pfad der PDF-Ausgabe bei verfügbarem Renderer. Standard: `<Projekt>.pdf`. |
| `--reference-wav <PFAD>` | Explizite WAV-Referenz. Ohne Angabe erfolgt die eindeutige Suche im Eingabeordner. |
| `--no-pdf` | Überspringt die PDF-Erzeugung. Standard: PDF-Erzeugung anfordern. |
| `--layout-pages <ANZAHL>` | Feste Zielseitenzahl. Ohne Angabe bestimmt das Programm sie anhand der Dichte. |
| `--no-polish` | Überspringt die Notensatz-Politur; Prüfmodus ohne MusicXML-Ausgabe. Standard: Politur aktiv. |
| `--lenient-partial-voices` | Unterfüllte Nebenstimmen erzeugen Warnungen statt Blockern. Standard: strenge Behandlung. Dies ist eine fachliche Prüfoption, kein Abschalten der Strict-Build-Gates. |
| `--top <ANZAHL>` | Anzahl der wichtigsten Warnungen/Hotspots im Bericht. Standard: `12`. |
| `--writer-trace[=<TAKTE>]` | Zusätzliche Render-Writer-Snapshots und ausführliche Balken-Diagnosen. Ohne Wert für alle Takte, mit `--writer-trace=5,7` nur für diese Takte. Ein Filter muss mit `=` angeschlossen werden. Standard: kein ausführlicher Trace; Balken-Auffälligkeiten und Summen bleiben sichtbar. |
| `--jobs <WERT>` | Gesamtbudget für Rechenparallelität: `auto`, `max` oder eine positive Zahl. Standard: `auto`, erkannte logische Threads minus eins, mindestens eins. `1` für seriellen Betrieb, `max` für volle Parallelität. Beim direkten Batch wird das Budget auf höchstens zwei gleichzeitige Projekte verteilt. Die semantische Suchbreite bleibt davon getrennt festgelegt. |
| `-h`, `--help` | Zeigt die Programmhilfe. |
| `-V`, `--version` | Zeigt die Programmversion. |

**Direkter Programmaufruf:** Nach einem erfolgreichen Strict-Build liegt das
Windows-Programm normalerweise unter `target\strict\etuschlopf.exe`.
Beispielsweise:

```cmd
.\target\strict\etuschlopf.exe --help
.\target\strict\etuschlopf.exe --version
.\target\strict\etuschlopf.exe --corpus-emergence --corpus-out "Korpusbericht.txt"
.\target\strict\etuschlopf.exe --all --jobs 4 --no-pdf
```

Falls das Binary noch fehlt, lässt es sich mit
`cargo build --locked --profile strict --bin etuschlopf` bauen. Der direkte
Programmstart führt die Launcher-Schritte für Tests, Exporte, Inventur und
Watch nicht aus. Für die Programmhilfe ist er auch deshalb der passende Weg:
`run.cmd --help` ohne Laufziel wird vom Launcher als fehlender Aufruf behandelt.

### Kompatibilität und interne Schalter

| Schreibweise | Status |
| --- | --- |
| `etuschlopf check "Projektname" …` | Historisches Präfix beim direkten Programmaufruf; `check` wird vor dem Parsen entfernt. Aktuell genügt `etuschlopf "Projektname" …`. |
| `--candidate-budget-jobs <ANZAHL>` | Versteckter, entfallener Schalter; wird ausdrücklich zurückgewiesen. `--jobs` steuert die Parallelität, der Suchumfang bleibt fest `192/5`. |
| `--take-zip`, `--take-zip=…` | Pensionierter direkter Launcher-Take-Pfad; wird zurückgewiesen. Der unterstützte Einstieg ist `run.cmd --take`. |
| `--recover-pending-take` | Interner Recovery-Einstieg des gesicherten Take-Bootstraps, bei offener Transaktion durch `run.cmd` aufgerufen. |
| `--recover-pending-public-artifacts` | Interner Recovery-Einstieg des gesicherten Bootstraps für einen offenen Public-Artifact-Vorgang, ebenfalls durch `run.cmd` koordiniert. |

### Beispiele für kombinierte Aufrufe

```cmd
.\run.cmd --watch --project "Mein Projekt" --jobs 4
.\run.cmd --project "Mein Projekt" --writer-trace=5,7 --top 20
.\run.cmd --project "Mein Projekt" --layout-pages 6 --pdf-out "Meine Fassung.pdf"
.\run.cmd --watch --all --inventory --assistant-export-pdf
.\run.cmd --visual-preflight
.\run.cmd --clippy-deep
.\run.cmd --take --watch --all
```

### Renderer festlegen und separate Diagnosewerkzeuge

Der Renderer kann über die Umgebungsvariable `ETUSCHLOPF_PDF_RENDERER`
ausdrücklich angegeben werden, zum Beispiel in der Windows-Eingabeaufforderung:

```cmd
set "ETUSCHLOPF_PDF_RENDERER=C:\Program Files\MuseScore 4\bin\MuseScore4.exe"
.\run.cmd --project "Mein Projekt"
```

Ohne diese Angabe sucht EtuSchlopf unter anderem in Windows-Registry und
Standardpfaden sowie im `PATH` nach MuseScore-/mscore-Aufrufen.

Zusätzlich enthält das Projekt separat aufrufbare Python-Diagnosewerkzeuge
für Vorabcheck, Exportvergleich und das Extrahieren von Register-/Schlüssel-
Eingaben. Ihre eigenen Argumente gehören zum jeweiligen Hilfsprogramm;
`run.cmd` startet sie nicht automatisch. Beispiele und Voraussetzungen stehen
in der [Diagnose-README](tools/assistant_diagnostics/README.md), die vollständige
Argumenthilfe liefert jeweils `python <Skriptpfad> --help`.

## Vom Ereignis zur Gestalt

Eine Notenseite ist zugleich Zeitordnung und räumliche Behauptung. Eine Pause
nimmt Dauer ein, auch wenn ihr Zeichen verschwindet. Ein Schlüssel kann eine
Linie lesbarer machen und an anderer Stelle neue Kosten erzeugen. Ein Balken
kann sichtbar zusammengehören, obwohl die Serialisierung mehrere Stimmen führt.
Eine Überbindung braucht mehr als zwei passende Tonhöhen.

EtuSchlopf behandelt solche Beziehungen ausdrücklich. Zu den bearbeiteten
Bereichen gehören unter anderem:

- Ereignis- und Quellidentität über mehrere Verarbeitungsschritte;
- Stimmen, Hände und zeitliche Belegung;
- rhythmische Darstellung, Pausen und Balkenzusammenhänge;
- Register, Schlüssel und Oktavdarstellung;
- getrennte Master- und Renderzustände;
- Layout, Renderer-Beobachtung und gebundene Ergebnisberichte.

Diese Bereiche besitzen eigene fachliche Grenzen. Eine günstigere Geometrie
begründet für sich noch keinen musikalischen Eingriff. Ein erfolgreiches
Schreiben beweist für sich noch keine zulässige Herkunft.

## Was bleiben muss

Im Quellbaum begegnen einem Begriffe wie *Occurrence*, *Lineage*, *Custody*,
*Receipt*, *Authority* und *Adoption*. Sie beschreiben verschiedene Fragen:

| Begriff | Die Frage dahinter |
| --- | --- |
| Occurrence | Welches konkrete Ereignis ist gemeint? |
| Lineage | Aus welchem Zustand ist dieser Zustand hervorgegangen? |
| Custody | Wie bleibt die Herkunft im Übergang erhalten? |
| Authority | Welche Instanz darf diese Art von Veränderung ausführen? |
| Receipt | Welcher Beleg bindet den ausgeführten Übergang? |
| Adoption | Welcher erzeugte Zustand wurde tatsächlich übernommen? |

Die genauen Bedeutungen und zulässigen Beziehungen stehen in den jeweils
zuständigen Verträgen und Implementierungen. Diese Tabelle ist ein Eingang,
keine Ersatzdefinition.

Wo ein Zusammenhang ungeklärt bleibt, muss das System mit dieser Unklarheit
umgehen. Ein unbekannter Ursprung wird durch häufiges Wiederholen nicht bekannt.
Ein Diagnosezähler wird durch seine Größe nicht zur Schreibberechtigung.

## Zeugen und Übergänge

Die Architektur trennt die Beobachtung eines Sachverhalts von der Befugnis,
aus ihm eine Änderung abzuleiten. Zwischen Vorschlag und übernommener Fassung
liegen Prüfungen und die Bindung an konkrete Zustände. Im Repository finden
sich deshalb neben musikalischer Verarbeitung zahlreiche Verträge, Audits,
Readbacks und Tests für die Grenzen dieser Verarbeitung.

Auch die Grenzen selbst sind Gegenstand der Prüfung. Ein neuer Reparaturpfad
braucht eine kausale Beschreibung seiner Fehlerklasse. Eine bekannte Etüde oder
Taktnummer kann einen Befund zeigen; sie soll die allgemeine Regel nicht ersetzen.
Synthetische Tests untersuchen den Zusammenhang. Der reale Korpus prüft seine
Breite und seine tatsächlichen Folgen.

Der erste systematische Einstieg in diese Ebene ist
[`docs/assistant_memory/000_start_here.md`](docs/assistant_memory/000_start_here.md).
Die dort beschriebene Lesereihenfolge führt zu den Architekturhinweisen und den
zuständigen Verträgen. Historische Notizen können die Suche verkürzen. Aktuelle
Quellen und gebundene Laufbelege entscheiden über den aktuellen Zustand.

## Der Maschinenraum

| Ort | Inhalt |
| --- | --- |
| `src/` | Verarbeitung, Launcher, Build-Gates und Laufmodell |
| `src/structural_write_authority/` | zuständige strukturelle Schreibpfade und Zustandsübergänge |
| `src/audit_only/` | ausdrücklich beobachtende Auswertungen |
| `tests/` | synthetische Regressionen und Architekturprüfungen |
| `docs/` | Verträge, Entwurfsbegründungen und Entwicklungsspuren |
| `tools/` | Hilfsprogramme und qualifizierte Komponenten |

Die Verzeichnisnamen geben eine erste Orientierung. Die konkrete Zuständigkeit
eines Eingriffs muss an seinem tatsächlichen Aufrufpfad geprüft werden.

## Grün ist ein bestimmtes Versprechen

Ein grüner Lauf bestätigt die eingehaltenen Prüfbedingungen. Das Ergebnis hat
Tests, Gates und die beobachteten Laufgrenzen passiert. Diese Aussage besitzt
einen definierten Umfang.

Ob eine Stelle gut zu lesen und angenehm zu spielen ist, braucht zusätzlich eine
musikalische und visuelle Bewertung. Eine formal zulässige Fassung kann weitere
Arbeit verlangen. Ein auffälliges Bild kann eine andere Ursache haben als die
zuletzt veränderte Funktion.

Darum betrachtet das Projekt mehrere Ebenen getrennt: Quellstand, Annahmebeleg,
XML-Veränderung, klingende Ereignisse, Renderer-Ergebnis und sichtbare Seite.
Sie werden miteinander verbunden, ohne einander stillschweigend zu ersetzen.

## Revisionen, Takes und Rückkehr

Der erste öffentliche Ausgangspunkt dieses Repositorys ist der akzeptierte
Quellstand **r1507**, Paketversion **0.10.1839**. Die öffentliche README, die MIT-Lizenz und die Git-Ausschlussregeln ergänzen
den importierten Quellstand. Der Git-Verlauf dokumentiert die Veröffentlichung.

Die frühere Entwicklung wird nicht als nachträglich erfundene Git-Historie
rekonstruiert. Ihre im Quellbaum enthaltenen Verträge und Begründungen bleiben
lesbar. Der öffentliche Verlauf beginnt mit diesem Import.

Daneben besitzt EtuSchlopf einen eigenen Take-Ablauf. Ein Take ist ein nach
Projektvertrag verpackter Quellstand mit deklariertem Änderungsumfang. Die
Annahme bindet sich an das konkrete Paket. Ein Git-Commit und ein akzeptierter
Take sind unterschiedliche Identitäten.

Einen solchen Take anwenden und anschließend den Vollkorpus prüfen:

```cmd
.\run.cmd --take --all
```

Ein GitHub-Quellarchiv ist allein durch seine Dateiendung noch kein Take.
`assistant_export_latest.zip` ist ein Belegpaket für die Auswertung und kein
installierbarer Take. Der Root-`Cargo.lock` wird im Git-Repository für die
Abhängigkeitsauflösung mitgeführt; die Regeln für Take-Pakete bleiben gesondert
maßgeblich.

## Wer etwas ändern möchte

Beschreibe zuerst den Zusammenhang, der verletzt wird. Zeige, welcher Zustand
betroffen ist, welche Instanz zuständig ist und woran eine korrekte Fortsetzung
erkennbar wäre. Dann kommt die Änderung.

Hilfreiche Beiträge enthalten eine nachvollziehbare Ursache, einen begrenzten
Eingriff und passende Belege. Eine Verbesserung darf an einem konkreten Fall
entdeckt werden. Ihre Regel muss auch außerhalb dieses Falls verständlich sein.

Die Prüfbedingungen im Repository gehören zum Entwurf. Ihre Abschwächung ist
selbst eine Änderung, die erklärt und beurteilt werden muss.

## Eine letzte Schwelle

EtuSchlopf ist ein gewachsenes Experiment. Es trägt Spuren seiner Versuche,
verworfene Wege in Dokumenten und präzise Grenzen an Stellen, an denen früher
etwas zu leicht möglich war.

Wer nur die Ausgabe sehen möchte, beginnt mit einem Projekt.
Wer verstehen möchte, weshalb sie so entstehen darf, folgt den Übergängen.

## Lizenz

Der eigene EtuSchlopf-Code steht unter der [MIT-Lizenz](docs/licenses/EtuSchlopf-MIT.txt).
Fremdkomponenten behalten ihre jeweiligen Lizenzen. Die Zuordnung, mitgelieferte
Lizenztexte und Grenzen der Prüfung stehen in
[Fremdlizenzhinweise](docs/THIRD_PARTY_NOTICES.md).
