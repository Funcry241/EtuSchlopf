# RetiredHistoricalM18RendererEvidenceDirectoryFallback

Stand: v0.10.1623 r01
Change-Klasse: OutcomeProjection

## Befund

Die aktuelle Renderer-Evidenzsuche las zwei Laufzeitverzeichnisse:

- `target/etuschlopf_pdf_output_freshness_runtime`
- `target/etuschlopf_m18_renderer_exit_code_strictness_and_musescore_surface_audit`

Der zweite Pfad war ein historischer M18-Ursprungspfad. Im aktuellen Quellstand
existiert kein Erzeuger mehr fuer dieses Verzeichnis. Die Auswahl akzeptiert
ohnehin nur Evidenz der aktuellen Version oder die explizit gepinnte persistente
Referenz `docs/reference_renderer_written_pdf_nonzero_exit_v1111.txt`.

Damit konnte der M18-Pfad keine notwendige aktuelle Autoritaet mehr liefern. Er
erweiterte lediglich die Dateisystemsuche um einen historisch benannten,
produktiv nicht mehr geschriebenen Ort.

## Rueckbau

Dieser Take entfernt ausschliesslich den M18-Laufzeitpfad aus
`assistant_context_renderer_state_from_runtime`.

Erhalten bleiben:

- das aktuelle Laufzeitverzeichnis `etuschlopf_pdf_output_freshness_runtime`,
- die versionsgebundene Freshnesspruefung,
- die persistente bestaetigte Referenzevidenz v0.10.1111,
- der typisierte RendererOutcome,
- alle Fremd-/Stale-Evidence-Abweisungen,
- Visual-Preflight-, RunOutcome- und Exitcode-Semantik.

Ein negativer Security-Test bindet, dass der historische M18-Verzeichnisname
nicht erneut in den Launcher gelangt.

## Invarianten

Keine Aenderung an Musik, MusicXML, PDF, Rendererkommando, Writer, Solver,
Kandidaten, Schwellenwerten oder Projekt-/Takt-/Pitch-Auswahl. Der Take reduziert
nur eine historische Evidenz-Suchquelle ohne aktuellen Produzenten.

## Sparsame Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Erwartet: Strict Clippy 0, Build/FinalExit 0, etwa 25 Sekunden Programmlaufzeit,
MusicXML nach Versionsnormalisierung identisch und sechs pixelidentische
PDF-Seiten. Kein Korpuslauf.
