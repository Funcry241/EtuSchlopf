# FinalRenderRawCursorFallbackAuthorityRetirement

## Revision

`EtuSchlopf v0.10.1839 r347`

Parent: real GREEN r346 `FinalRenderRawCursorDecisionOwnershipEvidence`.

## Ausgangsbefund aus r346

r346 hat die RawCursor-Domaene erstmals explizit als Ownership-Insel projiziert:

```text
PrimaryBoundary CursorLaneEndInvariantPrimaryBoundary
LateBoundary    FinalRenderMasterMeasureFallback
SharedDecisionOwner none
SameInvariant   raw_cursor_measure_end
OwnershipState  multiple_productive_raw_cursor_closure_boundaries_open
```

Der reale r346-Etüde-6-Lauf ist GREEN und released. Der finale ausgewaehlte
Renderstand ist bereits cursorgruen:

```text
SelectedPdfSourceRawCursor:
HardViolations 0
MasterHard 0
SelectedRenderHard 0
PartialBackups 0
Green ja

CommittedRawCursorStable ja
CommittedRawCursorGreen ja
```

Damit wird fuer diesen Kontrolllauf keine spaete Master-Takt-Recovery benoetigt.

## Warum der spaete Fallback keine legitime zweite Materialisierung ist

`repair_cursor_lane_end_invariant_xml` ist die bestehende produktive
CursorLane-Endgrenze. Sie klassifiziert einen roten Takt und materialisiert
innerhalb desselben Takts die historisch autorisierten Cursor-Korrekturen.

`close_final_render_raw_cursor_measures_from_master` arbeitet anders. Nach
weiteren Render-Writes scannt es den committed-nahen Renderstand erneut und
darf bei einem roten Render-Takt die komplette Timeline durch den
korrespondierenden gruenen Master-Takt ersetzen, sofern Meter und
nicht-timeline-relevante Surface uebereinstimmen.

Damit ist der spaete Pfad keine blosse Projektion derselben Entscheidung. Er
besitzt ein eigenes Recovery-Ergebnis:

```text
Render timeline red
+
Master timeline green
+
surface equal
->
replace entire Render measure timeline with Master measure timeline
```

Das ist eine zweite produktive Recovery-Authority fuer dieselbe
RawCursor-Endinvariante.

## r347

r347 entfernt ausschliesslich diese spaete Master-Takt-Recovery:

- `final_render_cursor_fallback_timeline_element_end`
- `final_render_cursor_fallback_next_timeline_element`
- `final_render_cursor_fallback_surface_signature`
- `close_final_render_raw_cursor_measures_from_master`
- die entsprechende FinalRender-Aufrufkante

Erhalten bleiben:

1. die bestehende primaere `repair_cursor_lane_end_invariant_xml`-Grenze;
2. die bestehende identity-freie Crossing-Rest-Closure als spaete legitime
   Projektion;
3. der bestehende FinalNotation-Checkpoint;
4. die committed/finalen RawCursor-Readbacks;
5. der fail-closed Release-/PDF-Sicherheitsvertrag.

Der letzte FinalNotation-Checkpoint schreibt damit keinen aus dem Master
zurueckkopierten Takt mehr. Ist ein spaeter Renderstand cursorrot, darf die
Pipeline ihn nicht mehr durch eine zweite Recovery-Wahrheit heilen; die
bestehenden RawCursor-Gates muessen ihn read-only erkennen und fail-closed
blockieren.

## Ownership nach dem Schnitt

```text
Primary productive owner:
repair_cursor_lane_end_invariant_xml

Legitime spaete Projektion:
close_beat_scoped_rest_completion_crossing_rests_in_xml

Committed/final postconditions:
FinalRenderSourceClosure
SelectedPdfSourceRawCursor

Former late recovery:
FinalRenderMasterMeasureFallback retired
```

Damit lautet die Zielarchitektur:

```text
eine produktive Cursor-Reparaturgrenze
+
legitime projektionseigene Materialisierung
+
mehrere read-only Postconditions
```

statt:

```text
primaere Cursor-Reparatur
+
spaetere erneute Master-basierte Recovery
```

## ChangeClass

`RefactorOnly`.

Kein neuer Writer, keine neue Repair-/Recovery-Authority, keine Schwelle, keine
Projekt-/Takt-/Pitch-Sonderlogik, keine Passorder-Aenderung und keine
Security-/Gate-Lockerung.

Der gueltige gruenen Outputpfad soll unveraendert bleiben. Fuer einen spaeten
cursorroten Renderzustand wird das historische Recovery-Verhalten bewusst
pensioniert: statt Master-Takt-Adoption gilt der bereits vorhandene
fail-closed RawCursor-Vertrag.

## Abnahme

Zuerst:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Erwartung fuer den r346-Kontrollfall:

- `FinalExit 0`
- `ReleaseDecision released`
- Build/Clippy/Launcher GREEN
- Testbestand mindestens 1088/1088
- Master-MusicXML invariant
- Render-MusicXML invariant
- PDF visuell/pixelgleich
- `SelectedPdfSourceRawCursor HardViolations 0`
- `CommittedRawCursorGreen ja`

Danach ist fuer die vollstaendige Retirement-Abnahme der 10er-Korpus relevant.
Ein Projekt, das nur durch den pensionierten Master-Takt-Fallback gruen war,
soll jetzt bewusst fail-closed werden und damit den noch vorhandenen
upstream/late Writer-Vertragsbruch sichtbar machen.
