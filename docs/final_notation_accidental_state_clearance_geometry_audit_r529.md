# FinalNotationAccidentalStateClearanceGeometryAudit r529

## Ausgangspunkt

Parent ist die real installierte GreenAuthority `v0.10.1839 r528`
`ChangeClassPathRoleChangeKindColdStartReadbackClosure`.

r528 hat die aus dem abgelehnten r526 sichtbar gewordene Policy-Drift zwischen
`TakeChangeControl::PathRole` und dem bereits bestehenden Audit-Owner-Vertrag
geschlossen. Ein expliziter `src/audit_only/`-Owner wird nun vor lexikalischen
Musikmarkern als `Audit` klassifiziert. Die unabhaengigen AuditOnly-
Mutationsschutzschichten bleiben unveraendert.

r526 `FinalNotationAccidentalStateClearanceGeometryAudit` wurde vor Installation
durch den damaligen PathRole-Vertrag abgelehnt. r527 wurde nach Installation
zur Probe im Strict-Test durch den Cold-Start-Readback-Witness abgelehnt und
vollstaendig auf r525 zurueckgerollt. Beide Revisionen bleiben rejected und
werden nicht wiederverwendet. r528 ist installiert und `dark_green/released`.

Der fachliche Audit-Gedanke aus r526 wird daher jetzt unter neuer Revision r529
auf dem korrigierten kanonischen Owner-Vertrag erneut eingebracht. Weder
ChangeClass noch Pfadname werden zur Gate-Umgehung veraendert.

## Visueller Befund

Etuede 1 / Takt 7 unten: Die letzte sichtbare 16tel-Pause steht vor B-flat und
die Restglyph ueberdeckt das kleine Vorzeichen. Der Takt ist fuer seinen finalen
Notationsinhalt sichtbar zu eng.

## Root-Cause-Hypothese

Die bestehende r514-Beobachtung erkennt eine
`following_accidental_clearance_edge` nur, wenn die vorherige Note derselben
diatonischen Position exakt an der 16tel-Pause endet.

In Etuede 1 / Takt 7 lautet die relevante Zustandskette dagegen:

```text
B3 natural -> G3 -> 16tel-Pause D3 -> B-flat3
```

Das B3 natural endet vor dem G3, sein Vorzeichenzustand wirkt im Takt aber bis
zum folgenden B-flat3 fort. Der sichtbare B-flat-Kontext ist daher real, obwohl
die bisherige unmittelbare Linksnachbarschaft ihn nicht erkennt.

## Audit

r529 fuegt ausschliesslich writerfreie Audit-Evidenz hinzu. Fuer jede sichtbare
16tel-Pause wird beobachtet, ob unmittelbar nach ihrem Ende auf derselben
Staff-/Voice-Timeline eine Note folgt, deren gleiche diatonische Position zuvor
im Takt einen anderen Alter-Zustand hatte oder ein explizites Vorzeichen traegt.

Der Zustandssource darf dabei weiter links liegen. Fuer jeden Kandidaten werden
nur finale Fakten berichtet:

- Takt, Staff, Voice;
- Rest-Onset und Restdauer;
- finale Rest-Displayposition;
- folgende Pitchposition und Alter;
- Ende der letzten vorherigen gleichen diatonischen Position;
- zeitlicher Abstand dieser Zustandssource zur Pause (`state_gap`);
- signed diatonischer Abstand zwischen finaler Restposition und folgender Note;
- explizites Vorzeichen vs. aus dem Alter-Zustandswechsel abgeleitete Evidenz.

`state_gap=0` entspricht der bereits bekannten unmittelbaren r514-Klasse.
`state_gap>0` ist die neue carried-state Beobachtung.

## Kein produktiver Eingriff

r529 veraendert nicht:

- `following_accidental_clearance_edges`;
- `final_notation_timeline_facts`;
- MeasureDemand;
- Systemkapazitaet;
- System-/Seitenpartition;
- MusicXML;
- Restposition;
- Renderer-Input;
- Writer-/Layout-Authority;
- TakeChangeControl-/Kodex-Policy aus r528.

Der Take gibt der neuen Beobachtung ausdruecklich kein Schreib- oder
Entscheidungsrecht.

## Erwartete primaere Evidenz

Etuede 1 muss mindestens beide bekannten Klassen sichtbar machen:

```text
Takt 3: state_gap = 0  -> ImmediateState
Takt 7: state_gap > 0  -> CarriedState
```

Beide besitzen in der aktuellen Bassnotation die finale Restposition `D3` und
einen folgenden B-Pitch oberhalb der Restposition. Takt 7 bleibt im produktiven
r514-Demand zunaechst unveraendert; genau diese Trennung ist Zweck des Audits.

Der Vollkorpus dient danach zur Falsifikation: weitere carried-state Kandidaten
werden nicht automatisch zu ClearanceEdges oder Demand. Ihre Geometrie muss
zuerst mit den sichtbaren Comparatoren verglichen werden.

## Frueher Preflight

Vor Candidate-Bau wurde auf dem finalen Delta die in r528 gebundene Matrix
`ChangeClass x PathRole x ChangeKind` ausgewertet:

- `ZIP_KIND.txt`: Metadata / modified;
- diese Dokumentation: Metadata / added;
- `src/audit_only/final_notation_accidental_state_clearance_geometry.rs`:
  Audit / added;
- `src/audit_only/mod.rs`: Audit / modified;
- `src/audit_only/renderer_system_break_readback.rs`: Audit / modified.

Damit besteht kein Widerspruch zwischen `AuditOnly`, Ownerrolle und ChangeKind.

## Abnahme

Zuerst:

```powershell
./run.cmd --take "Etüde Nr. 1"
```

Erwartung: `FinalExit 0`, `Warnings 0`, `dark_green`, `released`, keine neue
Musikmutation und eine Auditzeile mit mindestens Takt 3 und Takt 7.

Danach:

```powershell
./run.cmd --all
```

Erwartung: 10/10 unveraendert gruen. Erst aus dieser Korpusevidenz wird der
produktive Folgetake abgeleitet.

## ChangeClass

`AuditOnly`.
