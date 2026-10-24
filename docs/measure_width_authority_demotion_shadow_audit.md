# MeasureWidthAuthorityDemotionShadowAudit

## Zweck

Dieser Take ist ein read-only Audit der nach dem globalen Page+System-Reflow noch verbliebenen `measure/@width`-Autoritaet. Er veraendert weder den autoritativen Plan noch MusicXML oder PDF.

## Owner-Inventur

Jede explizite Width muss genau einen bestehenden Width-Decision-Owner besitzen:

- `LegacySeed` – bereits in den strukturellen Ausgangsdaten vorhandene Width,
- `DenseRegisterNotation` – generische dichte-/registerbasierte Mindestbreite,
- `VisualLayoutPressure` – generische visuelle Druckprojektion,
- `PianisticReadability` – explizite hohe Exceptional-Width fuer wenige besonders belastete Lesbarkeitsfaelle.

Page-/System-Owner duerfen keine Width besitzen; fehlende Provenienz stoppt fail-closed.

## Read-only Demotionsszenarien

1. `InheritedOnly`: nur `LegacySeed`-Widths wuerden entfernt.
2. `GenericPolicy`: `LegacySeed`, `DenseRegisterNotation` und `VisualLayoutPressure` wuerden auf RendererAuto demotiert; `PianisticReadability` bliebe als Exceptional-Width-Klasse erhalten.
3. `FullRendererAuto`: jede explizite Width wuerde entfernt.

Die drei Szenarien werden nur gezaehlt. Es gibt keinen Shadow-Writer, kein alternatives MusicXML und keinen Paired-Render.

## Invarianten

- `PolicyWrite nein`.
- Autoritativer Page-/System-/Width-Plan bleibt unveraendert.
- Keine neuen Width-Schwellen oder Scores.
- Keine Etueden-/Taktidentitaet.
- Keine Repair-/Recovery-Schicht.
- One Materializer bleibt unveraendert.
