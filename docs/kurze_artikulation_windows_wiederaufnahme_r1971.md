# Kurze Artikulation unter Windows abnehmen

Der Windows-Lauf hat r1969 mit Chopin op. 10 Nr. 3 p01 angenommen.
Beim r1970-Versuch waren Strict Clippy, Build und die Tests grün.
Der anschließende Aufruf für p02 endete vor der Notenanalyse, weil die
MIDI-Datei fehlte. EtuSchlopf stellte daraufhin r1969 wieder her.

r1971 übernimmt die unter Windows gebaute und getestete musikalische Änderung
aus r1970 bytegleich auf den angenommenen r1969. Die Ablehnung von r1970
bleibt im lokalen Take-Verlauf dokumentiert.

## Windows-Abnahme

Das ZIP ungeöffnet im Projektordner oder unter Downloads ablegen. Im
Projektordner das vorhandene Paar `Chopin_op10_no3_p01.mid` und
`Chopin_op10_no3_p01.wav` prüfen und den Take damit starten.

```cmd
.\run.cmd --take "Chopin_op10_no3_p01"
```

Für die Gegenprobe von p02 müssen `Chopin_op10_no3_p02.mid` und
`Chopin_op10_no3_p02.wav` derselben Aufnahme im Projektordner liegen.
Nach erfolgreicher Annahme von r1971 startet diese Gegenprobe mit dem
bereits installierten Stand.

```cmd
.\run.cmd "Chopin_op10_no3_p02"
```

Der Vollkorpus ist ein eigener Lauf mit `run.cmd --all`.
