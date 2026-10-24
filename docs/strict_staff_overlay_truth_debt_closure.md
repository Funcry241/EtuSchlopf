# StrictStaffOverlayTruthDebtClosure

## Zweck

Dieser `RefactorOnly`-Take bereinigt einen alten internen Widerspruch im aktiven `StrictStaffOverfullCoverageNormalization`-Pfad. Der bisherige Source behauptete diagnostisch eine zweistufige Strategie "vorhandene Forward-Zeit zuerst, Overlay nur als Fallback", obwohl der aktuelle Kontrollfluss ausschließlich die bereits bestehende Backup/Rest/Forward-Overlay-Materialisierung ausführte.

## Bereinigung

- entfernt `strict_staff_forward_intersections_cover_gap_parts`, weil dessen Ergebnis ausschließlich Trace-Text beeinflusste;
- entfernt die lokale `forwards`-Sammlung und das Feld `forward_cover`;
- entfernt den immer wahren Branch `rest_count == 0 || replacements.is_empty()`;
- entfernt den äußeren nie gelesenen `rest_parts`-Zwischenzustand;
- materialisiert denselben Overlay-Kandidaten direkt per `let ... else`;
- beschreibt Trace und Action-Kennung wahrheitsgemäß als explizite Staff-/Voice-Overlay-Materialisierung;
- korrigiert den HumanTrace-Beispielsatz: vorhandene `<forward>`-Fragmente sind Timeline-Evidenz, aber keine Staff-/Voice-besessene Restquelle.

## Invarianten

Unverändert bleiben:

- `strict_staff_overlay_rest_layer_for_gaps` und damit die erzeugten Backup/Rest/Forward-Fragmente;
- Kandidaten-Scoring und Kandidatenreihenfolge;
- maximale Aktionszahl;
- Structural-Write-Authority und der autorisierte Commit-Pfad;
- Parse-, Backup/Forward-, Raw-Cursor- und Sounding-Signature-Gates;
- sichtbare und verborgene Pitch-Invarianten;
- Rest-Delta-Adoption;
- Musikregeln, Thresholds und Passorder;
- Master-MusicXML, Neuro-Evidence und alle Security-Gates.

Der Take ändert bewusst nur Diagnoseidentität und entfernt nicht entscheidenden Zustand. Er fügt keine neue Materialisierungsstrategie hinzu.

## Abnahme

Autoritativ unter Windows:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Erwartet: normale Strict-Gates, unveränderte Musik-/Writer-Invarianten und GREEN. Ein 10er-Korpuslauf ist als zusätzlicher Beweis sinnvoll, weil dieser Writerpfad im Korpus aktiv ist, aber nicht Teil des Take-Installationsbefehls.
