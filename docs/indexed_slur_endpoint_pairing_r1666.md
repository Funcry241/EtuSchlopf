# r1666 – Gleiche Bogenpaarung mit lokalem Index

Parent: nativ angenommener r1665 / 0.10.1839. ChangeClass: RefactorOnly.

## Änderung

`remove_orphan_slur_and_tie_endpoints` suchte für jeden sichtbaren
Slur-Anfang erneut in allen späteren Noten nach einem sichtbaren Schluss
gleicher Nummer. Für jeden Schluss folgte die entsprechende Rückwärtssuche.
Jetzt bildet der bestehende Owner einen lokalen, rein lesenden Index aus denselben unveränderlichen
`NotationEndpointNote`-Beobachtungen den ersten sichtbaren Anfang und den
letzten sichtbaren Schluss je Nummer. Pausen und unsichtbare Noten tragen
weiterhin keinen Paarungsbeleg bei.

Die Abfragen bleiben exakt dieselben:

- Ein späterer Schluss existiert genau dann, wenn der letzte Schlussindex
  dieser Nummer strikt größer als der aktuelle Notenindex ist.
- Ein früherer Anfang existiert genau dann, wenn der erste Anfangsindex
  dieser Nummer strikt kleiner als der aktuelle Notenindex ist.

Endpunkte derselben Note stützen sich damit weiterhin nicht gegenseitig.
Doppelte Endpunkte und mehrere voneinander getrennte Vorkommen ändern diese
Existenzäquivalenz nicht. System und Stimme werden für Slurs weiterhin nicht
als zusätzliche Filter verwendet. Die gesonderte lokale `tied`-Paarung bleibt
bytegleich. Auch Nummernnormalisierung und XML-Leser bleiben unverändert.

Der Index leiht seine Schlüssel aus genau einer unveränderlichen Beobachtung.
Er besitzt keinen Hash-, XML-, Projekt- oder scoreübergreifenden Cache, keinen
Writer und keine musikalische Autorität. Der vorhandene Owner erzeugt dieselben
Löschbereiche und Originalbelege. Seine Aufrufe und die finale Polierschleife
bleiben unverändert.

## Beweis

Die vorhandene Vollsuche ist als unabhängige Testreferenz erhalten.
5.832 synthetische Drei-Noten-Folgen kombinieren sechs Endpunktmuster mit
sichtbaren, unsichtbaren und Pausennoten. Sie prüfen die tatsächlichen
Löschzahlen und Ergebnisbytes des ursprünglichen Owners gegen die alten
Vollsuchen. Die Probe variiert System und Stimme und wiederholt den Vergleich
nach Präfixverschiebung mit einer unabhängigen Nummer. Zusätzlich muss der
unabhängige Präfix die vorhandene Paarung unverändert lassen. Endpunkte
derselben Note, doppelte Endpunkte und fehlende Gegenstücke sind enthalten.
Leere Eingaben bleiben unverändert.

Eine zweite Probe bindet Nummernnormalisierung, XML-Kommentare und weitere
sichtbare/unsichtbare/Pausen-Endpunkte an denselben tatsächlichen Leser.
Klingende `tie`-Elemente müssen in beiden Proben erhalten bleiben. Die
bestehenden Custody-/Master-Adoptionstests prüfen weiterhin die ursprünglichen
Belege und die Verweigerung bei deren Verlust.

## Aufwand und native Grenze

Bei N beobachteten Noten, S Slur-Endpunkten und K Nummern kostet der Index
O(N + S log(K + 1)) Arbeit und O(K) zusätzlichen Speicher; die darauf folgenden
Abfragen kosten jeweils O(log(K + 1)). Wiederholte suffix-/prefixweite Suchen
für jeden Slur-Endpunkt entfallen. Ohne Slur-Endpunkte entstehen keine
Map-Einträge; eine native Laufzeitverbesserung wird nicht vorweggenommen.

Native Abnahme: `run.cmd --take --all`. Erwartet werden 20 bytegleiche
Master-/Render-XML-Dateien und pixelgleiche PDFs gegenüber r1665.
