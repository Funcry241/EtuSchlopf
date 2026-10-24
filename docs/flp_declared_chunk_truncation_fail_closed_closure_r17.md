# FlpDeclaredChunkTruncationFailClosedClosure r17

## Anlass

Die read-only Correctness-Risk-Triage nach r16 hat den produktiven FLP-Eingang auf
seltene Panic-, Grenz- und Integritaetspfade geprueft. Dabei wurde in `read_flp_notes`
ein echter fail-open Fall gefunden: Die deklarierte `FLdt`-Chunklaenge wurde bislang
mit der tatsaechlichen Dateilaenge abgeschnitten (`min`). Endete eine beschaedigte oder
unvollstaendig kopierte FLP-Datei zufaellig exakt an einer vollstaendigen Eventgrenze,
konnte der Reader deshalb die vorhandenen Events erfolgreich uebernehmen, obwohl der
`FLdt`-Header mehr Nutzdaten deklarierte als physisch vorhanden waren.

Eine Trunkierung mitten in einem Event wurde bereits durch `take_payload` abgewiesen.
Die Luecke bestand ausschliesslich fuer Trunkierungen an einer syntaktisch sauberen
Eventgrenze.

## Aenderung

`read_flp_notes` behandelt die deklarierte `FLdt`-Laenge nun als Integritaetsvertrag:

- die 32-Bit-Chunklaenge muss lesbar sein,
- `body_start + declared_size` wird mit `checked_add` gebildet,
- liegt das deklarierte Chunkende hinter dem physischen Dateiende, wird der Import
  fail-closed mit `FLdt-Daten abgeschnitten` abgebrochen,
- nur ein vollstaendig vorhandener deklarierter Chunk wird geparst.

Es gibt keine Reparatur, kein Auffuellen fehlender Bytes und keine partielle
Best-Effort-Uebernahme.

## Regression

Zwei fokussierte Unit-Tests binden die Grenze:

1. Ein minimaler vollstaendiger `FLdt`-Chunk mit genau einem 24-Byte-Note-Event bleibt
   akzeptiert.
2. Derselbe physische Inhalt mit um genau ein Byte groesser deklarierter `FLdt`-Laenge
   wird abgewiesen, obwohl die vorhandenen Bytes exakt an der vollstaendigen
   Eventgrenze enden. Damit ist der historische fail-open Fall direkt gebunden.

## Semantik

Der normale FLP-Pfad wird nicht neu interpretiert. Fuer jede vollstaendige Quelle ist
die Parserlogik hinter der Chunkgrenze bytegleich. Geaendert wird ausschliesslich das
Verhalten fuer strukturell unvollstaendige FLP-Container: vorher konnte ein Prefix als
vollstaendige Musikquelle weiterlaufen, jetzt wird die Quelle verworfen.

Der Take wird bewusst als `MusicMutation` deklariert, weil die FLP-Datei die
produktive musikalische Quelle ist und eine neue Ablehnungsgrenze an dieser Quelle
nicht als `RefactorOnly` beschriftet werden darf, auch wenn kein gueltiger Korpusfall
musikalisch veraendert werden soll.

Keine Aenderung an WAV, Solverprofilen, Handverteilung, Notation, Layout, Writer-
Authority, Renderer, Security-Gates oder Release-Policy.

## Abnahme

Zuerst ein bekannter gruener Einzellauf zum transaktionalen Commit:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Danach der Vollkorpus ohne `--take`:

```powershell
.\run.cmd --all
```

Erwartung fuer den Vollkorpus: 10/10 dark_green; fuer alle gueltigen Korpus-FLPs
identische MusicXML-/Render-MusicXML- und visuelle PDF-Artefakte gegen r16.
