# r538 – FinalNotationLowerStaffClefExistingWriterPrimaryBeamBoundaryClosure

## Parent und Rejection-Kette

r538 basiert direkt auf dem real installierten r535
`FinalNotationLowerStaffClefBeamBoundaryForeignTruthCutover`.

r536 und r537 wurden vor Installation fail-closed abgewiesen und sind keine Parents:

- r536 band die richtige Clef-Grenze an einen nichtkanonischen Legacy-Repair-Writer-Pfad;
  `NO-NEW-REPAIR-AUTHORITY` blockierte dies korrekt.
- r537 versuchte den lebenden Complete-Beat-Writer gleichzeitig in
  `StructuralWriteAuthority` zu verlagern. Dadurch wurde die WriterReachability des
  byteunveraenderten Audit-Orchestrators nicht nur verengt, sondern umgehaengt;
  `Semantic-Capability-Diff` blockierte dies korrekt.

Kein Gate wird gelockert. Keine abgewiesene Payload wird weiterverwendet.

## Musikalischer Befund

Etüde Nr. 8, Takt 25, untere Hand zeigt im finalen r535-Render eine explizite
Staff-2-Schluesselgrenze zwischen zwei Achteln derselben primaeren Balkenkette:

- erste Note: Beam-Level 1 `begin`;
- danach 8va-Ende und G-Schluessel fuer Staff 2;
- folgende Note: Beam-Level 1 `end`.

Der sichtbare Balken laeuft dadurch in den Schluessel hinein. Takt 26 besitzt die
analoge primaere Kette an einer Staff-2-Clef-/Octave-State-Grenze. In beiden realen
r535-Faellen ist die kollidierende Kette rein Level 1; es existiert dort kein
hoeheres Beam-Level, das mitveraendert werden muesste.

## Root Cause

r535 verhindert bereits, dass `notation` eine vollstaendige fremde Balkenwahrheit
ueber eine Lower-Staff-Clef-Grenze konserviert. Spaeter normalisiert jedoch der seit
langem existierende `FinalCompleteBeatPrimaryBeamContinuity`-Operator denselben
vollstaendigen Viertelschlag erneut und kann dabei die primaere `begin/end`-Kette
wieder herstellen.

Der Fehler ist damit keine neue Clef-Entscheidung und kein Rendererproblem, sondern
eine fehlende negative Grenze innerhalb genau dieses vorhandenen Complete-Beat-
Writers.

## r538-Closure

r538 behaelt Pfad, Symbol, Aufrufgraph und Writer-Root des bestehenden Operators
unveraendert. Es entsteht kein neuer Writer, kein neuer produktiver Helper, kein
neuer Checkpoint und keine neue Repair-/Recovery-Schicht.

Innerhalb des bestehenden Complete-Beat-Writers gilt zusaetzlich:

> Eine bereits vorhandene, reine Level-1-Balkenkette auf Staff 2 darf nicht durch
> eine explizit zwischen zwei Kettennoten materialisierte Staff-2-Clef-Oberflaeche
> laufen.

Die Kette wird aus den vorhandenen `begin/continue/end`-Zustaenden, gleicher
Staff/Voice, lueckenloser Onset-Folge und derselben Beat-Scope erkannt. Nur wenn
zwischen zwei benachbarten Kettennoten im realen MusicXML eine explizite
`clef number="2"`-Oberflaeche liegt und alle Kettenmitglieder ausschliesslich
Beam-Level 1 tragen, entfernt derselbe bestehende Writer sein eigenes Level 1 fuer
diese Kette. Hoehere Balkenebenen werden nicht angefasst und qualifizieren fuer
diesen Cutover nicht.

Die positive Complete-Beat-Normalisierung ueberspringt dieselbe negative Kette,
damit sie den gerade geschlossenen Beam nicht erneut materialisiert.

## Invarianten

Unveraendert bleiben:

- Pitch;
- Onset und Dauer;
- Staff und Voice;
- Clef-Entscheidung und Clef-Position;
- 8va/8vb-Entscheidung;
- Master-MusicXML;
- alle Nicht-Beam-Tags;
- Writer-Symbol und WriterReachability;
- Pass-Reihenfolge und bestehende StructuralWriteAuthority;
- Schwellen, Gewichte und Etueden-/Taktidentitaeten.

Der Operator prueft weiterhin `partial_beat_non_beam_compact(before) ==
partial_beat_non_beam_compact(after)` und bleibt damit Beam-only.

## Regression

Es wird kein neuer Test-Caller angelegt. Ein bereits bestehender writer-erreichbarer
Regressionstest derselben Datei wird um einen generischen Staff-2-Clef-Crossing-Fall
erweitert. Erwartet wird:

- vorhandenes primaeres `begin/end` ueber der Clef-Grenze verschwindet;
- die Clef-Oberflaeche bleibt erhalten;
- Nicht-Beam-MusicXML bleibt identisch;
- keine neue WriterReachability entsteht.

## Abnahme

Erst Etüde Nr. 8 einzeln. Im PDF sind Takt 25 und Takt 26 unten direkt zu pruefen.
Erst wenn beide Stellen musikalisch sauber sind, folgt ein Vollkorpus.

Die moegliche spaetere Pensionierung bzw. Relocation des historischen
`lower_dual_edge_beat_beam_continuity_repair`-Owners ist ausdruecklich **nicht**
Teil von r538 und wird erst nach Abschluss dieses Musikthemas neu bewertet.
