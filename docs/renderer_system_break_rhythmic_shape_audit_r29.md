# Renderer System-Break Rhythmic Shape Audit r29

Parent: `0.10.1839 r28`

ChangeClass: `AuditOnly`

## Ausgangslage

r28 hat den Renderer-System-Break-Readback an die bereits vom autoritativen Joint Page/System Planner berechneten Systemlasten und `AcceptedSystemCapacity` gebunden. Der anschliessende Vollkorpus ist 10/10 formal gruen und liefert fuer alle zehn Etueden gebundene Load/Capacity/Headroom-Evidence.

Die Korpusmessung widerlegt eine einfache globale Headroom-Schwelle:

- 160 geplante Systeme,
- 11 zusaetzliche Renderer-Systembreaks,
- 0 kollabierte geplante Systembreaks,
- 0 Page-Drifts,
- 97 geplante Zweitakt-Systeme, davon 0 mit Renderer-Split,
- 63 geplante Dreitakt-Systeme, davon 11 mit Renderer-Split,
- alle 11 Breaks liegen vor dem letzten Takt des betroffenen Dreierverbands,
- Split-Systeme besitzen Headroom von 7 bis 97,
- gehaltene Dreierverbande existieren sogar mit Headroom 1 und 3.

Damit darf `AcceptedSystemCapacity` nicht einfach pauschal abgesenkt werden.

## Hypothese

Die bisherige Demand-Funktion bewertet unter anderem Note-, Beam-, Alter-, Richtungs- und Kurznoten-Dichte. Fuer die reale horizontale Mindestbreite ist aber zusaetzlich entscheidend, wie viele **verschiedene rhythmische Zeitpositionen** MuseScore innerhalb eines Takts setzen muss.

Vier Akkordtoene auf demselben Onset benoetigen horizontal nicht denselben Platz wie vier nacheinander liegende Tonereignisse. Eine rohe Notenzahl kann diese beiden Faelle nicht unterscheiden.

r29 macht diese Hypothese erstmals maschinenlesbar, ohne daraus bereits eine Regel abzuleiten.

## Read-only Zeitspalten

Der Audit liest ausschliesslich das bereits finale Render-MusicXML:

- Measure-Timeline beginnt bei Offset 0,
- normale Note: aktuelle Cursorposition ist ihr Onset,
- `<chord/>`: gleicher Onset wie die unmittelbar vorherige Grundnote,
- normale Note mit Duration: Cursor wird um `<duration>` fortgeschrieben,
- Grace-Note: kein Duration-Fortschritt,
- `<backup>`: Cursor wird fail-closed zurueckgesetzt,
- `<forward>`: Cursor wird vorwaerts gesetzt,
- mehrere Stimmen teilen dadurch eine gemeinsame Measure-Zeitachse.

Pro Takt werden nur beobachtet:

- `columns`: verschiedene Note/Rest-Onsets,
- `pitchedColumns`: verschiedene klingende Onsets,
- `notes`: Anzahl Note-Elemente,
- `maxStack`: groesste Anzahl Note-Elemente auf derselben Zeitspalte.

Diese Werte sind **keine neue Demand-Funktion**.

## Systemprojektion

Fuer jedes bereits geplante System wird sinngemaess ausgegeben:

`13-15=len3/cols15+16+15/pitchedCols15+16+15/notes23+32+35/maxStack.../split15`

Zusaetzlich:

- Anzahl Zwei- und Dreitakt-Systeme,
- Anzahl split-betroffener Zwei- und Dreitakt-Systeme,
- ob alle eingefuegten Breaks unmittelbar vor dem letzten Takt ihres geplanten Systems liegen.

Der bestehende r28-Load/Capacity/Headroom-Readback bleibt unveraendert daneben bestehen.

## Fail-soft

Die neue Formevidenz besitzt keine Freigabe- oder Writer-Autoritaet.

Bei:

- fehlendem Render-MusicXML,
- nichtnumerischen/verschobenen Measure-Nummern,
- ungueltigen Duration-Werten,
- Timeline-Unterlauf,
- Chord ohne bindbaren vorherigen Onset,
- MeasureCount-Drift,

wird nur

`RendererSystemBreakReadbackRhythmicShape: Status UNAVAILABLE`

projiziert.

Der bestehende Renderer-System-Break-Readback, die Musik und die Release-Entscheidung bleiben davon unberuehrt.

## Invarianten

- keine Musikmutation,
- kein Demand-Recompute,
- keine Demand-Gewichtsaenderung,
- keine Capacity-Aenderung,
- keine neue Threshold-Regel,
- keine Page-/System-Auswahlaenderung,
- kein Writer,
- kein zweiter Renderer-Lauf,
- kein OCR,
- Width bleibt RendererAuto,
- One Joint Page/System Plan bleibt unveraendert,
- r28 DemandBinding bleibt autoritativ fuer die bestehende Planner-Last.

## Regressionstests

r29 fuegt drei Audit-Regressionstests hinzu:

1. Akkordnoten werden auf eine gemeinsame rhythmische Spalte kollabiert.
2. Zwei Stimmen werden ueber `<backup>` auf dieselbe Measure-Zeitachse gebunden.
3. Die Systemprojektion bindet einen Renderer-Split an die Form des geplanten Systems und erkennt den Break vor dem letzten Takt.

Bei unveraendertem restlichem Testbestand werden nach Installation **749/749** All-Targets-Tests erwartet.

## Naechster Schritt

Nach einem gruenden r29-Einzellauf wird erneut der 10/10-Korpus gemessen.

Erst dann wird geprueft, ob:

- Zeitspaltenzahl,
- Zeitspaltenverteilung innerhalb eines Dreierverbands,
- Notenstapelung pro Spalte,
- oder eine Kombination aus vorhandener Demand-Evidence und Zeitspalten

die Renderer-Splits konsistent erklaert.

Falls auch diese Evidence keinen ausreichend sauberen Separator liefert, wird **keine** heuristische Schwelle erfunden. Dann ist als naechste Ebene direkte PDF-Geometrie-Evidence des Renderers zu untersuchen.
