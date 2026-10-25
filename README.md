# EtuSchlopf

**EtuSchlopf erzeugt aus Projektmaterial eine Klavierfassung und prüft die
zulässigen Verarbeitungsschritte bis zu MusicXML, PDF und Ergebnisbericht.**
Die Implementierung ist Rust-basiert; der gepflegte Einstieg ist der
Windows-Launcher `run.cmd`.

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
- [Dokumentierte Abnahme von r1512](docs/evidence/r1512/README.md)
- [Kaltstart ohne Gesprächsgedächtnis](docs/assistant_memory/000_start_here.md)

**Geprüfter Stand am 25.10.2026:** r1512, Paketversion 0.10.1839; 2.482 Tests
bestanden, ein Test ignoriert, Build/Clippy erfolgreich, privater Evaluationskorpus
10/10. Das sind Belege dieses konkreten nativen Laufs, keine universelle
Spielbarkeitsgarantie und kein öffentlich vollständig reproduzierbarer Korpustest.
Das Notenbild bleibt in Arbeit. Der Git-Stand enthält die angenommene r1512-Quelle
und eine gesonderte öffentliche Dokumentationsschicht; Git-Commit und Take-Annahme
sind unterschiedliche Nachweise.

*Zwischen Ereignis und Zeichen liegt eine Entscheidung.*

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

Die beiden XML-Fassungen haben verschiedene Aufgaben. Ein Unterschied zwischen
ihnen braucht einen erklärbaren Übergang. Die PDF zeigt, was der Renderer daraus
macht. Der Bericht hält fest, was der Lauf tatsächlich beobachten konnte.

## Eine Maschine in Gang setzen

Der gepflegte Einstieg ist der Windows-Launcher `run.cmd`. Das Projekt verwendet
Rust; die festgelegte Toolchain steht in
[`rust-toolchain.toml`](rust-toolchain.toml). Für die PDF-Ausgabe wird eine
passende externe MuseScore-Installation benötigt. Einige Beobachtungspfade haben
weitere Voraussetzungen, die der jeweilige Lauf prüft.

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
