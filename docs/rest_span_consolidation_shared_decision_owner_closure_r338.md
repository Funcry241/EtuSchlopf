# RestSpanConsolidationSharedDecisionOwnerClosure r338

## Ausgangspunkt

r337 `RestSpanConsolidationDecisionOwnershipEvidenceSecurityRecovery` ist der reale GREEN-Parent.
Der Lauf fuer Etuede Nr. 6 endet mit `FinalExit 0`, Build/Clippy 0, Release `released`
und 0 Blockern / 0 Warnungen / 0 Hinweisen.

r337 beweist bereits mit einer vorhandenen GREEN-Kontrollfixture einen ueberlappenden
RestSpanConsolidation-Entscheidungsraum:

- StructuralTransactionalRestCoalesce
- PianisticRestConsolidation
- LowerStaffRestCompaction
- StructuralRestBeatMaterialization

Der sichere Kontrollfall `24 + 24 -> 48` wird von allen vier akzeptiert.
Der chord-anchored Gegenfall bleibt bei allen vier fail-closed.

## r338

r338 fuehrt keine neue Restregel und keinen neuen Writer ein. Stattdessen wird die bereits
identische fachliche Entscheidung in `model::metric_rest_notation` unter einem gemeinsamen
writer-freien DecisionOwner zusammengezogen:

`decide_rest_span_consolidation`

Der Owner liefert drei explizite Projektionen:

- `SinglePartShortBeatLocal`
- `MetricCompaction`
- `SinglePartBeatLocal`

Die vorhandenen vier Materialisierungsphasen bleiben bestehen. Sie bauen weiterhin ihre
lokalen Kandidaten, schreiben weiterhin ueber ihre bestehenden StructuralWriteAuthority-
Oberflaechen und behalten ihre bisherigen Transaktions-/Readback-Gates. Nur die Entscheidung,
ob ein bereits bewiesener zusammenhaengender sichtbarer Restspan in der jeweiligen Projektion
konsolidierbar ist, wird gemeinsam getroffen.

Der chord-anchor Block ist Teil des gemeinsamen Decisions und bleibt fail-closed.

## Verhaltensinvarianz

Die bisherigen lokalen Praedikate werden 1:1 in die drei Projektionen ueberfuehrt:

- Pianistic und Transactional: ein Teil, kurze Quellfragmente, beat-local.
- LowerStaffCompaction: metrische Reduktion auf weniger Teile.
- StructuralRestBeatMaterialization: ein Teil, beat-local.

Darstellbarkeit der resultierenden Resttypen, Overlap-Schutz, Transaktionsinvarianten,
Writer-Autoritaeten, Pass-Reihenfolge und maximale Aktionsbudgets bleiben unveraendert.

Die bestehende GREEN-Kontrollfixture bleibt erhalten. Zusaetzlich prueft der
MetricRestNotation-Owner direkt den sicheren `24+24 -> 48`-Fall, den chord-anchor
Fail-Closed-Vertrag und die projektionserhaltende Abgrenzung.

## Ownership-Evidence

Weil r338 ein verhaltensinvarianter `RefactorOnly`-Cut ist, wird die bereits in r337
eingefuehrte Audit-Evidence im selben Take auf den realen Zielzustand projiziert:

`single_shared_decision_owner_multiple_materialization_projections`

Damit ist kein separater AuditOnly-Nachtake erforderlich.

## Nicht geaendert

- keine neue MusicXML-Regel
- keine Schwelle
- keine Pass-Reihenfolge
- keine Writer-/Repair-/Recovery-Authority
- keine Etueden-/Takt-Sonderlogik
- keine sounding-event Semantik
- keine Security-/Gate-Lockerung
- keine Pensionierung der vier Materialisierungszeitpunkte
