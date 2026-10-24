# r1157 SemanticOccurrenceCorrespondenceReceiptPersistenceShadow

## Parent und Zweck

r1157 baut direkt auf dem akzeptierten r1156 `SemanticObligationStructuralObservationIdentityShadowCompileClosure` auf. r1156 hat im 10/10-Korpus bewiesen, dass Master- und Render-MusicXML strukturell beobachtbar sind, aber keine schema-definierte persistierte semantische Occurrence- oder Lane-Identitaet tragen. Gleichzeitig existiert produktiv bereits die transaction-lokale `FinalPdfSourceOccurrenceContinuation`, die jede kanonisch gebundene gepitchte Output-Occurrence auf `SourceEventId` und `SoundingAttackId` zurueckfuehrt.

Dieser Take schliesst ausschliesslich die Persistenzluecke fuer die Occurrence-Seite.

## Bereits vorhandene produktive Evidenz

Die `FinalPdfSourceOccurrenceContinuation` wird von der bestehenden StructuralWriteAuthority erzeugt und durch die autorisierten FinalNotation-, HorizontalLayout- und committed Cross-Staff-Projektionen transportiert. Ihre deklassifizierte read-only Projektion enthaelt bereits:

- `SourceEventId`,
- `SoundingAttackId`,
- `measure`,
- `pitched_ordinal`,
- `materialization_kind`,
- `fragment_ordinal`.

Diese Relation wird nicht neu berechnet und nicht aus physischer Geometrie erraten. `measure` und `pitched_ordinal` sind ausschliesslich der bereits gebundene physische Locator der Occurrence; die semantische Identitaet stammt aus SourceEvent/Attack.

## Persistenzschnitt

r1157 verwendet keinen neuen Artefakt-Writer. Die Relation wird im bereits bestehenden `.result.txt`-Sink innerhalb des vorhandenen `NOTATION-VOICE-CAUSAL-LANE-PRESSURE-AUDIT` als vollstaendige Receipt-Projektion persistiert.

Die Receipt enthaelt:

1. einen content-addressed Schema-SHA-256,
2. den SHA-256 der exakt gebundenen Render-MusicXML-Bytes,
3. die vollstaendige deterministisch sortierte Row-Menge,
4. einen SHA-256 ueber Schema-Bindung, Render-Bindung und kanonische Rows.

Dadurch kann ein spaeterer Reader die Persistenz unabhaengig gegen die gespeicherten Render-Bytes pruefen, ohne auf den transaction-lokalen privaten Receipt-State zugreifen zu muessen.

## Authority-Grenze

Der Take ist `AuditOnly`. Er:

- mintet keine Identitaet,
- rekonstruiert keine Identitaet aus Measure/Staff/Voice/Onset/Pitch,
- trifft keine Auswahl,
- erzeugt keine Lane-Korrespondenz,
- aendert keine MusicXML- oder PDF-Bytes,
- erzeugt keinen semantischen Contract `C`,
- erzeugt keine Obligation-Genesis oder Ledger-Semantik,
- besitzt kein Writer-, Repair- oder Release-Recht.

Lane-Identity bleibt nach r1157 weiterhin `UNAVAILABLE`.

## Sequenzierung

Der bestehende r1156-Reader wird in diesem Take absichtlich nicht erweitert, die neu geschriebene Receipt im selben Lauf wieder einzulesen. Schreiben der Persistenz und Konsumieren der Persistenz bleiben zwei getrennte Architektur-Schritte. Ein spaeterer Reader-Take darf die Receipt nur nach SHA-/Schema-Pruefung als persistierte Occurrence-Correspondence-Evidenz konsumieren.
