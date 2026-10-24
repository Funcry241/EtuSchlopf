# MusicXML 3.1 Baseline Conformance Provenance Audit Foundation — r542

## Zweck

r542 ist der erste Take des neuen MusicXML-Modernisierungsblocks. Er migriert EtuSchlopf **nicht**
auf MusicXML 4.0 und veraendert keine musikalische, zeitliche, notationstechnische, Layout-,
Renderer- oder Writer-Semantik.

Der Take macht ausschliesslich die bereits reale produktive Baseline MusicXML 3.1 im vorhandenen
read-only Conformance-Audit explizit und korrigiert eine zu enge Norm-Provenienz-Aussage.

## Parent

- Version: `0.10.1839`
- Revision: `r541`
- Parent: `FinalCompleteBeatPrimaryBeamStructuralWriteAuthorityRelocationStrictClippyClosure`
- ChangeClass: `AuditOnly`

r541 bleibt musikalischer und architektonischer GreenBase.

## Warum dieser Take zuerst kommt

Die aktuelle produktive Pipeline schreibt weiterhin:

`<score-partwise version="3.1">`

Gleichzeitig ist der bestehende Conformance-Audit historisch als MusicXML-4.0-Readiness aufgebaut.
Dadurch war die produktive 3.1-Baseline im Report nicht als eigene Norm- und Provenienzebene sichtbar.

Die vorbereitende MusicXML-Analyse hat ausserdem zwei offene Beweisgrenzen bestaetigt:

1. Der Audit rekonstruiert Master/Render weiterhin aus `score_title` statt einen autoritativen
   `SourcePlan`-Locator zu erhalten.
2. Exakte lokale MusicXML-3.1-Schema-Bytes und ein repository-/project-bound XSD-1.0-Validator
   sind noch nicht integriert.

r542 **loest diese beiden Grenzen noch nicht**. Er macht sie nur explizit fail-closed sichtbar.
Dadurch kann der nachfolgende Locator-/Schema-Take ohne begriffliche Vermischung geplant werden.

## Gepinnte produktive Baseline

- MusicXML-Version: `3.1`
- Final Community Group Report: `2017-12-07`
- Release-Tag: `v3.1`
- Release-Commit: `8bbe8e50606bf99317a86e9e1637618d6bdd1997`
- Schema-Quelle: `https://github.com/w3c-cg/musicxml/tree/v3.1/schema`
- benoetigte Baseline-Schema-Dateien:
  - `musicxml.xsd`
  - `xml.xsd`
  - `xlink.xsd`

Die Baseline wird nur als PublishedSpecification-Provenienz gebunden.
Es wird **kein** lokaler Schema-Digest und **keine** XSD-Konformitaet behauptet.

## Neue Report-Trennung

Der vorhandene Audit weist nun getrennt aus:

- `ProductiveBaselineVersion: 3.1`
- `MigrationTarget: none`
- `HorizonVersion: 4.0`

Damit wird MusicXML 4.0 nicht mehr implizit als bereits beschlossene Zielmigration dargestellt.

Zusaetzlich:

- `Xsd31Qualified no`
- `MusicXml31XsdConformanceProven no`
- `AuthoritativeSourcePlanLocatorBound no`
- `LocatorQualified no`
- `RevisionBound no`
- `SourceFingerprintBound no`
- `GeneratorIdentityQualified no`

Diese Werte sind bewusst fail-closed.

## Artifact Locator

Der Audit schreibt den aktuell tatsaechlich verwendeten rekonstruierten Pfad mit in die
Artifact-Zeile.

Das ist **keine** Locator-Closure.

Der Report sagt explizit:

`LocatorSource reconstructed_from_score_title`

und:

`AuthoritativeSourcePlanLocatorBound no`

Ein spaeterer Refactor-/Provenienz-Take muss den autoritativen Locator durch die Pipeline reichen.
r542 erfindet dafuer keine Parallel-Authority.

## Normative Rule Provenance

Die bestehende Regel:

`MusicXmlChordPredecessorDuration`

mit der Invariante:

`chord_duration <= predecessor_note_duration`

ist bereits im MusicXML-3.1-Schema dokumentiert.

Die bisherige Aussage:

`VersionScope: MusicXML 4.0`

war deshalb zu eng.

r542 verengt die Behauptung auf das tatsaechlich Belegte:

- `VersionScope: MusicXML 3.1-or-earlier`
- `MusicXml40Exclusive: no`
- `HistoricalFirstVersion: unqualified`

Damit wird nicht behauptet, in welcher aelteren Version die Regel historisch erstmals erschien.

## Tests

Die vorhandenen MusicXML-4.0-Horizon-/Chord-Duration-Fixtures bleiben **unveraendert erhalten**.
r542 ersetzt keine 4.0-Evidenz durch 3.1-Evidenz. Stattdessen kommen getrennte, additive
3.1-Baseline-Fixtures hinzu.

Neu beziehungsweise erweitert wird geprueft:

- produktive Baseline-Konstanten sind 3.1 / v3.1 / gepinnter Commit;
- die bestehenden 4.0-Horizon-Fixtures bleiben als 4.0-Fixtures bestehen;
- dieselbe publizierte Chord-Duration-Invariante wird zusaetzlich auf 3.1-Baseline-Fixtures geprueft;
- XSD-3.1-Readiness bleibt fail-closed;
- ein 3.1-Artefakt wird als produktive Baseline erkannt;
- der zusammengesetzte Report nennt Baseline 3.1 und MigrationTarget `none`;
- Locator- und Generator-Identity-Grenzen bleiben explizit unqualifiziert;
- die Chord-Duration-Regel wird nicht mehr als 4.0-exklusiv bezeichnet.

## Harte Nicht-Ziele

r542 fuegt **nicht** hinzu:

- MusicXML-4.0-Versionstring
- lokale XSD-Dateien
- neue Cargo-Abhaengigkeit
- XSD-Validator
- Supports-Deklarationen
- Writer-/StructuralWriteAuthority-Aufruf
- MusicXML-Mutation
- Master-/Render-Transformation
- Reparatur-/Recovery-Schicht
- Etueden-/Takt-Sonderlogik
- Audit→Writer-Abhaengigkeit

## Erwartete Ausgabewirkung

Nur der Text des read-only Ergebnisreports darf sich erweitern bzw. korrigieren.

Unveraendert erwartet:

- Master-MusicXML Bytes
- Render-MusicXML Bytes
- PDF Bytes
- musikalische Invarianten
- CompleteBeat-Idempotenz
- Renderergebnis

## Naechste getrennte Schritte

Nach einem gruenen r542:

1. Artifact-Locator-/Snapshot-Provenienz sauber an `SourcePlan` binden.
2. Exakte v3.1-Schema-Bytes repository-resident und gehasht einbringen.
3. Einen project-bound, offline arbeitenden XSD-1.0-Validator qualifizieren.
4. Erst dann Gate B ueber die 20 finalen MusicXML-Artefakte ausfuehren.
5. Supports/Capability bleibt ein eigener spaeterer semantischer Profil-Cutover.

r542 ist absichtlich klein: **erst die reale Baseline korrekt benennen und Beweisluecken sichtbar
machen, dann Infrastruktur schliessen.**
