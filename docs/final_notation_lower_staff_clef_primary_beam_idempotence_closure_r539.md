# r539 – FinalNotationLowerStaffClefPrimaryBeamIdempotenceClosure

## Parent

r539 basiert direkt auf dem real installierten und im 10er-Vollkorpus bestaetigten
r538 `FinalNotationLowerStaffClefExistingWriterPrimaryBeamBoundaryClosure`.

r536 und r537 bleiben rejected und sind keine Parents. Es wird keine ihrer
Writer-Relocations oder Repair-/Recovery-Autoritaeten uebernommen.

## Befund nach r538

r538 schliesst den sichtbaren Notenschluessel-/Balkenkonflikt in Etuede Nr. 8,
Takt 25 und Takt 26: die primaere `begin/end`-Kette, die eine explizite Staff-2-
Clef-Oberflaeche ueberspannt, wird entfernt. PDF und finaler Render sind sichtbar
korrekt; der 10er-Korpus bleibt dunkelgruen.

Der eingebaute unmittelbare Operator-Readback zeigt fuer Etuede Nr. 8 jedoch:

- `CompleteBeatImmediateIdempotenceReadback nein`
- `ExactFinalReadback nein`

Die Ursache ist zustandsabhaengig: r538 erkennt die negative Clef-Grenze beim
ersten Lauf aus der *noch vorhandenen* primaeren Balkenkette. Nach deren Entfernung
ist diese Kette beim zweiten Lauf nicht mehr beobachtbar; der positive Complete-
Beat-Pfad kann dieselben zwei beamfaehigen Noten deshalb wieder als vollstaendige
Schlaggruppe materialisieren.

## r539-Closure

r539 veraendert weder Writer-Pfad noch Writer-Symbol noch Aufrufgraph. Innerhalb
desselben bestehenden `FinalCompleteBeatPrimaryBeamContinuity`-Operators wird die
bereits materialisierte Staff-2-Clef-Oberflaeche nun als **zustandsunabhaengige
negative Invariante** der positiven L1-Materialisierung behandelt:

> Liegt zwischen zwei benachbarten beamfaehigen Noten derselben vollstaendigen
> Staff-2-/Voice-/Beat-Gruppe eine explizite `clef number="2"`-Oberflaeche, darf
> `FinalCompleteBeatPrimaryBeamContinuity` fuer diese Gruppe keinen primaeren
> Balken materialisieren – unabhaengig davon, ob im Eingang bereits Beam-Tags
> vorhanden sind.

Der bestehende r538-Pfad zur Entfernung einer bereits vorhandenen reinen Level-1-
Kette bleibt erhalten. Neu ist nur, dass der nachfolgende positive Complete-Beat-
Pfad dieselbe geometrische Grenze auch im bereits beamfreien Zustand respektiert.
Damit ist der Fix ein Fixpunkt statt eines Zwei-Zustands-Wechsels.

## Invarianten

Unveraendert bleiben:

- Pitch, Onset und Dauer;
- Staff und Voice;
- Clef-Entscheidung und Clef-Position;
- 8va/8vb-Entscheidung;
- Master-MusicXML;
- alle Nicht-Beam-Tags;
- Writer-Identitaet und WriterReachability;
- Pass-Reihenfolge;
- StructuralWriteAuthority;
- Schwellen, Gewichte und Etueden-/Taktidentitaeten.

Es entsteht kein neuer Writer, kein neuer produktiver Helper, kein neuer Repair-/
Recovery-Pfad und keine Legacy-Pensionierung. Das spaetere Architekturthema bleibt
bewusst ausserhalb dieses Musik-Takes.

## Regression

Der bereits in r538 erweiterte bestehende Regressionstest derselben Writer-Grenze
wird ohne neuen Test-Caller um den unmittelbaren zweiten Operatorlauf ergaenzt:

1. erster Lauf entfernt den primaeren Balken ueber der Staff-2-Clef-Grenze;
2. zweiter identischer Lauf schreibt nichts;
3. zweites Ergebnis ist byteidentisch zum ersten;
4. `exact_final_readback` bleibt wahr;
5. Clef und gesamtes Nicht-Beam-MusicXML bleiben erhalten.

## Abnahme

Zuerst Etuede Nr. 8 einzeln. Erwartet werden insbesondere:

- Takt 25 und 26 visuell unveraendert sauber gegen r538;
- `CompleteBeatImmediateIdempotenceReadback ja`;
- `ExactFinalReadback ja`;
- Clippy/Tests/Renderer gruen.

Erst danach folgt der 10er-Vollkorpus. Eine Untersuchung oder Pensionierung des
historischen `repair`-/`legacy`-Mantels erfolgt erst nach Abschluss dieser Musik-
Closure und ist nicht Bestandteil von r539.
