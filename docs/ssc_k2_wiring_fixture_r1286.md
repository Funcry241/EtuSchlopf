# r1286 – K2-Governance-Revision: Wiring im #57-Testbaum

Parent: accepted r1285 `SscAllCommitMeasurementV4`. Klasse `KodexOnly`, benannte Revision `SscK2GovernanceRevisionKodex`.

## Befund

Beim Bau von Take 5b (Enforcement) scheiterten drei K2-Tests (`proof_bundle_rejects_any_other_proof_or_identity_bytes`, `render57_impl_pin_detects_helper_drift_behind_an_unchanged_wrapper`, `render57_positive_requires_ready_and_the_complete_accepted_construction`).

`positive_render57_tree` legt die **live** `src/master_state_identity.rs` in einen Testbaum, dessen Wiring-Datei das fest eingebaute leere `FIXTURE_WIRING` ist. Jede nach Vertrag v4 A7 zulässige Verdrahtung von `commit_owner_receipts` wird dort deshalb als „V2 escapes observation“ gewertet, obwohl die echte Admission sie annimmt. Dasselbe Muster wie vor r1280: ein Kodex-Test liest eine Datei, die ein späterer Take legitim ändert, ohne deren zugehörige Daten.

## Änderung

- `positive_render57_tree` setzt zusätzlich die installierte `src/build_gate/ssc_verdict_wiring_v1.txt`. Live-Identitätsdatei und live Wiring werden damit gemeinsam beurteilt. Mit leerem Wiring (dieser Take) ist das Verhalten unverändert.
- Trust-Anchor-Pin `K2_CLOSED_1` auf die neuen Bytes der Testdatei.

Keine Regel, keine Prüffunktion, kein Validator, keine Laufzeit-, Beweis-, Envelope- oder Musikänderung. Der Escape-Schutz selbst bleibt unverändert und ist weiter durch `verdict_tokens_escape_observation_only_through_declared_wiring` belegt.

## Prüfung

Gegenprobe vorab: Der fertige 5b-Baum besteht mit dieser Testkorrektur alle 330 Gate-Tests, ohne sie 327.
