# r574 – SchemaSchlopfDependencyAdmissionBoundary

## Parent und Zweck

Autoritativer Parent ist EtuSchlopf `0.10.1839 r573`
`PhraseNotationPolicyStrictClippyWarningDebtClosure`, vollkorpus-gruen.

r574 integriert SchemaSchlopf **noch nicht** produktiv. Der Take schafft ausschliesslich
die fail-closed Governance-Grenze, die ein nachfolgender eigener Take fuer die exakte
SchemaSchlopf-r020-Anbindung benutzen darf.

Damit legitimiert der spaetere Dependency-/Runtime-Take seine eigene Cargo-Aenderung
nicht selbst.

## Gebundene Komponente

Zugelassen wird ausschliesslich:

- Projekt: `SchemaSchlopf`
- Version: `0.1.0`
- Revision: `r020`
- Name: `InstanceValidationClosure`
- Source-tree seal aus dem bereitgestellten finalen Export:
  `a44e7a8dd25aa914b496a6716294ca41946df18b569bffc925f9f6a63d54594e`
- `instance_validation_qualified=true`

SchemaSchlopf bleibt read-only Validator/Proof-Komponente. Die Zulassung erzeugt keine
Writer-, Repair-, MusicXML-Mutations- oder musikalische Entscheidungsautoritaet.

## Neue ChangeClass

r574 fuehrt die explizite Klasse

```text
ValidationOnly
```

ein. Sie ist fuer die persistierte Schema-Validierungsgrenze bestimmt und darf keine
Kodex-/Security-Dateien veraendern. Die in diesem Take selbst geaenderte Governance
bleibt deshalb weiterhin `KodexOnly`; `ValidationOnly` wird erst von einem spaeteren
Take benutzt.

Der spaetere ValidationOnly-Schnitt ist auf folgende Flaechen beschraenkt:

- `Cargo.toml`
- `ZIP_KIND.txt`
- Dokumentation
- `tools/qualified_components/schemaschlopf_r020/**`
- `src/persisted_schema_validation.rs`
- `src/pipeline.rs`
- `src/render.rs`
- `tests/persisted_schema_validation_gate.rs`

Andere Pfade bleiben fail-closed.

## Exakter Dependency-Vertrag

Ein nicht-version-only Root-Cargo-Wechsel bleibt grundsaetzlich verboten. Die einzige
neu vorbereitete Ausnahme verlangt gleichzeitig:

```text
change_class=ValidationOnly
schemaschlopf_r020_dependency_admission=closed_exact_local_path_0_1_0_r020_v1
```

und exakt die zusaetzliche `[dependencies]`-Zeile:

```toml
schemaschlopf = { path = "tools/qualified_components/schemaschlopf_r020", version = "=0.1.0", default-features = false }
```

Ausser dieser einen Zeile muss `Cargo.toml` bytegleich zum Parent bleiben. Andere
Versionen, Pfade, Features oder weitere Dependency-Aenderungen bleiben fail-closed.

## Qualifizierte Komponentenprojektion

Der spaetere Bindungs-Take darf die r020-Komponente nur erstmalig unter

```text
tools/qualified_components/schemaschlopf_r020/
```

materialisieren. Der Admission-Vertrag erwartet ausschliesslich die fuer die lokale
Rust-Komponente benoetigte, fest benannte Source-Projektion samt `VERSION`,
`Cargo.toml` und `SOURCE_MANIFEST.sha256`.

Die drei normativen MusicXML-3.1-XSD-Dateien duerfen dort **nicht** dupliziert werden.
Sie bleiben ausschliesslich in EtuSchlopfs bereits qualifiziertem Root

```text
docs/normative_sources/musicxml/3.1/schema
```

und werden spaeter als `schema_dir` an SchemaSchlopf uebergeben.

Die zugelassenen Projektionsdateien sind zusaetzlich einzeln auf die aus dem finalen
r020-Export abgeleiteten FNV64-Bytes gepinnt; Zusatzdateien sind nicht zugelassen.
`SOURCE_MANIFEST.sha256` bleibt als r020-Provenienzbeleg Teil der Projektion.

Die Dateien der qualifizierten Komponentenprojektion werden vom EtuSchlopf-
Semantic-Capability-Scanner als opake read-only Komponentenbytes inventarisiert und
nicht faelschlich als neue EtuSchlopf-Musiklogik interpretiert. Das erzeugt kein
Schreibrecht: die bestehende No-New-Repair-/Writer-Authority bleibt unveraendert.

## Noch nicht Teil von r574

r574 aendert insbesondere **nicht**:

- `Cargo.toml`
- `Cargo.lock`
- die produktive EtuSchlopf-Pipeline
- Master-/Render-MusicXML
- PDF/Renderer
- Oktavierungsentscheidungen
- Musik-, Writer-, Repair- oder Recovery-Semantik
- SchemaSchlopf selbst

Insbesondere wird `compile_bound_instance_validator()` in r574 noch nicht aufgerufen.

## Nachfolgender Take

Erst nach real gruener Installation von r574 darf der nachfolgende ValidationOnly-Take
SchemaSchlopf r020 materialisieren, die exakte Dependency aktivieren und die
produktive Reihenfolge schliessen:

```text
Master final persistieren
-> dieselben Bytes mit fs::read lesen
-> SchemaSchlopf validate_bytes
-> Render-MusicXML erzeugen/final persistieren
-> dessen eigene Bytes mit fs::read lesen
-> SchemaSchlopf validate_bytes
-> erst danach Renderer/PDF bzw. XSD-valid-Claim
```

Der Validator wird pro EtuSchlopf-Lauf einmal kompiliert und fuer alle Instanzen
wiederverwendet. Schemafehler bleiben fail-closed und duerfen keine Reparatur- oder
Retry-Schicht starten.
