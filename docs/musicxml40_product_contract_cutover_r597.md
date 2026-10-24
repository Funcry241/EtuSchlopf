# r597 MusicXml40ProductContractCutover

## Zweck

Dieser Take materialisiert den in r592 vorab zugelassenen MusicXML-4.0-Produktvertrag.
Er ist `ValidationOnly` und führt **keine neue Musik-, Notations-, Repair- oder
Normalizer-Semantik** ein.

Parent:

```text
EtuSchlopf 0.10.1839 r596
MusicXml40ProductContractCutoverRecoveryAdmission
```

Historie:

```text
r592  MusicXml40ProductContractAdmissionBoundary      GREEN
r593  MusicXml40ProductContractCutover                REJECTED
      Semantic-Capability-Diff: neue env!-Makroquelle im Audit-Test
r594  MusicXml40ProductContractCutoverRecoveryAdmission GREEN
r595  MusicXml40ProductContractCutover                REJECTED (test-only E0515)
r596  MusicXml40ProductContractCutoverTestCompileRecoveryAdmission GREEN
```

r593 besitzt keine Authority. r597 materialisiert den bereits fachlich
zugelassenen Cutover als unabhängigen Nachfolgekandidaten. Die in r593
beanstandete neue `env!`-Makroquelle wird nicht legitimiert und ist in r597
nicht enthalten.

Der durch r592 etablierte und durch r594/r596 auf r597 neu gebundene Admission-Marker ist exakt:

```text
musicxml40_product_contract_cutover=
closed_exact_r020_to_r021_persisted_musicxml40_v1
```

## SchemaSchlopf-Authority

Produktiv eingebunden wird genau der von r592 vorab bytegepinnt zugelassene
Qualifikationsstand:

```text
SchemaSchlopf 0.1.0
revision=r021
name=MusicXml40ProfileClosure
source_manifest_sha256=
0d573e5e03fb2831de14d77f7f16b0e65776e8ae2637010431fdd48888b3ed82
instance_validation_qualified=true
```

Die EtuSchlopf-Projektion übernimmt wie bei r020 nur die 19 zugelassenen
Komponentenfiles und härtet ausschließlich das lokale `Cargo.toml` von
`warnings = "warn"` auf `warnings = "deny"`.

Der später gelieferte SchemaSchlopf-Stand r022
`OpenSourcePublicationReadinessPortableTestRootClosure` ist **nicht** Gegenstand
dieses Produkt-Cutovers. Er basiert auf r021 und ändert gegenüber r021 die
SchemaSchlopf-Publikations-/Repository-Metadaten sowie Test-Root-Auflösung; die
r592-Admission-Authority bindet absichtlich den fachlichen MusicXML-4.0-
Qualifikationsabschluss r021. EtuSchlopf folgt damit nicht automatisch einer
unabhängigen nachgelagerten Publikationsrevision.

SchemaSchlopf bleibt read-only XSD-Validation-Authority und erhält keine
Writer-, Repair- oder MusicMutation-Authority.

## Normative MusicXML-4.0-Bytes

Repository-resident hinzugefügt werden ausschließlich:

```text
docs/normative_sources/musicxml/4.0/schema/musicxml.xsd
Bytes   379505
SHA-256 bfe37ed25a9ec00e6f2591d53df260b84efe12aed209ba3ac0a76f9287665a99

docs/normative_sources/musicxml/4.0/schema/xml.xsd
Bytes   5695
SHA-256 616a3077df5cfc954ac74a75abe9697b95eef7a85dbe09367d995a483e840eb5

docs/normative_sources/musicxml/4.0/schema/xlink.xsd
Bytes   2280
SHA-256 6e601f8eeb41618b50e4c7f944dff754e57ea43b602755470dda24c9c2f6df92
```

Quelle der realen Eingangsbytes:

