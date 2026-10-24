# Primary Readiness Contract Boundary

## Scope

`v0.10.1691 r01` ist Take 2 des auf maximal drei Takes begrenzten
`PrimaryRepairOwnershipArchitecture`-Superwurfs. Ausgangspunkt ist der
vollstaendig gruene und artefaktidentische 10/10-Korpusstand `v0.10.1690`.

Take 1 zeigt fuer Staccato im aktuellen Korpus:

- Primary: 14 Kandidaten, 4 Writes, 4 persistierte Writes,
- Persistence-Fallback: 4 Versuche, 0 Writes,
- alle vier Fallback-Versuche liegen in Projekten, in denen der Primary 0-mal schrieb.

Der produktive Staccato-Primary laeuft bereits nach den Rest-/Pausen-Producern.
Deshalb wird kein zweiter Scheduling- oder Producer-Vertrag erfunden. Die
fehlende Readiness-Grenze liegt zwischen Primary-Outcome und dem spaeteren
Persistence-Recovery-Pfad.

## Bestehenden Ergebnistyp wiederverwenden

Der Take fuehrt keine neue musikalische Decision- oder Ownership-Authority ein.
Der bestehende `SingleClearStaccatoRenderWriteAudit` bleibt die einzige Quelle
fuer den Primary-Outcome. Daraus wird lokal genau ein kleiner typisierter
`SingleClearStaccatoPrimaryReadinessContract` abgeleitet:

- `primary_write_committed`: der Primary hat tatsaechlich geschrieben und
  `RenderXmlWrite` bestaetigt,
- `primary_write_persisted`: genau dieser committed Primary-Write ist im finalen
  Artefakt vorhanden,
- `persistence_recovery_ready`: ein Primary-Write wurde committed, ist aber im
  finalen Artefakt nicht mehr vorhanden.

Damit ist erstmals explizit ausgedrueckt, wann ein Persistence-Recovery fachlich
ueberhaupt bereit waere. Positive Candidate-/Target-Evidence allein ist keine
Recovery-Readiness.

## Take-2-Grenze

Der bestehende Fallback-Zweig wird in diesem Take absichtlich **nicht**
umgestellt. Seine bisherige Bedingung

`target_scope > 0 && !staccato_primary_persisted`

bleibt produktiv unveraendert. Der neue Contract wird daneben aus derselben
Primary-Evidence berechnet und in der bestehenden
`REPAIR-LOOP-INVOCATION-EVIDENCE` sichtbar gemacht.

Dadurch kann der Korpus fuer Take 3 direkt vergleichen:

- bisher tatsaechlich ausgefuehrte Fallback-Versuche,
- nach dem neuen Readiness-Vertrag legitime Persistence-Recovery-Faelle.

Take 3 darf den Secondary Path erst dann an die neue Readiness binden, wenn der
10/10-Korpus diesen Unterschied bestaetigt.

## Explizit unveraendert

Dieser Take aendert nicht:

- Candidate-Erkennung oder Staccato-Entscheidung,
- Placement-/Staff-/Voice-Guards,
- Primary-Materialisierung,
- Structural-Write-Authority,
- bestehende Fallback-Bedingung,
- Fallback-Materialisierung,
- Passreihenfolge,
- Rest-/Pausen-Producer,
- Schwellenwerte oder Gewichte,
- MusicXML-/Render-MusicXML-Semantik,
- PDF-Auswahl oder Releaseentscheidung.

## Erwarteter Korpusbeweis

Abnahme mit genau einem Befehl:

```powershell
.\run.cmd --take --strict --all
```

Erwartet werden weiterhin 10/10 gruene Projekte und artefaktidentische
Master-/Render-MusicXML sowie pixelidentische PDFs. Zusaetzlich soll die
Readiness-Evidence im aktuellen Korpus zeigen:

- `write_committed = 1` genau in den vier Primary-Write-Projekten,
- `write_persisted = 1` genau in denselben vier Projekten,
- `persistence_recovery_ready = 0` in allen zehn Projekten,
- der noch unveraenderte Legacy-Fallback wird weiterhin in den vier
  Candidate-ohne-Primary-Write-Projekten versucht.

Wenn dieser Beweis eintritt, ist Take 2 abgeschlossen. Take 3 kann dann genau
diesen einen Branch konvergieren: Candidate-/Target-Evidence darf keinen
Persistence-Recovery-Writer mehr aktivieren; ein echter Boundary-Fallback bleibt
nur fuer den Fall `primary_write_committed && !primary_write_persisted` erhalten.
