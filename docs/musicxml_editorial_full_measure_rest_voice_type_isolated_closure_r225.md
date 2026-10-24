# MusicXmlEditorialFullMeasureRestVoiceTypeIsolatedClosure r225

r225 basiert direkt auf dem real installierten und gruen gelaufenen r223 `MusicXmlExistingWriterChildOrderRecovery`.

Der reale r223-Lauf von Etuede Nr. 6 ist ein wichtiger Kompensations-Kontrollpunkt:

- Build, Strict-Clippy und Renderer sind gruen; RunOutcome ist `dark_green`, `released`, FinalExit 0.
- Gegen den realen r221-Export ist das gerenderte PDF auf allen sechs Seiten pixelidentisch.
- Werden direkte `<note>`-Kinder nur fuer den Vergleich in MusicXML-Schema-Reihenfolge kanonisiert und reiner Whitespace ignoriert, sind r221 und r223 fuer Master und Render strukturell identisch. Damit hat r223 fuer diesen Test nur Child-Reihenfolge/Whitespace veraendert, nicht die musikalische XML-Bedeutung.
- Die `notations`-vor-`beam`-Restklasse ist in Etuede 6 geschlossen.
- Render hat keine direkte Note-Child-Ordnungsverletzung mehr.
- Im Master bleibt genau eine `type`-vor-`voice`-Verletzung: Takt 22, sichtbare Ganzpausen-Note, Voice 2, Staff 2, Duration 384.

Die verbleibende Master-Verletzung entsteht nicht in den bereits korrigierten Duration-/Type-Writern. Sie wird direkt durch die bestehende Funktion `editorial_voice_full_measure_rest_note()` in `cross_staff_editorial_voice_math_gate_tail.rs` serialisiert. Dort stand bisher:

`duration -> type -> voice -> staff`

r225 aendert ausschliesslich diese bestehende Literal-Reihenfolge auf:

`duration -> voice -> type -> staff`

Es wird keine neue Funktion, kein neuer Writer, kein Sink, kein Normalizer und keine neue Repair- oder Selection-Authority eingefuehrt. Pitch, Onset, Durationwert, Restinhalt, Voicewert, Staffwert, Kandidatenauswahl, Akzeptanzlogik und alle nachgelagerten musikalischen Entscheidungen bleiben unveraendert.

Dieser Take ist bewusst klein, um einen moeglichen Kompensationseffekt kausal pruefbar zu halten. Nach dem realen Lauf sind getrennt zu pruefen:

1. MusicXML korrekter: Master `type`-vor-`voice` 1 -> 0; Render bleibt 0.
2. Interne musikalische Bedeutung unveraendert.
3. Notenbild unveraendert oder besser. Falls es schlechter wird, ist das Evidenz fuer eine verborgene Fehlkompensation und kein Grund, den XML-Fehler wieder einzubauen.

Die FLP->MIDI-Seed-Terminologie und ein allgemeines read-only MusicXML-Conformance-Gate bleiben weiterhin getrennte Folgethemen.
