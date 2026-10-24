# r262 – RegisternotationsGrenzenevidenzUndAbschnittsbildung

## Ausgangspunkt

r261 `RegisternotationsFachmodellUndPlanungsbeweisKompilationsabschluss` ist real installiert und im Vollkorpus dunkelgruen: 10/10 Projekte, 948/948 Pruefungen, Bau und Clippy bestanden. Der schreibfreie Vor-K00-Beobachter reproduziert corpusweit 736/736 tatsaechliche Altentscheidungen ohne Abweichung. Gleichzeitig bestaetigt er die bereits getrennt bekannte Sichtbarkeitsgegenrechnung mit 53 Aenderungen, davon 28 bei vorhandener Oktavspanne.

r261 hat eine zuvor zu starke Annahme falsifiziert: Die spaetere Hauptdatei-Projektion mit 729 System-Takt-Gruppen und 10.562 Tonobjekten ist nicht identisch mit dem wirklichen Entscheidungsraum unmittelbar vor K00. Vor K00 liegen corpusweit 736 System-Takt-Gruppen und 10.575 Tonereignisse vor. Ausserdem wurden die in r261 bereits beobachteten Grenzmerkmale noch nicht zur eigentlichen Abschnittsbildung benutzt; der neue Planer blieb dadurch faktisch taktzentriert.

## Ziel

r262 bleibt vollstaendig schreibfrei. Der Take erhebt die bereits vorhandene Grenzenevidenz zur tatsaechlichen Abschnittsbildung, ohne neue Registerkosten oder frei erfundene Gewichtungen einzufuehren.

Die Altentscheidung bleibt weiterhin exakt auf `Teil × Takt × System` bezogen, damit die corpusweit bestaetigten 736/736 Vergleiche unveraendert als unabhaengiger Zeuge erhalten bleiben. Die neue Planung arbeitet dagegen auf einer fortlaufenden `Teil × System`-Ereignisfolge und darf deshalb eine zusammenhaengende Registerfigur ueber eine schwache Taktgrenze hinweg fortsetzen.

## Geordnete Grenzpolitik

Eine Taktgrenze allein ist keine Abschnittsgrenze. Jede beobachtete Grenze wird deterministisch als `Schwach` oder `Tragend` klassifiziert.

Eine Grenze ist tragend, wenn keine zeitliche Ueberlappung der angrenzenden Angriffe vorliegt und mindestens einer der folgenden fachlich benennbaren Faelle eintritt:

- die Stille erreicht mindestens eine vollstaendige Zaehlzeit;
- der kleinste Tonhoehenabstand der benachbarten Angriffe betraegt mindestens eine Oktave;
- eine Taktgrenze wird durch Stille oder Dauerwechsel zusaetzlich gestuetzt;
- eine Zaehlzeitgrenze besitzt gleichzeitig Stille und Dauerwechsel.

Alle anderen Grenzen bleiben schwach. Es gibt keine Punktesumme, keine frei kalibrierte Gewichtung und keine etueden-, takt- oder positionsspezifische Produktionsregel. Ueberlappende Angriffe werden nie allein durch einen Registersprung auseinandergerissen.

## Abschnitts- und Planwahrheit

Die neue Reihenfolge lautet in r262:

```text
model::Score vor K00
        ↓
RegisternotationsEreignisse
        ↓
fortlaufende Teil-System-Folge
        ↓
Grenzenevidenz
        ↓
Schwach / Tragend
        ↓
RegisternotationsAbschnitte
        ↓
RegisterNotationPlan
        ↓
Beweisbericht
```

Der `RegisterNotationPlan` traegt nun einen Beginn- und Endtakt. Eine schwache Taktgrenze kann innerhalb eines Plans liegen. Die betroffene Ereignismenge bleibt explizit; Beginn und Ende der semantischen Spanne werden weiterhin ausschliesslich aus dieser Menge abgeleitet.

Die bestehenden numerischen Regeln bleiben unveraendert: Registerbereiche 36..60 und 60..84, Mindestzahl 4, Verbesserungsschwelle 8, Doppeloktav-Bezeichnungsstrafe 24 und MIDI 69 fuer den Vorrang der spaeteren Schluesselbehandlung im unteren System.

## Schreibfreiheit

r262 besitzt weiterhin keinerlei MusicXML-Schreibrecht und keinerlei `StructuralWriteAuthority`-Befugnis. `apply_octave_notation_policy` und die gesamte produktive Altmaterialisierung bleiben unveraendert. Nach K00 wird ausschliesslich das bestehende Altmaterialisierungszeugnis gelesen.

Der Bericht ergaenzt fuer die neue Abschnittsbildung insbesondere:

- Anzahl aller Abschnitte;
- Anzahl taktuebergreifender Abschnitte;
- tragende und schwache Grenzen;
- tragende und schwache Taktgrenzen;
- Plananzahl und Plan-Fingerabdruck;
- die unveraenderten Topologie-Nullzaehler.

## Pruefvertrag

r261 besitzt real 948/948 Pruefungen. r262 fuegt genau acht neue kuenstliche Pruefungen fuer die Grenz- und Abschnittsbildung hinzu. Vorab festgelegte Erwartung fuer den realen strengen Lauf ist deshalb **956/956**.

Die neuen Pruefungen sichern insbesondere:

1. Taktgrenze allein bleibt schwach;
2. schwache Taktgrenze kann einen taktuebergreifenden Abschnitt ergeben;
3. Oktavsprung ist tragende Evidenz;
4. kurze Stille allein bleibt schwach;
5. Taktgrenze plus Dauerwechsel wird tragend;
6. ueberlappende Angriffe werden nicht durch Registersprung getrennt;
7. taktuebergreifende Plaene bleiben topologisch geschlossen;
8. ein taktuebergreifender Plan dokumentiert die ueberbrueckte schwache Taktgrenze.

## Falsifikationsregel

r262 ist kein Besitzwechsel. Der spaetere produktive Take darf ausschliesslich die in r262 bewiesene Planwahrheit materialisieren. Wenn r262 im Vollkorpus neue Nullspannen, systemfremde Ereignisse, widerspruechliche Ueberlappungen oder nicht klassifizierte Abweichungen erzeugt, ist die Besitzumschaltung gesperrt.
