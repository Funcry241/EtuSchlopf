# r967 – ExactMoveExpectedTransitionExperimentRetirement

Parent: `v0.10.1839 r966 ExactMoveBoundExpectedTransition`

ChangeClass: `RefactorOnly`

## Ausgangslage

r966 war der bewusst falsifizierbare zweite Architekturversuch nach r956. Er verschob fuer den bestehenden ExactMove-/Gesture-Pfad die bereits vor dem XML-Readback bestimmbare Attack-Semantik (`onset`, `staff`, `voice`) von einer produktiven XML-Rueckuebernahme zu einem vorab gebundenen Expected-Transition mit anschliessendem XML-Witness.

Der reale r966-Vollkorpuslauf war funktional stark:

- 10/10 Projekte OK;
- 10/10 final-clean;
- Build und Clippy gruen;
- All-Target-Tests gruen;
- Etuede 6: gleicher FinalSolverProfile und gleicher Solver-State-Fingerprint wie r965;
- drei MaterialMutationReceipts bytegleich;
- vierzehn `consensus_relocation_longitudinal_trace`-Zeilen bytegleich;
- Master- und Render-MusicXML bytegleich.

Damit wurde die fachliche Hypothese `Authority -> Witness` fuer ExactMove bestaetigt.

## Ergebnis des vorab definierten Erfolgstests

Der Versuch erreichte jedoch **keinen realen Komplexitaetsabbau**. Gegen r965 wuchsen die beiden produktiv geaenderten Rust-Dateien zusammen um 179 Zeilen:

- `candidate_material_lineage_spine_core.rs`: +155 Zeilen;
- `hidden_chord_reduction_texture_rebuild_tail.rs`: +24 Zeilen.

Damit entspricht r966 dem vorab definierten Ergebnisfall: architektonisch sauber und verhaltensgleich, aber fuer diesen Slice kein belegter Denkraumhebel. Ein weiteres Aufschichten von Expected-Transition-, Receipt- oder Framework-Mechanik waere deshalb gerade **kein** Abbau.

## r967-Cut

r967 pensioniert ausschliesslich den r966-Experimentmechanismus und stellt fuer die beiden betroffenen Produktdateien die bereits gruen bewiesenen r965-Bytes wieder her.

Dadurch verschwinden wieder:

- `CandidateMaterialExactMoveExpectedTransition`;
- die zusaetzlichen Anchor-Span-/Expected-Duration-Felder im ExactMove-Carrier;
- die ExactMove-spezifische Expected-Transition-Map;
- die zusaetzliche Witness/Expected-Sonderbehandlung in der generischen Reconciliation;
- die vergroesserten Move-Tupel im Gesture-/Gauss-Pfad.

Es entstehen keine Ersatzschicht, kein neues Transition-Framework, kein neuer Carrier, kein neues Ledger und keine neue Registry.

## Bewusst erhalten

r967 veraendert nicht:

- Anchor Selection;
- `anchor.start_tick`;
- BrokenGesture-/Gauss-/GaussChain-/ConstrainedTail-Scoring;
- Candidate Ranking oder Acceptance;
- Writer-/StructuralWriteAuthority-Primitiven;
- AIC-1, AIF-1 oder CMS-1;
- Architecture Essence;
- den r966-Dokumentationsnachweis als historische Experiment-Evidence.

Die Erkenntnis aus r966 bleibt damit erhalten, ohne den nicht lohnenden produktiven Mechanismus dauerhaft mitzuschleppen.

## Erwartete Bilanz

Produktive Rust-Bilanz gegen r966:

```text
candidate_material_lineage_spine_core.rs          -155
hidden_chord_reduction_texture_rebuild_tail.rs     -24
------------------------------------------------------
Netto                                                -179 Rust-Zeilen
```

Der Take ist erfolgreich, wenn der r966-Vollkorpuszustand nach der Pensionierung weiterhin funktional reproduziert wird. Bei unerwartetem Candidate- oder Musikdelta wird **keine Take-Serie** angeschlossen, sondern die Experimentannahme erneut bewertet.
