# RestBeatMaterializationMetricAuthorityClosure

Version: 0.10.1752 r02

`r01` wurde vor der Installation durch `#![deny(unused_variables)]` im Strict-Clippy-Lauf abgelehnt. `r02` entfernt ausschliesslich die unbenutzte lokale `expected`-Bindung im Gruppierungsschritt; die fachliche 1752-Logik bleibt unveraendert.

## Anlass

`MetricRestNotation` besitzt seit v0.10.1747 die zentrale writer-freie
Dauerplanung fuer sichtbare partielle Voice-Stille. Der aktuelle Source-Audit
zeigt aber noch zwei aktive Render-Pfade, die die Frage "darf diese Restkette
als genau eine sichtbare Pause materialisiert werden?" teilweise selbst
beantworten:

- `apply_structural_rest_beat_materialization()`
- der historische beat-lokale `2x16tel -> Achtel`-Zweig von
  `apply_upper_staff_initial_rest_and_beat_local_coalescing()`

Beide Writer bleiben fachlich und physisch unveraendert. Nur die
Dauerfreigabe wird auf denselben metrischen Planungs-Owner gebunden.

## Aenderung

`apply_structural_rest_beat_materialization()` verwendet die bereits vorhandene
scope-lokale `(expected_measure_duration, beat_duration)`-Wahrheit und fordert
vom zentralen Planner einen exakten Ein-Pausen-Plan fuer die zu
konsolidierende Restkette. Die bestehende Beat-Lane-Begrenzung bleibt ein
Writer-Scope-Guard und wird nicht zum Dauer-Owner.

Der Upper-Initial-/Beat-Local-Coalescer fragt `MetricRestNotation` jetzt auch
fuer seinen alten beat-lokalen Zwei-Sechzehntel-Fall ab. Die historischen
Kandidatenbedingungen `both_sixteenth` und `inside_single_beat` bleiben
erhalten; der Planner entscheidet nur noch, ob die gesamte bewiesene
Stillespanne als genau ein sichtbarer Restwert zulaessig ist.

## Invarianten

- Kein Writername und keine WriterReachability werden geaendert.
- Kein neues Cross-Beat-Schreibrecht.
- Keine Etueden-, Takt-, Staff- oder Onset-Sonderregel.
- Ganztaktpausen bleiben beim bestehenden kanonischen Owner.
- Voice-/Staff-/Silence-Beweis bleibt bei den bestehenden Candidate-Ownern.
- `MetricRestNotation` bleibt writer-frei und entscheidet nur Dauerstruktur.
- Bei nicht kanonischer Zeitbasis entscheidet der Planner fail-closed.
- Pitch, Onset, Dauer, Staff und Voice klingender Ereignisse werden nicht
  absichtlich veraendert.

## Regression

Die bestehenden Upper-Initial-Sicherheitsfaelle bleiben aktiv. Zusaetzlich
bindet ein Test den historischen beat-lokalen `24+24 -> 48`-Fall an den
zentralen Planner. Ein zweiter Test beweist, dass der Structural-Beat-Writer
bei einer nicht kanonischen `divisions=48`-Zeitbasis nicht mehr aus eigener
Dauerlogik schreibt.

## Abnahme

Zuerst ein Einzelprojektlauf mit Etuede Nr. 1. Erwartet werden:

- Takt 2 bleibt `192+96+48` Pause vor den zwei Sechzehnteln.
- Takt 17 bleibt chord-anker-sicher und wird nicht beatuebergreifend
  konsolidiert.
- Sounding-/All-Pitched-Timeline bleibt gegen v0.10.1751 stabil.

Ein Vollkorpuslauf ist fuer diesen Refactor nicht automatisch erforderlich.
