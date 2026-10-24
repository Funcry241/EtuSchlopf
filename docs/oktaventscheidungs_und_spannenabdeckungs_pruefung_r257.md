# Oktaventscheidungs- und Spannenabdeckungspruefung r257

## Status

r257 ist ein rein lesender Nachfolgeschnitt auf Grundlage des real dunkelgruenen Vollkorpus r256. Revisionsautoritaet entsteht erst durch die reale Vorabpruefung und erfolgreiche Uebernahme.

## Ausgangsbefund

r256 bewies die Oktavspannentopologie corpusweit und fand genau eine Nullspanne in Etuede Nr. 6, Takt 22. Die anschliessende statische Untersuchung zeigte zwei davon zu trennende Fragen:

1. Welche Tonobjekte wirken in der bestehenden Oktaventscheidungsformel mit?
2. Deckt die erzeugte sichtbare Oktavspanne den gedruckten System-Takt-Bereich ab, auf dem die Entscheidung beobachtbar beruht?

Die produktive Funktion `collect_visible_staff_octave_notes` sammelt alle Tonobjekte mit `<pitch>` des Systems, ohne `print-object="no"` auszuschliessen. Das beweist noch nicht, dass nicht gedruckte Tonobjekte fachlich unzulaessig sind. Deshalb bleibt der Ausschluss solcher Objekte ausschliesslich eine Gegenrechnung.

## Vorab festgelegte r256-Korpuswerte

### Hauptdateien

- System-Takt-Gruppen mit Tonhoehe: **729**
- Tonobjekte mit Tonhoehe: **10.562**
- gedruckte Tonobjekte: **7.855**
- nicht gedruckte Tonobjekte: **2.707**
- vorhandene Oktavspannen: **179**
- diagnostische Nachbildung der bestehenden Entscheidung: **729/729 Uebereinstimmungen**
- Abweichungen der diagnostischen Nachbildung: **0**
- Gegenrechnung nur mit gedruckten Tonobjekten aendert das Ergebnis: **53/729**
- davon bei vorhandener Oktavspanne: **28/179**
- Spannen mit gedruckten Tonereignissen vor dem Beginn: **20/179**
- gedruckte Tonereignisse in diesen betroffenen Bereichen: **292**
- davon vor dem Oktavbeginn: **164**
- davon innerhalb der Spanne: **128**
- Nullspannen: **1**

### Darstellungsdateien

- System-Takt-Gruppen mit Tonhoehe: **732**
- Tonobjekte mit Tonhoehe: **9.013**
- gedruckte Tonobjekte: **8.919**
- nicht gedruckte Tonobjekte: **94**
- vorhandene Oktavspannen: **141**
- Spannen mit gedruckten Tonereignissen vor dem Beginn: **18/141**
- gedruckte Tonereignisse in diesen betroffenen Bereichen: **327**
- davon vor dem Oktavbeginn: **189**
- davon innerhalb der Spanne: **138**
- Nullspannen: **1**

## Pruefvertrag

Die neue Pruefung liest nur vorhandene Haupt- und Darstellungs-MusicXML. Sie darf:

- Tonobjekte nach System und Takt zaehlen;
- `print-object="no"` als nicht gedruckt klassifizieren;
- die bestehende Oktaventscheidungsformel am finalen Hauptartefakt diagnostisch nachbilden;
- die Nachbildung gegen vorhandene Oktavbeginne vergleichen;
- dieselbe Formel als Gegenrechnung nur auf die gedruckte Tonmenge anwenden;
- die musikalische Zeit von Oktavbeginn und -ende aus `note`, `chord`, `duration`, `backup`, `forward`, `direction` und `offset` bestimmen;
- gedruckte Tonereignisse vor dem Beginn und innerhalb der Spanne zaehlen.

Sie darf nicht:

