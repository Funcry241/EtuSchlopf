# HigherBeamRecoveryAuthorityRetirement

## Autoritative Basis

Basis ist `v0.10.1775 r01 BeamDecisionScopeWriteFootprintClosure`, Vollkorpus 10/10 dunkelgruen. Die 1775-Abnahme zeigte:

```text
SourceHigherBeamNotesRestored   20 -> 0
Finalizer Candidates            60 -> 40
BeamLostBeginNotesWritten        0 -> 0
CommittedHigherBeamCandidates    0 -> 0
Master MusicXML                 10/10 invariant
PDF pages                       57/57 pixelidentisch
```

Die Differenz 60 -> 40 entspricht exakt den 20 entfallenen HigherBeam-Recoveries. Daneben bewahrte 1775 neun weitere Level-2-`begin`-Zustaende, die ausserhalb des engen historischen HigherBeam-Selektors lagen. Damit ist die bekannte upstream Ursache nicht bloss maskiert, sondern an ihrem fremden cross-beat Write-Recht entfernt.

## Exakte historische Recovery-Domain H

Der bisherige mastergebundene HigherBeam-Selektor akzeptiert nur eine sehr enge Zustandsform:

- genau vier sichtbare, nicht versteckte, nicht-restende, nicht-chordale Noten,
- gleiche Staff-/Voice-/Beat-Domain,
- jede Dauer exakt ein Viertel der Beat-Dauer,
- lueckenlos vom Beat-Anfang bis Beat-Ende,
- eindeutige Master/Candidate-Identitaet nach Part, Bar, Staff, Voice, Onset, Duration, Pitch und Rest,
- Candidate und Master Level 1: `begin, continue, continue, end`,
- Master Level 2: `begin, continue, continue, end`,
- Candidate Level 2: `none, continue, continue, end`.

Nenne diese Menge `H`. Der produktive Recovery-Writer setzte ausschliesslich am ersten Ereignis das fehlende Level-2-`begin` wieder ein.

## Statischer Writer-Beweis nach 1775

Retirement wird nicht aus `0 Writes` abgeleitet. Geprueft wurde, ob ein verbleibender produktiver Writer aus einem gueltigen Zustand einen Zustand in H erzeugen kann.

### Variant-A

Seit 1775 gilt `WriteSet(VariantA) subset DecisionScope(VariantA)`. Der historische NextBeat-Cleanup ist entfernt. Innerhalb des eigenen Beats erzeugt der vorhandene `BeamPlan` fuer vollstaendige 16tel-Hierarchie geschlossene Level-2-Laeufe. Der einzige nachgewiesene isolierte erste-Note-L2-Verlust ist damit nicht mehr im Bild dieses Operators.

### ScorewideMetricBeamContinuity

Der Writer materialisiert seine gefundenen Gruppen als BeamPlan. Bei 16tel-Hierarchie wird Level 2 gruppenweise mit `begin/continue/end` geschlossen. Er kann daher nicht nur das erste Level-2-`begin` entfernen und die drei Folgezustaende unangetastet lassen.

### ExternalFeedbackClosure / LocalVoice

Beide verwenden typisierte BeamPlan-Konstruktoren mit Level-Kontinuitaet. Eine vollstaendige 4x16tel-Gruppe erzeugt eine geschlossene sekundare Beam-Gruppe. H liegt nicht in ihrer Bildmenge.

### PartialBeatPostRestTripleBeam

Der Operator besitzt eine Dreiergruppe und schreibt fuer seine Zielgruppe Level 1 und Level 2 als geschlossenen Plan. Die Kardinalitaet ist mit H inkompatibel.

### GouldMetricBeam

Der Detektor besitzt exakt das Dauerprofil `16tel - 8tel - 16tel` als Dreierpaket. Der Writer entfernt zwar Beam-Tags und materialisiert danach nur den Primarbalken, aber dieses Target kann keine aus vier gleichen 16teln bestehende H-Gruppe sein. Die Eventform selbst liefert den Ausschlussbeweis.

### StructuralTail

Die feste Tail-Form besitzt `8tel - 16tel - 16tel` plus eigene Cross-Staff/Chord/Hidden/Rest-Rollen. Auch diese Dreierstruktur ist mit den vier identitaetserhaltenen sichtbaren 16teln von H inkompatibel.

### LowerDualEdge

Bei Vierergruppen wird nicht die erste Note als Ziel geschrieben; auf den Zielnoten werden Level 1 und Level 2 gemeinsam aus dem Plan gesetzt. Ein isoliertes Entfernen des ersten Level-2-`begin` ist damit ausgeschlossen.

### FinalCompleteBeatPrimaryBeamContinuity

