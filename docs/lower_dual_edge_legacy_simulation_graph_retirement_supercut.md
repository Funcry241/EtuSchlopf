# v0.10.1817 r03 – LowerDualEdgeLegacySimulationGraphRetirementSuperCut

Autoritative Basis ist der real grüne Windows-Stand `v0.10.1816 r01 Rust2024StyleEditionContract`.
Dieser `RefactorOnly`-Super-Take schließt den seit v0.10.1785 pensionierten
LowerDualEdge-Mutations-/LocalVoiceBatch-Simulationsgraphen, ohne einen neuen
Writer-Root zu erzeugen und ohne die lebende
`FinalCompleteBeatPrimaryBeamContinuity`-Autorität zu verlagern.

## r01 – Security-Gegenbeispiel

Der verworfene r01-Kandidat führte zusätzlich
`lower_dual_edge_beat_regression_observation` ein. Obwohl diese Funktion fachlich
read-only gedacht war, war sie eine neue Funktionsidentität in einem historischen
`repair`-Pfad und erreichte über `parse_musicxml` statisch einen Writer. Das
`NO-NEW-REPAIR-AUTHORITY`-Gate blockierte diesen Kandidaten korrekt.

## r02 – Owner-preserving Hauptschnitt

r02 löste das Gegenbeispiel durch Subtraktion statt durch Security-Ausnahme:

- keine neue produktive Funktion im historischen Repair-Root,
- `lower_dual_edge_beat_regression_candidates` blieb gegenüber dem real grünen
  1816-Parent byteidentisch,
- keine Ersatz-Observation für Measure-/Offset-Diagnostik,
- keine Helper-Relocation,
- kein neuer Writer-/Repair-/Recovery-Root.

Der reale Strict-Clippy-Compile von r02 zeigte anschließend neun `deny`-Fehler.
Alle neun waren consumerlose Restkanten des entfernten Legacy-Graphs.

## r03 – Strict-Clippy Dead-Graph Closure

r03 entfernt ausschließlich diese Restkanten. Es werden keine neuen Funktionen,
Writer, Repairs, Recoveries, Regeln, Schwellen oder Musikentscheidungen eingeführt.

### Timeline-Restgraph

Der `render_lane_timeline_observation_core` wurde vor 1817 ausschließlich vom nun
pensionierten LowerDualEdge-/LocalVoiceBatch-Graph konsumiert. Nach r02 waren
`RenderTimelineNoteObservation` und `observe_render_timeline_note` daher
consumerlos. r03 entfernt den Include und die Datei vollständig. Damit werden auch
die vier nun consumerlosen Bestandshelfer entfernt:

- `local_voice_batch_child_duration`,
- `local_voice_batch_child_text`,
- `local_voice_batch_parse_pitch`,
- `local_voice_batch_stable_fingerprint`.

### BeamPlan-Restgraph

`BeamPlan::from_explicit_states` und
`BeamPlan::from_event_levels_with_continuity` wurden im produktiven Baum nur noch
vom pensionierten LowerDualEdge-Graph konsumiert. Der Continuity-Konstruktor war
zugleich der einzige produktive Konstruktor für `BeamState::ForwardHook` und
`BeamState::BackwardHook`. r03 entfernt daher die beiden consumerlosen
Konstruktoren, ihre ausschließlich dazugehörigen Tests und die beiden nun
unerreichbaren Zielplan-Varianten.

Der crate-weite `BeamState`-Reexport war nach dem Hauptschnitt ebenfalls
consumerlos und wird entfernt. `BeamState` bleibt intern in der bestehenden
StructuralWriteAuthority, wo die lebenden Begin/Continue/End-Pläne ihn verwenden.
Die Gould-Observation kann MusicXML-Hooks weiterhin erkennen; Hooks sind lediglich
kein erzeugbarer erwarteter Zielzustand eines lebenden `BeamPlan` mehr.

## Hauptschnitt

Entfernt bleiben die seit 1785 sink-unreachable Legacy-Pfade:

- `patch_lower_dual_edge_beat_beam_continuity_in_xml`,
- der zugehörige Measure-Patcher und Before-Plan,
- `LocalVoiceBatchPlan` / Decision / Materialisierung,
- Candidate- und Final-Verification des pensionierten Batch-Graphen,
- historische LocalVoiceBatch-Patch-/Span-/Role-Helfer,
- ausschließlich dazugehörige Batch-/Transactional-Auditfelder,
- die dadurch inhaltslosen Reportsektionen
  `ORTHOGONAL-BEAM-ONLY-SPAN-TRANSACTIONAL-APPLY` und
  `DISTINCT-SOURCE-LANE-VOICE-SPLIT-TRANSACTIONAL-APPLY`.

## Lebende Autorität bleibt erhalten

Der bestehende Funktionsroot

```text
apply_lower_dual_edge_beat_beam_continuity_repair
```

bleibt bestehen. Innerhalb dieses bereits autorisierten historischen Roots bleibt
`FinalCompleteBeatPrimaryBeamContinuity` unverändert produktiv.

Der vollständige Closure-Text des lebenden Canonicalizers ist gegenüber 1816
byteidentisch:

```text
SHA-256 0305d2ad7900648c6ba7d4eb90047aaed0f45af242173aec6dad862b52828c94
```

Auch der bereits autorisierte read-only Detector

```text
lower_dual_edge_beat_regression_candidates
```

bleibt byteidentisch zum 1816-Parent:

```text
SHA-256 2a91d6a3c3055acdb0f29cf69ad652d8d96565e9348e036f4e668b8327554cd6
```

LowerDualEdge selbst materialisiert keine XML-Bytes mehr. Detaillierte historische
Target-Measure-/Edge-Offset-Felder werden nicht neu rekonstruiert; der Report
kennzeichnet diese Legacy-Diagnostik als pensioniert.

## Architekturpostcondition

```text
LegacySimulationExecution = 0
Legacy LowerDualEdge mutation graph = absent
Legacy LocalVoiceBatch materialization graph = absent
Legacy RenderTimeline observation graph = absent
Legacy explicit/continuity BeamPlan constructors = absent
LowerDualEdge productive write authority = none
Existing LowerDualEdge candidate detector = byteidentical to 1816
FinalCompleteBeatPrimaryBeamContinuity closure = byteidentical to 1816
New productive function identity in historical repair root = none
New writer root = none
New repair/recovery layer = none
Helper relocation = none
```

Die Mutation-Ownership-Projektion bleibt entsprechend korrigiert:
`BeamLocalVoiceBatch` ist nicht mehr `structured_primary_chain_existing`, sondern
mit dem LowerDualEdge-Legacy-Simulationsgraphen pensioniert.

## Erwartete Windows-Abnahme

- `ChangeDependencyPreflight` grün,
- `NO-NEW-REPAIR-AUTHORITY` grün,
- Strict/Launcher/Security grün,
- Etüde Nr. 6 formal unverändert,
- keine neue Writer-/Repair-/Recovery-Autorität,
- produktive MusicXML-/Render-MusicXML-Semantik gegenüber 1816 invariant,
  abgesehen von Versionsmetadaten,
- Report-Differenz ausschließlich durch die wahrheitsgemäße Entfernung der toten
  Legacy-Projektionen und Diagnosefelder.

Ein-Befehl-Abnahme:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```
