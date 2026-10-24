# r351 KeySignatureEnharmonicSpanLegacyAuthorityEvidence

Parent: r350 `LatePlayabilityBestCandidateSelectionSharedDecisionOwnerClosure`

ChangeClass: `AuditOnly`

## Ausgangspunkt

Der reale r350-Lauf ist GREEN und released:

- `FinalExit 0`
- Build/Clippy/Launcher GREEN
- 1092/1092 integrierte Tests
- Etuede Nr. 6 bleibt 0/0/0
- Master- und Render-MusicXML sind gegen r349 byteidentisch
- alle sechs PDF-Seiten sind pixelidentisch

Damit ist die LatePlayability-BestCandidate-Ownership geschlossen.

## Naechste Insel

Der UnifiedDomainOwnership-Bestand zeigt fuer `Tonal` weiterhin keinen expliziten
OwnerPattern. Der Quellbefund trennt jedoch zwei sehr verschiedene Situationen:

1. `KeySignatureEnharmonicSpelling` ist eine aktive tonale Entscheidungs- und
   Materialisierungsgrenze. Sie entscheidet scorewide oder fuer die finale Sektion,
   materialisiert Key-Signature plus funktionale Enharmonik im Render und spiegelt den
   committed Key-Vektor danach invariant in den Master.
2. `KeySignatureEnharmonicSpan` liegt seriell danach und besitzt noch eine eigene
   StructuralWriteAuthority-Capability, kann diese aber produktiv nicht oeffnen.

Der entscheidende statische Befund im spaeten Span-Pfad lautet:

`behind_bars_gate_open: false`

Daraus folgt fuer den bestehenden Code:

- `attempted = behind_bars_gate_open && target_bound`
- `attempted` ist damit immer `false`
- die Funktion kehrt vor jeder Candidate-Materialisierung zurueck
- `written` bleibt `false`
- der autorisierte Span-Checkpoint kann im produktiven Pfad nicht erreicht werden

Der reale r350-Bericht bestaetigt dieselbe Wahrheit:

- `KeySignatureEnharmonicSpanRenderWrite`: Attempted nein, Written nein,
  RenderXmlWrite nein
- `TonalSegmentationEvidenceAudit`: WriterOpen nein

## Interpretation

Das ist keine zweite **lebende** tonale DecisionAuthority mehr. Es ist eine alte,
statisch geschlossene Schreib-Authority, die noch produktiven Code und eine
StructuralWriteAuthority-Capability belegt, obwohl der Pfad nur noch Evidenz liefert.

r351 aendert diese Architektur noch nicht. Der Take projiziert ausschliesslich den
Befund unter der bestehenden MutationOwnershipEvidenceBoundary.

Zielzustand fuer einen Folgetake:

- aktive tonale Entscheidung bleibt unveraendert
- Render-Key/FunctionalRespelling bleibt unveraendert
- CanonicalMasterMirror bleibt unveraendert
- Span-Consolidation/TonalSegmentation bleiben read-only Evidenz
- die tote spaete Span-Write-Capability und ihr produktiver Sink werden pensioniert

## Nicht geaendert

- kein produktiver Rust-Pfad
- keine MusicXML-/Render-/PDF-Transformation
- keine Passorder
- keine Schwelle
- keine Tonal-Regel
- keine neue DecisionAuthority
- kein neuer Writer
- keine Security-/Gate-Lockerung
- Beam, RestSpan, CrossStaff, HiddenChordToneRecovery, RawCursor und
  LatePlayabilityBestCandidateSelection bleiben geschlossen

`ProductiveCutAllowed` wird nur fuer die enge Pensionierung der spaeten
Span-Write-Capability gesetzt. Der aktive Tonal-Owner wird in r351 nicht umgebaut.
