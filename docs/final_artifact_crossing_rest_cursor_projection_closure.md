# CursorLaneCrossingRestPostconditionClosure

## r06 – Postcondition statt Precondition

Die r02-r05-Linie zeigte eine eindeutige Master/Render-Asymmetrie: Die allgemeine Crossing-Rest-Authority konnte den spaeten Renderstand real auf RawCursor 0/384/384 schliessen, waehrend der Master unveraendert 0/408/384 blieb.

Die Ursache liegt in der Reihenfolge innerhalb des bereits vorhandenen `repair_cursor_lane_end_invariant_xml`-Owners. Bisher wurde die Crossing-Rest-Closure vor dessen historischer Reanchor-/Rest-Visibility-Reparatur angewandt. Diese historische Reparatur durfte danach eine neue Crossing-Rest-Ueberhangstruktur materialisieren. Der Candidate war deshalb am Eingang noch nicht vorhanden, am Ausgang aber wieder rot.

## Ownership-Korrektur

r06 fuehrt keinen neuen Owner ein.

1. Der bestehende `CursorLaneEndInvariantRepair` materialisiert wie bisher seinen Measure-Ausgang.
2. Auf dem vollstaendig materialisierten XML wird die unveraenderte Crossing-Rest-Authority als Postcondition angewandt.
3. Erst danach wird der verbleibende globale RawCursor-Hardcount bestimmt und genau dieses XML zurueckgegeben.
4. Der spaete Render-Aufruf derselben Authority bleibt als finale Render-Projektionsgrenze erhalten.

Damit kann derselbe Owner seine eigene Crossing-Rest-Postcondition nicht mehr nachtraeglich verletzen.

## Strukturklasse

Ein Candidate muss:

- aus plain sichtbaren Restnoten bestehen;
- innerhalb derselben Staff/Voice fortlaufend sein;
- vor dem erwarteten Taktende beginnen;
- ueber das erwartete Taktende hinausreichen;
- unmittelbar vor einem Backup ueber die volle erwartete Taktdauer stehen.

Hidden-, Chord-, Pitch-, Beam-, Tie-, Notations-, Stem- und explizit positionierte Restnoten sind ausgeschlossen.

## Adoption

Der verbleibende In-Measure-Anteil wird allein ueber `MetricRestNotation` zerlegt. Der entfernte Ueberhang wird nach dem vollen Backup als `forward` reankert.

Vor Adoption muessen zugleich gelten:

- komplette pitched Timeline vor/nach identisch;
- RawCursor danach gruen;
- keine partielle Backup-Landung;
- kein post-expected Event;
- sofortige Idempotenz.

## Evidenz aus r04/r05

Der reale Etuede-9-Renderstand wurde durch dieselbe Authority von der Crossing-Rest-Struktur auf einen cursor-gruenen 0/384/384-Stand geschlossen. Der Master blieb rot, weil die Authority dort vor der historischen Reanchor-Logik lag. Diese Beobachtung begruendet ausschliesslich die Reihenfolgekorrektur; die Candidate-Regel selbst bleibt unveraendert.

## Releasevertrag

Der bestehende globale `RHY01 RAW_CURSOR_INVARIANT_HARD_GATE` bleibt unveraendert autoritativ. Keine Gate-Lockerung, keine Projekt-/Taktidentitaet, kein neuer Writer und kein neuer Report-Repair.
