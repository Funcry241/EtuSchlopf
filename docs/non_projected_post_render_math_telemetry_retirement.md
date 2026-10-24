# NonProjectedPostRenderMathTelemetryRetirement

Stand: v0.10.1590 r04

## Revision r04

`r03` wurde vom Strict-Clippy-Gate vor jeder Installation abgelehnt. Im
`AudioSalienceConsensusMeasureAudit` waren noch vier Werte gespeichert, deren
letzter Consumer mit der nicht projizierten Telemetriewelt entfernt worden war.

r04 entfernt ausschließlich diese redundante Feldspeicherung. Die vier Werte
werden weiterhin lokal aus dem produktiven ReductionRoleConsensus gelesen und
bleiben vollständig wirksam für Klassifikation, Konfliktbehandlung und
`overlap_score`. Nur ihre anschließende zweite Ablage im priorisierten
Salienzobjekt entfällt. `priority_measures`, Sortierung und alle nachfolgenden
Gates bleiben unverändert. `r01`, `r02` und `r03` sind keine gültigen
Ausgangsstände.

## Revision r03

`r02` wurde nach der Cross-File-Closure vom Strict-Clippy-Gate vor jeder
Installation abgelehnt. Übrig geblieben waren eine ungenutzte
`accepted_candidate`-Variantenbindung, das nur redundant gespeicherte Feld
`AudioSalienceMeasureAudit.hidden_notes` und drei nicht mehr konsumierte
Consensus-Listen.

r03 entfernt nicht nur die Felder, sondern auch die ausschließlich zu ihrer
Befüllung ausgeführten Filter-, Klon-, Sortier- und Truncate-Schritte. Die
Hidden-Note-Zahl bleibt Bestandteil des produktiven Audio-Salienz-Scores;
`priority_measures` und alle daraus folgenden produktiven Gates bleiben
unverändert. `r01` und `r02` sind keine gültigen Ausgangsstände.

## Revision r02

`r01` wurde vom Take-CrossFile-Interface-Closure-Gate vor jeder Installation
abgelehnt. `EditorialRoleAudit.done_fragment` war im neuen Struct entfernt,
während `lower_staff_register_feedback_gate_core.rs` noch den gesamten
`EditorialRoleAudit` als Parameter führte. Derselbe breite Vertrag bestand
zwischen `ReductionRoleConsensus` und der produktiven Gestenprobe.

r02 verengt beide Grenzen auf die tatsächlich konsumierten Werte:

- `&[EditorialRoleMeasureAudit]` statt `&EditorialRoleAudit`,
- `usize role_confirmed` statt `&ReductionRoleConsensus`.

Damit sind nicht nur der zuerst gemeldete `done_fragment`-Befund, sondern auch
die sonst anschließend auftauchenden `pipeline_line`-Closure-Befunde geschlossen.
Die Berechnung, Sortierung, Writer-, Gate-, Report- und Musikautorität bleiben
unverändert. `r01` ist kein gültiger Ausgangsstand.

## Ausgangspunkt

Autoritativer Eingang ist der real installierte und grün bestätigte Stand
`v0.10.1589 r02 / Export 529`.

Die Abnahme zeigt:

- Strict-Build und Strict-Clippy erfolgreich,
- Clippy-Warnbudget 0,
- `FinalExit 0`,
- Master- und Render-MusicXML gegenüber v0.10.1588 nur mit geändertem
  Versionsstring,
- sechs PDF-Seiten pixelidentisch.

## Befund

Die produktive Salienz-, Rollen- und Konsenslogik lief bereits vor der
Post-Render-Stufe. Danach wurde dieselbe fachliche Welt erneut ausschließlich
für nicht projizierte `pipeline_lines` aufgebaut:

- ComposerIntentRecoveryAudit,
- zweiter MusicalReductionClassifier,
- zweiter ReductionRoleConsensus,
- MathSearchSpaceAudit,
- HigherMathKernelAudit,
- MathematicalEconomyAudit,
- zweite AudioSalienceConsensus-Projektion,
- AudioSalienceWeightedCostPreview,
- PitchSubstitutionImpactAudit,
- Gödel-Proof-Zeile,
- HotspotTimingAudit.

