# MusicXML Generator / Snapshot Identity Provenance Strict-Test Contract Closure – r550

Parent: v0.10.1839 r547  
ChangeClass: AuditOnly

## Anlass

r549 wurde nach erfolgreichem PRE-TAKE-DIFF-GATE, Clippy und Build im Rust-Testgate rejected und vollstaendig auf r547 zurueckgerollt. Die Ursache ist ein zu enger Legacy-Testvertrag im neuen Kandidaten: Der unqualified GeneratorIdentity-Fallback wurde absichtlich um `Reason ...` erweitert, waehrend der bestehende Test weiterhin die alte Satzzeichenform `GeneratorIdentityQualified no.` erwartete.

r550 basiert daher direkt auf r547 und fuehrt den Generator-/Snapshot-Identity-Schnitt erneut ein. r549 ist kein Parent.

## Generator Identity

Die finale Report-/Audit-Grenze bindet read-only:

- Cargo-Paketversion,
- Take-Revision und Take-Name aus `ZIP_KIND.txt`,
- einen kanonischen SHA-256-Fingerprint des Generator-Source-Snapshots unter dem compile-time `CARGO_MANIFEST_DIR`,
- SHA-256 des tatsaechlich laufenden Executables.

Der Source-Fingerprint `generator_source_snapshot_v1` umfasst die explizit gebundenen Root-Dateien (`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `ZIP_KIND.txt`, optional `rustfmt.toml` / `clippy.toml`) und alle regulaeren Dateien unter `src/`. Pfade, Laengen und Bytes werden sortiert und kanonisch gebunden; Symlinks und Root-Escapes failen geschlossen.

Die `GeneratorIdentitySha256` bindet Version, Revision, Take-Name, Fingerprint-Scope, Source-Fingerprint und Executable-Fingerprint.

## Snapshot Identity

Master und Render werden getrennt an dieselbe Generatoridentitaet gebunden:

```text
ArtifactRole
+ PipelineClosureState
+ SnapshotSha256
+ GeneratorIdentitySha256
```

Die bereits durch r547 qualifizierte Persistenzstufe bleibt unveraendert:

```text
final_report_generation_after_final_notation_layout_snapshot_close_and_renderer
```

## Strict-Test Contract Closure

Der Legacy-/Testpfad ohne gebundene Generatoridentitaet bleibt weiterhin fachlich unqualified. Der Regressionstest prueft jetzt die stabile Semantik:

```text
GeneratorIdentityQualified no;
```

und nicht mehr die alte volatile Punktuation:

```text
GeneratorIdentityQualified no.
```

Damit darf eine additive `Reason`-Projektion den Test nicht faelschlich rot machen.

## Readiness

Bei erfolgreicher Laufzeitbindung darf gelten:

```text
RevisionBound                  yes
SourceFingerprintBound         yes
ExecutableFingerprintBound     yes
GeneratorIdentityQualified     yes
SnapshotIdentityQualified      yes
```

Bewusst offen bleiben:

```text
Xsd31Qualified                 no
MusicXml31XsdConformanceProven no
WriteAllowed                   no
```

## Harte Nicht-Ziele

Keine Aenderung an:

- MusicXML-Version,
- produktivem Writer,
- Musikmutation,
- Notations-/Layoutentscheidung,
- XSD oder Validator,
- Supports,
- Dependencies / Cargo.lock,
- produktiven XML-Metadaten,
- Repair-/Shadow-Authority,
- Etueden-/Takt-Sonderlogik.
