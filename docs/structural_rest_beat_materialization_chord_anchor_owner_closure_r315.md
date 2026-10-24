# StructuralRestBeatMaterializationChordAnchorOwnerClosure

Stand: v0.10.1839 r315  
Parent: r314 `BehindBarsLowerStaffRestCompactionChordAnchorOwnerClosure`  
ChangeClass: `MusicMutation`

## Frischer r314-Beweis

r314 ist real installiert und fuer Etuede 5 prozessual GREEN. Der Neuro-Evidence-Export ist wieder erfolgreich. Die konkrete Master/Render-Gegenpruefung von Etuede 5, Takt 48 bleibt jedoch rot: Master Staff 2 / Voice 2 besitzt Rest 240/24, Rest 264/24 und die hidden Chord-Events B3/D4/F#4 bei Onset 264; der r314-Render besitzt Rest 240/48 und dieselben Chord-Events bei Onset 240.

Damit ist zugleich bewiesen, dass der r314-Guard im unteren Behind-Bars-Compactor nicht die letzte wirksame Mutation dieses Falls ist.

## Vollstaendige lokale Pass-Reihenfolge

Im bestehenden `BehindBarsRestAndTieAudit` laufen die Rest-Owner unmittelbar hintereinander:

1. `apply_behind_bars_rule_check_lower_staff_rest_compaction()`
2. `apply_structural_rest_beat_materialization()`
3. danach erst die weiteren Beat-/Voice-Rewrite-Schritte.

r314 schliesst Schritt 1 fuer chord-abhaengige Restketten. Schritt 2 gruppiert jedoch sichtbare Restfragmente derselben Staff/Voice innerhalb derselben dynamisch berechneten Zaehleinheit erneut zu Ketten. Fuer den Takt-48-Fall liegen 240/24 und 264/24 gemeinsam innerhalb des Beats 192..288; `MetricRestNotation` plant deshalb einen einzelnen 48er-Rest. Der zweite Rest wird anschliessend durch leeren Text ersetzt. Ohne Chord-Host-Precondition entsteht damit exakt erneut derselbe Onset-Verlust.

## r315-Closure

r315 propagiert die bereits etablierte `ChordAnchoredRestSpanSafety`-Invariante ausschliesslich in den bestehenden `apply_structural_rest_beat_materialization()`-Owner.

Vor dem Erzeugen von Replacements wird jede Restspanne der Kandidatenkette gegen die vorhandene generische `note_is_followed_by_chord_note()`-Beobachtung geprueft. Ist irgendein Restfragment direkter MusicXML-Host eines folgenden `<chord/>`-Events im selben Takt, wird die gesamte Beat-Materialisierung fuer diese Kette fail-closed nicht selektiert.

Es entsteht keine neue Funktion, kein neuer Writer, keine neue Repair-/Recovery-Authority, keine neue Schwelle und keine Etueden-/Takt-/Pitch-Sonderregel. Echte Restketten ohne Chord-Anker bleiben unveraendert materialisierbar.

## Regression

Der bereits vorhandene Test `chord_anchored_sounding_events_block_initial_rest_coalescing()` wird ohne neuen `#[test]`-Caller erweitert. Derselbe synthetische Staff-2-Rest-Rest-HiddenChord-Fall muss nun auch bei `apply_structural_rest_beat_materialization()` bytegleich abgelehnt werden. Ein entsprechendes echtes Staff-2-Restpaar ohne Chord-Anker muss weiterhin auf Dauer 48 materialisiert werden.

## Abnahme

- Etuede 5, Takt 48: B3/D4/F#4 muessen im finalen Render bei Onset 264 bleiben.
- Master-MusicXML bleibt unveraendert.
- Neuro-Evidence bleibt exportierbar; `LedgerRows == KeyChangeCandidates`.
- Testzahl bleibt 1067/1067.
- Erst ein frischer Export mit dem konkreten Onset-Beweis schliesst den Musikbefund.
