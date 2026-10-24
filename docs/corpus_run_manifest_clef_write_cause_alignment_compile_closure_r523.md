# CorpusRunManifestClefWriteCauseAlignmentCompileClosure r523

## Ausgangspunkt

Parent ist der committed Stand `v0.10.1839 r521`
`TakeIdentityLifecycleBoundSourceWitnessPreservationClosure`.

Der frische `--all`-Lauf erzeugt zehn formal erfolgreiche Projektresultate mit
`pipeline_exit_code=0`, vollständigen Outputs und beobachtetem Renderer, endet
aber in der Korpusfinalisierung mit `ProgramExitCode 1`. Alle zehn danach
einzeln ausgeführten Projektläufe enden mit `FinalExit 0`.

Der erste OutcomeProjection-Kandidat r522
`CorpusRunManifestClefWriteCauseAlignment` wurde nach grünem PRE-TAKE-DIFF-GATE
transaktional installiert, scheiterte aber bereits bei der Root-Cargo.lock-
Resolution im `cargo check --package etuschlopf --lib` mit E0425/E0560. Ursache
war eine versehentlich zusätzliche `clef_write_cause`-Initialisierung im
aggregate `CorpusRunManifest`, obwohl diese typisierte Wahrheit nur zum
per-project `CorpusRunProjectEvidence` und zum internen `FinalWriterOutcome`
gehört. Der Bootstrap rollte r522 vollständig auf r521 zurück und persistierte
die Candidate-Ablehnung. r523 ist deshalb eine neue Revision auf r521 und kein
zweiter Payload unter r522.

## Root Cause des Korpusfehlers

Seit `FinalRenderCommittedStateClosureSuperwurf` besitzt die finale Writerzeile
getrennte Clef-Wahrheiten:

- `LedgerBoundedClefBoundariesAccepted` beschreibt aktuelle pre-sink
  Clef-Readback-Kandidaten, die einen Commit anfordern können.
- `LedgerBoundedClefBoundariesWritten` beschreibt die im committed Artefakt
  verbliebenen Ledger-Marker.
- `PrimaryClefProjectionChanged` beschreibt eine aktuelle PrimaryClef-
  Projektion, die ebenfalls einen FinalNotation-Commit anfordern kann.

Der ältere `CorpusRunManifest`-Parser behandelte dagegen weiterhin
`LedgerBoundedClefBoundariesWritten > 0` als produktive Schreibursache.

Etüde Nr. 10 bildet den realen False Positive:

```text
LedgerBoundedClefBoundariesAccepted 0
LedgerBoundedClefBoundariesWritten  1
PrimaryClefProjectionChanged        nein
RenderXmlWrite                      nein
NotationMutation                    nein
```

Der committed Marker ist hier nur persistierte Readback-Evidenz und darf keinen
aktuellen Writer-Commit behaupten.

Die Gegenrichtung bleibt ebenso wichtig: Ein Clef-only-Lauf kann bei positiver
Acceptance oder geänderter PrimaryClef-Projektion schreiben, obwohl im
committed Endzustand kein Ledger-Marker verbleibt. Aktuelle Ursache und
committed Markerbestand dürfen deshalb nicht dieselbe Authority sein.

## Closure

`src/corpus_run_manifest.rs` bildet für die Outcome-Projektion:

```text
clef_write_cause =
    LedgerBoundedClefBoundariesAccepted > 0
    OR
    PrimaryClefProjectionChanged == ja
```

Die produktive Schreibursachenbilanz lautet danach:

```text
routes_written
OR cross_staff_rest_suppressions
OR beam_lost_begin_notes_written
OR source_higher_beam_notes_restored
OR rest_vertical_anchors_written
OR clef_write_cause
```

`ledger_bounded_clef_boundaries_written` bleibt unverändert als committed-state
Readback-Evidenz erhalten und wird nicht mehr als aktuelle Schreibursache
interpretiert.

Der per-project Manifestdatensatz bindet `writer_clef_write_cause`; deshalb
steigt das CorpusRunManifest-Schema von 4 auf 5. Der SelfCheck bleibt
fail-closed und rekonstruiert die Write-Konsistenz aus aktuellen Ursachen.

Das aggregate `CorpusRunManifest` besitzt bewusst **kein** eigenes
`clef_write_cause`-Feld. Es aggregiert wie bisher `write_projects`,
`render_xml_write_committed` und die einzelnen Evidenzzähler aus den
per-project Outcomes. Genau diese Typgrenze war im abgewiesenen r522-Candidate
versehentlich verletzt und ist in r523 geschlossen.

## Regressionen

Drei fokussierte Parserfälle binden beide Richtungen:

- committed Clef-Marker ohne aktuellen Clef-Write bleibt No-Write;
- aktuelle Ledger-Clef-Acceptance ohne andere Writerzähler ist eine echte
  Write-Cause;
- reine `PrimaryClefProjectionChanged`-Änderung ist auch ohne Ledger-Acceptance
  eine echte Write-Cause.

Die vorhandenen Higher-Beam-, Cross-Staff-, Hash-, Full-Corpus- und Manifest-
Roundtrip-Regressionen bleiben aktiv.

## Nicht geändert

- keine MusicXML-, PDF- oder Renderer-Semantik;
- keine Writer-, StructuralWriteAuthority- oder produktive Callgraph-Änderung;
- keine Clef-Auswahl, Clef-Kostenfunktion oder Clef-Materialisierung;
- keine Rest-, Beam-, Register-, Layout- oder Pitch-Entscheidung;
- kein MasterApply;
- kein Neuro-Evidence-Vertrag;
- keine Lockerung des CorpusRunManifest-SelfChecks;
- keine Änderung am r521-Take-Bootstrap oder dessen Source-Witnesses;
- keine Wiederverwendung der abgewiesenen Revision r522.

`ChangeClass=OutcomeProjection`.

## Erwartete Abnahme

Zuerst sparsamer Reexec auf dem schärfsten Gegenbeispiel:

```powershell
./run.cmd --take "Etüde Nr. 10"
```

Erwartet wird insbesondere für Etüde Nr. 10:

```text
writer_outcome_observed = true
render_xml_write = false
ledger_bounded_clef_boundaries_written = 1
writer_clef_write_cause = false
```

Erst nach erfolgreichem Commit von r523 folgt:

```powershell
./run.cmd --all
```

Die bestehende globale `clippy::type_complexity`-Warning im Take-Bootstrap ist
nicht Teil dieses Takes und bleibt getrennte Warning-Debt.
