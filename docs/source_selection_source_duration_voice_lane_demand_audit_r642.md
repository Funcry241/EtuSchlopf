# SourceSelection Source-Duration Voice-Lane Demand Audit r642

Parent: r641 `SourceSelectionRhythmAuthorityFrontierAudit`

ChangeClass: AuditOnly

## Anlass

r641 hat auf dem produktiven finalen SelectionPlan gezeigt, dass die heute
produktive staff-lokale Next-Onset-Dauer weder mit `MidiSeedNote.len` noch mit
einem staff-unabhaengigen globalen Onset-Grid allgemein identisch ist.

Ein direkter Source-Dauer-Cutover waere insbesondere nicht als eine Stimme pro
Staff materialisierbar, solange Source-Intervalle ueber spaetere Attacks
hinweg reichen, gleiche Onsets unterschiedliche Source-Enden besitzen oder
Source-Intervalle Taktgrenzen ueberschreiten.

## Shadow-Modell

r642 gruppiert retained Events ausschliesslich fuer den Audit nach:

`(final_staff, source_onset, source_end)`

Events mit exakt demselben Schluessel bilden eine Shadow-Chord-Cohort. Diese
Gruppierung ist keine musikalische Chord-Entscheidung und besitzt keine
Writer-Authority.

Fuer jede Staff/Takt-Scope wird anschliessend die maximale Zahl gleichzeitig
aktiver Shadow-Intervallkohorten bestimmt. Da Intervalle bei identischem Ende
vor einem neuen Start freigegeben werden, entspricht dieser Wert dem minimalen
gleichzeitigen Lane-Bedarf des reinen Intervallmodells in dieser Scope.

Zusaetzlich werden Source-Events und exakte Intervallkohorten gezaehlt, die
Taktgrenzen ueberschreiten.

## Harte Grenzen

`MidiSeedNote.len` stammt lossless aus dem MIDI Note-On/Note-Off-Paar. r642
behauptet ausdruecklich nicht, dass Key-Hold-Dauer automatisch die richtige
gedruckte Notationsdauer ist.

Keine neue Autoritaet:

- Source-duration writer authority: none
- Voice-lane writer authority: none
- Chord writer authority: none
- Tie-segmentation writer authority: none
- Staff/hand/selection authority: unveraendert
- MusicXML/Render/PDF write: unveraendert

Der Audit beantwortet nur, welche Mehrstimmigkeits- und Taktsegmentierungs-
Komplexitaet ein spaeterer Source-Dauer-Ansatz mindestens behandeln muesste.
