# FinalNotationCarriedAccidentalStateClearanceGeometryCutover r530

## Ausgangspunkt

Parent ist die real installierte und im Vollkorpus bestaetigte GreenAuthority
`v0.10.1839 r529` `FinalNotationAccidentalStateClearanceGeometryAudit`.

r529 ist AuditOnly und hat im autoritativen 10/10-Korpus neun Kandidaten
beobachtet. Die neue carried-state Klasse darf daraus nicht pauschal produktiv
werden:

- Etuede 1 / Takt 7: `CarriedState`, Rest D3, folgendes B-flat3, signed
  diatonic delta `+5`;
- Etuede 3: drei carried-state Gegenbeispiele mit delta `-2`;
- Etuede 6 / Takt 32: carried-state mit delta `+10`;
- Etuede 7 / Takt 36: carried-state mit delta `+8`.

Der positive Takt-7-Fall besitzt dagegen dieselbe transpositionsinvariante
16tel-Rest/Flat-Glyphgeometrie wie der bereits produktiv gebundene r514-Fall
Etuede 1 / Takt 3: flaches Vorzeichen an einer Note eine diatonische Quinte
oberhalb der finalen Rest-Displayposition. Der Unterschied ist ausschliesslich,
dass der relevante Vorzeichenzustand in Takt 7 ueber eine zwischenliegende Note
fortwirkt.

## Produktiver Cutover

r530 erweitert ausschliesslich den bestehenden writerfreien
`final_notation_timeline_facts`-Owner.

Die bisherige immediate-state Semantik bleibt unveraendert: endet die letzte
sichtbare Note derselben diatonischen Position exakt an der sichtbaren
16tel-Pause und folgt unmittelbar danach ein anderer Alter-Zustand bzw. ein
sichtbares Vorzeichen, bleibt dies eine
`following_accidental_clearance_edge`.

Zusaetzlich darf eine fortgetragene Zustandssource weiter links im selben
Takt eine Clearance-Kante erzeugen, wenn die finale Glyphgeometrie exakt der
durch r529 positiv belegten Klasse entspricht:

```text
sichtbare 16tel-Pause
+ unmittelbar folgende sichtbare Note derselben Staff/Voice-Timeline
+ letzte fruehere gleiche diatonische Pitchposition mit anderem Alter-Zustand
+ Zustandssource endet strikt vor der Pause
+ folgendes Alter = flat
+ folgende Note liegt diatonisch genau eine Quinte (+5) ueber
  der finalen Rest-Displayposition
```

Die Regel ist damit weder Etueden-, Takt- noch absolute-Pitch-spezifisch.
D3/B-flat3 ist nur der aktuelle konkrete Beleg; die produktive Bedingung ist
translation-equivalente Glyphgeometrie.

## Unveraenderte Authority

r530 fuehrt keinen neuen Layout-Owner und keine neue Reserve ein.

Die bestehende r514-Kette bleibt:

```text
finale Notationsquelle
-> final_notation_timeline_facts
-> following_accidental_clearance_edges
-> bestehender Global-Reflow-Demand
-> bestehende Page/System-Partition
-> bestehender HorizontalLayout-Materializer
-> Renderer
```

Die bestehende Reserve pro Clearance-Kante bleibt unveraendert aus den bereits
autoritativen Modellgroessen abgeleitet. Es gibt:

- keine neue Capacity;
- keinen neuen frei kalibrierten Gewichtswert;
- keinen `measure/@width`-Write;
- keinen `default-x`-Write;
- kein Renderer-/PDF-Feedback als Entscheidungsquelle;
- keine Abhaengigkeit des produktiven Owners vom r529-Auditmodul;
- keine Rest-, Pitch-, Onset-, Duration-, Voice- oder Staff-Mutation.

Das r529-Audit bleibt reine unabhaengige Evidenz.

## Regressionen

Die unit-regressiven Gegenbeispiele binden:

- immediate-state bleibt weiterhin Clearance;
- carried flat fifth wird Clearance;
- carried flat unterhalb der Restposition (`delta -2`) wird nicht promoviert;
- carried flat weit oberhalb (`delta +10`) wird nicht promoviert;
- carried natural bei gleicher diatonischer Distanz wird ohne passenden
  Flat-Glyphbeleg nicht promoviert.

Damit ist insbesondere ausgeschlossen, die neun r529-Kandidaten pauschal zu
produktiven Edges zu machen.

## Erwartete primaere Wirkung

Fuer Etuede 1 soll Takt 7 von

```text
following_accidental_clearance_edges = 0
Demand = 127
```

auf genau eine bestehende r514-Clearance-Kante wechseln. Mit dem unveraenderten
r514-Reservemodell entspricht dies im aktuellen Modell einem Demand von 161.

Takt 3 bleibt eine bestehende Clearance-Kante. Die konkrete Page/System-
Partition ist weiterhin allein Ergebnis des bestehenden globalen Layout-
Owners; r530 kodiert weder einen Break vor Takt 7 noch eine gewuenschte
Seiten-/Systemzahl.

## Abnahme

Zuerst:

```powershell
./run.cmd --take "Etüde Nr. 1"
```

Zu pruefen sind insbesondere:

- `FinalExit 0`, `Warnings 0`, `dark_green/released`;
- Takt 7 besitzt eine produktive `following_accidental_clearance_edge`;
- Demand Takt 7 steigt ueber die unveraenderte r514-Reserve;
- Takt 3 bleibt clearance-gebunden;
- im PDF ist die Pause/Flat-Kollision in Takt 7 beseitigt;
- kein neuer Kontakt in Takt 3.

Erst danach Vollkorpus.

## ChangeClass

`MusicMutation`, weil die neue Final-Notation-Clearance-Klasse die bestehende
produktive Layout-Demand-/Page-System-Entscheidung beeinflussen darf. Die
musikalischen Ereignisse selbst bleiben unveraendert.