Dieser Canonicalizer ersetzt ausschliesslich Beam-Level 1 und bewahrt hoehere Beam-Tags. Formal kann er H nicht erzeugen, weil er Level 2 nicht entfernt. Historisch machte er lediglich den Variant-A-Schaden fuer `FinalBeamLostBegin` unsichtbar, indem Level 1 bereits wieder vorhanden war.

### FinalBeamLostBegin

Der Writer ist additiv/mastergebunden und fuegt einen fehlenden Begin hinzu. Er entfernt keine Beam-Level und kann H nicht erzeugen.

### CrossStaff-/Transfer-/Tie-Merge-Pfade

CrossStaff-Materialisierung schreibt Level 1 und 2 gemeinsam. Staff/Voice-Transfer veraendert Staff/Voice bzw. Chord-Eigenschaften; SamePitch-Merge veraendert Dauer/Kardinalitaet. Damit brechen diese Pfade die fuer H erforderliche exakte Master/Candidate-Eventidentitaet und koennen H nicht erzeugen.

## Retirement-Satz

Fuer die nach 1775 verbleibende produktive Pipeline gilt fuer die eng definierte Recovery-Domain:

```text
Image(P_remaining) intersect H = empty
```

Damit ist die produktive Recovery-Transformation auf allen von der verbleibenden Pipeline erreichbaren Zustaenden identisch zur Identitaet. Das ist der statische Retirement-Beweis; der 10er-Korpus mit 0 Restores liefert die Runtime-Bestaetigung, ist aber nicht dessen alleinige Grundlage.

## Architektur nach dem Retirement

Vorher:

```text
Beam pipeline
  -> committed candidate
  -> HigherBeam recovery planning
  -> pending bind/store
  -> FinalNotation sink mutiert Beam-Level 2
  -> committed readback
```

Ab 1776:

```text
Beam pipeline
  -> committed candidate
  -> FinalNotation commit ohne HigherBeam-Recovery
  -> committed readback
  -> read-only H regression detector
  -> fail closed wenn candidates > 0
```

Die wichtige Grenze lautet damit:

```text
Regression erkannt -> Release blockieren
Regression erkannt -X-> spaet reparieren
```

## Entfernte produktive Authority

Entfernt werden:

- HigherBeam-Pending-Store in der Pipeline,
- Bind-/Take-Reentry der HigherBeam-Edits,
- Recovery-Plan-Uebergabe an den FinalNotation-Sink,
- Beam-only Mutationshelper des Sinks,
- produktive `SourceHigherBeamNotesRestored`-Mutation.

Erhalten bleiben:

- die Reportmetrik `SourceHigherBeamNotesRestored` als stabile Schemaoberflaeche mit Wert 0,
- der exakt mastergebundene historische Selektor als read-only Regression-Detector,
- `CommittedHigherBeamCandidates`,
- `CommittedStateReadbackGreen` als fail-closed Abschluss.

## Erwartete Abnahme

Da die produktive Authority auf dem gesamten 1775-Korpus bereits 0 Writes hatte und statisch auf erreichbaren Zustaenden obsolet ist, ist der Take `RefactorOnly`. Erwartet wird gegen 1775:

```text
10/10 dark_green
SourceHigherBeamNotesRestored = 0
CommittedHigherBeamCandidates = 0
CommittedStateReadbackGreen = ja
Master MusicXML semantisch invariant
Render MusicXML semantisch invariant
57/57 PDF pages pixelidentisch
```

Jedes Auftreten von `CommittedHigherBeamCandidates > 0` ist ab 1776 absichtlich kein Repair-Anlass innerhalb desselben Laufs, sondern ein Regression-Gegenbeispiel gegen den Retirement-Beweis und muss upstream analysiert werden.

## r02 – mechanische Dead-Code-Closure

Der reale r01-Take wurde nach erfolgreicher transaktionaler Installation im Strict-Clippy-Reexec mit Exit 101 abgebrochen und auf 0.10.1775 zurueckgerollt. Ursache war kein Gegenbeispiel gegen den Retirement-Beweis, sondern die konsequente Folge des entfernten produktiven Consumers: `BeamState::from_musicxml_text()` war danach ausschliesslich noch durch seinen eigenen `#[cfg(test)]`-Roundtrip referenziert und im normalen Binary unter `#![deny(dead_code)]` unzulaessig. r02 entfernt Methode und den exklusiven Test statt einen Allow-Bypass einzufuehren.

Der fachliche r01-Vertrag bleibt byte-/semantisch unveraendert; r02 fuegt keine neue Capability hinzu und aendert keine Beam-Entscheidung.
