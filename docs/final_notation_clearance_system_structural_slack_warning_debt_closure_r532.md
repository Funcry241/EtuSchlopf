# r532 – FinalNotationClearanceSystemStructuralSlackWarningDebtClosure

## Ausgangspunkt

Der reale r531-Einzelreexec von Etüde 1 bestätigt die beabsichtigte
Layout-Semantik:

- Takt 3 besitzt weiterhin `ClearanceEdge 1` und liegt im Zweiersystem `3-4`;
- Takt 7 besitzt weiterhin `ClearanceEdge 1` und liegt im Zweiersystem `7-8`;
- beide Pause/Flat-Stellen sind im erzeugten PDF frei;
- Pitch, Onset, Dauer, Voice, Staff und Master-MusicXML bleiben unverändert.

Der Lauf ist technisch Exit 0, aber die Freigabe bleibt gelb, weil Clippy genau eine
neue sichtbare Warning meldet:

```text
clippy::too_many_arguments
src/pipeline_shards/horizontal_layout_global_page_system.rs
build_renderer_feasibility_constraint_candidate
8/7
```

Die Warning entstand durch die r531-Erweiterung des bereits bestehenden
Renderer-Feasibility-Candidates um die Clearance-Evidenz.

## Closure

r532 schließt ausschließlich diese Warning-Debt als `RefactorOnly`.

Die beiden bereits zusammengehörigen Systemgrenzen

```text
accepted_system_capacity
max_system_measures
```

werden an der privaten Candidate-Funktion als ein einziges privates
`(usize, usize)`-Argument übergeben und unmittelbar am Funktionsanfang wieder in
dieselben beiden lokalen Werte zerlegt.

Damit sinkt die Anzahl der Funktionsargumente von 8 auf 7. Es wird kein Wert
verändert, keine Bedingung verschoben, keine Reihenfolge geändert und keine
zusätzliche Entscheidung eingeführt.

Die produktive r531-Regel

```text
clearance-tragendes System
→ nicht gleichzeitig bestehende max_system_measures vollständig ausschöpfen
```

bleibt byte-semantisch erhalten.

## ChangeClass / Warning-Debt

`ChangeClass=RefactorOnly`

`warning_debt_closure=closed_current_warning_debt_v1`

Kein `#[allow(...)]`, keine Lint-Abschwächung, kein Dummy-Use und keine neue
Musik-/Layout-Authority.

## Invarianten

- r530 CarriedState-Clearance unverändert;
- r531 Structural-Slack-Regel unverändert;
- MeasureDemands unverändert;
- AcceptedSystemCapacity unverändert;
- Systempartition-Entscheidungslogik unverändert;
- Renderer-No-Goods unverändert;
- keine Pitch-/Onset-/Duration-/Voice-/Staff-Mutation;
- keine Master-MusicXML-Mutation;
- keine Etüden-/Takt-ID-Sonderlogik.

## Erwartete Abnahme

Zuerst:

```powershell
./run.cmd --take "Etüde Nr. 1"
```

Erwartung:

- PRE-TAKE-DIFF-GATE grün;
- Build und Rust-Tests grün;
- Clippy `TotalWarnings: 0`;
- Takt 3 weiterhin System `3-4`;
- Takt 7 weiterhin System `7-8`;
- beide Pause/Flat-Stellen weiterhin frei;
- `FinalExit 0`;
- `dark_green / released`.

Danach Vollkorpus.
