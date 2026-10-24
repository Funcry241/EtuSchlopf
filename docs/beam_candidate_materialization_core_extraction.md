# BeamCandidateMaterializationCoreExtraction

## Zweck

v0.10.1704 ist Take 3 und Abschluss des auf drei Takes begrenzten `ComposerBeamTruthOwnershipArchitecture`-Superwurfs.

Nach Observation und Planning wird die kandidatenseitige Beam-XML-Materialisierung aus `composer_recovery_octave_beam_export_tail.rs` nach `composer_beam_candidate_materialization_core.rs` verschoben.

Der Core enthaelt ausschliesslich die bereits vorhandenen Fragmentoperationen zum Entfernen/Einsetzen/Ersetzen von Beam-Tags, den beam-only Normalisierungsvergleich und die lokale Structural-Beam-Closure auf einem bereits vom produktiven Owner bereitgestellten Candidate-XML. Er waehlt keine Quelldatei, schreibt keine Datei, projiziert keinen Report und besitzt keine Structural-Write-Authority.

Die produktiven `apply_*`-Operatoren, Candidate-Seal-/Materialize-Grenzen, Invarianten-Readbacks, Audittypen und deren Reihenfolge verbleiben im bisherigen Composer-Owner. Damit ist die Schichtung nun explizit:

```text
MetricBeamObservation
        -> MetricBeamPlanning
        -> BeamCandidateMaterialization
        -> bestehende produktive Operator-/Audit-Ownership
```

Kein Beam-Zustand, keine Regel, kein Schwellenwert, keine Staff-/Voice-/Rest-Entscheidung und kein sichtbarer Output wird absichtlich geaendert.

Abnahme:

```powershell
.\run.cmd --take --strict --all
```

Erwartet: 10/10 artefaktidentische Projekte gegen v0.10.1703. Mit gruener Abnahme ist `ComposerBeamTruthOwnershipArchitecture` beendet; kein Take 4.
