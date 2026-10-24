# Render-XML Snapshot Reader – Authority Map

Stand: v0.10.1583 r01
Ausgang: v0.10.1582 r02 / Export 521

## Problemgrenze

`pipeline_orchestrator_report_compaction.rs` besass vor diesem Take zwei private
Hilfsfunktionen fuer dieselbe technische Wahrheit:

- vorhandenen Render-MusicXML-Checkpoint lesen oder In-Memory-XML behalten,
- nach einem gemeldeten Write zwischen Checkpoint und Fallback waehlen.

Diese Entscheidung wurde 55-mal aus drei grossen Orchestratorfunktionen
aufgerufen. Damit lagen Pfadbindung, Datei-Readback und Fallback-Semantik direkt
im musikalischen Ablauftext, obwohl sie weder musikalische Policy noch
Schreibrecht darstellen.

## Authority Map vor dem Cut

InputSnapshot
: `best.layout_xml`, spaetere In-Memory-Snapshots und der feste
  `.render.musicxml`-Checkpointpfad.

Classification / Policy
: Die jeweiligen Rest-, Balken-, Artikulations-, Clef- und Layout-Komponenten.
  Dieser Take aendert keine ihrer Entscheidungen.

MutationPlan
: Die jeweiligen `build_*`, `apply_*` und typisierten Authority-Komponenten.
  Dieser Take aendert keinen Plan.

SingleWriter
: Die vorhandenen produktiven Writer in `structural_write_authority`.
  Dieser Take fuegt keinen Writer hinzu und entfernt keinen Writer.

Snapshot Selection
: Zwei private Funktionen im Orchestrator; 55 Aufrufe, Pfad und Fallback an
  jeder Stelle erneut sichtbar.

Verification
: Bestehende Readbacks und Audits. Keine Aenderung.

Projection / Report
: Bestehende `pipeline_lines` und FinalReport-Projektion. Keine neue
  Reportsektion und keine neue Freigabelogik.

## Zielgrenze dieses Takes

`RenderXmlSnapshotReader` besitzt genau eine schreibfreie Verantwortung:

```text
CheckpointPath + (WriteReported, FallbackXml)
    -> MaterializedCheckpointXml | unveraendertes FallbackXml
```

Der Reader besitzt ausdruecklich nicht:

- musikalische Auswahl,
- Write-Autorisierung,
- Reparaturrecht,
- Passreihenfolge,
- PDF-Quellenauswahl,
- Report- oder Release-Recht.

Die bisherige Fallback-Semantik bleibt exakt erhalten: Auch bei gemeldetem
Write wird bei fehlendem oder nicht lesbarem Checkpoint das uebergebene
In-Memory-XML verwendet.

## Beauty Ledger

| Merkmal | v0.10.1582 | v0.10.1583 |
|---|---:|---:|
| Private Snapshot-Helfer im Orchestrator | 2 | 0 |
| Aufrufe mit wiederholter roher Pfadbindung | 55 | 0 |
| Explizite Reader-Owner | 0 | 1 |
| Produktive Writer | unveraendert | unveraendert |
| Musikalische Policies | unveraendert | unveraendert |
| `.written`-Entscheidungen | unveraendert | unveraendert |
| Orchestrator-Datei | 5.529 Zeilen | 5.497 Zeilen |
| Physische Rust-Zeilen im gelieferten Baum | 128.177 | 128.204 |
| Netto physische Rust-Zeilen | – | +27 |

Die +27 Zeilen sind keine parallele Sicherheitslage: Die beiden alten Helfer
sind im selben Take entfernt. Der Mehrumfang besteht aus der echten Modulgrenze
und einem lokalen Vertrags-Test.

## Beweisumfang

1. Unit-Test des unveraenderten Fallback-Vertrags.
2. Strict-Build und Strict-Clippy.
3. Ein betroffenes Projekt.
4. Ein unabhaengiges Kontrastprojekt.
5. Kein Korpuslauf, sofern MusicXML, Render-MusicXML, PDF und Resultat im
   Projekt-/Kontrastvergleich unveraendert bleiben.

## Noch nicht geloeste Schuld

Dieser Take reduziert noch nicht:

- die 90 `.written`-Abfragen in `real_main`,
- die konkurrierenden oder historisch gewachsenen Writerfamilien,
- die 97 direkten Report-Anfuegungen,
- die Passfolklore der langen Renderkette.

Der naechste Cut darf erst nach gruenem Beweis entscheiden, ob ein typisierter
`RenderXmlCheckpointState` die verstreute Verfuegbarkeitsbuchhaltung ersetzen
kann. Das ist eine eigene Autoritaetsaenderung und wird nicht in diesen
Reader-Cut hineingemischt.