```text
musicxml-4.0.zip
Bytes   192235
SHA-256 4c815790c6cdcceef6b5c6a3c17b7229f52c2754eb31b61a63d30f60ef41acbc
Release v4.0
Commit  799e2defb2ece0ae7bafe08dcbcac25b2c631d53
```

`container.xsd`, `catalog.xml` und `.mxl` bleiben außerhalb dieses Scopes.

Die bereits gebundenen 3.1-XSDs bleiben byteidentisch als historische
Regressionsevidenz im Repository, besitzen nach diesem Cutover aber keine
produktive Format-Authority mehr.

## Gate A – EtuSchlopf Serialization Authority

Der produktive Writer ändert exakt:

```xml
<score-partwise version="3.1">
```

zu:

```xml
<score-partwise version="4.0">
```

Auf den **tatsächlich persistierten** Master-/Render-Bytes prüft EtuSchlopf
danach unabhängig:

```text
Root == score-partwise
DeclaredVersion == 4.0
```

Diese Policy wird nicht an SchemaSchlopf delegiert.

## Gate B – SchemaSchlopf XSD Authority

Genau dieselben bereits gelesenen persistierten Bytes werden anschließend
durch den einen produktiven SchemaSchlopf-r021-Validator gegen
`MUSICXML40_SCHEMA_DIR` geprüft.

Damit gilt ausführbar:

```text
Gate A != Gate B

XSD-4.0-valid
!=
DeclaredVersion == 4.0
```

Ein Regressionstest beweist ausdrücklich, dass ein ansonsten gültiges
`version="3.1"`-Dokument vom MusicXML-4.0-XSD akzeptiert werden kann, aber am
EtuSchlopf-Produktversionsvertrag scheitert.

Es gibt keine Shadow-Validation und keinen nachträglichen XML-Normalizer.

## Gate C – Serialisierungs-/Semantikerhalt

Der vor dem produktiven Cutover read-only eingefrorene r587/r592-Korpus zeigte für alle 20
Master-/Render-Artefakte:

```text
MusicXML-4.0-XSD gültig                20/20
nur 3.1 -> 4.0 geändert und gültig     20/20
```

Für die tatsächliche Full-Corpus-Qualifikation nach Installation von r597 gilt:

```text
DeclaredVersion40 Master     10/10
DeclaredVersion40 Render     10/10
XSD40 Master                 10/10
XSD40 Render                 10/10
Semantic/notation gates      GREEN
Renderer                     10/10
FinalClean                   10/10
```

Die vorab eingefrorene Byteerwartung ist stärker als eine bloße abstrakte
Semantikprojektion: Wenn Writer und übrige Serialisierung unverändert bleiben,
darf gegenüber der 3.1-Baseline je XML ausschließlich die zweistellige
Versionsdeklaration wechseln.

## Historischer r249-Report

`format40_conformance_foundation.rs` bleibt read-only und erhält keine zweite
XSD-Authority. Er wird auf den realen Zustand geschlossen:

```text
ProductiveVersion 4.0
MigrationTarget none
HistoricalBaselineVersion 3.1
Productive 4.0 schema bytes locally hash-bound
DeclaredVersion40 observed from artifacts
Xsd40Authority delegated to the persisted product-contract gate / SchemaSchlopf r021
ShadowValidation none
```

Der Report beweist XSD-Konformität nicht erneut.

## Nicht-Scope

```text
keine neue Musik
keine neue Notation
keine Phrase-/Gestaltentscheidung
keine Repair-Schicht
keine XML-Normalisierung
keine neue Writer-Architektur
keine parallele 3.1/4.0-Produkthaftung
kein .mxl
kein Netzwerk im Produktlauf
```

Nach erfolgreichem r597-Lauf und anschließendem normalen 10er-Korpus ist
ausdrücklich zu prüfen, ob der MusicXML-4.0-Arbeitsblock damit **fertig** ist.
Ein weiterer Closure-Take ist kein Selbstzweck.
