# r592 MusicXml40ProductContractAdmissionBoundary

## Zweck

Dieser Take ist **KodexOnly**. Er verändert weder Cargo.toml noch produktive Runtime,
MusicXML-Writer, SchemaSchlopf-Komponentenbytes, normative XSD-Bytes oder musikalische
Ausgabe.

Er schafft ausschließlich die vorab definierte fail-closed Admission-Grenze für den
nachfolgenden produktiven MusicXML-4.0-Cutover.

Parent/GreenBase bleibt:

```text
EtuSchlopf 0.10.1839 r587
RegisterNotationVisibleToneAnchorFeasibleBoundaryStateSpaceClosure
```

Die verworfenen Kandidaten r588-r590 sind keine Parent-Authority.

r591 `MusicXml40ProductContractAdmissionBoundary` wurde ebenfalls **vor jeder Installation**
vom bereits autoritativen Take-Tree-Integrity-Gate abgewiesen, weil das ausgelieferte
Paket den lokalen Root-Pfad `Cargo.lock` enthielt. `Cargo.lock` ist in EtuSchlopf
`LocalTransactional` und darf niemals Bestandteil eines eingehenden Take-Payloads sein.
r591 ist daher keine Parent-Authority und wird nicht wiederverwendet. r592 uebernimmt
den fachlichen Admission-Inhalt auf demselben r587-GreenBase; die Recovery korrigiert
zusaetzlich Revisions-/Nachfolgerbindung und Paketstruktur.

## Extern qualifizierte Zielkomponente

Der spätere Cutover darf ausschließlich die qualifizierte SchemaSchlopf-Komponente
mit folgendem Vertrag aufnehmen:

```text
SchemaSchlopf 0.1.0
revision=r021
name=MusicXml40ProfileClosure
source_manifest_sha256=0d573e5e03fb2831de14d77f7f16b0e65776e8ae2637010431fdd48888b3ed82
instance_validation_qualified=true
```

EtuSchlopf übernimmt die qualifizierte Source-Projektion weiterhin mit der bereits
für r020 etablierten lokalen Integrationshärtung:

```text
warnings = "deny"
```

SchemaSchlopf bleibt read-only Validation Authority und erhält keine Writer-, Repair-
oder MusicMutation-Authority.

## Exakt gebundene MusicXML-4.0-XSD-Bytes

Für den späteren produktiven `.musicxml`-Vertrag sind ausschließlich diese drei
Schemaartefakte zugelassen:

```text
musicxml.xsd
SHA-256 bfe37ed25a9ec00e6f2591d53df260b84efe12aed209ba3ac0a76f9287665a99
FNV1A64 afc1f16c22700368

xml.xsd
SHA-256 616a3077df5cfc954ac74a75abe9697b95eef7a85dbe09367d995a483e840eb5
FNV1A64 43fabc53127b25d9

xlink.xsd
SHA-256 6e601f8eeb41618b50e4c7f944dff754e57ea43b602755470dda24c9c2f6df92
FNV1A64 7e337c4b1b608650
```

`container.xsd`, `catalog.xml` und `.mxl` sind nicht Teil dieses Produktauftrags.
Die bestehenden gebundenen MusicXML-3.1-XSD-Bytes dürfen beim Cutover nicht
verändert werden.

## Einmalige zukünftige Transition

r592 führt den exakten Manifestmarker ein:

```text
musicxml40_product_contract_cutover=closed_exact_r020_to_r021_persisted_musicxml40_v1
```

Er ist nur für `ValidationOnly` zulässig. Der Marker legitimiert nicht allgemein
Schema-/Dependency-/Writeränderungen, sondern genau einen atomaren Übergang:

```text
SchemaSchlopf r020 vollständig entfernt
SchemaSchlopf r021 vollständig und bytegepinnt hinzugefügt
Cargo local path r020 -> r021, sonst unverändert
MusicXML-4.0 musicxml.xsd/xml.xsd/xlink.xsd exakt hinzugefügt
persisted-byte ProductVersion Gate auf 4.0
persisted-byte SchemaSchlopf Gate auf MUSICXML40_SCHEMA_DIR
midi_seed.rs exakt nur score-partwise version 3.1 -> 4.0
historischer format40-Report auf den realen Produktvertrag geschlossen
Regressionstest für Gate A != Gate B
```

Der vorgesehene unmittelbare Nachfolger ist exakt:

```text
revision=r593
name=MusicXml40ProductContractCutover
parent_revision=r592
change_class=ValidationOnly
```

Wenn dieser konkrete Cutover nicht zustande kommt, darf der Marker nicht als
allgemeine spätere Ausnahme umgedeutet werden.

## Gate-Trennung

Der spätere Cutover muss auf denselben tatsächlich persistierten Master-/Render-Bytes
zwei getrennte Wahrheiten beweisen:

```text
Gate A (EtuSchlopf Serialization Authority)
DeclaredVersion == 4.0

Gate B (SchemaSchlopf Validation Authority)
XSD 4.0 valid
```

Das ist notwendig, weil XSD-4.0-Validität allein die deklarierte Produktversion nicht
beweist. SchemaSchlopf bekommt deshalb keinen versteckten EtuSchlopf-Policy-Check.

## Qualified-component immutability

r574/r575 erlaubten die erstmalige r020-Aufnahme. Im späteren steady state dürfen
`ValidationOnly`-Takes qualifizierte Komponentenbytes nicht mehr aus Gewohnheit
verändern.

Ab r592 gilt daher:

```text
normales ValidationOnly
  -> r020/r021 qualified component bytes unveränderlich

exakter MusicXML40 product-contract marker
  -> vollständiger r020-delete + vollständiger r021-add erlaubt
```

SemanticCapabilityDiff behandelt während genau dieser Transition beide exakt benannten
Komponentenpfade als opaque qualified-component bytes. Es gibt bewusst keinen
Wildcard-Prefix `schemaschlopf_*`.

## Nicht-Scope dieses Takes

```text
kein Cargo.toml-Diff
kein SchemaSchlopf-Install
keine XSD-Aufnahme
kein version="4.0" Writer-Diff
keine produktive Gate-A/Gate-B-Umschaltung
keine Musikmutation
keine neue Notation
keine Repair-/Normalizer-Schicht
keine Shadow-Validation
kein MXL
```

Damit legitimiert r592 die spätere Änderung **vor** deren Materialisierung, statt dass
der Runtime-Take seine eigene Ausnahme erzeugt.
