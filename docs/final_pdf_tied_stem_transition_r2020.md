# r2020 · Vorher-Nachher-Abgleich gezeichneter Halsketten

Der Audit verwendet dieselben MusicXML-Quellnoten in der Ausgangsfassung und
im Kandidaten. Er fordert identische Akkord-, Balken- und Fähnchenzuordnungen
im Quellinventar. Eine Änderung an einer einzelnen Halsrichtung muss sich
als exakter Austausch des vorhandenen Stem-Elements darstellen lassen.

Der Prüfer bindet farbige Quellmarken über die eingebetteten Köpfe und
tatsächlich gezeichneten Halsstriche. Er vergleicht die normale und die
markierte Fassung jeweils auf identische Zeichenoperationen und
Schriftressourcen. Zwischen Ausgangsfassung und Kandidat behandelt er
die qualifizierten deckenden schwarzen Zeichen über ihrem unveränderten
Untergrund als Multimenge. Der Vergleich prüft dabei die tatsächlichen
Glyphenkonturen und die Lage der Zeichen. Die Nummer eines Glyphen in einer
neu geschriebenen Teilschrift kann sich dabei ändern.

Die Probe verlangt den Halsansatz in der vorgesehenen Region des eigenen
Kopfs und berechnet eine konservative Untergrenze für die Abstände zu
fremden Zeichen. Bei einem taktübergreifenden Haltebogen darf derselbe
per SVG und PDF gebundene Taktstrich in beiden Fassungen gekreuzt werden.
Die Prüfung verlangt den eindeutigen Strich zwischen den zugehörigen
Köpfen und einen positiven Schnitt der beiden Bogenkonturen mit seinem
Inneren. Weitere Taktstriche bleiben in der Abstandsprüfung.

Der Abgleich erhält für Ausgangsfassung, Einzelvorschläge und gemeinsame
Vorschau jeweils die normale und die markierte Quelle und ihr konkretes
Renderpaar. Die beiden Quellnoten sowie die Markierungen von Kopf, Hals
und Bogen werden je Gruppe übergeben. Taktordinale beginnen bei eins und
Notenordinale bei null. Die folgenden zwei Takte erzeugt die Probe selbst.

```powershell
cargo +1.98.0 test --locked --bin etuschlopf synthetic_two_measure_tied_stem_previews_are_bound_to_pdf -- --ignored --nocapture
```

`ETUSCHLOPF_PDF_RENDERER` verweist auf MuseScore. Für die spätere Ausgabe
muss derselbe Nachweis im tatsächlichen
Renderer-Lauf mit der dort gebundenen Quellfortsetzung vorliegen. Die
gemeinsame Übernahme von MusicXML und PDF benötigt Checkpoint 58.
