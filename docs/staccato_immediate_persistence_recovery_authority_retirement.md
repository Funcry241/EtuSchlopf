# StaccatoImmediatePersistenceRecoveryAuthorityRetirement

## Ausgangslage

Autoritative Basis: v0.10.1786 r03. Vollkorpus: 10/10 dark_green.

Korpus-Summe:

- Primary-Kandidaten: 14
- Primary-Writes: 4
- unmittelbar persistierte Primary-Ziele: 4
- Persistence-Fallback attempted: 0
- Persistence-Fallback written: 0

## Statischer Unerreichbarkeitsbeweis

Der Primary-Writer `SingleClearStaccatoRenderWriteWithPlacementReadback` schreibt ueber `write_single_staccato_candidate_authorized`. Dessen `VerifiedRenderCandidateSink::write` materialisiert die vollstaendige MusicXML-Datei, liest exakt dieselbe Datei unmittelbar wieder ein und liefert nur dann `Ok(())`, wenn Readback-Bytes und FNV-Fingerprint exakt dem Kandidaten entsprechen.

Nach der Rueckkehr aus dem Primary-Writer findet vor der Persistenzpruefung kein weiterer Writer statt. Der Orchestrator liest den selben Pfad erneut. Falls dieses Lesen fehlschlaegt, wird der bereits exakt verifizierte `single_staccato_after_xml`-Kandidat verwendet.

Damit gilt hinter einem erfolgreichen Primary-Commit:

`PrimaryWriteCommitted -> PrimaryTargetPersistedAtBoundary`.

Die historische Recovery-Bedingung

`PrimaryWriteCommitted && !PrimaryTargetPersistedAtBoundary`

ist im verbleibenden Pfad statisch unerreichbar.

## Retirement

Entfernt werden:

- `force_persist_single_clear_staccato_target_from_audit`
- `SingleClearStaccatoPrimaryReadinessContract`
- der produktive Fallback-Zweig im Rest-/Artikulations-Orchestrator
- der dedizierte `ForcedVisualRest`-Checkpoint
- Fallback-/Recovery-Readiness-Evidencefelder

Erhalten bleiben:

- Primary-Staccato-Entscheidung und -Materialisierung
- unmittelbarer Persistence-Readback
- fail-closed Boundary-Postcondition
- spaetere unabhaengige Artikulations-Owner

Es gibt keinen neuen Writer und keine neue Musikregel.
