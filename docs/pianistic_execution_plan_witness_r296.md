# r296 PianisticExecutionPlanWitness

## Parent und Ziel

r296 basiert direkt auf dem real installierten und dunkelgruenen r295 `PianisticConfigurationConstraintKernelDirectAuthorityClosure` mit 1026/1026 Tests. r292 liefert die exakte Attack/Hold/Release- und Tempo-Zeitachse; r295 liefert den read-only statischen Keyboard/Hand/Finger/Capability-Kern.

r296 fuehrt darauf erstmals die konstruktive ExecutionPlan-Domaene ein. Der Take bleibt `AuditOnly`: kein MusicXML-Writer, kein Render-Writer, kein MasterApply und keine Musikmutation.

## Physische Key-Obligations statt Notationsduplikate

Die 1237 `PianisticExecutionEvent`-Ereignisse von Etuede 6 sind musikalische Attack/Hold/Release-Ereignisse. Fuer einen koerperlichen Witness wird zusaetzlich eine physische Key-Obligation gebildet.

Ereignisse werden nur dann zu einer einzigen physischen Key-Obligation zusammengefasst, wenn MIDI-Pitch, Attack-Tick, Release-Tick und beide physischen Zeitendpunkte exakt identisch sind. Die Source-Ereignisse bleiben als Multiplicity gebunden. Gleiches MIDI mit abweichendem Attack oder Release wird nicht still zusammengelegt.

Auf dem aktuellen r295-Master werden fuer r296 vorab erwartet:

- SourceExecutionEvents: 1237
- PhysicalKeyObligations: 1234
- CoalescedExactDuplicateEvents: 3
- AttackBoundaryCount: 626
- BoundaryCount: 645
- MaxSimultaneousAttacks: 7
- MaxConcurrentKeyObligations: 7

Diese Werte sind keine Etueden-Sonderregel und werden nicht hardcodiert; der reale Report berechnet sie aus dem Master.

## ExecutionPlan und Suchraum v1

Ein `PianisticExecutionPlan` ordnet jede physische Key-Obligation genau einem `Hand + Finger`-Slot zu. Eine positive Aussage ist nur erlaubt, wenn alle Obligations lueckenlos abgedeckt sind.

Der v1-Technikraum ist bewusst eng:

- eine Taste pro Finger
- ein Finger pro Taste
- Hand- und Fingeridentitaet bleiben waehrend einer Hold-Obligation stabil
- Fingersubstitution waehrend eines Holds ist noch nicht modelliert
- Handuebernahme waehrend eines Holds ist noch nicht modelliert
- Staff und Voice sind weiterhin keine Handwahrheit

Ein engerer v1-Technikraum darf positive Witnesses liefern. Ein Misserfolg darf jedoch nie verschweigen, dass andere Techniken noch unmodelliert sein koennen.

## Statische Capability

Jeder vollstaendige Handzustand an einer Boundary wird erneut ueber den r295-Owner `PianisticConfigurationConstraintKernel` geprueft.

- `SUPPORTED` darf im Suchpfad weiterlaufen.
- `REJECTED` verwirft den Pfad unter expliziter Evidenz.
- `UNMODELED` verwirft den Pfad fuer einen positiven Witness, zaehlt aber als Modellluecke und verhindert ein unehrliches negatives Gesamturteil.

Es wird keine Max-Span-, Pairwise-Span- oder Staff-Hand-Heuristik eingefuehrt.

## TransitionCapability

r296 fuehrt eine separate partielle Transition-Evidenz ein. Eine Transition bindet:

- Hand
- ganze Ausgangskonfiguration
- ganze Zielkonfiguration
- relative Verschiebung der tiefsten Taste
- explizite Mindestzeit in Mikrosekunden
- Evidence-ID

Die Konfigurationen selbst bleiben an die r295-Signaturen gebunden. Fehlende Transition-Evidenz ist `UNMODELED`, nicht `REJECTED`.

