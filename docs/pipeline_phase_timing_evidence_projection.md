# PipelinePhaseTimingEvidenceProjection

## Zweck

Die vorhandene Konsolen-Fortschrittsmessung deckt nur sichtbare Schleifen ab und erklärte in
Export 558 rund 20 Sekunden einer etwa 161 Sekunden langen Programmlaufzeit. Dieser Take bindet
deshalb zusätzlich sequenzielle Top-Level-Phasen mit `Instant`-Messungen.

## Bericht

Neue Sektion: `PIPELINE-PERFORMANCE-TIMING-AUDIT-NO-MUTATION`.

Sie enthält:

- Projektvorbereitung
- Arrangement/Solver
- Master-Notation und Salienz
- Render-/Writer-Vorbereitung
- PDF-Quellenauswahl und Renderer
- Post-Render-Audit
- unklassifizierte Pre-Report-Zeit
Die bereits vorhandenen sichtbaren Fortschrittsläufe bleiben in der Konsole. Sie können
überlappen und werden deshalb nicht als Gesamtlaufzeit in den neuen Audit übernommen. Die
Messung besitzt keine Freigabe-, Kandidaten-, Writer- oder Musikautorität.
