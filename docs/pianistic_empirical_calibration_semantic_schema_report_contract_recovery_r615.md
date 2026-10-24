# r615 – PianisticEmpiricalCalibrationSemanticSchemaReportContractRecovery

## Ausgangspunkt

r615 basiert direkt auf dem real installierten GreenBase r613. Der Kandidat r614 `PianisticEmpiricalCalibrationSemanticSchemaClosure` wurde nach erfolgreichem PRE-TAKE-DIFF-GATE, Clippy und Build vom integrierten `--bin etuschlopf`-Testgate mit Exit 101 verworfen und vollständig zurückgerollt. r614 besitzt daher keine Authority.

## Exakter Gegenbefund

r614 führte einen eigenen Vertragstest `generic_observation_cannot_omit_concrete_static_or_transition_signature` ein. Dieser ruft `build_for_score_title(None)` auf und verlangt, dass der scorelose Report die drei konkreten Witness-Datenformen `StaticConfigurationSignature`, `TransitionSignature` und `SameKeyTechniqueSignature` benennt.

Die ersten beiden Bezeichnungen standen bereits im vor dem Frühreturn ausgegebenen `ProfileSemanticClosure`. `SameKeyTechniqueSignature` wurde jedoch erst im späteren `WitnessShapeCompatibility`-Abschnitt geschrieben. Dieser Abschnitt ist bei `score_title=None` nicht erreichbar. Der Test konnte daher unabhängig von der Capability-Semantik nicht grün werden.

## Recovery

r615 reintroduziert die beabsichtigte semantische Profilform direkt auf r613 und ändert gegenüber der r614-Idee ausschließlich den Reportvertrag: `ProfileSemanticClosure` benennt nun auch die explizite `SameKeyTechniqueSignature` bereits vor dem scorelosen Frühreturn.

Unverändert bleiben insbesondere:

- `StaticConfigurationSignature` als statische Whole-Configuration-Datenform,
- `TransitionSignature` als konkrete From→To-Transitionsdatenform,
- explizite Same-Key-Felder Hand/Finger/Tastenoberfläche,
- per-Hand-Abdeckung aller drei Evidenzdimensionen,
- unabhängige Rohmessungs-Provenienz,
- `CalibrationProfileBinding NONE`,
- `EmpiricalCalibration NOT_BOUND`,
- `HumanPlayabilityClaim NOT_EVALUATED`,
- keine MusicMutation, kein Writer, kein Repair und keine neue menschliche Schwelle.

## ChangeClass

`AuditOnly`.
