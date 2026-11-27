# r2017 · Halskettenbeleg und Prüfung des Änderungsprotokolls

Der Take baut auf dem angenommenen r2015 auf. Er enthält die Halskettenprobe
aus r2016 und ergänzt die Fehlerausgabe der vorgelagerten Arrangementsuche.
Die Prüfung verlangt weiterhin, dass die aufgezeichneten Änderungen die
tatsächlich erzeugte MusicXML-Fassung exakt ergeben.

Schlägt diese Prüfung fehl, nennt der Export den Verarbeitungsschritt
`NotationPolish` oder `PianistLayout`, den Schritt im Änderungsprotokoll, die
Anzahl der aufgezeichneten Operationen und die erste abweichende Byteposition.
Der vorhandene Fehlerexport übernimmt diese Angaben.

Der gezielte Windows-Lauf verwendet ein Projekt und dieselbe Zahl an
Solver-Workern wie der fehlgeschlagene Kindlauf. Die neue Halskettenprüfung
meldet Vorschläge. Die produktive Freigabe der Korrekturen braucht den
noch ausstehenden Nachweis im endgültigen PDF.
