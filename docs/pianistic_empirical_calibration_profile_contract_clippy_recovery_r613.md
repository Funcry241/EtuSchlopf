# r613 – PianisticEmpiricalCalibrationProfileContractClippyRecovery

## Parent

r613 basiert direkt auf dem real installierten GreenBase r611
`PianisticHandConfigurationReferenceBracketAudit`.

Der Kandidat r612 `PianisticEmpiricalCalibrationProfileContract` wurde nicht installiert.
Der PRE-TAKE-DIFF-GATE war GREEN, der Kandidat wurde installiert und beim Reexec aber
vom integrierten Strict-Clippy-/Rust-Test-Gate fail-closed verworfen und vollständig
zurückgerollt.

## Rejected-r612-Ursache

Der fachliche Calibration-Profile-Contract war nicht die Ursache.
Strict-Clippy meldete:

- `PLAYABILITY_CLAIM` war unter `#![deny(dead_code)]` unbenutzt.
- zusätzlich bestand die mechanische Warnung `clippy::useless_format` in der
  synthetischen Runtime-Integrity-Rückgabe.

Damit besitzt r612 keine Authority.

## Recovery

r613 übernimmt den beabsichtigten r612-Calibration-Profile-Contract direkt auf r611
und schließt ausschließlich diese mechanischen Gegenfälle:

1. `PLAYABILITY_CLAIM` wird nun in beiden relevanten Reportpfaden tatsächlich als
   kanonische Report-Konstante verwendet.
2. Das konstante `format!` wird durch `.to_string()` ersetzt.

Keine Capability-Semantik, keine Demand-Berechnung, keine empirische Schwelle und
keine Witness-Entscheidung werden dadurch verändert.

## Fachlicher Vertrag

Der Take erfindet weiterhin keine menschliche Handspanne, keine Sprunggeschwindigkeit
und keine Tempo-Grenze. Er definiert ausschließlich den fail-closed Vertrag für ein
späteres empirisch kalibriertes Pianistenprofil.

Ein später bindbares Profil muss mindestens besitzen:

- `profile_id`
- `session_id`
- SHA-256 der unabhängigen Kalibrationsquelle
- Provenienz
- Standard-88-Tasten-Geometrie
- gebundene Rohmessungs-Provenienz
- per-Hand Whole-Configuration-Beobachtungen
- per-Hand Transition-Timing-Beobachtungen
- Same-Key-Rearticulation-Timing
- eindeutige Evidence-IDs
- positive Wiederholungszahlen

Score-Demand darf Kalibrationsfragen bestimmen, aber niemals Capability-Antworten
aus dem zu prüfenden Score selbst erzeugen.

## Claim-Grenze

Auch r613 bindet noch kein reales Pianistenprofil:

- `CalibrationProfileBinding NONE`
- `EmpiricalCalibration NOT_BOUND`
- `CapabilityInterpolation NONE`
- `HumanPlayabilityClaim NOT_EVALUATED`
- `WitnessIntegration NOT_RUN`

Ein Playability-Claim darf erst aus einem vollständigen Execution-Witness plus
unabhängigem Replay unter genau dem gebundenen empirischen Profil entstehen.

## ChangeClass

`AuditOnly`.

Keine MusicMutation, kein MasterApply, kein RenderXmlWrite, kein Writer, keine
Reparatur und keine neue Spielbarkeitsgrenze.
