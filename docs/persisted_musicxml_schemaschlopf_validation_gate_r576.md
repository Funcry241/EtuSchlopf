# r576 – PersistedMusicXmlSchemaSchlopfValidationGateClosure

## Parent und Scope

Autoritativer Parent ist EtuSchlopf `0.10.1839 r575`
`SchemaSchlopfIntegrationManifestWarningFreeClosure`, real gruen installiert.

r576 benutzt erstmals die in r574/r575 getrennt vorbereitete `ValidationOnly`-Zulassung
und bindet die fertige Komponente SchemaSchlopf `0.1.0 r020` produktiv als
read-only XSD-Proof-Komponente ein.

SchemaSchlopf selbst wird nicht weiterentwickelt.

## Authority-Grenze

SchemaSchlopf besitzt in EtuSchlopf ausschliesslich read-only Validierungs- und
Proof-Autoritaet.

Nicht zugelassen sind Writer-, Repair-, MusicXML-Mutations-, musikalische
Entscheidungs-, Retry- oder Netzwerk-/Schema-Download-Autoritaet.
Ein Schemafehler ist fail-closed.

## Persistierte Bytes als Proof-Subject

Die produktive Authority-Grenze ist ausschliesslich:

```rust
let bytes = std::fs::read(path)?;
let report = validator.validate_bytes(&bytes)?;
```

Master und Render werden getrennt aus ihren final persistierten Dateien gelesen und
gegen ihre jeweils eigenen Bytes validiert. Ein erfolgreicher Projektlauf erwartet
beide Standardartefakte:

```text
*.musicxml
*.render.musicxml
```

Fehlt eines, ist das Persisted-Byte-XSD-Gate rot.

## Canonical-SourcePlan-Bindung ohne neue Identity-Ingress-Grenze

Der erste verworfene r576-Kandidat las in `pipeline::run()` die CLI mit
`Cli::normalized_from_env()` ein zweites Mal. EtuSchlopfs Productive-Music-Identity-
Firewall blockierte das korrekt.

Der nachfolgende verworfene c2-Kandidat beseitigte diese Identity-Kante, fuehrte aber
innerhalb der `ValidationOnly`-Klasse neue `env!`- und Test-`write`-Capabilities ein.
Auch das wurde korrekt fail-closed blockiert.

Der aktuelle Kandidat schwaecht weder Firewall noch Semantic-Capability-Policy:

1. `src/pipeline.rs` ruft weiterhin genau den vorhandenen kanonischen
   `crate::source::resolve_source_plan(...)` auf.
2. Ein schmaler lokaler Facade registriert nur dessen bereits strukturierte finale
   Pfade `musicxml_out`, `render_musicxml_out` und `pdf_out`.
3. CLI/Environment werden nicht erneut eingelesen.
4. `src/persisted_schema_validation.rs` enthaelt kein `env!` und keinen Schreibpfad.
5. Der Integrationstest mit temporaeren Dateischreibvorgaengen ist entfallen; die
   produktive Persistenzgrenze selbst bleibt durch `fs::read()` erzwungen.
6. `src/render.rs` behaelt exakt seine bereits vorhandenen drei `env!`-Vorkommen;
   kein neuer Macro/Generated-Source-Eintrag wird erzeugt.

## Ein Validator pro Prozesslauf

`src/persisted_schema_validation.rs` haelt den aus EtuSchlopfs drei gebundenen XSDs
kompilierten `BoundInstanceValidator` in einem `OnceLock`.

Die Schema-Qualification wird pro EtuSchlopf-Prozess hoechstens einmal aufgebaut.
Master und Render verwenden dieselbe Validatorinstanz.

Der Schema-Pfad stammt direkt aus dem oeffentlichen r020-Vertrag
`schemaschlopf::MUSICXML31_SCHEMA_DIR` und lautet:

```text
docs/normative_sources/musicxml/3.1/schema
```

`run.cmd` setzt sein Arbeitsverzeichnis vor jedem Build/Lauf auf die Repository-Wurzel;
damit verweist der relative, von r020 selbst definierte Pfad auf EtuSchlopfs bereits
gebundene normative XSD-Bytes. Die Komponentenprojektion enthaelt keine XSD-Kopie.

## Reale Pipelineposition

Die vorhandene EtuSchlopf-Pipeline materialisiert und friert vor dem finalen Renderer
die finale Master-/Render-Notation bereits ein.

r576 setzt das Gate an die bestehende finale Rendererfunktion, ohne eine neue
Writerfunktion oder Writer-Autoritaet zu erzeugen:

```text
finale Master-/Render-Persistierung
→ bestehender FinalNotation/Snapshot-Close
→ fs::read(Master)
→ SchemaSchlopf validate_bytes(Master-Bytes)
→ fs::read(Render)
→ SchemaSchlopf validate_bytes(Render-Bytes)
→ erst dann echter MuseScore-Renderer
```

Interne Renderer-Feasibility-Shadow-Dateien treffen das Gate nicht. Die finale Grenze
wird nur erkannt, wenn der registrierte PDF-Pfad und eine registrierte finale
MusicXML-Quelle uebereinstimmen.

Bei `--no-pdf` wird dasselbe registrierte Gate vor erfolgreichem Prozess-Exit
ausgewertet.

## Fehlerweg

Schlaegt die Validierung fehl:

1. EtuSchlopf meldet den SchemaSchlopf-Fehler mit Artefaktpfad und Detail.
2. MuseScore wird fuer den finalen Aufruf nicht gestartet.
3. Das gespeicherte Gate-Ergebnis bleibt rot.
4. EtuSchlopf darf nicht erfolgreich enden.

Es gibt keinen XML-Rewrite, keine Reparaturschicht und keinen zweiten Versuch mit
veraenderten XML-Bytes.

## Dependency- und Lock-Grenze

Root `Cargo.toml` erhaelt exakt die in r575 zugelassene Zeile:

```toml
schemaschlopf = { path = "tools/qualified_components/schemaschlopf_r020", version = "=0.1.0", default-features = false }
```

Root `Cargo.lock` ist weiterhin nicht TakeManaged und liegt nicht im Take-Paket.
Der bestehende transaktionale Lock-Vertrag bleibt unveraendert.

## Qualifizierte Komponentenprojektion

Unter

```text
tools/qualified_components/schemaschlopf_r020/
```

liegen die von r575 exakt hashgebunden zugelassenen r020-Rust-/Provenienzbytes und
das EtuSchlopf-seitig gehaertete lokale Integrationsmanifest mit
`warnings = "deny"`.

Die Projektion ist keine Fork-, Repair- oder zweite Validator-Schicht.

## Tests

Die produktive `validate_persisted(...)`-Grenze kann nur ueber `fs::read(path)` zu
`validate_bytes(&bytes)` gelangen. Modul-Tests pruefen zusaetzlich direkt am einmal
kompilierten r020-Validator einen gueltigen und einen schemaungueltigen Bytefall, ohne
neue Dateischreib-Capability in die `ValidationOnly`-Aenderung einzufuehren.

## Nicht Teil von r576

r576 aendert nicht:

- Musikentscheidungen,
- Oktavierungsregeln,
- Writer-/Repair-Autoritaet,
- normative XSD-Bytes,
- SchemaSchlopf-r020-Validatorcode,
- Renderer-Shadow-Authority.

Das Oktavierungs-Spangrenzen-Thema bleibt ein separater nachfolgender Musik-Take.
