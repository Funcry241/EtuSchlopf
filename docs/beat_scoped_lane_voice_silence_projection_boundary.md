# BeatScopedLaneVoiceSilenceProjectionBoundary

## Ausgangslage

v0.10.1758/r02 hat die bereits vorhandene MusicXML-Cursor-/Lane-Beobachtung
als kanonischen schreibfreien `RenderLaneTimelineMeasure`-Owner herausgeloest.
Der produktive Beat-Scoped-Rest-Completion-Plan blieb absichtlich unveraendert:
Er erkennt Legacy-Projektionsgaps weiterhin aus sichtbaren, nicht-Chord,
gepitchten Events nach `(staff, voice)`.

Diese Staff-Projektion ist noch keine musikalische Silence-Wahrheit.

Ein Projektionsgap kann gleichzeitig enthalten:

- pitched Aktivitaet derselben seriellen Voice auf einem anderen Staff,
- Hidden pitched Events,
- Chord-Anker,
- echte Stille,
- oder sogar zwei gleich nummerierte Voices aus verschiedenen `lane_epoch`s,
  wenn ein historischer `(staff, voice)`-Vergleich ueber `<backup>` hinweg
  verbindet.

Darum darf weder ein Staff-Gap noch ein einfacher Schluesselwechsel auf
`(lane_epoch, voice)` unmittelbar Rest-Schreibrecht erhalten.

## Dieser Take

v0.10.1759 fuehrt einen writer-freien
`BeatScopedLaneVoiceSilenceProjectionPlan` ein.

Der Plan nimmt ausschliesslich zwei bereits vorhandene Wahrheiten als Input:

1. `BeatScopedRestCompletionGapPlan` als historischen Staff-Projektionsbereich,
2. `RenderLaneTimelineMeasure` als kanonische Cursor-/Lane-Beobachtung.

Fuer jeden Legacy-Gap wird zuerst geprueft, ob linker und rechter sichtbarer
gepitchter Projektionsanker eindeutig dieselbe `lane_epoch` besitzen.

Nur bei genau einer gemeinsamen Lane-Epoche wird weitergerechnet.

Danach werden innerhalb des Gap-Intervalls alle pitched Events derselben
`(lane_epoch, voice)` als Belegungsintervalle gesammelt. Dabei ist der Staff
irrelevant; Hidden pitched Events und Chord-Events bleiben vollwertige
Timeline-Belegung.

Die Belegungsintervalle werden vereinigt und vom Projektionsgap abgezogen:

`Projektionsgap - pitched coverage = writer-freie Silence-Teilintervalle`

Kann die Lane-Epoche nicht eindeutig aufgeloest werden, liefert der Plan
fail-closed:

- `lane_epoch = None`,
- keine pitched coverage,
- keine Silence-Spans.

Damit behauptet ein historischer Gap ueber `<backup>` hinweg keine neue
musikalische Wahrheit.

## Produktiver Pfad bleibt unveraendert

Der historische Kombipass berechnet den bereits seit v0.10.1757 vorhandenen
`BeatScopedRestCompletionGapPlan` jetzt genau einmal.

Der neue Silence-Projection-Plan wird daneben als Shadow-Plan aus der
kanonischen Timeline berechnet. Sein Ergebnis besitzt in dieser Version
ausdruecklich kein Schreibrecht.

Der bestehende Materialisierer konsumiert weiterhin exakt den unveraenderten
Legacy-Gap-Plan. Unveraendert bleiben insbesondere:

- `(staff, voice)` als produktive Rest-Completion-Selektion,
- `beat_bounded_rest_segments()`,
- Rest-Glyph-/Staff-Auswahl,
- `MetricRestNotation`,
- BeamPlan und Beam-Regeln,
- XML-Rebuild und Post-Materialization-Rest-Split,
- Writer-Reihenfolge,
- Master-/Render-Adoption.

Es gibt keine neue Musikmutation, keinen Selektor, keine Gate-Lockerung und
keine neue Writerwurzel.

## Generische Beweise

Die neuen Tests binden drei allgemeine Invarianten:

