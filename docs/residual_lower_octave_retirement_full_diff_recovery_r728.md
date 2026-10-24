# r728 ResidualLowerOctaveRetirementFullDiffRecovery

## Ausgang

r725 `RetiredObservationProjectionHarvestClippyClosure` ist der letzte autoritative dunkelgruene Parent.
r726 wurde installiert, kompilierte und bestand Strict-Clippy, scheiterte aber im normalen Lauf mit
`FinalExit 1`, weil die bereits pensionierte Reportsektion
`RESIDUAL-LOWER-OCTAVE-CONTEXT-AUDIT-NO-WRITE` im `FinalReportContract` weiterhin exakt einmal
verlangt wurde. Der Lauf wurde auf r725 zurueckgerollt.

r727 sollte diese Vertragskante schliessen, wurde aber bereits im Take-Change-Control vor jeder
Installation abgewiesen: Gegen den realen r725-Arbeitsbaum waren zusaetzlich die fuenf r726-Pfade
Teil des tatsaechlichen Payload-Diffs, aber nicht deklariert. r727 ist deshalb kein Parent und wird
nicht wiederverwendet.

r728 rekonstruiert Wave 2 vollstaendig direkt auf r725 und deklariert den gesamten realen Diff in
einem einzigen unabhaengigen `RefactorOnly`-Take.

## Vollstaendig pensionierte Capability

Pensioniert wird die geschlossene `ResidualLowerOctaveContextAudit`-Familie einschliesslich ihrer
letzten exakten FinalReport-Pflichtkante:

- Audittyp und Lane-Event-Hilfstyp,
- XML-/Boundary-/Register-/VoiceRun-Klassifikationshelfer,
- die ausschliesslich zu dieser Auditfamilie gehoerenden Unit-Tests,
- Stage-Erzeugung und Release-Transport,
- beide Reportprojektionen,
- die Pflichtsektion `RESIDUAL-LOWER-OCTAVE-CONTEXT-AUDIT-NO-WRITE` in
  `REQUIRED_FINAL_REPORT_SECTIONS`.

## Explizit erhalten

Unveraendert bleiben insbesondere:

- produktive Register-/Oktavierungs- und Clef-Entscheidungen,
- StructuralWriteAuthority und alle produktiven Writer,
- TieTail-/Beam-/Rest-/Staff-/Voice-Gegenbeweise,
- Renderer-/PDF-/MusicXML-Senken,
- RunOutcome-/Release-Logik,
- Recovery, Change-Control und Semantic-Capability-Diff,
- die typisierte FinalReport-Ledger-/Finalization-Architektur.

Weiterhin exakt erforderlich bleiben die aktiven FinalReport-Vertraege fuer
`TYPED-LOWER-CLEF-SURFACE-MATERIALIZATION`, `SILENT-VOICE-COVERAGE-WRITE-OUTCOME` und den
RendererOutcome-Vertrag.

## Change-Control-Recovery

Der Kandidat wird gegen den realen r725-Baum bewertet. Alle zehn Payload-Aenderungen werden
vollstaendig deklariert. Es gibt keine versteckte Uebernahme eines roten oder abgewiesenen
Zwischenstands.

## Zeilenbilanz

Physische Rust-Zeilen `src/ + tests/ + tools/`:

- r725: 202.628
- r728: 201.856
- vollstaendiger Wave-2-Retirement gegen r725: **-772**
- kumuliert seit der 203.261-Baseline: **-1.405 / -60.000**
