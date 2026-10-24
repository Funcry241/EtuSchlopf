# RetiredCompleteBeatContextAndTypedBeamPreservation

**Version:** 0.10.1614 r02  
**Change-Klasse:** RefactorOnly

## Ausgangsschuld

Seit v0.10.1611 trifft die finale Higher-Beam-Closure ihre Entscheidung allein
aus dem kanonischen Master und der finalen PDF-Quelle. Der zuvor eingefuehrte
Complete-Beat-Kontextstore transportierte trotzdem weiterhin pro Projekt:

- die XML-Baseline vor der Primaerbalkennormalisierung,
- den normalisierten XML-Zwischenstand,
- einen globalen, mutexgeschuetzten Pfadeintrag.

Die beiden XML-Inhalte wurden nicht mehr gelesen. Nur das erfolgreiche
Entnehmen des Stores blieb als Veto in der Write-Bedingung erhalten. Damit
verursachte die pensionierte Architektur Speicher-, Kopier- und
Synchronisationsarbeit und konnte bei einem vergifteten Mutex einen
strukturell gueltigen Write ablehnen.

## Rueckbau

Entfernt werden:

- `PendingFinalCompleteBeatBeamContext`,
- `PendingFinalCompleteBeatBeamContextStore`,
- `pending_final_complete_beat_beam_context_store`,
- `bind_final_complete_beat_beam_context`,
- `take_final_complete_beat_beam_context`,
- die Bindung von `lower_after_xml` und `final_after_xml`,
- die `complete_beat_context_green`-Kopplung,
- der historische Context-Binding-Fehlerpfad.

Die bestehende Complete-Beat-Idempotenz- und Beam-only-Pruefung bleibt
erhalten.

## Typisierte Preservation

Der Higher-Beam-Plan wird nicht mehr als freie
`BTreeMap<u32, String>` zwischen Detector und Zentralwriter transportiert.
Der Detector erzeugt einen `BeamPlan` mit `BeamState`; die pfadgebundene
Pending-Bindung speichert denselben Plan. Die vorhandene Final-Notation-Senke
prueft:

- genau ein gebundenes Ereignisordinal,
- eindeutige und lueckenlose Balkenebenen,
- unveraendertes Beam-Level 1,
- Beam-only-Delta,
- parsebares MusicXML,
- exakten Dateireadback.

Erst bei der Serialisierung wird `BeamState::as_musicxml_text()` verwendet.

## Nichtveraenderung

Keine neue Writerwurzel, keine neue Dateischreibflaeche und keine neue
musikalische Auswahl. Projekt, Etuede, Takt, Pitch und Locator bleiben
vollstaendig ausserhalb der Regel. Master-/Render-MusicXML und PDF muessen
gegen v0.10.1613 – abgesehen von der Werkzeugversionszeile beziehungsweise
PDF-Metadaten – identisch bleiben.

## r02 – mechanischer Compile-Abschluss

r01 wurde durch `#![deny(unused_mut)]` gestoppt, weil die Bindung
`final_audit` nach dem Kontext-Rueckbau nicht mehr mutiert wird. r02 entfernt
ausschliesslich diesen ueberfluessigen Mutabilitaetsmarker.
