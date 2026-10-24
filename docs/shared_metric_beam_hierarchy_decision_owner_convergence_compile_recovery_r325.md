# r325 SharedMetricBeamHierarchyDecisionOwnerConvergenceCompileRecovery

## Ausgangspunkt

Autoritativ bleibt der real installierte und vollkorpus-gruene Stand v0.10.1839 r323 `StructuralUpperBeamHierarchyDecisionConvergenceRecovery`.

Der r323-Vollkorpus beweist weiterhin:

- 10/10 Projekte GREEN,
- Master: 0 Achtelnoten mit Beam-Level 2,
- Render: 12 Achtelnoten mit Beam-Level 2,
- alle 12 ausschliesslich Staff 2 / Voice 2 im bewusst unangetasteten Lower-RestBridge-Scope,
- die 467 falschen Upper-Achtel+Beam2 des r321-Standes sind verschwunden,
- `ExternalFeedbackClosureRenderWrite UpperBeamGroups` verbleibt corpusweit bei 7 (Etuede 2=1, 5=2, 7=3, 8=1).

Damit war ein unmittelbares Retirement des spaeten Upper-Materialisierungscheckpoints nicht bewiesen. r324 sollte deshalb nur die doppelte Beam-Hierarchy-Decision-Implementierung konvergieren.

## Warum r324 verworfen ist

r324 `SharedMetricBeamHierarchyDecisionOwnerConvergence` wurde installiert, passierte den produktiven Clippy- und Build-Schritt, scheiterte danach aber im integrierten `cargo test --all-targets` und wurde automatisch auf r323 zurueckgerollt.

Der Fehler war ausschliesslich testlokale Sichtbarkeit:

- der neue gemeinsame Planer `structural_metric_beam_hierarchy_plan` existierte produktiv korrekt,
- zwei neue Regressionstests lagen in `behind_bars_beam_single_owner_tests`,
- dieses `#[cfg(test)]`-Kindmodul importierte den gemeinsamen Planer nicht ueber `use super::{...}`,
- daher `E0425 cannot find function structural_metric_beam_hierarchy_plan in this scope`.

r324 ist damit nie GreenAuthority geworden und wird nicht wiederverwendet.

## r325

r325 basiert erneut direkt auf r323 und uebernimmt den gesamten fachlichen r324-Cut unveraendert:

- ein gemeinsamer herkunftsneutraler DecisionOwner `structural_metric_beam_hierarchy_plan(group, highest_owned_level)`,
- Level 1 ueber die vollstaendige bereits strukturell akzeptierte Gruppe,
- Level 2 bis maximal Level 4 nur ueber zusammenhaengende Runs von mindestens zwei Noten, deren Notenwerte das jeweilige Level tragen,
- frueher BeatScoped-Pfad konsumiert denselben Plan bis Level 4,
- spaeter Upper-Postcondition-Pfad konsumiert denselben Plan bis Level 2,
- vorhandene Level 3/4 bleiben im spaeten Upper-Pfad read-through,
- Lower RestBridge und FinalCompleteBeat bleiben unveraendert.

Die einzige Recovery-Aenderung gegenueber dem nie gruen gewordenen r324-Kandidaten ist, dass das testlokale Modul den bereits produktiv vorhandenen gemeinsamen Planer explizit aus seinem Parent-Scope importiert. Es entsteht keine weitere Funktion, kein Alias, keine neue Authority und keine semantische Tarnung.

## Erwarteter Effekt

Gegenueber r323 wird weiterhin kein MusicXML-Delta erwartet.

Harte Abnahme:

- Strict-Clippy GREEN,
- alle Cargo-Targets/Tests GREEN,
- Etuede 6 weiterhin Render Achtel+Beam2 = 5 und `UpperBeamGroups = 0`,
- Vollkorpus Master Achtel+Beam2 = 0,
- Vollkorpus Render Achtel+Beam2 = 12,
- alle 12 Staff 2 / Voice 2,
- `UpperBeamGroups` corpusweit weiterhin 7,
- Pitch/Onset/Duration/Staff/Voice/Sounding-Events unveraendert,
- Lower RestBridge unveraendert,
- FinalCompleteBeat unveraendert,
- `architectural_budget = 0`.

Nach einem realen GREEN ist die Beam-Hierarchy-Decision damit gemeinsam besessen; erst danach folgt als groesserer eigener Take die Materialisierungs-Lifecycle-Closure der sieben verbleibenden Upper-Writes.
