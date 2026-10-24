# RegisterNotation Silent Bridge Persistent Scope Re-Cue · r599

## Status

Historischer installierter MusicMutation-Zwischenstand auf r597; durch r602 und den vollkorpusqualifizierten r605 ueberholt. r599 besitzt weiterhin historische Authority fuer die Einfuehrung der persistenten `octave-shift type="continue"`-Topologie, ist aber nicht mehr aktueller GreenBase.

Dieser Take war eine bewusst progressive editorielle Produktentscheidung. Er behauptet **nicht**, dass ein inner-systemischer Ottava-Re-Cue als menschliches Optimum mathematisch oder experimentell bewiesen sei. Er operationalisiert stattdessen die bereits vorhandene Trennung zwischen semantischem Register-Scope und sichtbarem Cue in einer kontrollierten, reversiblen Form.


## Recovery from rejected r598

r598 wurde nach erfolgreichem Pre-Take-Diff-Gate, Clippy und Build im integrierten `--bin etuschlopf`-Testtarget fail-closed rejected und vollständig auf r597 zurückgerollt. r598 besitzt keine Authority und wird nicht wiederverwendet.

Die Ursache war ausschließlich eine neu hinzugefügte Test-Assertion in `stille_bruecke_materialisiert_continue_ohne_scope_neustart`: Sie zählte den Text `number="1"` im gesamten `<measure>` und erfasste dadurch zusätzlich das Attribut `<measure number="1">`. Erwartet waren drei MusicXML-Oktavliniennummern; der Test zählte dadurch vier Textvorkommen.

r599 basiert erneut direkt auf dem weiterhin autoritativen r597-GreenBase. Die fachliche Produktänderung bleibt gegenüber dem rejected r598 unverändert. Korrigiert wird nur der test-only Witness: Er zählt nun drei `<octave-shift>`-Elemente und separat drei selbstschließende `number="1"/>`-Attribute der Oktavrichtungen, sodass die Measure-Nummer nicht mehr als Linienidentität fehlklassifiziert wird.

Es gibt keine neue musikalische Entscheidung, keinen zusätzlichen Writer, keine neue Schwelle und keine Policy-Aufweitung gegenüber dem rejected r598.

## Ausgangslage

r597 besitzt weiterhin die qualifizierte RegisterNotation-Architektur aus r282/r283 und den späteren Closure-Ständen. r283 darf zwei gleich dargestellte semantische Registerpläne über positive Stille zu einer Sichtspanne verbinden, wenn die bestehende relative Grenzrhythmik gilt:

```text
Luecke > 0
und
Luecke <= min(LinkeGrenzgestaltDauer, RechteGrenzgestaltDauer)
```

Dieser Take ändert diese Entscheidung **nicht**.

Er führt insbesondere keine neue Dauergrenze, keine Etüden-/Taktkennung, keinen Score, keine Gewichtung und keinen neuen Phrase-/Chunk-Owner ein.

## Produktentscheidung

Für jede bereits durch r283 qualifizierte stille Brücke gilt künftig:

```text
Semantischer Ottava-Scope:
    bleibt ein zusammenhängender Scope

Sichtlinie:
    bleibt dieselbe zusammenhängende RegisterNotation-Sichtspanne

Rechter Wiedereintritt nach der stillen Brücke:
    erhält genau einen MusicXML-4.0-<octave-shift type="continue">-Cue

Linienidentität:
    identisch zu Start und Stop desselben Scopes

Darstellung / Größe / Seite:
    identisch zum fortbestehenden Scope
```

Damit wird erstmals produktiv ausgedrückt:

```text
Scope != Cue
```

ohne daraus einen Scope-Neustart zu machen.

## Keine Stop/Start-Zerlegung

Nicht eingeführt wird:

```text
Start -> Stop -> Stille -> neuer Start -> Stop
```

Die produktive Form lautet:

```text
Start -> ... -> Stille -> Continue -> ... -> Stop
```

`continue` besitzt kein eigenes Registerentscheidungsrecht. Es darf weder Oktavtyp noch System/Staff noch Scope-Linie ändern.

## Bestehende Authority bleibt bestehen

Die Entscheidung, **ob** eine stille Brücke existiert, verbleibt vollständig bei der vorhandenen RegisterNotation-Sichtzustandslogik.

Der Writer erhält lediglich die neue, bereits geschlossene Sichtinformation:

```text
sichtwiederholungen[]
```

und materialisiert sie exakt.

Damit bleibt:

```text
Detection / Registerentscheidung
    -> RegisterNotation

Sichtzustandspfad / Brückenentscheidung
    -> RegisterNotation

MusicXML-Projektion
    -> bestehender StructuralWriteAuthority-Writer
```

