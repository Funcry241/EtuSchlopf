# v0.10.1793 r03 – SourceLaneVoiceOwnershipClosure

## r03 – Thirty-Surface Centralized-Writer Closure


Der r02-Kandidat bestand anschließend den Semantic-Capability-Diff und wurde transaktional installiert, wurde aber beim Architecture-Build-Gate wieder zurückgerollt: Durch einen fünften direkten Aufruf des historischen lokalen Surfaces `ensure_child_text_value_local` stieg dessen gepinnte Thirty-Surface-Occurrence von 4 auf 5. Auch dieses Gate wird nicht gelockert. r03 behält die identische Voice-DecisionScope-Logik, materialisiert den neuen Voice-Wert aber direkt über die bereits zentralisierte `ensure_arrangement_child_text_value_authorized`-Authority. Dadurch bleibt `ensure_child_text_value_local` exakt auf seinem gepinnten Stand 4/4; es entsteht kein neuer lokaler Surface und kein neuer produktiver Writer-Knoten.

Der r01-Kandidat wurde vom Semantic-Capability-Diff vor jeder Installation korrekt
fail-closed abgewiesen. Die Fachregel war richtig, ihre erste Autoritätsplatzierung
aber nicht: Das neue Modul `structural_write_authority/source_lane_voice_identity.rs`
führte einen neuen writer-erreichbaren Funktionsknoten in den transitive Reachability-
Graphen des geschützten Audit-Orchestrators `pipeline_orchestrator_report_compaction::real_main`
ein. MusicMutation darf dessen Audit-WriterReachability nicht verändern.

r03 lockert kein Gate. Die neue Funktion und das neue Modul entfallen vollständig.
Die identische Source-Lane-Voice-Wahrheit wird stattdessen in den bereits vor 1793
writer-erreichbaren `apply_visible_rhythm_ledger()`-Owner integriert. Die eigentliche
`<voice>`-Materialisierung verwendet weiterhin ausschließlich den bereits vorhandenen
`ensure_arrangement_child_text_value_authorized`-Pfad der StructuralWriteAuthority.
Damit entsteht kein neuer produktiver Writer-Knoten, kein neuer Caller außerhalb der
StructuralWriteAuthority und keine Repair-/Recovery-Schicht.

## Frische Evidenz

Ausgangspunkt ist Etüde 3, Takt 19, oberes System im autoritativen Stand
v0.10.1792 r01. Die sichtbare Überlagerung der 16tel ist kein reines Beam- oder
MuseScore-Layoutproblem. Bereits im Master-MusicXML existieren zwei durch
`<backup>` getrennte serialisierte Zeitspuren, die beide `voice=1` tragen und sich
im selben Staff zeitlich überlappen.

Im Reproduktionsfall liegen unter anderem gleichzeitig vor:

```text
Lane-Epoche 0: Staff 1 / Voice 1 / F#6  312..336
Lane-Epoche 1: Staff 1 / Voice 1 / C#6  312..336

Lane-Epoche 0: Staff 1 / Voice 1 / B5   336..360
Lane-Epoche 1: Staff 1 / Voice 1 / F#6  336..360
```

Zwei klingende Nicht-Chord-Ereignisse derselben Voice können im selben Staff
nicht gleichzeitig unterschiedliche Tonereignisse derselben seriellen Stimme sein.
Die wiederverwendete Import-Voice-ID ist an dieser Stelle daher keine kanonische
Voice-Identität. Etüde 3 / Takt 19 ist ausschließlich Reproduktionsadresse; es gibt
keinen Projekt-, Takt-, Pitch- oder Registerselektor.

## Producer / Ownership

`arrangement.rs` bewahrt historisch die alte Import-Stimmenachse formal. Dabei wird
bis 1792 nicht zwischen durch `<backup>` getrennten Lane-Epochen unterschieden.
Spätere Beam-/Polish-Owner gruppieren korrekt nach der gelieferten Voice-ID und machen
die bereits falsche Identität nur sichtbar.

Die fachliche Wahrheit lautet:

```text
ImportedVoiceId ist innerhalb einer Lane-Epoche stabil.

Verschiedene Lane-Epochen dürfen dieselbe ImportedVoiceId behalten,
solange keine sichtbaren klingenden Nicht-Chord-Ereignisse derselben Voice
im selben Staff zeitlich überlappen.

Bei positiver Überdeckung ist die Voice-Identität bewiesen verschieden.
```

## DecisionScope und Materialisierung

`lane_epoch` wird direkt aus der serialisierten `<backup>`-Struktur des aktuellen
Measure-Artefakts abgeleitet. Eine Kollision benötigt gleichzeitig:

- dieselbe explizite Voice-ID,
- verschiedene Lane-Epochen,
- denselben Staff,
- sichtbare klingende Nicht-Chord-Ereignisse und
- positive zeitliche Überdeckung.

Bei einer bewiesenen Kollision behält die frühere Lane ihre Voice-ID. Die spätere
`(lane_epoch, imported_voice)`-Lane erhält deterministisch die nächste freie numerische
Voice-ID. Alle Note-Blöcke derselben Lane/Voice-Kombination werden konsistent umgebunden,
auch Rest-/Chord-Carrier, damit die Lane intern nicht auseinanderfällt.

Der WriteFootprint bleibt auf die bereits vorhandene autorisierte Child-Text-Mutation
begrenzt:

```text
geändert:    <voice>...</voice>
unverändert: Pitch
             Onset
             Duration
             Staff
             Rest/Chord-Rolle
             <backup>/<forward>-Timeline
             bestehende Stem-Werte
```

Wenn ein betroffener Note-Block keine explizite passende Voice besitzt oder die
autorisierte Voice-Materialisierung nicht exakt gelingt, wird die gesamte lokale
Canonicalization verworfen. Nach dem Kandidaten wird dieselbe strukturelle Domain
read-only erneut geprüft; bleibt eine gleiche-Voice-Überlappung zwischen verschiedenen
Lane-Epochen bestehen, wird der Kandidat ebenfalls nicht übernommen.

## Erwartung am Reproduktionsfall

Etüde 3, Takt 19 besitzt bereits Voice `1` und `2`. Für die kollidierende zweite
`voice=1`-Lane ist deshalb Voice `3` zu erwarten. Die erste Lane bleibt Voice `1`.
Beam- und Stem-Owner werden nicht verändert; sie erhalten nur die korrigierte
Voice-Wahrheit.

## Security-Beweisform

r03 führt keinen neuen writer-erreichbaren Funktionsnamen ein.
`apply_visible_rhythm_ledger()` war bereits im 1792-WriterReachability-Graphen, und
der verwendete Child-Text-Writer der StructuralWriteAuthority war von diesem Owner
bereits transitiv erreichbar. Das 1790-Gate gegen neue produktive Repair-/Recovery-
Authorities bleibt daher unverändert und soll diesen Kandidaten nur dann zulassen,
wenn der bestehende Graph tatsächlich nicht erweitert wird.

## Abnahme

```powershell
.\run.cmd --take --strict --writer-trace=19 "Etüde Nr. 3"
```

Erwartet:

```text
Semantic-Capability-Diff green
keine gleiche Voice-ID für die kollidierenden Lane-Epochen in Takt 19
Pitch-/Onset-/Duration-/Staff-Multiset stabil
keine neue Repair-/Recovery-Authority
Security 18/18 weiterhin blocked
```

Erst danach folgt die visuelle Einzelprüfung und anschließend der Vollkorpus.
