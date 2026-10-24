# r221 – MusicXmlNoteSchemaOrderClosureRecovery

## Zweck

r221 ist der saubere, eigenstaendige Recovery-Take fuer den von unabhaengiger MusicXML-Consumer sichtbar gemachten MusicXML-Note-Order-Fehler. Er basiert direkt auf dem autoritativ gruenen r219 und benoetigt keinen installierten r220.

Die zuvor angebotenen r220-Pakete wurden nicht installiert: das erste enthielt irrtuemlich einen unzulaessigen Root-`Cargo.lock`, das danach korrigierte Paket kollidierte bei gleicher Revision mit dem bereits vorhandenen, inhaltlich verschiedenen r220-Archiv. r221 verwendet deshalb bewusst eine neue Revision und ueberholt beide r220-Kandidaten in der Bootstrap-Auswahl. Es ist keine Umbenennung eines bereits installierten Takes und kein Revisions-Evasion-Versuch; der produktive Delta bleibt der isolierte MusicXML-Writer-Fix gegen r219.

## Produktiver Fehler

Der zentrale Duration/Type/Dot-Writer entfernte vorhandene `<type>`-/`<dot>`-Kinder und setzte sie unmittelbar hinter `</duration>` wieder ein. Dadurch konnten bereits vorhandene `<tie>`- und `<voice>`-Kinder schemawidrig hinter `<type>` geraten. unabhaengiger MusicXML-Consumer meldete diese Reihenfolge wiederholt als ungueltig.

r221 setzt `<type>` und `<dot>` nun erst vor das erste nachfolgende Note-Kind, das schema-seitig hinter ihnen stehen darf, beziehungsweise vor `</note>`. Damit bleiben insbesondere die Relationen

- `duration < voice < type`
- `duration < tie < voice < type`

erhalten.

## Nicht veraendert

Nicht veraendert werden Pitch, Onset, Dauerwerte, Voice-/Staff-Werte, Tie-Semantik, Beam-Entscheidungen, Kandidatenauswahl, MIDI-/WAV-Autoritaet, Security-Policy oder Passreihenfolge.

Der allgemeine exportseitige MusicXML-Conformance-Gate bleibt absichtlich einem spaeteren Closure-Take vorbehalten. r221 ist nur der kausale Writer-Fix und die saubere Recovery-Verpackung.
