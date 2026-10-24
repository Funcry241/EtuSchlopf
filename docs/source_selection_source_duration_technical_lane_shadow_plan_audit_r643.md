# SourceSelection Source-Duration Technical-Lane Shadow-Plan Audit r643

Parent: r642 `SourceSelectionSourceDurationVoiceLaneDemandAudit`

ChangeClass: AuditOnly

## Anlass

r642 hat fuer den retained final SelectionPlan gezeigt, dass die kanonischen MIDI-Source-Intervalle
nicht mit einer einzigen Zeitspur pro Staff materialisiert werden koennen. Etuede 6 benoetigt
bis zu vier gleichzeitig unabhaengige Intervallspuren je Staff; 25 Source-Events reichen ueber
Taktgrenzen.

Diese Beobachtung erteilt noch keine Voice- oder Tie-Authority. Vor einem produktiven Cutover
muss zuerst gezeigt werden, dass die Source-Intervalle ueberhaupt deterministisch und verlustfrei
in eine technische Zwischenstruktur zerlegt werden koennen.

## Shadow-Plan

r643 gruppiert retained SourceEvents ausschliesslich nach:

`(final_staff, source_onset, source_end)`

Damit bleiben nur Events mit exakt gleichem Source-Intervall in derselben Shadow-Chordkohorte.

Die Kohorten werden je Staff in stabiler zeitlicher Reihenfolge auf technische Lanes verteilt.
Eine Lane darf erst wiederverwendet werden, wenn ihre vorherige Kohorte beendet ist. Die
Lane-Zahl ist technische Intervallpartitionierung und besitzt keine musikalische Voice-Semantik.

Taktuebergreifende Kohorten werden read-only in technische Taktsegmente zerlegt. Die Lane bleibt
ueber die Grenze stabil; weder MusicXML-Ties noch Notationsentscheidungen werden erzeugt.

## Gepruefte Invarianten

- jeder retained SourceEvent ist exakt einer Kohorte zugeordnet,
- jede Kohorte ist exakt einer technischen Lane zugeordnet,
- innerhalb einer Lane existieren keine ueberlappenden Kohorten,
- gleiche Source-Intervalle bleiben als Kohorte zusammen,
- der Plan ist von der Eingabereihenfolge unabhaengig,
- die benoetigte Lane-Zahl wird gegen die in r642 beobachtete maximale Konkurrenz geprueft,
- Taktsegmentierung veraendert weder Source-Intervall noch Staff noch Event-Identitaet.

## Authority

- MusicXML Voice WriterAuthority: none
- Tie WriterAuthority: none
- Chord WriterAuthority: none
- Source-duration WriterAuthority: none
- Staff/Selection Authority: unchanged
- Human voice semantics: NOT_EVALUATED

r643 ist ausschliesslich ein Shadow-Plan-Beweis.
