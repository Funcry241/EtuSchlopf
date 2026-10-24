# WatchRust2024Resolver3RustVersionAdmissionBoundary

Stand: v0.10.1813 r02

## r02 – Semantic-Capability-Identity-Closure

Der erste r01-Kandidat wurde auf dem realen Windows-Zielsystem **vor jeder
Installation** vom bestehenden Semantic-Capability-Diff korrekt fail-closed
abgewiesen. Ursache war keine Lockerung der neuen Watch-Cargo-Grenze, sondern die
Umbenennung der bereits vorhandenen negativen Security-Prüfung
`watch_project_manifest_and_lock_are_refactor_only_other_source_paths`. Für die
Capability-Inventur bedeutete die Umbenennung: alte `SecurityNegativeCheck`-Identität
entfernt und neue Identität hinzugefügt. Das Entfernen einer negativen Security-Prüfung
ist unter `SecurityOnly` absichtlich verboten.

r02 lockert dieses Gate **nicht**. Die historische Test-Symbolidentität bleibt erhalten;
der Testinhalt beweist nun die strengere Zielinvariante: Watch-`Cargo.toml` und
Watch-`Cargo.lock` sind Security-Pfade, `RefactorOnly` wird abgewiesen und nur die
vorgesehene Security-Grenze bleibt zulässig. Alle übrigen fachlichen Änderungen aus
r01 bleiben gleich.

## Ausgangspunkt

Basis ist der real grüne v0.10.1812-r01-Stand `Rust2024Resolver3RustVersionContract`.
Der Root-Crate verwendet Edition 2024 mit `rust-version = "1.88"` und `resolver = "3"`.
Das separate Cargo-Projekt `tools/etuschlopf_watch` steht dagegen weiterhin auf
`edition = "2021"` und besitzt bislang keinen gleichwertigen MSRV-/Resolver-Vertrag.

## Warum dieser Take vorgeschaltet ist

Der Watch-Crate ist ein eigenständiges Cargo-Projekt mit eigenem `Cargo.toml` und
eigenem `Cargo.lock`. Bis v0.10.1812 wurden beide Pfade im Take-Change-Control als
`OtherSource` behandelt und konnten damit unter `RefactorOnly` verändert werden.
Für eine tatsächlich ausgeführte zweite Cargo-Grenze ist das nach Abschluss des
Root-Cargo-Vertrags zu breit.

Dieser Take migriert die Watch deshalb **noch nicht** auf Edition 2024. Er schließt
zuerst ausschließlich die Zulassungsgrenze. Damit kann der neue Security-Vertrag
nicht im selben Take seine eigene Manifeständerung autorisieren.

## Neue Watch-Cargo-Sicherheitsgrenze

Ab v0.10.1813 gelten

```text
tools/etuschlopf_watch/Cargo.toml
tools/etuschlopf_watch/Cargo.lock
```

im Take-Change-Control als `Security`-Pfade. `RefactorOnly`, `MechanicalOnly`,
`MusicMutation` und andere nicht autorisierte ChangeClasses dürfen diese Cargo-Grenze
nicht mehr verändern.

Zusätzlich kennt der Take-Change-Control den exakt geschlossenen Marker

```text
watch_rust_2024_resolver_contract=closed_watch_2021_to_2024_resolver3_msrv_1_88_v1
```

Der Marker ist ausschließlich unter `SecurityOnly` zulässig und autorisiert für das
Watch-Manifest genau folgenden späteren Übergang:

```toml
[package]
version = "0.10.1737"  # bleibt unverändert
edition = "2021"       # -> "2024"
rust-version = "1.88"  # neu
resolver = "3"         # neu
```

Alle übrigen Paketfelder, Dependencies, Profile und sonstigen Manifestzeilen müssen
strukturell unverändert bleiben. Eine andere MSRV, ein anderer Resolver, eine
Watch-Package-Versionsänderung, eine Dependency-/Feature-Änderung oder irgendeine
sonstige Watch-Cargo-Manipulation bleibt fail-closed.

`tools/etuschlopf_watch/Cargo.lock` erhält in diesem Vertrag absichtlich **kein**
Änderungsrecht. Eine spätere Dependency-/Lock-Änderung benötigt einen eigenen,
expliziten Vertrag.

Der neue Watch-Marker darf nicht mit den historischen Root-Cargo-Migrationsmarkern im
selben Take kombiniert werden.

## Sicherheitskoordination

`src/take_change_control.rs` ist byte-gepinnte Security-Architektur. Deshalb werden
wie bei früheren Security-Gate-Änderungen die zugehörigen Vertrauensanker koordiniert
nachgezogen:

- `src/identity_debt_build.rs`
- `src/identity_gate_trust_anchor.rs`

Der Take ändert keine Watch-Runtime-Datei, keine Watch-Dependency, kein Watch-Lockfile,
keine Musik-, Writer-, Solver-, Repair-, Recovery-, Launcher-, Passorder-, Render-
oder Artifact-Semantik.

## Nachfolgender Schritt

Erst nach real grüner Windows-Abnahme von v0.10.1813 darf ein separater SecurityOnly-
Take den neuen Marker verwenden und das Watch-Manifest exakt auf Edition 2024,
`rust-version = "1.88"` und `resolver = "3"` umstellen.

## Abnahme

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Der normale Strict-Pfad bleibt die Freigabeautorität. Deep-Clippy bleibt optional und
diagnostisch.
