# r531 – FinalNotationClearanceSystemStructuralSlackClosure

## Ausgangspunkt

r530 hat die in r529 beobachtete `CarriedState`-Geometrie korrekt in die bestehende
`following_accidental_clearance_edges`-Authority aufgenommen. Der reale Einzelreexec
von Etüde 1 ist technisch vollständig grün, zeigt im PDF aber ein strukturelles
Gegenbeispiel der nachgelagerten Systemplanung:

- Takt 7 erhält die neue Clearance-Kante und wird im finalen System `6-7` mit hoher
  struktureller Reserve sauber dargestellt.
- Gleichzeitig wird Takt 3, dessen bereits seit r514 bestehende Clearance-Kante
  unverändert aktiv ist, in das maximal dichte Dreiersystem `3-5` gepackt; dort
  erscheint die bekannte Pause/Flat-Kollision erneut.

Der Fehler liegt damit nicht mehr in der Erkennung der lokalen Glyphklasse. Er liegt
in der nachgelagerten gemeinsamen Page/System-Partition: ein MeasureDemand mit
Clearance-Reserve darf zwar die Systemlast erhöhen, aber ein clearance-tragender
Takt kann anschließend trotzdem Teil eines Systems mit der maximal erlaubten
Taktanzahl werden. Da explizite Measure-Widths pensioniert sind und der Renderer die
Binnenverteilung besitzt, ist die lokale Reserve dadurch nicht als struktureller
Freiraum garantiert.

## Closure

r531 führt **keine neue Pixel-, Width-, Capacity- oder Gewichtsregel** ein. Die
bestehende `following_accidental_clearance_edges`-Evidenz wird lediglich auch an die
bereits autoritative finale Renderer-Feasibility-Partition gebunden.

Ein Candidate-System, das mindestens eine solche Clearance-Kante enthält, darf
nicht zugleich die bestehende `max_system_measures`-Grenze vollständig ausschöpfen.
Es bleibt also genau **ein struktureller Slot Reserve**. Der Vertrag ist relativ zur
bereits vorhandenen Systemlängen-Authority formuliert; es wird keine neue absolute
"2-Takte-Regel" eingeführt. Falls die bestehende maximale Systemlänge selbst 1 ist,
bleibt der Singleton zulässig, damit der Vertrag nicht unmöglich wird.

Die Regel wirkt ausschließlich in der finalen gemeinsamen Page/System-Partition und
wird bei der Promotion erneut fail-closed validiert. Renderer-No-Goods bleiben
Renderer-No-Goods; die Clearance-Policy wird nicht als künstliches No-Good getarnt.
Der r529-Audit bleibt reine Evidenz und wird nicht zum produktiven Dependency.

## Erwartung für Etüde 1

Mit r530 gilt im realen Einzelreexec:

- Takt 3: ClearanceEdge 1, finales System `3-5`, Headroom 40 – visuell erneut zu eng.
- Takt 7: ClearanceEdge 1, finales System `6-7`, Headroom 140 – visuell sauber.

Die r531-Regel macht `3-5` als maximal dichtes clearance-tragendes System
unzulässig. Die bestehende PageCount-Frontier und Renderer-Feasibility-Authority
müssen daraus selbst eine zulässige Partition bestimmen. Es wird kein konkreter
Takt, keine Etüde und keine gewünschte Page-/Systempartition codiert.

## Invarianten

- keine Änderung an Pitch, Onset, Dauer, Voice oder Staff;
- keine Master-MusicXML-Mutation;
- keine neue Rest- oder Vorzeichen-Authority;
- keine neue Capacity, kein neues Gewicht und kein expliziter Width-Writer;
- r529 bleibt AuditOnly;
- r530 `CarriedState`-Clearance bleibt erhalten;
- Renderer-Feasibility bleibt die einzige produktive Endauswahl der gemeinsamen
  Page/System-Partition;
- keine Etüden-/Takt-ID-Sonderlogik.

`ChangeClass=MusicMutation`.
