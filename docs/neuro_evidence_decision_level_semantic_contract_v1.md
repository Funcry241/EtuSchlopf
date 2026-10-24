# NeuroEvidenceDecisionLevelSemanticContractV1

Dieser `SecurityOnly`-Take zieht nach dem strukturell erfolgreichen r40-End-to-End-Beweis den bestehenden EtuSchlopf -> NeuroSchlopf Machine-Evidence-Vertrag auf Decision-Level fest.

Der aeussere Trust-Container bleibt unveraendert:

```text
etuschlopf_neuro_evidence_<UTC>.zip
|- manifest.json
`- decisions.jsonl
```

`manifest.json` behaelt `artifact-kind=etuschlopf_neuro_evidence`, Schema 1, `producer=EtuSchlopf`, `corpus-id=etuschlopf-tonal-audit`, `decision-class=tonal_segmentation_boundary_audit_v1` und `decision-contract-fingerprint=fnv64:4f2c765d00c527e6`. Bytegroesse, SHA-256, flache Zwei-Dateien-Closure, temporarer Readback und atomare Publikation bleiben unveraendert.

## Decision-Level-Payload

`decisions.jsonl` enthaelt keine aggregierte Run-Summary mehr. Es enthaelt genau einen JSONL-Datensatz pro tatsaechlichem `KeyChangeCandidate` des bereits existierenden `TonalSegmentationEvidenceAudit`. Der Producer konsumiert dafuer ausschliesslich den bereits maschinell erzeugten `TonalBoundaryLedger` im Ergebnisbericht und verlangt strikte Closure gegen `KeyChangeCandidates`, `FullEvidenceCandidates`, `PromisingCandidates` und `HeldCandidates`. Eine gekuerzte, mehrdeutige oder inkonsistente Ledger-Projektion publiziert kein Evidence-Artefakt.

Jeder Datensatz besitzt:

```text
schema_version = 1
evidence_id = stabile SHA-256-Maschinen-ID aus Decision-Class, Projekt und Boundary-Locator
contract.owner = TonalSegmentationEvidenceAudit
contract.decision_class = tonal_segmentation_boundary_audit_v1
contract.epistemic_kind = unresolved
contract.contract_fingerprint = fnv64:4f2c765d00c527e6
label = null
proof = null
```

Die `decision_observations` transportieren ausschliesslich bereits maschinell bestimmte Rohsignale des Tonal-Audits als `pre_decision`:

```text
source_fifths
candidate_fifths
lbdm
bee
margin
stable
bass_cadence
accidental_cost_before
accidental_cost_after
hmm
green_groups
counter_boundary_weak
counter_profile_margin_low
counter_short_tonicization_risk
counter_bass_cadence_unconfirmed
counter_notation_saving_small
counter_transition_prior_rejects
counter_none_but_corpus_calibration_open
```

Der historische EtuSchlopf-Audit-Verdict wird separat als `source_audit_verdict` mit `phase=post_decision` transportiert. Er ist BenchmarkReference, nicht GroundTruth und nicht Learning-Target.

Die drei `legal_candidates` sind immer exakt:

```text
promising_candidate_more_corpus_calibration
held_as_tonicization_or_chromatic_colour
full_evidence_candidate_write_still_closed
```

Jeder Kandidat besitzt ausschliesslich die drei `pre_decision` One-Hot-Observations `choice_promising`, `choice_held` und `choice_full_evidence`.

## Provenienz

Jede Evidence-Zeile bindet echte Maschinen-Provenienz:

```text
etuschlopf_version       = reale Cargo-Version plus installierte ZIP_KIND-Revision
etuschlopf_source_identity = bestehende source_build_signature des Launchers
corpus_run_id            = reale Observation-Session-ID des aktuellen Launcher-Laufs
project_id               = aktuelles Projekt
locator                  = Boundary-Measure plus Audit-Fenster
```

Diese Provenienz bleibt reine Provenienz und ist kein automatisch freigegebenes Lernfeature.

## Trust-Domain und Fail-Independence

Keine Feature-Vektoren, keine Ground Truth, kein Learning-Target, keine Prediction, keine HumanPreference, kein Assistant-Kontext, kein Handover, README oder Diagnosefreitext werden transportiert. `assistant_export.rs` bleibt unveraendert und kennt den Neuro-Producer weiterhin nicht.

Fehlt eine erforderliche maschinelle Ledger-Information oder stimmt die Decision-/Verdict-Closure nicht, wird kein Neuro-Evidence-Artefakt publiziert. Der bereits deterministisch abgeschlossene EtuSchlopf-Musiklauf bleibt davon unberuehrt.

Fuer den beobachteten Etuede-6-Lauf gilt damit als Abnahmekriterium:

```text
BoundariesChecked     43
KeyChangeCandidates   18
FullEvidenceCandidates 0
PromisingCandidates    8
HeldCandidates        10
DecisionEvidence      18
potentielle CandidateRows in NeuroSchlopf 54
GroundTruth           NONE
```
