# Watch Web Corpus Layout

Version: 0.10.1737 r03

## Anlass

Die lokale Web-Watch wurde mit v0.10.1736 erstmals real als Einzelprojektoberflaeche
bestaetigt. Fuer `--all` existiert dieselbe Runtime-Wahrheit bereits als Launcher-,
Corpus- und Project-Sources innerhalb einer ObservationSession. Zehn grosse Karten
untereinander waeren technisch korrekt, aber fuer den normalen Blick unnoetig laut.

Dieser Take aendert deshalb ausschliesslich die Browserprojektion fuer Corpus-Sessions.
Einzelprojekt-Sessions behalten die akzeptierte grosszuegige Kartenansicht.

## Automatische Darstellungswahl

Eine Session wird als Korpus dargestellt, sobald eine echte `corpus`-Source vorhanden
ist oder mehrere echte Project-Sources vorliegen. Es wird keine fehlende Source erfunden.

```text
Einzelprojekt
  -> grosszuegige Launcher-/Projektkarten

Korpus
  -> eine Fortschrittsleiste
  -> kompakte Projektliste
  -> dezente Launcher-/Corpus-Systemzeile
```

## Projektprioritaet

Die Browserprojektion sortiert nur die bereits vorhandenen Project-Sources fuer den
menschlichen Blick:

```text
Failed
Running
Cancelled
Completed
```

Innerhalb derselben Statusklasse wird der Projektname numerisch/natuerlich sortiert.
Damit stehen laufende oder fehlgeschlagene Projekte oben, waehrend abgeschlossene
Projekte zu ruhigen Zeilen zusammenschrumpfen.

Diese Sortierung besitzt keinerlei Runtime-, Scheduler- oder Release-Autoritaet.

## Korpusfortschritt

Die einzige Fortschrittsanzeige verwendet ausschliesslich `SnapshotProgressV1` der
Corpus-Source. Sichtbar sind `completed`, `active` und der daraus rein darstellerisch
berechnete Rest `total - completed - active`. Nicht empfangene Project-Sources werden
nicht als kuenstliche Projektzeilen materialisiert.

Bei terminal abgeschlossenem Korpus darf die Ueberschrift ruhig `n / n abgeschlossen`
anzeigen; `FAILED` und `CANCELLED` bleiben weiterhin aus den echten Source-Statuswerten
abgeleitet.

## Detailtiefe

Jede Projektzeile zeigt Name, aktuellen Stage/Activity-Pfad bzw. terminalen Status und
Zustandsalter. Die Zeile kann aufgeklappt werden, um `gesehen` und `Zustand seit` zu
sehen. Aufgeklappte Zeilen bleiben auch beim sekundenweisen lokalen Altersrefresh offen.

Launcher und Corpus bleiben als kleine Systemzeile sichtbar, konkurrieren im Korpusmodus
aber nicht mit den Projekten um grosse Kartenflaeche.

## Nicht geaendert

- keine Aenderung an `SourceSnapshotV1` oder `SessionSnapshotV1`
- keine Aenderung an Corpus-Scheduler oder Project-Lifecycle
- keine Aenderung an MQTT Topic, Payload oder Subscription-Lifecycle
- keine Aenderung an UDP, Receiver oder SessionAggregator
- keine neue Health-, Pending- oder Runtime-Wahrheit
- keine Launcher-Aenderung
- keine Musik-, Writer-, Render- oder Regelmutation
- keine externe Cargo- oder Web-Abhaengigkeit

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 6"
```

Danach ist der erste echte Korpus-Webtest:

```powershell
.\run.cmd --watch --strict --all
```
