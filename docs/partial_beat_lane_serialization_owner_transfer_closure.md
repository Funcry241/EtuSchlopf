# PartialBeatLaneSerializationOwnerTransferClosure

## Version

`v0.10.1781 r02`

## Autoritative Basis

`v0.10.1780 r01 FinalBeamLostBeginRecoveryAuthorityRetirement`, 10/10 dark_green.

## r01-Ablehnung und Capability-Grenze

Der erste 1781-Kandidat wurde vom Semantic-Capability-Diff vor Apply abgewiesen.
Er hatte die Fachlogik direkt in `run_initial_render_transaction_stage()` eingehangen.
Diese Funktion ist wegen Proof-/Report-Dateireadbacks ein geschuetzter
`ProofReentryReader`. Zugleich wurde der neue `apply_*`-Helper als produktiver
Writer-Root erkannt; dadurch aenderte sich die vom Audit-Orchestrator erreichbare
Writer-Menge (`WriterReachability`). Eine MusicMutation darf diese Cross-Role-Grenzen
nicht veraendern.

r02 umgeht die Policy nicht. Die geschuetzte Funktion bleibt byteidentisch zu 1780.
Der Owner-Transfer wird in die bereits vorhandene Cursor-Authority
`repair_cursor_lane_end_invariant_xml()` integriert. Der interne Transfer-Helper ist
kein neuer Writer-Root; die bekannte Cursor-Authority bleibt der produktive Owner.

## Writer-Trace-Beweis

Der report-only Lauf

```powershell
.\run.cmd --strict --writer-trace=27 "Etüde Nr. 6"
```

zeigt fuer den einzigen produktiven historischen PartialBeat-FinalRepair-Fall:

```text
vor fruehem FinalNotation:
    drei 16tel musikalisch vorhanden, dritte Note physisch in anderer Lane-Serialisierung

fruehes FinalNotation / Rest-Completion:
    sichtbare Post-Rest-Gestalt; erste zwei Noten als Zweiergruppe

CursorLaneEndInvariantRepair:
    logischer Onset der dritten Note korrekt, physischer Forward/Backup-Split bleibt

spaete Zwischenwriter:
    kein geschlossener Stitch wird zerstoert

PartialBeatPostRestTripleBeamFinalRepair:
    Forward(24) mit dritter Note tauschen und zusaetzlich Beam 1+2 schreiben
```

Die historische CauseTrace-Zahl `before 1 -> after_polish 0 -> final 1` war damit
zeitlich zu grob gebunden. Der Kern ist Lane-Serialisierung, nicht ein spaeter
Beamverlust.

## TruthDomain-Trennung

```text
LaneSerializationTruth
    Owner: bestehende Render-Cursor-/Lane-Authority
    Operation: cursorneutraler Forward/Note-Swap

BeamTruth
    Owner: allgemeiner BeamPlan-Core
    Operation: partial_beat_extend_states_from_plan
```

Der Transfer wird nur fuer `RenderXML` ausgefuehrt. `MasterXML` bleibt von dieser
Normalisierung unberuehrt.

## Score-lokale Singularitaet

Der historische Human-Bound-Vertrag bleibt erhalten: Ueber den ganzen Score muss
genau ein Lane-Stitch-Kandidat existieren. 0 oder >1 Kandidaten erzeugen kein
Schreibrecht. Es gibt keinen Etueden- oder Taktlocator als produktiven Selektor.

## Cross-Voice-Rest-Vertrag

Der sichtbare Rest darf derselben Staff, aber einer anderen Voice als die drei
Pitch-Noten angehoeren. `RestVoice != NoteVoice` bleibt erlaubt. Die allgemeine
BeamPlan-Erzeugung bindet nur Staff/NoteVoice/Beat und benoetigt die Rest-Voice nicht.

## Fail-Closed Gates

Der Transfer verlangt:

- genau einen score-lokal singulaeren Lane-Stitch-Kandidaten,
- drei sichtbare, nicht-chordale 16tel derselben Staff/NoteVoice,
- passendes Forward mit exakt einer Notendauer,
- mindestens ein Backup zwischen Split und dritter Note,
- unveraenderte Notenzahl,
- identische vollstaendige Note-Event-Multimenge,
- gueltigen Backup/Forward-Cursor vor/nach,
- exakt sechs BeamPlan-Zeilen aus dem allgemeinen Core,
- drei unmittelbar serialisierte Zielnoten,
- keinen Forward/Backup zwischen ihnen,
- exakten Beam-Level-1+2-Readback.

Bei jedem roten Gate wird der Eingang unveraendert zurueckgegeben.

## Recovery bleibt als Evidence

`PartialBeatPostRestTripleBeamFinalRepair` wird in 1781 nicht entfernt. Erwartung:

```text
1780 Etuede 6: 1 produktiver FinalRepair-Write
1781 Etuede 6: 0 produktive FinalRepair-Writes
```

Erst nach Vollkorpus- und Output-Invarianzbeweis folgt ein separater Retirement-Beweis.

## ChangeClass

`MusicMutation`: Zeitpunkt und Owner der Render-MusicXML-Materialisierung aendern sich
real. Musikregel, Pitch, Onset, Dauer, Staff, Voice, Rest-Timeline, Clef, Layout und
Security bleiben unveraendert.
