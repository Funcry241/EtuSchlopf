# Renderer System-Break Demand Evidence Binding r28

Parent: `0.10.1839 r27`

ChangeClass: `RefactorOnly`

## Ausgangslage

r27 hat die externe Renderer-System-Break-Grenze im echten Windows-Lauf bewiesen. Etuede 6 materialisiert 19 geplante Systeme, MuseScore rendert 21 Systeme und fuegt vor Takt 26 sowie vor Takt 37 zusaetzliche Systemumbrueche ein. Die sechs geplanten Seitenanfaenge bleiben dagegen exakt erhalten. Der r27-Readback ist read-only und 743/743 All-Targets-Tests sind gruen.

r27 hat Demand/Capacity absichtlich noch nicht an den PDF-Readback gebunden. Die Ergebniszeile markiert diese Luecke explizit als `deferred_to_existing_horizontal_planner_evidence_transport`.

## Ziel

r28 schliesst ausschliesslich diese Evidence-Transport-Luecke.

Der bereits existierende Joint Page/System Planner berechnet fuer jedes ausgewaehlte System schon heute seine Systemlast und prueft diese gegen `AcceptedSystemCapacity`. r28 erfindet keine neue Rechnung. Es materialisiert lediglich genau diese bereits berechneten Lasten als read-only Planner-Evidence und bindet sie spaeter an den r27-PDF-Readback.

Damit kann fuer jedes geplante System korpusweit gezeigt werden:

- Measure-Bereich,
- bereits vom Joint Planner berechnete Systemlast,
- bereits vom Joint Planner verwendete AcceptedSystemCapacity,
- Headroom = Capacity minus Planner-Last,
- vom Renderer innerhalb dieses geplanten Systems eingefuegte Breakpositionen.

## Kein zweites Demand-Modell

Die Systemlasten stammen direkt aus derselben `global_reflow_system_loads`-Berechnung, die der Joint Planner vor seiner bestehenden Capacity-Pruefung bereits verwendet.

r28:

- ruft keine zweite Demand-Formel auf,
- rekonstruiert Demand nicht aus dem PDF,
- leitet keine neue Capacity ab,
- aendert keinen Threshold,
- aendert keine Auswahlordnung,
- aendert keine Page-/Systementscheidung,
- aendert keinen Writer,
- aendert keinen Rendereraufruf.

Die neue Planner-Projektion `SelectedSystemLoads` ist reine Evidence der bereits akzeptierten Joint-Partition.

## Bindungsvertrag

Der r27-Readback akzeptiert die Planner-Evidence nur, wenn sie exakt zu seinen geplanten Systemgruppen passt.

Fail-soft fuer den Audit:

- fehlende Planner-Evidence => DemandBinding `UNAVAILABLE`,
- doppelte Capacity => `UNAVAILABLE`,
- ungueltige Systembereiche => `UNAVAILABLE`,
- andere Systemgruppen als im materialisierten Plan => `UNAVAILABLE`,
- Planner-Last groesser als die transportierte Capacity => `UNAVAILABLE`.

Die bestehende Produktionsfreigabe wird dadurch nicht veraendert.

## Erwartete Projektion

Sinngemaess:

`RendererSystemBreakReadbackDemandBinding: Status BOUND | Source existing_joint_planner_evidence | AcceptedSystemCapacity ... | PlannedSystemEvidence ... | BreakAffectedSystems ... | ModelRecompute nein | ThresholdChange nein | PolicyWrite nein`

Ein System wird beispielsweise als

`13-15=load511/headroom8/split15`

projiziert, wenn der bestehende Planner 511 Last bei Capacity 519 berechnet hat und MuseScore innerhalb des geplanten Systems vor Takt 15 bricht.

## Etuede 6 als erster Zielsystembeweis

r27 hat fuer Etuede 6 bereits bewiesen:

- PlannedSystems 19,
- RenderedSystems 21,
- InsertedSystemBreaks 26,37,
- PlannedPages 6,
- RenderedPages 6,
- keine Page-Drift.

r28 soll im selben Ein-Befehl-Lauf fuer die beiden betroffenen geplanten Systeme zusaetzlich die bereits vorhandenen Planner-Lasten und den Headroom binden.

## Invarianten

- musikalische Bytes sollen gegen r27 unveraendert bleiben,
- PDF soll gegen r27 pixelidentisch bleiben,
- One Joint Page/System Plan unveraendert,
- One Materializer unveraendert,
- Width bleibt RendererAuto,
- keine Etueden-/Taktspeziallogik,
- keine neue System-Break-Authority,
- keine Capacity-Aenderung,
- kein Threshold-Tuning,
- keine Mutation aus Readback-Evidence,
- kein OCR,
- kein zweiter Renderer-Lauf.

## Warum RefactorOnly

Der Schnitt erweitert eine bestehende musiknahe Planner-Struktur ausschliesslich um bereits intern berechnete, verhaltensneutrale Evidence und transportiert sie in den Audit. Da dafuer zwei vorhandene Pipeline-Shards um reine Evidence-Projektion erweitert werden, ist `AuditOnly` gemaess dem aktuellen Cross-Role-Vertrag zu eng. `RefactorOnly` beschreibt den Schnitt korrekt: keine Musiksemantik, keine Policy und kein Writer werden veraendert.

## Naechster Schritt

Erst nach einem gruenden r28-Einzellauf und anschliessender 10/10-Korpusmessung darf statistisch gefragt werden, wie Headroom mit Renderer-Splits korreliert. Aus r28 selbst folgt noch keine neue Capacity-Schwelle.
