# r282 – Registernotation: Vier-Domaenen- und Sichtzustandspfad-Beweis

## Zweck

r282 ist ein ausschliesslich schreibfreier Architekturbeweis auf dem real installierten r281. Er reagiert auf die menschliche Falsifikation der r277-Partitur und auf die anschliessende Literatur- und Architekturpruefung. Keine MusicXML- oder PDF-Datei wird mutiert. Es gibt keine neue Oktavierungsheuristik, keine Takt-/Etuedensonderlogik, keine Akkordreparatur und keine StructuralWriteAuthority.

## Fachliche Korrektur gegenueber r281

r281 hatte einen vorhandenen MusicXML-Akkorddauerueberhang an einer Sichtgrenze als Projektionsblocker behandelt. Diese Kopplung ist zu stark. Ein laenger klingendes, unsichtbares Akkordmitglied bleibt eine Quellkonformitaetsbeobachtung, bestimmt aber nicht automatisch die rhythmische Geltungsgrenze oder die grafische Endposition einer Oktavlinie.

r282 trennt daher zwei Wahrheiten:

- **rhythmisches Ende des notierten Ereignisses**: das Ende des cursorbewegenden Hauptnotenereignisses;
- **klingender Akkordnachlauf**: das maximale klingende Ende aller zur Akkordgruppe gehoerenden Tonobjekte.

Der zweite Wert bleibt sichtbar und wird nicht repariert. Er ist jedoch kein automatisches Oktav-Schreibveto mehr.

## Vier Fachdomänen

### 1. Logik

Die Logikdomaene besitzt ausschliesslich die Frage, welche Ereignisse eine Registerdarstellung benoetigen und welche Darstellung (`8va`, `15ma`, `8vb`, `15mb`) semantisch vorgesehen ist. Grundlage bleibt der vorhandene `RegisterNotationPlan`. Die Domaene kennt keine grafische Y-Lage, keine MusicXML-`number`-Kennung und keinen Renderer.

### 2. Geltung

Die Geltungsdomaene bestimmt, an welchen rhythmischen Ereignissen eine Registerdarstellung beginnt und endet. Ereignisanker werden bevorzugt. Ein laenger klingender unsichtbarer Akkordnachlauf darf die rhythmische Endposition nicht heimlich verschieben. Zeitanker sind nur eine spaetere Ausweichmoeglichkeit, falls ein echter Ereignisanker fehlt; r282 plant fuer das aktuelle Falsifikationsstueck keinen solchen Ausweichanker.

### 3. Sicht

Die Sichtdomaene bestimmt, welche semantischen Teilplaene als eine durchgehende sichtbare Oktavlinie erscheinen. Exakter Kontakt gleicher Darstellung darf sicher fusioniert werden. Darstellungswechsel am Kontakt und stille Luecken bleiben offene musikalisch-notatorische Entscheidungen. Die Domaene darf keine Etueden- oder Taktkennung kennen. Die grafische Grundinformation besteht nur aus Seite und konfliktfreier Spur; eine numerische `default-y`-Position wird nicht vorentschieden.

### 4. Projektion

Die Projektion bildet eine bereits feststehende sichtbare Oktavspanne auf MusicXML ab. Sie besitzt kein musikalisches Entscheidungsrecht. Liniennummern entstehen aus realer Ueberlappung gleichzeitig aktiver Linien und nicht aus der Systemnummer. Die Projektion darf Quellkonformitaetsbefunde nicht als versteckte Musikregel umdeuten.

## Literaturbezug

Die Architektur folgt nicht einer einzelnen Quelle, sondern der Konvergenz mehrerer Fachrichtungen:

- Elaine Gould, *Behind Bars*: Oktavlinien sind Lesbarkeits- und Gruppierungsnotation; unnoetige Wechsel und unruhige Linienfuehrung sind zu vermeiden.
- MEI Guidelines 5.1: logische, visuelle, gestische und analytische Informationen werden getrennt modelliert; Kontrollereignisse koennen an Ereignis-IDs oder Zeitpositionen gebunden werden.
- MusicXML 4.0: `octave-shift`, `offset`, grafische Attribute und `number` besitzen getrennte technische Bedeutungen; `number` dient der Unterscheidung ueberlappender Linien. Akkordton-Dauern besitzen einen eigenen Quellkonformitaetsvertrag.
- Dokumentation externer Notensatzsoftware: rhythmische Geltung einer Oktavlinie und deren grafische Position/Laenge koennen getrennt bearbeitet werden.
- Aiba/Sakaguchi: geuebte Pianisten lesen Registerlagen nicht nur ueber einzelne Hilfslinien, sondern auch ueber geometrische Muster; ein grosser Intervallsprung ist daher keine automatische Sichtgrenze.

## Etuede 6 als Falsifikationsstueck

Etüde 6 bleibt in dieser Phase das einzige Falsifikationsstueck. Diese Auswahl beeinflusst keine Produktivregel. Ein Korpuslauf ist keine Voraussetzung fuer r282.

Aus der realen r281-Datei ist vorab zu erwarten:

- 22 semantische Registerplaene;
- 4 sichere Kontaktfusionen;
- 18 Sichtspannen;
- 18 rhythmische Startanker;
- 18 rhythmische Stopanker;
- 18 exakte rhythmische Dokumentbereiche;
- 24 vorhandene Akkordgruppen mit Dauerueberhang;
- 1 Sichtgrenze mit gleichzeitigem klingendem Akkordnachlauf;
- 0 technische Sichtschreibblocker nach Trennung von Geltung und Quellkonformitaet;
- 3 weiterhin offene musikalische Sichtentscheidungen: 1 Darstellungswechsel am Kontakt und 2 stille Kontinuitaetskandidaten.

Diese drei offenen Entscheidungen sind kein technischer Fehler und duerfen in r282 nicht automatisch geloest werden.

## Stopregel

r282 ist abgeschlossen, wenn der reale Einzelstuecklauf den Vier-Domaenen-Vertrag technisch schliesst, den Quellkonformitaetsbefund separat erhaelt und die offenen musikalischen Sichtentscheidungen ausdruecklich offen laesst. Erst der folgende Thread entscheidet den schreibfreien Zielbildbeweis und spaeter die MusicMutation.
