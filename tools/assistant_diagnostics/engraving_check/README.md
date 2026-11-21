# Gegenprobe zu Notenhälsen und Balken

`compare_engraving.py` liest aus einem Exportverzeichnis oder ZIP gleichnamige Paare aus PDF und `.render.musicxml`. Die Auswertung verwendet das Render-MusicXML für die Reihenfolge der Seiten, Systeme und Takte. Sie gewinnt Hälse, Balkensegmente, Notenkopfgrenzen und Fähnchen aus dem PDF. Die Eingaben bleiben unverändert.

Voraussetzungen sind Python 3.10 oder neuer, PyMuPDF und fontTools. Führe die Befehle in diesem Verzeichnis aus.

```shell
python -m unittest -v test_geometry
python compare_engraving.py BASIS.zip --json-out basis.json
python compare_engraving.py BASIS.zip KANDIDAT.zip --json-out vergleich.json
```

Die JSON-Ausgabe enthält die Messwerte pro Takt und die Namen der vergleichbaren sowie fehlenden oder hinzugekommenen Partituren. Beim Vergleich müssen die PDF- und Render-XML-Paare denselben Dateistamm haben und die aufeinander abgebildeten Partituren dieselbe Taktfolge besitzen. Seitenumbrüche dürfen sich ändern. Die Analyse eines einzelnen Kandidatenwerks ist damit ebenfalls möglich.

## Was die Zahlen belegen

- Eine bestätigte Fremdkopf-Kollision erfordert ein Balkenband über der Mitte eines gefüllten Notenkopfs, zwei am Balken erkannte Hälse und eine eindeutige, davon getrennte Halszuordnung des Kopfs. Unklare Zuordnung bleibt als eigener Befund sichtbar. Der Detailwert enthält die Kopf- und Balkengeometrie und die dafür zugeordneten Hälse.
- Eine physische Balkengruppe umfasst die über Balkensegmente zusammenhängenden Hälse. Ein kurzer zweiter Balken innerhalb derselben Gruppe zählt als Segment und erhöht die Gruppenzahl nicht.
- Die Halslänge bezeichnet den erkannten vertikalen Strich in Notenlinien-Zwischenräumen. Ein großer Wert dient zum Vergleich der Halsgeometrie.
- Der Taktvergleich führt Änderungen der physischen Gruppenzahl, der Fähnchen, der maximalen Halslänge und der Kollisionszahl auf. Gleichbleibende Gruppenzahlen garantieren keinen Erhalt konkreter musikalischer Verbindungen. Das erfordert die paarweise Rücklesung mit gebundener Notenidentität am bestehenden Owner.

Der Profilbereich sind Vektor-PDFs mit zwei Klaviersystemen pro Systemzeile, eingebettetem Leland und zur tatsächlichen Ausgabe passenden Umbruchmarken im MusicXML. Die Kollisionsprüfung erfasst zentral überdruckte gefüllte Köpfe. Randberührungen, hohle Köpfe, andere Glyphentypen und unerkannte Vektorformen brauchen eine eigene Prüfung. Die form- und maßbezogenen Filter gehören zur PDF-Auswertung.

## Gemessene r1971-Basis

In 15 PDFs sind alle 528 Taktfelder auf 59 Seiten zugeordnet. Die Auswertung reproduziert die drei im Prüfbericht sichtgeprüften Fremdkopf-Kollisionen. Sie erkennt 5.522 Balkensegmente, 2.784 physisch verbundene Balkengruppen und 522 Fähnchen mit erkennbaren Halsanschlüssen. Der längste erkennbare Hals misst 11,677 Zwischenräume. Beim Vergleich des identischen Exports ergeben sich null geänderte Takte.

Der historische Vergleich umfasst 14 gemeinsame Stücke mit 505 Takten in r1968. Auf diesen Stücken steigen die Fremdkopf-Kollisionen von null auf zwei. Die dritte Kollision liegt in der mit r1971 hinzugekommenen Einspielung. In der separaten MuseScore-Eingriffsprobe für die zwei Einspielungen sinkt die Zahl über 45 Takte von drei auf null. Die Zahlen für erkennbare Gruppen, Segmente und Fähnchen bleiben gleich. Die Eingriffsprobe überprüft den Effekt der geänderten Halsrichtung.
