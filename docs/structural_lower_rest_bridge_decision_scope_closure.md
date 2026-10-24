# StructuralLowerRestBridgeDecisionScopeClosure – v0.10.1784 r01

## Autoritative Basis

Ausgangspunkt ist `v0.10.1783 r02 – PartialBeatPostRestFinalRepairAuthorityRetirement` mit 10/10 `dark_green`, `FinalExit 0`, 20/20 invariantem MusicXML gegen 1782 nach Versionsnormalisierung und 57/57 pixelidentischen PDF-Seiten.

Der neue `--writer-trace=all`-Export von Etuede 6 wird ausschliesslich als Forensik-Evidence verwendet. Es gibt keinen produktiven Etueden-, Takt- oder Pitch-Selektor.

## Erster falscher Writer

Die beiden bekannten LowerDualEdge-Gruppen in Takt 12 liegen in Staff 2 / Voice 2 bei:

```text
0 / 24 / 48 / 72
288 / 312 / 336 / 360
```

Der erste falsche Beam-Zustand entsteht bereits in `ExternalFeedbackClosureRenderWrite` durch die bisherige Auswahl eines lokalen `Note-Rest-Note`-Fensters. Beispiel:

```text
vor ExternalFeedbackClosure:
288  Note  begin / begin
312  Note  continue / ...
336  Rest
360  Note  end / end

lokaler RestBridge-DecisionScope bisher:
312 Note
336 Rest
360 Note

vollstaendiger BeamPlan fuer dieses Dreierfenster:
begin / continue / end
```

Damit wird bei 312 aus fachlich `continue` ein zweites `begin`. Die gleiche Klasse kann am rechten Fensterrand ein verfruehtes `end` erzeugen.

Formal:

```text
RestBridgeTriggerScope  subset  ActualContiguousBeamGroup
MaterializationScope   =       RestBridgeTriggerScope
```

Der Trigger war damit kleiner als die Wahrheit, die der Writer materialisiert.

## Korpuslokale Gegenbeispiele in Etuede 6

Die 14 vorhandenen Lower-RestBridge-Zellen wurden read-only gegen den finalen 1783-r02-Render rekonstruiert. Fuenf Gruppen zeigen denselben abgeschnittenen Scope:

```text
Takt 10:  0 / 24 / 48 / 72      -> Offset 24 begin -> continue
Takt 12:  0 / 24 / 48 / 72      -> Offset 24 begin -> continue
Takt 12: 288 /312 /336 /360     -> Offset 312 begin -> continue
Takt 30: 288 /312 /336 /360     -> Offset 336 end   -> continue
Takt 37: 96 /120 /144 /168      -> Offset 120 begin -> continue
```

Jede Korrektur betrifft Beam-Level 1 und 2. Die erwartete direkte Render-Wirkung sind damit zehn Beam-Tag-Zustandsaenderungen, ohne Pitch-/Timeline-Aenderung.

## Neue DecisionScope-Regel

Die RestBridge bleibt nur der Erkennungs-Trigger. Fuer jede Staff-2/Voice-Gruppe werden kurze sichtbare Nicht-Chord-Ereignisse weiterhin unter denselben Dauerbedingungen betrachtet. Innerhalb eines Beats werden maximale kontiguierliche Runs gebildet. Ein Run wird nur dann vom RestBridge-Owner uebernommen, wenn er mindestens ein lokales `Note-Rest-Note`-Muster enthaelt.

Vor der Materialisierung wird der Run auf seine erste und letzte klingende Zeile begrenzt:

```text
fuehrende Rest ohne linken Klanganker     -> nicht absorbieren
nachlaufende Rest ohne rechten Klanganker -> nicht absorbieren
interne RestBridge                          -> Teil des vollstaendigen Runs
kontiguierliche klingende Randnote          -> Teil des DecisionScopes
```

Damit gilt fuer tatsaechlich uebernommene RestBridge-Gruppen:

```text
MaterializationScope = ContiguousAnchoredDecisionScope
```

Der vorhandene allgemeine `BeamPlan` bleibt unveraendert und materialisiert den geschlossenen Run. Es entsteht kein neuer Writer, kein Repair und keine neue Authority.

## Noninterference

Unveraendert bleiben:

- Pitch, Onset und Dauer,
- Staff und Voice,
- Chord-/Rest-Timeline,
- RawCursor,
- Clef- und Layout-Policy,
- Passreihenfolge,
- ExternalFeedbackClosure-Checkpoint-Authority,
- FinalRender committed-state Grenze,
- Renderer read-only,
- LowerDualEdge-Implementierung selbst.

Die ChangeClass ist `MusicMutation`, weil der Take bewusst vorhandene falsche Render-Beam-Tags korrigiert. Master-MusicXML soll invariant bleiben.

## Synthetische Regressionstests

Drei Unit-Tests pruefen direkt die Gruppenauswahl:

1. `Note / Note / Rest / Note` wird als Vierergruppe geschlossen statt als abgeschnittenes Dreierfenster.
2. `Note / Rest / Note / Note` wird ebenfalls als Vierergruppe geschlossen.
3. `Note / Rest / Note / Rest` bleibt auf `Note / Rest / Note` begrenzt; die nachlaufende Pause ohne klingenden Gegenanker wird nicht absorbiert.

## Erste Zielsystemabnahme

```powershell
.\run.cmd --take --strict --writer-trace=all "Etüde Nr. 6"
```

Erwartet:

```text
BuildExit 0
ClippyExit 0
ProgramExit 0
FinalExit 0

ExternalFeedbackClosure LowerRestBridgeCells weiterhin strukturell plausibel
LowerDualEdge Takt-12-Domain 2 -> 0
CommittedStateReadbackGreen ja
CommittedBeamLostBeginCandidates 0
CommittedHigherBeamCandidates 0
CommittedPartialBeatLaneSplitCandidates 0
CommittedRestPlacementCandidates 0
CommittedRawCursorGreen ja
CommittedPitchedEventMultisetStable ja
RendererMusicXmlWrites 0
```

Master muss gegen 1783 nach Versionsnormalisierung invariant sein. Render-Deltas duerfen nur Beam-Tags betreffen. PDF-Pixelidentitaet ist fuer diesen MusicMutation-Take kein Pflichtziel, weil sichtbar falsche Balkenzustaende bewusst korrigiert werden.

Erst nach erfolgreicher Etuede-6-Abnahme folgt der 10er-Vollkorpus. Ein moegliches spaeteres LowerDualEdge-Retirement ist ausdruecklich nicht Bestandteil dieses Takes.
