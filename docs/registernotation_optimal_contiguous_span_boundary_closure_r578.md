# r578 – RegisterNotationOptimalContiguousSpanBoundaryClosure

## Autoritativer Ausgangspunkt

r578 basiert direkt auf dem real ausgefuehrten und dunkelgruenen r577
`RegisterNotationLocalReadabilitySpanBoundaryClosure` aus dem aktuellen
Assistant-Export. r577 bleibt damit Projektgeschichte; dieser Take rollt nicht
auf r576 zurueck.

SchemaSchlopf r020, die drei gebundenen MusicXML-3.1-XSDs und das
Persisted-Byte-XSD-Gate bleiben byteunveraendert.

## Nach r577 neu bewerteter Befund

r577 hat die richtige Fehlerklasse getroffen, aber zwei zusaetzliche
Mechanismen eingefuehrt, die fuer die beobachteten Oktavierungsgrenzen nicht
notwendig sind:

1. Weighted-Interval-Scheduling kann innerhalb eines einzigen bereits
   qualifizierten Evidenzsegments mehrere voneinander getrennte Oktavspannen
   erzeugen.
2. Die `1..3`-Ereignis-Randinsel-Kontinuitaet ist eine eigene Heuristik, die
   isolierte Extremtoene gegen die bestehende Mindestmengenregel in eine
   Nachbarspanne hineinprojizieren kann.

Nach dem Literaturabgleich wird beides pensioniert. Einzelne hohe oder tiefe
Toene duerfen weiterhin loco notiert bleiben; eine Oktavlinie wird nicht allein
wegen einer kurzen Extreminsel erzwungen.

## Einzige neue produktive Regel

Fuer jedes bereits von `evidenz_segmente()` gebildete Segment wird hoechstens
**ein** zusammenhaengendes Oktavfenster gewaehlt.

Auf den bereits in r577 eingefuehrten sicheren Angriffsschnitten werden alle
zusammenhaengenden Fenster `[a,b]` betrachtet. Ein Schnitt bleibt nur zulaessig,
wenn ein neuer Angriff beginnt und kein vorher gestartetes Ereignis in den
neuen Angriff hineinragt. Gleichzeitige Angriffe/Akkordflaechen und sustainende
Texturen werden daher nicht durch eine neue Oktavgrenze zerschnitten.

Fuer jedes Fenster und jede bereits vorhandene Darstellung `8va`, `15ma`,
`8vb`, `15mb` gilt unveraendert:

`Vorteil = Loco-Lesbarkeitswert - Darstellungs-Lesbarkeitswert`.

Die vorhandene 15ma/15mb-Bezeichnungsstrafe ist bereits Teil des
Darstellungswerts. Nur Fenster, die die unveraenderte
`VERBESSERUNGSSCHWELLE` erfuellen, sind Kandidaten.

Gewonnen hat genau das zusammenhaengende Fenster mit dem groessten vorhandenen
Lesbarkeitsvorteil. Damit wird nur die Randwahl der schon vorhandenen
Registerentscheidung verfeinert; es entsteht kein zweiter Register-Classifier.

## Tie-Break ohne neue musikalische Kosten

Nur bei exakt gleichem vorhandenen Lesbarkeitsvorteil wird deterministisch
gebrochen. Diese Tie-Breaks veraendern die musikalische Kostenfunktion nicht:

1. kleinere bereits vorhandene Oktavbezeichnung vor groesserer (`8` vor `15`),
2. vorhandene Segment-/Grenzenevidenz vor einem bloss sicheren Angriffsschnitt,
3. groessere strukturelle Abdeckung,
4. stabile fruehere Position.

Die Grenzenevidenz wird nur im exakten Gleichstand gelesen. Es gibt keinen
neuen Phrase-Classifier und keine neue Phrase-, Takt-, Beat- oder
Hilfsliniengewichtung.

## Explizit pensioniert

r578 entfernt aus dem produktiven r577-Pfad:

- Mehrfachauswahl nichtueberlappender lokaler Oktavfenster innerhalb desselben
  Evidenzsegments,
- `KurzeRandinselKontinuitaet`,
- die kuenstliche Mindestmengen-Erfuellung fuer `1..3` Randereignisse.

Die bereits vor r577 vorhandene `KlingendeZwischeninselKontinuitaet` bleibt
unveraendert; sie ist nicht Gegenstand dieses Takes.

## Unveraenderte Invarianten

Unveraendert bleiben insbesondere:

- `MINDEST_TONOBJEKTE = 4`,
- `VERBESSERUNGSSCHWELLE = 8`,
- `DOPPELOKTAV_BEZEICHNUNGSSTRAFE = 24`,
- obere/untere Lesbarkeitszonen,
- Unteres-System-Schluesselvorrang,
- Staff-, Voice-, Onset-, Dauer- und klingende Pitch-Entscheidungen,
- Clef-Owner und Pass-Reihenfolge,
- StructuralWriteAuthority und Writer,
- SchemaSchlopf und XSD-Authority.

Es gibt keine Etueden-, Takt-, Pitch- oder Positionssonderregel.

## Regressionen

Die Unit-Regressions pruefen allgemein:

1. eine hohe Insel kann aus loco-Raendern als ein zusammenhaengendes Fenster
   ausgeschnitten werden,
2. ein durchgehend hoher Abschnitt bleibt eine einzige ganze Spanne,
3. zwei getrennte hohe Inseln innerhalb eines Evidenzsegments duerfen nicht in
   zwei lokale Oktavlinien zerfallen; es wird genau die bessere Insel gewaehlt,
4. zwei isolierte Extremtoene erhalten keine Randinsel-Sonderautoritaet,
5. ein hineinragender klingender Ton sperrt den lokalen Schnitt.

Die menschlich beobachteten Stellen in Etuede Nr. 6 (T13, T15, T25-27, T32,
T35) bleiben ausschliesslich PDF-Falsifikationsadressen und erscheinen nicht im
Produktivcode.

## Fertigkeitsgrenze

Nach Installation wird zuerst Etuede Nr. 6 gerendert und an den genannten
Stellen visuell geprueft. Nur wenn diese Stellen plausibel sind, folgt der
normale 10er-Korpuslauf. Sind die sechs Befunde geschlossen und entstehen im
Korpus keine neuen Oktavierungsregressionen, ist dieses Thema fertig; es wird
kein weiterer Oktavierungs-Take aus Gewohnheit geplant.
