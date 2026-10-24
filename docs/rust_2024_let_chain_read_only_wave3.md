# Rust 2024 Let-Chain Read-Only Wave 3

Basis ist der real gruene v0.10.1803-Stand mit 10/10 aktuellem Vollkorpus,
`FinalExit 0` und `RunSeverity dark_green`. Da dieser Vollkorpus unmittelbar vor
diesem Refactor erbracht wurde, wird er fuer diese rein mechanische Welle bewusst
nicht erneut als Erstnachweis verlangt.

Diese Welle ersetzt sechs weitere verschachtelte Bedingungen durch Rust-2024-
Let-Chains. Betroffen sind ausschliesslich nicht gepinnte Read-/Helper-Pfade:

- `notation_endpoint_layout_core.rs`: Range-Merge-Helfer.
- `cross_staff_editorial_voice_math_gate_tail.rs`: zwei reine Scope-Praedikate.
- `notation_endpoint_clef_layout_playability_tail.rs`: WAV-Sample-Lesehilfe,
  Nachbaranker-Lesehilfe und sichtbarer Staff-2-Pitch-Ankerfilter.

Unveraendert bleiben Security-/Build-/Launcher-Pins, Writer, Structural-Write-
Authorities, Solver, Passorder, Kosten, Schwellen, Repair, Recovery, Dependencies
und Toolchain. Die Kurzschluss- und Auswertungsreihenfolge bleibt erhalten.

## Sparsame Abnahme

Erstnachweis:

```powershell
.\run.cmd --take --strict "Etüde Nr. 6"
```

Danach diagnostisch:

```powershell
.\run.cmd --clippy-deep
```

Kein automatischer Vollkorpus fuer diese Welle. Ein neuer `--all`-Nachweis wird
erst nach mehreren mechanischen Wellen oder bei einer produktiven/pipelineweiten
Aenderung wieder verlangt.

Ausgehend von 103 bestaetigten `collapsible_if`-Diagnosen in v0.10.1802 und der
reinen Konsolenaenderung in v0.10.1803 werden fuer diese Welle ungefaehr sieben
wegfallende Diagnosen erwartet; die exakte Zahl bleibt Clippy-autoritativer
Messwert.
