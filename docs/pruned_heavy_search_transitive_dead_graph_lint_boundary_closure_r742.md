# r742 – PrunedHeavySearchTransitiveDeadGraphLintBoundaryClosure

## Parent

Autoritativer Parent ist r738 (`SourceSelectionLegacyGuardCompatibilityRetirementWave8`), dunkelgruen und released.

r739 und r740 scheiterten im Strict-Clippy/Compiler. r741 wurde vor Copy von `WARNING-FREE BUILD INTEGRITY` abgelehnt und ist ebenfalls kein Parent.

## Gate-Befund aus r741

Die transitive Dead-Graph-Closure entfernte unter anderem `push_hand_v2_focused_variant`. In r738 war unmittelbar an genau diesem Item das lokale Attribut

```rust
#[allow(clippy::too_many_arguments)]
```

gebunden. Nach Entfernung des Items blieb das Attribut in r741 stehen und band sich syntaktisch an das naechste lebende Item `han_repair_candidate_risk`. Das Admission-Gate erkannte dies korrekt als neue, verschobene oder verbreiterte Lint-Schwaechung und stoppte den Kandidaten vor Copy.

## Closure

r742 behaelt die fachlich unveraenderte Dead-Graph-Pensionierung bei und entfernt das verwaiste `allow` vollstaendig. Es wird kein Lint-Attribut hinzugefuegt, verschoben oder verbreitert.

Die drei bereits deterministisch no-op Heavy-Search-Roots sowie der vom Compiler freigelegte transitive Hand-V2-/Hidden-Chord-Suchgraph bleiben pensioniert. Gemeinsam genutzte XML-Helfer und alle lebenden aktuellen Pfade bleiben erhalten.

Keine Voice-/Tie-/Duration-/Chord-Authority wird umgeschaltet. T3/T16/T22 bleiben unveraenderte Witnesses fuer die spaetere Authority-Arbeit.

## Rust-Bilanz

- r738: 197.984
- r742: 196.238
- Delta: -1.746
- Gegen Baseline 203.261: -7.023 / -60.000