Der Writer darf keine zusätzliche Brücke erfinden.

## Fail-closed Invarianten

Für einen vollständigen Lauf muss gelten:

```text
ContinueCount == stille_bruecken
StartCount    == sichtlinien
StopCount     == sichtlinien
```

Jeder Continue-Cue muss:

- am Beginn des rechts anschließenden sichtbaren Ereignisankers liegen,
- dieselbe MusicXML-`number`-Linienidentität wie Start und Stop besitzen,
- dieselbe Ottava-Größe besitzen,
- dieselbe Sichtseite (`above`/`below`) besitzen,
- ohne `<offset>` materialisiert werden,
- ohne `default-y` materialisiert werden.

Die Materialisierungsprüfung vergleicht weiterhin die tatsächlich persistierte Richtungsfolge exakt mit dem bewiesenen Sichtauftrag.

## MusicXML-4.0-Vertrag

MusicXML 4.0 definiert `octave-shift/@type="continue"` ausdrücklich als Fortsetzung eines bestehenden Oktavshifts.

Dieser Take nutzt diese Formatfähigkeit **nicht**, weil MusicXML 4.0 sie zufällig anbietet, sondern weil die Produktentscheidung einen fortbestehenden Scope mit erneuter Sichtbestätigung ausdrücken soll.

## Erwarteter r597-Korpus

Der unmittelbar vor diesem Take rekonstruierte Korpus besitzt:

```text
10 Projekte
270 semantische Registerpläne
201 produktive Sichtspannen
201 Starts
201 Stops
12 stille_bruecken
0 Continues
```

Bei unverändertem Registerplan und unveränderter r283-Brückenentscheidung ist deshalb als neue Materialisierung zu erwarten:

```text
201 Starts
12 Continues
201 Stops
```

Die zwölf vorhandenen Brücken bleiben dabei dieselben Brücken. Es gibt keine identitiespezifische Fallliste im Produktcode.

Der Forschungsstand unterscheidet darunter sieben inner-systemische Fälle und fünf Fälle an System-/Seitenübergängen. Diese Stellen dienen ausschließlich der visuellen Falsifikation; sie sind **keine** produktive Entscheidungslogik.

## Progressive Akzeptanz

Dieser Take behauptet keinen universellen Human-Factors-Beweis. Die Produktentscheidung ist absichtlich falsifizierbar.

Erste visuelle Akzeptanzadresse ist Etüde Nr. 6, insbesondere die bekannten stillen Brücken um:

```text
T29 -> T30
T33 -> T34
```

Akzeptiert wird die Änderung nur, wenn der reale Renderer die `continue`-Materialisierung als lesbare Wiederanzeige des fortbestehenden Ottava-Scope darstellt und dabei keine neue visuelle Mehrdeutigkeit, Scope-Zerlegung oder Layoutschädigung entsteht.

Ein Renderer, der inner-systemisches `continue` nicht sinnvoll sichtbar macht, falsifiziert **diese Materialisierungsstrategie**. Das ist kein Grund, mit Sonderfällen oder Geometrie-Patches nachzureparieren.

## Nicht Teil dieses Takes

Kein:

- neuer r283-Schwellwert,
- ReadabilityScore,
- PianistChunkScore,
- PhraseBoundary-Owner,
- Tragend->Stop,
- Takt-/Etüden-Sonderfall,
- Stop/Start-Repair-Pass,
- Default-y- oder Offset-Geometrie,
- MusicXML-Normalizer,
- zweiter Validator,
- musikalischer Pitch-/Rhythmus-/Voice-/Staff-Umbau.

## Falsifikationsregel

Der Take ist fachlich verworfen, wenn mindestens eines gilt:

```text
ContinueCount != stille_bruecken

Continue erzeugt einen neuen Scope statt denselben Scope fortzusetzen

Continue benötigt identity-spezifische Sonderbehandlung

Continue erzwingt einen neuen frei erfundenen Geometrie-/Schwellwert

persistierte MusicXML-Bytes verletzen den MusicXML-4.0-Produktvertrag

sichtbares Notenbild wird an den Zielstellen schlechter oder mehrdeutig
```

## Kern

r599 ist kein Beweis, dass mehr Cues immer besser sind.

r598 ist der kontrollierte progressive Versuch, eine bereits vorhandene Architekturtrennung produktiv auszuschöpfen:

```text
fortbestehende semantische Geltung
!=
nur einmal sichtbare Zustandsinformation
```

Die Innovation bleibt klein, allgemein, identity-frei, reversibel und am vollständigen Korpus falsifizierbar.
