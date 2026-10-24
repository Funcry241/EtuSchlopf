# r1393: Harmonische Mikro-Korrektur als Sichtbarkeitstausch (MusicMutation)

Freigegeben von Wolfgang (Variante 2, Weg A) am 14.10.2026.

## Befund

Population Row 8 (`apply_harmonic_pitch_swap_authorized`) überschrieb im ausgewählten
Master die Tonhöhe eines sichtbaren gespielten Akkordtons X mit der Tonhöhe eines
gleichzeitig liegenden, versteckten gespielten Akkordtons Y. Der versteckte Ton blieb
unverändert. Nach dem Tausch stand Y deshalb doppelt im Dokument (einmal sichtbar, einmal
versteckt), und X fehlte in jeder späteren Fassung.

Ein Test mit dem echten Graphen und dem echten Owner hat das an einem synthetischen Takt
bei allen vier Knoten gezeigt. Der r1391-Export enthält gleichzeitig sichtbare und
versteckte gleiche Tonhöhen in genau den Etüden, die Row 8 ausführt. Nach der
Projektregel darf nur das Spielbarkeitssystem gespielte Töne weglassen; die harmonische
Mikro-Korrektur gehört nicht dazu.

## Änderung

- `coincident_chord_tone_print_exchange_steps` liefert die zwei geordneten Schritte
  und schreibt selbst nichts. Die beiden bestehenden Aufrufer des Row-8-Wrappers
  (Graph-Exploration und Rematerialisierung im Portfolio) führen die Schritte
  direkt über die eingefrorenen Owner aus. Der versteckte Ton wird über #11
  sichtbar gemacht, der sichtbare Ton über #5 versteckt. Der später im Dokument
  liegende Ton wird zuerst geschrieben, damit die Offsets des früheren gültig
  bleiben.
- Offenlegung zur Writer-Reachability: Ein erster Entwurf mit eigener
  Wrapper-Funktion (`apply_…`, danach umbenannt) wies das Gate zu Recht als neue
  Writer-Schicht außerhalb von StructuralWriteAuthority ab. Deshalb gibt es keinen
  neuen Wrapper: Nur bestehende Aufrufer rufen bestehende kanonische Owner auf. Die
  Writer-Reachability verengt sich streng, weil Row 8 entfällt, und es kommt keine
  Schreibfähigkeit hinzu.
- Gebundene Töne (`<tie>` oder `<tied>`) werden auf beiden Seiten abgelehnt.
- Der Knotenbeweis `visibility_exchange_only` verlangt:
  - gleiche Noten in gleicher Reihenfolge mit unveränderter Tonhöhe, Einsatz, Dauer,
    Staff und Stimme
  - genau einen Ton, der sichtbar wird, und genau einen, der versteckt wird
  - unveränderten sichtbaren Rhythmus des Takts
    (`visible_measure_rhythm_invariant_signature`)
  - in der tonhöhenfreien Struktur (Noten, Backups, Forwards) genau zwei geänderte
    Einträge, die sich nur im Hidden-Flag unterscheiden

  Beide Signaturen stammen aus dem bisherigen Row-8-Beweis und bleiben
  unverändert.
- `MaterializedDeltaCounts` zählt den Sichtbarkeitswechsel (`visibility`). Er gehört nur
  zu dieser einen Familie und geht in `total()` ein. So bleibt das Budget
  (`total() <= 3`) gleich streng.
- Row 8 hat danach keinen produktiven Aufrufer mehr. Der eingefrorene Owner, seine
  Capability und sein Mint bleiben unverändert als `#[cfg(test)]`-Fixture der
  Receipt-Matrix erhalten. Deskriptor, Population und Gates bleiben unverändert.
- Die nur von Row 8 genutzten Tonhöhen-Beweishelfer (`true_pitch_only_mutation_proof`,
  Pitch-Block-Vergleich) und die Felder `pitch_start`/`pitch_end` des Slots entfallen.

## Musikalische Wirkung (deklariert)

- Das Notenbild zeigt an den betroffenen Stellen weiterhin den harmonisch gewählten Ton.
- Der zuvor überschriebene gespielte Ton steht jetzt versteckt im Dokument.
- Kein gespielter Ton geht verloren, keiner wird verdoppelt.
- Weil der Knoten anders materialisiert wird, kann der Solver in Einzelfällen andere
  Kandidaten übernehmen. Deshalb ist der Take MusicMutation und nicht RefactorOnly.

## Custody

Beide Schritte sind reine Sichtbarkeitswechsel bereits getragener EXACT-Owner. Der Test
`visibility_exchange_is_carried_by_the_selected_course_custody` trägt die empfangene
Course-Custody durch Freilegung und Tausch und prüft vollständigen Kontext und gleiche
Identitäten. Damit fällt die Row-8-Grenze an `MasterXmlReplacementRequest` weg; Row 1
bleibt die nächste offene Grenze.

## Tests

`tests/unit/initial_chord_course_consumer_safety.rs`:

- gebundener sichtbarer Ton wird abgelehnt
- gebundener versteckter Ton wird abgelehnt
- jeder gespielte Ton bleibt erhalten, nur der Sichtbarkeitsstatus wechselt
- ein versteckter Ton wird höchstens einmal gezeigt
- die Custody trägt den Tausch
- jeder Knoten des echten Graphen zeigt, statt zu überschreiben; geprüft bei zwei
  verschobenen Taktnummern
