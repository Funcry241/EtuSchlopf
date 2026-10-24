# SourceSelection Rhythm Authority Frontier Audit r641

Parent: r640 `SourceSelectionStaffRhythmAuthorityCouplingAudit`

ChangeClass: AuditOnly

## Anlass

r640 hat historisch gebunden, dass die initiale Seed-Dauer derzeit aus dem Abstand
zum naechsten Onset innerhalb des final zugeordneten Staffs entsteht, obwohl die
kanonische MIDI-Dauer `MidiSeedNote.len` weiter transportiert wird. Zwei
Staff-Reassignment-Proben zeigten dadurch eine unzulaessige Kopplung zwischen
Staff-Entscheidung und Rhythmusmaterialisierung.

Ein direkter produktiver Cutover auf `len` waere jedoch ohne vorherige
Mehrstimmigkeitsanalyse nicht belegt. Ebenso ist ein staff-unabhaengiges globales
Onset-Grid nicht automatisch semantisch aequivalent.

## Audit-Frontier

r641 berechnet read-only auf den retained Zeilen des bereits autoritativen
SelectionPlans:

1. Vergleich `Source len` gegen die heute produktive staff-lokale
   Next-Onset-Dauer.
2. Anzahl der Source-Dauern, die den naechsten Attack desselben Staffs
   ueberlappen.
3. Anzahl der Source-Dauern, die ueber die aktuelle Taktgrenze reichen.
4. Anzahl gleichzeitiger Staff-Gruppen mit unterschiedlichen Source-Dauern.
5. Anzahl Events, deren Seed-Dauer sich unter einem globalen retained Onset-Grid
   aendern wuerde, einschliesslich Gesamt- und Maximalverkuerzung.

## Authority

Alle Alternativen bleiben reine Evidence.

- Productive seed duration: unveraendert `staff_local_next_onset`
- `MidiSeedNote.len` writer authority: none
- global retained onset grid writer authority: none
- Voice-/Chord-Rewrite authority: none
- MusicMutation: none

Der Audit darf nur zeigen, ob ein spaeterer Cutover klein, breit oder ohne
Voice-/Tie-/Chord-Modell unzulaessig waere. Er darf keine dieser Entscheidungen
selbst treffen.
