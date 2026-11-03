# Indexierte rhythmische Custody-Pruefung r1581

RefactorOnly auf nativ angenommenem r1580. Die Herkunfts- und Taktpruefungen
bleiben vollstaendig. Keine Aenderung an Suchbudget, Kandidaten, Reihenfolge,
Worker-Aufteilung, Auswahl, Notation, Writer-Authority oder Fehlerbehandlung.

Der bisherige Validator filtert fuer jede Wurzel alle physischen Vorkommen.
Ein lokaler BTreeMap-Index gruppiert dieselben Referenzen einmal nach root.
Die Wurzeln werden weiter in derselben Reihenfolge geprueft; jeder Bucket
behaelt die physische Eingangsreihenfolge und dieselbe stabile Intervallsortierung.
Fehlende Buckets bleiben leer. Mehrfache Vorkommen werden niemals vereinigt.

Die abschliessende Taktpruefung filtert bisher ebenfalls den ganzen Satz je Takt.
Ein zweiter lokaler Index nach (Einsatz, physischer Ordinalzahl) liefert genau
das bisherige halboffene Zeitfenster. Vor der unveraenderten physischen Pruefung
wird die urspruengliche Ordinalreihenfolge wiederhergestellt. Die bestehende
measure-Methode nutzt weiterhin ihre bisherige Suche; nur die gemeinsame
Konstruktion und Validierung des CourseMeasure wurde herausgezogen.

Beide Indizes entstehen pro voller Validierung aus dem aktuellen Zustand und
werden danach verworfen. Keine Memoisierung, kein neuer Identitaetserzeuger,
keine erfolgreiche Vorpruefung ersetzt eine spaetere Pruefung. Auch bei gleichen
Einsaetzen und Akkorden bleiben alle einzelnen Identitaeten erhalten.

Ein test-exklusiver unveraenderter r1580-Validator vergleicht Resultat und genaue
Fehlermeldung: Mehrtaktigkeit, beide Systeme, gleichzeitige Akkordmitglieder,
Taktgrenzen, echte Owner-Splits, verborgene Mitglieder, fehlende oder doppelte
Vorkommen, falsche Reihenfolge, fehlende Geschichte/Wurzel, doppelte Course-
Identitaet, falsche Lebensdauer, geaenderte Notenbytes und fremder Snapshot.

Native Abnahme: normaler Zehnerlauf ohne -f, alle 20 Master-/Render-XMLs
bytegleich zu r1580. Laufzeiten gesondert gegen r1580 messen. Ein lokaler
Algorithmusvergleich beweist keine Beschleunigung des gesamten nativen Laufs.
Die offenen musikalischen Fragen bleiben fuer den nachfolgenden Take erhalten.
