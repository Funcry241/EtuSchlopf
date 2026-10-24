# r299 – PianisticReferenceAssumptionCapabilityModelAuthority

## Zweck

r299 bindet erstmals ein explizites Referenz-Annahmemodell als eigene read-only Autorität. Es ist **kein empirisches Pianistenprofil** und darf keinen menschlichen Spielbarkeits-Claim erzeugen.

Die Modellquelle ist unabhängig vom r297-Demand-Inventar definiert. Das Demand-Inventar formuliert Fragen; r299 übernimmt keine der 105 statischen oder 509 Transition-Signaturen als automatische Antworten.

## Modellidentität

- ModelId: `etuschlopf-reference-assumption-standard88-static-v1`
- EvidenceBasis: `REFERENCE_ASSUMPTION`
- ClaimBoundary: `MODEL_RELATIVE_ONLY`
- ModelSourceSha256: `4d6eedf088607e4e9bc44a53483df1238b2e8ee9d6a1346db89dbf6f93d518b0`
- ScoreDemandUsedToDefineModel: `no`
- HumanPlayabilityAuthority: `no`

## Statische Modellannahme

Das Modell arbeitet auf der Standard-88-Tasten-Geometrie.

Für eine gleichzeitige Einzelhand-Konfiguration gilt modellrelativ:

1. höchstens fünf verschiedene Tasten / fünf verschiedene Finger,
2. eindeutige Taste je Finger und eindeutiger Finger je Taste,
3. monotone Fingerreihenfolge innerhalb einer gleichzeitig gehaltenen Handkonfiguration,
4. äußerer Abstand der gleichzeitig gehaltenen Tasten höchstens 12 Halbtöne.

Der vierte Punkt ist ausdrücklich **eine Modellannahme**, keine behauptete anatomische Universalgrenze. Er wird nicht in den allgemeinen `PianisticConfigurationConstraintKernel` eingebaut und verändert dessen exakte `SUPPORTED / REJECTED / UNMODELED`-Semantik nicht.

## Transition-Modell

r299 setzt für Transitionen noch **keine menschliche Mindestzeit**.

```text
TransitionPolicy = TEMPORALLY_RELAXED_ZERO_LOWER_BOUND
MinimumTransitionTimeUs = 0
```

Damit kann dieses Modell später höchstens eine globale statische Hand-/Finger-Realisierbarkeit unter entspannten Zeitannahmen falsifizieren. Es kann keine Tempo-Frontier und keinen menschlichen Ausführbarkeitsnachweis tragen.

## Literaturkontext

Die folgenden Quellen motivieren ausschließlich die strikte Trennung von Standardklaviatur, Handgröße und Belastung. Sie definieren **nicht** die r299-Modellschwellen:

- Sakai et al., *Hand Span and Digital Motion on the Keyboard*, Journal of Hand Surgery, 2006, DOI `10.1016/j.jhsa.2006.02.009`.
- Boyle et al., *Interaction between hand span and different sizes of keyboards on EMG activity in pianists*, Applied Ergonomics, 2021, DOI `10.1016/j.apergo.2021.103518`.
- Turner et al., *An alternative 6-inch octave keyboard reduces forearm muscle activation level and improves finger postures in expert pianists with smaller hand spans*, Applied Ergonomics, 2026, DOI `10.1016/j.apergo.2025.104709`.

## Sicherheitsgrenzen

- AuditOnly
- WriterAuthority: none
- MusicMutation: no
- MasterApply: no
- RenderXmlWrite: no
- StaffIsHand: false
- VoiceIsHand: false
- HumanPlayabilityClaim: forbidden for this model
- WitnessIntegration: deferred

## Nächster Schritt

Nach real grüner r299-Verifikation darf r300 dieses `MODEL_RELATIVE_ONLY`-Modell an den bestehenden `PianisticExecutionPlanWitness` anbinden.

Ein dort gefundener Witness wäre dann ausschließlich:

`REALIZATION_WITNESS_FOUND_UNDER_REFERENCE_ASSUMPTION`

und weiterhin **kein menschlicher Playability-Claim**.

Die zeitliche/tempoabhängige Capability bleibt danach ein eigener Schritt.
