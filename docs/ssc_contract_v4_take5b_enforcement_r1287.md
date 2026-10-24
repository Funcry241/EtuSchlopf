# r1287 – SSC-Vertrag v4, Take 5b: Enforcement an den A7-Adoptionsgrenzen

Parent: accepted r1286 `SscK2GovernanceRevisionKodex` (auf r1285 `SscAllCommitMeasurementV4`).
Klasse `GovernedChange`, `music_effect=none`.

## Grundlage

r1285 hat jeden Commit gemessen, auch die später verworfenen Kandidaten: 1438 released und 12 635 unreleased Commits, alle `Permit`, ohne Integritätsfehler, `V4AllCommitsPermitted: yes` in 10/10 Projekten. Damit ist A7.4 für alle Commits erfüllt.

r1286 hat den K2-Testbaum so korrigiert, dass die live Identitätsdatei mit ihrem installierten Wiring beurteilt wird. Ohne diese Korrektur wäre jede zulässige Verdrahtung der Master-Grenze in drei K2-Tests als Escape gezählt worden.

## Verdrahtung (verdict_wiring)

| Grenze | Funktion | Verhalten |
|---|---|---|
| Master | `src/master_state_identity.rs::commit_owner_receipts` | Für **alle** Receipts der Transition werden zuerst Kontinuität (Pre = laufender Kopf), Identity (Lineage, Genesis, Batch) und das V4-Verdikt geprüft, danach die Abdeckung von `state_after`. Erst dann wird committet. |
| Render | `src/master_state_identity/render_lineage.rs::commit` | Das Verdikt jedes Schritts kommt vor jeder Lineage-Änderung, auch vor Genesis- und FromMaster-Elementen. |

- Der Receipt-`commit` selbst bleibt unverändert und unverdrahtet.
- `ssc_adoption_verdict` steht ausschließlich in den beiden verdrahteten Funktionen.
- Die Receipt-Sicht (`adoption_view`) trägt keinen geschützten Namen und keine Autorität.

## Verdikt (proof_rule, `src/ssc_evidence_proof.rs`)

- `ssc_adoption_verdict(origin, AdoptionReceiptView, SscAdoptionDomain) -> Result<(), String>`:
  - Replay-Schutz: Ist die Receipt-Identity schon adoptiert, folgt die Verweigerung mit `DuplicateEvidence`.
  - Dann `classify_bound_origin`, **derselbe** Klassifikator wie im Bericht. Er prüft Identity-, Scope- und Fingerprint-Bindung, Captures, Invariante und Containment.
  - Additive-Origin aus der Pending-Bindung, nur gelesen.
- `adoption_decision`: Zugelassen wird nur bei genau einem `Permit` ohne Integritätsfehler. Sonst wird verweigert, mit Verdiktzahlen, Integritätsklassen und der vollständigen `V4DeniedWitness`-Zeile.
- Es gibt keinen Cache: jeder Receipt wird einzeln frisch klassifiziert.
- Verweigerung:
  - Master: `MASTER SOVEREIGNTY VIOLATION`;
  - Render: `RENDER SOVEREIGNTY VIOLATION: ReceiptCoverageFailures`.

  Kein Teilcommit, kein Fallback.
- Bericht:
  - `PermitEnforcement: active`;
  - `EnforcementReadiness: active_at_adoption`.

  Alle übrigen V2- und V4-Zeilen sowie die r1285-Messung (`V4UnreleasedCommits`, `V4AllCommitsPermitted`) bleiben unverändert.

## Tests

- **Verweigert, Lineage und Dokument unverändert:**
  - Exact mit Test-Descriptor ohne Zeile (`unproven=1`). Das erste, zulässige Receipt derselben Transition wird **nicht** committet.
  - Block ohne eingefrorene Zeile.
  - Additive ohne gebundenen Origin (`MissingEvidence`).
  - Echter Scalar-Owner außerhalb seiner Zeile (`denied=1`, mit Witness).
- **Entscheidung über den Bericht-Klassifikator:** Fremde Identity, fremder Scope, stale Pre, abweichender Post und kaputte Pre/Post-Captures werden verweigert. Nur die gebundene Release ist zugelassen.
- **Positiv unter Enforcement:**
  - Lineage-, Transport-, Snapshot- und Render-Tests über echte eingefrorene Descriptoren: HARMONIC_PITCH_SWAP_EXACT, CLEF_MEASURE_BLOCK, PIANIST_LAYOUT_PRESERVING, DIRECTION_INSERTION_ADDITIVE, MASTER_EXPORT_FIFTHS_EXACT und FUNCTIONAL_RESPELLING_EXACT;
  - echte Owner (NoteVisibility, AllowedRange, Pitch-Swap, Scalar, Clef-Block).
- **Bestehende Tests mit Test-Descriptoren:** Sie committeten bisher Receipts ohne Envelope-Zeile. Sie nutzen jetzt Permit-fähige Receipts oder prüfen die Verweigerung. Eine Test-Ausnahme im Commit-Pfad gibt es nicht.

## Bekannte Kosten

Jeder Commit wird jetzt beim Commit und noch einmal im Bericht klassifiziert. Etüde 6 hat rund 4070 Commits. Die Laufzeit steigt entsprechend; Musik und Adoption ändern sich nicht.

## Erwartung auf Windows

- FinalExit 0, 10/10.
- 1438 released `V4Permitted`, `V4UnreleasedCommits` wie in r1285, `V4AllCommitsPermitted: yes`, `PermitEnforcement: active`.
- 20/20 XML bytegleich, PDF-Seiten gerendert gleich.
