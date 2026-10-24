# r1071 PostAcceptanceHandoffMarkerLineBoundaryClosure

## Zweck

Schliesst den r1070-Befund, bei dem die Post-Acceptance-Finalisierung Markertexte innerhalb des eingebetteten eigenen Source-Texts faelschlich als bereits vorhandenen Lifecycle-Block interpretierte.

## Schnitt

- Marker werden ausschliesslich als vollstaendige eigene Handoff-Zeilen erkannt.
- Beliebige Teilstrings in Source-, Dokumentations- oder Diagnoseinhalt besitzen keine Block-Semantik.
- Mehrere, verwaiste oder falsch geordnete echte Markerzeilen bleiben fail-closed.
- Die Laufzeitwerte der beiden Marker bleiben unveraendert. Ihre Source-Schreibweise wird fuer den einmaligen Parent-Uebergang auf `concat!` verteilt, damit der akzeptierte r1070-Outer-Bootstrap den r1071-Child-Handoff nicht erneut an seiner alten Substring-Suche blockiert.
- Acceptance-, Rejection-, Writer-, Music-, Selection-, Proof- und Release-Authority bleiben unveraendert.

## Regression

Ein gezielter Test erzeugt einen Handoff, der die Markerwerte nur innerhalb selbstbeschreibender Source-Zeilen enthaelt. Der Upsert muss diese ignorieren, exakt einen echten Lifecycle-Block anhaengen und beim zweiten Aufruf idempotent bleiben.

## Uebergang

Der Take ist absichtlich so gebaut, dass noch der r1070-Parent-Bootstrap seinen Acceptance-Handover fuer r1071 erfolgreich finalisieren kann. Erst danach besitzt der installierte r1071-Parser die neue Zeilengrenzen-Semantik fuer folgende Takes und Recoveries.
