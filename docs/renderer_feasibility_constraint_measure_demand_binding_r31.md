# Renderer Feasibility Constraint Measure-Demand Binding r31

Parent: `0.10.1839 r30`

ChangeClass: `RefactorOnly`

## Ausgangslage

r30 hat die Rhythmic-Shape-Timeline korpusfest geschlossen. Der autoritative
10/10-Lauf liefert fuer alle zehn Etueden `RhythmicShape: Status OBSERVED`.
Im Korpus stehen 63 geplante Dreitakt-Systeme 11 vom MuseScore-Renderer
zusaetzlich aufgetrennten Dreitakt-Systemen gegenueber; alle 97 geplanten
Zweitakt-Systeme bleiben erhalten. Die 11 Splits lassen sich weder durch
Headroom noch durch Spaltenzahl, Notenzahl oder Stack-Groesse allein sauber
trennen.

Damit soll keine weitere inoffizielle Breitenformel neben den Joint Planner
gestellt werden. Der naechste Architekturpfad behandelt den Renderer nur als
Feasibility-Witness: ein vom Renderer nachweislich aufgetrennter geplanter
Systembereich darf spaeter als Unzulaessigkeits-Constraint in einen erneuten
Joint-Plan-Beweis eingehen. Die Auswahl bleibt beim EtuSchlopf-Planner.

## Ziel

r31 transportiert dafuer die bereits im Joint Planner berechneten
**Measure-Demands** als read-only Evidence bis zum bestehenden
Renderer-System-Break-Readback.

Der Planner berechnet den Demand-Vektor bereits heute aus dem finalen
`HorizontalLayoutFactSnapshot`. r31 ruft diese Formel nicht ein zweites Mal auf.
Es speichert denselben vorhandenen Vektor nur im bestehenden Auditobjekt und
projiziert ihn als `SelectedMeasureDemands`.

Der Readback akzeptiert die neue Evidence nur, wenn:

- genau ein Demand fuer jeden Measure-Slot vorhanden ist,
- die Measure-Nummern lueckenlos bei 1 beginnen,
- jede Summe der Measure-Demands exakt der bereits gebundenen
  `SelectedSystemLoads`-Last desselben geplanten Systems entspricht,
- Systemgruppen weiterhin exakt mit dem materialisierten Plan uebereinstimmen,
- keine Planner-Systemlast die transportierte Capacity ueberschreitet.

Bei Drift wird die Demand-Bindung nur `UNAVAILABLE`; es entsteht kein Ersatzwert.

## Neue Projektion

Die bestehende Joint-Planner-Evidence erhaelt:

`SelectedMeasureDemands 1=...,2=...,...`

Die bestehende Readback-Bindung projiziert zusaetzlich:

`MeasureDemands ... | MeasureDemandRecompose exact`

`MeasureDemandRecompose exact` bedeutet ausschliesslich, dass die transportierten
Measure-Demands die bereits vorhandenen Planner-Systemlasten exakt reproduzieren.
Es ist **kein** zweites Demand-Modell.

## Keine Musik- oder Layoutaenderung

r31 aendert nicht:

- Demand-Formel,
- AcceptedSystemCapacity,
- PageCount-Preference,
- System-/Page-Auswahl,
- Writer,
- Materializer,
- Rendereraufruf,
- MusicXML,
- PDF,
- musikalische Semantik.

Die drei musiknahen Source-Aenderungen sind reine Evidence-Transport-/Bindungslogik.

## Warum jetzt Measure-Demands

Der r30-Korpus zeigt, dass weitere einfache Summenproxies die MuseScore-Grenze
nicht belastbar erklaeren. Fuer einen spaeteren renderer-constraint-basierten
Shadow-Replan muss EtuSchlopf alternative Systembereiche bewerten koennen, ohne
die Demand-Formel im Audit nachzubauen. Der originale Measure-Demand-Vektor ist
dafuer die minimale autoritative Grundlage.

## Regression

r31 ergaenzt einen fail-closed Test:

- stimmt die Summe der Measure-Demands eines geplanten Systems nicht exakt mit
  dessen `SelectedSystemLoads`-Last ueberein, wird die Bindung abgelehnt.

Die vorhandenen r27-r30 Readback-, Demand- und Rhythmic-Shape-Tests bleiben
unveraendert autoritativ.

Erwarteter Testbestand: **752/752**.

## Naechster Schritt

Nach gruendem Einzellauf und 10/10-Korpus kann der naechste reine Shadow-Schnitt
die vom Renderer verworfenen Systembereiche als No-Good-Constraints gegen
**diesen gebundenen originalen Demand-Vektor** pruefen. Erst ein danach
rendererbestaetigter Constraint-Replan darf fuer eine produktive Closure
ueberhaupt in Betracht kommen.

Kein Threshold-Tuning. Keine Etueden-/Taktspeziallogik. Kein `--force`.
