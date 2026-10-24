# Scheduler-Quick-Win: MIDI-Groesse bei drei Slots

Ein separater RefactorOnly-Take auf nativ angenommenem r1456. Nutzergrenze:
maximal ein weiterer Take; Modell, Prototyp und lokale Qualifikation sind Vorarbeit.
Keine Wiederaufnahme von Custody/r1451 und keine neue Freigabe-/Writer-Authority.

## Ursache und kleiner Eingriff

Die natuerliche Startreihenfolge beginnt auch bei drei Slots mit kleinen Projekten.
Ein spaet gestartetes langes Projekt kann den Schluss des Korpus allein bestimmen.
MIDI-Bytegroesse ist ein einfacher unvollkommener Kostenhinweis ohne neue Historie.
Die Warteschlange wird einmal absteigend nach dieser Groesse geordnet, nachdem
alle stabilen Projektindizes zugewiesen wurden und bevor das erste Projekt startet.
Gleichstaende behalten die urspruenglichen Indizes. Jede vorhandene Aufgabe wird
unveraendert genau einmal transportiert; Namen sind nur Ausgabe/Identitaet, kein
Prioritaetsmerkmal. Keine fest eingebaute Etueden- oder Referenzreihenfolge.

Nur mindestens drei parallele Slots aktivieren diese Heuristik. Bei ein/zwei Slots,
hoechstens einer Aufgabe oder auch nur einem fehlenden, leeren oder nicht regulaeren
MIDI-Groessenhinweis bleibt die gesamte bisherige Reihenfolge erhalten. Alle
Hinweise werden gelesen, bevor irgendeine Aufgabe verschoben wird. Kein Cache,
keine Manifestmigration und kein neuer Fehlerausgang wegen Scheduling-Metadaten.

Die Paarliste, expected_projects, Aufgabenindizes, Child-Observation-IDs, Pfade,
Kindprozessargumente/Umgebung, Worker und Suchbudgets bleiben bytegleich.
Die Solver-Reihenfolge innerhalb eines Projekts wird nicht beruehrt. Auch die
bisherigen Output-, Fehler-, Manifest- und Freigabepruefungen bleiben unveraendert.
Konsole: KorpusStartOrder zeigt die eingefrorene Reihenfolge; KorpusProjectTiming
haelt kuenftig die beobachteten Einzelprozesszeiten in Millisekunden fest. Keine
neuen Zeilen in den Projektberichten, keine musikalische Autoritaet dieser Zeiten.

## Vorarbeit und belegter Umfang

Die nativen Exporte r1453/r1454/r1455/r1456 enthalten MIDI-Groessen im Dateikatalog
und je Projekt sieben sequenzielle, nicht ueberlappende Hauptphasen in der Konsole.
Ihre Summe ist eine fast vollstaendige Prozesslaufzeit, nicht die exakte vom Parent
beobachtete Abschlusszeit. Unterphasen/Reportsektionen werden nicht doppelt addiert.
Das Modell mit festen Einzelzeiten trifft die gemessene natuerliche Wandzeit in
allen vier Laeufen auf unter 1,5 Sekunden. Mit drei Slots ergibt die Groessenregel
modelliert 60,227 / 65,369 / 54,930 / 55,946 Sekunden weniger. Bei zwei Slots ist
der Effekt gemischt (-11,560 / +7,837 / -2,098 / +6,239 Sekunden); dort keine Umordnung.
Diese Ergebnisse betreffen vier Laeufe desselben Korpus, keine fremden Korpora.

Das Modell haelt Laufzeiten fest und kann veraenderten CPU-/RAM-Wettbewerb nicht
beweisen. Lokale Original-MIDI/WAV-Daten fehlen; neu geordneter Korpuslauf und
RAM-Spitze sind hier nicht gemessen. Keine RAM-Ersparnis behauptet. Anzahl der
Prozesse/Renderer und deren Budgets bleiben gleich, Phasen koennen anders ueberlappen.
Ein realer Gesamtgewinn wird erst mit dem nativen Lauf bewertet.

Drei synthetische Tests pruefen jede Aufgabe mit allen Feldern unveraendert,
stabile Groessen-Gleichstaende, Wiederholbarkeit, unveraenderte Ein-/Zweislot-
Reihenfolge und vollstaendigen Fallback bei fehlendem, leerem oder Verzeichnis-Hinweis.
Der fruehe unveraenderte Parent-Gate akzeptiert den Eingriff als RefactorOnly;
die genaue finale Paketbindung und gesamte lokale Qualifikation folgen separat.

## Native Abnahme

10/10 frisch; alle PDFs mit -o ohne -f; alle 20 XMLs bytegleich gegen r1456;
unveraenderte Kanal-/Overlap-/Zeitpruefungs-Readbacks. Gleiches Ressourcenprofil.
Startreihenfolge und Einzelzeitbelege in der Konsole pruefen; Wandzeit gesondert
gegen 11:11 aus r1456 bewerten. Ein Laufpaar trennt Laufstreuung nicht kausal ab.
Bei Regression keine automatische zweite Take-Runde; Grenze bleibt ein Take.
