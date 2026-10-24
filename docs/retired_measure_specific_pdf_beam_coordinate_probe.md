# RetiredMeasureSpecificPdfBeamCoordinateProbe

## r02 – Strict-Compile-Abschluss

`r01` wurde vor jeder Installation durch `#![deny(dead_code)]` abgelehnt. Der
entfernte PDF-Koordinatenprobe war der letzte Aufrufer der allgemeinen
Report-Hilfsmethode `push_static_named_report_value_line_v768`. `r02` entfernt
diese nun unbenutzte Traitmethode samt Implementierung. Es entsteht keine neue
Reportform, keine Verhaltensaenderung und keine zusaetzliche Schreibflaeche.

## Zweck

Der historische PDF-Koordinatenprobe fuer die Unterhand-Balkenzelle war fest an
Takt 29, Seite 4 und absolute PDF-Content-Koordinaten gebunden. Er war
audit-only, erzeugte keinen Write, autorisierte keine Regel und blieb im
aktuellen Export 561 erneut ungebunden (`TargetWindowRows 0`).

Der Probe ist damit weder eine allgemeine musikalische Regel noch eine
wiederverwendbare Verifikation. Er konservierte konkrete Etueden-4-Layoutdaten
im produktiven Berichtspfad und las dafuer bei jedem Lauf das komplette PDF,
dekodierte Flate-Streams und klassifizierte Pfadgeometrien.

## Rueckbau

Entfernt werden:

- `PdfBeamShapeReadback`,
- `PdfBeamShapeRow`,
- der Abschnitt
  `LOWER-STAFF-BEAM-CONTINUITY-PDF-COORDINATE-READBACK-NO-MUTATION`,
- die fest verdrahtete Auswahl `TargetMeasure 29`, `Staff 2`, `Voice 2`,
- die Bindung an PDF-Seite 4,
- die absoluten Content-Koordinatenfenster,
- der lokale Flate-/Content-Stream-/Pfadformen-Parser,
- die zugehoerige finale Reportprojektion.

Die beiden bereits kompakten allgemeinen PDF-Hooks bleiben bestehen.

## Produktive Regel

Der produktive Akzent-Suppressor ist nicht taktgebunden. Er findet scoreweit
genau einen relationalen Kandidaten aus Tie-Start/Tie-Stop, gleicher
Staff/Voice/Pitch-Relation und Akzent auf dem gebundenen Tail. Der irrefuehrende
Berichtstext `weiterhin taktgebunden` wird deshalb auf die tatsaechliche
scoreweite relationale Bindung korrigiert.

## Invarianten

Unveraendert bleiben:

- Solver, Kandidaten und Gewichtungen,
- alle MusicXML- und PDF-Writer,
- Rendererreihenfolge und Fallbacks,
- PDF-Quelle,
- MusicXML- und PDF-Inhalt,
- Freigabe- und Severityentscheidungen,
- produktive Akzent-Suppression,
- alle scoreweiten Balkenoperatoren.

Der Take entfernt nur einen nicht autorisierenden, etueden- und
layoutspezifischen Auditpfad.
