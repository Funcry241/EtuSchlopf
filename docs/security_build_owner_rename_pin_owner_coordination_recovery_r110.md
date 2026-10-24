# r110 – Security Build Owner Rename Pin Owner Coordination Recovery

## Zweck

r109 wurde vor dem Kopieren vom bestehenden Take-Change-Control abgelehnt, weil die gepinnten Architekturpfade `src/build_gate/pinned_policy_security.rs` und `src/build_gate/semantic_capability_diff.rs` geändert wurden, ohne dass beide vorgeschriebenen Pin-Eigentümer als reale Änderungen im Kandidaten enthalten waren.

r110 führt weiterhin **keinen** Build-Owner-Rename aus. Er übernimmt ausschließlich die bereits vorgesehene enge FNV-Pin-Synchronisierungsregel und koordiniert sie korrekt mit beiden bestehenden Pin-Eigentümern:

- `src/identity_debt_build.rs`
- `src/identity_gate_trust_anchor.rs`

`src/identity_debt_build.rs` erhält nur eine nichtsemantische Vertragsnotiz. Dadurch ist der Build-Owner selbst eine reale, explizite Änderung und erfüllt den bereits vorhandenen Architektur-Pin-Koordinationsvertrag.

## Geschlossene Regel

Der spätere `security_build_owner_rename=closed_exact_cargo_build_owner_rename_v1` bleibt auf positionsgebundene Alt→Neu-Pfadsubstitutionen beschränkt. Nur in den zwei vorhandenen Pin-Ownern `src/build_gate/pinned_policy_security.rs` und `src/identity_gate_trust_anchor.rs` dürfen zusätzlich FNV-Hexliterale an bestehenden Pin-Stellen synchronisiert werden.

Zusätzlich wird der Trust-Anchor-Fall explizit auf die reale `include_bytes!("identity_debt_build.rs")`-Schreibweise ohne `src/`-Präfix geschlossen. Der Rename-Comparator darf dort ausschließlich den korrespondierenden Basename auf `security_build_orchestrator.rs` umstellen; die Cargo-/Source-Pfadform bleibt an allen anderen Referenzquellen unverändert maßgeblich. Ein Regressionstest bindet genau diesen Fall.

Andere Text-, Kontrollfluss-, Error-, Capability-, Writer-, Musik-, Renderer-, Pipeline-, Recovery- oder Kalibrierungsänderungen bleiben verboten.

r110 ändert keinen Cargo-Build-Owner und keine Runtime-/Musiksemantik.
