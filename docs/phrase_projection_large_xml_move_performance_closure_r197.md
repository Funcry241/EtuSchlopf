# PhraseProjectionLargeXmlMovePerformanceClosure r197

## Ausgangspunkt

r196 ist der autoritativ gruene Stand. Der normale Clippy-Sockel liegt weiterhin bei 98 Meldungen und `architectural_budget=0`. Master- und Render-MusicXML sind gegen r195 byte-identisch.

## Befund

`materialize_final_render_notation_with_phrase_projection_authorized()` liest die finale Render-MusicXML in `source_xml`, leitet daraus anschliessend vollstaendig den besessenen `PhraseSlurEditPlan` ab und erzeugt danach einen mutierbaren Kandidaten.

Nach erfolgreicher Planbildung besitzt `source_xml` keinen weiteren Leser. Der bisherige Ausdruck `source_xml.clone()` erzeugt daher eine unnoetige Vollkopie der Render-MusicXML, bevor die geplanten Slur-Insertionen in den Kandidaten geschrieben werden.

## Aenderung

Der bereits besessene `String` wird direkt in `candidate_xml` verschoben. Plan, Insertionsreihenfolge, Parse-/Fingerprint-/Pair-Balance-Pruefungen, exakter Persistenz-Readback und Writer-Sink bleiben unveraendert.

## Sicherheits- und Musikgrenze

Keine architektur-gepinnte Datei wird veraendert. Keine Signatur, kein Selector, keine Schwelle, keine Phrase-Entscheidung, kein Writer-Recht, kein Security-Gate und kein XML-Inhalt wird veraendert. Es handelt sich ausschliesslich um verhaltensneutrale Ownership-/Allokationsbereinigung.

## Abnahme

- normaler Clippy `98 = 0 + 98`
- `RunSeverity dark_green`
- Master-MusicXML byte-identisch zu r196
- Render-MusicXML byte-identisch zu r196
