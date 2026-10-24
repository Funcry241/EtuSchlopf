# BehindBarsLowerStaffRestCompactionChordAnchorOwnerClosure

Stand: v0.10.1839 r314  
Parent: r313 `PianisticRestConsolidationChordAnchorOwnerClosure`  
ChangeClass: `MusicMutation`

## Frischer r313-Beweis

r313 ist real installiert und fuer Etuede 5 prozessual GREEN. Die konkrete Master/Render-Gegenpruefung von Etuede 5, Takt 48 bleibt jedoch rot: Master Staff 2 / Voice 2 besitzt Rest 240/24, Rest 264/24 und die hidden Chord-Events B3/D4/F#4 bei Onset 264; der r313-Render besitzt Rest 240/48 und dieselben Chord-Events bei Onset 240.

Damit ist bewiesen, dass weder der r311-Transactional-Coalescer noch der in r313 geschuetzte `apply_pianistic_rest_consolidation()` der materialisierende Owner dieses konkreten Deltas ist.

## Tatsaechlicher Owner

Im Initial-Render-Pfad folgt nach dem Pianistic-Rest-Trial der bestehende `BehindBarsRestAndTieAudit`. Dessen `apply_behind_bars_rule_check_lower_staff_rest_compaction()` arbeitet ausdruecklich auf Staff 2, bildet zusammenhaengende sichtbare Restketten, laesst `MetricRestNotation` die kuerzere kanonische Restfolge planen und ersetzt nicht mehr benoetigte Restfragmente durch leeren Text.

Fuer 24+24 innerhalb derselben metrischen Einheit plant der bestehende MetricRestNotation-Owner genau 48. Bisher fehlte hier die bereits etablierte Chord-Anker-Precondition. Ein geloeschter Rest konnte daher direkter MusicXML-Host folgender `<chord/>`-Events sein und deren Onset veraendern, obwohl Pitch-, Dauer- und reine Zaehlerguards gruen blieben.

## r314-Closure

r314 aendert ausschliesslich die Precondition des bestehenden unteren Behind-Bars-Restketten-Owners. Vor der Erzeugung von Replacements wird jede Restspanne der Kette gegen die bereits vorhandene generische `note_is_followed_by_chord_note()`-Beobachtung geprueft. Ist irgendein Restfragment direkter Host eines folgenden Chord-Events im selben Takt, wird die gesamte Compaction fuer diese Kette fail-closed nicht selektiert.

Es entsteht keine neue Funktion, kein neuer Writer, keine neue Repair-/Recovery-Authority, keine neue Schwelle und keine Etueden-/Takt-/Pitch-Sonderregel. Echte Staff-2-Restketten ohne Chord-Anker bleiben unveraendert konsolidierbar.

## Regression

Der bereits vorhandene Test `chord_anchored_sounding_events_block_initial_rest_coalescing()` wird ohne neuen `#[test]`-Caller erweitert. Er prueft nun denselben synthetischen Rest-Rest-HiddenChord-Fall auch direkt gegen den unteren Behind-Bars-Compactor und verlangt bytegleiche Ablehnung. Ein entsprechendes echtes Staff-2-Restpaar ohne Chord-Anker muss weiterhin auf Dauer 48 kompaktiert werden.

## Abnahme

- Etuede 5, Takt 48: B3/D4/F#4 muessen im finalen Render bei Onset 264 bleiben.
- Master-MusicXML bleibt unveraendert.
- r312-Neuro-Evidence-Closure bleibt erhalten: `LedgerRows == KeyChangeCandidates`.
- Testzahl bleibt 1067/1067.
- Erst ein frischer Export mit dem konkreten Onset-Beweis schliesst den Musikbefund.
