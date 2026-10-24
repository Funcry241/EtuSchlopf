# SourceSelection Staff/Rhythm Authority Coupling Audit r640

Parent: r639 `SerializedTemporalChordAnchorDiagnosticLocalization`

ChangeClass: AuditOnly

## Anlass

Zwei unabhaengige MusicMutation-Proben der `smooth_fast_staff_jumps`-Logik wurden im
Post-Apply-Reexec fail-closed verworfen. Beide erzeugten denselben lokalisierten
Master-MusicXML-Konflikt in Etuede 6, Takt 42, Onset 96:

- Anchor B2, Voice 2, Staff 2, Dauer 48
- `<chord/>`-Member Es4, Voice 1, Staff 1, Dauer 96

Der produktiv installierte Parent r639 bleibt gruen und wird durch diesen Take
nicht musikalisch veraendert.

## Historisch gebundener Quellbefund

Der r639-`MidiSeedNote` fuehrt die kanonische MIDI-Dauer `len: i64`.

Die r639-Funktion `append_staff_voice` verwendet diese Dauer bei der
MusicXML-Dauermaterialisierung jedoch nicht. Sie partitioniert die Events nach
Staff, bildet die Onset-Menge dieses Staffs und setzt fuer alle Pitches eines
Onsets:

`duration = next_start - start`

Damit ist die serialisierte Rhythmusdauer von der aktuellen Staff-Belegung
abhaengig. Eine reine Staff-Reassignment-Aenderung kann deshalb ohne Aenderung
von `MidiSeedNote.len` die spaetere MusicXML-Dauer veraendern.

Der fehlgeschlagene r640-Lauf zeigte genau diesen Effekt: Gegenueber dem gruenen
r639-Takt 42 verschoben sich die materialisierten Dauern so, dass der spaetere
Cursor-Lane-Reparaturpfad aktiviert wurde.

## Zweiter historischer Befund

`repair_cursor_lane_end_invariant_measure` besitzt in r639 einen Fallback fuer
`partial_backup_count == 0 && post_expected_events > 0`. Fuer die erste
post-expected klingende Note ruft dieser Pfad
`cursor_lane_repair_insert_chord_tag` auf.

Dieser historische Fallback prueft vor dem Einfuegen von `<chord/>` keine
Voice-/Staff-Identitaet eines tragenden Anchors. Unter der Staff-Mutation wurde
dadurch Es4 Voice 1 / Staff 1 an B2 Voice 2 / Staff 2 gebunden. Die seit r639
lokalisierte Temporal-Invariante blockierte diese Ausgabe korrekt.

## Schluss

Der naechste produktive Schritt darf daher weder

- den Temporal-Validator abschwaechen,
- den konkreten Takt 42 sonderbehandeln,
- noch lediglich weitere Staff-Moves sperren.

Vor einer neuen SourceSelection-Mutation muss die Architekturfrage getrennt
werden:

1. Welche Autoritaet besitzt `MidiSeedNote.len` fuer die initiale Rhythmusdauer?
2. Darf Staff-Reassignment die Onset-Nachbarschaft und damit Rhythmusdauer
   veraendern?
3. Wie muss ein post-expected neuer Voice-/Staff-Lane seriell reanchored werden,
   ohne ein unbelegtes `<chord/>` zu erzeugen?

Dieser r640 beantwortet diese Fragen nicht semantisch. Er persistiert nur den
historischen Beweis. Keine produktive Rust-Source wird geaendert.

FixtureSha256: 2f5f28a3aa1e9ab86e7129dce38185488c988131910f30efe3e36d59bad1ed20
