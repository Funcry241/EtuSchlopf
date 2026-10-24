# FinalNotationLayoutSnapshotClosure

## Zweck

Take 2 der horizontalen Layoutmigration bindet den finalen Notationszustand, ohne die bestehende horizontale Layoutpolicy zu veraendern.

Der bisherige Layoutpass laeuft weiterhin unveraendert an seiner alten Stelle. Dieser Take verschiebt keinen System- oder Seitenumbruch, aendert keine `measure/@width`, keine Score-/Threshold-Policy und keinen Layoutwriter.

## Revisionsbefunde r01 bis r04

- `r01` scheiterte ausschliesslich an unzulaessigen inneren `//!`-Kommentaren in einer per `include!` eingebundenen Datei (`E0753`).
- `r02` kompilierte, erreichte den neuen Snapshot-Closure und bewies im realen Windows-Lauf einen bislang verdeckten letzten MusicXML-Writer: die bereits vorhandene `FinalRenderCommittedStateClosure` materialisierte die ausgewaehlte Render-Quelle erst innerhalb von `render_pdf_if_possible()` und damit nach dem zu fruehen `SnapshotClose`.
- `r03` versuchte diesen bestehenden Commit vor den Snapshot zu ziehen, fuehrte dafuer aber eine neue `run_pdf_render_stage`-Aufrufschicht und eine neue Writer-Reachability ausserhalb `StructuralWriteAuthority` ein. Der bestehende `NO-NEW-REPAIR-AUTHORITY`-Gate wies den Kandidaten deshalb korrekt **vor der Installation** ab.
- `r04` schwaecht kein Gate ab und baut nicht auf der abgewiesenen Writer-Schicht weiter. Der Schnitt wird vom letzten gruenen `0.10.1823 r03` neu aufgebaut. Der historische Release-Owner `run_render_pdf_source_selection_stage` bleibt der einzige Orchestrator dieser Strecke. Die bereits vorhandene finale Render-Source-Materialisierung erhaelt lediglich eine source-bound Aufrufgrenze in `src/structural_write_authority/final_pdf_source_commit.rs`. Danach wird der read-only Snapshot geschlossen und derselbe bestehende Renderer gestartet.

Damit lautet die Architekturfolge:

```text
bestehende PDF-Quellenauswahl
        |
        v
FinalPdfSourceCommitAuthority
(source-bound; bestehende FinalRenderCommittedStateClosure)
        |
        v
FinalNotationLayoutSnapshotSet
        |
        v
SnapshotClose
        |
        v
bestehender MuseScore-Renderer
        |
        v
Exact-Readback gegen denselben Snapshot
```

Es entsteht keine neue spaete Writer-Aufrufschicht ausserhalb `StructuralWriteAuthority`.

## Verhaltenserhalt

Die fachliche Finalisierung selbst bleibt dieselbe. `SilentVoiceVisibleMeasureCoverage`, OrphanChordHost-Kanonisierung, Clef-Fixpunkt, RestPlacement und der vorhandene `FinalNotation`-Commit werden nicht neu erfunden, nicht erweitert und nicht neu gewichtet.

Die bestehende Funktion `run_render_pdf_source_selection_stage` behaelt Quellenauswahl, Evidence-Ledger, RenderWritePdfSourceAdoptionTruthGate, PDF-Progress und Renderer-Aufruf. Es gibt keinen neuen `run_pdf_render_stage`-Owner.

`render_pdf_if_possible()` bekommt nur den bereits geschlossenen, source-bound `FinalPdfSourceCommit` und besitzt fuer diesen Aufruf kein eigenes finales MusicXML-Materialisierungsrecht mehr. Sein vorhandener bytegenauer Vorher-/Nachher-Guard um MuseScore bleibt erhalten.

Auch die bisherige Bedingung bleibt bestehen: Die finale Pre-Renderer-Materialisierung wird nur ausgefuehrt, wenn der Lauf ohne finale Blocker tatsaechlich PDF rendert und `--no-pdf` nicht gesetzt ist.

## Snapshot-Vertrag

`FinalNotationLayoutArtifactSnapshot` enthaelt nur:

- exakte XML-Bytes als immutable `String`
- einen stabilen FNV-1a-64-Fingerprint dieser Bytes

`FinalNotationLayoutSnapshotSet` enthaelt:

- einen materialisierten Master-Snapshot
- optional einen Render-Snapshot

Autoritativ ist die durch die bestehende PDF-Quellenauswahl gewaehlte Quelle. Keine Projektidentitaet, keine Etuedenidentitaet, keine Taktidentitaet, keine Layoutentscheidung und kein Writer-Token werden in den Snapshot-Typen gespeichert.

Unmittelbar nach `SnapshotClose` und erneut nach MuseScore muss die ausgewaehlte MusicXML-Quelle bytegenau plus Fingerprint mit dem gebundenen Snapshot uebereinstimmen. Jede spaetere MusicXML-Mutation wird fail-closed erkannt.

## Noch nicht Bestandteil dieses Takes

- keine neue `HorizontalLayoutPlanningAuthority`
- keine Verlagerung der heutigen horizontalen Layoutpasses
- kein globaler Reflow
- keine neue Page-Policy
- keine Width-Demotion
- keine Aenderung der PDF-Quellenauswahl
- kein neuer Repair-/Recovery-Pass
- keine neue Musikregel oder Schwelle

## Abnahme

Der normale Take-Lauf muss GREEN bleiben. Fuer Etuede 6 sollen Master-/Render-MusicXML gegen den letzten gruenen v0.10.1823-Stand bis auf unvermeidbare Versionsmetadaten identisch bleiben. Besonders wichtig sind:

```text
NO-NEW-REPAIR-AUTHORITY SECURITY GATE: kein neuer Befund
FinalRenderCommittedStateClosure ... PreRenderFinalization GREEN
RendererMusicXmlWrites 0
FinalNotationLayoutSnapshotClosure: kein Mismatch
```

Ein-Befehl-Regel:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```
