# Rust2024LetChainReadOnlySweep

Stand: v0.10.1801 r01

## Ausgangslage

v0.10.1800 r01 hat den ersten echten Rust-2024-Let-Chain-Piloten im installierten
Vollkorpus bestaetigt. Der Lauf endete mit `FinalExit 0`, `RunSeverity dark_green`,
Build/Strict-Clippy gruen und 10/10 frischen Korpusprojekten. Der anschliessende
Deep-Clippy-Lauf meldet die Pilotstelle in `rustig_clef_planning.rs` nicht mehr als
`collapsible_if`.

Damit ist die Sprachfeature-Frage geschlossen. Der naechste Schritt soll Let-Chains
nicht mechanisch ueber den gesamten Quellbaum verteilen, sondern nur dort einsetzen,
wo sie eine bereits vorhandene reine Read-/Audit-Bedingung linear ausdruecken.

## Scope

Dieser `RefactorOnly`-Take linearisiert genau sechs bestehende verschachtelte
Bedingungen in vier nicht gepinnten Read-/Audit-Hilfspfaden:

- `human_pdf_pianistic_quality_core.rs`
  - sichtbare Nicht-Rest-/Nicht-Chord-Note + MIDI-Beobachtung
  - sichtbare Staff-2-Note + MIDI-Beobachtung
- `notation_endpoint_layout_core.rs`
  - sichtbare Staff-2-Pitch-MIDI-Sammlung vor einem lokalen Limit
  - reine Extremregister-Zaehlung
- `report_hygiene_cursor_localbest_trace_tail.rs`
  - Lower-Staff-Treble-Clef-Beobachtung + MIDI-Filter
- `staff_voice_rest_glyph_topology_tail.rs`
  - Clef-Nummer-Match + vorhandenes `<sign>`-Readback

Die jeweiligen Kurzschlussbedingungen, Bindings und Rueckgabewerte bleiben identisch.
Es wird keine neue Bedingung eingefuehrt und keine bestehende Bedingung entfernt.

## Bewusste Nicht-Ziele

Nicht geaendert werden:

- gepinnte Security-/Build-/Launcher-Quellen,
- Trust-Anchor-Fingerprints,
- MusicXML-/PDF-Writer,
- Kandidatenauswahl oder Solver,
- Pass-Reihenfolge,
- Kosten, Schwellen oder Regeln,
- Repair-/Recovery-Authority,
- Dependencies oder Rust-Toolchain,
- Master-/Render-Artefaktsemantik.

Insbesondere werden Clippy-Vorschlaege in `identity_debt_scan.rs`,
`semantic_capability_diff.rs`, `take_change_control.rs`, dem gemeinsamen
`RUST_SECURITY_GATE_CORE` und dem Launcher in diesem Take bewusst nicht angefasst.

## Erwartete Diagnosewirkung

Der bestaetigte Deep-Clippy-Lauf von v0.10.1800 enthaelt fuer die sechs geaenderten
Stellen jeweils genau eine `collapsible_if`-Diagnose. Nach v0.10.1801 werden daher
genau diese sechs Diagnosen nicht mehr erwartet.

Der Deep-Clippy bleibt diagnostisch und non-blocking; aus dem erwarteten Rueckgang
wird kein neues Release-Gate.

## Abnahme

Ein-Befehl-Regel:

```powershell
.\run.cmd --take --strict --all
```

Danach diagnostisch:

```powershell
.\run.cmd --clippy-deep
```

Erwartet werden unveraendert ein vollstaendiger 10/10-Korpus, gruene Strict-Gates und
keine neue Music-/Writer-/Security-Semantik.
