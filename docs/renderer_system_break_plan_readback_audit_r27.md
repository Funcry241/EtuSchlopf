# Renderer System-Break Plan Readback Audit r27

Parent: `0.10.1839 r26`

ChangeClass: `AuditOnly`

## Ziel

r27 oeffnet ausschliesslich die durch den frischen Vergleich Etuede 3 Takt 15 vs. Takt 22 bewiesene externe Renderer-Grenze: Der autoritative Joint Page/System Plan kann einen Systemverband materialisieren, den MuseScore beim tatsaechlichen PDF-Layout dennoch automatisch trennt.

Der Schnitt ist strikt read-only. Er aendert weder MusicXML noch Render-MusicXML, Break-Policy, Width-Policy, Demand, Capacity, PreferredPages, PageCount, Writer noch Rendereraufruf.

## Readback

Der Audit liest zwei bereits vorhandene finale Artefakte:

1. die materialisierte finale MusicXML-Quelle fuer die geplanten System-/Seitenstarts,
2. das von MuseScore geschriebene PDF fuer die tatsaechlichen System-/Seitenstarts.

MuseScore schreibt Taktnummern an Systemanfaengen als echten PDF-Text. r27 liest diese Textobjekte inklusive physischer Position direkt aus dem PDF. OCR, Rasteranalyse, Bildheuristik und ein zweiter Renderer-Lauf werden nicht verwendet.

Der PDF-Leser ist absichtlich ein enger, fail-soft MuseScore/Qt-Readback. Nicht unterstuetzte PDF-Dialekte erzeugen `Status UNAVAILABLE`; sie blockieren die bestehende Produktionsfreigabe nicht und erzeugen niemals Schreibrecht.

## Persistierte Evidenz

Die bestehende audit-only Projektion `HORIZONTAL-LAYOUT-PAGE-COUNT-PREFERENCE-SHADOW` erhaelt zusaetzlich:

- PlannedSystems / RenderedSystems,
- PlannedSystemStarts / RenderedSystemStarts,
- PlannedSystemGroups / RenderedSystemGroups,
- InsertedSystemBreaks,
- CollapsedSystemBreaks,
- PlannedPages / RenderedPages,
- PlannedPageStarts / RenderedPageStarts,
- InsertedPageBreaks,
- CollapsedPageBreaks,
- PDF-Seitenzahl,
- eindeutige Readback-Basis,
- `PolicyWrite nein`.

Demand/Capacity werden in r27 bewusst **nicht** neu berechnet. Die vorhandene horizontale Planner-Evidence bleibt dafuer die einzige Authority. Der Audit meldet daher explizit, dass die Demand-Bindung spaeter ueber einen bestehenden Planner-Evidence-Transport geschlossen werden muss. Es entsteht kein zweites Demand-Modell.

## Aktuelle r26-Korpusevidenz vor dem Take

Read-only Analyse des gelieferten r26-Exports zeigt:

- Etuede 1: Plan 8 Systeme, Renderer 8, keine Drift.
- Etuede 2: Plan 13, Renderer 16, zusaetzliche Starts 3, 6, 24.
- Etuede 3: Plan 12, Renderer 14, zusaetzliche Starts 3, 15.
- Etuede 4: Plan 11, Renderer 11, keine Drift.
- Etuede 5: Plan 22, Renderer 22, keine Drift.
- Etuede 6: Plan 19, Renderer 21, zusaetzliche Starts 26, 37.
- Etuede 7: Plan 20, Renderer 23, zusaetzliche Starts 32, 37, 43.
- Etuede 8: Plan 20, Renderer 21, zusaetzlicher Start 23.
- Etuede 9: Plan 16, Renderer 16, keine Drift.
- Etuede 10: Plan 19, Renderer 19, keine Drift.

Damit zeigen 5/10 Projekte automatische Renderer-Systembrueche. In dieser Probe wurde kein geplanter Systembreak vom Renderer zusammengelegt. Alle geplanten Seitenanfaenge stimmen mit den tatsaechlichen Seitenanfaengen ueberein.

Das isoliert die offene Authority-Luecke auf die Systembildung; die Page-Plan-Authority zeigt in diesem Korpus keinen Drift.

## Etuede 3 als Beweisfall

Geplant:

`... 11-12 | 13-14-15 | 16-17 ...`

Gerendert:

`... 11-12 | 13-14 | 15 | 16-17 ...`

Der bereits bekannte Planner-Wert `13-15 Demand = 511` bei `AcceptedSystemCapacity = 519` bleibt externe vorhandene Evidence; r27 dupliziert diese Rechnung nicht.

## Invarianten

- AuditOnly.
- MusicMutation nein.
- PolicyWrite nein.
- WriterAuthority nein.
- Width bleibt RendererAuto.
- One Joint Page/System Plan bleibt unveraendert.
- One Materializer bleibt unveraendert.
- Keine Etueden-/Taktspeziallogik.
- Keine Capacity-/Demand-Aenderung.
- Keine neue harte Release-Grenze.
- Kein OCR.
- Kein zweiter Renderer-Lauf.
- Unsupported PDF => explizit UNAVAILABLE, nie geraten.

## Naechster Schritt nach Evidenz

Wenn der r27-Korpus die beobachtete Drift reproduziert, ist der naechste Schnitt **nicht** Takt 15 und **nicht** eine willkuerliche Capacity-Senkung. Dann muss die bereits vorhandene Planner-Demand-Evidence read-only an die gerenderten Systemgruppen gebunden werden, damit Headroom und Renderer-Break-Drift korpusweit korreliert werden koennen, ohne eine zweite Demand-Authority zu schaffen.
