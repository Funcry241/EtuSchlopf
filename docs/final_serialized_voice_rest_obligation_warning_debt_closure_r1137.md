# r1137 — FinalSerializedVoiceRestObligationWarningDebtClosure

Parent: accepted r1136 `FinalSerializedVoiceRestObligationProjectionIdentityTransportClosure`.

ChangeClass: `RefactorOnly`.

Warning debt closure: `closed_current_warning_debt_v1`.

## Anlass

r1136 wurde akzeptiert und der Etuede-6-Lauf endete mit `FinalExit 0`, Renderer/XSD gruen und
`Blocker 0 | Warnungen 0 | Hinweise 0`. Die typed RunOutcome-Projektion blieb ausschliesslich
wegen zwei Clippy-Warnungen gelb:

- `clippy::collapsible_if` in der lokalen Intervall-Merge-Schleife;
- `clippy::type_complexity` auf dem privaten Rueckgabetyp der pitched-timeline Signatur.

## Mechanische Closure

Die verschachtelte `if let`-Bedingung wird als aequivalente let-chain formuliert. Reihenfolge,
Vergleich `start <= last.1`, Max-Bildung und `continue` bleiben unveraendert.

Der private Tuple-Rueckgabetyp der pitched-timeline Signatur erhaelt den Alias
`FinalChordPitchedTimelineSignatureRow`. Tuple-Reihenfolge, Feldtypen, erzeugte Werte und
Vergleichssemantik bleiben unveraendert.

Keine Rest-Selektion, kein Staff/Voice-Binding, keine Intervallgrenze, keine Provenienz,
kein Writer, kein XML-Delta und keine Musikregel wird geaendert. Es wird kein Lint
unterdrueckt oder abgeschwaecht.

## Erwartete Abnahme

- Clippy `TotalWarnings: 0`;
- alle bestehenden Tests gruen;
- Etuede 6 weiterhin `FinalExit 0`;
- E6/T40 im Master und Render weiterhin:
  - F#4 `[192,216)`,
  - sichtbare punktierte Achtelpause `[216,288)`,
  - Viertelakkord `[288,384)`;
- typed RunOutcome ohne Warning-Debt gruen.
