# r308 – SecurityPolicyValidationContextPinCoordinationRecovery

## Ausgangslage

r306 ist real dunkelgruen mit 1064/1064 Tests und 143 Strict-Clippy-Diagnosen. Die letzten zwei Architekturdiagnosen sind dieselbe `too_many_arguments`-Stelle in `semantic_capability_diff::validate_policy()`.

Der r307-Kandidat sollte diese Stelle durch einen benannten `PolicyValidationContext` schliessen, wurde aber **vor dem Kopieren** vom bestehenden Take-Change-Control abgewiesen. Grund: `src/build_gate/semantic_capability_diff.rs` ist ueber `identity_gate_trust_anchor.rs` ein gepinnter Architekturpfad. Eine Aenderung daran muss synchron die Pin-Eigentuemer `src/security_build_orchestrator.rs` und `src/identity_gate_trust_anchor.rs` beteiligen. r307 wurde nicht installiert.

## Fachliche Aenderung

Die sechs bereits vorhandenen Policy-Abhaengigkeiten werden in einem verpflichtenden `PolicyValidationContext` gebunden:

- Audit-Cross-Role-Retirement,
- Security-Capability-Retirements,
- Security-Writer-Relocation,
- Security-Negative-Check-Relocations,
- Security-Build-Owner-Rename-Writer-Relocations,
- Derived-Audit-Writer-Reachability-Narrowing.

`validate_policy()` erhaelt `change_class`, `deltas` und den Kontext. Die boolesche und mengenbasierte Policy-Logik bleibt identisch. Der Kontext hat keinen `Default` und keine Fallbackwerte.

## Pin-Koordination

Die bestehende Vertrauenskette wird synchron nachgezogen, nicht erweitert:

1. `pinned_policy_security.rs` aktualisiert ausschliesslich den erwarteten FNV1A64 der bewusst geaenderten `semantic_capability_diff.rs`.
2. `security_build_orchestrator.rs` wird als vorgeschriebener Pin-Eigentuemer beteiligt; dort wird nur die Koordination dokumentiert.
3. `identity_gate_trust_anchor.rs` pinnt die neuen Bytes von Semantic-Capability-Diff, PinnedPolicySecurity und BuildPolicy compile-time und im bestehenden Regressionstest.

Es wird kein Pin entfernt, keine Hashpruefung gelockert und kein Security-Gate umgangen.

## Sicherheitsgrenze

`ChangeClass=SecurityOnly`. Keine Musik-/Render-/Audit-Aenderung, keine Writer-/Repair-/Shadow-Schicht, keine Capability-Retirement- oder Relocation-Erklaerung und keine Erweiterung zulaessiger ChangeClasses.

## Erwartete Verifikation

- 1064/1064 Tests,
- Strict-Clippy 143 -> 141,
- `architectural_budget` 2 -> 0,
- `small_mechanical_budget` 141 unveraendert.
