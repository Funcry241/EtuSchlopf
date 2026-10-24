# r1038 SemanticGovernanceOptionsWarningDebtClosure

Parent: accepted r1037 `RhythmicSpineOneTenutoAuthorityRetirement`.

## Anlass

Der vollstaendige r1037-Korpus ist formal sauber: 10/10 Projekte OK, 0 Blocker, 0 Warnungen, 0 Hinweise und 10/10 final-clean. Die Launcher-Freigabe bleibt dennoch gelb, weil der Strict-Clippy-Lauf genau zwei identische Warnungen fuer dieselbe Funktion meldet (Build-Script und Take-Bootstrap-Kompilation):

```text
clippy::too_many_arguments
src/build_gate/semantic_capability_diff.rs
validate_semantic_change_with_governance_options
8/7
```

Der Launcher projiziert vorhandene Warning-Debt absichtlich gelb; dies ist keine Musik- oder r1037-Retirement-Regression.

## Closure

Die vier bereits vorhandenen Governance-Bools werden unveraendert in `SemanticGovernanceOptions` gebuendelt:

- `kodex_writer_relocation`
- `kodex_negative_check_relocation`
- `kodex_negative_check_module_relocation`
- `authority_retirement`

`validate_semantic_change_with_governance_options` erhaelt dadurch fuenf Argumente statt acht. Beide bestehenden Aufrufer uebergeben weiterhin exakt dieselben Manifestwerte. Die Reihenfolge der Pruefungen bleibt unveraendert: zuerst `validate_semantic_change_with_kodex_relocation_options`, danach bei gesetztem Retirement-Vertrag `validate_authority_retirement_semantics_v1`.

Keine Negative-Check-, Selector-, Writer-, Threshold-, MusicXML-, Renderer-, Musik- oder Release-Semantik wird veraendert. Es wird kein `#[allow(...)]`, keine Warning-Suppression und kein Fake-Use eingefuehrt.

## Pin-Koordination

Da `src/take_change_control.rs` und `src/build_gate/semantic_capability_diff.rs` bytegepinnt sind, werden ausschliesslich ihre bestehenden FNV1A64-Sollwerte in `src/identity_gate_trust_anchor.rs` auf die finalen Bytes neu gebunden.

## ChangeClass

`KodexOnly` mit:

```text
warning_debt_closure=closed_current_warning_debt_v1
```

## Erwartete Abnahme

Der Strict-Testbestand bleibt unveraendert. Erwartet werden `TotalWarnings: 0`, Clippy/Build/Tests gruen und ein nicht mehr durch Warning-Debt gelb projizierter Lauf. Keine Musikdatei soll sich durch diesen Take aendern.
