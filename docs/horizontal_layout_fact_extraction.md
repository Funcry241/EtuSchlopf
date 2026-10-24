# HorizontalLayoutFactExtraction

## Zweck

Dieser `RefactorOnly`-Take ist Migrationsstufe 1 der horizontalen Layoutarchitektur. Er aendert keine Layoutpolicy und materialisiert keine neue Geometrie. Stattdessen trennt er erstmals die read-only Beobachtung eines Taktes von den bestehenden Legacy-Entscheidungen fuer Measure-Width, VisualLayoutPressure und PianisticReadability.

Der Take fuehrt `MeasureHorizontalLayoutFacts` als gemeinsamen, nicht schreibenden Snapshot beobachtbarer Taktmerkmale ein. Die vorhandenen Legacy-Formeln werden aus diesen Fakten rekonstruiert und behalten ihre bisherigen Schwellen, Gewichte, Sortierungen, Caps und Writer unveraendert.

## Neue Fact-Grenze

`src/pipeline_shards/horizontal_layout_fact_extraction_core.rs` sammelt pro Takt ausschliesslich Rohbeobachtungen, darunter:

- struktureller Initial-Measure-Kontext aus der Reihenfolge der Measure-Elemente, ohne Taktidentitaet;
- Anzahl von Note-, Beam-, Alter-, Octave-Shift-, Clef- und Direction-Elementen;
- Achtel-, 16tel- und 32stel-Vorkommen;
- extreme Registerereignisse;
- unterschiedliche Voice- und Staff-Werte;
- untere Clef-, sichtbare Pitch-/Non-Rest- und Rest-Vorkommen;
- Octave-Shift-Starts.

Der Fact-Typ enthaelt bewusst keine Felder wie `score`, `target_width`, `should_break`, `preferred_system`, `page_target` oder sonstige Layoutentscheidungen.

## Legacy-Layout-State bleibt getrennt

Die heutige `PianisticReadability`-Policy verwendet bereits materialisierte Geometrie als Eingang: ein vorhandenes `<print new-system="yes">` veraendert den Score. Dieser Zustand wird deshalb nicht als musikalischer Fact umgedeutet.

`LegacyHorizontalLayoutStateObservation` kapselt ausschliesslich die fuer exakte Verhaltensparitaet noch benoetigte Legacy-Oberflaeche. Dazu gehoeren der bestehende `new-system`-Zustand und die historisch exakt doppelt-quotierte Staff-2-Clef-Zaehllogik. Diese Struktur ist ausdruecklich kein zukuenftiger Evidence-Vertrag und keine neue Authority.

## Unveraenderte Legacy-Policy

Unveraendert bleiben insbesondere:

- `dense_register_notation_score_and_width` mit denselben Gewichten, Schwelle 72 und Cap 1680;
- `visual_layout_pressure_score` mit denselben Schwellen, Gewichten und Reasons;
- Pressure-Point-Schwelle 18;
- Systembreak-Kandidatenschwelle 30, `max_extra_breaks` und `min_gap`;
- `PianisticReadability` mit denselben 62/82-Schwellen sowie 2050/2300 Width-Zielen;
- `compute_page_breaks` und `compute_system_breaks`;
- das bestehende 2-Takt-Grundraster;
- `FinalShortSystemBalance`;
- alle bestehenden Horizontal-/Vertikal-Writer und Structural-Write-Authorities;
- Passorder und Pipelinezeitpunkt der Layoutschichten.

Insbesondere werden `staff-distance` und `octave-shift default-y` nicht in eine neue horizontale Authority verschoben. Der Take erzeugt keine allgemeine Layout-Authority und keinen Planner.

## Paritaetsvertrag

Die Legacy-Projektionen fuer DenseRegister, VisualLayoutPressure und PianisticReadability werden aus dem neuen Fact-Snapshot berechnet. Eingebaute Regressionstests vergleichen die neuen Projektionen mit den vorherigen Inline-Formeln auf unterschiedlichen XML-Oberflaechen, einschliesslich strukturellem Initial-Measure-, Systembreak- und Single-Quote-Faellen.

Zusaetzlich wurde vor Paketierung der gelieferte 10er-Korpus read-only ausgewertet: 383 finale Master-Takte plus 383 zugehoerige Render-Takte ergeben 766 Taktinstanzen beziehungsweise 2298 Projektionsvergleiche ueber die drei Legacy-Auswertungen. Ergebnis: 0 Abweichungen.

## Invarianten

Dieser Take soll beweisen:

```text
HORIZONTAL_LAYOUT_FACTS_ARE_READ_ONLY
HORIZONTAL_LAYOUT_FACTS_CONTAIN_NO_DECISION
LEGACY_LAYOUT_POLICY_PROJECTION_IS_PARITY_PRESERVING
LEGACY_LAYOUT_STATE_IS_NOT_MUSICAL_EVIDENCE
NO_HORIZONTAL_LAYOUT_POLICY_CHANGE
NO_HORIZONTAL_LAYOUT_WRITER_CHANGE
NO_PASSORDER_CHANGE
NO_ETUDE_OR_MEASURE_SPECIFIC_LAYOUT_POLICY
PRODUCTIVE_HORIZONTAL_LAYOUT_FACTS_CONTAIN_NO_MEASURE_IDENTITY
```

Keine Musikregel, kein Threshold, keine neue Reparatur-/Recovery-Schicht, keine Neuro-Evidence-Semantik und keine Security-Authority werden geaendert.

## Abnahme

Autoritativ unter Windows:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Erwartung: Strict-Gates GREEN, unveraenderte musikalische und horizontale Projektion sowie kein neuer Layout-Writer. Danach ist ein kompletter 10er-Korpuslauf die bevorzugte zusaetzliche Paritaetsabnahme fuer Breaks, Pages und Widths.

Take 2 darf erst auf dieser geschlossenen Fact-Grenze aufbauen. Eine neue Layoutpolicy ist ausdruecklich nicht Bestandteil dieses Takes.
