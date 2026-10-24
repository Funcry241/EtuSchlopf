# Renderer Feasibility Constraint Shadow Replan r32

Parent: `0.10.1839 r31`

ChangeClass: `AuditOnly`

## Ausgangslage

Der autoritative r31-Vollkorpus ist 10/10 DARK GREEN. Fuer alle zehn Etueden ist
die Renderer-Demand-Bindung `BOUND`, die vorhandenen Measure-Demands setzen sich
fuer jedes geplante System exakt zu `SelectedSystemLoads` zusammen und die
Rhythmic-Shape-Evidence ist 10/10 `OBSERVED`.

Der Renderer-Readback zeigt weiterhin genau 11 zusaetzliche Systembreaks in
Etuede 2, 3, 6, 7 und 8. Es gibt keine kollabierten geplanten Systembreaks und
keine Page-Drifts. Headroom, Zeitspalten, Notenzahl und Stack-Groesse besitzen
jeweils Gegenbeispiele; r32 fuehrt deshalb keinen weiteren Breiten- oder
Glyph-Score ein.

## Ziel

r32 behandelt ausschliesslich einen bereits beobachteten Renderer-Widerspruch
als read-only Unzulaessigkeits-Evidence:

- Ein geplanter Systembereich, den MuseScore im finalen PDF tatsaechlich
  aufgetrennt hat, wird fuer den Shadow als exaktes `No-Good` markiert.
- Der originale, in r31 gebundene Measure-Demand-Vektor bleibt die einzige
  Demand-Wahrheit.
- `AcceptedSystemCapacity`, `MaxAllowedSystemMeasures`, `SelectedPages` und
  `VerticalMaxSystemsPerPage` werden unveraendert aus der bestehenden
  Joint-Planner-Evidence gelesen.
- Der Shadow sucht deterministisch einen alternativen Page/System-Kandidaten,
  der die beobachteten No-Goods nicht wiederverwendet.

Der Shadow besitzt keinerlei Auswahl- oder Schreibrecht.

## Statusprojektion

Fuer Projekte ohne Renderer-Drift:

`RendererFeasibilityConstraintShadow: Status NOT_REQUIRED ...`

Fuer einen alternativen Kandidaten, der PageCount und Systemzahl haelt und die
Singleton-Zahl nicht verschlechtert:

`Status FEASIBLE_SAME_ENVELOPE`

Falls die bekannten No-Goods nur mit einer groesseren Systemzahl bzw. sonstiger
Verschlechterung des bisherigen Envelopes vermeidbar sind:

`Status FEASIBLE_DEGRADED`

Die Projektion nennt zusaetzlich:

- `ObservedNoGoods`
- `CandidatePages`
- `CandidateSystems`
- `CandidateSingletons`
- `CandidatePageSlots`
- `CandidateSystemGroups` mit Load
- `MaxCandidateSystemLoad`
- `AcceptedSystemCapacity`
- `SamePageCount`
- `SameSystemCount`
- `SingletonNotWorse`
- `AvoidedObservedNoGoods`

und immer:

- `CandidateOnly ja`
- `SelectionAuthority nein`
- `WriterAuthority nein`
- `PolicyWrite nein`

## Harte Bindungen

Der Constraint-Shadow wird nur ausgewertet, wenn:

1. r31-DemandBinding gegen den materialisierten Plan exakt validiert,
2. `SelectedPages` exakt der geplanten Page-Zahl entspricht,
3. `SelectedSystems` exakt der geplanten Systemzahl entspricht,
4. `SelectedSingletons` exakt aus dem materialisierten Plan reproduzierbar ist,
5. der produktive Plan keine `ForcedOverCapacitySingletons` besitzt,
6. nur eingefuegte Renderer-Systembreaks vorliegen; Page-Drift oder kollabierte
   Systembreaks werden nicht still umgedeutet.

Bei jeder Inkonsistenz wird der neue Shadow nur `UNAVAILABLE`.

## Keine produktive Aenderung

r32 aendert nicht:

- Demand-Formel,
- Capacity,
- Thresholds,
- PageCount-Preference,
- produktiven Joint Plan,
- Materializer,
- Writer,
- Rendereraufruf,
- MusicXML,
- PDF,
- musikalische Semantik.

Die alternative Partition wird nicht in eine Datei geschrieben und nicht an
MuseScore uebergeben.

## Erwarteter Korpusbefund

Die Offline-Gegenprobe auf der r31-Evidence zeigt, dass die bekannten No-Goods
in allen fuenf betroffenen Projekten unter denselben Capacity-/Shape-Hardgrenzen
vermeidbar bleiben. Fuer Etuede 2, 3, 6 und 8 existiert dabei ein Kandidat mit
dergleicher Systemzahl; fuer Etuede 7 steigt die minimale Shadow-Systemzahl nach
Ausschluss der drei beobachteten No-Goods voraussichtlich um eins. Diese Werte
sind vor dem Windows-r32-Lauf nur eine Gegenprobe und keine autoritative
Runtime-Evidence.

## Regression

r32 ergaenzt Tests dafuer, dass:

- ein beobachtetes No-Good nicht im Shadow-Kandidaten wieder auftaucht,
- ohne eingefuegten Renderer-Break kein Constraint-Replan gestartet wird,
- Planner-Envelope-Drift fail-closed als nicht bindbar gilt.

Erwarteter Testbestand: **756/756**.

## Naechster Schritt

Erst wenn r32 im 10/10-Korpus die Constraint-Feasibility sauber beweist, darf
ein spaeterer Shadow-Schnitt einen Kandidaten temporaer materialisieren und vom
Renderer pruefen lassen. Auch dieser Schritt darf die produktive Ausgabe noch
nicht automatisch uebernehmen.

Kein Threshold-Tuning. Keine Etueden-/Taktspeziallogik. Kein `--force`.
