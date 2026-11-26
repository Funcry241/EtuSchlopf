# r2014 · PDF-Beleg für Hals, Kopf und Haltebogen

Der Take baut auf dem unter Windows angenommenen r2013 „Quellanker“ auf.
Der Export enthält die Annahme für das angebotene Quellpaket
`aee0b11ec623d456d75fc9419e49fdd05f27e0cc2a8c6d6c29eeda3e46a19511`,
Exit 0 und Ergebnisse für alle 15 Korpusprojekte. Die folgenden
Beobachtungen verwenden ausschließlich synthetisch erzeugte Noten.

## Halsstriche und Bogenpfade

Die MuseScore-Probe importiert zwei MusicXML-Fassungen getrennt. Sie
verfolgt die Quellmarken durch MSCX und SVG bis ins PDF. Pro Fassung
vergleicht sie den regulären Export mit dem markierten Schattenexport.
Die unterstützten PDF-Zeichenanweisungen und ihre Operanden stimmen
bis auf die Farbe exakt überein. SVG-Abmessungen in Millimetern bestimmen
die physikalische Skalierung. Der PDF-Nachweis prüft die drei Halsstriche
und die gefüllte und gestrichene, geschlossene Bézierkontur des
Haltebogens jeweils mit ihrer Strichbreite und ihrem Stil. Fremde
Zeichenoperatoren, mehrdeutige Konturen, veränderte Kurven bei identischen
Endpunkten und gleicher Außenhülle sowie Beschneidung verweigern den Beleg.

```powershell
cargo test --locked --bin etuschlopf musicxml_stem_source_anchor_across_two_imports -- --ignored --nocapture
```

`ETUSCHLOPF_PDF_RENDERER` bezeichnet für diese zusätzliche Probe die
lokale MuseScore-Programmdatei.

## Kontur des tatsächlich eingebetteten Notenkopfs

Die separate PDF-Prüfung erzeugt reguläre und markierte Ausgaben für
zwei weitere künstliche Notensituationen. Sie verfolgt den PDF-Zeichencode
über die Fontressource bis zur eingebetteten TrueType-Glyphe und ihrer
Transformation. Die symmetrische Konturprüfung berücksichtigt, dass
MuseScore SVG und PDF mit verschiedenen Kurvensegmenten erzeugt. Die
beiden Proben banden unter MuseScore 4.7.5 je zwölf Notenköpfe. Geänderte
Glyphen mit gleicher Breite, geänderte eingebettete Konturen bei gleichem
Zeichencode, Verschiebungen und ein Clip durch den Kopf verweigerten die
Zuordnung.

```powershell
py docs/evidence/engraving-pdf/r2014/run_heads.py "C:\Pfad\zu\MuseScore4.exe"
```

Die optionale Kopfprobe verwendet Python mit `pypdf 6.21.4` und
`fonttools 4.61.1`. Die Befehle erzeugen die Test-PDFs in temporären
Verzeichnissen. EtuSchlopf erhält keine neue Laufzeitabhängigkeit.

## Trockenlauf und Grenze des Belegs

Die Rust-Tests prüfen die Übergangsregel mit synthetisch vorgegebenen
Quellbindungen, Erhaltungsbelegen und betroffenen Paaren. Sie decken
unabhängige und gekoppelte Gruppen, unklare Abstände, exakt unveränderte
Konturen und einen abweichenden Schlussbeleg ab. Der einmal gewählte
Teilzustand wird bestätigt oder verworfen. Ein abweichender Schlussbeleg
führt zur zuvor belegten Ausgangsfassung.

Die Prüfreserven für diese geometrischen Vergleiche gelten nur für die
Proben. Musikalische Mindestabstände je Paarklasse und die vollständige
Quellprovenienz für reguläre Ausgaben bleiben zu belegen. Dazu gehören
mehrseitige Zuordnungen, geteilte Zeichen sowie die Erfassung aller
betroffenen Paare. Die Prüfungen dieses Takes laufen im Testpfad.
Für eine produktive Halskorrektur braucht
der bestehende Verantwortliche einen quellengebundenen Beleg für die
tatsächlich ausgegebene Fassung, nachdem auch spätere Layoutänderungen
wirksam geworden sind.
