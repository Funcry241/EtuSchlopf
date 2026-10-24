# MusicXML 4.0 Conformance Foundation — r249

## Zweck

r249 ist ein ausschliesslich lesender `AuditOnly`-Take. Er etabliert den veroeffentlichten MusicXML-4.0-Beweisvertrag, ohne produktive MusicXML-, Rhythmus-, Musik-, Writer-, Renderer-, Solver- oder Layoutbytes zu mutieren.

Der produktive Exportvertrag bleibt in r249 unveraendert bei `version="3.1"`. Ein MusicXML-4.0-Konformitaetsanspruch ist deshalb in r249 ausdruecklich unzulaessig.

## Veroeffentlichter Quellenstand

- MusicXML 4.0 Final Community Group Report: 2021-06-01
- Release: `v4.0`
- gepinnter Release-Commit: `799e2defb2ece0ae7bafe08dcbcac25b2c631d53`
- Bericht: `https://www.w3.org/2021/06/musicxml40/`
- Release: `https://github.com/w3c/musicxml/releases/tag/v4.0`
- Schema-Uebersicht: `https://www.w3.org/2021/06/musicxml40/listings/overview/`
- lokaler Katalog: `https://www.w3.org/2021/06/musicxml40/listings/catalog.xml/`

MusicXML 4.0 ist ein Final Community Group Report und kein W3C-Standard auf dem W3C Standards Track. Fuer den EtuSchlopf-4.0-Vertrag ist diese eingefrorene veroeffentlichte Fassung die technische Primaerquelle.

## Schema-Provenienzmatrix r249

r249 erfindet keine SHA-256-Werte und kopiert keine unqualifizierten Schemaartefakte in den Produktbaum. Die exakten lokalen Bytes werden erst dann XSD-Beweisautoritaet, wenn sie aus dem gepinnten Release materialisiert und gehasht wurden.

| Datei | Release/Commit | offizielle Quelle | SHA-256 in r249 | Status |
| --- | --- | --- | --- | --- |
| `musicxml.xsd` | `v4.0` / `799e2def...31d53` | `schema/musicxml.xsd` | nicht materialisiert | BLOCKED |
| `xml.xsd` | `v4.0` / `799e2def...31d53` | `schema/xml.xsd` | nicht materialisiert | BLOCKED |
| `xlink.xsd` | `v4.0` / `799e2def...31d53` | `schema/xlink.xsd` | nicht materialisiert | BLOCKED |
| `container.xsd` | `v4.0` / `799e2def...31d53` | `schema/container.xsd` | nicht materialisiert | BLOCKED |
| `catalog.xml` | `v4.0` / `799e2def...31d53` | `schema/catalog.xml` | nicht materialisiert | Mapping-Evidenz |

Der offizielle 4.0-Katalog mappt die bekannten MusicXML-Schema-URIs auf lokale Dateien. Der spaetere produktive Validator muss deshalb offline, deterministisch und fail-closed arbeiten: bekannte URI -> exakt gepinnte lokale Datei; unbekannte URI -> HARD FAIL; Netzwerkzugriff -> niemals.

## r249-Readiness-Stack

r249 implementiert ausschliesslich:

1. XML-Wohlgeformtheit via bereits vorhandener read-only XML-Parsing-Abhaengigkeit.
2. separate Erfassung der deklarierten Root-Version.
3. Root-Klassifikation `score-partwise` / `score-timewise`.
4. veroeffentlichte harte MusicXML-4.0-Semantikinvariante `chord_duration <= predecessor_note_duration`.
5. separate Klassifikation von Dauerverstoessen mit zwischenliegendem `forward` bzw. `backup`.
6. sichtbaren Readiness-Report fuer Master und Render.

Nicht implementiert und deshalb ausdruecklich nicht behauptet:

- keine XSD-1.0-Validitaet,
- kein gepinnter XSD-Resolver,
- kein MusicXML-4.0-Produktgate,
- kein `3.1 -> 4.0`-Cutover,
- keine Chord-Reparatur,
- keine MXL-Reparatur,
- keine Repair Authority.

Der Report muss daher solange wahrheitsgemaess `Xsd40Qualified no`, `WriteAllowed no` und `MusicXml40Conformant no` melden, bis der gepinnte Schema-/Validatorbeweis in einer spaeteren Revision wirklich geschlossen ist.

## Harte Semantikinvariante

RuleId: `MusicXmlChordPredecessorDuration`

- VersionScope: MusicXML 4.0
- EvidenceClass: PublishedSpecification
- Invariant: `chord_duration <= predecessor_note_duration`
- RepairAuthority: `none`

Kuerzere zusaetzliche Akkordtoene bleiben gueltig und duerfen nicht auf Gleichheit normalisiert werden. r249 erkennt nur den belegten Verstoss und klassifiziert Timeline-Zwischenereignisse; er schreibt keine XML-Bytes.

## Erwarteter Ausgangsbefund

Der autoritative r248-Export erzeugt weiterhin MusicXML 3.1. Der r249-Readiness-Report muss deshalb `DeclaredVersion40 no` zeigen. Gegen den unmittelbar vor r249 geprueften realen Etuede-6-r248-Export wurden read-only 24 Dauerverstoesse im Master und 7 im Render reproduziert. Diese Zahl ist Evidenz fuer den anschliessenden Owner-/Semantikabschluss, kein r249-Reparaturziel.

## Abnahme

r249 ist nur dann akzeptiert, wenn:

- Rust-1.98-Baseline unveraendert aktiv bleibt,
- Build GREEN,
- Clippy GREEN,
- alle Tests GREEN,
- realer Lauf GREEN,
- Master-MusicXML byteidentisch zu r248,
- Render-MusicXML byteidentisch zu r248,
- PDF visuell/pixelbezogen unveraendert,
- neuer Reportabschnitt vorhanden,
- keine Musikmutation,
- keine Writer-/Repair-Authority,
- XSD-4.0 weiterhin fail-closed als `not qualified` ausgewiesen wird, solange die exakten Schema-Bytes fehlen.
