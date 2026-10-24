# LaneVoiceSilenceShadowRetirementSuperCut

## Stand

v0.10.1762 r01

Change-Klasse: `RefactorOnly`

## Ausgangslage

v0.10.1759 und v0.10.1760 fuehrten absichtlich zwei writer-freie Shadow-Pfade ein:

```text
BeatScopedRestCompletionGapPlan
        ↓
BeatScopedLaneVoiceSilenceProjectionPlan
        ↓
CanonicalLaneVoiceSilenceSpanPlan
        ↓
verwerfen
```

Diese Beweise waren wichtig, um Cross-Staff-Pitch, Hidden-/Chord-Anker und
`lane_epoch`-Grenzen zu verstehen, ohne die produktive Rest-Completion zu
veraendern. Der v0.10.1761-Supertake hat anschliessend Beam und Rest produktiv
entkoppelt und die lokale Rest-Segmentierungswahrheit durch `MetricRestNotation`
ersetzt. Die 1759/1760-Ergebnisse werden seitdem im Runtime-Pfad nicht mehr
verwendet; der kanonische Output wird unmittelbar verworfen.

Damit sind diese beiden Shards keine Truth-Owner mehr, sondern historische
Shadow-Evidenz.

## Retirement

Dieser Take entfernt vollstaendig:

```text
src/pipeline_shards/composer_lane_voice_silence_projection_planning_core.rs
src/pipeline_shards/composer_lane_voice_canonical_silence_planning_core.rs
```

sowie:

- beide `pipeline.rs`-Includes,
- den Runtime-Aufruf im Beat-Scoped-Rest-Stage,
- alle ausschliesslich in diesen Shards lebenden Shadow-Typen und Tests,
- `render_lane_timeline_measure_fragment()`, dessen einziger produktnaher
  Consumer der entfernte Shadow-Pfad war.

Die aktive Rust-Source sinkt dadurch um 460 Zeilen.

## Was ausdruecklich bleibt

`RenderLaneTimelineMeasure` und `RenderLaneTimelineEvent` bleiben kanonische
Observation. Sie werden produktiv von der Lower-Dual-Edge-Lane-/Beam-Architektur
verwendet und sind deshalb keine Retirementskandidaten.

Ebenso unveraendert bleiben:

- `BeatScopedRestCompletionGapPlan`,
- `plan_visible_voice_rest_completion_gaps()`,
- `MetricRestNotation`,
- die getrennten Beam-/Rest-Stages aus v0.10.1761,
- Ganztaktpausen-Ownership,
- Writer-Reihenfolge,
- Master-/Render-Adoption,
- alle Semantic-Capability- und Change-Control-Gates.

## Invarianten

Dieser Cut trifft keine Musikentscheidung. Daher gilt fuer die reale Abnahme:

```text
all-pitched timeline 1761 r03 == 1762 r01
rest structure         1761 r03 == 1762 r01
beam structure         1761 r03 == 1762 r01
PDF pixels             1761 r03 == 1762 r01
```

Abweichungen ausser Versions-/Inventarmetadaten sind ein Stop-Signal.

## Architektur danach

```text
RenderLaneTimeline Observation
        │
        └──────────────→ produktive Lane-/Beam-Consumer

PartialBeat Observation
        │
        ├──────────────→ Beam Plan → Beam Write
        │
        └──────────────→ Rest Gap Plan → MetricRestNotation → Rest Write
```

Es existiert kein paralleler Lane-/Voice-Silence-Shadow mehr im produktiven
Composer-Pfad. Historische Dokumentation der 1759/1760-Beweise bleibt erhalten,
aber nicht als Runtime-Architektur.

Merksatz:

`Ein Beweis darf wieder verschwinden, wenn seine Erkenntnis in der Architektur aufgegangen ist.`
