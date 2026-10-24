# r337 RestSpanConsolidationDecisionOwnershipEvidenceSecurityRecovery

## Autoritative Basis

Parent ist der real installierte und GREEN gelaufene Stand
`v0.10.1839 r335 BeamDomainOwnershipEvidenceProjectionClosure`.

r336 wurde vom Take-Change-Control vor Installation blockiert und ist keine
GreenAuthority. Der Grund war korrekt: Eine neu hinzugefuegte AuditOnly-Testfunktion
hatte per `include_str!` eine produktive Rest-Datei referenziert, deren Pfad den fuer
AuditOnly verbotenen Mutations-/Writer-Token enthielt.

Die Security-Regel wird nicht gelockert. r337 entfernt genau diese unzulaessige
Cross-Role-Bindung und startet erneut direkt von r335.

## Bereits im r335-Bestand vorhandener Beweis

Der bestehende GREEN-Test
`chord_anchored_sounding_events_block_initial_rest_coalescing`
beweist bereits die fuer diesen Audit benoetigte Ueberlappung.

Eine sichere zusammenhaengende Restkette aus zwei 16tel-Resten
`24 + 24 -> 48` wird von vier verschiedenen produktiven Entscheidern
als konsolidierbar akzeptiert:

- StructuralTransactionalRestCoalesce
- PianisticRestConsolidation
- LowerStaffRestCompaction
- StructuralRestBeatMaterialization

Der chord-anchored Gegenfall wird von allen vier fail-closed abgelehnt.

Damit ist ein realer gemeinsamer RestSpanConsolidation-Entscheidungsraum
bereits durch den installierten Parentbestand bewiesen. r337 erzeugt dafuer
keine neue produktive Reachability.

## Dieser Take

r337 projiziert ausschliesslich diese bereits vorhandene Evidenz in die
Mutation-Ownership-Evidence.

Die Projektion markiert:

`RestSpanConsolidation -> duplicate_decision_space_proven_owner_unresolved`

und blockiert ausdruecklich einen produktiven Cut, solange noch kein einzelner
DecisionOwner festgelegt wurde.

Die AuditOnly-Datei referenziert keine produktive Source-Datei und ruft keinen
produktiven Pfad auf. Ihre bestehenden lokalen Report-Tests pruefen nur, dass
die Ownership-Projektion im Audit-Report erhalten bleibt.

## Nicht geaendert

Keine Aenderung an:

- produktiven Music-Domain-Dateien
- Writer-Roots
- StructuralWriteAuthority
- Pass-Reihenfolge
- Rest-Planern oder Schwellen
- Pitch, Onset, Duration, Staff, Voice
- MusicXML oder PDF
- Security-Gates

## Naechster Schritt bei GREEN

Wenn r337 GREEN ist, folgt kein weiterer Audit-Take.

Dann wird direkt der produktive `RestSpanConsolidationOwnershipClosure`
vorbereitet. Der historische Kontrollfall `Etüde 5 / Takt 48` bleibt fuer die
corpusweite Abnahme relevant; r337 behauptet dazu keine neue Runtime-Tatsache.
