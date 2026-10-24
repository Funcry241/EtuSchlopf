# r541 – FinalCompleteBeatPrimaryBeam StructuralWriteAuthority Relocation Strict-Clippy Closure

## Ausgangspunkt

Autoritative Basis ist `v0.10.1839 r539 FinalNotationLowerStaffClefPrimaryBeamIdempotenceClosure`.
Der Kandidat r540 wurde nicht installiert: Strict Clippy stoppte ihn wegen zweier nach der geplanten Writer-Relocation verwaister privater Helper in `composer_beam_candidate_materialization_core.rs`.

r541 wird unabhaengig frisch von r539 aufgebaut. Er uebernimmt denselben reinen Authority-Refactor und schliesst ausschliesslich die dadurch sichtbar gewordene Dead-Code-Schuld.

## Ziel

Der historische LowerDualEdge-Recovery-Anteil bleibt read-only. Die bereits bewiesene `FinalCompleteBeatPrimaryBeamContinuity`-Materialisierung wird ohne Musikregel-Aenderung aus dem grandfathered `...repair`-Writerroot in `StructuralWriteAuthority` verlagert.

Dabei gilt:

```text
Pipeline / historischer LowerDualEdge-Owner
  -> LowerDualEdge read-only Regressionsevidenz
  -> FinalCompleteBeat Auswahl und source-bound Edit-Plan

StructuralWriteAuthority
  -> Bindung an den exakten Quell-Snapshot
  -> Beam-Level-1-Materialisierung
  -> Beam-only / Parse-Invarianten
```

Der produktive Funktionsroot `apply_lower_dual_edge_beat_beam_continuity_repair` wird entfernt. Der historische LowerDualEdge-Detektor und seine Regressionsevidenz bleiben erhalten.

## Strict-Clippy-Closure

Durch die Relocation hatte `partial_beat_replace_primary_beam_state` keinen produktiven oder Test-Aufrufer mehr. Der Helper wird deshalb entfernt.

`partial_beat_non_beam_compact` wird nach der Relocation nur noch von bestehenden Regressionstests benoetigt. Er wird deshalb mit `#[cfg(test)]` ausschliesslich in Test-Builds kompiliert. Es wird kein `#[allow(dead_code)]` eingefuehrt.

## Nicht enthalten

- keine neue Musikregel;
- keine Aenderung der T25/T26-Clef-Grenzenlogik aus r539;
- keine neue Beam-Gewichtung, Schwelle oder Pass-Reihenfolge;
- keine Pitch-, Onset-, Dauer-, Staff-, Voice-, Clef- oder Octave-Semantik;
- keine neue Repair-/Recovery-Autoritaet;
- keine Gate-, Security- oder Kodex-Aenderung;
- keine Dependency-Aenderung und kein Root-`Cargo.lock`;
- keine Entfernung der read-only LowerDualEdge-Regressionsevidenz;
- kein Lint-Suppressionsattribut.

## Invarianten

Gegen r539 muessen fuer den produktiven Lauf insbesondere gelten:

```text
Master-MusicXML                          byteidentisch
Render-MusicXML                          byteidentisch
PDF                                      visuell identisch
CompleteBeatImmediateIdempotenceReadback ja
ExactFinalReadback                       ja
LowerDualEdge RecoveryEnabled            nein
LowerDualEdge RepairedNotes              0
LowerDualEdge BeamTagsReplaced           0
Warnings                                 0
```

## Abnahme

Zuerst Einzelreexec von Etuede 8. Nur bei gruenem Build, Tests, Kodex und Strict Clippy sowie unveraenderten Musikartefakten folgt der 10er-Vollkorpus.
