# MetricFinalNotationPhaseContracts

## Ziel

Der bisherige flache `MetricBeamAndFinalNotationStageOutcome` mit 30 Feldern wird in vier fachlich zusammenhängende Phasenverträge zerlegt.

## Verträge

- `RestAndArticulationRenderStageOutcome`
- `MetricBeamAndRegisterStageOutcome`
- `NotationAndLayoutStageOutcome`
- `FinalRenderNotationAuthorityStageOutcome`

Der äußere Stage-Outcome besitzt damit nur noch vier Felder. Bereits vorhandene Stage-Outcomes werden direkt weitergereicht, statt nach dem Aufruf sofort zerlegt und anschließend identisch wieder aufgebaut zu werden.

## Messbare Strukturwirkung

- äußerer Outcome: 30 auf 4 Felder
- wiederverwendete bestehende Verträge: 9 + 9 Felder
- neue Fachpakete: 6 + 6 Felder
- Metrik-/Finalnotationsdatei: 432 auf 397 Zeilen
- Rust gesamt gegenüber `0.10.1674`: -23 physische Zeilen
- `#[allow(...)]`: unverändert 25

## Verhaltenstreue

- Alle bestehenden Auditwerte werden unverändert erzeugt und an denselben nachgelagerten Code übergeben.
- Der bestehende Beam-Variant-A-Audit bleibt Bestandteil der Rest-/Artikulationsphase und wird unverändert weitergereicht.
- Writer, Passreihenfolge, XML-Snapshots, Releasebedingungen, Reports und Fehlertexte bleiben unverändert.
- Es werden keine neuen Clippy-Bereinigungen oder physischen Support-Extraktionen vorgenommen.

## Stopkriterium

Der äußere Outcome hat vier fachliche Felder; danach endet der Take.
