# HorizontalLayoutPlanContractProjection

Take 3 der horizontalen Layoutmigration ist weiterhin `RefactorOnly` und absichtlich read-only. Er fuehrt **noch keine neue Planning Policy und keinen neuen Writer** ein.

Nach `FinalNotationLayoutSnapshotClosure` wird die bereits materialisierte, snapshot-gebundene Notationsquelle strukturell in einen immutable Planvertrag projiziert. Der Vertrag enthaelt in Sequenzreihenfolge ausschliesslich die horizontalen Direktiven, die spaeter von einer echten Planning Authority besessen werden sollen:

- explizite `measure/@width`-Information, falls vorhanden,
- `new-system`,
- `new-page`.

Die Projektion besitzt keine Projekt-, Etueden- oder Taktidentitaet. Die Reihenfolge der Measure-Elemente ist selbst die Struktur; `number=` wird weder gelesen noch als Policy-Input verwendet.

Der Planvertrag ist an den Fingerprint des bereits geschlossenen finalen Notationssnapshots gebunden und wird vor und nach MuseScore read-only gegen dieselbe Quelle verifiziert. Er kann keine Musik- oder Layoutmutation ausloesen.

Warum dieser Zwischentake notwendig ist: Eine echte `HorizontalLayoutPlanningAuthority` darf nicht bloss bereits geschriebenes XML nachtraeglich zur Wahrheit erklaeren. Dieser Take definiert daher zuerst die **Zielgestalt des Planobjekts und seinen Snapshot-Vertrag**. Im folgenden Take koennen die bestehenden Legacy-Entscheider schrittweise so umgestellt werden, dass sie diesen Vertrag vor Materialisierung erzeugen.

Unveraendert bleiben insbesondere:

- `choose_auto_pianist_layout`,
- `compute_page_breaks`,
- das Zwei-Takt-Systemraster,
- `VisualLayoutPressure`,
- `PianisticReadability`,
- `FinalShortSystemBalance`,
- alle Width-Targets und Caps,
- alle StructuralWriteAuthority-Writer,
- PDF-Quellenauswahl und MuseScore-Aufruf.

Acceptance: Der reale Lauf muss DARK GREEN bleiben; Master- und Render-MusicXML duerfen sich gegenueber 0.10.1824 ausschliesslich in der Software-Versionsmetadatenzeile unterscheiden.
