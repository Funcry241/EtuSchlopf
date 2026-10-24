# r247 – Cargo198TargetCacheTagTrustAnchorTestPinRecovery

## Parent

`0.10.1839 r240` `MidiSourceEvidenceVocabularyClosure` bleibt der autoritative Parent. r241, r242, r245 und r246 wurden transaktional zurueckgerollt; r243 wurde vor Kopieren abgelehnt; r244 wurde nicht autoritativ. Keine dieser Revisionen ist Parent.

## Reale Evidenz aus r246

Der r246-Reexec erreichte die neue Cargo-Target-Cache-Behandlung erfolgreich. Clippy und Build waren GREEN. Erst der integrierte `cargo test --all-targets --no-fail-fast`-Lauf endete mit Exit 101; als einziges fehlgeschlagenes Target wurde `--bin etuschlopf` gemeldet.

Die Ursache liegt in `src/identity_gate_trust_anchor.rs`: Die drei produktiven Compile-Time-Pins fuer `IDENTITY_BUILD_POLICY_FNV1A64`, `RECOVERY_TRANSACTION_ORDER_SECURITY_FNV1A64` und `RUN_CMD_FNV1A64` waren bereits auf die realen r246-Bytes aktualisiert. Der Unit-Test `all_identity_gate_trust_anchors_match()` erwartete jedoch noch drei andere historische Werte. Damit waren Produkt-Trust-Anchor und Regressionstest nicht synchron.

## Recovery

r247 baut die funktionale r246-Aenderung direkt auf r240 neu auf und aendert gegenueber dieser Funktionalitaet nur die drei Test-Sollwerte:

- `IDENTITY_BUILD_POLICY_FNV1A64` -> `0x63745f22deb44e12`
- `RECOVERY_TRANSACTION_ORDER_SECURITY_FNV1A64` -> `0x25ed71176df86659`
- `RUN_CMD_FNV1A64` -> `0xb2daf32349b69bcf`

Die Werte entsprechen exakt den bereits vorhandenen Compile-Time-Pins im selben Trust-Anchor-Modul. Es entsteht keine neue Security-Capability; kein Check wird entfernt oder gelockert.

## Cargo-1.98-Cleanup-Vertrag

Der r246-Vertrag bleibt funktional unveraendert: Fehlt `CACHEDIR.TAG`, darf EtuSchlopf den Standardmarker nur fuer die beiden fest gebundenen Target-Wurzeln `target` und `target\\etuschlopf_bootstrap_cache` erzeugen. Existiert der Marker bereits, bleibt er unangetastet; die nachfolgende Cargo-Operation ist die autoritative fail-closed Validierungsinstanz.

## Grenzen

- kein `rust-toolchain.toml`
- kein Dependency-Update
- kein manuelles `Cargo.lock`
- keine Musik- oder MusicXML-Aenderung
- keine Writer-, Renderer-, Solver-, Layout- oder Kalibrationsaenderung
- keine neue Repair Authority

ChangeClass: `SecurityOnly`.
