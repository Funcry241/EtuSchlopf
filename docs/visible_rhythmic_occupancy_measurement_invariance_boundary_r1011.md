# r1011 VisibleRhythmicOccupancyMeasurementInvarianceBoundary

r1010 ist akzeptierter Parent und liefert fuer Etuede 6 einen dunklen gruenen Einzellauf mit 1328 Tests. Der neue S0/S1-Audit benutzt bereits dieselbe voice- und beam-freie atomare Occupancy-Closure auf beiden Seiten. Der r1010-Bericht ging bei einigen Labels jedoch semantisch zu weit: Gleichheit einer Occupancy-Signatur ueber die Phasengrenze ist noch keine fortgefuehrte Ereignisidentitaet.

Das persistierte Render-MusicXML besitzt keine per-note `SourceEventId`, `SoundingAttackId` oder gleichwertige stabile Provenienz. Deshalb trennt r1011 zwei Beweisstufen strikt.

## Stufe 1: Messgeometrie

S0 und S1 werden mit derselben Definition gemessen: `(measure, staff, onset, duration, atom-kind, pitches)`. Voice-Labels und Beams sind aus dieser Messung ausgeschlossen. Der Bericht spricht nur noch von:

- `SharedPitchedOccupancySignatures`;
- `SharedDuplicateRestConflictGeometries`;
- `SharedRestPitchedConflictGeometries`;
- `S1NewDuplicateRestConflictGeometries`;
- `S1NewRestPitchedConflictGeometriesAtS0HiddenRestSignature`;
- `S1VisibleAtS0HiddenSignatures`.

Damit bedeutet ein S0-hidden/S1-visible Treffer nur: dieselbe sichtbare Belegungssignatur liegt in S1 dort vor, wo in S0 dieselbe Signatur nur hidden beobachtet wurde. Er bedeutet nicht, dass dasselbe konkrete Ereignis sichtbar geschaltet wurde.

## Stufe 2: Provenienz

`PhaseEventIdentityBinding` ist explizit `unavailable_persisted_render_has_no_source_event_or_attack_identity`. Deshalb erzeugt r1011 keine Event-Level-`VISIBILITY_PROMOTION`- oder `RENDER_INTRODUCED_*`-Behauptung. Die bisherigen r1010-Labels werden durch geometrisch praezise Bezeichnungen ersetzt.

MetricNotationLine bleibt davon getrennt: die Bindung nutzt weiterhin nur Measure/Staff/Intervall und geordnete Onset/Duration-Struktur; Voice und Beam werden erst nach erfolgreicher Bindung diagnostisch gelesen.

## Voice-Invarianz-Gegenbeweis

Ein neuer metamorpher Regressionstest baut zweimal dieselbe Geometrie: eine Note und zwei deckungsgleiche Pausen. Zwischen beiden Kopien werden ausschliesslich die `<voice>`-Labels geaendert (getrennte Voices gegen zusammengefuehrte Voice). Die atomaren Occupancy-Snapshots muessen exakt gleich bleiben; insbesondere bleiben Duplicate-Rest- und Rest/Pitch-Konfliktgeometrie unveraendert.

Damit ist das Messgeraet selbst gegen reine Voice-Umetikettierung abgesichert, bevor seine Resultate zur Analyse der Voice-/Render-Pipeline benutzt werden.

## Grenzen

- audit-only / `RefactorOnly`;
- kein Writer, keine Musikmutation, keine Regel- oder Gewichtungsaenderung;
- keine aus Rendergeometrie erfundene Event-Provenienz;
- kein Voice-/Beam-Selektor;
- kein neuer Reparatur- oder Release-Owner.

Erwarteter Strict-Testbestand: **1329**.

Naechstes Gate: nur `./run.cmd --take "Etüde Nr. 6"`; kein `--all`, kein `--inventory`.
