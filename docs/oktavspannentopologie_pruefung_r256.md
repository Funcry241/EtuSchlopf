# OktavspannentopologiePruefung r256

## Ausgangspunkt

r256 basiert direkt auf dem real installierten, dunkelgruenen und im Vollkorpus 10/10 abgenommenen r255 `TakeStartArchitekturpruefungWahrheitsprojektionPinEigentuemerAbschluss`.

Die wissenschaftliche Untersuchung der Oktavierung zeigte im aktuellen Korpus einen eng begrenzten, reproduzierbaren Beweisbedarf: EtuSchlopf prueft bisher Tonhoeheninvarianz und Darstellungsparameter von Oktavbeginnen, besitzt aber keinen allgemeinen lesenden Beweis fuer die Topologie der gesamten Oktavspanne. Insbesondere wurde in Etuede Nr. 6, Takt 22 eine Spanne beobachtet, deren Beginn und Ende am selben musikalischen Zeitpunkt liegen. Ein unabhaengiger MusicXML-Verbraucher erhielt diese Konstruktion nicht als vollstaendiges Paar. Der fremde Verbraucher ist dabei nur Beobachter, nicht Normgeber.

## Aenderung

r256 fuegt eine ausschliesslich lesende `OktavspannentopologiePruefung` in der bestehenden Pruefdomaene hinzu. Sie liest die bereits erzeugte Haupt- und Darstellungs-MusicXML und zaehlt beziehungsweise klassifiziert:

- Oktavbeginne und -enden,
- vollstaendige Paare,
- alleinstehende Beginne und Enden,
- Nullspannen,
- rueckwaerts laufende Spannen,
- System-, Nummer- und Groessenabweichungen,
- positive Spannen ohne erfasstes klingendes Tonereignis.

Die Pruefung verfolgt den MusicXML-Zeitzeiger innerhalb eines Taktes ueber Noten, Akkordtoene, `backup`, `forward` und Richtungsversatz. Sie erzeugt nur Berichtsevidenz. Es gibt keine automatische Auswahl eines Taktes oder Projekts.

## Beweisgrenze

Die neue Invariante lautet rein beobachtend:

`Jeder Oktavbeginn soll genau einem spaeteren Ende desselben Teils, Systems und derselben Nummer entsprechen; die Spanne soll mindestens ein klingendes Tonereignis erfassen.`

Diese Aussage ist kein Reparaturrecht. Ein gefundener Befund darf nur eine spaetere, getrennte Ursachenuntersuchung ausloesen.

## Nicht geaendert

- keine Musiklogik
- keine MusicXML-Erzeugung
- keine Tonhoehe, Dauer, Stimme oder Systemzuordnung
- kein Schreiber
- keine Darstellung
- kein Loeser
- kein Layout
- keine Sicherheitslogik
- keine Reparaturbefugnis
- keine Auswahlbefugnis
- keine Abhaengigkeit
- kein Cargo.lock
- keine Rust-Werkzeugkette

## Aenderungsklasse

Die bestehende interne Aenderungsklasse ist `AuditOnly`, weil ausschliesslich neue lesende Pruef- und Berichtsevidenz in der vorhandenen Pruefdomaene hinzugefuegt wird.
