# Renderer System-Break Rhythmic Shape Timeline Closure r30

Parent: `0.10.1839 r29`

ChangeClass: `AuditOnly`

## Anlass

Der autoritative r29-Einzellauf war mit 749/749 Tests gruen. Im anschliessenden 10/10-Korpus blieb die bestehende Renderer-System-Break- und Demand-Bindung ebenfalls gruen, die neue Rhythmic-Shape-Evidence wurde jedoch fuer zwei Projekte fail-soft als `UNAVAILABLE` ausgewiesen:

`RendererSystemBreakReadbackRhythmicShape: chord ohne vorherigen Onset`

Die betroffenen finalen Render-MusicXML-Dateien enthalten eine vom produktiven Renderer akzeptierte Serialisierung, bei der nach einer normalen Grundnote ein `<forward>` steht und danach weitere `<note><chord/>...</note>`-Elemente folgen. Der r29-Audit loeschte beim `<forward>` seinen zuletzt gebundenen Grundnoten-Onset und konnte diese Akkordelemente deshalb nicht mehr rein lesend einordnen.

Das ist eine Luecke im neuen Auditparser, keine Musik-, Planner- oder Renderer-Fehlentscheidung.

## Closure

r30 aendert ausschliesslich den read-only Timeline-Parser:

- eine normale Nicht-Chord-Note setzt weiterhin den bindbaren Grundnoten-Onset,
- `<forward>` verschiebt weiterhin nur den Measure-Cursor,
- `<forward>` verwirft den zuletzt gebundenen Grundnoten-Onset nicht mehr,
- ein danach serialisiertes `<chord/>` bleibt dadurch an die vorangegangene Grundnote gebunden,
- `<backup>` verwirft den Grundnoten-Onset weiterhin fail-closed, weil damit die Zeitachse bzw. Voice-Sequenz zurueckgesetzt wird,
- ein Chord ohne bindbare Grundnote bleibt weiterhin `UNAVAILABLE`.

Damit wird keine fehlende Note erfunden und kein Onset aus Pitch, Voice, Staff oder Projektidentitaet rekonstruiert. Es wird nur die im vorhandenen XML bereits ausgedrueckte Chord-Bindung ueber ein intervenierendes `<forward>` erhalten.

## Warum kein Modell-Take

Der r29-Vollkorpus bestaetigt weiterhin:

- 10/10 Projekte formal gruen,
- 11 zusaetzliche Renderer-Systembreaks,
- keine kollabierten geplanten Breaks,
- keine Page-Drifts,
- alle beobachteten Renderer-Splits liegen in geplanten Dreitakt-Systemen und vor deren letztem Takt,
- Headroom allein ist kein Separator,
- reine Zeitspaltenzahl allein ist ebenfalls kein Separator.

Solange die Shape-Evidence nicht fuer 10/10 Projekte verfuegbar ist, darf daraus keine neue Breiten- oder Capacity-Regel abgeleitet werden.

## Regressionstests

r30 ergaenzt zwei Tests:

1. `<forward>` zwischen Grundnote und spaeter serialisierter Chordnote behaelt den Chord-Owner; beide Noten landen auf derselben rhythmischen Spalte.
2. `<backup>` setzt den Chord-Owner weiterhin zurueck; ein anschliessender Chord ohne neue Grundnote wird abgewiesen.

Bei unveraendertem restlichem Testbestand werden nach Installation **751/751** All-Targets-Tests erwartet.

## Invarianten

- AuditOnly,
- keine Musikmutation,
- kein Demand-Recompute,
- keine Demand-Gewichtsaenderung,
- keine Capacity-Aenderung,
- keine neue Threshold-Regel,
- keine Page-/System-Auswahlaenderung,
- kein Planner-Write,
- kein Materializer-Write,
- kein Renderer-Write,
- kein zweiter Renderer-Lauf,
- keine Etueden-/Takt-Sonderlogik,
- r28 DemandBinding bleibt unveraendert,
- r27 PDF-System-Readback bleibt unveraendert.

## Abnahme

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Da Etuede 6 den neuen Randfall nicht enthaelt, beweist der anschliessende Korpuslauf die eigentliche Closure:

```powershell
.\run.cmd --all
```

Ziel ist `RendererSystemBreakReadbackRhythmicShape: Status OBSERVED` fuer 10/10 Projekte. Erst danach darf der naechste Audit die noch fehlenden horizontalen Notations-/Glyph-Overheads untersuchen.
