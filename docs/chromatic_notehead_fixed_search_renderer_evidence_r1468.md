# r1468: Kopfabgleich, konstante Suche und Renderer-Beleg

Parent: nativ angenommener r1467. Klasse: GovernedChange.
Explizite Nutzerfreigabe vom 21.10.2026: „und ich das Otter sage, ja Freigabe erteilt“.
Die Musikwirkung wird im Manifest getrennt begruendet; diese Notiz ersetzt kein Gate.

## Getrennte Wirkungen

1. Kopfabgleich: Pro diatonischer Hoehe muss die Zahl verschiedener physischer
   Koepfe exakt der Zahl verschiedener klingender Tonhoehen entsprechen.
   Identische Tonduplikate und deckungsgleiche Glyphen zaehlen einmal.
   Zwei chromatische Koepfe muessen auf gegenueberliegenden Halsseiten liegen.
   Ein geteilter Kopf ist nur fuer dieselbe eindeutig bekannte Tonhoehe und
   dieselbe Takt-/Staff-/Onset-Zelle zulaessig. Chromatische Mehrkopf-Sets
   begruenden keine individuelle Tonhoehe des linken oder rechten Kopfes und
   duerfen deshalb nicht als geteilte Unisono-Koepfe freigegeben werden.
   Die bestehende eindeutige Gesamtzuordnung und die exklusive Halsbindung bleiben.
   Diagnosen nennen Takt, Staff und Ablehnungsgrund auf stderr; keine Schreibrechte.
   Erwartete neue Bindungen im Evaluationskorpus: E3/T12 unten und E10/T30 oben.
   Das sind Evaluationen, keine Selektoren. E4/T30 und E6/T42 bleiben mehrdeutig.

2. Suche: HumanPotential-Budget 192, maximale Zahl der Moves 5, unabhaengig von
   Workerzahl und Hardware. Das erhaelt die Suchparameter des angenommenen
   19-Worker-Laufs, vergroessert aber bewusst den Suchumfang kleinerer Rechner.
   --jobs steuert die Parallelitaet. Der fruehere versteckte Schalter
   --candidate-budget-jobs wird mit klarer Fehlermeldung zurueckgewiesen, nicht
   still ignoriert. Der Korpus-Executor reicht ihn nicht mehr an Kinder weiter.

3. Renderer: Beide Wiederverwendungspfade duerfen eine -f-Variante nicht vor
   die ungezwungenen Versuche ziehen. Die vorhandenen -f-Fallbacks bleiben am
   Ende erhalten. Der bestehende Laufbeleg nennt den aufgeloesten Rendererpfad,
   Dateilaenge und FNV64 der nach dem erfolgreichen Aufruf gelesenen Datei.
   Dieser Beleg ist Beobachtung, keine kryptografische Attestierung der waehrend
   des Aufrufs geladenen Datei oder ihrer Konfiguration. Keine weitere Ausfuehrung,
   keine Vorauswahl nach Fingerprint und keine neue Musik-Authority.

4. Clippy: Kontextstruktur fuer bound_chord_resources; dessen Schutzpruefungen
   und Ergebnisse bleiben erhalten. E007-Dokumentation stimmt mit dem tatsaechlich
   vorhandenen historischen Beobachter ueberein; dessen Code wird nicht repariert.

## Abnahme

Unveraenderte Parent-, Incoming- und Reexec-Validatoren; vollstaendige Strict-Suite
und Clippy mit -D warnings. Synthetische Tests, kein realer Korpus als normative
Test-Fixture. Der native Zehnerlauf erfolgt nach Installation beim Nutzer.
Auf dessen Rechner sollen ausser den zwei neu gebundenen Zellen keine weiteren
XML-Aenderungen entstehen. Tonhoehen, Einsaetze, Dauern, Staffs und Bogen-Endpunkte
muessen erhalten bleiben; jede Folgeaenderung wird einzeln ausgewiesen.

Die sieben sichtbaren Faehnchen-Reststellen und die beiden mehrdeutigen Zellen
werden in diesem Take nicht repariert. Keine Aufweichung von Gates, keine
Entruempelung und keine validate_binding-Zwischenspeicherung.
