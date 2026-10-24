# WavTraceIdentityBoundWriterRetirementSuperCut

## Ziel

Die historische WAV-Trace-Agogik-/Tempo-Writerfamilie wird vollstaendig aus dem
produktiven Runtimepfad und aus der StructuralWriteAuthority entfernt.

## Befund im autoritativen v0.10.1764/r01

Der Agogikwriter trug elf konkrete Zielnotentupel aus Scope, Staff, Voice,
Onset, Duration und MIDI. Der berechnete Tempo-Writer war zusaetzlich an einen
historischen Sonderfall `q=126 -> 121 -> 126` mit `+71 ms` gebunden.

Im realen Referenzlauf Etuede 1 waren beide Pfade write-dead:

- Agogik: `TargetsBound 0/11`, `RenderXmlWrite nein`.
- Tempo: `LocatorCandidates 0`, `RenderXmlWrite nein`.

Die Pfade waren trotzdem produktive Writerfamilien und besassen eigene
StructuralWrite-Capabilities. Historische Diagnosekoordinaten sollen
Regressionadressen bleiben und kein produktives Schreibrecht besitzen.

## Retirement

Entfernt:

- `WavTraceAgogicTimingMarkRenderWriteAudit` samt Writer und 11er-Zieltabelle.
- `WavTraceCalculatedTempoMarkRenderWriteAudit` samt Source-Bound-Locator und
  festem Tempo-/Offset-Sonderfall.
- beide Reportsektionen.
- beide PDF-Source-Evidence-Eintraege.
- die zugehoerigen Tests.
- `write_agogic_trial_xml_authorized`.
- `write_calculated_tempo_trial_xml_authorized`.
- `insert_bound_direction_batch_authorized` samt seiner nur dafuer vorhandenen
  DirectionInsertionBatchCapability.

Die allgemeine Working-Memory-Reportpflege wird aus dem alten Misch-Shard
herausgeloest und unveraendert in
`pipeline_shards/working_memory_report_prune_core.rs` weitergefuehrt.

## Invarianten

Der Take ist `RefactorOnly`.

Nicht geaendert werden:

- Master-MusicXML-Musik.
- Render-MusicXML-Musik.
- Beam-/Rest-/Gould-Entscheidungen.
- MetricRestNotation.
- PDF-Layout.
- Gate-Policy.

Erwarteter Artefaktbeweis: v0.10.1764/r01 und v0.10.1765/r01 unterscheiden sich
musikalisch nicht; PDFs sind pixelgleich.

## Quellbilanz

Aktiver `src`-Rust: 123.432 -> 122.314 Zeilen, netto -1.118 Zeilen.
