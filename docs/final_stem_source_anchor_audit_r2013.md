# Synthetischer Quellanker für Hals, Kopf und Haltebogen

EtuSchlopf r2013 folgt auf r2012. Die Probe untersucht, welche
Zeichenbindung bei zwei getrennten MuseScore-Importen aus MusicXML erhalten
bleibt. Sie arbeitet im Testpfad mit einer synthetischen Zweitaktpartitur.

## Was die Probe belegt

Die Vorlage vergibt getrennte Farbmarken für Kopf und Hals, einschließlich
beider Köpfe an den Enden des Haltebogens. Der Bogen erhält seine Marke am
Start. MuseScore importiert beide Varianten unabhängig voneinander. Sie haben
unterschiedliche Taktnummern und entgegengesetzte Richtungen des unteren Halses.
Die Probe liest die
Farben und ihre Rollen im nativen Format zurück. Sie prüft die
Zugehörigkeit von Kopf und Hals zum selben Akkord und die beiden Enden
des einzigen gebundenen Bogens.

Pro Variante belegt die Probe, dass die Markierung im direkten SVG nur
die Farbe ändert. Ein weiteres SVG aus der nativen Fassung muss dieselben
Marken mit den schon für den SVG-Readback geltenden Koordinatenreserven
an denselben Zeichen zeigen. Erst dann vergleicht sie über die getrennten
Importe hinweg den einen eindeutig gebundenen unteren Hals. Die Mitte seiner
gezeichneten Kontur muss relativ zur Kopfmitte einmal darunter und einmal darüber
liegen.

Die Standardprobe verweigert fehlende, doppelte und an die falsche
native Zeichenart gebundene Farbmarken. Die reale MuseScore-Probe lässt
sich mit `ETUSCHLOPF_PDF_RENDERER` und der folgenden Anweisung zusätzlich
in der lokalen Installation ausführen.

```powershell
cargo test --locked --bin etuschlopf musicxml_stem_source_anchor_across_two_imports -- --ignored --nocapture
```

## Reichweite

Die Farben sind experimentelle Quellanker. Die nativen Objekt-IDs eines
separaten Imports tragen hier keine Identität zwischen Fassungen. Geteilte
Hälse und in mehrere SVG-Segmente zerlegte Bögen brauchen eine eigene
Zuordnungsregel. Auch die reguläre Quellprovenienz und die Zeichenbindung im
tatsächlich ausgegebenen PDF brauchen einen eigenen Nachweis. Die Probe
liefert dafür eine beobachtete Schnittstelle und erteilt keine
Schreibbefugnis.
