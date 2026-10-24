# r734 – ArticulationProofReentryReaderIsolationRefactor

## Anlass

r733 wurde vor Installation korrekt gestoppt. Der geplante MusicMutation-Cut
veraenderte `run_rest_and_articulation_render_stage`, obwohl diese Funktion
durch ihren unmittelbaren Primary-Staccato-Artefakt-Readback zugleich ein
`ProofReentryReader` ist.

## Refactor

r734 verschiebt ausschliesslich diesen einen direkten Dateiread in
`readback_primary_staccato_boundary_artifact`. Der neue Reader fuehrt exakt

`std::fs::read_to_string(render_xml_trial_path).unwrap_or(fallback_xml)`

aus. Der Aufrufer uebergibt denselben Pfad und denselben bereits berechneten
Fallback-String wie zuvor. Es gibt keine zusaetzliche oder entfallene
Materialisierung.

Die gesamte Writer-Orchestrierung bleibt in r734 unveraendert:

- `SingleClearStaccatoRenderWriteWithPlacementReadback`,
- `LegacyLowerStaccatoProxySuppressRenderWrite`,
- `AssertiveVisibleArticulationRenderWrite`,
- WAV-Truth-Binding,
- AccentReserve und ExternalNotationFeedback.

Ebenso bleiben die bisherigen Checkpoints, Auditwerte, Reportzeilen und
Failure-Postconditions unveraendert.

## Zweck

Die Proof-Reentry-Capability wird nicht entfernt oder getarnt, sondern in einen
dedizierten readback-benannten Reader isoliert. Dadurch ist der restliche
Artikulations-Orchestrator im Folge-Take nicht mehr gleichzeitig die direkte
Proof-Reentry-Grenze. Erst danach wird der r733-Retirement-Inhalt erneut als
MusicMutation konstruiert.

## Parent

Autoritativer Parent ist r732. r733 bleibt rejected und wird weder installiert
noch als Parent benutzt.
