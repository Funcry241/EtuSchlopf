# Register-Sichtschreibauftrag-Vertragsbeweis r280

## Zweck

r280 schliesst die zwischen r262/r277 fehlende technische Fachgrenze **vor** einem erneuten sichtbaren Schreibversuch. Der Take bleibt vollstaendig schreibfrei. Er beantwortet nicht erneut die Frage, welche Oktavart musikalisch gewaehlt wird, sondern ob die bereits feststehenden sichtbaren Spannen ueber einen consumer-robusten, ereignisgebundenen MusicXML-Auftrag ausdrueckbar sind.

## Harte Trennung

```text
RegisterNotationPlan
  -> Sichtzustandsprojektion
  -> Sichtschreibauftrag
       - Start-Tonanker
       - Stop-Tonanker
       - exakter Dokument-Scope
       - Liniennummer als Ueberlappungsidentitaet
       - Seite und Konfliktspur
       - kein explizites default-y
  -> erst spaeter StructuralWriteAuthority
```

## Ereignisanker

Der Startanker ist die frueheste reale Tonereignisgruppe desselben Systems am exakten Beginn der Sichtspanne, der Stopanker die spaeteste Tonereignisgruppe am exakten Ende. Akkordtoene werden als eine Dokumentgruppe behandelt, damit eine spaetere Richtung niemals einen Akkord zertrennt. Der Dokumentcursor vor dem Startanker und nach dem Stopanker muss exakt der semantischen Grenzzeit entsprechen.

Zusaetzlich muss die Menge der Tonereignisgruppen desselben Systems zwischen beiden Dokumentankern exakt der semantisch von der Sichtspanne umfassten Menge entsprechen. Backup-/Voice-Verschachtelungen koennen diese Bedingung falsifizieren; dann bleibt der Writer geschlossen. Verdeckte Tonereignisse gehoeren zur Scope-Gleichheit, sichtbare Grenztoene werden separat als Voraussetzung fuer die sichtbare Ankerlage geprueft.

## Linienidentitaet

`octave-shift number` ist keine Staff-/Systemkennung. r280 plant fuer jede gleichzeitig aktive Sichtspanne die kleinste freie Liniennummer 1..16 und gibt die Nummer nach dem Ende wieder frei. Exakt aneinandergrenzende Spannen duerfen dieselbe Nummer wiederverwenden; echte Ueberlappung erfordert verschiedene Nummern.

## Geometrie

8va/15ma liegen auf der Oberseite, 8vb/15mb auf der Unterseite. Fuer gleichzeitig aktive Linien derselben Seite wird eine konfliktfreie Spur geplant. r280 plant bewusst **keine numerische Y-Koordinate**. Damit kann ein spaeterer Renderer zunaechst seine eigene Platzierung verwenden; falls spaeter eine explizite Kollisionkorrektur erforderlich wird, muss sie Start und Stop derselben Linienidentitaet gemeinsam besitzen.

## Keine Sonderlogik

Gedruckte Taktbezeichnung und Partiturtitel besitzen keinerlei Entscheidungsrecht. Etuede 6 ist ausschliesslich Falsifikations- und Sichtpruefungsstueck. Die Algorithmen lesen nur strukturelle Taktordinalitaet, Ereigniszeiten, System, Darstellung und Dokumentreihenfolge.

## Prozess

In der aktuellen Fehlerfindungsphase wird bewusst kein Vollkorpus vorausgesetzt. Erst wenn der Einzelstueck-Schreibauftrag geschlossen ist und ein spaeterer produktiver Versuch visuell plausibel abgenommen wurde, wird der Korpus zum Generalisierungsbeweis herangezogen.
