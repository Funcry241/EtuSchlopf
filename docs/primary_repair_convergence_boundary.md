# Primary Repair Convergence Boundary

## Scope

`v0.10.1692 r01` ist Take 3 und damit der letzte Take des auf maximal drei Takes
begrenzten `PrimaryRepairOwnershipArchitecture`-Superwurfs. Ausgangspunkt ist
der vollstaendig gruene und artefaktidentische 10/10-Korpusstand `v0.10.1691`.

Take 2 bestaetigt fuer Staccato exakt den erwarteten Readiness-Vertrag:

- Primary: 14 Kandidaten, 4 Writes, 4 persistierte Writes,
- `primary_write_committed = 1` in genau 4 Projekten,
- `primary_write_persisted = 1` in genau denselben 4 Projekten,
- `persistence_recovery_ready = 0` in allen 10 Projekten,
- der bisherige Secondary Path wird trotzdem in 4 Projekten versucht,
- diese 4 Versuche liegen ausschliesslich in Projekten ohne Primary-Write,
- der Secondary Path schreibt im gesamten Korpus 0-mal.

Damit ist belegt: Candidate-/Target-Evidence allein ist keine
Persistence-Recovery-Readiness.

## Convergence

Take 3 ersetzt genau die bisherige Secondary-Branch-Bedingung

`target_scope > 0 && !staccato_primary_persisted`

durch den in Take 2 eingefuehrten typisierten Vertrag

`staccato_primary_readiness.persistence_recovery_ready`.

Dieser Vertrag ist genau dann wahr, wenn

`primary_write_committed && !primary_write_persisted`.

Damit gilt produktiv:

1. Primary sieht keinen oder mehrere nicht akzeptierte Kandidaten und schreibt
   nicht -> keine Persistence-Recovery.
2. Primary schreibt und der Write persistiert -> keine Persistence-Recovery.
3. Primary schreibt und genau dieser committed Write fehlt im finalen Artefakt
   -> der bestehende Recovery-Writer bleibt zulaessig.

Der bestehende `FinalArtifactStaccatoPersistenceFix` wird also nicht geloescht.
Er verliert aber seine bisherige zweite fachliche Entscheidungsrolle und bleibt
nur als echter Boundary-Recovery-Pfad erhalten.

## Ownership-Ergebnis

Vor Take 3:

`Primary candidate/decision -> optional Primary write -> target present? ->
Secondary candidate recheck/write attempt`

Nach Take 3:

`Primary candidate/decision -> optional Primary write -> committed write
persistence readback -> boundary recovery only on actual loss`

Damit besitzt der Primary die musikalische Staccato-Entscheidung. Der
Secondary Path darf keine Entscheidung mehr allein aus Candidate-/Target-Evidence
ableiten.

## Explizit unveraendert

Dieser Take aendert nicht:

- Candidate-Erkennung,
- Staccato-Auswahl oder Placement-Entscheidung,
- Staff-/Voice-/Onset-/Pitch-Guards,
- Primary-Materialisierung,
- Primary-Readback,
- Recovery-Materialisierung selbst,
- Structural-Write-Authority,
- Passreihenfolge,
- Rest-/Pausen-Producer,
- Schwellenwerte oder Gewichte,
- Master-MusicXML,
- beabsichtigte Render-MusicXML-Semantik,
- PDF-Auswahl oder Releaseentscheidung.

## Erwarteter Korpusbeweis

Abnahme mit genau einem Befehl:

```powershell
.\run.cmd --take --strict --all
```

Erwartet werden:

- 10/10 gruene Projekte,
- Primary weiterhin 14 Kandidaten, 4 Writes, 4 Persistierungen,
- Readiness weiterhin 4 committed, 4 persisted, 0 recovery-ready,
- `StaccatoPersistenceFallback attempted` im aktuellen Korpus 4 -> 0,
- `StaccatoPersistenceFallback written` weiterhin 0,
- final persistierte Staccato-Primary-Writes weiterhin 4,
- normalisierte Master-/Render-MusicXML artefaktidentisch zu v0.10.1691,
- PDFs pixelidentisch zu v0.10.1691.

Wenn dieser Beweis eintritt, ist `PrimaryRepairOwnershipArchitecture`
abgeschlossen. Es gibt keinen Take 4. Danach endet die allgemeine
Architekturarbeit und der naechste Auftrag waehlt wieder einen konkreten
musikalischen Bereich.