### Cross-Staff-Intervallkomplement

Ein historischer Staff-2-Gap `24..72`, dessen gleiche Lane/Voice auf Staff 1
`24..48` gepitcht belegt, projiziert nicht auf den ganzen Gap als Stille,
sondern nur auf:

`48..72`

### Hidden pitched chord extent

Ein unsichtbarer pitched Chord bleibt Timeline-Belegung. Seine Dauer wird beim
Intervallkomplement nicht als Stille behandelt, obwohl der Legacy-Plan
sichtbare Events fuer seine Staff-Projektion verwendet.

### Backup-Epoch-Grenze

Gleich nummerierte Voices links und rechts eines `<backup>` werden nicht zu
einer gemeinsamen Voice-Stille verklebt. Haben die beiden Projektionsanker
keine gemeinsame `lane_epoch`, bleibt die Projektion unaufgeloest.

## Reale Regressionsevidenz aus dem gruenden 1758-Stand

Der vollstaendige Etuede-1-Master enthaelt beim erneuten Legacy-Audit genau
fuenf Beat-Scoped-Rest-Completion-Projektionsgaps, alle in der bekannten
Regressionstruktur.

Vier Gaps koennen eindeutig `lane_epoch 0` zugeordnet werden und zerfallen
durch pitched Cross-Staff-Belegung in kleinere Silence-Teilintervalle.

Ein weiterer Gap (`Staff 2 / Voice 1`, `192..264`) verbindet einen linken
Projektionsanker aus `lane_epoch 0` mit einem rechten Projektionsanker aus
`lane_epoch 1`. Der neue Plan verweigert dafuer korrekt jede
Silence-Behauptung.

Die eindeutig aufgeloesten Projektionen enthalten nach Deduplizierung dieselben
vier 24-Division-Silence-Intervalle:

- `48..72`
- `96..120`
- `144..168`
- `192..216`

Diese Werte sind ausschliesslich Regressionsevidenz und keine Produktselektoren.

Der gruene 1758-Renderstand liefert beim selben Legacy-Audit keine
Rest-Completion-Gaps.

## Ownership nach diesem Cut

```text
RenderLaneTimelineMeasure
        |
        +--> Beam-/Lane-Consumer
        |
        +--> BeatScopedLaneVoiceSilenceProjectionPlan   [writer-frei]
                         ^
                         |
BeatScopedRestCompletionGapPlan [Legacy-Projektionsbereich]
        |
        +--> bestehender Rest-Materialisierer           [weiterhin produktiv]
```

Damit ist erstmals explizit getrennt:

- historische Staff-Projektion,
- kanonische Lane-/Cursor-Wahrheit,
- pitched Belegung,
- daraus abgeleitete Silence-Teilintervalle,
- und produktive Rest-Materialisierung.

## Noch nicht entschieden

Dieser Take entscheidet bewusst noch nicht:

- welcher Staff eine echte Lane-/Voice-Stille sichtbar repraesentieren soll,
- ob ueberlappende Projektionsgaps zu einem kanonischen Silence-Span
  zusammengefuehrt werden,
- welche bereits sichtbaren Restspannen von einem spaeteren Completion-Plan
  wiederverwendet werden,
- wie `MetricRestNotation` diese Spannen metrisch schreibt,
- oder wann die Legacy-Staff-Gaps produktiv abgeloest werden.

Der naechste Architekturentscheid darf erst diese Schreibprojektion klaeren.

Merksatz:

`Staff-Gap ist Kandidatenraum. Lane-Timeline ist Wahrheit. Silence ist das Intervallkomplement.`


---

## Retirement-Status ab v0.10.1762

Der hier dokumentierte Shadow war eine temporaere Architektur-Evidenz.
`LaneVoiceSilenceShadowRetirementSuperCut` entfernt seinen Runtime-Code und die
zugehoerigen Shadow-Shards vollstaendig. Dieses Dokument bleibt ausschliesslich
als historische Begruendung der damals gewonnenen Timeline-/Silence-Erkenntnisse
erhalten.
