# BeatRestMetricAuthorityBeamDecouplingSuperCut

## Stand

v0.10.1761 r03

Change-Klasse: `MusicMutation`

## Ziel

Der historische Pass `enforce_beat_scoped_beaming_and_rest_completion_in_xml()`
hat Beam-Planung, Rest-Completion, Rest-Dauersegmentierung, XML-Mutation und eine
nachgelagerte Rest-Reparatur in einer gemeinsamen Writer-Grenze gekoppelt.

v0.10.1761 zerlegt diese produktive Kopplung unterhalb der stabilen Orchestrator-Schnittstelle in zwei ausdrueckliche Stages:

```text
enforce_beat_scoped_beaming_and_rest_completion_in_xml()  // Kompatibilitaetsfassade
        ↓
enforce_beat_scoped_beaming_in_xml()
        ↓
enforce_beat_scoped_rest_completion_in_xml()
```

Die Fassade besitzt keine eigene Beam-, Rest-, Gap-, Silence- oder Metric-Entscheidung; sie sequenziert nur die beiden getrennten Stages. Master und Render behalten dadurch den bytegleichen 1760-Orchestrator. Der Beam-Stage erzeugt keine Pausen mehr. Der Rest-Stage besitzt keine Beam-Zustaende mehr.

r01 wurde vor jeder Installation vom Semantic-Capability-Diff korrekt verworfen, weil der MusicMutation-Candidate den `ProofReentryReader` `pipeline_orchestrator_initial_render_stage.rs::run_initial_render_transaction_stage` mit veraendert hatte. r02 setzt diese Datei exakt auf den autoritativen 1760-Stand zurueck. Die Capability-Policy bleibt unveraendert; die fachliche Entkopplung wird vollstaendig unterhalb der bestehenden Schnittstelle geschlossen.

## r03: Full-Measure-Rest-Ownership-Closure

r02 lief real dunkelgruen und released. Der direkte Vergleich der realen 1760- und 1761-Artefakte bewies gleichzeitig: Die gepitchte Timeline blieb in Master und Render exakt invariant, aber im Render wurde eine bestehende Ganztaktpause mit Dauer 384 in vier Viertelpausen 96+96+96+96 umgeschrieben. Das war kein Timeline-Fehler, aber eine unerwuenschte Ownership-Uebernahme durch den partiellen MetricRestNotation-Pfad.

r03 laesst `plan_exact_metric_visible_rest_durations()` deshalb fuer `start == 0 && duration == measure_duration` fail-closed entscheiden. Vollstaendige Measure-Spannen werden nicht normalisiert und bleiben beim bereits vorhandenen Full-Measure-Rest-Owner. Partielle Spannen, Rest-Completion, Beam-/Rest-Entkopplung und der exakte Dauersummenbeweis bleiben unveraendert.

## MetricRestNotation als einziger Rest-Dauer-Owner dieses Pfades

Die lokalen Helper

```text
beat_rest_is_bounded()
beat_bounded_rest_segments()
```

werden vollstaendig entfernt.

Sowohl neu materialisierte Rest-Completion-Spannen als auch sichtbare partielle
Pausen werden nun ausschliesslich ueber
`model::metric_rest_notation::plan_partial_visible_rest_durations()` geplant.
Der writer-freie Adapter `plan_exact_metric_visible_rest_durations()` akzeptiert
einen Plan nur, wenn die Summe der geplanten Restdauern exakt der angeforderten
Dauer entspricht. Ein leerer oder nicht dauererhaltender Plan schreibt nichts.

Damit bleibt die vom bestehenden Rest-Completion-Gap-Plan erzeugte Cursorzeit
erhalten, waehrend die sichtbare metrische Schreibweise zentral entschieden
wird. Der Take darf deshalb Rest-Glyphen bzw. deren Gruppierung aendern, aber
nicht die eingefuegte Gesamtdauer eines akzeptierten Gap-Plans.