Initiale Handpositionierung vor dem ersten Einsatz wird nicht als Reach-Regel erfunden. Eine reine Freigabe in den leeren Handzustand benoetigt ebenfalls keine erfundene Bewegungsgrenze. Eine spaetere Wiederaufnahme nach einer leeren Phase kann die seit der Freigabe vergangene physische Zeit verwenden.

## Bounded exhaustive search und ehrliche Statuswerte

Der Suchkern ist deterministisch und budgetiert. Er darf exakt folgende fachlichen Statuswerte erzeugen:

- `REALIZATION_WITNESS_FOUND`
- `NO_REALIZATION_UNDER_MODEL`
- `MODEL_INSUFFICIENT`
- `RESOURCE_LIMIT`
- `NOT_EVALUATED`

Ein Ressourcenabbruch ist niemals ein negatives Spielbarkeitsurteil. Wenn alle bislang modellierten Pfade scheitern, aber mindestens ein statischer oder dynamischer Zweig `UNMODELED` war, lautet das Ergebnis `MODEL_INSUFFICIENT`.

## Independent Witness Replay

Ein gefundener Plan wird nicht direkt als Wahrheit akzeptiert. `IndependentWitnessReplay` rekonstruiert aus Score + Modell + Witness erneut:

- Score-Fingerprint-Bindung
- Modell-ID-Bindung
- vollstaendige Obligation-Coverage
- genau eine Hand/Finger-Zuordnung je physischer Obligation
- keine gleichzeitige Doppelbelegung eines Fingerslots
- keine stillschweigend ueberlappenden unabhaengigen Obligations derselben Taste
- jede statische Konfiguration
- jede benoetigte Transition
- Witness-SHA256

Nur `IndependentWitnessReplay=VERIFIED` darf zu `REALIZATION_WITNESS_FOUND` fuehren.

## Produktive Grenze in r296

r295 besitzt absichtlich noch kein produktives Pianistenprofil. r296 erfindet deshalb keines nachtraeglich.

Fuer den realen Score wird `search_execution_plan(score, None, ...)` tatsaechlich aufgerufen. Das erwartete Ergebnis lautet:

`MODEL_INSUFFICIENT; no-productive-pianist-execution-capability-model-bound`

Damit ist die Such-/Replay-Architektur real adoptiert, ohne eine fingierte Spielbarkeitsaussage zu erzeugen. Ein synthetisches, vollstaendig benanntes Runtime-Integrity-Modell beweist dagegen, dass ein echter Witness gefunden und unabhaengig replay-validiert werden kann. Dieses synthetische Modell ist ausdruecklich kein Pianistenprofil.

## Regressionen

r296 fuegt zehn allgemeine Regressionstests hinzu:

1. exakte Sound-Duplikate werden physisch einmal gebunden
2. gleiche Tonhoehe mit anderem Release wird nicht still koalesziert
3. fehlendes produktives Modell => `MODEL_INSUFFICIENT`
4. vollstaendiges synthetisches Modell => Witness + Replay `VERIFIED`
5. fehlende statische Evidenz => `MODEL_INSUFFICIENT`
6. explizit vollstaendig abgelehnte Einzelnote => `NO_REALIZATION_UNDER_MODEL`
7. fehlende Transition-Evidenz => `MODEL_INSUFFICIENT`
8. Suchbudgetende => `RESOURCE_LIMIT`
9. fremder Score-Fingerprint wird im Replay verworfen
10. Runtime-Integrity bindet Suche, Replay und fail-closed Statussemantik

Ausgehend von den realen 1026/1026 r295-Tests ist die erwartete integrierte Summe 1036/1036.

## Naechster Schritt

Nach real gruener r296-Verifikation ist noch keine reale Spielbarkeit von Etuede 6 bewiesen, weil kein produktives Capability-Modell gebunden ist. Der folgende fachliche Schritt muss deshalb die Capability-Evidenz fuer reale Konfigurationen/Transitionen ehrlich schliessen beziehungsweise deren Modellluecken sichtbar machen, bevor r297 Obstruction/Reserve/Tempo-Frontier belastbare Aussagen ableiten darf.
