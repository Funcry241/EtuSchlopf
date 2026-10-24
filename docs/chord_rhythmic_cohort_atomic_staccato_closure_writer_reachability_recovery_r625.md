# r625 ChordRhythmicCohortAtomicStaccatoClosureWriterReachabilityRecovery

Stand: v0.10.1839 r625  
Parent: r623 `PianisticSymbolicDirectBoundaryCliqueObstructionClosure`  
ChangeClass: `MusicMutation`

## Ausgangspunkt und rejected r624

r623 ist korpusweit 10/10 GREEN mit 1234/1234 Tests. Der externe Notationsimport-Befund in Etuede 2/Takt 13 ist als Transaction-/Delta-Fehler des bestehenden `SingleClearStaccatoRenderWriteWithPlacementReadback`-Owners gebunden: Eine vorher homogene 24/24/24/24-Akkordkohorte wurde beim Konsum einer folgenden 24er Pause nur am nicht-`<chord/>`-Anchor auf 48 retimed.

r624 versuchte die richtige chord-atomare Semantik, fuehrte dafuer jedoch `set_pitched_chord_member_duration_type_authorized` als neuen produktiven Writer-Endpunkt ein. Das PRE-TAKE-DIFF-GATE lehnte den Kandidaten deshalb korrekt ab: Eine `MusicMutation` darf die Audit-WriterReachability nur streng verengen, nicht erweitern oder umhaengen. r624 hat keine Authority.

## Recovery ohne neuen Writer

r625 baut direkt auf r623. Die fachliche Transaktion bleibt erhalten, aber es wird **kein neuer Writer-Endpunkt** eingefuehrt. Chord-Member werden im bereits bestehenden SingleClearStaccato-Owner mit zwei bereits produktiv erreichbaren Primitive-Familien retimed:

1. `ensure_atomic_move_child_text_value_authorized(..., "duration", ...)` -> bestehender `child_text_value_writer`;
2. `sync_atomic_move_note_type_and_dots_authorized(...)` -> bestehender `duration_type_dots_writer`.

Beide Writer waren bereits vor r625 aus `real_main` produktiv erreichbar. r625 erweitert deshalb nicht die Writer-Menge, sondern verwendet vorhandene Autoritaet innerhalb des bereits existierenden Staccato-Pfads.

## Chord-Kohorten-Vertrag

Die bestehende `StaccatoPersistence`-Authority akzeptiert den Batch nur, wenn:

- genau ein nicht-`<chord/>`-Anchor um exakt die Dauer der entfernten Folgepause verlaengert wird und Staccato erhaelt;
- genau eine Folgepause derselben Voice und desselben Staff entfernt wird;
- alle unmittelbar folgenden `<chord/>`-Member vollstaendig im Batch gebunden sind;
- diese Member vor der Mutation dieselbe Dauer, Voice und denselben Staff wie der Anchor besitzen;
- jedes Member Pitch und `<chord/>`-Identitaet behaelt und exakt auf die neue Anchor-Dauer retimed wird;
- kein Member versehentlich ein neues Staccato erhaelt.

Ein bereits vorher kuerzeres/asymmetrisches Chord-Member ist weiterhin legaler MusicXML-Ausgangszustand, wird aber von dieser Transaktion nicht normalisiert; der Staccato-Write faellt in diesem Fall fail-closed aus.

## Readback

Unmittelbarer und finaler Readback pruefen Anchor plus gesamte gebundene Chord-Kohorte: neue Gesamtdauer, Voice/Staff und das sortierte MIDI-Multiset der Member. Ein verlorenes, zusaetzliches oder nicht mitretimtes Member macht den Write rot.

## Produktdelta

Master-MusicXML bleibt invariant. Render-MusicXML/PDF duerfen sich nur an bereits selektierten SingleClearStaccato-Zielen mit homogener unmittelbar abhaengiger Chord-Kohorte aendern. Fuer das generische Muster hinter Etuede 2/Takt 13 wird 24/24/24/24 + Pause 24 zu 48/48/48/48, die Pause wird entfernt, Staccato bleibt nur am Anchor.

Keine Projekt-, Takt-, Pitch- oder Importer-Sonderregel; keine globale Chord-Duration-Equality-Regel; keine Repair-Schicht.
