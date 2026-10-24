# Mutation Ownership Evidence Boundary

## Scope

`v0.10.1690 r04` ist Take 1 des auf maximal drei Takes begrenzten
`PrimaryRepairOwnershipArchitecture`-Superwurfs. Ausgangspunkt ist der
vollstaendig bestaetigte Korpusanker `v0.10.1689`.

Der Take ist ausschliesslich `AuditOnly`. Er verschiebt keinen Pass, aendert
keinen Writer, entfernt keinen Repair und fuehrt keine neue produktive
Ownership-Abstraktion ein. Stattdessen verbindet er drei bereits vorhandene
Beweisquellen in einer neuen read-only Ergebnisprojektion:

1. das build-time erzeugte `PRODUCTIVE-WRITE-PATH-INVENTORY`,
2. bereits vorhandene explizite Owner-Marker aus produktiven Contracts,
3. die mit v0.10.1687 eingefuehrte `REPAIR-LOOP-INVOCATION-EVIDENCE`.

## Neue Projektion

Der Ergebnisbericht erhaelt direkt unter der bestehenden
`MUTATION-SEMANTICS-AUTHORITY` die Sektion:

`MUTATION-OWNERSHIP-EVIDENCE-BOUNDARY`

Sie zeigt pro Projekt:

- Zahl der produktiven physischen XML-Write-Surfaces,
- Anteil innerhalb der zentralen Structural-Write-Authority,
- im finalen Ergebnis vorhandene aktive Writerzeilen in ihrer Projektionsreihenfolge,
- bereits vorhandene explizite `MusicalDecisionOwner`-/`MaterializationOwner`-
  Marker,
- vier fuer den Superwurf entscheidende Ownership-Muster.

## Keine zweite Authority

Die Sektion ist bewusst keine neue Schreib- oder Mutationsauthority. Sie erteilt
keine Rechte und besitzt keine produktiven Capabilities. Die bereits vorhandene
`MUTATION-SEMANTICS-AUTHORITY` bleibt unveraendert autoritativ fuer die Trennung
von RenderXmlWrite, RenderNotationMutation, SoundingEventMutation,
MasterMusicXmlMutation und DeclaredTakeChangeClass.

Die neue Projektion beantwortet nur die davon verschiedene Frage:

> Welche bereits vorhandene Decision-/Plan-/Materialization-Kette ist heute als
> fachlicher Owner erkennbar, und wo ist diese Ownership noch implizit oder
> ueberlappt mit einem spaeteren Repair/Fallback?

## Bestehende positive Vorbilder

### Beam Local Voice Batch

Der in v0.10.1684 bis v0.10.1686 strukturierte Pfad wird nur als bestehendes
Referenzmuster sichtbar gemacht:

`Observation -> Decision -> LocalVoiceBatchPlan -> Materialization -> Verification`

Kein Beam-Code wird in diesem Take geaendert.

### Lower Clef Surface

Der aktuelle Code besitzt bereits explizit getrennte Marker:

- `MusicalDecisionOwner RustigClefSurfaceDecision`
- `MaterializationOwner TypedClefSurfaceMaterialization`

Auch hier erfolgt keinerlei Aenderung. Dieses Muster ist Evidence dafuer, dass
EtuSchlopf bereits geeignete Owner-Vertraege besitzt und Take 2 moeglichst auf
bestehende Contracts aufsetzen soll.

## Noch nicht typisierte Ownership-Faelle

### ShadowRest

Der aktuelle Pfad ist nach v0.10.1689 bereits auf genau eine Position nach
StrictStaff reduziert. Die neue Sektion bindet die vorhandene Runtime-Evidence
an die Aussage, dass die Position nun eindeutig ist, der semantische Owner aber
noch nicht als eigener typisierter Vertrag ausgedrueckt wird.

### ShortRestGlyph

