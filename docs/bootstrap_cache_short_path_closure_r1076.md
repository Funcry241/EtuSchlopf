# r1076 BootstrapCacheShortPathClosure

## Zweck

r1076 folgt direkt auf den kanonisch angenommenen r1075 `ValidationBreadthProofPlacementClosure`
und verkürzt ausschließlich den dedizierten Bootstrap-/All-Target-Cargo-Targetpfad von
`target/etuschlopf_bootstrap_cache` auf `target/bc`.

Die Änderung spart pro darunterliegendem Windows-Pfad 24 Zeichen. Sie adressiert damit die
bekannten langen Pfade beim Kopieren des Projektordners, ohne Cargo-, Recovery- oder
Validierungsautorität zu verändern.

## Geschlossener Vertrag

- `run.cmd`, integrierte Strict-All-Target-Tests und der Take-Bootstrap verwenden denselben
  neuen Root `target/bc`.
- `CARGO_INCREMENTAL=1` bleibt unverändert.
- Source-Signatur, Cargo-Fingerprint und Binary-FNV bleiben die alleinige Cache-Reuse-Bindung.
- Rollback bereinigt weiterhin nur das Paket `etuschlopf` per `cargo clean --package` im
  dedizierten Bootstrap-Cargo-Target; kein rekursives Recovery- oder Source-Cleanup wird erweitert.
- Der CACHEDIR.TAG-Ownervertrag wird geschlossen von V2 auf V3 und `target/bc` umgebunden.
- Der alte Root `target/etuschlopf_bootstrap_cache` ist keine aktive Buildwurzel mehr und wird
  ausschließlich durch Launcher-Hygiene als abgeleiteter Legacy-Cache entfernt.
- Der aktuelle kurze Cache behält die bisherige Hygiene-Semantik für `debug/incremental`;
  andere Cargo-Artefakte bleiben wie zuvor für Wiederverwendung verfügbar.

## Regression

Die Launcher-Tests beweisen:

1. `run.cmd` bindet die aktive Bootstrapwurzel an `target\bc` und nicht mehr an den langen Altpfad.
2. Build-Cache-Hygiene entfernt den aktuellen kurzen Bootstrap-Incrementalpfad.
3. Ein vorhandener alter langer Bootstrap-Cache wird als Legacy-Derived-State vollständig entfernt.
4. Die übrigen fingerprintgebundenen Cache-Hygienegrenzen bleiben unverändert.

## Grenzen

- ChangeClass: `KodexOnly`
- keine MusicMutation
- keine Änderung an Acceptance, Rejection oder Failure-/Post-Acceptance-Handover
- keine Writer-, Repair-, Selection-, Layout-, Proof- oder Release-Authority
- kein Root-`Cargo.lock` im Take-Paket
- kein universelles MAX_PATH-Gate; der produktive Pfad wird an seiner realen Ursache verkürzt
