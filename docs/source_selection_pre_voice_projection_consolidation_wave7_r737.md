# r737 – SourceSelectionPreVoiceProjectionConsolidationWave7

## Ziel

Cleanup unmittelbar vor dem spaeteren Voice-/Tie-/Duration-Cutover, ohne bereits eine dieser produktiven Authorities zu uebernehmen oder T3/T16/T22 zu reparieren.

## Pensionierte Vorstufen

Die read-only Projektionen `SourceSelectionIdentityTransportProjection` und `SourceSelectionDecisionReasonProjection` waren nach dem spaeteren Full-Causal-Cutover redundant. Ihre einzigen produktiven Consumer waren zwei eigene Final-Report-Sektionen; sie besassen keine Writer-, Voice-, Tie-, Hand-, Staff-, Ranking-, Threshold- oder MusicMutation-Authority.

Entfernt werden deshalb die beiden Projektionsstrukturen, ihre Builder, ihre `MidiSeedStats`-Transportfelder, ihre Report-Owner, die zwei kleinen Auditmodule sowie die ausschliesslich diese alten Sektionen pruefenden Contract-Tests.

## Lebende Nachfolge-Authority

`SourceSelectionFullCausalAudit` bleibt unveraendert und bindet bereits fail-closed:

- lueckenlose/doppelfreie CanonicalSourceEvent-Ordinalitaet,
- retained Payload fuer Position, Dauer und Pitch,
- finale retained/removed-Partition,
- Removal-Reasons gegen Remove-Decisions,
- Staff-Moves und finalen Trace-State,
- den produktiven SelectionPlan und seine spaeteren SourceDuration-/Technical-Lane-Bindungen.

Der bestehende `source_selection_full_causal_audit_shadow_plan_contract` bleibt erhalten. r643/r644, Representation-Neutrality und Sounding-Material-Mutation-Frontier bleiben ebenfalls erhalten.

## Authority-Grenze

Dieser Take uebernimmt weder `MusicXmlVoiceAuthority` noch `TieWriterAuthority`; beide bleiben `none`. Keine MusicXML-, Voice-, Tie-, Duration-, Chord- oder Carrier-Mutation wird hinzugefuegt oder entfernt.

## Bilanz

- Parent: r736, dark_green/released.
- Rust r736: 198.679.
- Rust r737: 198.028.
- Netto: -651.
- Kumuliertes physisches Retirement gegen 203.261: -5.233 / -60.000.
