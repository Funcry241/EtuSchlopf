# Reparaturpfade und ihre fachlichen Owner

Basis ist der angenommene r1655. Der vollständige Korpuslauf enthält zehn
Projekte und 2.967 erfolgreiche Tests. Die folgende Bestandsaufnahme trennt
aktive Schreibpfade, bereits zurückgeführte Entscheidungen und lesende
Regressionsprüfungen. Dateinamen allein reichen zur Zuordnung nicht aus.

| Pfad | Ausführung und Ursache | Owner und Konsequenz |
|---|---|---|
| E052 `decompose_lower_staff_clef_islands` in `src/notation.rs` | Der Initial-Render ruft nach `polish_musicxml` einen zweiten Schlüsselplaner auf. Dieser kann einen G2-Schlüssel in einen F4-Abschnitt einsetzen, sofern die eigene Suche genau einen Kandidaten liefert. Der native Lauf meldet ab `before_rest_articulation` viermal denselben fehlenden E052-Transport der Balkenprovenienz. Das sind vier Beobachtungspunkte eines Ausfalls. | E024 `apply_rustig_clef_course_to_buffer` besitzt den vollständigen F4/G2-Verlauf samt geschützter Zeit- und Balkengrenzen. Der Render soll seinen Schlüsselverlauf vor der nächsten Render-Transaktion vom vorhandenen Owner erhalten. Die alte E052-Heuristik gehört aus der produktiven Aufrufkette. |
| E054 in `polish_musicxml` | Vor und nach der Notenpolitur ruft derselbe Pfad `ledger_bounded_lower_staff_clef_readback` auf. Der erste Pass liefert Schlüsselgrenzen an die Balkenpolitur. Der Kommentar des zweiten nennt einen entfernten Rückkehrschlüssel, während `polish_measure` die Zwischenräume mit den Schlüsselblöcken kopiert und nur Notenblöcke umschreibt. | Der Schlüsselverlauf gehört E024, die Balkensegmentierung E051. Vor einem Eingriff ist der Vorher-Nachher-Vergleich des zweiten Passes über alle produktiven Eingangsklassen nötig. Die frühe Übergabe an E051 bleibt ein eigener Untersuchungsgegenstand. |
| E057 `human_bound_cursor_lane_owner_transfer_xml_with_receipt` | Nach der Pausenkonsolidierung kann der Pfad noch aus dem XML rekonstruieren. In der PDF-Vorbereitung konsumiert er bereits gebundene, nicht überlappende Tausche aus `cursor_exchange_course.rs`. Beide Aufrufe sind produktiv erreichbar und haben verschiedene Eingabeverträge. | Der kanonische Besitzer der Verbindung liefert die Occurrence-Bindung. E057 materialisiert die Cursor-Reihenfolge mit bestehender Quittung. Für den frühen Aufruf fehlt eine Gleichheitsprobe der Eingabeverträge. Ein Entfall oder Vorziehen braucht den Beleg, an welcher Grenze die ursprüngliche Reihenfolge vollständig bestimmt ist. |
| `lower_dual_edge_beat_continuity_repair.rs` und die Regressionsprüfung des historischen Post-Rest-Pfads | Der Final-Notation-Reader zählt Regressionen. Der Dateiname trägt die Historie der früheren Writer. | Die Prüfungen bleiben beim Abnehmer. Eine Meldung begründet für sich keinen neuen Schreibpfad. |
| `FinalRestVerticalPlacement` nach `FinalPianoRestGlyph` | Die späte Entscheidung liest den gebundenen Schlüsselzustand und materialisiert die Anzeigehöhe. Im nativen Abschluss sind vier geschriebene Anker dokumentiert. | Die Darstellungshöhe gehört zur Pausenplatzierung mit finalem Schlüssel. Der Ablauf lässt sich erst zusammenlegen, wenn dieselbe Schlüssel- und Stimmenevidenz bereits am früheren Aufruf vollständig vorliegt. |

## Nachweisgrenze für E052

Die sichtbare Folge und der erste konkurrierende Write sind getrennt
nachzuweisen. Die E052-Heuristik liegt zwischen der Balkenpolitur und der
nächsten Render-Transaktion. Der bestehende Initial-Render-Checkpoint kann die Schlüsselentscheidung
an E024 delegieren. E024 prüft dazu die Balken und Haltebögen des tatsächlich
vorliegenden Zustands. Ein Gegenversuch prüft den vollständigen
Schlüsselverlauf ohne den historischen E052-Aufruf. Ein zweites unabhängiges
Vorkommen erhält dieselbe lokale Entscheidung. Der Korpus dient der
anschließenden Auswertung des Notenbilds, der Balkenprovenienz und der
erlaubten Ausgabeänderungen. Projektidentität und gedruckte Taktkennungen
sind reine Messadressen.
