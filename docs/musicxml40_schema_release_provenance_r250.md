# MusicXML 4.0 Published Schema Release Provenance — r250

## Zweck

r250 verengt die in r249 etablierte read-only Conformance-Foundation auf das konkrete, veroeffentlichte MusicXML-4.0-Release-Artefakt. Es wird keine produktive MusicXML-, Rhythmus-, Musik-, Writer-, Renderer-, Solver-, Layout- oder MXL-Semantik geaendert.

## Gepinnte Veroeffentlichung

- Final Community Group Report: 2021-06-01
- Release: `v4.0`
- Release-Commit: `799e2defb2ece0ae7bafe08dcbcac25b2c631d53`
- offizielles Release-Asset: `musicxml-4.0.zip`
- Release-Asset-URL: `https://github.com/w3c-cg/musicxml/releases/download/v4.0/musicxml-4.0.zip`
- veroeffentlichte Asset-Groesse: `192235` Bytes

Die GitHub-v4.0-Release-Seite weist dieses Asset als eigenes Release-Artefakt aus. Die W3C-MusicXML-4.0-Dateiuebersicht weist die fuer EtuSchlopf benoetigten Dateien `musicxml.xsd`, `xml.xsd`, `xlink.xsd`, `container.xsd` und `catalog.xml` als Bestandteile der 4.0-Schema-Distribution aus.

## Harte Grenze

Die Asset-Identitaet ersetzt keinen Bytebeweis. r250 behauptet deshalb weiterhin **nicht**, dass ein lokaler SHA-256 des Release-Assets oder der enthaltenen Schema-Dateien qualifiziert sei. Ebenso wird weiterhin keine XSD-1.0-Validitaet behauptet.

Der Readiness-Status lautet daher fail-closed:

```text
PublishedSchemaReleaseAsset: pinned
LocalDigestQualified: no
Xsd40Qualified: no
WriteAllowed: no
MusicXml40Conformant: no
NetworkAllowed: no
```

Ein spaeterer XSD-Beweis darf erst dann autoritativ werden, wenn exakt dieses gepinnte Release-Artefakt beziehungsweise byteidentische daraus stammende Schema-Dateien lokal vorliegen, ihre Digests feststehen und ein qualifizierter XSD-1.0-Validator ausschliesslich gegen die lokalen Bytes arbeitet.

## Kein manueller Nutzer-Schritt

Dieser Vertrag verlangt keinen manuellen Download und keinen PowerShell-Schritt vom Nutzer. Die Beschaffung bzw. Einbettung der Originalbytes bleibt Aufgabe eines nachfolgenden EtuSchlopf-Takes; ein normaler Validierungslauf darf dabei keinen Netzwerkzugriff benoetigen.

## Abnahme

- ChangeClass `AuditOnly`
- Rust 1.98.0 unveraendert
- Build/Clippy/Tests GREEN
- produktive Master-/Render-MusicXML unveraendert
- PDF unveraendert
- kein Cargo.lock-/Dependency-Update
- keine Writer-/Repair-Authority