- MusicXML veraendern;
- eine Oktaventscheidung ersetzen;
- `print-object="no"` als fachlich unzulaessige Entscheidungsgrundlage festlegen;
- Projekte, Takte oder Reparaturziele auswaehlen;
- aus einer Abweichung einen Freigabefehler erzeugen.

## Falsifikationsregel

Die diagnostische Nachbildung ist als Messinstrument nur belastbar, wenn der reale Vollkorpus die vorhandenen Hauptdatei-Entscheidungen ohne Abweichung reproduziert. Der vorab festgelegte r256-Sollwert ist **729/729**.

Wenn diese Uebereinstimmung im realen Lauf nicht erreicht wird, ist zuerst die Pruefung selbst zu untersuchen. Ein solcher Befund darf nicht durch Anpassung der Produktlogik passend gemacht werden.

## Kausale Grenze

r257 beweist noch nicht, welche spaetere Hauptdatei-Verarbeitungsstufe die 20 verspäteten Spannen erzeugt. Eine spaetere instrumentierte Kausalkettenpruefung bleibt getrennte Arbeit. Insbesondere werden die Fragen Sichtbarkeit der Entscheidungseingabe und zeitliche Spannenabdeckung nicht zu einer einzigen Reparaturursache verschmolzen.

## Aenderungsklasse

`AuditOnly` im bestehenden internen Vertrag. Keine Musik-, MusicXML-Erzeugungs-, Schreib-, Darstellungs-, Loeser-, Layout-, Sicherheits-, Reparatur- oder Auswahlbefugnis.
## Vorab festgelegte Anwendungsgrenzen des aktuellen Korpus

Eine zweite, unabhaengige XML-Auswertung wurde nicht nur auf die Sollwerte, sondern auf noch nicht allgemein modellierte MusicXML-Sonderfaelle angesetzt. Im r256-Korpus treten fuer Oktavrichtungen keine `continue`-Marken, keine Groesse 22, keine anderen Groessen als 8 oder 15, keine nicht gedruckten Oktavrichtungen, keine Oktavrichtungen ohne Systemangabe, keine taktruebergreifenden Oktavspannen, keine doppelten aktiven Beginne gleicher Teil/System/Nummer-Kombination, keine alleinstehenden Enden, keine offenen Beginne und keine Takte mit mehreren `divisions`-Werten auf. Oktavrichtungen mit `offset` treten im realen Korpus ebenfalls nicht auf; der Quellentwurf besitzt dafuer dennoch einen kuenstlichen Prueffall.

In zwei Darstellungsdateien existiert jeweils ein nicht gedruckter Akkordton unmittelbar nach `forward`. Diese beiden Tonobjekte koennen die gedruckte Spannenabdeckung nicht beeinflussen, weil sie `print-object="no"` tragen; die Entscheidungsnachbildung wird fuer Darstellungsdateien ohnehin nicht ausgefuehrt. Der heutige Korpus enthaelt keinen Akkordton unmittelbar nach einer Oktavrichtung.

Diese Null- bzw. Randbedingungen sind keine allgemeine MusicXML-Regel. Sie begrenzen nur, fuer welchen realen r256-Korpus die vorab festgelegten r257-Sollwerte bereits unabhaengig gegengerechnet wurden.

## Kuenstliche Pruefmatrix

r257 enthaelt zwoelf kuenstliche Rust-Prueffaelle. Neben den sechs urspruenglichen Faellen werden jetzt ausdruecklich Gegenfaelle fuer weniger als vier Tonobjekte, eine vollstaendig abgedeckte Spanne, einen Ton exakt am Oktavbeginn, mehrere vorhandene Beginne derselben System-Takt-Gruppe, fehlerhaftes XML und die gemeinsame Beginnzeit eines Akkordtons mit seinem Vorgaengerton geprueft.

Ausgehend von den realen 900 r256-Pruefungen ergibt sich daraus eine noch nicht autoritative Erwartung von **912** Pruefungen. Erst der reale strenge Prueflauf darf diese Zahl bestaetigen oder widerlegen.