## Bewusst unveraenderte Gap-Semantik

Der produktive Selektor

```text
plan_visible_voice_rest_completion_gaps()
```

bleibt in v0.10.1761 unveraendert. Insbesondere wird die historische
`(staff, voice)`-Gap-Auswahl nicht gleichzeitig mit dem MetricRestNotation-
Cutover ersetzt.

Grund: Die in v0.10.1759/1760 bewiesene kanonische Lane-/Voice-Silence ist eine
Truth-Evidenz fuer die resultierende Timeline. Vor der Rest-Materialisierung ist
die einzufuegende Zeit im MusicXML-Cursor jedoch noch nicht notwendigerweise als
Stille vorhanden. Ein direkter Einsatz als neuer Writer-Selektor wuerde deshalb
wieder Analyse- und Mutationssemantik vermischen.

Die vorhandene Lane-/Voice-Silence-Analyse bleibt im Rest-Stage writer-frei und
besitzt weiterhin kein Staff-, Glyph-, Insertions- oder Beam-Schreibrecht.

## Nachgelagerte Rest-Lesung

Rest-Completion kann die reale Cursorposition bereits vorhandener sichtbarer
Pausen verschieben. Deshalb bleibt vorerst genau eine zweite Lesung innerhalb
des Rest-Owners erhalten:

```text
normalize_metric_visible_rests_in_materialized_measure_only()
```

Diese Funktion besitzt kein Gap-, Silence- oder Beam-Entscheidungsrecht. Sie
darf ausschliesslich bereits materialisierte sichtbare Restdauern erneut durch
denselben `MetricRestNotation`-Owner normalisieren. Die fruehere lokale
Beat-Segmentierungsregel existiert dort nicht mehr.

Damit ist die alte fachlich gemischte Repair-Schleife auf eine einzige
Rest-Domain-Normalisierung reduziert. Ihre spaetere Entfernung kann separat
ueber eine strukturierte Vorabplanung erfolgen, ohne erneut Beam- oder
Silence-Wahrheit hineinzuziehen.

## Invarianten

Fuer jeden akzeptierten Metric-Rest-Plan gilt:

```text
sum(planned_rest_durations) == requested_duration
```

Fuer Rest-Completion bleibt die Menge und Gesamtdauer der produktiven
Legacy-Gaps unveraendert. Es gibt keine Etueden-, Takt-, Staff-, Pitch- oder
Onset-Sonderregel, keine Schwellenwertaenderung und keine Gate-Lockerung.

Der gezielte Lauf fuer Etuede 1 muss insbesondere den bekannten Takt-13-
Regressionfall gegen eine erneute gepitchte Timeline-Verschiebung abdecken.

## Architektur nach dem Cut

```text
PartialBeat / RenderLane Observation
            │
      ┌─────┴─────┐
      │           │
      ▼           ▼
  Beam Plan   Rest Gap Plan
      │           │
      │      Lane/Voice Truth Evidence
      │           │
      │      MetricRestNotation
      │           │
      ▼           ▼
 Beam Write    Rest Write
                  │
                  ▼
          Metric-only Readback
```

Der Writer muss damit nicht mehr gleichzeitig Beam- und Rest-Wahrheit besitzen.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 1"
```

Ein Korpuslauf ist fuer die erste Abnahme nicht erforderlich. Erst wenn diese
Regressionadresse dunkelgruen bleibt, ist der Supertake als neuer
Architekturstand autoritativ.


---

## Nachfolge: Shadow-Retirement in v0.10.1762

Nach der realen dunkelgruenen und artefaktidentischen Abnahme von v0.10.1761/r03
werden die in diesem Dokument noch erwaehnten writer-freien 1759/1760-
Lane-/Voice-Silence-Shadows in v0.10.1762 vollstaendig aus dem Runtime-Pfad
entfernt. `RenderLaneTimeline` bleibt als produktiv konsumierte kanonische
Observation bestehen.