Diese Ergebnisse hatten kein Writer-, Auswahl-, Blocker-, Release-,
Artefakt- oder typisiertes Finalreport-Recht.

## Retirement

Der Take entfernt die komplette zweite Audit-/Telemetriewelt und die nur dafür
notwendigen Transportfelder, Typen, Builder, Hilfsfunktionen und Timingdaten.

Zusätzlich werden drei Folgeschulden geschlossen:

1. Der MusicalReductionClassifier war nach Entfernung seiner Berichtsausgabe
   nur noch ein zusätzlicher vollständiger Hidden-Chord-Scan. Der aktive
   ReductionRoleConsensus benötigt seine Werte nicht für Sortierung,
   Klassifikation oder Gate-Verhalten und wird nun direkt aufgebaut.
2. AudioSalienceWeightedCostPreview war ein exaktes Derivat der bereits
   vorhandenen `priority_measures`. Sein einziger produktiv erscheinender
   Fallback konnte niemals zusätzliche Takte liefern, weil Klassenlabel und
   Takt eins zu eins aus derselben Liste stammten.
3. `build_editorial_role_audit` duplizierte die bereits vorhandene
   taktweise EditorialRole-Berechnung. Der Aggregate-Owner verwendet nun die
   identische `build_editorial_role_measure_row`-Wahrheit.

## Erhaltene produktive Autoritäten

Unverändert bleiben:

- `rule_math::score_penalty`,
- `musical_reduction_candidate_class`,
- `reduction_role_consensus_class`,
- `reduction_role_candidate_confirmed`,
- der produktive ReductionRoleConsensus und seine Top-Measure-Reihenfolge,
- AudioSalienceConsensus-Klassifikation und Priorisierung,
- SalienceWeightedCandidateGate,
- DiscreteActionSalienceTranslator,
- ConsensusGestureProbe und BrokenGesture-Composer,
- die vier produktiven Kompatibilitätsprädikate
  `math_time_compatible`, `math_staff_compatible`,
  `math_voice_compatible` und `math_rhythm_safe`,
- sämtliche Writer, Validatoren, Passreihenfolgen, Artefaktentscheidungen,
  Blocker-, Release- und Exit-Autoritäten.

## Drei-Achsen-Bilanz

### Schönheit

Post-Render-Verifikation erzählt keine zweite mathematische Welt mehr. Der
Orchestrator transportiert keine ausschließlich für unsichtbare Audittexte
benötigten Objekte mehr.

### Geschwindigkeit

Garantiert entfallen pro Projekt unter anderem:

- die komplette Post-Render-Math-/Gödel-/Economy-Berechnung,
- zwei zusätzliche MusicalReduction-Hidden-Chord-Scans im Standardlauf,
- redundante Candidate-/Slot-Matrix-, Pareto-, Topologie-, Entropie- und
  Gauss-Berechnungen,
- Hotspot-Batch-Timing und seine Stringprojektion,
- die redundante Weighted-Cost-Preview samt Sortierung,
- PitchSubstitution-Kontext- und Umgebungsformatierung.

Ein Sekundenwert wird erst aus dem realen Lauf abgeleitet.

### Komplexität

- eine vollständige Rust-Datei gelöscht,
- keine Ersatzabstraktion,
- kein Adapter,
- kein Shadowpfad,
- weniger Stage-Transportfelder,
- weniger fachliche Zustandsdarstellungen,
- weniger private Audittypen und Hilfsalgorithmen.

## Zeilenbilanz

Physische Rust-Zeilen unter `src`:

- v0.10.1589 r02: 125.492
- v0.10.1590 r04: 123.883
- Netto: -1.609

Der Orchestrator sinkt von 5.157 auf 5.042 Zeilen. Der frühere
`hidden_chord_math_reduction_core.rs` sinkt von 984 auf 310 Zeilen.

## Abnahme

Einmalig:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Kein zweiter Routinelauf und kein `--force`. Ein weiterer Lauf ist nur bei
einer konkreten Output- oder Gate-Abweichung begründet.
