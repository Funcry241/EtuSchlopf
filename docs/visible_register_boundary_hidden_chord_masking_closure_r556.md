# r556 – VisibleRegisterBoundaryHiddenChordMaskingClosure

## Elternstand

Autoritativer Elternstand ist EtuSchlopf v0.10.1839 r555 `MusicXml31OfficialSchemaByteProvenanceBindingClosure`, korpusbewiesen 10/10 dunkelgruen mit 1137/1137 Tests, Build und Clippy gruen.

## Sichtbefund

Im r555-Endartefakt von Etuede 5 steht der hohe Oberstimmenabschnitt in Takt 33 ohne Oktavspanne. Unter den gedruckten Toenen liegen unter anderem A#6 und G6. Der vorherige 15ma-Zustand endet in Takt 32.

Die r555-Grenzlogik liefert dafuer eine konkrete, quellnahe Fehlerklasse ohne Etueden- oder Taktsonderregel. `grenzen_evidenz` berechnet den kleinsten Tonhoehenabstand eines Angriffspaars bisher aus allen Tonereignissen des Angriffs. Dadurch koennen `print-object="no"`-Akkordtoene einen grossen sichtbaren Registerwechsel maskieren.

Im finalen r555-Artefakt zeigt die Grenze T33 -> T34 den Mechanismus exemplarisch:

- letzter gedruckter Angriff T33: F#5, MIDI 78;
- erster gedruckter Angriff T34: D4, MIDI 62;
- sichtbarer Registerabstand: 16 Halbtone, also tragende Registerevidenz;
- ein verborgener Strukturton am letzten Angriff von T33 liegt bei MIDI 66 und reduziert den bisherigen All-Event-Minimalabstand auf 4 Halbtone;
- dadurch bleibt die Grenze bisher schwach und T33 wird mit dem tieferen Folgeregister zu einem gemeinsamen Abschnitt verklebt.

Eine artefaktnahe Gegenrechnung am finalen r555-MusicXML zeigt: Wird T33 an dieser sichtbaren Registergrenze eigenstaendig bewertet, bleibt die vorhandene Registerentscheidung ausreichend. Fuer die gedruckten T33-Toene ergibt die unveraenderte Kostenfunktion `loco 426`, `8va 0`, `15ma 96`. Die bestehende Verbesserungsschwelle waehlt damit 8va. Diese Gegenrechnung ist keine Behauptung ueber byteidentische Vor-K00-Daten; der reale Windows-Lauf bleibt die Falsifikation.

## Enge Korrektur

r556 bleibt im vorhandenen Owner `src/registernotation/mod.rs`.

`Angriff` fuehrt neben der vollstaendigen strukturellen Tonhoehenmenge eine zweite, ausschliesslich gedruckte Tonhoehenmenge. Fuer die Registerabstands-Evidenz gilt:

1. Die bisherige strukturelle Minimaldistanz bleibt die Default-Wahrheit.
2. Ausschliesslich im oberen System und ausschliesslich dann, wenn die strukturelle Minimaldistanz unter der bestehenden Oktavgrenze 12 liegt, wird zusaetzlich die gedruckte Minimaldistanz betrachtet.
3. Nur wenn diese gedruckte Minimaldistanz die bestehende Oktavgrenze `>= 12` erreicht, ersetzt sie fuer genau diese Grenzklassifikation die durch verborgene Toene maskierte strukturelle Distanz.
4. Unteres System, Angriffe ohne beidseitig gedruckte Tonhoehen und alle Faelle ohne Klassifikationswechsel behalten die bisherige strukturelle Distanz.
5. Zeitliche Ueberlappung, Stille, Taktgrenze, Zaehlzeitgrenze und Dauerwechsel bleiben unveraendert.
6. Die nachfolgende Segmentbildung und `RegisterNotationPlan`-Entscheidung bleiben unveraendert.

Damit koennen verborgene Akkordtoene weiterhin strukturelle Kontinuitaet tragen. Nur im oberen System duerfen sie keinen beidseitig gedruckt beobachtbaren Oktavsprung mehr unter die bereits vorhandene Registerbruchgrenze maskieren.

## Unveraendert

r556 aendert nicht:

- `MINDEST_TONOBJEKTE = 4`;
- die Registerkostenbereiche 60..84 und 36..60;
- `VERBESSERUNGSSCHWELLE = 8`;
- die Doppeloktav-Bezeichnungsstrafe;
- den Lower-Staff-Clef-Vorrang ab MIDI 69;
- 8va/15ma/8vb/15mb-Kandidaten oder deren Tie-Break;
- Pitch, Onset, Dauer, Voice, Staff oder Sichtbarkeit;
- den MusicXML-Writer oder StructuralWriteAuthority;
- Materialisierung, Passordnung, Layout, Beam, Rest oder Clef-Owner;
- Kodex, Bootstrap, Take-Gates oder Cargo-Abhaengigkeiten;
- E1/T3-Spacing; dieser getrennte Lesbarkeitsbefund bleibt ausserhalb dieses Takes.

Es gibt keine Etueden-, Takt-, Positions- oder konkrete Pitch-Sonderregel.

## Regressionstest

Die zwei neuen kuenstlichen Tests erzeugen dieselbe maskierte Angriffskonstellation im oberen und unteren System. Im oberen System muss die gedruckte Oktavdistanz gewinnen; im unteren System muss die bisherige strukturelle Distanz unveraendert bleiben.

Der obere Test erzeugt zwei benachbarte Angriffe:

- links gedruckt MIDI 90 plus verborgen MIDI 78;
- rechts gedruckt MIDI 74 plus verborgen MIDI 77.

Der alte All-Event-Minimalabstand waere 1 und damit schwach. r556 verlangt den sichtbaren Abstand 16 und damit `Tragend`. Der Test beweist damit direkt, dass verborgene Akkordtoene einen sichtbaren Registerbruch nicht mehr maskieren.

Ausgehend von 1137/1137 in r555 wird fuer den realen Windows-Lauf 1139/1139 erwartet. Diese Zahl ist vor dem Lauf nur Erwartung, kein behauptetes Testergebnis.

## Abnahme

Der erste reale Lauf ist Etuede 5. Erwartet werden:

- PRE-TAKE-DIFF-GATE gruen;
- Build und Strict Clippy gruen;
- 1139/1139 Tests;
- keine Topologieverletzung der Registerplaene;
- T33 oben wird durch die bestehende Registerentscheidung lesbarer oktaviert;
- keine neue Fehloktavierung an benachbarten Stellen.

Erst nach dieser Einzelabnahme folgt der Vollkorpus. r555 bleibt bis dahin Corpus-GreenBase.
