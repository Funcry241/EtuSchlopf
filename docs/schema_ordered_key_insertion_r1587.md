# r1587 – Pausenerhaltung mit schemafester Tonarteinfügung

Parent ist der nach r1586 wiederhergestellte, nativ angenommene r1585. Die
Pausenkorrektur aus r1586 wird unverändert mitgeführt. r1586 selbst ist nicht
angenommen: Der native Lauf bestand 2.901 Tests und Clippy ohne Warnung, brach
aber in Etüde 3 am PersistedByteXsd40Gate ab. Die gemeldete Kindfolge lautete
`clef, key`. Neun andere Projekte liefen erfolgreich; der Take wurde vollständig
zurückgerollt.

## Ursache und bestehender Owner

`measure_key_signature_edit` und die parallele Kandidatenkonstruktion hängten
eine fehlende Tonart vor `</attributes>` an. Das ist für einen Block mit
bereits vorhandenen Schlüsseln, Taktarten oder anderen nachgelagerten Elementen
schemawidrig. Die gebundene normative Quelle ist
`docs/normative_sources/musicxml/4.0/schema/musicxml.xsd`, Typ `attributes`:
editorial, divisions, key, time, staves, part-symbol, instruments, clef,
staff-details, transpose/for-part, directive, measure-style.

Der vorhandene Tonartplan bindet eine neue Tonart nun vor dem ersten direkten
Kindelement nach editorial/divisions ein. Kommentare und verschachtelter Text
sind keine Einfügegrenze. Alle vorhandenen Bytes und alle Ereignispositionen
bleiben erhalten. Unlesbare Attributfragmente binden keine neue Einfügung.
Vorhandene Tonarten werden wie zuvor ersetzt; bei fehlendem Attributblock
bleibt die bisherige Konstruktion bestehen.

Die Kandidatenkonstruktion verwendet jetzt dieselbe lokale Editgeometrie wie
die bestehende exakte Quittung. Es gibt keinen neuen Writer, Sink oder
Reparaturpass. Die tonale Auswahl, Zieltonarten und musikalischen
Entscheidungsbedingungen sind unverändert. Kein Schema, Gate, Pin oder
Prüfer wurde geändert.

## Prüfungen

Zwei zusätzliche synthetische Tests prüfen die gebundene vollständige
SchemaSchlopf-r021-XSD-Validierung: gültiger Eingang, reproduzierbar ungültige
alte Anhängung und gültige neue Einfügung für Schlüssel, Taktart, Systemzahl,
Klammer/Instrumentzahl, Systemdetails, Transposition und Mehrtaktpause.
Sie prüfen Originalbyteerhaltung, die ursprüngliche exakte Quittung,
Taktverlagerung, unabhängige Verdopplung und Ablehnung defekter Fragmente.
Die sechs Pausenerhaltungsproben aus r1586 bleiben unverändert.

## Native Abnahme

`run.cmd --take --all` ohne Zusatzflag. Erforderlich bleiben alle zehn Projekte,
die unveränderten XSD- und musikalischen Schutzprüfungen sowie 1T13 und 2T29
auf erhaltene korrekt geteilte untere Pausen. 6T6 ist ein Regressionsfall. Die
untere Balkenbrücke in 1T13 war im r1586-Readback noch nicht hergestellt und
wird durch diese Serialisierungskorrektur nicht als gelöst erklärt.
