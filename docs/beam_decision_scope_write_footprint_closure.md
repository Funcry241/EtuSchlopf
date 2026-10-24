# BeamDecisionScopeWriteFootprintClosure

## Basis

Autoritative Basis ist `v0.10.1774 r03 AbsorbedPreOwnerWriterRetirementSuperwurf` mit 10/10 dunkelgruenem Vollkorpus. FinalRender committed-state closure, read-only Renderer, Security-Freeze sowie die bereits pensionierten DynamicLowerRestBridge-/ShortRest-Authorities bleiben unveraendert.

## Problem

`ExternalNotationFeedbackBeamVariantA` entscheidet eine strukturelle Beam-Zelle innerhalb eines Beats. Seine Kandidatengruppe wird nach `(staff, voice, beat)` gebildet und durch `beat_group_stays_inside_one_beat()` explizit auf diesen Beat begrenzt. `structural_rest_bridge_beam_states()` erzeugt daraus einen vollstaendigen `BeamPlan`: Level 1 wird innerhalb der Zielgruppe geschlossen, und bei Mixed-Hierarchy wird Level 2 ausschliesslich ueber die kurzen Noten derselben Zielgruppe geschlossen.

Der historische Writer erweiterte diesen Decision-Scope dennoch bei Mixed-Hierarchy um einen fremden Schreibpunkt:

```text
cleanup_start = (target_beat + 1) * beat_unit
first note of following beat
strip every beam tag
```

Formal galt damit:

```text
DecisionScope(VariantA) = beat b
WriteFootprint(VariantA) = beat b U {first note of beat b+1}
WriteFootprint !subset DecisionScope
```

Diese Mutation besass keine eigene Beam-Entscheidung fuer Beat b+1. Sie war historisches Cleanup und damit eine Ownership-Verletzung.

## Last-Writer-Kette

Die produktive Reihenfolge zeigt:

```text
Variant-A
  -> loescht an der ersten Note von b+1 alle Beam-Level

FinalCompleteBeatPrimaryBeamContinuity
  -> materialisiert nur Beam-Level 1
  -> hoehere Beam-Level bleiben bewusst unangetastet

FinalBeamLostBegin
  -> verlangt eine vollstaendig beamlose Zielnote
  -> greift nach dem Level-1-Rebuild deshalb nicht

HigherBeamPreservation
  -> repariert eine enge 4x16tel-Signatur fuer fehlendes Level-2-begin
```

Damit ist die Reihenfolge nicht neutral. Fuer Beam-Level 2 gilt konzeptionell:

```text
CompleteBeat(VariantACleanup(S)) != CompleteBeat(S)
```

Der spaete HigherBeam-Owner ist funktional korrekt, aber teilweise Recovery fuer einen upstream Informationsverlust.

## Korpus-Evidence aus v0.10.1774 r03

Die vorhandenen Reports zeigen korpusweit 20 `HigherBeamPreservation`-Restores. Der Variant-A-Audit projiziert seine historischen Cleanup-Locators nicht in den Ergebnisbericht; daher wird keine konkrete historische Einzelzeile behauptet, die nicht direkt belegt ist.

Eine read-only Rekonstruktion der committed Render-Struktur findet jedoch zusaetzlich neun lower-staff Stellen mit folgender finaler Level-2-Zustandsform:

```text
Render: [none, continue, begin, end]
Master: [begin, continue, continue, end]
```

Ein einfacher Level-2-Zustandsautomat erkennt dort jeweils ein `continue` ohne zuvor offenen Level-2-Beam. Die entsprechenden Master-Stellen besitzen diese Verletzung nicht. Diese neun Faelle liegen ausserhalb des engen HigherBeam-Selektors, weil dessen erwartete Candidate-Signatur `[none, continue, continue, end]` lautet.

Die committed HigherBeam-Postcondition bleibt damit fuer ihre eigene Domain korrekt (`CommittedHigherBeamCandidates = 0`), ist aber keine globale Garantie gegen jede Form von cross-beat Beam-State-Verlust. Das Problem muss upstream am falschen Writer-Recht geschlossen werden, nicht durch Erweiterung des Recoverys.

## Neue Ownership-Regel

Fuer Variant-A gilt ab v0.10.1775:

```text
WriteSet(VariantA) subset DecisionScope(VariantA)
pi_beam(b+1)(VariantA(S)) = pi_beam(b+1)(S)
```

Produktiv werden nur `states`-Zielnoten des im selben Beat erzeugten BeamPlans geschrieben. Die historische `cleanup_indices`-/`cleanup_start`-Logik wird vollstaendig entfernt. Es entsteht kein Ersatzwriter.

## Runtime- und Unit-Beweis

Der Writer fuehrt `WriteFootprintOutsideDecisionScopeRows` und `DecisionScopeWriteFootprintGreen`. Nach jedem Measure-Rewrite werden die Noten erneut strukturell gelesen; jede Nicht-Zielnote wird ordinal gegen ihren Eingangs-Notentext bytegenau verglichen. Schon ein Out-of-Scope-Delta verwirft den gesamten Variant-A-Write fail-closed und liefert das unveraenderte Eingangs-XML zurueck.

Der synthetische Unit-Test `mixed_hierarchy_write_keeps_following_beat_byte_identical` konstruiert:

- Beat b: eighth + 16th + 16th als Mixed-Hierarchy-Kandidat,
- Beat b+1: vollstaendig zweistufig gebalkte 4x16tel-Gruppe.

Er verlangt gleichzeitig:

- Variant-A schreibt den Zielbeat,
- `WriteFootprintOutsideDecisionScopeRows = 0`,
- `DecisionScopeWriteFootprintGreen = ja`,
- der vollstaendige Text von Beat b+1 bleibt byteidentisch,
- der Zielbeat wird mit geschlossenem Level-1-/Level-2-Plan materialisiert.

## Nicht-Ziele

Dieser Take aendert nicht:

- keine Pitch-/Onset-/Duration-Regel,
- keine Staff-/Voice-Zuordnung,
- keine Rest-/Chord-Timeline,
- keine Clef- oder Layout-Regel,
- keine Security-Grenze,
- keine committed-state Renderer-Grenze,
- keinen HigherBeam-Selektor,
- keinen FinalBeamLostBegin-Selektor,
- keine konkrete Etueden-/Taktposition als produktiven Selektor.

## Abnahme

Erwartet wird kein pauschal pixelinvariantes PDF, weil die Entfernung eines falschen Beam-Cleanups bewusst sichtbare Beam-Tags korrigieren kann. Abnahme deshalb fachlich und strukturell:

```text
BuildExit = 0
ClippyExit = 0
ProgramExit = 0
FinalExit = 0
LauncherTests = 128/128
FullCorpus = true
CommittedStateReadbackGreen = ja
CommittedPrimaryClefReentryChanges = 0
CommittedRestPlacementCandidates = 0
CommittedHigherBeamCandidates = 0
CommittedRawCursorGreen = ja
CommittedPitchedEventMultisetStable = ja
RendererMusicXmlWrites = 0
WriteFootprintOutsideDecisionScopeRows = 0
DecisionScopeWriteFootprintGreen = ja
```

Master muss gegen 1774 semantisch invariant bleiben. Render-Deltas duerfen ausschliesslich Beam-Tags betreffen; klingende Timeline und RawCursor muessen invariant sein. Die neue Zahl der HigherBeam-Restores ist Evidence, kein Zielwert.
