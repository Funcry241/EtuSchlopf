# PianisticRestConsolidationChordAnchorOwnerClosure

Stand: v0.10.1839 r313  
Parent: r312 `TonalBoundaryLedgerDecisionEvidenceClosure`  
ChangeClass: `MusicMutation`

## Frischer Beweis aus dem r312-Export

r312 schliesst den unabhaengigen Neuro-Evidence-Ledgerfehler: Etuede 5 publiziert 36 `KeyChangeCandidates` und 36 `TonalBoundaryLedger`-Zeilen. Die Gegenpruefung der Musikartefakte zeigt jedoch, dass der in r311 adressierte Takt-48-Gegenfall noch besteht.

Master, Etuede 5, Takt 48, Staff 2 / Voice 2:

- Rest 240/24
- Rest 264/24
- hidden chord B3/D4/F#4 bei 264

r312-Render:

- Rest 240/48
- dieselben hidden chord B3/D4/F#4 bei 240

Damit wurde die zweite Pause weiterhin als MusicXML-Chord-Host entfernt und die gepitchte Timeline um 24 divisions vorgezogen.

## Warum r311 den realen Fall nicht traf

r311 schuetzt `apply_structural_transactional_rest_coalesce_render_write()`. Der reale Takt-48-Merge wird jedoch ueber den bereits vorhandenen produktiven Owner `apply_pianistic_rest_consolidation()` materialisiert. Dieser Owner selektierte Restketten bisher ohne die bereits etablierte Chord-Host-Invariante und loeschte alle Folgereste byteweise. Seine spaeteren globalen Guards vergleichen Zaehler und sichtbare Sounding-Signaturen, wodurch eine reine Onset-Verschiebung unsichtbarer Chord-Mitglieder nicht erkannt wird.

## r313-Closure

r313 korrigiert ausschliesslich die bestehende Precondition-Grenze von `apply_pianistic_rest_consolidation()`. Ein Rest-Consolidation-Candidate wird nicht selektiert, wenn irgendein zu entfernendes Restfragment direkter Host eines folgenden `<chord/>`-Events innerhalb desselben Takts ist. Es gibt keinen neuen Writer, keinen neuen Repair-/Recovery-Owner, keine neue Schwelle und keine Etueden-/Takt-/Pitch-Sonderregel.

Die bereits vorhandene Regression `chord_anchored_sounding_events_block_initial_rest_coalescing()` wird ohne neuen Testsymbol-Caller erweitert: Sie prueft nun auch den tatsaechlich produktiven `apply_pianistic_rest_consolidation()`-Owner auf bytegleiche Ablehnung des Chord-Host-Falls und auf weiterhin erlaubte Konsolidierung eines echten Rest-Rest-Paars ohne Chord-Anker.

## Erwartete Wirkung

- Master-MusicXML bleibt unveraendert.
- Render-MusicXML/PDF duerfen sich nur dadurch aendern, dass eine unsichere Restkonsolidierung unterbleibt.
- Etuede 5, Takt 48: B3/D4/F#4 bleiben bei Onset 264.
- Die r312-Neuro-Evidence-Closure bleibt unveraendert: LedgerRows == KeyChangeCandidates.
- Testzahl bleibt 1067/1067, weil kein neuer `#[test]`-Caller entsteht.
