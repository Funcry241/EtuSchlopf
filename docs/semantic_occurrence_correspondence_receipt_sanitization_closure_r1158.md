# r1158 SemanticOccurrenceCorrespondenceReceiptSanitizationClosure

## Parent und Befund

r1158 baut direkt auf dem akzeptierten und dunkelgruenen r1157 `SemanticOccurrenceCorrespondenceReceiptPersistenceShadow` auf.

r1157 persistiert erstmals die bereits produktiv bewiesene transaction-lokale `FinalPdfSourceOccurrenceContinuation` im bestehenden `.result.txt`-Sink. Der 10/10-Korpuslauf bestaetigt vollstaendige Row-Projektionen fuer alle gepitchten Render-Occurrences und eine exakte Bindung an den jeweiligen Render-MusicXML-SHA-256.

Die Persistenz hat jedoch eine rein projektive Luecke: r1157 berechnet seinen Receipt-SHA ueber Rows mit dem internen physischen Locator `(measure, pitched_ordinal)`. Danach maskiert der bestehende globale Report-Sanitizer numerische Measure-Locatoren in der final persistierten `.result.txt` als `<locator>`. Ein spaeterer unabhaengiger Reader kann deshalb den r1157-Receipt-SHA nicht allein aus den persistierten Rows reproduzieren.

Das ist kein Verlust semantischer Identity und keine MusicMutation. Es ist eine Self-Verification-Luecke der Audit-Persistenz.

## Closure

r1158 ersetzt ausschliesslich die persistierte physische Locator-Darstellung.

Die semantische Identity bleibt unveraendert:

`SourceEventId + SoundingAttackId`

Der interne bereits bewiesene physische Locator `(measure, pitched_ordinal)` wird nicht als Identity interpretiert. Fuer die persistierte Receipt wird daraus ein opaker, rendergebundener Locator:

`physical_locator_sha256 = SHA256(domain, render_xml_sha256, measure, pitched_ordinal)`

Domain:

`semantic_occurrence_render_physical_locator_v1`

Der numerische Measure-/Ordinal-Locator bleibt damit innerhalb der bestehenden Audit-Projektion. Persistiert werden nur:

- `SourceEventId`,
- `SoundingAttackId`,
- `physical_locator_sha256`,
- `materialization_kind_rank`,
- `fragment_ordinal`.

Der Receipt-SHA wird ueber genau diese final persistierten kanonischen Felder sowie Schema- und Render-Bindung gebildet.

## Unabhaengige Reproduzierbarkeit

Ein spaeterer Reader kann fuer jede gepitchte Note der exakt gebundenen Render-MusicXML aus deren physischen `(measure, pitched_ordinal)` denselben opaken Locator berechnen und damit die Receipt-Row zuordnen. Die semantische Identity wird weiterhin ausschliesslich aus `SourceEventId/SoundingAttackId` uebernommen und nie aus dem Locator rekonstruiert.

Die v2-Row-Menge wird kanonisch nach `physical_locator_sha256`, Source-/Attack-ID und Materialisierungsmetadaten sortiert. Dadurch kann der Reader den Receipt-SHA ohne transaction-lokalen State reproduzieren.

Receipt-Schema-v2-SHA-256:

`bb6257a42d136a489d974954065bbb8c51804605a4fa349acdf374e40b2d7e90`

## Regression

Der synthetische Receipt-Test prueft zusaetzlich, dass die vollstaendige v2-Receipt den bereits kanonischen `sanitize_measure_locator_authority_report` bytegleich ueberlebt. Die Testdaten tragen keine konkrete Korpus-, Etueden- oder Takt-Witness-Identitaet.

## Authority-Grenze

Der Take bleibt `AuditOnly`.

Er:

- mintet keine neue Identity,
- leitet Identity nicht aus Measure/Staff/Voice/Onset/Pitch oder Dokumentposition ab,
- aendert keine MusicXML-/Render-MusicXML-/PDF-Bytes,
- erzeugt keinen neuen Writer oder Sidecar-Sink,
- erzeugt keinen Selector,
- erzeugt keinen semantischen Contract `C`,
- erzeugt keine Obligation-Genesis oder Ledger-Semantik,
- besitzt kein Repair- oder Release-Recht.

Lane-Identity bleibt weiterhin `UNAVAILABLE`. Der Reader-Konsum der Receipt ist nicht Bestandteil dieses Takes.
