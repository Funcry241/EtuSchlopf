# LaneVoiceCanonicalSilenceDedupBoundary

## Ausgangslage

v0.10.1759/r01 wurde auf dem Zielsystem dunkelgruen freigegeben. Der
writer-freie `BeatScopedLaneVoiceSilenceProjectionPlan` kann damit historische
`(staff, voice)`-Projektionsgaps sicher auf die kanonische
`(lane_epoch, voice)`-Timeline projizieren und pitched Belegung als
Intervallkomplement von echter Silence-Evidenz trennen.

Diese Projektion ist absichtlich pro historischem Staff-Gap. Dadurch kann
dieselbe musikalische Silence-Evidenz mehrfach entstehen: Zwei verschiedene
Staff-Projektionsgaps derselben Lane/Voice koennen nach Abzug ihrer jeweiligen
pitched Cross-Staff-Belegung exakt denselben Silence-Teilspan liefern.

Mehrfachprojektion ist keine mehrfach vorhandene musikalische Zeit und darf
spaeter weder mehrfach materialisiert noch zur Staff-Wahl benutzt werden.

## Dieser Take

v0.10.1760 fuehrt den writer-freien
`CanonicalLaneVoiceSilenceSpanPlan` ein.

Die kanonische Identitaet eines bereits bewiesenen Silence-Spans besteht nur
aus:

- `lane_epoch`,
- `voice`,
- `start`,
- `end`.

`staff`, Quell-Gap, Insertionspunkt, Rest-Glyphe und metrische Zerlegung sind
kein Teil dieser Wahrheit.

Die Kanonisierung ist absichtlich konservativ:

1. unaufgeloeste Projektionen (`lane_epoch = None`) werden ignoriert,
2. leere oder rueckwaertige Spannen werden ignoriert,
3. exakt identische `(lane_epoch, voice, start, end)`-Tupel werden
   dedupliziert,
4. ueberlappende Spannen werden **nicht** vereinigt,
5. angrenzende Spannen werden **nicht** vereinigt.

Damit kann dieser Plan keine neue Silence-Zeit erfinden. Jeder kanonische Span
muss bereits vollstaendig als Silence-Teilspan mindestens einer aufgeloesten
1759-Projektion existieren.

## Generische Beweise

Ein Cross-Staff-Beispiel erzeugt aus zwei unterschiedlichen Staff-Gaps zweimal
denselben Lane-/Voice-Silence-Span `48..72`. Die kanonische Ausgabe enthaelt
diesen Span genau einmal.

Ein zweiter Test beweist, dass exakte Deduplizierung weder benachbarte Spannen
`24..48` und `48..72` noch einen ueberlappenden Span `60..84` vereinigt.

Ein dritter Test beweist, dass derselbe Zeitspan in verschiedenen
`lane_epoch`s oder Voices getrennte Wahrheiten bleibt und eine unaufgeloeste
Backup-Projektion kein kanonisches Silence-Tupel erzeugt.

## Reale Regressionsevidenz

Im gruenden Etuede-1-Stand erzeugen die vier eindeutig aufgeloesten
Takt-13-Projektionen insgesamt sieben Silence-Teilspannen. Exakte
Kanonisierung reduziert diese Mehrfachprojektion auf vier eindeutige
Lane-/Voice-Spannen:

- `48..72`,
- `96..120`,
- `144..168`,
- `192..216`.

Diese Koordinaten sind ausschliesslich Regressionsevidenz und keine
Produktselektoren.

## Produktiver Pfad bleibt unveraendert

Der Kombipass berechnet weiterhin denselben
`BeatScopedRestCompletionGapPlan` und gibt genau diesen unveraendert an den
bestehenden Rest-Materialisierer. Der neue kanonische Silence-Plan wird nur
writer-frei aus der bereits writer-freien Projektion berechnet und anschliessend
verworfen.

Unveraendert bleiben insbesondere:

- `(staff, voice)` als produktive Legacy-Rest-Completion-Selektion,
- `beat_bounded_rest_segments()`,
- Staff-/Rest-Glyph-Auswahl,
- `MetricRestNotation`,
- BeamPlan und Beam-Regeln,
- XML-Rebuild und Post-Materialization-Rest-Split,
- Writer-Reihenfolge,
- Master-/Render-Adoption.

Keine Musikmutation, keine neue Writerwurzel, keine Gate-Lockerung und keine
Etueden-/Taktregel.

## Ownership nach diesem Cut

```text
RenderLaneTimelineMeasure
        |
        v
BeatScopedLaneVoiceSilenceProjectionPlan   [pro Legacy-Staff-Gap]
        |
        v
CanonicalLaneVoiceSilenceSpanPlan          [staff-frei, exakt dedupliziert]

BeatScopedRestCompletionGapPlan
        |
        +----------------------------------> bestehender Rest-Materialisierer
                                             [weiterhin produktiv]
```

Der naechste Architekturentscheid darf damit erstmals getrennt fragen, wie ein
kanonischer Lane-/Voice-Silence-Span auf sichtbare Restnotation projiziert
werden soll. Diese Projektion muss Staff-Wahl, bereits vorhandene sichtbare
Restabdeckung und metrische Schreibweise getrennt behandeln; sie ist in 1760
noch nicht entschieden.

Merksatz:

`Mehrere Staff-Projektionen duerfen dieselbe Stille beweisen. Die Stille selbst existiert nur einmal.`


---

## Retirement-Status ab v0.10.1762

Der hier dokumentierte Shadow war eine temporaere Architektur-Evidenz.
`LaneVoiceSilenceShadowRetirementSuperCut` entfernt seinen Runtime-Code und die
zugehoerigen Shadow-Shards vollstaendig. Dieses Dokument bleibt ausschliesslich
als historische Begruendung der damals gewonnenen Timeline-/Silence-Erkenntnisse
erhalten.
