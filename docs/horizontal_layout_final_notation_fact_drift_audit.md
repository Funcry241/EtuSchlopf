# HorizontalLayoutFinalNotationFactDriftAudit

## Zweck

Take 7 der horizontalen Layoutmigration ist ein reiner read-only Zeitautoritaets-Audit.
Er veraendert weder Legacy-Layoutpolicy noch Materialisierung.

Der bereits geschlossene `HorizontalLayoutLegacyPlan` bindet beim PlanClose einen
identity-freien `HorizontalLayoutFactSnapshot` des damaligen Notationszustands.
Nach `FinalNotationLayoutSnapshotClosure` werden aus der tatsaechlich ausgewaehlten
finalen PDF-Notationsquelle dieselben Rohfakten erneut extrahiert.

Verglichen werden ausschliesslich strukturelle Measure-Slots und die in Take 1
eingefuehrten beobachtbaren Fakten. Projekt-, Etueden- und Taktnummern sind kein
Teil des Audits.

Der Lauf meldet:

- verglichene Slots,
- Slots mit mindestens einem Drift,
- Anzahl geaenderter Feldwerte,
- Anzahl betroffener Faktenfamilien.

Ein Drift besitzt in diesem Take **kein Schreibrecht** und veraendert keine
Layoutentscheidung. Der Audit soll nur beweisen oder widerlegen, ob die bisherige
fruehe Layoutplanung auf einem spaeter veraenderten Notationszustand beruht.

## Unveraendert

- `choose_auto_pianist_layout`
- `compute_page_breaks`
- `compute_system_breaks`
- alle Legacy-Scores, Thresholds und Caps
- der geschlossene HorizontalLayoutLegacyPlan als produktive Entscheidung
- der One-Materializer aus Take 6
- PDF-Quellenauswahl und MuseScore
- alle Musikwriter und Security-Gates

## Naechster Schritt

Erst das reale Korpusergebnis dieses Audits entscheidet, ob vor dem neuen
Global-Reflow eine eigene Final-Notation-Planning-Authority noetig ist.
