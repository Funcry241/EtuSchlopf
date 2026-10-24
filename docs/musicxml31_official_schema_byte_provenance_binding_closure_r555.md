# r555 MusicXML 3.1 Official Schema Byte Provenance Binding Closure

## Parent und Klasse

- Parent: `r554`
- Revision: `r555`
- ChangeClass: `AuditOnly`
- Name: `MusicXml31OfficialSchemaByteProvenanceBindingClosure`

## Zweck

r554 hat den lokalen Byte-Ingress-Vertrag fail-closed etabliert, ohne lokale Originalbytes oder
externe Originalitaet zu behaupten. r555 bringt erstmals die drei fuer den produktiven
MusicXML-3.1-XSD-Vertrag benoetigten offiziellen Schema-Dateien repository-resident ein und bindet
ihre exakten Bytefolgen an feste SHA-256-/Laengen-Erwartungen.

Die Bytes wurden unveraendert aus dem bereitgestellten offiziellen `v3.1`-Source-Archiv extrahiert.
Der ZIP-Kommentar bindet das Archiv an den bereits in r542 gepinnten Commit
`8bbe8e50606bf99317a86e9e1637618d6bdd1997`. Die Extraktion schreibt die ZIP-Memberbytes ohne
Textdekodierung, Zeilenumbruchkonvertierung oder sonstige Transformation.

## Upstream-Bindung

- Version: `3.1`
- Final Community Group Report: `2017-12-07`
- Tag: `v3.1`
- Commit: `8bbe8e50606bf99317a86e9e1637618d6bdd1997`
- Eingangsarchiv: `musicxml-3.1.zip`
- Eingangsarchiv SHA-256: `be6f7b7602697042b1bea198ea3adb51e4aebb4f10fa8a7ed275950cd98827e6`

Die Acquisition-Metadaten sind zusaetzlich maschinenlesbar in
`docs/normative_sources/musicxml/3.1/PROVENANCE.txt` abgelegt.

## Repository-residente Originalbytes

Kanonischer Root:

`docs/normative_sources/musicxml/3.1/schema`

| Datei | Bytes | SHA-256 |
|---|---:|---|
| `musicxml.xsd` | 335762 | `49ae9e34e04ba5b72004687994f70d598323eb3353eadb62faedcc9e9dc4bd7f` |
| `xml.xsd` | 5695 | `616a3077df5cfc954ac74a75abe9697b95eef7a85dbe09367d995a483e840eb5` |
| `xlink.xsd` | 2280 | `67c7f7a02491babc51e11c10268150b1a76070da36d5abe88e68515009611966` |

Der Audit qualifiziert `OriginalityQualified yes` und
`OfficialSchemaByteProvenanceBound yes` nur dann, wenn **alle drei** Dateien vorhanden sind und
sowohl Byte-Laenge als auch SHA-256 exakt den gepinnten Erwartungen entsprechen. Beliebige lokale
Dreiersaetze bleiben trotz `LocalBytesComplete yes` unqualifiziert.

## Absichtliche Grenzen

r555 qualifiziert **nur die offiziellen Schema-Bytes und deren Provenienz**.

Weiterhin unveraendert:

- `NetworkAllowed no`
- `FallbackReconstructionAllowed no`
- `Xsd31Qualified no`
- `MusicXml31XsdConformanceProven no`
- `WriteAllowed no`
- kein XML-Catalog / Resolver
- kein XSD-Validator
- keine 20/20-XSD-Korpusvalidierung
- keine MusicXML-Writer-Aenderung
- keine MusicXML-Versionaenderung
- keine Supports
- keine Musikmutation
- keine Reparaturautoritaet
- keine Aenderung an E3 M15, E4 M30 oder E7 M6
- keine Dependency-/Cargo.lock-Aenderung

## Tests

r555 erweitert die r554-Regressionen so, dass:

1. beliebige vollstaendige lokale Testbytes **nicht** als offiziell qualifiziert werden,
2. die drei repository-residenten Originaldateien exakt die gepinnten Laengen und SHA-256-Werte
   besitzen,
3. erst der vollstaendige Digest-Match `OriginalityQualified yes` und
   `OfficialSchemaByteProvenanceBound yes` erzeugt,
4. Tag und Voll-Commit unveraendert gebunden bleiben,
5. `NetworkAllowed` weiterhin `false` bleibt.

## Produktive Wirkung

Keine. Die drei XSD-Dateien sind normative read-only Referenzbytes. r555 fuehrt keinen Validator
aus und aendert weder Master-/Render-MusicXML noch PDF, Writer, Renderer oder musikalische Semantik.

## Naechster getrennter Schnitt

Erst nach realem r555-Green darf ein eigener spaeterer Take den **qualifizierten Offline-XSD-1.0-
Validator samt Resolver/Catalog-Architektur** einfuehren. Der eigentliche 20/20-XSD-Korpusbeweis
bleibt wiederum ein separater Schritt.
