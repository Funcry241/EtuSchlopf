# r509 – CanonicalMeasureRestMetricTruthCallGraphPreservationClosure

## Ausgangspunkt

r508 ist der installierte Parent. Ein erster r509-Candidate wurde vom Kodex verworfen, weil ein neu hinzugefuegter Test im produktiven `src`-Modul einen neuen Einstieg in den bereits writer-erreichbaren CanonicalMeasureRest-Pfad erzeugte. Der Candidate wird nicht kosmetisch umbenannt oder indirekter gemacht; seine Architekturinformation wird uebernommen.


## Zweite Kandidatenablehnung – Change-Class-Trennung

Ein zweiter r509-Candidate wurde vor dem Semantic-Gate vom Take-Change-Control verworfen, weil die MusicMutation zusaetzlich `tests/architecture_essence_cold_start_readback.rs` veraenderte. Das Gate arbeitet korrekt: repository-residente Essenz-Dokumentation ist Metadata und darf den fachlichen Take begleiten; Audit-Code gehoert jedoch nicht in eine MusicMutation.

Der korrigierte Candidate laesst den Audit-Test deshalb byteunveraendert. Die beiden Architekturlehren werden innerhalb der bestehenden Essenzregeln E05/E06 als Metadata geschaerft, ohne den E00-E31-Regelindex oder dessen Audit-Vertrag zu veraendern. Der fachliche Regressionstest verbleibt ausschliesslich im bereits existierenden `src`-Test-Einstieg des betroffenen Moduls und erzeugt keine neue Funktion bzw. keinen neuen produktiven Mutationseinstieg.

## Fachliche Ursache

Eine vollstaendig stille Lane kann als eine sichtbare Viertelpause plus drei `forward`-Carrier vor dem ersten `backup` vorliegen. Der bestehende CanonicalMeasureRestSingleOwner bestimmte fuer diese erste Lane die erwartete Dauer lokal aus Note-Event-Enden und erhielt dadurch 96 statt der aus 4/4 und divisions=96 geerbten metrischen Scope-Dauer 384. Nach einem `backup duration=384` besass die spaetere Lane dagegen bereits die richtige Erwartung und wurde kanonisiert. Ueberlebende `forward`-Carrier konnten spaeter wieder als sichtbare Viertelpausen materialisiert werden.

## Closure

Der bestehende CanonicalMeasureRest-Pfad bevorzugt nun die bereits vorhandene writerfreie `render_scope_metric_expected_duration_map` fuer die erwartete Taktdauer. `infer_measure_duration_from_measure_xml` bleibt nur Fallback. Die bestehende produktive Writer-Aufrufkette und die StructuralWriteAuthority-Funktion werden nicht erweitert.

Die Regression wird in einen bereits existierenden Test-Einstieg integriert. Es entsteht keine neue Funktion, die den Mutation-Graphen erreicht.

## Essenzgewinn aus den Kandidatenablehnungen

Die repository-residente Essenz wird ohne neue Regelnummern an den bereits passenden Stellen geschaerft:

- E05 `Capability ist graphbasiert`: Capability-Identitaet wird semantisch verfolgt, nicht ueber Namen oder Ort. Verschieben, Umbenennen, Wrappen, Aufteilen oder semantiknahes Neuformulieren darf Retirement oder Neuanlage nicht vortaeuschen.
- E06 `Keine neue nichtkanonische Writer-Erreichbarkeit`: StructuralWriteAuthority besitzt nicht nur den Writer, sondern auch den produktiven Einstieg in den Mutation-Graphen. Pipeline-, Analyse-, Metric-, Calibration-, Audit- und Render-Polish-Schichten duerfen keine neue Writer-Aufrufautoritaet eroeffnen.

Damit bleibt der bestehende E00-E31-Regelindex unveraendert; der Audit-Code fuer den Cold-Start-Readback muss in dieser MusicMutation nicht geaendert werden.

## Nicht geaendert

- kein neuer Writer,
- kein neuer Writer-Root,
- keine neue produktive Call-Site,
- keine neue Repair-/Recovery-Schicht,
- kein Projekt-/Takt-/Staff-/Voice-/Pitch-Identitaetsselektor,
- keine Klangmutation,
- kein MasterApply,
- kein Layout-/Beam-/Clef-/Octave-Shift-Write,
- keine Lockerung eines Kodex- oder Semantic-Capability-Gates.

Der spaetere Rest-Glyph-Write, der bei Display-Positionierung `measure="yes"` entfernen kann, wird in diesem Take bewusst nicht veraendert; er ist nicht die Ursache der vierteiligen Restspur und bleibt ein separater Befund.
