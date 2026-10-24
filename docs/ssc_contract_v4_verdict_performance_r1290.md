# r1290 – SSC-Vertrag v4: schnelleres Adoptionsverdikt

Parent: accepted r1289 `SscUnreleasedCleanup`. Klasse `GovernedChange`, `music_effect=none`, genau eine `proof_rule` auf `src/ssc_evidence_proof.rs`.

## Anlass

r1289 brachte den Vollkorpus von 12:28 auf 8:42. Vor dem Enforcement lag er bei etwa 7:00. Der Rest ist das Verdikt vor jedem der rund 14 000 Commits. Messung im Release-Build auf echten Partituren: Ein Verdikt kostete 14 bis 47 ms. Davon entfielen 6 bis 27 ms auf das Containment und je Dokument 2 bis 10 ms auf die Edit-Bereiche. Jedes Dokument wurde dabei mehrfach geparst, und die Kanonform entstand aus vielen Einzel-Allokationen.

## Änderung

- `receipt_containment_assessment` parst Pre und Post genau einmal. Edit-Bereiche (`mutable_intervals_of`) und Kanonform (`containment_form_of`) nutzen dieselben Dokumente.
- Die Kanonform wird mit `push_str` und `write!` in einen Puffer geschrieben statt mit `format!` je Element, Attribut und Textknoten. Bytes und Reihenfolge sind identisch.
- `containment`, `mutable_intervals` und `undeclared_edit` bleiben als dünne, nur im Test kompilierte Hüllen für die bestehenden Tests erhalten.
- Unparsebare Bytes bleiben `ContainmentUnproven` wie bisher.

Keine Änderung an Envelope-Tabelle, Klassen, Verdikt, Witness, Bericht oder Enforcement.

## Nachweis

- **Bitgleichheit:** Die alte r1289-Kanonform und die neue wurden auf allen 20 Korpus-XML (Master und Render) für alle 76 Envelope-Zeilen verglichen: Kanonform, Whole-Element-Liste und Edit-Bereiche, insgesamt 1520 Prüfungen, alle identisch. Lokale Diagnose, nicht ausgeliefert.
- **Laufzeit eines Verdikts:**

  | Etüde | vorher | nachher |
  |---|---:|---:|
  | 1 | 14 ms | 7 ms |
  | 5 | 47 ms | 25 ms |
  | 6 | 38 ms | 30 ms |

  Release-Build, Rauschen etwa ±20 %.
- Alle bestehenden Containment-, Witness- und Enforcement-Tests gelten unverändert.

## Erwartung

Etwa 30 bis 50 Sekunden weniger im Vollkorpus, also knapp unter oder um 8:00. 20/20 XML bytegleich, alle SSC-Zeilen identisch zu r1289.
