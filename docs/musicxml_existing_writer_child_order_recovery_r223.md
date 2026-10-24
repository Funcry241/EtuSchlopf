# MusicXmlExistingWriterChildOrderRecovery r223

r223 basiert direkt auf dem real gruenen r221. Der danach angebotene r222 `MusicXmlNoteChildOrderFinalSinkConformanceClosure` wurde vom Semantic-Capability-Diff vor der Installation korrekt fail-closed abgewiesen. Ursache war nicht die MusicXML-Ordnungsanforderung, sondern die neu eingefuehrte globale `normalize_musicxml_note_child_order`-Mutation am terminalen Artefakt-Sink: Dadurch entstand neue Writer-Reachability bis in den Audit-Orchestrator. r223 uebernimmt deshalb keinen Code aus r222.

Der reale r221-Export von Etuede Nr. 6 enthaelt nach der ersten Duration/Type/Dot-Korrektur nur noch zwei direkte MusicXML-Note-Child-Ordnungsfehlerklassen:

- Master: 1 `type` vor `voice`, 107 `notations` vor spaeterem `beam`.
- Render: 11 `type` vor `voice`, 113 `notations` vor spaeterem `beam`.
- Die zuvor massenhaften `tie`-vor/bei-`type`-Fehler sind geschlossen.

r223 korrigiert diese Restklasse ausschliesslich in bereits existierenden Writer-Ownern. Es entsteht keine neue Normalizer-, Repair-, Sink- oder Writer-Schicht und keine neue produktive Funktion.

Geaendert werden nur bestehende Einfuegeoperationen:

- die drei bereits vorhandenen Duration/Type-Varianten in `thirty_surface_productive_authority.rs` setzen `type/dot` nach vorhandener `voice` und vor spaeteren Note-Kindern ein;
- vorhandene Beam-Writer in `thirty_surface_productive_authority.rs`, `writer_entry_boundary.rs`, `gould_metric_beam_authorized_writer.rs`, `composer_beam_candidate_materialization_core.rs` und `notation.rs` setzen Beam-Tags vor `notations/lyric/play/listen` statt pauschal unmittelbar vor `</note>` ein;
- bestehende Beam-Zustandswahl, Beam-Level, Pitch, Onset, Duration, Voice, Staff, Tie-/Notation-Inhalt und Kandidatenauswahl bleiben unveraendert.

Fokussierte Regressionstests binden `voice < type` fuer Rest-/Pitch-Duration-Rewrites und `staff < beam < notations` fuer bestehende Beam-Materialisierung.

Die FLP->MIDI-Seed-Terminologie bleibt weiterhin getrennt und folgt erst nach real gruenem r223. Ein allgemeines read-only MusicXML-Conformance-Gate wird ebenfalls nicht in diesen Recovery-Take hineingemischt.
