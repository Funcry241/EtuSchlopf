# Synthetische Geometrieprobe für Hals, Kopf und Haltebogen

EtuSchlopf r2012 basiert auf dem unter Windows angenommenen r2011. Der Take
prüft einen Teil des geometrischen Nachweises aus dem [Reparaturbereich für Haltebogenketten](https://github.com/Funcry241/EtuSchlopf/blob/bdd827716c8e5af1f806b3d647b8f8e118933db9/docs/final_stem_tie_chain_repair_scope.md).
Die neuen Proben laufen im Testpfad.

## Synthetische Abstände und eindeutige Zuordnung

Die SVG-Probe vergleicht einen Hals mit einem fremden Notenkopf und mit einem
fremden Haltebogen. Sie unterscheidet einen belegten Abstand, belegte
Überdeckung und einen ungeklärten Abstand. Die bestehenden geometrischen
Reserven gehen in den Abstandsbeleg ein. Der Prüfwert von einer SVG-Einheit
dient diesen synthetischen Beispielen. Die Probe setzt damit keinen
musikalischen Mindestabstand für eine Paarklasse fest.

Ein Zeichen muss in der gemalten und ungemalten Fassung desselben SVGs bei
unveränderter Geometrie eindeutig gebunden sein. Fehlende Farbmarkierung,
doppelte Zuordnung und eine Geometrieänderung durch die Markierung bleiben
ungeklärt. Die Tests enthalten außerdem Überdeckung, Berührung und einen
Abstand genau an der Prüfgrenze.

## Reale MuseScore-Probe

Die MuseScore-Probe importiert eine synthetische zweisystemige Partitur
einmal. Aus diesem nativen Ausgang erzeugt sie eine Fassung, in der sich
allein die Länge eines Halses ändert. Je zwei ungemalte und gemalte SVGs
zeigen, ob sich dieser Hals sowie der fremde Kopf und Haltebogen in beiden
Fassungen genau einmal zuordnen lassen. Sie belegen den Abstand zu beiden
fremden Zeichen in dieser Vorlage. Die Bindung gilt für diesen gemeinsamen
nativen Ursprung. Für getrennte Importe fehlt weiterhin die Bindung an
mitgeführte Quellprovenienz.

Die Standardtests laufen bei der lokalen Abnahme über
`run.cmd --take --all`. Für die zusätzliche MuseScore-Probe setzt man
`ETUSCHLOPF_PDF_RENDERER` auf die ausführbare MuseScore-Datei und startet
im installierten Quellverzeichnis

```powershell
cargo test --locked --bin etuschlopf native_stem_pair_audit_across_two_svgs -- --ignored --nocapture
```

## Reichweite

Die synthetischen SVGs tragen noch keinen Freigabebeleg für den Kopf oder
Haltebogen im tatsächlich ausgegebenen PDF. Für die Übernahme einer
Halsänderung fehlen außerdem der fassungsübergreifende Quellanker in
regulären Noten, qualifizierte Abstände je Paarklasse und die Prüfung der
konkret übernommenen Konfliktgruppen.
