# r298 PianisticCapabilityModelProvenanceAndBindingContract

## Zweck

r297 hat den endlichen Fragenraum des realen Scores materialisiert. r298 verhindert, dass dieser Fragenraum selbst zur Antwortautoritaet wird. Ein Capability-Modell muss eine separat identifizierte Quelle besitzen.

## Zwei Provenienzklassen

- `EMPIRICAL_CALIBRATION`: explizite Kalibrierung eines opaque benannten Profils. Diese Klasse kann spaeter prinzipiell fuer einen menschlichen Claim qualifizieren, ist aber durch die reine Bindung noch kein Spielbarkeitsbeweis.
- `REFERENCE_ASSUMPTION`: explizit deklarierte Modellannahme. Sie darf Suche, Gegenbeispiele und Modellfalsifikation antreiben, bleibt aber dauerhaft `MODEL_RELATIVE_ONLY` und darf keinen menschlichen Playability-Claim autorisieren.

## Bindungsvertrag

Jede Modellbindung benoetigt mindestens:

- eindeutige `model_id`,
- eigene `model_source_sha256`,
- nichtleere Provenienz,
- explizite Evidenzbasis,
- bei Referenzannahmen einen nichtleeren Annahmevertrag,
- bei empirischer Kalibrierung eine opaque Profil-ID ohne personenbezogene Klartextdaten.

Das r297-Demand-Inventar definiert nur die zu beantwortenden Konfigurations- und Transition-Fragen. Es ist keine Capability-Evidenz und darf nicht als zirkulaere `alles SUPPORTED`-Quelle benutzt werden.

## Claim-Grenze

`REFERENCE_ASSUMPTION -> MODEL_RELATIVE_ONLY` ist hart. Ein Versuch, daraus einen menschlichen Playability-Claim zu autorisieren, wird fail-closed abgewiesen.

`EMPIRICAL_CALIBRATION -> HUMAN_CALIBRATION_ELIGIBLE` bedeutet nur, dass die Provenienzklasse einen spaeteren menschlichen Claim prinzipiell tragen kann. Witness, Independent Replay, Modellabdeckung und weitere Falsifikation bleiben zwingend.

## Realer r298-Stand

Fuer Etuede 6 wird noch kein produktives Capability-Modell gebunden. Erwartet:

- `BindingRuntimeIntegrity PASS`
- `ProductiveCapabilityModel none`
- `BindingStatus NONE`
- `PlayabilityClaim NOT_EVALUATED`

Keine Musikmutation, kein Writer, keine Hand-/Fingerbelegung, keine Transitionzeit und keine Feasibility-Aussage.