Der aktuelle Einzelaufruf liegt nach `FragmentedRestChainTransactionalWrite`.
Die Evidence trennt Readiness-Producer, Detector/Guard und Materialisierung.
`written = 0` ist ausdruecklich kein Loeschsignal, weil reale Kandidaten
weiterhin beobachtet werden koennen.

### Staccato

Der reale Kontrollfluss wird ohne Aenderung sichtbar als:

`Primary -> Persistence Readback -> bedingter Secondary Path -> finaler Readback`

Wichtig fuer Take 2/3: Der Secondary Path verlangt heute nicht, dass der Primary
zuvor tatsaechlich geschrieben hat. Er kann bereits bei positivem `target_scope`
und fehlender Persistence versucht werden. Die Projektion bezeichnet das nur als
Ownership-Ueberlappung; sie aendert die Bedingung nicht.

## Take-2-Auswahlregel

Take 2 wird erst nach dem 10/10-Korpusbeweis festgelegt. Gewaehlt wird genau eine
reale Domaene, in der ein bereits vorhandener Plan-/Decision-Vertrag mit der
kleinstmoeglichen Readiness-Grenze verbunden werden kann.

Nicht erlaubt in Take 2:

- globaler Scheduler,
- Vollmigration aller Repairs,
- neue musikalische Regel,
- neue Gewichtung,
- Writer-Verschiebung ausserhalb der gewaehlten Domaene.

## Explizit unveraendert

Dieser Take aendert nicht:

- produktiven Pipeline-Call-Graph,
- Passreihenfolge,
- Structural-Write-Authority,
- Candidate-Erkennung,
- Plans oder Decisions,
- XML-/MusicXML-Mutation,
- Staff/Voice/Pitch/Onset/Duration,
- Schwellenwerte oder Gewichte,
- Commit/Rollback,
- Render-/PDF-Auswahl,
- Releaseentscheidung.

## Zielsystembeweis

Da Take 2 aus der realen Korpusverteilung ausgewaehlt wird, ist die Abnahme:

```powershell
.\run.cmd --take --strict --all
```

Erwartung:

- 10/10 Projekte,
- BuildExit 0,
- ClippyExit 0,
- FinalExit 0,
- 0 Blockerprojekte,
- 0 Warnprojekte,
- normalisierte Master-/Render-MusicXML gegen v0.10.1689 identisch,
- PDF pixelidentisch,
- pro Projekt genau eine `MUTATION-OWNERSHIP-EVIDENCE-BOUNDARY`-Sektion.

Erst aus diesem Beweis wird `PrimaryReadinessContractBoundary` konkret
ausgewaehlt. Take 1 konvergiert bewusst noch keinen Repair.

## r03 Final-report contract closure

Der r02-Korpuslauf zeigte korrekt fail-closed, dass ein `project_section`-Producer keine zweite, nicht deklarierte formale Sektion erzeugen darf. Die Ownership-Evidence wird deshalb als klar markierter audit-only Unterblock innerhalb der bereits deklarierten `MUTATION-SEMANTICS-AUTHORITY` projiziert. Der typisierte FinalReportProjection-Vertrag und die produktive Pipeline bleiben unveraendert.


## r04 Installed-baseline dependency closure

Der nach r02 erzeugte Korpusreport beweist, dass der neue Programmstand zwar
ausgefuehrt wurde, der fehlgeschlagene Lauf den Take aber nicht installiert hat.
Der anschliessende Rust-Take-Bootstrap meldete weiterhin explizit
`rust-take-bootstrap v0.10.1689` und `aktueller Stand 0.10.1689`. Damit ist der
reale Ausgangsstand fuer die naechste Revision eindeutig v0.10.1689.

r04 ist deshalb kein Same-Version-Take. Es enthaelt den vollstaendigen
AuditOnly-Diff von v0.10.1689 nach v0.10.1690 einschliesslich der bereits in r03
korrigierten Final-Report-Projektion. Produktiver Call-Graph, Passreihenfolge,
Writer, Authorities, Plans, Guards und Musiksemantik bleiben unveraendert.
