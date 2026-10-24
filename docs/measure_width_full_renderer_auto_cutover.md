# MeasureWidthFullRendererAutoCutover

## Ziel

Take 15 demotiert nach dem real DARK GREEN bestaetigten Width-Demotion-Shadow die verbleibende
explizite `measure/@width`-Dimension vollstaendig auf RendererAuto.

Der vorherige Audit auf Etuede Nr. 6 meldete 44 strukturelle Slots, 43 explizite Widths und genau
einen bereits automatischen Slot. Die Width-Provenienz bestand ausschliesslich aus generischer
Policy: 38 `DenseRegisterNotation`, 5 `VisualLayoutPressure`, 0 `LegacySeed` und
0 `PianisticReadability`. Damit gibt es in diesem Lauf keine als Exceptional-Width begruendete
explizite Width.

## Promotion

Der bereits autoritative finale Page+System-Plan bleibt die einzige Planwahrheit. Die Promotion
klont diesen Plan und entfernt ausschliesslich:

- `explicit_width`
- `width_owner`

System- und Page-Direktiven sowie deren Owner bleiben unveraendert.

Der bestehende One-Materializer bleibt der einzige horizontale Writer. `None` in der Width-Dimension
bedeutet, dass ein vorhandenes `measure/@width` bei der Materialisierung entfernt wird und MuseScore
die horizontale Taktbreite selbst bestimmt.

## Fail-closed Bedingungen

Die Promotion wird blockiert, wenn:

- der Shadow nicht dieselbe Slotanzahl wie der autoritative Plan bindet,
- `RetainedExceptionalWidths != 0`,
- die generisch demotierbaren Widths nicht alle expliziten Widths abdecken,
- `FullRendererAutoChanges != ExplicitWidths`,
- die tatsaechlich entfernten Widths von der Shadow-Evidence abweichen,
- die Planpromotion einen System- oder Pagebreak veraendert,
- der Readback nach Promotion noch eine explizite Width enthaelt,
- nicht alle Slots nach Promotion RendererAuto sind,
- ein zweiter identischer RendererAuto-Cut noch eine Width-Aenderung finden wuerde.

Keine neue Width-Schwelle, keine Etueden-/Taktlogik, kein zusaetzlicher Writer, keine
Repair-/Recovery-Schicht und keine Security-Abschwaechung.

## Erwartete Evidence fuer den beobachteten Lauf

```text
HorizontalLayoutMeasureWidthFullRendererAutoCutover:
Slots 44 |
WidthChanges 43 |
SystemBreakChanges 0 |
PageBreakChanges 0 |
ExplicitWidthsBefore 43 |
ExplicitWidthsAfter 0 |
RendererAutoSlotsAfter 44 |
PolicyWrite ja
```

Die konkrete PDF-Wirkung ist bewusst **nicht** vorweggenommen; autoritativ ist der echte
MuseScore-/Renderer-Lauf.

ChangeClass: `MusicMutation`.
Parent: `0.10.1836 r02`.
