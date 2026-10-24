# r98 – Security Boundary Adversarial Owner Extraction

## Parent

`0.10.1839 r97` `FeedbackCompileTimeSecurityOwnerExtractionTestPinRecovery`.

Der Parent ist durch den realen Assistant-Export als `dark_green`, `FinalExit 0`, Build/Clippy/Renderer gruen und mit vollstaendigem r97-Source-Snapshot belegt.

## Ziel

Der zweite produktive Architekturcut verkleinert `src/identity_debt_build.rs` nicht nach Zeilenzahl, sondern nach Security-Authority.

Die eigenstaendige Security-Domaene

`validate_security_boundary_adversarial_completion(...)`

wird unveraendert in den echten Child-Owner

`src/build_gate/security_boundary_adversarial_security.rs`

verlagert.

Dieser Owner traegt die geschlossene adversariale 18/18-Abschlusspruefung fuer unter anderem:

- zusaetzliche oder unbekannte oeffentliche Feedback-Sinks,
- unbekannte oder doppelte Proof-Projektionsfelder,
- Retired-Detail-Reentry,
- private Debug-/Error-Leaks,
- Security-to-Music-/Calibration-Reachability,
- SecurityNegativeCheck-Kontinuitaet,
- Rollback- und Transaction-Vertraege,
- Trust-Anchor-Kontinuitaet,
- externe Locator- und neue Repair-Authority-Angriffe.

## Relocation-Vertrag

Die produktive Negativpruefung wird nicht neu implementiert.

- Symbol identisch.
- Kanonische Signatur identisch.
- Funktionskoerper identisch.
- Rejection-Surface identisch.
- Einzige Sichtbarkeitsaenderung: `fn` -> `pub(super) fn`.
- Echter `#[path]`-Child-Modulschnitt.
- Qualifizierter Aufruf aus dem bestehenden Build-Orchestrator.
- Kein `include!`.
- Kein Wrapper.
- Kein Retirement.
- Kein Shadow-Check.

Der bestehende `security_negative_check_module_relocation=closed_private_to_parent_module_relocation_v1`-Vertrag muss den Owner-Wechsel fail-closed beweisen.

## Compile-Time-Security-Owner-Domain

r97 hat beim ersten Owner-Schnitt gezeigt, dass Compile-Time-Security-Owner nicht mit der oeffentlichen Feedback-Sink-Menge gleichgesetzt werden duerfen.

r98 fuehrt deshalb eine explizite, im Build-Policy-Owner selbst bytegepinnt enthaltene geschlossene Owner-Menge ein:

- `src/identity_debt_build.rs`
- `src/build_gate/feedback_structuralization_security.rs`
- `src/build_gate/security_boundary_adversarial_security.rs`

Die bestehende `feedback_observable_sink_allowlist.tsv` wird dabei **nicht erweitert**. Damit bleibt die oeffentliche Observable-Sink-Wahrheit unveraendert, waehrend private Compile-Time-Security-Owner separat registriert sind.

Die bisherige Helper-Signatur bleibt absichtlich stabil; nur ihre interne Quelle der Owner-Wahrheit wechselt von der Sink-Rollenprojektion auf die geschlossene Compile-Time-Owner-Menge.

## Trust Anchor

Der neue Child-Owner wird direkt mit `include_bytes!` und FNV-1a im bestehenden `identity_gate_trust_anchor.rs` gepinnt. Compile-Time-Assert und Unit-Test verwenden denselben Wert.

Der geaenderte `identity_debt_build.rs`-Build-Policy-Pin wird ebenfalls in Compile-Time-Assert und Unit-Test synchron aktualisiert.

## Umfang

Vor r98:

- `src/identity_debt_build.rs`: 2173 Zeilen.

Nach r98-Kandidat:

- `src/identity_debt_build.rs`: 1836 Zeilen.
- `src/build_gate/security_boundary_adversarial_security.rs`: 359 Zeilen.

Keine Musik-, Writer-, Renderer-, Pipeline-, Console- oder Kalibrierungssemantik wird geaendert.

ChangeClass: `SecurityOnly`.
