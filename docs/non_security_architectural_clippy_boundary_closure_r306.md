# NonSecurityArchitecturalClippyBoundaryClosure r306

## Autoritativer Parent

Parent ist der real installierte r305 `LegacyClefIslandStaffProjectionRetirement`.
Der anschliessende Vollkorpus ist vollstaendig und gruen: 10/10 Projekte `OK`,
0 Blockerprojekte, 0 Warnprojekte, 10/10 `FinalClean`, alle Ausgaben vollstaendig.

Strict-Clippy meldet aktuell 146 Diagnosen:

- 141 mechanisch
- 5 architektonisch

Die 5 Architekturdiagnosen repraesentieren vier verschiedene Funktionen.
`semantic_capability_diff::validate_policy()` liegt im Security-Gate und wird in
zwei Targets kompiliert, weshalb dieselbe `too_many_arguments`-Stelle zweimal
gezaehlt wird. r306 beruehrt diese Security-Domaene ausdruecklich nicht.

## Ziel

r306 schliesst die drei Nicht-Security-Architekturdiagnosen nicht durch
`#[allow(...)]`, sondern durch benannte Daten-/Kontextgrenzen.

### PianisticExecutionPlanWitness

`validate_candidate_state()` erhielt bisher acht Argumente. Fuenf davon bilden
einen stabil zusammengehoerigen read-only Bewertungsrahmen: aktuelle Boundary,
vorherige Boundary-Zeit, DemandTopology, KeyboardGeometry und gebundenes
CapabilityModel. Diese Werte werden als `CandidateValidationContext` gebunden.

Der Kandidat, sein Vorgaenger und die mutablen Suchzaehler bleiben explizit.
Damit veraendert sich weder Suchraum noch Bewertungsreihenfolge noch
Capability-Entscheidung. Der eingefrorene r292-r300-Pianistikstrang wird nicht
fachlich fortgesetzt.

### RendererFeasibility Frontier

`build_renderer_feasibility_frontier_candidate()` erhielt acht positionsabhaengige
Argumente. Die Parameter gehoeren zu genau einem Frontier-Auftrag und werden nun
als `RendererFeasibilityFrontierRequest` mit benannten Feldern transportiert.

Der produktive Orchestrator bindet exakt dieselben Werte aus der bestehenden
RendererFeasibility-Basis und dasselbe run-lokale No-Good-Set. Algorithmus,
Preference-Vergleich und Candidate-Promotion bleiben unveraendert.

### Registernotation

`kanonische_taktzeiten()` lieferte `(i64, BTreeMap<(String, usize), TaktZeit>)`.
Skalierung und Tabelle sind gemeinsam die kanonische Zeitachse der
Registernotation und werden nun als `KanonischeTaktzeitAchse` benannt.

Es werden keine Zeitwerte, Eventgrenzen, Oktavierungsentscheidungen oder
Materialisierungsauftraege geaendert.

## ChangeClass

`RefactorOnly`

Keine Security-Datei wird veraendert. Keine MusicXML-Mutation, kein neuer Writer,
keine neue Repair-/Recovery-Schicht, keine Schwelle und keine Etueden-/Takt-/Pitch-
Selektionsregel werden eingefuehrt.

## Erwartung

Nach realer Installation:

- Build/Strict-Clippy/Renderer weiterhin gruen
- integrierte Tests weiterhin 1064/1064
- Strict-Clippy 146 -> 143
- `architectural_budget` 5 -> 2
- `small_mechanical_budget` unveraendert 141

Die verbleibenden zwei Architekturdiagnosen muessen danach dieselbe isolierte
Security-Funktion betreffen und werden nur in einem getrennten `SecurityOnly`-
Schritt betrachtet.
