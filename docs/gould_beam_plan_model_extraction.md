# GouldBeamPlanModelExtraction

## Ausgangspunkt

Autoritativer Quellstand ist v0.10.1596 r02. Die Voranalyse zeigte, dass der
produktive Gould-Pfad bereits seit v0.10.1212 zentralisiert ist. Es existieren
bereits:

1. eine read-only Beobachtung der Render-MusicXML,
2. ein identitaetsfreier Gould-Detector,
3. eine source-bound Autorisierung mit move-only Token,
4. ein zentraler tokengebundener Writer,
5. eine bytegenaue Legacy-Shadow-Paritaet und
6. ein typisierter produktiver Render-Sink.

Der erste Balken-Take darf deshalb keinen zweiten zentralen Writer einfuehren.
Sein sinnvoller Schnitt ist die Entfernung der Gould-eigenen Zwischenmodelle.

## Aenderung

Neu ist `structural_write_authority/beam_plan.rs` mit einem kleinen gemeinsamen
Planmodell:

- `BeamEvent` fuer relative Onset-/Dauer-Ereignisse,
- `BeamTimeline` fuer die metrische Eingabesicht,
- `BeamState` fuer typisierte Zielzustaende,
- `BeamEdit` fuer Eventordinal, Balkenebene und Zielzustand,
- `BeamPlan` als Regelergebnis.

Der Gould-Detector liefert jetzt direkt einen `BeamPlan`. Entfernt sind die vier
Gould-spezifischen Modelle:

- `GouldMetricBeamEvent`,
- `GouldMetricBeamView`,
- `GouldMetricBeamState`,
- `GouldMetricBeamWriteIntent`.

Der Autorisierungstoken speichert den unveraenderten, erneut detektierbaren Plan
anstelle separater Ordinal- und State-Kopien. Der Writer akzeptiert weiterhin
nur den move-only Token und liest die drei autorisierten Edits aus dem Plan.
Rohe Plaene, Zielbereiche oder freie Beam-Zustaende koennen den Writer nicht
erreichen.

## Unveraenderte Autoritaet

Unveraendert bleiben:

- die 1/4-1/2-1/4-Erkennungsregel,
- die Auswahl der letzten drei Ereignisse einer Voice-Timeline,
- Quarter-Ausrichtung und Kontinuitaetspruefung,
- Source-Document- und Note-Fingerprints,
- beobachtete Ausgangs-Beamzustaende,
- gebundene XML-Zielbereiche,
- der zentrale Gould-Writer,
- Beam-only-Delta-, Parse- und Note-count-Pruefung,
- die bytegenaue Legacy-Shadow-Paritaet,
- der einzige produktive Render-Sink,
- Writer- und Pipeline-Reihenfolge.

Der Identity-Free-Quellscan umfasst nun sowohl den Detector als auch das neue
Planmodell. Das Planmodell enthaelt keine Projekt-, Takt-, Pitch-, Staff-,
Voice- oder Dateidentitaet.

## Statische Evidenz

Die alte und die neue Detektorform wurden mit 200.000 zufaellig erzeugten
Kombinationen aus Quarter-Units und Ereignisfolgen gegeneinander geprueft. Fuer
jede Kombination waren Ablehnung beziehungsweise die drei Edits
`begin/continue/end` identisch.

Es wurde keine neue `#[allow(dead_code)]`-Stelle, keine Dummy-Lesung, kein
History-Sink und kein zusaetzlicher Writer eingefuehrt.

Physische Rust-Zeilen unter `src`:

- v0.10.1596 r02: 114.971
- v0.10.1597 r02: 115.024
- Netto: **+53 Rust-Zeilen**

Die Zunahme ist das einmalige gemeinsame Planfundament. Gleichzeitig sinkt der
Gould-Detector von 111 auf 37 Zeilen und der Token fuehrt keine parallelen
Ordinal-/State-Felder mehr.

## r02 Strict-Clippy-Closure

Der erste reale Gate-Lauf zeigte einen zu breiten crate-weiten Reexport.
`BeamEdit`, `BeamPlan` und `BeamState` werden nur innerhalb von
`structural_write_authority` direkt aus `beam_plan` importiert. Am Modulrand
werden ausschliesslich `BeamEvent` und `BeamTimeline` von der Pipeline
verwendet. r02 verengt deshalb nur diesen Reexport; das Planmodell und alle
produktiven Autoritaeten bleiben unveraendert.


## Abnahme

Verbindlicher Lauf:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Erwartet werden Strict-Clippy 0, identische Master-/Render-MusicXML und sechs
pixelidentische PDF-Seiten. Bis dahin bleibt v0.10.1596 autoritativ.
